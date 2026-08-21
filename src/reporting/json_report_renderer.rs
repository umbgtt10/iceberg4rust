// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use serde::Serialize;

use crate::reporting::file_risk_report::FileRiskReport;
use crate::reporting::report_printer::ReportPrinter;
use crate::reporting::risk_ordering::RiskOrdering;
use serde_json::to_string_pretty;

#[derive(Serialize)]
struct JsonReport<'a> {
    threshold: f64,
    scored_files: usize,
    visible_files: usize,
    total_risk: f64,
    files: Vec<&'a FileRiskReport>,
}

pub struct JsonReportRenderer {
    threshold: f64,
}

impl JsonReportRenderer {
    #[must_use]
    pub const fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    pub fn render(&self, reports: &[FileRiskReport]) -> Result<String> {
        // `top` is a display convenience for the table; a machine consumer wants
        // every file at or above the threshold, so no limit is applied here.
        let printer = ReportPrinter::new(self.threshold, usize::MAX);
        let mut files = printer.select_visible(reports);
        // Ordering is stated by this contract rather than inherited from the
        // caller, so a consumer can rely on it without knowing who built the
        // slice or how the printer happens to order today.
        files.sort_by(|left, right| RiskOrdering::descending(left, right));

        let report = JsonReport {
            threshold: self.threshold,
            scored_files: reports.len(),
            visible_files: files.len(),
            total_risk: reports.iter().map(|report| report.risk_score).sum(),
            files,
        };

        to_string_pretty(&report).context("failed to render report as JSON")
    }
}
