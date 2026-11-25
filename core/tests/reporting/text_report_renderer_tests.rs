// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::reporting::file_metrics::FileMetrics;
use iceberg4rust::reporting::file_risk_report::FileRiskReport;
use iceberg4rust::reporting::private_function_report::PrivateFunctionReport;
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
            functions: vec![
                PrivateFunctionReport {
                    name: "decode".to_string(),
                    line: 12,
                    complexity: 1,
                    is_hidden: true,
                },
                PrivateFunctionReport {
                    name: "encode".to_string(),
                    line: 30,
                    complexity: 3,
                    is_hidden: true,
                },
                PrivateFunctionReport {
                    name: "verify".to_string(),
                    line: 41,
                    complexity: 4,
                    is_hidden: true,
                },
            ],
            data_structs: vec!["Payload".to_string()],
            effective_loc: 50,
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
            "        41           4  verify\n",
            "        30           3  encode\n",
            "        12           1  decode\n",
            "    helper structs: data=[Payload]  behavioral=[]\n",
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
