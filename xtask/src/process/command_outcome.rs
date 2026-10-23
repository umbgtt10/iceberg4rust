// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutcome {
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
}

impl CommandOutcome {
    pub fn new(exit_code: Option<i32>, stdout: String, stderr: String) -> Self {
        Self {
            exit_code,
            stdout,
            stderr,
        }
    }

    pub fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    pub fn stderr(&self) -> &str {
        &self.stderr
    }

    pub fn stdout(&self) -> &str {
        &self.stdout
    }
}
