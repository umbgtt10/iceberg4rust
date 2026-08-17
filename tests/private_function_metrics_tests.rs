// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeSet;

use iceberg4rust::private_function_metrics::PrivateFunctionMetrics;

#[test]
fn zero_complexity_and_empty_structs() {
    // Arrange & Act
    let metrics = PrivateFunctionMetrics {
        name: "helper".to_string(),
        line: 1,
        complexity: 0,
        used_private_structs: BTreeSet::new(),
        is_hidden: true,
    };

    // Assert
    assert_eq!(metrics.complexity, 0);
    assert!(metrics.used_private_structs.is_empty());
}

#[test]
fn holds_provided_complexity_value() {
    // Arrange & Act
    let metrics = PrivateFunctionMetrics {
        name: "helper".to_string(),
        line: 1,
        complexity: 5,
        used_private_structs: BTreeSet::new(),
        is_hidden: true,
    };

    // Assert
    assert_eq!(metrics.complexity, 5);
}

#[test]
fn holds_provided_used_private_structs() {
    // Arrange
    let mut structs = BTreeSet::new();
    structs.insert("Config".to_string());
    structs.insert("State".to_string());

    // Act
    let metrics = PrivateFunctionMetrics {
        name: "helper".to_string(),
        line: 1,
        complexity: 3,
        used_private_structs: structs,
        is_hidden: true,
    };

    // Assert
    assert_eq!(metrics.used_private_structs.len(), 2);
    assert!(metrics.used_private_structs.contains("Config"));
    assert!(metrics.used_private_structs.contains("State"));
}

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
