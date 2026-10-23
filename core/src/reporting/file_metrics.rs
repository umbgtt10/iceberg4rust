// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::reporting::private_function_report::PrivateFunctionReport;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FileMetrics {
    functions: Vec<PrivateFunctionReport>,
    data_structs: Vec<String>,
    behavioral_structs: Vec<String>,
    effective_loc: usize,
    private_function_count: usize,
    private_complexity_sum: u32,
    data_private_struct_count: usize,
    behavioral_private_struct_count: usize,
    risk_score: f64,
}

impl FileMetrics {
    pub fn behavioral_private_struct_count(&self) -> usize {
        self.behavioral_private_struct_count
    }

    pub fn behavioral_structs(&self) -> &[String] {
        &self.behavioral_structs
    }

    pub fn data_private_struct_count(&self) -> usize {
        self.data_private_struct_count
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

    pub fn private_complexity_sum(&self) -> u32 {
        self.private_complexity_sum
    }

    pub fn private_function_count(&self) -> usize {
        self.private_function_count
    }

    pub fn risk_score(&self) -> f64 {
        self.risk_score
    }

    pub fn with_behavioral_private_struct_count(
        self,
        behavioral_private_struct_count: usize,
    ) -> Self {
        Self {
            behavioral_private_struct_count,
            ..self
        }
    }

    pub fn with_behavioral_structs(self, behavioral_structs: Vec<String>) -> Self {
        Self {
            behavioral_structs,
            ..self
        }
    }

    pub fn with_data_private_struct_count(self, data_private_struct_count: usize) -> Self {
        Self {
            data_private_struct_count,
            ..self
        }
    }

    pub fn with_data_structs(self, data_structs: Vec<String>) -> Self {
        Self {
            data_structs,
            ..self
        }
    }

    pub fn with_effective_loc(self, effective_loc: usize) -> Self {
        Self {
            effective_loc,
            ..self
        }
    }

    pub fn with_functions(self, functions: Vec<PrivateFunctionReport>) -> Self {
        Self { functions, ..self }
    }

    pub fn with_private_complexity_sum(self, private_complexity_sum: u32) -> Self {
        Self {
            private_complexity_sum,
            ..self
        }
    }

    pub fn with_private_function_count(self, private_function_count: usize) -> Self {
        Self {
            private_function_count,
            ..self
        }
    }

    pub fn with_risk_score(self, risk_score: f64) -> Self {
        Self { risk_score, ..self }
    }
}
