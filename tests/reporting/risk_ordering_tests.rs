// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::reporting::file_risk_report::FileRiskReport;
use iceberg4rust::reporting::risk_ordering::RiskOrdering;
use std::cmp::Ordering;

fn report(file: &str, risk: f64) -> FileRiskReport {
    FileRiskReport {
        functions: Vec::new(),
        data_structs: Vec::new(),
        behavioral_structs: Vec::new(),
        package_name: "pkg".to_string(),
        relative_file: file.to_string(),
        effective_loc: 100,
        private_function_count: 4,
        private_complexity_sum: 6,
        data_private_struct_count: 0,
        behavioral_private_struct_count: 0,
        risk_score: risk,
    }
}

#[test]
fn descending_sorts_a_slice_from_the_highest_score_down() {
    // Arrange
    let mut reports = [
        report("src/low.rs", 3.0),
        report("src/high.rs", 30.0),
        report("src/mid.rs", 12.0),
    ];

    // Act
    reports.sort_by(RiskOrdering::descending);

    // Assert
    let order: Vec<&str> = reports
        .iter()
        .map(|report| report.relative_file.as_str())
        .collect();
    assert_eq!(order, vec!["src/high.rs", "src/mid.rs", "src/low.rs"]);
}

#[test]
fn descending_with_a_higher_score_on_the_left_returns_less() {
    // Arrange & Act
    let ordering = RiskOrdering::descending(&report("src/a.rs", 20.0), &report("src/b.rs", 10.0));

    // Assert
    assert_eq!(ordering, Ordering::Less);
}

#[test]
fn descending_with_a_higher_score_on_the_right_returns_greater() {
    // Arrange & Act
    let ordering = RiskOrdering::descending(&report("src/a.rs", 10.0), &report("src/b.rs", 20.0));

    // Assert
    assert_eq!(ordering, Ordering::Greater);
}

#[test]
fn descending_with_an_identical_score_and_path_returns_equal() {
    // Arrange & Act
    let ordering = RiskOrdering::descending(&report("src/a.rs", 15.0), &report("src/a.rs", 15.0));

    // Assert
    assert_eq!(ordering, Ordering::Equal);
}

// A NaN score cannot be compared, and `partial_cmp` returns None for it. The
// comparator has to yield a total order regardless or `sort_by` may panic.
#[test]
fn descending_with_an_incomparable_score_falls_back_to_the_file_path() {
    // Arrange & Act
    let ordering =
        RiskOrdering::descending(&report("src/a.rs", f64::NAN), &report("src/z.rs", 15.0));

    // Assert
    assert_eq!(ordering, Ordering::Less);
}

#[test]
fn descending_with_equal_scores_orders_by_file_path() {
    // Arrange & Act
    let ordering = RiskOrdering::descending(&report("src/a.rs", 15.0), &report("src/z.rs", 15.0));

    // Assert
    assert_eq!(ordering, Ordering::Less);
}
