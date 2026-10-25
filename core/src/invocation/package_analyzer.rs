// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

use crate::invocation::analyzer::Analyzer;
use crate::invocation::manifest_resolver::ManifestResolver;
use crate::invocation::package_context::PackageContext;
use crate::invocation::run_failure::RunFailure;
use crate::reporting::file_risk_report::FileRiskReport;
use crate::reporting::risk_ordering::RiskOrdering;
use crate::traits::source_tree::SourceTree;
use crate::workspace::source_path::SourcePath;

// One package's files, read through the source tree it was given and scored;
// a file outside the production tree or scoring zero yields no report.
pub struct PackageAnalyzer<'a> {
    source_tree: &'a dyn SourceTree,
    analyzer: Analyzer,
}

impl<'a> PackageAnalyzer<'a> {
    pub fn new(source_tree: &'a dyn SourceTree, analyzer: Analyzer) -> Self {
        Self {
            source_tree,
            analyzer,
        }
    }

    pub fn analyze_package(
        &self,
        package: &PackageContext,
    ) -> Result<Vec<FileRiskReport>, RunFailure> {
        let mut reports = Vec::new();
        for source_root in &package.source_roots {
            reports.extend(self.analyze_source_root(source_root, package)?);
        }
        reports.sort_by(RiskOrdering::descending);
        Ok(reports)
    }

    fn analyze_source_file(
        &self,
        file_path: &SourcePath,
        package: &PackageContext,
    ) -> Result<Option<FileRiskReport>, RunFailure> {
        let relative = ManifestResolver::relative_file(&package.manifest_dir, file_path);
        if !ManifestResolver::is_production_relative_file(&relative) {
            return Ok(None);
        }

        let source = self.source_tree.read_source(file_path)?;
        let metrics = self.analyzer.analyze_source(&source)?;
        if metrics.risk_score <= f64::EPSILON {
            return Ok(None);
        }

        Ok(Some(FileRiskReport::new(
            package.name.clone(),
            relative,
            metrics,
        )))
    }

    fn analyze_source_root(
        &self,
        source_root: &SourcePath,
        package: &PackageContext,
    ) -> Result<Vec<FileRiskReport>, RunFailure> {
        self.source_tree
            .rust_files(source_root)?
            .iter()
            .filter_map(|file_path| self.analyze_source_file(file_path, package).transpose())
            .collect()
    }
}
