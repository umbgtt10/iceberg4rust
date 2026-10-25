// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::reporting::file_metrics::FileMetrics;

#[test]
fn file_metrics_partial_eq_compares_all_fields() {
    // Arrange
    let a = FileMetrics {
        effective_loc: 30,
        private_function_count: 2,
        private_complexity_sum: 8,
        behavioral_private_struct_count: 1,
        risk_score: 8.5,
        ..Default::default()
    };
    let b = FileMetrics {
        effective_loc: 30,
        private_function_count: 2,
        private_complexity_sum: 8,
        behavioral_private_struct_count: 1,
        risk_score: 8.5,
        ..Default::default()
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
