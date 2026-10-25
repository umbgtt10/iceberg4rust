// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_source_tree::NotHeld;
use iceberg4rust::reporting::file_risk_report::FileRiskReport;
use iceberg4rust::reporting::render_failure::RenderFailure;
use iceberg4rust::reporting::risk_report::RiskReport;
use iceberg4rust::traits::report_renderer::ReportRenderer;
use std::cell::RefCell;

// A renderer that writes nothing a reader would want: it answers with a fixed
// text, or fails, and remembers the files of each report it was handed -- every
// offender, then the visible ones -- by relative path.
pub struct RecordingReportRenderer {
    fails: bool,
    handed: RefCell<Vec<(Vec<String>, Vec<String>)>>,
}

impl RecordingReportRenderer {
    pub fn new() -> Self {
        Self {
            fails: false,
            handed: RefCell::new(Vec::new()),
        }
    }

    pub fn failing() -> Self {
        Self {
            fails: true,
            handed: RefCell::new(Vec::new()),
        }
    }

    pub fn handed(&self) -> Vec<(Vec<String>, Vec<String>)> {
        self.handed.borrow().clone()
    }
}

impl Default for RecordingReportRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportRenderer for RecordingReportRenderer {
    fn render(&self, report: &RiskReport<'_>) -> Result<String, RenderFailure> {
        let names = |files: &[&FileRiskReport]| {
            files
                .iter()
                .map(|file| format!("{} {}", file.package_name(), file.relative_file()))
                .collect::<Vec<String>>()
        };
        self.handed
            .borrow_mut()
            .push((names(&report.offenders), names(&report.visible)));
        if self.fails {
            return Err(RenderFailure {
                cause: Box::new(NotHeld),
            });
        }
        Ok(String::from("rendered"))
    }
}
