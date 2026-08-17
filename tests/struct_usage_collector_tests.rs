// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeMap;

use syn::{Block, File, Item, Signature, parse_file};

use iceberg4rust::helper_struct_kind::HelperStructKind;
use iceberg4rust::struct_usage_collector::collect_used_private_structs;

fn empty_helpers() -> BTreeMap<String, HelperStructKind> {
    BTreeMap::new()
}

fn helpers_with(names: &[&str], kind: HelperStructKind) -> BTreeMap<String, HelperStructKind> {
    names.iter().map(|n| (n.to_string(), kind)).collect()
}

fn collect_used(source: &str, helper_structs: &BTreeMap<String, HelperStructKind>) -> Vec<String> {
    let file: File = parse_file(source).expect("failed to parse source");
    let (sig, block) = first_fn_sig_and_block(&file);
    let mut result: Vec<String> = collect_used_private_structs(sig, block, helper_structs)
        .into_iter()
        .collect();
    result.sort();
    result
}

fn first_fn_sig_and_block(file: &File) -> (&Signature, &Block) {
    for item in &file.items {
        if let Item::Fn(f) = item {
            return (&f.sig, &f.block);
        }
    }
    panic!("no function found in source");
}

#[test]
fn empty_body_returns_empty() {
    // Arrange
    let helpers = empty_helpers();

    // Act
    let used = collect_used("fn foo() {}", &helpers);

    // Assert
    assert!(used.is_empty());
}

#[test]
fn return_type_with_private_struct_is_detected() {
    // Arrange
    let helpers = helpers_with(&["Config"], HelperStructKind::Data);

    // Act
    let used = collect_used("fn foo() -> Config { Config }", &helpers);

    // Assert
    assert_eq!(used, vec!["Config"]);
}

#[test]
fn parameter_type_with_private_struct_is_detected() {
    // Arrange
    let helpers = helpers_with(&["State"], HelperStructKind::Data);

    // Act
    let used = collect_used("fn foo(s: State) {}", &helpers);

    // Assert
    assert_eq!(used, vec!["State"]);
}

#[test]
fn local_variable_type_annotation_is_detected() {
    // Arrange
    let helpers = helpers_with(&["Helper"], HelperStructKind::Behavioral);

    // Act
    let used = collect_used("fn foo() { let x: Helper; }", &helpers);

    // Assert
    assert_eq!(used, vec!["Helper"]);
}

#[test]
fn struct_literal_expression_is_detected() {
    // Arrange
    let helpers = helpers_with(&["Config"], HelperStructKind::Data);

    // Act
    let used = collect_used("fn foo() -> Config { Config { field: 1 } }", &helpers);

    // Assert
    assert_eq!(used, vec!["Config"]);
}

#[test]
fn type_annotation_in_body_is_detected() {
    // Arrange
    let helpers = helpers_with(&["Builder"], HelperStructKind::Behavioral);

    // Act
    let used = collect_used("fn foo() { let x: Builder; }", &helpers);

    // Assert
    assert_eq!(used, vec!["Builder"]);
}

#[test]
fn struct_not_in_helpers_is_not_reported() {
    // Arrange
    let helpers = empty_helpers();

    // Act
    let used = collect_used("fn foo() -> External { External }", &helpers);

    // Assert
    assert!(used.is_empty());
}

#[test]
fn multiple_structs_are_all_detected() {
    // Arrange
    let mut helpers = BTreeMap::new();
    helpers.insert("Input".to_string(), HelperStructKind::Data);
    helpers.insert("Output".to_string(), HelperStructKind::Data);

    // Act
    let used = collect_used(
        "fn foo(input: Input) -> Output { Output { value: input } }",
        &helpers,
    );

    // Assert
    assert_eq!(used, vec!["Input", "Output"]);
}

#[test]
fn return_type_self_is_not_mistaken_for_helpers() {
    // Arrange
    let helpers = helpers_with(&["Self_"], HelperStructKind::Data);

    // Act
    let used = collect_used("fn foo() -> Self { Self }", &helpers);

    // Assert
    assert!(used.is_empty());
}

#[test]
fn only_matching_structs_from_helpers_are_reported() {
    // Arrange
    let helpers = helpers_with(&["Internal"], HelperStructKind::Data);

    // Act
    let used = collect_used(
        "fn foo() -> Internal { let x: External = External::new(); x }",
        &helpers,
    );

    // Assert
    assert_eq!(used, vec!["Internal"]);
}
