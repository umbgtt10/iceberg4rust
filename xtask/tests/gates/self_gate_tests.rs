// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::process::command_runner_tests::FakeCommandRunner;
use xtask::gates::gate::Gate;
use xtask::gates::self_gate::SelfGate;

fn gate(runner: &FakeCommandRunner) -> SelfGate<'_> {
    SelfGate::new(
        runner,
        String::from("Cargo.toml"),
        String::from("cargo-iceberg4rust"),
        vec![String::from("cargo-iceberg4rust")],
        String::from("9.5"),
    )
}

#[test]
fn label_names_the_file_risk_gate() {
    // Arrange
    let runner = FakeCommandRunner::new();

    // Act
    let label = gate(&runner).label();

    // Assert
    assert_eq!(label, "File risk (self-analysis)");
}

#[test]
fn run_builds_the_binary_from_this_checkout_rather_than_an_installed_one() {
    // Arrange
    let runner = FakeCommandRunner::new();

    // Act
    let _ = gate(&runner).run();

    // Assert
    let call = &runner.calls()[0];
    assert_eq!(call[0], "run");
    assert!(call.contains(&String::from("--bin")));
    assert!(call.contains(&String::from("cargo-iceberg4rust")));
}

#[test]
fn run_passes_the_configured_threshold() {
    // Arrange
    let runner = FakeCommandRunner::new();

    // Act
    let _ = gate(&runner).run();

    // Assert
    let call = &runner.calls()[0];
    assert!(call.contains(&String::from("--threshold")));
    assert!(call.contains(&String::from("9.5")));
}

#[test]
fn run_with_a_zero_exit_code_returns_ok() {
    // Arrange
    let runner = FakeCommandRunner::new().with_streaming_code(Some(0));

    // Act
    let result = gate(&runner).run();

    // Assert
    assert!(result.is_ok());
}

#[test]
fn run_with_exit_code_one_reports_the_exit_code() {
    // Arrange
    let runner = FakeCommandRunner::new().with_streaming_code(Some(1));

    // Act
    let result = gate(&runner).run();

    // Assert
    assert!(result.is_err_and(|error| error.contains("exit code")));
}

// 2 is iceberg4rust's own "offenders found", which the gate must report as a
// breached ceiling rather than as the tool failing.
#[test]
fn run_with_exit_code_two_names_the_ceiling_it_breached() {
    // Arrange
    let runner = FakeCommandRunner::new().with_streaming_code(Some(2));

    // Act
    let result = gate(&runner).run();

    // Assert
    assert_eq!(
        result,
        Err(String::from("a file is at or above the ceiling of 9.5"))
    );
}
