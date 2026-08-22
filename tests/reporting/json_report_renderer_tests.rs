// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::reporting::file_risk_report::FileRiskReport;
use iceberg4rust::reporting::json_report_renderer::JsonReportRenderer;
use serde_json::Value;
use serde_json::from_str;

fn rendered(reports: &[FileRiskReport], threshold: f64) -> Value {
    let json = JsonReportRenderer::new(threshold)
        .render(reports)
        .expect("render should succeed");
    from_str(&json).expect("output should be valid json")
}

fn report(package: &str, file: &str, risk: f64) -> FileRiskReport {
    FileRiskReport {
        functions: Vec::new(),
        data_structs: Vec::new(),
        behavioral_structs: Vec::new(),
        package_name: package.to_string(),
        relative_file: file.to_string(),
        effective_loc: 223,
        private_function_count: 63,
        private_complexity_sum: 12,
        data_private_struct_count: 1,
        behavioral_private_struct_count: 2,
        risk_score: risk,
    }
}

#[test]
fn render_carries_every_metric_field_for_a_file() {
    // Arrange
    let reports = vec![report("node", "src/a.rs", 24.5)];

    // Act
    let value = rendered(&reports, 20.0);

    // Assert
    let file = &value["files"][0];
    assert_eq!(file["package_name"], "node");
    assert_eq!(file["relative_file"], "src/a.rs");
    assert_eq!(file["effective_loc"], 223);
    assert_eq!(file["private_function_count"], 63);
    assert_eq!(file["private_complexity_sum"], 12);
    assert_eq!(file["data_private_struct_count"], 1);
    assert_eq!(file["behavioral_private_struct_count"], 2);
    assert_eq!(file["risk_score"], 24.5);
}

#[test]
fn render_orders_files_by_descending_risk() {
    // Arrange
    let reports = vec![
        report("node", "src/lower.rs", 21.0),
        report("node", "src/higher.rs", 26.0),
    ];

    // Act
    let value = rendered(&reports, 20.0);

    // Assert
    let files = value["files"].as_array().expect("files array");
    assert_eq!(files[0]["relative_file"], "src/higher.rs");
    assert_eq!(files[1]["relative_file"], "src/lower.rs");
}

#[test]
fn render_produces_valid_json_for_an_empty_report_set() {
    // Arrange & Act
    let value = rendered(&[], 20.0);

    // Assert
    assert_eq!(value["files"].as_array().expect("files array").len(), 0);
}

#[test]
fn render_reports_only_files_at_or_above_the_threshold() {
    // Arrange
    let reports = vec![
        report("node", "src/high.rs", 24.0),
        report("node", "src/low.rs", 3.0),
    ];

    // Act
    let value = rendered(&reports, 20.0);

    // Assert
    let files = value["files"].as_array().expect("files array");
    assert_eq!(files.len(), 1);
    assert_eq!(files[0]["relative_file"], "src/high.rs");
}

#[test]
fn render_states_the_threshold_it_applied() {
    // Arrange & Act
    let value = rendered(&[], 20.0);

    // Assert
    assert_eq!(value["threshold"], 20.0);
}

#[test]
fn render_summarises_scored_and_visible_counts_separately() {
    // Arrange
    let reports = vec![
        report("node", "src/high.rs", 24.0),
        report("node", "src/low.rs", 3.0),
    ];

    // Act
    let value = rendered(&reports, 20.0);

    // Assert
    assert_eq!(value["scored_files"], 2);
    assert_eq!(value["visible_files"], 1);
}

#[test]
fn render_totals_risk_across_every_scored_file_not_just_the_visible_ones() {
    // Arrange
    let reports = vec![
        report("node", "src/high.rs", 24.0),
        report("node", "src/low.rs", 3.0),
    ];

    // Act
    let value = rendered(&reports, 20.0);

    // Assert
    assert_eq!(value["total_risk"], 27.0);
}
