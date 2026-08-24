// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PrivateFunctionReport {
    pub name: String,
    pub line: usize,
    pub complexity: u32,
    pub is_hidden: bool,
}
