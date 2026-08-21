// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::analysis::complexity_scorer::ComplexityScorer;
use syn::parse_file;

fn score_block(source: &str) -> u32 {
    let wrapped = format!("fn _dummy() {{ {} }}", source);
    let syntax = parse_file(&wrapped).expect("valid source");
    let items = syntax.items;
    let item_fn = items.into_iter().next().expect("one item");
    let syn::Item::Fn(f) = item_fn else {
        panic!("expected fn")
    };
    ComplexityScorer::new().score(&f.block)
}

#[test]
fn binary_expr_does_not_add_extra() {
    // Arrange & Act
    let score = score_block("let x = 1 + 2 * 3;");

    // Assert
    assert_eq!(score, 0);
}

#[test]
fn block_expr_does_not_add_extra() {
    // Arrange & Act
    let score = score_block("{ let x = 1; }");

    // Assert
    assert_eq!(score, 0);
}

#[test]
fn deeply_nested_ifs_compound() {
    // Arrange & Act
    let score = score_block("if true { if false { if true {} } }");

    // Assert
    assert_eq!(score, 6);
}

#[test]
fn else_if_increments_with_nesting() {
    // Arrange & Act
    let score = score_block("if true {} else if false {} else {}");

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn empty_block_returns_zero() {
    // Arrange & Act
    let score = score_block("");

    // Assert
    assert_eq!(score, 0);
}

#[test]
fn for_loop_adds_one() {
    // Arrange & Act
    let score = score_block("for _ in 0..10 {}");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn if_else_adds_one() {
    // Arrange & Act
    let score = score_block("if true {} else {}");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn infinite_loop_adds_one() {
    // Arrange & Act
    let score = score_block("loop {}");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn match_adds_one_base() {
    // Arrange & Act
    let score = score_block("match x { 1 => {}, _ => {} }");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn match_inside_if_compounds_scores() {
    // Arrange & Act
    let score = score_block("if true { match x { 1 => {}, _ => {} } }");

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn multiple_control_flows_sum() {
    // Arrange & Act
    let score = score_block("if true {} for _ in 0..5 {} loop {}");

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn nested_if_increments_nesting() {
    // Arrange & Act
    let score = score_block("if true { if false {} }");

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn single_if_adds_one() {
    // Arrange & Act
    let score = score_block("if true {}");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn while_loop_adds_one() {
    // Arrange & Act
    let score = score_block("while true {}");

    // Assert
    assert_eq!(score, 1);
}
