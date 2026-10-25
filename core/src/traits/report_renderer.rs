// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

use crate::reporting::render_failure::RenderFailure;
use crate::reporting::risk_report::RiskReport;

pub trait ReportRenderer {
    fn render(&self, report: &RiskReport<'_>) -> Result<String, RenderFailure>;
}
