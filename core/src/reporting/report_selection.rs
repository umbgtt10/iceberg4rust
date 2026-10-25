// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

use crate::reporting::file_risk_report::FileRiskReport;
use crate::reporting::risk_ordering::RiskOrdering;
use crate::reporting::risk_report::RiskReport;

pub struct ReportSelection {
    threshold: f64,
    top: usize,
}

impl ReportSelection {
    pub fn new(threshold: f64, top: usize) -> Self {
        Self { threshold, top }
    }

    // The exit-code contract uses the same selection the table shows, so a
    // consumer that only checks the exit code and one that reads the report
    // can never disagree. `top` limits display only, never the verdict.
    #[must_use]
    pub fn has_offenders(&self, reports: &[FileRiskReport]) -> bool {
        reports
            .iter()
            .any(|report| report.risk_score() >= self.threshold)
    }

    // Every renderer gets the same choice of files: a machine consumer every
    // offender, a reader the `top` of them, both in the one ranking, and the
    // totals across every scored file rather than the visible ones.
    #[must_use]
    pub fn report<'a>(&self, reports: &'a [FileRiskReport]) -> RiskReport<'a> {
        RiskReport {
            threshold: self.threshold,
            top: self.top,
            scored_files: reports.len(),
            total_risk: reports.iter().map(|report| report.risk_score()).sum(),
            offenders: self.ranked(reports),
            visible: self.select_visible(reports),
        }
    }

    // Ranking happens here rather than upstream because this is where `top`
    // truncates. The runner appends one package's reports after another, so the
    // incoming slice is ordered only within each package; taking `top` off that
    // would cut whichever package came last instead of the lowest scores, and
    // could omit the highest-risk file in the workspace entirely.
    pub fn select_visible<'a>(&self, reports: &'a [FileRiskReport]) -> Vec<&'a FileRiskReport> {
        let mut visible = self.ranked(reports);
        visible.truncate(self.top);
        visible
    }

    fn ranked<'a>(&self, reports: &'a [FileRiskReport]) -> Vec<&'a FileRiskReport> {
        let mut ranked: Vec<&FileRiskReport> = reports
            .iter()
            .filter(|report| report.risk_score() >= self.threshold)
            .collect();
        ranked.sort_by(|left, right| RiskOrdering::descending(left, right));
        ranked
    }
}
