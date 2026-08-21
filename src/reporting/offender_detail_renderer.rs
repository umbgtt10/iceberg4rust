// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fmt::Write;

use crate::reporting::file_risk_report::FileRiskReport;
use crate::reporting::private_function_report::PrivateFunctionReport;

pub struct OffenderDetailRenderer;

impl OffenderDetailRenderer {
    // The summary table says which file is over; this says what is in it, so a
    // reader has somewhere to go without opening the file first. Ordered by
    // complexity because that is where extraction usually starts.
    #[must_use]
    pub fn render(offenders: &[&FileRiskReport]) -> String {
        if offenders.is_empty() {
            return String::new();
        }

        let mut out = String::from("Offender detail:\n");
        for offender in offenders {
            Self::render_offender(&mut out, offender);
        }
        out
    }

    fn render_offender(out: &mut String, offender: &FileRiskReport) {
        let _ = writeln!(
            out,
            "\n  {}  {}  (risk {:.2}, {} private fns, complexity {})",
            offender.package_name,
            offender.relative_file,
            offender.risk_score,
            offender.private_function_count,
            offender.private_complexity_sum,
        );
        let _ = writeln!(out, "    {:>6}  {:>10}  function", "line", "complexity");

        for function in Self::by_descending_complexity(&offender.functions) {
            Self::render_function(out, function);
        }

        Self::render_structs(out, offender);
    }

    fn by_descending_complexity(
        functions: &[PrivateFunctionReport],
    ) -> Vec<&PrivateFunctionReport> {
        let mut ordered: Vec<&PrivateFunctionReport> = functions.iter().collect();
        ordered.sort_by(|left, right| {
            right
                .complexity
                .cmp(&left.complexity)
                .then_with(|| left.line.cmp(&right.line))
        });
        ordered
    }

    fn render_function(out: &mut String, function: &PrivateFunctionReport) {
        // A trait method is flagged because it does not count toward the private
        // surface, so a reader is not left wondering why the tally disagrees.
        let marker = if function.is_hidden { "" } else { "  (trait)" };
        let _ = writeln!(
            out,
            "    {:>6}  {:>10}  {}{}",
            function.line, function.complexity, function.name, marker,
        );
    }

    fn render_structs(out: &mut String, offender: &FileRiskReport) {
        if offender.data_structs.is_empty() && offender.behavioral_structs.is_empty() {
            return;
        }
        let _ = writeln!(
            out,
            "    helper structs: data=[{}]  behavioral=[{}]",
            offender.data_structs.join(", "),
            offender.behavioral_structs.join(", "),
        );
    }
}
