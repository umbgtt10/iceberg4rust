// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::reporting::file_risk_report::FileRiskReport;
use crate::reporting::offender_detail_renderer::OffenderDetailRenderer;
use crate::reporting::risk_ordering::RiskOrdering;

pub struct ReportPrinter {
    threshold: f64,
    top: usize,
}

impl ReportPrinter {
    pub fn new(threshold: f64, top: usize) -> Self {
        Self { threshold, top }
    }

    pub fn print(&self, reports: &[FileRiskReport]) {
        let visible = self.select_visible(reports);

        println!("cargo-iceberg4rust report");
        println!();

        if visible.is_empty() {
            println!(
                "No files with private implementation risk at or above {:.2} were found.",
                self.threshold,
            );
            return;
        }

        let package_width = visible
            .iter()
            .map(|report| report.package_name.len())
            .max()
            .unwrap_or(7)
            .max("package".len());
        let file_width = visible
            .iter()
            .map(|report| report.relative_file.len())
            .max()
            .unwrap_or(4)
            .max("file".len());

        println!(
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
        println!(
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

        for report in &visible {
            println!(
                "{:<package_width$}  {:<file_width$}  {:>5}  {:>13}  {:>18}  {:>12}  {:>18}  {:>10.2}",
                report.package_name,
                report.relative_file,
                report.effective_loc,
                report.private_function_count,
                report.private_complexity_sum,
                report.data_private_struct_count,
                report.behavioral_private_struct_count,
                report.risk_score,
            );
        }

        println!();
        print!("{}", OffenderDetailRenderer::render(&visible));

        println!();
        println!(
            "summary: scored_files={} visible_files={} threshold={:.2} top={} total_risk={:.2}",
            reports.len(),
            visible.len(),
            self.threshold,
            self.top,
            reports.iter().map(|report| report.risk_score).sum::<f64>(),
        );
    }

    // The exit-code contract uses the same selection the table shows, so a
    // consumer that only checks the exit code and one that reads the report
    // can never disagree. `top` limits display only, never the verdict.
    #[must_use]
    pub fn has_offenders(&self, reports: &[FileRiskReport]) -> bool {
        reports
            .iter()
            .any(|report| report.risk_score >= self.threshold)
    }

    // Ranking happens here rather than upstream because this is where `top`
    // truncates. The runner appends one package's reports after another, so the
    // incoming slice is ordered only within each package; taking `top` off that
    // would cut whichever package came last instead of the lowest scores, and
    // could omit the highest-risk file in the workspace entirely.
    pub fn select_visible<'a>(&self, reports: &'a [FileRiskReport]) -> Vec<&'a FileRiskReport> {
        let mut visible: Vec<&FileRiskReport> = reports
            .iter()
            .filter(|report| report.risk_score >= self.threshold)
            .collect();
        visible.sort_by(|left, right| RiskOrdering::descending(left, right));
        visible.truncate(self.top);
        visible
    }
}
