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
    let original = PrivateFunctionMetrics::new("helper".to_string(), 1, 7, structs, true);

    // Act
    let cloned = original.clone();

    // Assert
    assert_eq!(cloned.complexity(), original.complexity());
    assert_eq!(
        cloned.used_private_structs(),
        original.used_private_structs()
    );
}

#[test]
fn debug_format_includes_fields() {
    // Arrange
    let mut structs = BTreeSet::new();
    structs.insert("A".to_string());
    let metrics = PrivateFunctionMetrics::new("helper".to_string(), 1, 1, structs, true);

    // Act
    let debug = format!("{metrics:?}");

    // Assert
    assert!(debug.contains("complexity: 1"));
    assert!(debug.contains("used_private_structs"));
    assert!(debug.contains("\"A\""));
}

#[test]
fn holds_provided_complexity_value() {
    // Arrange & Act
    let metrics = PrivateFunctionMetrics::new("helper".to_string(), 1, 5, BTreeSet::new(), true);

    // Assert
    assert_eq!(metrics.complexity(), 5);
}

#[test]
fn holds_provided_used_private_structs() {
    // Arrange
    let mut structs = BTreeSet::new();
    structs.insert("Config".to_string());
    structs.insert("State".to_string());

    // Act
    let metrics = PrivateFunctionMetrics::new("helper".to_string(), 1, 3, structs, true);

    // Assert
    assert_eq!(metrics.used_private_structs().len(), 2);
    assert!(metrics.used_private_structs().contains("Config"));
    assert!(metrics.used_private_structs().contains("State"));
}

#[test]
fn zero_complexity_and_empty_structs() {
    // Arrange & Act
    let metrics = PrivateFunctionMetrics::new("helper".to_string(), 1, 0, BTreeSet::new(), true);

    // Assert
    assert_eq!(metrics.complexity(), 0);
    assert!(metrics.used_private_structs().is_empty());
}
