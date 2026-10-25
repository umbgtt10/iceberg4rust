// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateFunctionReport {
    pub name: String,
    pub line: usize,
    pub complexity: u32,
    pub is_hidden: bool,
}
