// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;

use crate::invocation::analyzer::Analyzer;
use crate::invocation::config::Config;
use crate::invocation::manifest_resolver::ManifestResolver;
use crate::invocation::package_analyzer::PackageAnalyzer;
use crate::invocation::run_outcome::RunOutcome;
use crate::reporting::report_selection::ReportSelection;
use crate::traits::report_renderer::ReportRenderer;
use crate::traits::source_tree::SourceTree;
use crate::traits::workspace_reader::WorkspaceReader;

pub struct Runner<'a> {
    resolver: ManifestResolver<'a>,
    analyzer: PackageAnalyzer<'a>,
    renderer: &'a dyn ReportRenderer,
}

impl<'a> Runner<'a> {
    pub fn new(
        workspace_reader: &'a dyn WorkspaceReader,
        source_tree: &'a dyn SourceTree,
        renderer: &'a dyn ReportRenderer,
    ) -> Self {
        Self {
            resolver: ManifestResolver::new(workspace_reader),
            analyzer: PackageAnalyzer::new(source_tree, Analyzer::new()),
            renderer,
        }
    }

    // The verdict is taken from the same selection the renderer is handed, so a
    // caller that reads only `has_offenders` and one that reads the output can
    // never disagree. Printing it, and turning it into an exit code, is the
    // caller's.
    pub fn run(&self, config: &Config) -> Result<RunOutcome> {
        let packages = self.resolver.resolve_packages(config)?;

        let mut reports = Vec::new();
        for package in &packages {
            reports.extend(self.analyzer.analyze_package(package)?);
        }

        let selection = ReportSelection::new(config.threshold, config.top);
        let output = self.renderer.render(&selection.report(&reports))?;

        Ok(RunOutcome {
            output,
            has_offenders: selection.has_offenders(&reports),
        })
    }
}
