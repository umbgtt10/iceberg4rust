// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::reporting::file_risk_report::FileRiskReport;

// What a renderer is handed: the run's numbers and its files already chosen and
// ranked, so that no renderer decides which file is an offender. `offenders` is
// every file at or above the threshold, highest risk first; `visible` is the
// same ranking cut to `top`, for a reader rather than a machine.
#[derive(Debug, Clone, PartialEq)]
pub struct RiskReport<'a> {
    pub threshold: f64,
    pub top: usize,
    pub scored_files: usize,
    pub total_risk: f64,
    pub offenders: Vec<&'a FileRiskReport>,
    pub visible: Vec<&'a FileRiskReport>,
}
