// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::reporting::file_risk_report::FileRiskReport;

#[test]
fn file_risk_report_clone_produces_independent_copy() {
    // Arrange
    let report = FileRiskReport {
        functions: Vec::new(),
        data_structs: Vec::new(),
        behavioral_structs: Vec::new(),
        package_name: "pkg".to_string(),
        relative_file: "src/main.rs".to_string(),
        effective_loc: 100,
        private_function_count: 5,
        private_complexity_sum: 20,
        data_private_struct_count: 2,
        behavioral_private_struct_count: 0,
        risk_score: 12.0,
    };

    // Act
    let cloned = report.clone();

    // Assert
    assert_eq!(cloned, report);
}

#[test]
fn file_risk_report_holds_provided_values() {
    // Arrange & Act
    let report = FileRiskReport {
        functions: Vec::new(),
        data_structs: Vec::new(),
        behavioral_structs: Vec::new(),
        package_name: "pkg".to_string(),
        relative_file: "src/lib.rs".to_string(),
        effective_loc: 50,
        private_function_count: 3,
        private_complexity_sum: 12,
        data_private_struct_count: 1,
        behavioral_private_struct_count: 2,
        risk_score: 25.5,
    };

    // Assert
    assert_eq!(report.package_name, "pkg");
    assert_eq!(report.relative_file, "src/lib.rs");
    assert_eq!(report.effective_loc, 50);
    assert_eq!(report.private_function_count, 3);
    assert_eq!(report.private_complexity_sum, 12);
    assert_eq!(report.data_private_struct_count, 1);
    assert_eq!(report.behavioral_private_struct_count, 2);
    assert!((report.risk_score - 25.5).abs() < f64::EPSILON);
}

#[test]
fn file_risk_report_partial_eq_compares_all_fields() {
    // Arrange
    let a = FileRiskReport {
        functions: Vec::new(),
        data_structs: Vec::new(),
        behavioral_structs: Vec::new(),
        package_name: "pkg".to_string(),
        relative_file: "src/lib.rs".to_string(),
        effective_loc: 50,
        private_function_count: 3,
        private_complexity_sum: 12,
        data_private_struct_count: 1,
        behavioral_private_struct_count: 2,
        risk_score: 25.5,
    };
    let b = FileRiskReport {
        functions: Vec::new(),
        data_structs: Vec::new(),
        behavioral_structs: Vec::new(),
        package_name: "pkg".to_string(),
        relative_file: "src/lib.rs".to_string(),
        effective_loc: 50,
        private_function_count: 3,
        private_complexity_sum: 12,
        data_private_struct_count: 1,
        behavioral_private_struct_count: 2,
        risk_score: 25.5,
    };
    let c = FileRiskReport {
        functions: Vec::new(),
        data_structs: Vec::new(),
        behavioral_structs: Vec::new(),
        package_name: "other".to_string(),
        ..a.clone()
    };

    // Act & Assert
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn file_risk_report_zero_risk_score_is_stored_correctly() {
    // Arrange & Act
    let report = FileRiskReport {
        functions: Vec::new(),
        data_structs: Vec::new(),
        behavioral_structs: Vec::new(),
        package_name: "pkg".to_string(),
        relative_file: "src/empty.rs".to_string(),
        effective_loc: 0,
        private_function_count: 0,
        private_complexity_sum: 0,
        data_private_struct_count: 0,
        behavioral_private_struct_count: 0,
        risk_score: 0.0,
    };

    // Assert
    assert!((report.risk_score - 0.0).abs() < f64::EPSILON);
}
