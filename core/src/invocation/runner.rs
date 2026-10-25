// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::process::ExitCode;

use anyhow::Result;

use crate::invocation::analyzer::Analyzer;
use crate::invocation::config::Config;
use crate::invocation::manifest_resolver::ManifestResolver;
use crate::invocation::package_analyzer::PackageAnalyzer;
use crate::reporting::json_report_renderer::JsonReportRenderer;
use crate::reporting::report_printer::ReportPrinter;
use crate::traits::source_tree::SourceTree;

const OFFENDERS_FOUND: u8 = 2;

pub struct Runner<'a> {
    analyzer: PackageAnalyzer<'a>,
}

impl<'a> Runner<'a> {
    pub fn new(source_tree: &'a dyn SourceTree) -> Self {
        Self {
            analyzer: PackageAnalyzer::new(source_tree, Analyzer::new()),
        }
    }

    pub fn run(&self, config: &Config) -> Result<ExitCode> {
        let resolver = ManifestResolver::new(config.clone());
        let packages = resolver.resolve_packages()?;

        let mut reports = Vec::new();
        for package in &packages {
            reports.extend(self.analyzer.analyze_package(package)?);
        }

        let printer = ReportPrinter::new(config.threshold, config.top);
        if config.json {
            println!(
                "{}",
                JsonReportRenderer::new(config.threshold).render(&reports)?
            );
        } else {
            printer.print(&reports);
        }

        // 0 clean, 2 offenders, 1 for an error via the `Err` path. A distinct
        // code for offenders lets a caller that keeps its own allow-list tell
        // "the tool failed" from "the tool found something".
        Ok(if printer.has_offenders(&reports) {
            ExitCode::from(OFFENDERS_FOUND)
        } else {
            ExitCode::SUCCESS
        })
    }
}
