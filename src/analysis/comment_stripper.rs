// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

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

    pub fn strip(&mut self, line: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = line.chars().collect();
        let mut index = 0;

        while index < chars.len() {
            if self.in_block_comment {
                self.skip_block_comment_end(&chars, &mut index);
                continue;
            }

            if Self::starts_line_comment(&chars, index) {
                break;
            }

            if Self::starts_block_comment(&chars, index) {
                self.in_block_comment = true;
                index += 2;
                continue;
            }

            result.push(chars[index]);
            index += 1;
        }

        result
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

    fn skip_block_comment_end(&mut self, chars: &[char], index: &mut usize) {
        if *index + 1 < chars.len() && chars[*index] == '*' && chars[*index + 1] == '/' {
            self.in_block_comment = false;
            *index += 2;
        } else {
            *index += 1;
        }
    }

    fn starts_line_comment(chars: &[char], index: usize) -> bool {
        index + 1 < chars.len() && chars[index] == '/' && chars[index + 1] == '/'
    }

    fn starts_block_comment(chars: &[char], index: usize) -> bool {
        index + 1 < chars.len() && chars[index] == '/' && chars[index + 1] == '*'
    }
}
