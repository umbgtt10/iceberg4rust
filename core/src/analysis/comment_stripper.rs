// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

const LINE_COMMENT_START: (char, Option<char>) = ('/', Some('/'));
const BLOCK_COMMENT_START: (char, Option<char>) = ('/', Some('*'));
const BLOCK_COMMENT_END: (char, Option<char>) = ('*', Some('/'));

#[derive(Default)]
pub struct CommentStripper {
    depth: usize,
}

impl CommentStripper {
    pub fn new() -> Self {
        Self { depth: 0 }
    }

    pub fn count_effective_loc(source: &str) -> usize {
        let mut stripper = CommentStripper::new();
        let mut count = 0;

        for line in source.lines() {
            let stripped = stripper.strip(line);
            if !stripped.trim().is_empty() {
                count += 1;
            }
        }

        count
    }

    pub fn strip(&mut self, line: &str) -> String {
        let mut result = String::new();
        let mut chars = line.chars().peekable();

        while let Some(current) = chars.next() {
            match (current, chars.peek().copied()) {
                BLOCK_COMMENT_START => {
                    self.depth += 1;
                    chars.next();
                }
                BLOCK_COMMENT_END if self.depth > 0 => {
                    self.depth -= 1;
                    chars.next();
                }
                _ if self.depth > 0 => {}
                LINE_COMMENT_START => break,
                _ => result.push(current),
            }
        }

        result
    }
}
