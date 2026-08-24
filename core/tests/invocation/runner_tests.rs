// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use clap::Parser;
use iceberg4rust::invocation::args::Args;
use iceberg4rust::invocation::runner::Runner;

// With no --manifest-path the manifest is resolved from the working directory,
// which resolves to this workspace rather than to one member. A virtual root
// has no package of its own to default to, so the run has to ask which member
// to analyse instead of guessing.
#[test]
fn run_with_no_arguments_in_a_workspace_asks_for_a_package() {
    // Arrange
    let args = Args::parse_from(["cargo-iceberg4rust"]);

    // Act
    let result = Runner::run(args);

    // Assert
    assert!(
        result.is_err_and(|error| error.to_string().contains("--package")),
        "a virtual workspace root must name the member rather than pick one"
    );
}

#[test]
fn run_with_nonexistent_manifest_returns_error() {
    // Arrange
    let args = Args::parse_from([
        "cargo-iceberg4rust",
        "--manifest-path",
        "C:\\nonexistent_path_12345\\Cargo.toml",
    ]);

    // Act
    let result = Runner::run(args);

    // Assert
    assert!(result.is_err());
}
