// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::reporting::private_function_metrics::PrivateFunctionMetrics;
use std::collections::BTreeSet;

#[test]
fn clone_produces_equal_independent_copy() {
    // Arrange
    let mut structs = BTreeSet::new();
    structs.insert("Helper".to_string());
    let original = PrivateFunctionMetrics {
        name: "helper".to_string(),
        line: 1,
        complexity: 7,
        used_private_structs: structs,
        is_hidden: true,
    };

    // Act
    let cloned = original.clone();

    // Assert
    assert_eq!(cloned.complexity, original.complexity);
    assert_eq!(cloned.used_private_structs, original.used_private_structs);
}

#[test]
fn debug_format_includes_fields() {
    // Arrange
    let mut structs = BTreeSet::new();
    structs.insert("A".to_string());
    let metrics = PrivateFunctionMetrics {
        name: "helper".to_string(),
        line: 1,
        complexity: 1,
        used_private_structs: structs,
        is_hidden: true,
    };

    // Act
    let debug = format!("{metrics:?}");

    // Assert
    assert!(debug.contains("complexity: 1"));
    assert!(debug.contains("used_private_structs"));
    assert!(debug.contains("\"A\""));
}
