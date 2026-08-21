// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::reporting::file_risk_report::FileRiskReport;
use iceberg4rust::reporting::report_printer::ReportPrinter;
use std::panic::AssertUnwindSafe;
use std::panic::catch_unwind;

fn report(file: &str, risk: f64) -> FileRiskReport {
    FileRiskReport {
        functions: Vec::new(),
        data_structs: Vec::new(),
        behavioral_structs: Vec::new(),
        package_name: "node".to_string(),
        relative_file: file.to_string(),
        effective_loc: 100,
        private_function_count: 5,
        private_complexity_sum: 7,
        data_private_struct_count: 1,
        behavioral_private_struct_count: 2,
        risk_score: risk,
    }
}

fn report_in(package: &str, file: &str, risk: f64) -> FileRiskReport {
    FileRiskReport {
        functions: Vec::new(),
        data_structs: Vec::new(),
        behavioral_structs: Vec::new(),
        package_name: package.to_string(),
        relative_file: file.to_string(),
        effective_loc: 100,
        private_function_count: 5,
        private_complexity_sum: 7,
        data_private_struct_count: 1,
        behavioral_private_struct_count: 2,
        risk_score: risk,
    }
}

#[test]
fn has_offenders_ignores_the_top_limit() {
    // Arrange
    let printer = ReportPrinter::new(20.0, 1);
    let reports = vec![
        report("src/a.rs", 30.0),
        report("src/b.rs", 29.0),
        report("src/c.rs", 28.0),
    ];

    // Act
    let offenders = printer.has_offenders(&reports);

    // Assert
    assert!(offenders);
}

#[test]
fn has_offenders_with_a_score_above_the_threshold_returns_true() {
    // Arrange
    let printer = ReportPrinter::new(20.0, 20);

    // Act
    let offenders = printer.has_offenders(&[report("src/a.rs", 20.1)]);

    // Assert
    assert!(offenders);
}

#[test]
fn has_offenders_with_a_score_exactly_at_the_threshold_returns_true() {
    // Arrange
    let printer = ReportPrinter::new(20.0, 20);

    // Act
    let offenders = printer.has_offenders(&[report("src/a.rs", 20.0)]);

    // Assert
    assert!(offenders);
}

#[test]
fn has_offenders_with_every_score_below_the_threshold_returns_false() {
    // Arrange
    let printer = ReportPrinter::new(20.0, 20);

    // Act
    let offenders = printer.has_offenders(&[report("src/a.rs", 19.9), report("src/b.rs", 3.0)]);

    // Assert
    assert!(!offenders);
}

#[test]
fn has_offenders_with_no_reports_returns_false() {
    // Arrange
    let printer = ReportPrinter::new(20.0, 20);

    // Act
    let offenders = printer.has_offenders(&[]);

    // Assert
    assert!(!offenders);
}

#[test]
fn print_report_no_visible_files_does_not_panic() {
    // Arrange
    let reports: Vec<FileRiskReport> = vec![];
    let printer = ReportPrinter::new(10.0, 10);

    // Act & Assert
    let output = catch_unwind(AssertUnwindSafe(|| {
        printer.print(&reports);
    }));
    assert!(output.is_ok());
}

#[test]
fn print_report_with_reports_does_not_panic() {
    // Arrange
    let reports = vec![FileRiskReport {
        functions: Vec::new(),
        data_structs: Vec::new(),
        behavioral_structs: Vec::new(),
        package_name: "pkg".to_string(),
        relative_file: "src/lib.rs".to_string(),
        effective_loc: 50,
        private_function_count: 3,
        private_complexity_sum: 8,
        data_private_struct_count: 1,
        behavioral_private_struct_count: 0,
        risk_score: 15.5,
    }];
    let printer = ReportPrinter::new(10.0, 10);

    // Act & Assert
    let output = catch_unwind(AssertUnwindSafe(|| {
        printer.print(&reports);
    }));
    assert!(output.is_ok());
}

// `--top` has to cut the lowest-risk rows, not the rows of whichever package
// happened to be passed last. Scanning a workspace with a low-risk package
// listed first used to hide the highest-risk file in the whole repository.
#[test]
fn select_visible_applies_top_to_the_highest_risk_files_across_packages() {
    // Arrange
    let printer = ReportPrinter::new(0.0, 2);
    let reports = vec![
        report_in("execution-types", "src/state_root.rs", 2.50),
        report_in("execution-types", "src/block.rs", 2.13),
        report_in("evm", "src/slot_analyzer.rs", 27.60),
    ];

    // Act
    let visible = printer.select_visible(&reports);

    // Assert
    let order: Vec<&str> = visible
        .iter()
        .map(|report| report.relative_file.as_str())
        .collect();
    assert_eq!(order, vec!["src/slot_analyzer.rs", "src/state_root.rs"]);
}

#[test]
fn select_visible_breaks_equal_risk_ties_by_file_path() {
    // Arrange
    let printer = ReportPrinter::new(0.0, 10);
    let reports = vec![
        report_in("node", "src/z.rs", 9.0),
        report_in("evm", "src/a.rs", 9.0),
    ];

    // Act
    let visible = printer.select_visible(&reports);

    // Assert
    let order: Vec<&str> = visible
        .iter()
        .map(|report| report.relative_file.as_str())
        .collect();
    assert_eq!(order, vec!["src/a.rs", "src/z.rs"]);
}

// The runner appends one package's reports after another, so a slice arriving
// here is ordered only within each package. The table has to rank globally or
// it reads as a ranking it is not.
#[test]
fn select_visible_orders_reports_from_every_package_by_risk_descending() {
    // Arrange
    let printer = ReportPrinter::new(0.0, 10);
    let reports = vec![
        report_in("execution-types", "src/state_root.rs", 2.50),
        report_in("evm", "src/slot_analyzer.rs", 27.60),
        report_in("node", "src/validation.rs", 26.89),
    ];

    // Act
    let visible = printer.select_visible(&reports);

    // Assert
    let order: Vec<&str> = visible
        .iter()
        .map(|report| report.relative_file.as_str())
        .collect();
    assert_eq!(
        order,
        vec![
            "src/slot_analyzer.rs",
            "src/validation.rs",
            "src/state_root.rs"
        ]
    );
}

#[test]
fn select_visible_reports_empty_input_returns_empty() {
    // Arrange
    let reports: Vec<FileRiskReport> = vec![];
    let printer = ReportPrinter::new(10.0, 10);

    // Act
    let visible = printer.select_visible(&reports);

    // Assert
    assert!(visible.is_empty());
}

#[test]
fn select_visible_reports_threshold_filters_lower_scores() {
    // Arrange
    let reports = vec![
        FileRiskReport {
            functions: Vec::new(),
            data_structs: Vec::new(),
            behavioral_structs: Vec::new(),
            package_name: "demo-node".to_string(),
            relative_file: "src/a.rs".to_string(),
            effective_loc: 10,
            private_function_count: 1,
            private_complexity_sum: 2,
            data_private_struct_count: 0,
            behavioral_private_struct_count: 0,
            risk_score: 12.0,
        },
        FileRiskReport {
            functions: Vec::new(),
            data_structs: Vec::new(),
            behavioral_structs: Vec::new(),
            package_name: "demo-node".to_string(),
            relative_file: "src/b.rs".to_string(),
            effective_loc: 20,
            private_function_count: 2,
            private_complexity_sum: 4,
            data_private_struct_count: 1,
            behavioral_private_struct_count: 0,
            risk_score: 8.0,
        },
    ];
    let printer = ReportPrinter::new(10.0, 10);

    // Act
    let visible = printer.select_visible(&reports);

    // Assert
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].relative_file, "src/a.rs");
}

#[test]
fn select_visible_reports_top_limits_results() {
    // Arrange
    let reports = vec![
        FileRiskReport {
            functions: Vec::new(),
            data_structs: Vec::new(),
            behavioral_structs: Vec::new(),
            package_name: "pkg".to_string(),
            relative_file: "src/a.rs".to_string(),
            effective_loc: 10,
            private_function_count: 1,
            private_complexity_sum: 1,
            data_private_struct_count: 0,
            behavioral_private_struct_count: 0,
            risk_score: 20.0,
        },
        FileRiskReport {
            functions: Vec::new(),
            data_structs: Vec::new(),
            behavioral_structs: Vec::new(),
            package_name: "pkg".to_string(),
            relative_file: "src/b.rs".to_string(),
            effective_loc: 10,
            private_function_count: 1,
            private_complexity_sum: 1,
            data_private_struct_count: 0,
            behavioral_private_struct_count: 0,
            risk_score: 15.0,
        },
        FileRiskReport {
            functions: Vec::new(),
            data_structs: Vec::new(),
            behavioral_structs: Vec::new(),
            package_name: "pkg".to_string(),
            relative_file: "src/c.rs".to_string(),
            effective_loc: 10,
            private_function_count: 1,
            private_complexity_sum: 1,
            data_private_struct_count: 0,
            behavioral_private_struct_count: 0,
            risk_score: 10.0,
        },
    ];
    let printer = ReportPrinter::new(0.0, 2);

    // Act
    let visible = printer.select_visible(&reports);

    // Assert
    assert_eq!(visible.len(), 2);
}
