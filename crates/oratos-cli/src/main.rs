mod changed;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use oratos_audit::audit_pages;
use oratos_core::{apply_ignore_rules, OratosConfig};
use oratos_generate::{
    generate_html_remediation_prompt, generate_llms_txt, generate_metadata_recommendations,
    generate_phoenix_remediation_prompt,
};
use oratos_html::{load_pages, CrawlOptions, LoadOptions};
use oratos_report::ReportFormat;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(
    name = "oratos",
    version,
    about = "Website visibility intelligence for SEO, accessibility, structured metadata, and AI readiness."
)]
struct Cli {
    /// Path to oratos.toml (overrides auto-discovery)
    #[arg(long, global = true)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Audit a local directory, file, or URL
    Audit {
        target: String,
        #[arg(long, default_value = "console")]
        format: String,
        #[arg(long)]
        output: Option<String>,
        #[arg(long)]
        fail_under: Option<f64>,
        #[arg(long)]
        strict: bool,
        /// Crawl same-origin pages when target is a URL (or enable [crawl] in config)
        #[arg(long)]
        crawl: bool,
        /// Only audit HTML files changed in git (local directory targets)
        #[arg(long)]
        changed_only: bool,
    },
    Generate {
        #[command(subcommand)]
        command: GenerateCommands,
    },
    Prompt {
        #[command(subcommand)]
        command: PromptCommands,
    },
}

#[derive(Subcommand)]
enum GenerateCommands {
    Llms {
        target: String,
        #[arg(long)]
        output: Option<String>,
    },
    Metadata {
        target: String,
        #[arg(long)]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
enum PromptCommands {
    Html {
        file_or_url: String,
        #[arg(long)]
        output: Option<String>,
    },
    /// Phoenix-oriented remediation prompt (priv/static exports)
    Phoenix {
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
            crawl,
            changed_only,
        } => {
            run_audit(AuditOptions {
                target: &target,
                format: &format,
                output: output.as_deref(),
                fail_under,
                strict,
                crawl,
                changed_only,
                config_path: cli.config.as_deref(),
            })
            .await
        }
        Commands::Generate { command } => run_generate(command, cli.config.as_deref()).await,
        Commands::Prompt { command } => run_prompt(command, cli.config.as_deref()).await,
    }
}

struct AuditOptions<'a> {
    target: &'a str,
    format: &'a str,
    output: Option<&'a str>,
    fail_under: Option<f64>,
    strict: bool,
    crawl: bool,
    changed_only: bool,
    config_path: Option<&'a Path>,
}

fn load_config(explicit: Option<&Path>, target: &str) -> Result<OratosConfig> {
    if let Some(path) = explicit {
        return Ok(OratosConfig::load(path)?
            .unwrap_or_default());
    }
    let start = Path::new(target);
    Ok(OratosConfig::discover(start)?
        .map(|(_, c)| c)
        .unwrap_or_default())
}

async fn run_audit(opts: AuditOptions<'_>) -> Result<ExitCode> {
    let cfg = load_config(opts.config_path, opts.target)?;

    let format = opts.format.to_string();
    let format = cfg
        .audit
        .format
        .as_deref()
        .unwrap_or(format.as_str());

    let fail_under = opts.fail_under.or(cfg.audit.fail_under);
    let strict = opts.strict || cfg.audit.strict.unwrap_or(false);
    let changed_only = opts.changed_only || cfg.audit.changed_only;

    let report_format = ReportFormat::parse(format).with_context(|| {
        format!("unknown format: {format}. Use console, json, markdown, html, or sarif")
    })?;

    let mut load_options = LoadOptions::default();
    let url_target = opts.target.starts_with("http://") || opts.target.starts_with("https://");
    if url_target && (opts.crawl || cfg.crawl.enabled) {
        load_options.crawl = Some(CrawlOptions {
            max_pages: cfg.crawl.max_pages,
            max_depth: cfg.crawl.max_depth,
            respect_robots: cfg.crawl.respect_robots,
            use_sitemap: cfg.crawl.use_sitemap,
        });
    }

    let mut pages = load_pages(opts.target, &load_options)
        .await
        .context("failed to load pages")?;

    if changed_only {
        let root = Path::new(opts.target);
        if root.is_dir() {
            pages = changed::filter_pages_by_changed(pages, root)?;
        } else {
            eprintln!("warning: --changed-only applies to directory targets; ignoring");
        }
    }

    if pages.is_empty() {
        bail!("no HTML pages found at target: {}", opts.target);
    }

    let mut report = audit_pages(opts.target, &pages);
    apply_ignore_rules(&mut report, &cfg.audit.ignore_rules);

    let rendered = report_format.render(&report);
    write_output(opts.output, &rendered)?;

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

async fn run_generate(command: GenerateCommands, config_path: Option<&Path>) -> Result<ExitCode> {
    let _cfg = config_path;
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

async fn run_prompt(command: PromptCommands, _config_path: Option<&Path>) -> Result<ExitCode> {
    match command {
        PromptCommands::Html {
            file_or_url,
            output,
        } => {
            let (audit_target, audit_pages_set, page) = resolve_prompt_page(&file_or_url).await?;
            let report = Some(audit_pages(&audit_target, &audit_pages_set));
            let prompt = generate_html_remediation_prompt(&page, report.as_ref());
            write_output(output.as_deref(), &prompt)?;
        }
        PromptCommands::Phoenix {
            file_or_url,
            output,
        } => {
            let (audit_target, audit_pages_set, page) = resolve_prompt_page(&file_or_url).await?;
            let report = Some(audit_pages(&audit_target, &audit_pages_set));
            let prompt = generate_phoenix_remediation_prompt(&page, report.as_ref());
            write_output(output.as_deref(), &prompt)?;
        }
    }
    Ok(ExitCode::SUCCESS)
}

async fn resolve_prompt_page(
    file_or_url: &str,
) -> Result<(String, Vec<oratos_html::HtmlPage>, oratos_html::HtmlPage)> {
    if file_or_url.starts_with("http://") || file_or_url.starts_with("https://") {
        let pages = load_pages(file_or_url, &LoadOptions::default()).await?;
        let page = pages
            .first()
            .cloned()
            .with_context(|| format!("no HTML found at: {file_or_url}"))?;
        return Ok((file_or_url.to_string(), pages, page));
    }

    let path = Path::new(file_or_url);
    if path.is_dir() {
        let all = load_pages(file_or_url, &LoadOptions::default()).await?;
        let page = all
            .iter()
            .find(|p| p.url_or_path.ends_with("index.html"))
            .or_else(|| all.first())
            .cloned()
            .with_context(|| format!("no HTML found at: {file_or_url}"))?;
        return Ok((file_or_url.to_string(), all, page));
    }

    if path.is_file() {
        let parent = path.parent().unwrap_or(path);
        let all = load_pages(&parent.to_string_lossy(), &LoadOptions::default())
            .await
            .with_context(|| format!("failed to load sibling pages from {}", parent.display()))?;
        let canonical_target = path.canonicalize().ok();
        let page = all
            .iter()
            .find(|p| {
                Path::new(&p.url_or_path).canonicalize().ok() == canonical_target
            })
            .cloned()
            .or_else(|| all.first().cloned())
            .with_context(|| format!("no HTML found at: {file_or_url}"))?;
        return Ok((parent.to_string_lossy().to_string(), all, page));
    }

    bail!("target not found: {file_or_url}")
}

fn write_output(path: Option<&str>, content: &str) -> Result<()> {
    if let Some(path) = path {
        std::fs::write(path, content).with_context(|| format!("failed to write {path}"))?;
    } else {
        print!("{content}");
    }
    Ok(())
}
