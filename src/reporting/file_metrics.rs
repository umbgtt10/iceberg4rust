// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::reporting::private_function_report::PrivateFunctionReport;

#[derive(Debug, Clone, PartialEq)]
pub struct FileMetrics {
    pub functions: Vec<PrivateFunctionReport>,
    pub data_structs: Vec<String>,
    pub behavioral_structs: Vec<String>,
    pub effective_loc: usize,
    pub private_function_count: usize,
    pub private_complexity_sum: u32,
    pub data_private_struct_count: usize,
    pub behavioral_private_struct_count: usize,
    pub risk_score: f64,
}
