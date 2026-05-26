use std::path::Path;
use std::process::ExitCode;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use oratos_audit::audit_pages;
use oratos_generate::{
    generate_html_remediation_prompt, generate_llms_txt, generate_metadata_recommendations,
};
use oratos_html::{load_file, load_pages, LoadOptions};
use oratos_report::ReportFormat;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(
    name = "oratos",
    version,
    about = "Website visibility intelligence for SEO, accessibility, structured metadata, and AI readiness."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Audit a local directory, file, or URL
    Audit {
        /// Path to directory/file or URL to audit
        target: String,
        /// Output format: console, json, markdown, html, sarif
        #[arg(long, default_value = "console")]
        format: String,
        /// Write report to file instead of stdout
        #[arg(long)]
        output: Option<String>,
        /// Exit with non-zero status if overall score is below threshold (0-100)
        #[arg(long)]
        fail_under: Option<f64>,
        /// Treat warnings as errors for exit status
        #[arg(long)]
        strict: bool,
    },
    /// Generate recommendations and artifacts
    Generate {
        #[command(subcommand)]
        command: GenerateCommands,
    },
    /// Generate LLM remediation prompts
    Prompt {
        #[command(subcommand)]
        command: PromptCommands,
    },
}

#[derive(Subcommand)]
enum GenerateCommands {
    /// Generate llms.txt from discovered pages
    Llms {
        target: String,
        #[arg(long)]
        output: Option<String>,
    },
    /// Generate metadata and JSON-LD recommendations
    Metadata {
        target: String,
        #[arg(long)]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
enum PromptCommands {
    /// Generate an HTML remediation prompt for an LLM
    Html {
        /// HTML file path or URL
        file_or_url: String,
        #[arg(long)]
        output: Option<String>,
    },
}

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    match run().await {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::from(1)
        }
    }
}

async fn run() -> Result<ExitCode> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Audit {
            target,
            format,
            output,
            fail_under,
            strict,
        } => run_audit(&target, &format, output.as_deref(), fail_under, strict).await,
        Commands::Generate { command } => run_generate(command).await,
        Commands::Prompt { command } => run_prompt(command).await,
    }
}

async fn run_audit(
    target: &str,
    format: &str,
    output: Option<&str>,
    fail_under: Option<f64>,
    strict: bool,
) -> Result<ExitCode> {
    let report_format = ReportFormat::parse(format).with_context(|| {
        format!("unknown format: {format}. Use console, json, markdown, html, or sarif")
    })?;

    let pages = load_pages(target, &LoadOptions::default())
        .await
        .context("failed to load pages")?;

    if pages.is_empty() {
        bail!("no HTML pages found at target: {target}");
    }

    let report = audit_pages(target, &pages);
    let rendered = report_format.render(&report);

    write_output(output, &rendered)?;

    let mut failed = false;

    if let Some(threshold) = fail_under {
        if report.scores.overall < threshold {
            eprintln!(
                "audit failed: overall score {:.1} is below threshold {threshold}",
                report.scores.overall
            );
            failed = true;
        }
    }

    if strict {
        let has_warnings = report
            .findings
            .iter()
            .any(|f| f.severity == oratos_core::Severity::Warning);
        let has_errors = report
            .findings
            .iter()
            .any(|f| f.severity == oratos_core::Severity::Error);
        if has_warnings || has_errors {
            if has_errors {
                eprintln!("audit failed: errors found (--strict)");
            } else {
                eprintln!("audit failed: warnings found (--strict)");
            }
            failed = true;
        }
    }

    if failed {
        Ok(ExitCode::from(1))
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

async fn run_generate(command: GenerateCommands) -> Result<ExitCode> {
    match command {
        GenerateCommands::Llms { target, output } => {
            let pages = load_pages(&target, &LoadOptions::default()).await?;
            let txt = generate_llms_txt(&pages, None);
            write_output(output.as_deref(), &txt)?;
        }
        GenerateCommands::Metadata { target, output } => {
            let pages = load_pages(&target, &LoadOptions::default()).await?;
            let recs = generate_metadata_recommendations(&pages);
            let json = serde_json::to_string_pretty(&recs)?;
            write_output(output.as_deref(), &json)?;
        }
    }
    Ok(ExitCode::SUCCESS)
}

async fn run_prompt(command: PromptCommands) -> Result<ExitCode> {
    match command {
        PromptCommands::Html {
            file_or_url,
            output,
        } => {
            let pages = if file_or_url.starts_with("http://") || file_or_url.starts_with("https://")
            {
                load_pages(&file_or_url, &LoadOptions::default()).await?
            } else {
                let path = Path::new(&file_or_url);
                if path.is_dir() {
                    let all = load_pages(&file_or_url, &LoadOptions::default()).await?;
                    let index = all
                        .iter()
                        .find(|p| p.url_or_path.ends_with("index.html"))
                        .or_else(|| all.first());
                    index.map(|p| vec![p.clone()]).unwrap_or(all)
                } else {
                    vec![load_file(path)?]
                }
            };

            let page = pages
                .first()
                .with_context(|| format!("no HTML found at: {file_or_url}"))?;

            let audit_target = if Path::new(&file_or_url).is_dir() {
                file_or_url.clone()
            } else {
                page.url_or_path.clone()
            };
            let report = Some(audit_pages(&audit_target, std::slice::from_ref(page)));

            let prompt = generate_html_remediation_prompt(page, report.as_ref());
            write_output(output.as_deref(), &prompt)?;
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn write_output(path: Option<&str>, content: &str) -> Result<()> {
    if let Some(path) = path {
        std::fs::write(path, content).with_context(|| format!("failed to write {path}"))?;
    } else {
        print!("{content}");
    }
    Ok(())
}
