// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::analysis::source_root_collector::SourceRootCollector;
use iceberg4rust::workspace::source_path::SourcePath;
use iceberg4rust::workspace::target_description::TargetDescription;

fn bench_target(src_path: &str) -> TargetDescription {
    TargetDescription {
        kinds: vec![String::from("bench")],
        src_path: SourcePath::new(String::from(src_path)),
    }
}

fn bin_target(src_path: &str) -> TargetDescription {
    TargetDescription {
        kinds: vec![String::from("bin")],
        src_path: SourcePath::new(String::from(src_path)),
    }
}

fn build_target(src_path: &str) -> TargetDescription {
    TargetDescription {
        kinds: vec![String::from("custom-build")],
        src_path: SourcePath::new(String::from(src_path)),
    }
}

fn example_target(src_path: &str) -> TargetDescription {
    TargetDescription {
        kinds: vec![String::from("example")],
        src_path: SourcePath::new(String::from(src_path)),
    }
}

fn lib_target(src_path: &str) -> TargetDescription {
    TargetDescription {
        kinds: vec![String::from("lib")],
        src_path: SourcePath::new(String::from(src_path)),
    }
}

fn test_target(src_path: &str) -> TargetDescription {
    TargetDescription {
        kinds: vec![String::from("test")],
        src_path: SourcePath::new(String::from(src_path)),
    }
}

#[test]
fn collect_from_targets_bench_target_is_skipped() {
    // Arrange
    let target = bench_target("/home/user/project/benches/bench.rs");

    // Act
    let mut collector = SourceRootCollector::new();
    collector.collect_from_targets(&[target]);

    // Assert
    let roots = collector.into_roots();
    assert!(roots.is_empty());
}

#[test]
fn collect_from_targets_bin_target_adds_parent_dir() {
    // Arrange
    let target = bin_target("/home/user/project/src/main.rs");

    // Act
    let mut collector = SourceRootCollector::new();
    collector.collect_from_targets(&[target]);

    // Assert
    let roots = collector.into_roots();
    assert_eq!(
        roots,
        vec![SourcePath::new(String::from("/home/user/project/src"))]
    );
}

#[test]
fn collect_from_targets_build_target_is_skipped() {
    // Arrange
    let target = build_target("/home/user/project/build.rs");

    // Act
    let mut collector = SourceRootCollector::new();
    collector.collect_from_targets(&[target]);

    // Assert
    let roots = collector.into_roots();
    assert!(roots.is_empty());
}

#[test]
fn collect_from_targets_example_target_is_skipped() {
    // Arrange
    let target = example_target("/home/user/project/examples/demo.rs");

    // Act
    let mut collector = SourceRootCollector::new();
    collector.collect_from_targets(&[target]);

    // Assert
    let roots = collector.into_roots();
    assert!(roots.is_empty());
}

#[test]
fn collect_from_targets_lib_target_adds_parent_dir() {
    // Arrange
    let target = lib_target("/home/user/project/src/lib.rs");

    // Act
    let mut collector = SourceRootCollector::new();
    collector.collect_from_targets(&[target]);

    // Assert
    let roots = collector.into_roots();
    assert_eq!(
        roots,
        vec![SourcePath::new(String::from("/home/user/project/src"))]
    );
}

#[test]
fn collect_from_targets_multiple_distinct_roots() {
    // Arrange
    let primary = lib_target("/home/user/project/src/lib.rs");
    let secondary = lib_target("/home/user/project/other/mod.rs");

    // Act
    let mut collector = SourceRootCollector::new();
    collector.collect_from_targets(&[primary, secondary]);

    // Assert
    let mut roots = collector.into_roots();
    roots.sort();
    assert_eq!(
        roots,
        vec![
            SourcePath::new(String::from("/home/user/project/other")),
            SourcePath::new(String::from("/home/user/project/src")),
        ]
    );
}

#[test]
fn collect_from_targets_multiple_targets_produces_deduped_roots() {
    // Arrange
    let lib = lib_target("/home/user/project/src/lib.rs");
    let bin = bin_target("/home/user/project/src/main.rs");
    let helper = lib_target("/home/user/project/src/helper.rs");

    // Act
    let mut collector = SourceRootCollector::new();
    collector.collect_from_targets(&[lib, bin, helper]);

    // Assert
    let roots = collector.into_roots();
    assert_eq!(
        roots,
        vec![SourcePath::new(String::from("/home/user/project/src"))]
    );
}

#[test]
fn collect_from_targets_non_rs_extension_is_skipped() {
    // Arrange
    let target = lib_target("/home/user/project/src/lib.c");

    // Act
    let mut collector = SourceRootCollector::new();
    collector.collect_from_targets(&[target]);

    // Assert
    let roots = collector.into_roots();
    assert!(roots.is_empty());
}

#[test]
fn collect_from_targets_test_target_is_skipped() {
    // Arrange
    let target = test_target("/home/user/project/tests/integration.rs");

    // Act
    let mut collector = SourceRootCollector::new();
    collector.collect_from_targets(&[target]);

    // Assert
    let roots = collector.into_roots();
    assert!(roots.is_empty());
}

#[test]
fn collector_new_is_empty() {
    // Arrange
    let collector = SourceRootCollector::new();

    // Act
    let roots = collector.into_roots();

    // Assert
    assert!(roots.is_empty());
}

#[test]
fn ensure_fallback_empty_collector_adds_src_dir() {
    // Arrange
    let manifest_dir = SourcePath::new(String::from("/home/user/project"));

    // Act
    let mut collector = SourceRootCollector::new();
    collector.ensure_fallback(&manifest_dir);

    // Assert
    let roots = collector.into_roots();
    assert_eq!(
        roots,
        vec![SourcePath::new(String::from("/home/user/project/src"))]
    );
}

#[test]
fn ensure_fallback_non_empty_collector_does_not_add_fallback() {
    // Arrange
    let target = lib_target("/home/user/project/src/lib.rs");
    let manifest_dir = SourcePath::new(String::from("/home/user/project"));
    let mut collector = SourceRootCollector::new();
    collector.collect_from_targets(&[target]);

    // Act
    collector.ensure_fallback(&manifest_dir);

    // Assert
    let roots = collector.into_roots();
    assert_eq!(
        roots,
        vec![SourcePath::new(String::from("/home/user/project/src"))]
    );
}
