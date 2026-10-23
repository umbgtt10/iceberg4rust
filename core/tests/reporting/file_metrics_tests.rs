// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::reporting::file_metrics::FileMetrics;

#[test]
fn file_metrics_holds_provided_values() {
    // Arrange & Act
    let metrics = FileMetrics::default()
        .with_effective_loc(30)
        .with_private_function_count(2)
        .with_private_complexity_sum(8)
        .with_behavioral_private_struct_count(1)
        .with_risk_score(8.5);

    // Assert
    assert_eq!(metrics.effective_loc(), 30);
    assert_eq!(metrics.private_function_count(), 2);
    assert_eq!(metrics.private_complexity_sum(), 8);
    assert_eq!(metrics.data_private_struct_count(), 0);
    assert_eq!(metrics.behavioral_private_struct_count(), 1);
    assert!((metrics.risk_score() - 8.5).abs() < f64::EPSILON);
}

#[test]
fn file_metrics_partial_eq_compares_all_fields() {
    // Arrange
    let a = FileMetrics::default()
        .with_effective_loc(30)
        .with_private_function_count(2)
        .with_private_complexity_sum(8)
        .with_behavioral_private_struct_count(1)
        .with_risk_score(8.5);
    let b = FileMetrics::default()
        .with_effective_loc(30)
        .with_private_function_count(2)
        .with_private_complexity_sum(8)
        .with_behavioral_private_struct_count(1)
        .with_risk_score(8.5);
    let c = a
        .clone()
        .with_functions(Vec::new())
        .with_data_structs(Vec::new())
        .with_behavioral_structs(Vec::new())
        .with_effective_loc(99);

    // Act & Assert
    assert_eq!(a, b);
    assert_ne!(a, c);
}
