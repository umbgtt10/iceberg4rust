// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::reporting::file_metrics::FileMetrics;
use iceberg4rust::reporting::file_risk_report::FileRiskReport;
use iceberg4rust::reporting::report_selection::ReportSelection;

fn report(file: &str, risk: f64) -> FileRiskReport {
    FileRiskReport::new(
        "node".to_string(),
        file.to_string(),
        FileMetrics {
            effective_loc: 100,
            private_function_count: 5,
            private_complexity_sum: 7,
            data_private_struct_count: 1,
            behavioral_private_struct_count: 2,
            risk_score: risk,
            ..Default::default()
        },
    )
}

fn report_in(package: &str, file: &str, risk: f64) -> FileRiskReport {
    FileRiskReport::new(
        package.to_string(),
        file.to_string(),
        FileMetrics {
            effective_loc: 100,
            private_function_count: 5,
            private_complexity_sum: 7,
            data_private_struct_count: 1,
            behavioral_private_struct_count: 2,
            risk_score: risk,
            ..Default::default()
        },
    )
}

#[test]
fn has_offenders_ignores_the_top_limit() {
    // Arrange
    let selection = ReportSelection::new(20.0, 1);
    let reports = vec![
        report("src/a.rs", 30.0),
        report("src/b.rs", 29.0),
        report("src/c.rs", 28.0),
    ];

    // Act
    let offenders = selection.has_offenders(&reports);

    // Assert
    assert!(offenders);
}

#[test]
fn has_offenders_with_a_score_above_the_threshold_returns_true() {
    // Arrange
    let selection = ReportSelection::new(20.0, 20);

    // Act
    let offenders = selection.has_offenders(&[report("src/a.rs", 20.1)]);

    // Assert
    assert!(offenders);
}

#[test]
fn has_offenders_with_a_score_exactly_at_the_threshold_returns_true() {
    // Arrange
    let selection = ReportSelection::new(20.0, 20);

    // Act
    let offenders = selection.has_offenders(&[report("src/a.rs", 20.0)]);

    // Assert
    assert!(offenders);
}

#[test]
fn has_offenders_with_every_score_below_the_threshold_returns_false() {
    // Arrange
    let selection = ReportSelection::new(20.0, 20);

    // Act
    let offenders = selection.has_offenders(&[report("src/a.rs", 19.9), report("src/b.rs", 3.0)]);

    // Assert
    assert!(!offenders);
}

#[test]
fn has_offenders_with_no_reports_returns_false() {
    // Arrange
    let selection = ReportSelection::new(20.0, 20);

    // Act
    let offenders = selection.has_offenders(&[]);

    // Assert
    assert!(!offenders);
}

#[test]
fn report_counts_every_scored_file_and_totals_their_risk() {
    // Arrange
    let reports = vec![report("src/a.rs", 30.0), report("src/b.rs", 2.5)];
    let selection = ReportSelection::new(20.0, 1);

    // Act
    let summary = selection.report(&reports);

    // Assert
    assert!((summary.threshold - 20.0).abs() < f64::EPSILON);
    assert_eq!(summary.top, 1);
    assert_eq!(summary.scored_files, 2);
    assert!((summary.total_risk - 32.5).abs() < f64::EPSILON);
}

#[test]
fn report_offers_every_offender_ranked_and_the_top_of_them_as_visible() {
    // Arrange
    let reports = vec![
        report("src/a.rs", 21.0),
        report("src/b.rs", 29.0),
        report("src/c.rs", 3.0),
    ];
    let selection = ReportSelection::new(20.0, 1);

    // Act
    let summary = selection.report(&reports);

    // Assert
    let offenders: Vec<&str> = summary
        .offenders
        .iter()
        .map(|file| file.relative_file())
        .collect();
    let visible: Vec<&str> = summary
        .visible
        .iter()
        .map(|file| file.relative_file())
        .collect();
    assert_eq!(offenders, vec!["src/b.rs", "src/a.rs"]);
    assert_eq!(visible, vec!["src/b.rs"]);
}

// `--top` has to cut the lowest-risk rows, not the rows of whichever package
// happened to be passed last. Scanning a workspace with a low-risk package
// listed first used to hide the highest-risk file in the whole repository.
#[test]
fn select_visible_applies_top_to_the_highest_risk_files_across_packages() {
    // Arrange
    let selection = ReportSelection::new(0.0, 2);
    let reports = vec![
        report_in("execution-types", "src/state_root.rs", 2.50),
        report_in("execution-types", "src/block.rs", 2.13),
        report_in("evm", "src/slot_analyzer.rs", 27.60),
    ];

    // Act
    let visible = selection.select_visible(&reports);

    // Assert
    let order: Vec<&str> = visible
        .iter()
        .map(|report| report.relative_file())
        .collect();
    assert_eq!(order, vec!["src/slot_analyzer.rs", "src/state_root.rs"]);
}

#[test]
fn select_visible_breaks_equal_risk_ties_by_file_path() {
    // Arrange
    let selection = ReportSelection::new(0.0, 10);
    let reports = vec![
        report_in("node", "src/z.rs", 9.0),
        report_in("evm", "src/a.rs", 9.0),
    ];

    // Act
    let visible = selection.select_visible(&reports);

    // Assert
    let order: Vec<&str> = visible
        .iter()
        .map(|report| report.relative_file())
        .collect();
    assert_eq!(order, vec!["src/a.rs", "src/z.rs"]);
}

// The runner appends one package's reports after another, so a slice arriving
// here is ordered only within each package. The table has to rank globally or
// it reads as a ranking it is not.
#[test]
fn select_visible_orders_reports_from_every_package_by_risk_descending() {
    // Arrange
    let selection = ReportSelection::new(0.0, 10);
    let reports = vec![
        report_in("execution-types", "src/state_root.rs", 2.50),
        report_in("evm", "src/slot_analyzer.rs", 27.60),
        report_in("node", "src/validation.rs", 26.89),
    ];

    // Act
    let visible = selection.select_visible(&reports);

    // Assert
    let order: Vec<&str> = visible
        .iter()
        .map(|report| report.relative_file())
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
    let selection = ReportSelection::new(10.0, 10);

    // Act
    let visible = selection.select_visible(&reports);

    // Assert
    assert!(visible.is_empty());
}

#[test]
fn select_visible_reports_threshold_filters_lower_scores() {
    // Arrange
    let reports = vec![
        FileRiskReport::new(
            "demo-node".to_string(),
            "src/a.rs".to_string(),
            FileMetrics {
                effective_loc: 10,
                private_function_count: 1,
                private_complexity_sum: 2,
                risk_score: 12.0,
                ..Default::default()
            },
        ),
        FileRiskReport::new(
            "demo-node".to_string(),
            "src/b.rs".to_string(),
            FileMetrics {
                effective_loc: 20,
                private_function_count: 2,
                private_complexity_sum: 4,
                data_private_struct_count: 1,
                risk_score: 8.0,
                ..Default::default()
            },
        ),
    ];
    let selection = ReportSelection::new(10.0, 10);

    // Act
    let visible = selection.select_visible(&reports);

    // Assert
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].relative_file(), "src/a.rs");
}

#[test]
fn select_visible_reports_top_limits_results() {
    // Arrange
    let reports = vec![
        FileRiskReport::new(
            "pkg".to_string(),
            "src/a.rs".to_string(),
            FileMetrics {
                effective_loc: 10,
                private_function_count: 1,
                private_complexity_sum: 1,
                risk_score: 20.0,
                ..Default::default()
            },
        ),
        FileRiskReport::new(
            "pkg".to_string(),
            "src/b.rs".to_string(),
            FileMetrics {
                effective_loc: 10,
                private_function_count: 1,
                private_complexity_sum: 1,
                risk_score: 15.0,
                ..Default::default()
            },
        ),
        FileRiskReport::new(
            "pkg".to_string(),
            "src/c.rs".to_string(),
            FileMetrics {
                effective_loc: 10,
                private_function_count: 1,
                private_complexity_sum: 1,
                risk_score: 10.0,
                ..Default::default()
            },
        ),
    ];
    let selection = ReportSelection::new(0.0, 2);

    // Act
    let visible = selection.select_visible(&reports);

    // Assert
    assert_eq!(visible.len(), 2);
}
