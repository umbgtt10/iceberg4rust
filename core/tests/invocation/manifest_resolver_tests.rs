// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_workspace_reader::InMemoryWorkspaceReader;
use crate::support::in_memory_workspace_reader::library_package;
use iceberg4rust::invocation::config::Config;
use iceberg4rust::invocation::manifest_resolver::ManifestResolver;
use iceberg4rust::workspace::package_description::PackageDescription;
use iceberg4rust::workspace::source_path::SourcePath;
use iceberg4rust::workspace::target_description::TargetDescription;
use iceberg4rust::workspace::workspace_description::WorkspaceDescription;

fn config(packages: &[&str]) -> Config {
    Config {
        packages: packages.iter().map(|name| String::from(*name)).collect(),
        threshold: 15.0,
        top: 10,
    }
}

fn path(text: &str) -> SourcePath {
    SourcePath::new(String::from(text))
}

fn target_of_kind(kind: &str) -> TargetDescription {
    TargetDescription {
        kinds: vec![String::from(kind)],
        src_path: path("/project/src/lib.rs"),
    }
}

fn workspace(names: &[&str], root_package: Option<&str>) -> WorkspaceDescription {
    WorkspaceDescription {
        packages: names
            .iter()
            .map(|name| library_package(name, &format!("/project/{name}")))
            .collect(),
        root_package: root_package.map(String::from),
    }
}

#[test]
fn is_production_relative_file_benches_path_returns_false() {
    // Arrange & Act
    let result = ManifestResolver::is_production_relative_file("benches/bench.rs");

    // Assert
    assert!(!result);
}

#[test]
fn is_production_relative_file_build_rs_returns_false() {
    // Arrange & Act
    let result = ManifestResolver::is_production_relative_file("build.rs");

    // Assert
    assert!(!result);
}

#[test]
fn is_production_relative_file_deep_src_path_returns_true() {
    // Arrange & Act
    let result =
        ManifestResolver::is_production_relative_file("src/implementations/raft/client.rs");

    // Assert
    assert!(result);
}

#[test]
fn is_production_relative_file_examples_path_returns_false() {
    // Arrange & Act
    let result = ManifestResolver::is_production_relative_file("examples/demo.rs");

    // Assert
    assert!(!result);
}

#[test]
fn is_production_relative_file_src_path_returns_true() {
    // Arrange & Act
    let result = ManifestResolver::is_production_relative_file("src/lib.rs");

    // Assert
    assert!(result);
}

#[test]
fn is_production_relative_file_tests_nested_path_returns_false() {
    // Arrange & Act
    let result =
        ManifestResolver::is_production_relative_file("tests/implementations/raft/client_tests.rs");

    // Assert
    assert!(!result);
}

#[test]
fn is_production_relative_file_tests_path_returns_false() {
    // Arrange & Act
    let result = ManifestResolver::is_production_relative_file("tests/all_tests.rs");

    // Assert
    assert!(!result);
}

#[test]
fn is_production_target_for_a_binary_returns_true() {
    // Arrange
    let target = target_of_kind("bin");

    // Act & Assert
    assert!(ManifestResolver::is_production_target(&target));
}

#[test]
fn is_production_target_for_a_build_script_returns_false() {
    // Arrange
    let target = target_of_kind("custom-build");

    // Act & Assert
    assert!(!ManifestResolver::is_production_target(&target));
}

#[test]
fn is_production_target_for_a_lib_returns_true() {
    // Arrange
    let target = target_of_kind("lib");

    // Act & Assert
    assert!(ManifestResolver::is_production_target(&target));
}

#[test]
fn is_production_target_for_a_test_kind_beside_a_lib_kind_returns_false() {
    // Arrange -- an excluded kind wins over an included one, so a target that
    // is both never counts as production.
    let target = TargetDescription {
        kinds: vec![String::from("lib"), String::from("test")],
        src_path: path("/project/src/lib.rs"),
    };

    // Act & Assert
    assert!(!ManifestResolver::is_production_target(&target));
}

#[test]
fn is_production_target_for_a_test_returns_false() {
    // Arrange
    let target = target_of_kind("test");

    // Act & Assert
    assert!(!ManifestResolver::is_production_target(&target));
}

#[test]
fn relative_file_inside_base_dir_strips_prefix() {
    // Arrange
    let base_dir = path("/home/user/project");
    let file_path = path("/home/user/project/src/lib.rs");

    // Act
    let relative = ManifestResolver::relative_file(&base_dir, &file_path);

    // Assert
    assert_eq!(relative, "src/lib.rs");
}

// `\` is a path separator on Windows but an ordinary filename byte on Unix,
// so a `join()` argument that contains one lands differently on each: on
// Windows it splits into an extra component before stripping; on Unix it
// survives as part of one component's name. Either way the component(s)
// left after strip_prefix carry a literal `\` into the same replace('\\',
// "/") line, so this exercises normalization on every OS with one test.
#[test]
fn relative_file_normalizes_backslashes_to_forward_slashes() {
    // Arrange
    let base_dir = path("project");
    let file_path = base_dir.join("weird\\name.rs");

    // Act
    let relative = ManifestResolver::relative_file(&base_dir, &file_path);

    // Assert
    assert_eq!(relative, "weird/name.rs");
    assert!(!relative.contains('\\'));
}

#[test]
fn relative_file_outside_base_dir_returns_full_path() {
    // Arrange
    let base_dir = path("/home/user/project");
    let file_path = path("/tmp/other.rs");

    // Act
    let relative = ManifestResolver::relative_file(&base_dir, &file_path);

    // Assert
    assert_eq!(relative, "/tmp/other.rs");
}

#[test]
fn relative_file_same_path_returns_empty_string_or_dot() {
    // Arrange
    let base_dir = path("/home/user/project");
    let file_path = path("/home/user/project");

    // Act
    let relative = ManifestResolver::relative_file(&base_dir, &file_path);

    // Assert
    // strip_prefix("/home/user/project", "/home/user/project") yields ""
    // which to_string_lossy() renders as ""
    assert_eq!(relative, "");
}

#[test]
fn resolve_packages_for_a_package_with_no_production_target_falls_back_to_its_src() {
    // Arrange
    let reader = InMemoryWorkspaceReader::new(WorkspaceDescription {
        packages: vec![PackageDescription {
            name: String::from("bare"),
            manifest_path: path("/project/bare/Cargo.toml"),
            targets: vec![],
        }],
        root_package: None,
    });
    let resolver = ManifestResolver::new(&reader);

    // Act
    let packages = resolver
        .resolve_packages(&config(&["bare"]))
        .expect("resolve packages");

    // Assert
    assert_eq!(packages[0].source_roots, vec![path("/project/bare/src")]);
}

#[test]
fn resolve_packages_with_a_requested_package_returns_it_with_its_source_root() {
    // Arrange
    let reader = InMemoryWorkspaceReader::new(workspace(&["foo", "bar"], None));
    let resolver = ManifestResolver::new(&reader);

    // Act
    let packages = resolver
        .resolve_packages(&config(&["bar"]))
        .expect("resolve packages");

    // Assert
    assert_eq!(packages.len(), 1);
    assert_eq!(packages[0].name, "bar");
    assert_eq!(packages[0].manifest_dir, path("/project/bar"));
    assert_eq!(packages[0].source_roots, vec![path("/project/bar/src")]);
}

#[test]
fn resolve_packages_with_a_workspace_that_cannot_be_read_returns_the_error() {
    // Arrange
    let reader = InMemoryWorkspaceReader::failing();
    let resolver = ManifestResolver::new(&reader);

    // Act
    let result = resolver.resolve_packages(&config(&[]));

    // Assert
    assert!(result.is_err());
}

#[test]
fn select_packages_with_requested_name_returns_matching_package() {
    // Arrange
    let workspace = workspace(&["foo", "bar"], None);

    // Act
    let packages = ManifestResolver::select_packages(&workspace, &["bar".to_string()])
        .expect("select_packages");

    // Assert
    assert_eq!(packages.len(), 1);
    assert_eq!(packages[0].name, "bar");
}

#[test]
fn select_packages_with_unknown_name_returns_error() {
    // Arrange
    let workspace = workspace(&["foo"], None);

    // Act
    let result = ManifestResolver::select_packages(&workspace, &["nonexistent".to_string()]);

    // Assert
    assert!(result.is_err());
}

#[test]
fn select_packages_without_requested_and_without_a_root_asks_for_a_package() {
    // Arrange
    let workspace = workspace(&["foo", "bar"], None);

    // Act
    let result = ManifestResolver::select_packages(&workspace, &[]);

    // Assert
    assert_eq!(
        result.expect_err("no root").to_string(),
        "manifest contains multiple packages; pass --package <name>"
    );
}

#[test]
fn select_packages_without_requested_uses_root() {
    // Arrange
    let workspace = workspace(&["root-pkg"], Some("root-pkg"));

    // Act
    let packages = ManifestResolver::select_packages(&workspace, &[]).expect("select_packages");

    // Assert
    assert_eq!(packages.len(), 1);
    assert_eq!(packages[0].name, "root-pkg");
}
