// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::analysis::test_attr_checker::TestAttrChecker;
use syn::Attribute;
use syn::parse_quote;

fn checker() -> TestAttrChecker {
    TestAttrChecker::new()
}

#[test]
fn allow_attr_returns_false() {
    // Arrange
    let attrs = vec![parse_quote!(#[allow(unused)])];

    // Act
    let result = checker().has_test_attrs(&attrs);

    // Assert
    assert!(!result);
}

#[test]
fn cfg_test_attr_returns_true() {
    // Arrange
    let attrs = vec![parse_quote!(#[cfg(test)])];

    // Act
    let result = checker().has_test_attrs(&attrs);

    // Assert
    assert!(result);
}

#[test]
fn cfg_test_in_nested_meta_is_not_detected() {
    // Arrange
    let attrs = vec![parse_quote!(#[cfg_attr(feature = "test", ignore)])];

    // Act
    let result = checker().has_test_attrs(&attrs);

    // Assert
    assert!(!result);
}

#[test]
fn derive_attr_returns_false() {
    // Arrange
    let attrs = vec![parse_quote!(#[derive(Debug)])];

    // Act
    let result = checker().has_test_attrs(&attrs);

    // Assert
    assert!(!result);
}

#[test]
fn empty_attrs_returns_false() {
    // Arrange
    let attrs: Vec<Attribute> = vec![];

    // Act
    let result = checker().has_test_attrs(&attrs);

    // Assert
    assert!(!result);
}

#[test]
fn multiple_attrs_one_is_test_returns_true() {
    // Arrange
    let attrs = vec![
        parse_quote!(#[allow(unused)]),
        parse_quote!(#[test]),
        parse_quote!(#[derive(Clone)]),
    ];

    // Act
    let result = checker().has_test_attrs(&attrs);

    // Assert
    assert!(result);
}

#[test]
fn should_panic_attr_returns_false() {
    // Arrange
    let attrs = vec![parse_quote!(#[should_panic])];

    // Act
    let result = checker().has_test_attrs(&attrs);

    // Assert
    assert!(!result);
}

#[test]
fn test_attr_returns_true() {
    // Arrange
    let attrs = vec![parse_quote!(#[test])];

    // Act
    let result = checker().has_test_attrs(&attrs);

    // Assert
    assert!(result);
}
