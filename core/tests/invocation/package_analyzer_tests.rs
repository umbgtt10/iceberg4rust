// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_source_tree::InMemorySourceTree;
use iceberg4rust::invocation::analyzer::Analyzer;
use iceberg4rust::invocation::package_analyzer::PackageAnalyzer;
use iceberg4rust::invocation::package_context::PackageContext;
use iceberg4rust::workspace::source_path::SourcePath;

const HIDES_MORE_PRIVATE_WORK: &str = r#"
pub struct Worker;

impl Worker {
    pub fn run(&self, v: u32) -> u32 {
        Self::step(v)
    }

    fn step(v: u32) -> u32 {
        if v > 1 { if v > 2 { 2 } else { 3 } } else { 4 }
    }

    fn other(v: u32) -> u32 {
        if v > 1 { 2 } else { 3 }
    }

    fn third(v: u32) -> u32 {
        if v > 1 { 2 } else { 3 }
    }
}
"#;
const HIDES_PRIVATE_WORK: &str = r#"
pub struct Worker;

impl Worker {
    pub fn run(&self, v: u32) -> u32 {
        Self::step_one(v) + Self::step_two(v)
    }

    pub(crate) fn step_one(v: u32) -> u32 {
        if v > 1 { 2 } else { 3 }
    }

    pub(crate) fn step_two(v: u32) -> u32 {
        if v > 2 { 4 } else { 5 }
    }
}
"#;

fn demo_package() -> PackageContext {
    PackageContext {
        name: String::from("demo"),
        manifest_dir: SourcePath::new(String::from("/demo")),
        source_roots: vec![SourcePath::new(String::from("/demo/src"))],
    }
}

#[test]
fn analyze_package_drops_a_file_that_scores_zero() {
    // Arrange
    let tree = InMemorySourceTree::new().with_file("/demo/src/lib.rs", "pub fn open() {}");
    let analyzer = PackageAnalyzer::new(&tree, Analyzer::new());

    // Act
    let reports = analyzer
        .analyze_package(&demo_package())
        .expect("analyze package");

    // Assert
    assert!(reports.is_empty());
}

#[test]
fn analyze_package_names_each_file_relative_to_the_manifest_directory() {
    // Arrange
    let tree = InMemorySourceTree::new().with_file("/demo/src/deep/lib.rs", HIDES_PRIVATE_WORK);
    let analyzer = PackageAnalyzer::new(&tree, Analyzer::new());

    // Act
    let reports = analyzer
        .analyze_package(&demo_package())
        .expect("analyze package");

    // Assert
    assert_eq!(reports[0].package_name(), "demo");
    assert_eq!(reports[0].relative_file(), "src/deep/lib.rs");
}

#[test]
fn analyze_package_over_a_source_root_returns_a_report_per_file() {
    // Arrange
    let tree = InMemorySourceTree::new()
        .with_file("/demo/src/lib.rs", HIDES_PRIVATE_WORK)
        .with_file("/demo/src/other.rs", HIDES_PRIVATE_WORK);
    let analyzer = PackageAnalyzer::new(&tree, Analyzer::new());

    // Act
    let reports = analyzer
        .analyze_package(&demo_package())
        .expect("analyze package");

    // Assert
    assert_eq!(reports.len(), 2);
}

#[test]
fn analyze_package_over_a_source_root_that_holds_nothing_returns_no_reports() {
    // Arrange
    let tree = InMemorySourceTree::new();
    let analyzer = PackageAnalyzer::new(&tree, Analyzer::new());

    // Act
    let reports = analyzer
        .analyze_package(&demo_package())
        .expect("analyze package");

    // Assert
    assert!(reports.is_empty());
}

#[test]
fn analyze_package_ranks_its_reports_by_descending_risk() {
    // Arrange
    let tree = InMemorySourceTree::new()
        .with_file("/demo/src/a.rs", HIDES_PRIVATE_WORK)
        .with_file("/demo/src/b.rs", HIDES_MORE_PRIVATE_WORK);
    let analyzer = PackageAnalyzer::new(&tree, Analyzer::new());

    // Act
    let reports = analyzer
        .analyze_package(&demo_package())
        .expect("analyze package");

    // Assert
    let order: Vec<&str> = reports
        .iter()
        .map(|report| report.relative_file())
        .collect();
    assert_eq!(order, vec!["src/b.rs", "src/a.rs"]);
}

#[test]
fn analyze_package_skips_a_test_tree_beneath_a_source_root() {
    // Arrange
    let package = PackageContext {
        name: String::from("demo"),
        manifest_dir: SourcePath::new(String::from("/demo")),
        source_roots: vec![SourcePath::new(String::from("/demo"))],
    };
    let tree = InMemorySourceTree::new()
        .with_file("/demo/tests/all_tests.rs", HIDES_PRIVATE_WORK)
        .with_file("/demo/build.rs", HIDES_PRIVATE_WORK);
    let analyzer = PackageAnalyzer::new(&tree, Analyzer::new());

    // Act
    let reports = analyzer.analyze_package(&package).expect("analyze package");

    // Assert
    assert!(reports.is_empty());
}

#[test]
fn analyze_package_with_a_file_that_does_not_parse_returns_an_error() {
    // Arrange
    let tree = InMemorySourceTree::new().with_file("/demo/src/lib.rs", "fn {");
    let analyzer = PackageAnalyzer::new(&tree, Analyzer::new());

    // Act
    let result = analyzer.analyze_package(&demo_package());

    // Assert
    assert_eq!(
        result.expect_err("unparseable").to_string(),
        "failed to parse Rust source"
    );
}

#[test]
fn analyze_package_with_an_unreadable_file_names_it_in_the_error() {
    // Arrange
    let tree = InMemorySourceTree::new().with_unreadable_file("/demo/src/lib.rs");
    let analyzer = PackageAnalyzer::new(&tree, Analyzer::new());

    // Act
    let result = analyzer.analyze_package(&demo_package());

    // Assert
    assert_eq!(
        result.expect_err("unreadable").to_string(),
        "failed to read source file /demo/src/lib.rs"
    );
}
