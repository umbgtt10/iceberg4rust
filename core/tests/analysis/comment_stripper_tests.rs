// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::analysis::comment_stripper::CommentStripper;

#[test]
fn count_effective_loc_block_comment_across_lines() {
    // Arrange
    let source = "fn main() {\n/*\nmulti\nline\n*/\n}\n";

    // Act
    let count = CommentStripper::count_effective_loc(source);

    // Assert
    assert_eq!(count, 2);
}

#[test]
fn count_effective_loc_code_lines_are_counted() {
    // Arrange
    let source = "fn foo() {\n    let x = 1;\n    x\n}\n";

    // Act
    let count = CommentStripper::count_effective_loc(source);

    // Assert
    assert_eq!(count, 4);
}

#[test]
fn count_effective_loc_empty_source_returns_zero() {
    // Arrange & Act
    let count = CommentStripper::count_effective_loc("");

    // Assert
    assert_eq!(count, 0);
}

#[test]
fn count_effective_loc_mixed_comments_and_code() {
    // Arrange
    let source = "// license header\nfn main() {\n    let x = 1; // inline\n    /* block */\n    println!(\"{x}\");\n}\n";

    // Act
    let count = CommentStripper::count_effective_loc(source);

    // Assert
    assert_eq!(count, 4);
}

#[test]
fn count_effective_loc_nested_block_comment_across_lines() {
    // Arrange
    let source = "/* outer\n/* inner */\nstill outer */\nlet x = 1;\n";

    // Act
    let count = CommentStripper::count_effective_loc(source);

    // Assert
    assert_eq!(count, 1);
}

#[test]
fn count_effective_loc_only_comments_and_blanks_returns_zero() {
    // Arrange
    let source = "\n// comment\n/* block */\n\n  \n";

    // Act
    let count = CommentStripper::count_effective_loc(source);

    // Assert
    assert_eq!(count, 0);
}

#[test]
fn count_effective_loc_trailing_block_comment_ending_same_line() {
    // Arrange
    let source = "let x = 1; /* end */ let y = 2;";

    // Act
    let count = CommentStripper::count_effective_loc(source);

    // Assert
    assert_eq!(count, 1);
}

#[test]
fn new_creates_stripper_with_no_active_block_comment() {
    // Arrange
    let mut stripper = CommentStripper::new();
    let line = "code";

    // Act
    let result = stripper.strip(line);

    // Assert
    assert_eq!(result, "code");
}

#[test]
fn strip_block_comment_closed_after_text_keeps_what_follows() {
    // Arrange
    let mut stripper = CommentStripper::new();
    let line = "abc/**/d";

    // Act
    let result = stripper.strip(line);

    // Assert
    assert_eq!(result, "abcd");
}

#[test]
fn strip_block_comment_continues_across_lines() {
    // Arrange
    let mut stripper = CommentStripper::new();
    let line1 = "let x = 42; /* start";
    let line2 = "still in block";
    let line3 = "end */ let y = 1;";

    // Act
    let r1 = stripper.strip(line1);
    let r2 = stripper.strip(line2);
    let r3 = stripper.strip(line3);

    // Assert
    assert_eq!(r1, "let x = 42; ");
    assert_eq!(r2, "");
    assert_eq!(r3, " let y = 1;");
}

#[test]
fn strip_block_comment_opened_by_slash_star_slash_stays_open() {
    // Arrange
    let mut stripper = CommentStripper::new();
    let line = "/*/ a */ b";

    // Act
    let result = stripper.strip(line);

    // Assert
    assert_eq!(result, " b");
}

#[test]
fn strip_block_comment_start_removes_from_slash_star() {
    // Arrange
    let mut stripper = CommentStripper::new();
    let line = "let x = 42; /* block comment";

    // Act
    let result = stripper.strip(line);

    // Assert
    assert_eq!(result, "let x = 42; ");
}

#[test]
fn strip_line_comment_inside_block_comment_is_not_special() {
    // Arrange
    let mut stripper = CommentStripper::new();
    let line = "/* start // still in block";

    // Act
    let result = stripper.strip(line);

    // Assert
    assert_eq!(result, "");
}

#[test]
fn strip_line_comment_only_returns_empty() {
    // Arrange
    let mut stripper = CommentStripper::new();
    let line = "// just a comment";

    // Act
    let result = stripper.strip(line);

    // Assert
    assert_eq!(result, "");
}

#[test]
fn strip_line_comment_removes_from_double_slash() {
    // Arrange
    let mut stripper = CommentStripper::new();
    let line = "let x = 42; // this is a comment";

    // Act
    let result = stripper.strip(line);

    // Assert
    assert_eq!(result, "let x = 42; ");
}

#[test]
fn strip_line_ending_in_a_slash_keeps_it() {
    // Arrange
    let mut stripper = CommentStripper::new();
    let line = "a /";

    // Act
    let result = stripper.strip(line);

    // Assert
    assert_eq!(result, "a /");
}

#[test]
fn strip_nested_block_comment_closes_at_its_outer_end() {
    // Arrange
    let mut stripper = CommentStripper::new();
    let line = "/* a /* b */ c */ d";

    // Act
    let result = stripper.strip(line);

    // Assert
    assert_eq!(result, " d");
}

#[test]
fn strip_nested_block_comment_continues_across_lines() {
    // Arrange
    let mut stripper = CommentStripper::new();
    let line1 = "x /* a /* b";
    let line2 = "c */ d";
    let line3 = "e */ f";

    // Act
    let r1 = stripper.strip(line1);
    let r2 = stripper.strip(line2);
    let r3 = stripper.strip(line3);

    // Assert
    assert_eq!(r1, "x ");
    assert_eq!(r2, "");
    assert_eq!(r3, " f");
}

#[test]
fn strip_no_comment_preserves_line() {
    // Arrange
    let mut stripper = CommentStripper::new();
    let line = "let x = 42;";

    // Act
    let result = stripper.strip(line);

    // Assert
    assert_eq!(result, "let x = 42;");
}

#[test]
fn strip_slash_in_string_is_treated_as_line_comment() {
    // Arrange
    let mut stripper = CommentStripper::new();
    let line = "let s = \"hello // world\";";

    // Act
    let result = stripper.strip(line);

    // Assert
    assert_eq!(result, "let s = \"hello ");
}

#[test]
fn strip_star_slash_outside_a_comment_is_kept() {
    // Arrange
    let mut stripper = CommentStripper::new();
    let line = "a */ b /* c */ d";

    // Act
    let result = stripper.strip(line);

    // Assert
    assert_eq!(result, "a */ b  d");
}
