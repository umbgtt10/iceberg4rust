// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct PrivateFunctionMetrics {
    pub name: String,
    pub line: usize,
    pub complexity: u32,
    pub used_private_structs: BTreeSet<String>,
    pub is_hidden: bool,
}
