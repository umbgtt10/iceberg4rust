// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use syn::{File, Item, parse_file};

use crate::comment_stripper::CommentStripper;
use crate::file_metrics::FileMetrics;
use crate::file_risk_report::FileRiskReport;
use crate::helper_struct_kind::HelperStructKind;
use crate::manifest_resolver::ManifestResolver;
use crate::package_context::PackageContext;
use crate::private_function_collector::PrivateFunctionCollector;
use crate::private_function_report::PrivateFunctionReport;
use crate::private_helper_classifier::PrivateHelperClassifier;
use crate::risk_ordering::RiskOrdering;
use crate::source_file_walker::SourceFileWalker;

#[derive(Default)]
pub struct Analyzer;

impl Analyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze_package(&self, package: &PackageContext) -> Result<Vec<FileRiskReport>> {
        let mut reports = Vec::new();
        for source_root in &package.source_roots {
            reports.extend(self.analyze_source_root(source_root, package)?);
        }
        reports.sort_by(RiskOrdering::descending);
        Ok(reports)
    }

    fn analyze_source_root(
        &self,
        source_root: &Path,
        package: &PackageContext,
    ) -> Result<Vec<FileRiskReport>> {
        if !source_root.exists() {
            return Ok(Vec::new());
        }

        SourceFileWalker::walk(source_root)
            .iter()
            .filter_map(|file_path| self.analyze_source_file(file_path, package).transpose())
            .collect()
    }

    fn analyze_source_file(
        &self,
        file_path: &Path,
        package: &PackageContext,
    ) -> Result<Option<FileRiskReport>> {
        let relative = ManifestResolver::relative_file(&package.manifest_dir, file_path);
        if !ManifestResolver::is_production_relative_file(&relative) {
            return Ok(None);
        }

        let source = fs::read_to_string(file_path)
            .with_context(|| format!("failed to read source file {}", file_path.display()))?;
        let metrics = self.analyze_source(&source)?;
        if metrics.risk_score <= f64::EPSILON {
            return Ok(None);
        }

        Ok(Some(FileRiskReport {
            package_name: package.name.clone(),
            relative_file: relative,
            effective_loc: metrics.effective_loc,
            private_function_count: metrics.private_function_count,
            private_complexity_sum: metrics.private_complexity_sum,
            data_private_struct_count: metrics.data_private_struct_count,
            behavioral_private_struct_count: metrics.behavioral_private_struct_count,
            functions: metrics.functions,
            data_structs: metrics.data_structs,
            behavioral_structs: metrics.behavioral_structs,
            risk_score: metrics.risk_score,
        }))
    }

    pub fn analyze_source(&self, source: &str) -> Result<FileMetrics> {
        let syntax = parse_file(source).context("failed to parse Rust source")?;

        let effective_loc = CommentStripper::count_effective_loc(source);
        let primary_struct = first_struct_name(&syntax);
        let classifier = PrivateHelperClassifier::new(&syntax.items, primary_struct.as_deref());
        let helper_structs = classifier.into_map();
        let collector = PrivateFunctionCollector::new(&helper_structs);
        let private_functions = collector.collect(&syntax.items);

        let private_function_count = private_functions
            .iter()
            .filter(|function| function.is_hidden)
            .count();
        let private_complexity_sum: u32 = private_functions.iter().map(|f| f.complexity).sum();
        let used_private_structs: BTreeSet<String> = private_functions
            .iter()
            .flat_map(|f| f.used_private_structs.iter().cloned())
            .collect();

        let data_structs = Self::structs_of_kind(
            &used_private_structs,
            &helper_structs,
            HelperStructKind::Data,
        );
        let behavioral_structs = Self::structs_of_kind(
            &used_private_structs,
            &helper_structs,
            HelperStructKind::Behavioral,
        );
        let data_private_struct_count = data_structs.len();
        let behavioral_private_struct_count = behavioral_structs.len();

        let functions = private_functions
            .iter()
            .map(|function| PrivateFunctionReport {
                name: function.name.clone(),
                line: function.line,
                complexity: function.complexity,
                is_hidden: function.is_hidden,
            })
            .collect();
        let risk_score = Self::compute_file_risk(
            effective_loc,
            private_function_count,
            private_complexity_sum,
            data_private_struct_count,
            behavioral_private_struct_count,
        );

        Ok(FileMetrics {
            functions,
            data_structs,
            behavioral_structs,
            effective_loc,
            private_function_count,
            private_complexity_sum,
            data_private_struct_count,
            behavioral_private_struct_count,
            risk_score,
        })
    }

    fn structs_of_kind(
        used: &BTreeSet<String>,
        helper_structs: &BTreeMap<String, HelperStructKind>,
        kind: HelperStructKind,
    ) -> Vec<String> {
        used.iter()
            .filter(|name| {
                helper_structs
                    .get(*name)
                    .is_some_and(|found| *found == kind)
            })
            .cloned()
            .collect()
    }

    const DATA_STRUCT_WEIGHT: f64 = 0.5;
    const BEHAVIORAL_STRUCT_WEIGHT: f64 = 2.0;

    pub fn compute_file_risk(
        effective_loc: usize,
        private_function_count: usize,
        private_complexity_sum: u32,
        data_private_struct_count: usize,
        behavioral_private_struct_count: usize,
    ) -> f64 {
        // No early return for `private_function_count == 0`. The zero baseline
        // falls out of the formula when every term is zero, as SPEC.md states,
        // and short-circuiting on P alone would discard the complexity of a
        // trait impl — which no longer counts toward P but is still burden.
        let size_factor = ((effective_loc + 1) as f64).log2() / 10.0;

        size_factor
            * ((private_function_count as f64)
                + 0.5 * f64::from(private_complexity_sum)
                + Self::DATA_STRUCT_WEIGHT * (data_private_struct_count as f64)
                + Self::BEHAVIORAL_STRUCT_WEIGHT * (behavioral_private_struct_count as f64))
    }
}

fn first_struct_name(syntax: &File) -> Option<String> {
    syntax.items.iter().find_map(|item| {
        if let Item::Struct(item_struct) = item {
            Some(item_struct.ident.to_string())
        } else {
            None
        }
    })
}
