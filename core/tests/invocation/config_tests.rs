// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::invocation::config::Config;
use std::path::PathBuf;

#[test]
fn config_clone_produces_independent_copy() {
    // Arrange
    let config = Config {
        manifest_path: Some(PathBuf::from("Cargo.toml")),
        packages: vec!["foo".to_string()],
        threshold: 10.0,
        top: 5,
        json: false,
    };

    // Act
    let cloned = config.clone();

    // Assert
    assert_eq!(cloned.manifest_path, config.manifest_path);
    assert_eq!(cloned.packages, config.packages);
    assert_eq!(cloned.threshold, config.threshold);
    assert_eq!(cloned.top, config.top);
}
