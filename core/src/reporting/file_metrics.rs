// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec::Vec;

use crate::reporting::private_function_report::PrivateFunctionReport;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FileMetrics {
    pub functions: Vec<PrivateFunctionReport>,
    pub data_structs: Vec<String>,
    pub behavioral_structs: Vec<String>,
    pub effective_loc: usize,
    pub risk_score: f64,
}
