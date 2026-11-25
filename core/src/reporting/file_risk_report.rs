// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec::Vec;

use crate::reporting::private_function_report::PrivateFunctionReport;

use crate::reporting::file_metrics::FileMetrics;

#[derive(Debug, Clone, PartialEq)]
pub struct FileRiskReport {
    functions: Vec<PrivateFunctionReport>,
    data_structs: Vec<String>,
    behavioral_structs: Vec<String>,
    package_name: String,
    relative_file: String,
    effective_loc: usize,
    risk_score: f64,
}

impl FileRiskReport {
    pub fn new(package_name: String, relative_file: String, metrics: FileMetrics) -> Self {
        Self {
            functions: metrics.functions,
            data_structs: metrics.data_structs,
            behavioral_structs: metrics.behavioral_structs,
            package_name,
            relative_file,
            effective_loc: metrics.effective_loc,
            risk_score: metrics.risk_score,
        }
    }

    pub fn behavioral_private_struct_count(&self) -> usize {
        self.behavioral_structs.len()
    }

    pub fn behavioral_structs(&self) -> &[String] {
        &self.behavioral_structs
    }

    pub fn data_private_struct_count(&self) -> usize {
        self.data_structs.len()
    }

    pub fn data_structs(&self) -> &[String] {
        &self.data_structs
    }

    pub fn effective_loc(&self) -> usize {
        self.effective_loc
    }

    pub fn functions(&self) -> &[PrivateFunctionReport] {
        &self.functions
    }

    pub fn package_name(&self) -> &str {
        &self.package_name
    }

    pub fn private_complexity_sum(&self) -> u32 {
        self.functions
            .iter()
            .map(|function| function.complexity)
            .sum()
    }

    pub fn private_function_count(&self) -> usize {
        self.functions
            .iter()
            .filter(|function| function.is_hidden)
            .count()
    }

    pub fn relative_file(&self) -> &str {
        &self.relative_file
    }

    pub fn risk_score(&self) -> f64 {
        self.risk_score
    }
}
