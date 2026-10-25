// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::reporting::file_risk_report::FileRiskReport;
use iceberg4rust::reporting::private_function_report::PrivateFunctionReport;
use iceberg4rust::reporting::render_failure::RenderFailure;
use iceberg4rust::reporting::risk_report::RiskReport;
use iceberg4rust::traits::report_renderer::ReportRenderer;
use serde::Serialize;
use serde_json::to_string_pretty;

// The machine surface. Its records are its own: the core's carry no serde, so
// the shape of the JSON is written down here, field for field in the order the
// document has always had.
#[derive(Serialize)]
struct JsonReport<'a> {
    pub threshold: f64,
    pub scored_files: usize,
    pub visible_files: usize,
    pub total_risk: f64,
    pub files: Vec<JsonFile<'a>>,
}

#[derive(Serialize)]
struct JsonFile<'a> {
    pub functions: Vec<JsonFunction<'a>>,
    pub data_structs: &'a [String],
    pub behavioral_structs: &'a [String],
    pub package_name: &'a str,
    pub relative_file: &'a str,
    pub effective_loc: usize,
    pub private_function_count: usize,
    pub private_complexity_sum: u32,
    pub data_private_struct_count: usize,
    pub behavioral_private_struct_count: usize,
    pub risk_score: f64,
}

#[derive(Serialize)]
struct JsonFunction<'a> {
    pub name: &'a str,
    pub line: usize,
    pub complexity: u32,
    pub is_hidden: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct JsonReportRenderer;

impl JsonReportRenderer {
    pub fn new() -> Self {
        Self
    }

    fn file(report: &FileRiskReport) -> JsonFile<'_> {
        JsonFile {
            functions: report.functions().iter().map(Self::function).collect(),
            data_structs: report.data_structs(),
            behavioral_structs: report.behavioral_structs(),
            package_name: report.package_name(),
            relative_file: report.relative_file(),
            effective_loc: report.effective_loc(),
            private_function_count: report.private_function_count(),
            private_complexity_sum: report.private_complexity_sum(),
            data_private_struct_count: report.data_private_struct_count(),
            behavioral_private_struct_count: report.behavioral_private_struct_count(),
            risk_score: report.risk_score(),
        }
    }

    fn function(function: &PrivateFunctionReport) -> JsonFunction<'_> {
        JsonFunction {
            name: &function.name,
            line: function.line,
            complexity: function.complexity,
            is_hidden: function.is_hidden,
        }
    }
}

impl ReportRenderer for JsonReportRenderer {
    // Every offender, not the `top` a reader sees: a machine consumer wants
    // them all, ranked as the report ranks them.
    fn render(&self, report: &RiskReport<'_>) -> Result<String, RenderFailure> {
        let document = JsonReport {
            threshold: report.threshold,
            scored_files: report.scored_files,
            visible_files: report.offenders.len(),
            total_risk: report.total_risk,
            files: report
                .offenders
                .iter()
                .map(|file| Self::file(file))
                .collect(),
        };
        to_string_pretty(&document)
            .map(|json| format!("{json}\n"))
            .map_err(|error| RenderFailure {
                cause: Box::new(error),
            })
    }
}
