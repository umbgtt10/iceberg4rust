// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct PrivateFunctionMetrics {
    name: String,
    line: usize,
    complexity: u32,
    used_private_structs: BTreeSet<String>,
    is_hidden: bool,
}

impl PrivateFunctionMetrics {
    pub fn new(
        name: String,
        line: usize,
        complexity: u32,
        used_private_structs: BTreeSet<String>,
        is_hidden: bool,
    ) -> Self {
        Self {
            name,
            line,
            complexity,
            used_private_structs,
            is_hidden,
        }
    }

    pub fn complexity(&self) -> u32 {
        self.complexity
    }

    pub fn is_hidden(&self) -> bool {
        self.is_hidden
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn used_private_structs(&self) -> &BTreeSet<String> {
        &self.used_private_structs
    }
}
