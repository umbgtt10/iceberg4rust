// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PrivateFunctionReport {
    name: String,
    line: usize,
    complexity: u32,
    is_hidden: bool,
}

impl PrivateFunctionReport {
    pub fn new(name: String, line: usize, complexity: u32, is_hidden: bool) -> Self {
        Self {
            name,
            line,
            complexity,
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
}
