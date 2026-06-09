use std::path::PathBuf;

use assert_cmd::Command;
use predicates::prelude::*;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn oratos() -> Command {
    Command::cargo_bin("oratos").unwrap()
}

#[test]
fn audit_local_directory_console_output() {
    let site = workspace_root().join("examples/static_site");
    oratos()
        .args(["audit", site.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Oratos Audit Report"));
}

#[test]
fn audit_json_to_stdout() {
    let site = workspace_root().join("examples/static_site");
    oratos()
        .args(["audit", site.to_str().unwrap(), "--format", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"findings\""));
}

#[test]
fn audit_output_flag_writes_file() {
    let site = workspace_root().join("examples/static_site");
    let out = tempfile::NamedTempFile::new().unwrap();
    let path = out.path().to_str().unwrap();

    oratos()
        .args([
            "audit",
            site.to_str().unwrap(),
            "--format",
            "json",
            "--output",
            path,
        ])
        .assert()
        .success();

    let body = std::fs::read_to_string(path).unwrap();
    assert!(body.contains("\"core_version\""));
}

#[test]
fn unknown_format_fails() {
    let site = workspace_root().join("examples/static_site");
    oratos()
        .args(["audit", site.to_str().unwrap(), "--format", "xml"])
        .assert()
        .failure();
}
