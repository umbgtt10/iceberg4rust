// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust_cli::invocation::entry_point::EntryPoint;
use std::process::ExitCode;

fn argv(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|part| (*part).to_string()).collect()
}

#[test]
fn run_against_this_workspace_above_every_score_exits_with_success() {
    // Arrange
    let args = argv(&[
        "cargo-iceberg4rust",
        "--manifest-path",
        "../Cargo.toml",
        "--package",
        "iceberg4rust-core",
        "--package",
        "cargo-iceberg4rust",
        "--threshold",
        "1000",
        "--json",
    ]);

    // Act
    let code = EntryPoint::new().run(args).expect("run");

    // Assert
    assert_eq!(code, ExitCode::SUCCESS);
}

// The exit-code contract, end to end: 2 when a file is at or above the
// threshold, through the argv cargo hands a subcommand.
#[test]
fn run_through_cargo_against_this_workspace_with_a_zero_threshold_exits_with_two() {
    // Arrange
    let args = argv(&[
        "cargo-iceberg4rust",
        "iceberg4rust",
        "--manifest-path",
        "../Cargo.toml",
        "--package",
        "iceberg4rust-core",
        "--threshold",
        "0",
    ]);

    // Act
    let code = EntryPoint::new().run(args).expect("run");

    // Assert
    assert_eq!(code, ExitCode::from(2));
}

// With no --manifest-path the manifest is resolved from the working directory,
// which resolves to this workspace rather than to one member. A virtual root
// has no package of its own to default to, so the run has to ask which member
// to analyse instead of guessing.
#[test]
fn run_with_no_arguments_in_a_workspace_asks_for_a_package() {
    // Arrange
    let args = argv(&["cargo-iceberg4rust"]);

    // Act
    let result = EntryPoint::new().run(args);

    // Assert
    assert!(
        result.is_err_and(|error| error.to_string().contains("--package")),
        "a virtual workspace root must name the member rather than pick one"
    );
}

#[test]
fn run_with_nonexistent_manifest_returns_error() {
    // Arrange
    let args = argv(&[
        "cargo-iceberg4rust",
        "--manifest-path",
        "C:\\nonexistent_path_12345\\Cargo.toml",
    ]);

    // Act
    let result = EntryPoint::new().run(args);

    // Assert
    assert!(result.is_err());
}
