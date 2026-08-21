// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::reporting::file_metrics::FileMetrics;

#[test]
fn file_metrics_holds_provided_values() {
    // Arrange & Act
    let metrics = FileMetrics {
        functions: Vec::new(),
        data_structs: Vec::new(),
        behavioral_structs: Vec::new(),
        effective_loc: 30,
        private_function_count: 2,
        private_complexity_sum: 8,
        data_private_struct_count: 0,
        behavioral_private_struct_count: 1,
        risk_score: 8.5,
    };

    // Assert
    assert_eq!(metrics.effective_loc, 30);
    assert_eq!(metrics.private_function_count, 2);
    assert_eq!(metrics.private_complexity_sum, 8);
    assert_eq!(metrics.data_private_struct_count, 0);
    assert_eq!(metrics.behavioral_private_struct_count, 1);
    assert!((metrics.risk_score - 8.5).abs() < f64::EPSILON);
}

#[test]
fn file_metrics_partial_eq_compares_all_fields() {
    // Arrange
    let a = FileMetrics {
        functions: Vec::new(),
        data_structs: Vec::new(),
        behavioral_structs: Vec::new(),
        effective_loc: 30,
        private_function_count: 2,
        private_complexity_sum: 8,
        data_private_struct_count: 0,
        behavioral_private_struct_count: 1,
        risk_score: 8.5,
    };
    let b = FileMetrics {
        functions: Vec::new(),
        data_structs: Vec::new(),
        behavioral_structs: Vec::new(),
        effective_loc: 30,
        private_function_count: 2,
        private_complexity_sum: 8,
        data_private_struct_count: 0,
        behavioral_private_struct_count: 1,
        risk_score: 8.5,
    };
    let c = FileMetrics {
        functions: Vec::new(),
        data_structs: Vec::new(),
        behavioral_structs: Vec::new(),
        effective_loc: 99,
        ..a.clone()
    };

    // Act & Assert
    assert_eq!(a, b);
    assert_ne!(a, c);
}
