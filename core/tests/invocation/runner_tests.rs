// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_source_tree::InMemorySourceTree;
use crate::support::in_memory_workspace_reader::InMemoryWorkspaceReader;
use iceberg4rust::invocation::config::Config;
use iceberg4rust::invocation::runner::Runner;

#[test]
fn run_with_a_workspace_that_cannot_be_read_returns_the_error() {
    // Arrange
    let config = Config {
        packages: vec![],
        threshold: 20.0,
        top: 20,
        json: false,
    };
    let reader = InMemoryWorkspaceReader::failing();
    let tree = InMemorySourceTree::new();

    // Act
    let result = Runner::new(&reader, &tree).run(&config);

    // Assert
    assert_eq!(
        result.expect_err("unreadable").to_string(),
        "failed to read Cargo metadata"
    );
}
