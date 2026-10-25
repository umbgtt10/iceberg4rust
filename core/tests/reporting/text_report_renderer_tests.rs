// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::reporting::file_metrics::FileMetrics;
use iceberg4rust::reporting::file_risk_report::FileRiskReport;
use iceberg4rust::reporting::report_selection::ReportSelection;
use iceberg4rust::reporting::text_report_renderer::TextReportRenderer;
use iceberg4rust::traits::report_renderer::ReportRenderer;

#[test]
fn render_with_a_report_lists_it_in_the_table_the_detail_and_the_summary() {
    // Arrange
    let reports = vec![FileRiskReport::new(
        "pkg".to_string(),
        "src/lib.rs".to_string(),
        FileMetrics {
            effective_loc: 50,
            private_function_count: 3,
            private_complexity_sum: 8,
            data_private_struct_count: 1,
            risk_score: 15.5,
            ..Default::default()
        },
    )];
    let report = ReportSelection::new(10.0, 10).report(&reports);

    // Act
    let output = TextReportRenderer::new()
        .render(&report)
        .expect("text always renders");

    // Assert
    assert_eq!(
        output,
        concat!(
            "cargo-iceberg4rust report\n",
            "\n",
            "package  file          loc    private_fns  private_complexity  data_structs  behavioral_structs        risk\n",
            "-------  ----------  -----  -------------  ------------------  ------------  ------------------  ----------\n",
            "pkg      src/lib.rs     50              3                   8             1                   0       15.50\n",
            "\n",
            "Offender detail:\n",
            "\n",
            "  pkg  src/lib.rs  (risk 15.50, 3 private fns, complexity 8)\n",
            "      line  complexity  function\n",
            "\n",
            "summary: scored_files=1 visible_files=1 threshold=10.00 top=10 total_risk=15.50\n",
        )
    );
}

#[test]
fn render_with_no_visible_files_says_none_was_found() {
    // Arrange
    let reports: Vec<FileRiskReport> = vec![];
    let report = ReportSelection::new(10.0, 10).report(&reports);

    // Act
    let output = TextReportRenderer::new()
        .render(&report)
        .expect("text always renders");

    // Assert
    assert_eq!(
        output,
        "cargo-iceberg4rust report\n\nNo files with private implementation risk at or above 10.00 were found.\n"
    );
}
