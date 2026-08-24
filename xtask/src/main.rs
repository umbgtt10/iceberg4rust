// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Deserialize;
use serde_json::from_str;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitCode;

const PACKAGE: &str = "cargo-iceberg4rust";
const CRAP_THRESHOLD: &str = "15";
const SELF_GATE_THRESHOLD: &str = "9.5";

type Gate = fn(&Path) -> Result<(), String>;

fn main() -> ExitCode {
    match env::args().nth(1).as_deref() {
        Some("stage2") => run_stage2(),
        _ => {
            eprintln!("usage: cargo xtask stage2");
            ExitCode::FAILURE
        }
    }
}

fn run_stage2() -> ExitCode {
    let manifest = workspace_manifest_path();

    let gates: [(&str, Gate); 4] = [
        ("House rules iceberg4rust", gate_stern4rust),
        ("CRAP iceberg4rust", gate_crap4rust),
        ("Mirrored tests iceberg4rust", gate_twin4rust),
        ("File risk iceberg4rust (self-analysis)", gate_self_analysis),
    ];

    for (label, gate) in gates {
        println!("{label}...");
        if let Err(reason) = gate(&manifest) {
            eprintln!("\nFailed: {label} ({reason})");
            return ExitCode::FAILURE;
        }
    }

    println!("\niceberg4rust Stage 2 passed!");
    ExitCode::SUCCESS
}

fn workspace_manifest_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives one directory below the workspace root")
        .join("Cargo.toml")
}

// Mirrors `Get-Command <name> -ErrorAction SilentlyContinue` from the
// PowerShell gate: check before running, so a missing tool reads as an
// install instruction rather than a confusing "no such subcommand" from
// cargo itself.
fn require_cargo_subcommand(binary_name: &str) -> Result<(), String> {
    if which(binary_name).is_some() {
        return Ok(());
    }
    Err(format!(
        "{binary_name} is not installed -- run: cargo install {binary_name}"
    ))
}

fn which(binary_name: &str) -> Option<PathBuf> {
    let exe_name = format!("{binary_name}{}", env::consts::EXE_SUFFIX);
    let path_var = env::var_os("PATH")?;
    env::split_paths(&path_var)
        .map(|dir| dir.join(&exe_name))
        .find(|candidate| candidate.is_file())
}

fn gate_stern4rust(manifest: &Path) -> Result<(), String> {
    require_cargo_subcommand("cargo-stern4rust")?;

    let status = Command::new("cargo")
        .arg("stern4rust")
        .arg("--manifest-path")
        .arg(manifest)
        .args(["--package", PACKAGE])
        .status()
        .map_err(|e| format!("failed to launch cargo stern4rust: {e}"))?;

    // 2 is a rule broken; 1 is the tool failing to run at all. Kept apart so
    // a bad manifest cannot read as a clean codebase.
    match status.code() {
        Some(0) => Ok(()),
        Some(2) => Err("a house coding rule was broken".to_string()),
        code => Err(format!("could not run, exit code {code:?}")),
    }
}

fn gate_twin4rust(manifest: &Path) -> Result<(), String> {
    require_cargo_subcommand("cargo-twin4rust")?;

    let status = Command::new("cargo")
        .arg("twin4rust")
        .arg("--manifest-path")
        .arg(manifest)
        .args(["--package", PACKAGE])
        .status()
        .map_err(|e| format!("failed to launch cargo twin4rust: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err("source files without a mirrored test".to_string())
    }
}

fn gate_self_analysis(manifest: &Path) -> Result<(), String> {
    let status = Command::new("cargo")
        .args(["run", "--quiet", "--bin", PACKAGE, "--"])
        .arg("--manifest-path")
        .arg(manifest)
        .args(["--threshold", SELF_GATE_THRESHOLD])
        .status()
        .map_err(|e| format!("failed to launch {PACKAGE}: {e}"))?;

    // 2 is the tool's own "offenders found"; anything else non-zero is a
    // failure to run at all.
    match status.code() {
        Some(0) => Ok(()),
        Some(2) => Err(format!(
            "a file is at or above the ceiling of {SELF_GATE_THRESHOLD}"
        )),
        code => Err(format!("exit code {code:?}")),
    }
}

#[derive(Deserialize)]
struct CrapReport {
    total_functions: u32,
    crappy_functions: u32,
    crappy_percent: f64,
}

fn gate_crap4rust(manifest: &Path) -> Result<(), String> {
    require_cargo_subcommand("cargo-crap4rust")?;

    let output = Command::new("cargo")
        .arg("crap4rust")
        .arg("--manifest-path")
        .arg(manifest)
        .args(["--package", PACKAGE])
        .args(["--warn-only", "--threshold", CRAP_THRESHOLD])
        .args(["--output-format", "json"])
        .output()
        .map_err(|e| format!("failed to launch cargo crap4rust: {e}"))?;

    if !output.status.success() {
        return Err(format!("exit code {:?}", output.status.code()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let report = parse_crap_report(&stdout)?;

    println!(
        "crap4rust: {}/{} functions crappy ({:.1}%)",
        report.crappy_functions, report.total_functions, report.crappy_percent
    );

    if report.crappy_functions > 0 {
        Err(format!(
            "{} crappy functions detected",
            report.crappy_functions
        ))
    } else {
        Ok(())
    }
}

// crap4rust's coverage pass runs the crate's own test suite first, so its
// stdout carries that test run's output ahead of the JSON report. The report
// is always the last top-level `{` in the stream.
fn parse_crap_report(stdout: &str) -> Result<CrapReport, String> {
    let start = stdout
        .lines()
        .enumerate()
        .filter(|(_, line)| *line == "{")
        .last()
        .map(|(index, _)| index)
        .ok_or_else(|| "could not find a JSON report in crap4rust's output".to_string())?;

    let json_text = stdout.lines().skip(start).collect::<Vec<_>>().join("\n");
    from_str(&json_text).map_err(|e| format!("could not parse crap4rust JSON: {e}"))
}
