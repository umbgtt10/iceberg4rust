// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

const LINE_COMMENT_START: [char; 2] = ['/', '/'];
const BLOCK_COMMENT_START: [char; 2] = ['/', '*'];
const BLOCK_COMMENT_END: [char; 2] = ['*', '/'];

#[derive(Default)]
pub struct CommentStripper {
    in_block_comment: bool,
}

impl CommentStripper {
    pub fn new() -> Self {
        Self {
            in_block_comment: false,
        }
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
        let chars: Vec<char> = line.chars().collect();
        let mut index = 0;

        while index < chars.len() {
            if self.in_block_comment {
                index = self.skip_block_comment_end(&chars, index);
                continue;
            }

            if Self::pair_at(&chars, index, LINE_COMMENT_START) {
                break;
            }

            if Self::pair_at(&chars, index, BLOCK_COMMENT_START) {
                self.in_block_comment = true;
                index += 2;
                continue;
            }

            result.push(chars[index]);
            index += 1;
        }

        result
    }

    fn pair_at(chars: &[char], index: usize, pair: [char; 2]) -> bool {
        index + 1 < chars.len() && chars[index] == pair[0] && chars[index + 1] == pair[1]
    }

    fn skip_block_comment_end(&mut self, chars: &[char], index: usize) -> usize {
        if Self::pair_at(chars, index, BLOCK_COMMENT_END) {
            self.in_block_comment = false;
            index + 2
        } else {
            index + 1
        }
    }
}
