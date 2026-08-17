// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;

use iceberg4rust::manifest_resolver::ManifestResolver;

#[test]
fn relative_file_inside_base_dir_strips_prefix() {
    // Arrange
    let base_dir = Path::new("/home/user/project");
    let file_path = Path::new("/home/user/project/src/lib.rs");

    // Act
    let relative = ManifestResolver::relative_file(base_dir, file_path);

    // Assert
    assert_eq!(relative, "src/lib.rs");
}

#[test]
fn relative_file_outside_base_dir_returns_full_path() {
    // Arrange
    let base_dir = Path::new("/home/user/project");
    let file_path = Path::new("/tmp/other.rs");

    // Act
    let relative = ManifestResolver::relative_file(base_dir, file_path);

    // Assert
    assert_eq!(relative, "/tmp/other.rs");
}

#[test]
fn relative_file_normalizes_backslashes_to_forward_slashes() {
    // Arrange
    let base_dir = Path::new("C:\\Users\\user\\project");
    let file_path = Path::new("C:\\Users\\user\\project\\src\\lib.rs");

    // Act
    let relative = ManifestResolver::relative_file(base_dir, file_path);

    // Assert
    assert_eq!(relative, "src/lib.rs");
    assert!(!relative.contains('\\'));
}

#[test]
fn relative_file_same_path_returns_empty_string_or_dot() {
    // Arrange
    let base_dir = Path::new("/home/user/project");
    let file_path = Path::new("/home/user/project");

    // Act
    let relative = ManifestResolver::relative_file(base_dir, file_path);

    // Assert
    // strip_prefix("/home/user/project", "/home/user/project") yields ""
    // which to_string_lossy() renders as ""
    assert_eq!(relative, "");
}

#[test]
fn is_production_relative_file_src_path_returns_true() {
    // Arrange & Act
    let result = ManifestResolver::is_production_relative_file("src/lib.rs");

    // Assert
    assert!(result);
}

#[test]
fn is_production_relative_file_tests_path_returns_false() {
    // Arrange & Act
    let result = ManifestResolver::is_production_relative_file("tests/all_tests.rs");

    // Assert
    assert!(!result);
}

#[test]
fn is_production_relative_file_examples_path_returns_false() {
    // Arrange & Act
    let result = ManifestResolver::is_production_relative_file("examples/demo.rs");

    // Assert
    assert!(!result);
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
fn is_production_relative_file_tests_nested_path_returns_false() {
    // Arrange & Act
    let result =
        ManifestResolver::is_production_relative_file("tests/implementations/raft/client_tests.rs");

    // Assert
    assert!(!result);
}

#[test]
fn select_packages_with_requested_name_returns_matching_package() {
    // Arrange
    let metadata: cargo_metadata::Metadata = serde_json::from_value(serde_json::json!({
        "packages": [
            {
                "name": "foo",
                "version": "0.1.0",
                "id": "foo 0.1.0 (path+file:///project/foo)",
                "license": null,
                "license_file": null,
                "description": null,
                "source": null,
                "dependencies": [],
                "targets": [],
                "features": {},
                "manifest_path": "/project/foo/Cargo.toml",
                "metadata": null,
                "publish": null,
                "authors": [],
                "categories": [],
                "keywords": [],
                "readme": null,
                "repository": null,
                "homepage": null,
                "documentation": null,
                "edition": "2021",
                "links": null
            },
            {
                "name": "bar",
                "version": "0.2.0",
                "id": "bar 0.2.0 (path+file:///project/bar)",
                "license": null,
                "license_file": null,
                "description": null,
                "source": null,
                "dependencies": [],
                "targets": [],
                "features": {},
                "manifest_path": "/project/bar/Cargo.toml",
                "metadata": null,
                "publish": null,
                "authors": [],
                "categories": [],
                "keywords": [],
                "readme": null,
                "repository": null,
                "homepage": null,
                "documentation": null,
                "edition": "2021",
                "links": null
            }
        ],
        "workspace_members": [],
        "workspace_default_members": [],
        "resolve": null,
        "target_directory": "/project/target",
        "version": 1,
        "workspace_root": "/project",
        "metadata": null
    }))
    .expect("valid metadata");

    // Act
    let packages = ManifestResolver::select_packages(&metadata, &["bar".to_string()])
        .expect("select_packages");

    // Assert
    assert_eq!(packages.len(), 1);
    assert_eq!(packages[0].name, "bar");
}

#[test]
fn select_packages_without_requested_uses_root() {
    // Arrange
    let metadata: cargo_metadata::Metadata = serde_json::from_value(serde_json::json!({
        "packages": [
            {
                "name": "root-pkg",
                "version": "0.1.0",
                "id": "root-pkg 0.1.0 (path+file:///project)",
                "license": null,
                "license_file": null,
                "description": null,
                "source": null,
                "dependencies": [],
                "targets": [],
                "features": {},
                "manifest_path": "/project/Cargo.toml",
                "metadata": null,
                "publish": null,
                "authors": [],
                "categories": [],
                "keywords": [],
                "readme": null,
                "repository": null,
                "homepage": null,
                "documentation": null,
                "edition": "2021",
                "links": null
            }
        ],
        "workspace_members": ["root-pkg 0.1.0 (path+file:///project)"],
        "workspace_default_members": ["root-pkg 0.1.0 (path+file:///project)"],
        "resolve": {
            "nodes": [
                {
                    "id": "root-pkg 0.1.0 (path+file:///project)",
                    "dependencies": [],
                    "deps": []
                }
            ],
            "root": "root-pkg 0.1.0 (path+file:///project)"
        },
        "target_directory": "/project/target",
        "version": 1,
        "workspace_root": "/project",
        "metadata": null
    }))
    .expect("valid metadata");

    // Act
    let packages = ManifestResolver::select_packages(&metadata, &[]).expect("select_packages");

    // Assert
    assert_eq!(packages.len(), 1);
    assert_eq!(packages[0].name, "root-pkg");
}

#[test]
fn select_packages_with_unknown_name_returns_error() {
    // Arrange
    let metadata: cargo_metadata::Metadata = serde_json::from_value(serde_json::json!({
        "packages": [
            {
                "name": "foo",
                "version": "0.1.0",
                "id": "foo 0.1.0 (path+file:///project/foo)",
                "license": null,
                "license_file": null,
                "description": null,
                "source": null,
                "dependencies": [],
                "targets": [],
                "features": {},
                "manifest_path": "/project/foo/Cargo.toml",
                "metadata": null,
                "publish": null,
                "authors": [],
                "categories": [],
                "keywords": [],
                "readme": null,
                "repository": null,
                "homepage": null,
                "documentation": null,
                "edition": "2021",
                "links": null
            }
        ],
        "workspace_members": [],
        "workspace_default_members": [],
        "resolve": null,
        "target_directory": "/project/target",
        "version": 1,
        "workspace_root": "/project",
        "metadata": null
    }))
    .expect("valid metadata");

    // Act
    let result = ManifestResolver::select_packages(&metadata, &["nonexistent".to_string()]);

    // Assert
    assert!(result.is_err());
}
