// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::reporting::file_metrics::FileMetrics;
use iceberg4rust::reporting::file_risk_report::FileRiskReport;
use iceberg4rust::reporting::private_function_report::PrivateFunctionReport;

#[test]
fn file_risk_report_clone_produces_independent_copy() {
    // Arrange
    let report = FileRiskReport::new(
        "pkg".to_string(),
        "src/main.rs".to_string(),
        FileMetrics {
            effective_loc: 100,
            private_function_count: 5,
            private_complexity_sum: 20,
            data_private_struct_count: 2,
            risk_score: 12.0,
            ..Default::default()
        },
    );

    // Act
    let cloned = report.clone();

    // Assert
    assert_eq!(cloned, report);
}

#[test]
fn file_risk_report_holds_provided_values() {
    // Arrange & Act
    let report = FileRiskReport::new(
        "pkg".to_string(),
        "src/lib.rs".to_string(),
        FileMetrics {
            effective_loc: 50,
            private_function_count: 3,
            private_complexity_sum: 12,
            data_private_struct_count: 1,
            behavioral_private_struct_count: 2,
            risk_score: 25.5,
            ..Default::default()
        },
    );

    // Assert
    assert_eq!(report.package_name(), "pkg");
    assert_eq!(report.relative_file(), "src/lib.rs");
    assert_eq!(report.effective_loc(), 50);
    assert_eq!(report.private_function_count(), 3);
    assert_eq!(report.private_complexity_sum(), 12);
    assert_eq!(report.data_private_struct_count(), 1);
    assert_eq!(report.behavioral_private_struct_count(), 2);
    assert!((report.risk_score() - 25.5).abs() < f64::EPSILON);
}

#[test]
fn file_risk_report_partial_eq_compares_all_fields() {
    // Arrange
    let a = FileRiskReport::new(
        "pkg".to_string(),
        "src/lib.rs".to_string(),
        FileMetrics {
            effective_loc: 50,
            private_function_count: 3,
            private_complexity_sum: 12,
            data_private_struct_count: 1,
            behavioral_private_struct_count: 2,
            risk_score: 25.5,
            ..Default::default()
        },
    );
    let b = FileRiskReport::new(
        "pkg".to_string(),
        "src/lib.rs".to_string(),
        FileMetrics {
            effective_loc: 50,
            private_function_count: 3,
            private_complexity_sum: 12,
            data_private_struct_count: 1,
            behavioral_private_struct_count: 2,
            risk_score: 25.5,
            ..Default::default()
        },
    );
    let c = FileRiskReport::new(
        "other".to_string(),
        "src/lib.rs".to_string(),
        FileMetrics {
            effective_loc: 50,
            private_function_count: 3,
            private_complexity_sum: 12,
            data_private_struct_count: 1,
            behavioral_private_struct_count: 2,
            risk_score: 25.5,
            ..Default::default()
        },
    );

    // Act & Assert
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn file_risk_report_zero_risk_score_is_stored_correctly() {
    // Arrange & Act
    let report = FileRiskReport::new(
        "pkg".to_string(),
        "src/empty.rs".to_string(),
        FileMetrics::default(),
    );

    // Assert
    assert!((report.risk_score() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn new_with_functions_and_helper_structs_keeps_them() {
    // Arrange & Act
    let report = FileRiskReport::new(
        "pkg".to_string(),
        "src/lib.rs".to_string(),
        FileMetrics {
            functions: vec![PrivateFunctionReport {
                name: "decode".to_string(),
                line: 42,
                complexity: 7,
                is_hidden: true,
            }],
            data_structs: vec!["Payload".to_string()],
            behavioral_structs: vec!["Encoder".to_string()],
            ..Default::default()
        },
    );

    // Assert
    assert_eq!(
        report.functions(),
        [PrivateFunctionReport {
            name: "decode".to_string(),
            line: 42,
            complexity: 7,
            is_hidden: true,
        }]
    );
    assert_eq!(report.data_structs(), ["Payload".to_string()]);
    assert_eq!(report.behavioral_structs(), ["Encoder".to_string()]);
}
