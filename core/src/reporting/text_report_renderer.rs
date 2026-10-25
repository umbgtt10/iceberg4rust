// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use core::fmt::Write;

use crate::reporting::file_risk_report::FileRiskReport;
use crate::reporting::offender_detail_renderer::OffenderDetailRenderer;
use crate::reporting::render_failure::RenderFailure;
use crate::reporting::risk_report::RiskReport;
use crate::traits::report_renderer::ReportRenderer;

// The report for a reader: the table of the visible files, what is inside each,
// and a summary line. Pure string building; the choice of files is the
// report's.
#[derive(Debug, Clone, Copy, Default)]
pub struct TextReportRenderer;

impl TextReportRenderer {
    pub fn new() -> Self {
        Self
    }

    fn render_table(visible: &[&FileRiskReport]) -> String {
        let package_width = visible
            .iter()
            .map(|report| report.package_name().len())
            .max()
            .unwrap_or(7)
            .max("package".len());
        let file_width = visible
            .iter()
            .map(|report| report.relative_file().len())
            .max()
            .unwrap_or(4)
            .max("file".len());

        let mut out = String::new();
        let _ = writeln!(
            out,
            "{:<package_width$}  {:<file_width$}  {:>5}  {:>13}  {:>18}  {:>12}  {:>18}  {:>10}",
            "package",
            "file",
            "loc",
            "private_fns",
            "private_complexity",
            "data_structs",
            "behavioral_structs",
            "risk",
        );
        let _ = writeln!(
            out,
            "{}  {}  {}  {}  {}  {}  {}  {}",
            "-".repeat(package_width),
            "-".repeat(file_width),
            "-".repeat(5),
            "-".repeat(13),
            "-".repeat(18),
            "-".repeat(12),
            "-".repeat(18),
            "-".repeat(10),
        );

        for report in visible {
            let _ = writeln!(
                out,
                "{:<package_width$}  {:<file_width$}  {:>5}  {:>13}  {:>18}  {:>12}  {:>18}  {:>10.2}",
                report.package_name(),
                report.relative_file(),
                report.effective_loc(),
                report.private_function_count(),
                report.private_complexity_sum(),
                report.data_private_struct_count(),
                report.behavioral_private_struct_count(),
                report.risk_score(),
            );
        }
        out
    }
}

impl ReportRenderer for TextReportRenderer {
    fn render(&self, report: &RiskReport<'_>) -> Result<String, RenderFailure> {
        let mut out = String::from("cargo-iceberg4rust report\n\n");

        if report.visible.is_empty() {
            let _ = writeln!(
                out,
                "No files with private implementation risk at or above {:.2} were found.",
                report.threshold,
            );
            return Ok(out);
        }

        out.push_str(&Self::render_table(&report.visible));
        out.push('\n');
        out.push_str(&OffenderDetailRenderer::render(&report.visible));
        out.push('\n');
        let _ = writeln!(
            out,
            "summary: scored_files={} visible_files={} threshold={:.2} top={} total_risk={:.2}",
            report.scored_files,
            report.visible.len(),
            report.threshold,
            report.top,
            report.total_risk,
        );
        Ok(out)
    }
}
