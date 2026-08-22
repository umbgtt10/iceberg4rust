// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::cmp::Ordering;

use crate::reporting::file_risk_report::FileRiskReport;

pub struct RiskOrdering;

impl RiskOrdering {
    // Equal scores break on the file path so the table is byte-identical
    // between runs. Without the tie-break two files sharing a score would swap
    // places depending on which package the walker reached first.
    #[must_use]
    pub fn descending(left: &FileRiskReport, right: &FileRiskReport) -> Ordering {
        right
            .risk_score
            .partial_cmp(&left.risk_score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| left.relative_file.cmp(&right.relative_file))
    }
}
