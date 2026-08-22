// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::analysis::helper_struct_kind::HelperStructKind;
use iceberg4rust::analysis::private_function_collector::PrivateFunctionCollector;
use std::collections::BTreeMap;
use syn::Item;
use syn::parse_file;

fn parse_items(source: &str) -> Vec<Item> {
    parse_file(source).expect("failed to parse source").items
}

#[test]
fn collect_captures_the_function_name_and_its_line() {
    // Arrange
    let items = parse_items(
        r#"
struct Worker;

impl Worker {
    fn step(&self) {}
}
"#,
    );
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name, "step");
    assert_eq!(functions[0].line, 5);
}

#[test]
fn collect_captures_the_name_and_line_of_a_free_function() {
    // Arrange
    let items = parse_items("\n\nfn helper() -> u32 { 42 }");
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions[0].name, "helper");
    assert_eq!(functions[0].line, 3);
}

#[test]
fn collect_cfg_test_module_is_excluded() {
    // Arrange
    let items = parse_items(
        r#"
#[cfg(test)]
mod tests {
    fn internal_helper() {}
}
"#,
    );
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert!(functions.is_empty());
}

// Every restricted form counts; only bare `pub` escapes.
#[test]
fn collect_counts_every_restricted_form_and_excludes_only_pub() {
    // Arrange
    let items = parse_items(
        r#"
fn plain() -> u32 { 1 }
pub(crate) fn crate_wide() -> u32 { 2 }
pub(self) fn only_here() -> u32 { 3 }
pub(in crate::inner) fn scoped() -> u32 { 4 }
pub fn open() -> u32 { 5 }
"#,
    );
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    let names: Vec<&str> = functions.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(names, vec!["plain", "crate_wide", "only_here", "scoped"]);
}

#[test]
fn collect_empty_items_returns_empty() {
    // Arrange
    let items = parse_items("");
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert!(functions.is_empty());
}

#[test]
fn collect_function_tracks_used_private_structs() {
    // Arrange
    let mut helper_structs = BTreeMap::new();
    helper_structs.insert("Config".to_string(), HelperStructKind::Data);

    let items = parse_items(
        r#"
fn configure() -> Config {
    Config
}
"#,
    );

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions.len(), 1);
    assert!(functions[0].used_private_structs.contains("Config"));
}

#[test]
fn collect_gives_each_function_its_own_line() {
    // Arrange
    let items = parse_items(
        r#"
fn first() {}

fn second() {}
"#,
    );
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions[0].line, 2);
    assert_eq!(functions[1].line, 4);
}

#[test]
fn collect_inherent_impl_method_is_hidden() {
    // Arrange
    let items = parse_items(
        r#"
struct Worker;

impl Worker {
    fn step(&self) {}
}
"#,
    );
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions.len(), 1);
    assert!(functions[0].is_hidden);
}

#[test]
fn collect_mixed_inherent_and_trait_methods_marks_each_correctly() {
    // Arrange
    let items = parse_items(
        r#"
struct Worker;

impl Worker {
    fn step(&self) {}
}

impl Watching for Worker {
    fn on_event(&self) {}
}
"#,
    );
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions.len(), 2);
    assert_eq!(functions.iter().filter(|f| f.is_hidden).count(), 1);
    assert_eq!(functions.iter().filter(|f| !f.is_hidden).count(), 1);
}

#[test]
fn collect_private_fn_and_impl_method_together() {
    // Arrange
    let items = parse_items(
        r#"
fn top_level() {}

pub struct Worker;

impl Worker {
    fn internal() {}
}
"#,
    );
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions.len(), 2);
}

#[test]
fn collect_private_free_function_is_hidden() {
    // Arrange
    let items = parse_items("fn helper() -> u32 { 42 }");
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions.len(), 1);
    assert!(functions[0].is_hidden);
}

#[test]
fn collect_private_free_function_is_included() {
    // Arrange
    let items = parse_items("fn helper() -> u32 { 42 }");
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions.len(), 1);
}

#[test]
fn collect_private_function_in_non_test_module_is_included() {
    // Arrange
    let items = parse_items(
        r#"
mod inner {
    fn hidden() -> u32 { 42 }
}
"#,
    );
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions.len(), 1);
}

#[test]
fn collect_private_impl_method_is_included() {
    // Arrange
    let items = parse_items(
        r#"
pub struct Worker;

impl Worker {
    fn assist() -> u32 { 42 }
}
"#,
    );
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions.len(), 1);
}

#[test]
fn collect_pub_crate_free_function_carries_its_complexity() {
    // Arrange
    let items = parse_items("pub(crate) fn helper(v: u32) -> u32 { if v > 1 { 2 } else { 3 } }");
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions[0].complexity, 1);
}

// Restricted visibility is not reachable from another crate, and this project's
// tests are their own crate, so a `pub(crate)` function is hidden implementation
// exactly as a bare `fn` is. Only `pub` is genuinely reachable.
#[test]
fn collect_pub_crate_free_function_is_included() {
    // Arrange
    let items = parse_items("pub(crate) fn helper() -> u32 { 42 }");
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions.len(), 1);
}

// An inherent impl is the file's own implementation, so a restricted method on
// it is hidden and counts toward the private surface, not merely its complexity.
#[test]
fn collect_pub_crate_impl_method_is_hidden() {
    // Arrange
    let items = parse_items(
        r#"
pub struct Worker;

impl Worker {
    pub(crate) fn assist() -> u32 { 42 }
}
"#,
    );
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert!(functions[0].is_hidden);
}

#[test]
fn collect_pub_crate_impl_method_is_included() {
    // Arrange
    let items = parse_items(
        r#"
pub struct Worker;

impl Worker {
    pub(crate) fn assist() -> u32 { 42 }
}
"#,
    );
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions.len(), 1);
}

#[test]
fn collect_pub_in_path_free_function_is_included() {
    // Arrange
    let items = parse_items("pub(in crate::inner) fn helper() -> u32 { 42 }");
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions.len(), 1);
}

#[test]
fn collect_pub_self_free_function_is_included() {
    // Arrange
    let items = parse_items("pub(self) fn helper() -> u32 { 42 }");
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions.len(), 1);
}

#[test]
fn collect_pub_super_function_inside_a_module_is_included() {
    // Arrange
    let items = parse_items(
        r#"
mod inner {
    pub(super) fn helper() -> u32 { 42 }
}
"#,
    );
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions.len(), 1);
}

#[test]
fn collect_public_free_function_is_excluded() {
    // Arrange
    let items = parse_items("pub fn helper() -> u32 { 42 }");
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert!(functions.is_empty());
}

#[test]
fn collect_public_impl_method_is_excluded() {
    // Arrange
    let items = parse_items(
        r#"
pub struct Worker;

impl Worker {
    pub fn assist() -> u32 { 42 }
}
"#,
    );
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert!(functions.is_empty());
}

#[test]
fn collect_test_function_is_excluded() {
    // Arrange
    let items = parse_items(
        r#"
#[test]
fn test_thing() {
    assert!(true);
}
"#,
    );
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert!(functions.is_empty());
}

// A trait impl method is reachable by anyone holding the trait, so it is not
// hidden implementation however little visibility syntax it carries. Its
// complexity still counts — a heavy `next` or `poll` is real burden.
#[test]
fn collect_trait_impl_method_is_not_hidden() {
    // Arrange
    let items = parse_items(
        r#"
struct Observer;

impl Watching for Observer {
    fn on_event(&self) {}
}
"#,
    );
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions.len(), 1);
    assert!(!functions[0].is_hidden);
}

#[test]
fn collect_trait_impl_method_still_carries_its_complexity() {
    // Arrange
    let items = parse_items(
        r#"
struct Observer;

impl Watching for Observer {
    fn on_event(&self, flag: bool) {
        if flag {
            act();
        }
    }
}
"#,
    );
    let helper_structs = BTreeMap::new();

    // Act
    let functions = PrivateFunctionCollector::new(&helper_structs).collect(&items);

    // Assert
    assert_eq!(functions.len(), 1);
    assert!(!functions[0].is_hidden);
    assert!(functions[0].complexity > 0);
}
