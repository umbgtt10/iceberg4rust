// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use clap::Parser;
use iceberg4rust::args::Args;
use iceberg4rust::runner::Runner;

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

// With no --manifest-path the manifest is resolved from the working directory.
// This crate is a single package, so resolution succeeds and the run analyses
// it. A workspace of several members errors instead, asking for --package —
// which is why this assertion is the opposite of what it was inside one.
#[test]
fn run_with_no_arguments_analyses_the_current_package() {
    // Arrange
    let args = Args::parse_from(["cargo-iceberg4rust"]);

    // Act
    let result = Runner::run(args);

    // Assert
    assert!(result.is_ok());
}
