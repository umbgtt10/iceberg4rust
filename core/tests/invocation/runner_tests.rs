// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::invocation::config::Config;
use iceberg4rust::invocation::runner::Runner;
use std::path::PathBuf;

#[test]
fn run_with_a_manifest_that_does_not_exist_returns_an_error() {
    // Arrange
    let config = Config {
        manifest_path: Some(PathBuf::from("C:\\nonexistent_path_12345\\Cargo.toml")),
        packages: vec![],
        threshold: 20.0,
        top: 20,
        json: false,
    };

    // Act
    let result = Runner::run(&config);

    // Assert
    assert!(result.is_err());
}
