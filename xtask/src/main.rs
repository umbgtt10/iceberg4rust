// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::args;
use std::path::Path;
use std::process::ExitCode;
use xtask::crap::crap_report_parser::CrapReportParser;
use xtask::gates::crap_gate::CrapGate;
use xtask::gates::gate::Gate;
use xtask::gates::self_gate::SelfGate;
use xtask::gates::stage2::Stage2;
use xtask::gates::stern_gate::SternGate;
use xtask::gates::twin_gate::TwinGate;
use xtask::process::system_command_runner::SystemCommandRunner;

const CORE_PACKAGE: &str = "cargo-iceberg4rust";
const XTASK_PACKAGE: &str = "xtask";
const CRAP_THRESHOLD: &str = "15";
const SELF_GATE_THRESHOLD: &str = "9.5";

// Reading the real process argv and wiring the concrete runner are the two
// things no test can reach, so they are all this binary does.
fn main() -> ExitCode {
    match args().nth(1).as_deref() {
        Some("stage2") => run_stage2(),
        _ => {
            eprintln!("usage: cargo xtask stage2");
            ExitCode::FAILURE
        }
    }
}

fn run_stage2() -> ExitCode {
    let manifest_path = workspace_manifest_path();
    let runner = SystemCommandRunner::new();
    let parser = CrapReportParser::new();

    // Both members, so the crate that enforces the bar is held to it too.
    let packages = vec![String::from(CORE_PACKAGE), String::from(XTASK_PACKAGE)];

    let stern = SternGate::new(&runner, manifest_path.clone(), packages.clone());
    let crap = CrapGate::new(
        &runner,
        &parser,
        manifest_path.clone(),
        packages.clone(),
        String::from(CRAP_THRESHOLD),
    );
    let twin = TwinGate::new(&runner, manifest_path.clone(), packages.clone());
    let self_gate = SelfGate::new(
        &runner,
        manifest_path,
        String::from(CORE_PACKAGE),
        packages,
        String::from(SELF_GATE_THRESHOLD),
    );

    let gates: Vec<&dyn Gate> = vec![&stern, &crap, &twin, &self_gate];

    match Stage2::new(gates).run() {
        Ok(()) => {
            println!("\niceberg4rust Stage 2 passed!");
            ExitCode::SUCCESS
        }
        Err(reason) => {
            eprintln!("\nFailed: {reason}");
            ExitCode::FAILURE
        }
    }
}

fn workspace_manifest_path() -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives one directory below the workspace root")
        .join("Cargo.toml")
        .to_string_lossy()
        .into_owned()
}
