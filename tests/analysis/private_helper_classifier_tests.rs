// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::analysis::helper_struct_kind::HelperStructKind;
use iceberg4rust::analysis::private_helper_classifier::PrivateHelperClassifier;
use syn::Item;
use syn::parse_file;

fn parse_items(source: &str) -> Vec<Item> {
    let syntax = parse_file(source).expect("failed to parse source");
    syntax.items
}

#[test]
fn cfg_test_module_is_skipped() {
    // Arrange
    let items = parse_items(
        r#"
pub struct Worker;
struct Helper;

impl Helper {
    fn assist() {}
}

#[cfg(test)]
mod tests {
    struct InternalHelper;

    impl InternalHelper {
        fn do_work() {}
    }
}
"#,
    );

    // Act
    let map = PrivateHelperClassifier::new(&items, None).into_map();

    // Assert
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("Helper"), Some(&HelperStructKind::Behavioral));
    assert!(!map.contains_key("InternalHelper"));
}

#[test]
fn empty_items_returns_empty_map() {
    // Arrange
    let items = parse_items("");

    // Act
    let map = PrivateHelperClassifier::new(&items, None).into_map();

    // Assert
    assert!(map.is_empty());
}

#[test]
fn impl_without_matching_struct_is_ignored() {
    // Arrange
    let items = parse_items(
        r#"
pub struct Worker;

impl SomeExternalType {
    fn do_work() {}
}
"#,
    );

    // Act
    let map = PrivateHelperClassifier::new(&items, None).into_map();

    // Assert
    assert!(map.is_empty());
}

#[test]
fn inline_module_impls_are_recursed_into() {
    // Arrange
    let items = parse_items(
        r#"
pub mod outer {
    pub struct Worker;
    struct InnerHelper;

    impl InnerHelper {
        fn do_work() {}
    }
}
"#,
    );

    // Act
    let map = PrivateHelperClassifier::new(&items, None).into_map();

    // Assert
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("InnerHelper"), Some(&HelperStructKind::Behavioral));
}

#[test]
fn inline_module_structs_are_recursed_into() {
    // Arrange
    let items = parse_items(
        r#"
pub mod outer {
    pub struct Worker;
    struct InnerHelper;
}
"#,
    );

    // Act
    let map = PrivateHelperClassifier::new(&items, None).into_map();

    // Assert
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("InnerHelper"), Some(&HelperStructKind::Data));
}

#[test]
fn multiple_private_structs_are_all_collected() {
    // Arrange
    let items = parse_items(
        r#"
pub struct Worker;
struct Alpha;
struct Beta;

impl Beta {
    fn action() {}
}
"#,
    );

    // Act
    let map = PrivateHelperClassifier::new(&items, None).into_map();

    // Assert
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("Alpha"), Some(&HelperStructKind::Data));
    assert_eq!(map.get("Beta"), Some(&HelperStructKind::Behavioral));
}

#[test]
fn primary_struct_is_excluded() {
    // Arrange
    let items = parse_items(
        r#"
pub struct Primary;
struct Helper;
"#,
    );

    // Act
    let map = PrivateHelperClassifier::new(&items, Some("Primary")).into_map();

    // Assert
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("Helper"), Some(&HelperStructKind::Data));
    assert!(!map.contains_key("Primary"));
}

#[test]
fn primary_struct_with_same_name_is_excluded_even_if_private() {
    // Arrange
    let items = parse_items(
        r#"
struct Primary;
struct Helper;
"#,
    );

    // Act
    let map = PrivateHelperClassifier::new(&items, Some("Primary")).into_map();

    // Assert
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("Helper"), Some(&HelperStructKind::Data));
    assert!(!map.contains_key("Primary"));
}

#[test]
fn private_struct_with_methods_is_behavioral() {
    // Arrange
    let items = parse_items(
        r#"
pub struct Worker;
struct Helper;

impl Helper {
    fn assist() {}
}
"#,
    );

    // Act
    let map = PrivateHelperClassifier::new(&items, None).into_map();

    // Assert
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("Helper"), Some(&HelperStructKind::Behavioral));
}

#[test]
fn private_struct_with_mixed_methods_is_behavioral() {
    // Arrange
    let items = parse_items(
        r#"
pub struct Worker;
struct Helper;

impl Helper {
    pub fn visible() {}
    fn hidden() {}
}
"#,
    );

    // Act
    let map = PrivateHelperClassifier::new(&items, None).into_map();

    // Assert
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("Helper"), Some(&HelperStructKind::Behavioral));
}

#[test]
fn private_struct_with_only_pub_methods_is_behavioral() {
    // Arrange
    let items = parse_items(
        r#"
pub struct Worker;
struct Helper;

impl Helper {
    pub fn assist() {}
}
"#,
    );

    // Act
    let map = PrivateHelperClassifier::new(&items, None).into_map();

    // Assert
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("Helper"), Some(&HelperStructKind::Behavioral));
}

#[test]
fn private_struct_without_impl_is_data() {
    // Arrange
    let items = parse_items(
        r#"
pub struct Worker;
struct Helper;
"#,
    );

    // Act
    let map = PrivateHelperClassifier::new(&items, None).into_map();

    // Assert
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("Helper"), Some(&HelperStructKind::Data));
}

#[test]
fn public_structs_are_not_included() {
    // Arrange
    let items = parse_items(
        r#"
pub struct Worker;
pub struct Helper;
"#,
    );

    // Act
    let map = PrivateHelperClassifier::new(&items, None).into_map();

    // Assert
    assert!(map.is_empty());
}
