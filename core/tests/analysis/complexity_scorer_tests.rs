// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::analysis::complexity_scorer::ComplexityScorer;
use syn::Item;
use syn::parse_file;

fn score_block(source: &str) -> u32 {
    let wrapped = format!("fn _dummy() {{ {} }}", source);
    let syntax = parse_file(&wrapped).expect("valid source");
    let items = syntax.items;
    let item_fn = items.into_iter().next().expect("one item");
    let Item::Fn(f) = item_fn else {
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
fn score_assignment_of_an_if_adds_the_if_score() {
    // Arrange & Act
    let score = score_block("x = if a { 1 } else { 2 };");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn score_async_block_holding_an_if_adds_the_if_score() {
    // Arrange & Act
    let score = score_block("let f = async { if a {} };");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn score_binary_operand_holding_an_if_adds_the_if_score() {
    // Arrange & Act
    let score = score_block("let x = 1 + if a { 1 } else { 2 };");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn score_block_expression_holding_an_if_adds_the_if_score() {
    // Arrange & Act
    let score = score_block("{ if a {} }");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn score_break_value_holding_an_if_scores_it_inside_the_loop() {
    // Arrange & Act
    let score = score_block("loop { break if a { 1 } else { 2 }; }");

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn score_call_argument_holding_an_if_adds_the_if_score() {
    // Arrange & Act
    let score = score_block("consume(if a { 1 } else { 2 });");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn score_closure_body_holding_an_if_scores_it_one_level_deeper() {
    // Arrange & Act
    let score = score_block("let f = || if a {};");

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn score_closure_inside_an_if_scores_its_body_two_levels_deeper() {
    // Arrange & Act
    let score = score_block("if a { let f = || if b {}; }");

    // Assert
    assert_eq!(score, 4);
}

#[test]
fn score_closure_passed_to_a_method_holding_an_if_scores_it_one_level_deeper() {
    // Arrange & Act
    let score = score_block("items.iter().map(|x| if x { 1 } else { 2 });");

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn score_closure_without_control_flow_adds_nothing() {
    // Arrange & Act
    let score = score_block("let f = |x: u32| x + 1;");

    // Assert
    assert_eq!(score, 0);
}

#[test]
fn score_constructs_one_after_another_score_at_the_outer_nesting() {
    // Arrange & Act
    let score = score_block("match x { _ => {} } while b {} loop {} if a {}");

    // Assert
    assert_eq!(score, 4);
}

#[test]
fn score_for_loop_body_holding_an_if_scores_it_one_level_deeper() {
    // Arrange & Act
    let score = score_block("for _ in 0..1 { if a {} }");

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn score_for_loop_inside_an_if_adds_its_nesting() {
    // Arrange & Act
    let score = score_block("if a { for _ in 0..1 {} }");

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn score_for_loop_over_a_match_adds_the_match_score() {
    // Arrange & Act
    let score = score_block("for _ in match x { _ => 0..1 } {}");

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn score_if_after_a_closure_scores_at_the_outer_nesting() {
    // Arrange & Act
    let score = score_block("let f = || if b {}; if a {}");

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn score_if_condition_holding_a_match_adds_the_match_score() {
    // Arrange & Act
    let score = score_block("if match x { _ => true } {}");

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn score_if_inside_a_macro_call_returns_zero() {
    // Arrange & Act
    let score = score_block("assert!(if a { true } else { false });");

    // Assert
    assert_eq!(score, 0);
}

#[test]
fn score_if_let_adds_one() {
    // Arrange & Act
    let score = score_block("if let Some(v) = x {}");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn score_let_else_block_holding_an_if_adds_the_if_score() {
    // Arrange & Act
    let score = score_block("let Some(v) = x else { if a {} return; };");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn score_loop_body_holding_an_if_scores_it_one_level_deeper() {
    // Arrange & Act
    let score = score_block("loop { if a {} }");

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn score_loop_inside_an_if_adds_its_nesting() {
    // Arrange & Act
    let score = score_block("if a { loop {} }");

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn score_match_arm_guard_holding_a_match_scores_it_one_level_deeper() {
    // Arrange & Act
    let score = score_block("match x { y if match y { _ => true } => {}, _ => {} }");

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn score_match_arm_holding_an_if_scores_it_one_level_deeper() {
    // Arrange & Act
    let score = score_block("match x { 1 => if a {}, _ => {} }");

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn score_match_scrutinee_holding_an_if_adds_the_if_score() {
    // Arrange & Act
    let score = score_block("match if a { 1 } else { 2 } { _ => {} }");

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn score_method_call_argument_holding_an_if_adds_the_if_score() {
    // Arrange & Act
    let score = score_block("x.consume(if a { 1 } else { 2 });");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn score_method_call_receiver_holding_a_match_adds_the_match_score() {
    // Arrange & Act
    let score = score_block("let z = match x { _ => y }.consume();");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn score_negated_if_adds_the_if_score() {
    // Arrange & Act
    let score = score_block("let z = !if a { true } else { false };");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn score_nested_fn_adds_its_body_score() {
    // Arrange & Act
    let score = score_block("fn inner() { if a {} if b {} }");

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn score_nested_impl_method_adds_its_body_score() {
    // Arrange & Act
    let score = score_block("impl Local { fn inner(&self) { if a {} if b {} } }");

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn score_nested_module_fn_adds_its_body_score() {
    // Arrange & Act
    let score = score_block("mod local { fn inner() { if a {} if b {} } }");

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn score_parenthesised_if_adds_the_if_score() {
    // Arrange & Act
    let score = score_block("let z = (if a { 1 } else { 2 });");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn score_reference_to_an_if_adds_the_if_score() {
    // Arrange & Act
    let score = score_block("let r = &if a { 1 } else { 2 };");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn score_return_value_holding_an_if_adds_the_if_score() {
    // Arrange & Act
    let score = score_block("return if a { 1 } else { 2 };");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn score_struct_literal_field_holding_an_if_adds_the_if_score() {
    // Arrange & Act
    let score = score_block("let s = S { v: if a { 1 } else { 2 } };");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn score_try_on_an_if_adds_the_if_score() {
    // Arrange & Act
    let score = score_block("let v = if a { x } else { y }?;");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn score_tuple_element_holding_an_if_adds_the_if_score() {
    // Arrange & Act
    let score = score_block("let t = (1, if a { 1 } else { 2 });");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn score_unsafe_block_holding_an_if_adds_the_if_score() {
    // Arrange & Act
    let score = score_block("unsafe { if a {} }");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn score_while_let_adds_one() {
    // Arrange & Act
    let score = score_block("while let Some(v) = it.next() {}");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn score_while_loop_body_holding_an_if_scores_it_one_level_deeper() {
    // Arrange & Act
    let score = score_block("while b { if a {} }");

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn score_while_loop_inside_an_if_adds_its_nesting() {
    // Arrange & Act
    let score = score_block("if a { while b {} }");

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn score_while_loop_over_a_match_condition_adds_the_match_score() {
    // Arrange & Act
    let score = score_block("while match x { _ => true } {}");

    // Assert
    assert_eq!(score, 2);
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
