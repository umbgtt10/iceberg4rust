// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::invocation::package_context::PackageContext;
use std::path::PathBuf;

#[test]
fn package_context_clone_produces_independent_copy() {
    // Arrange
    let ctx = PackageContext {
        name: "pkg".to_string(),
        manifest_dir: PathBuf::from("/a"),
        source_roots: vec![PathBuf::from("/a/src"), PathBuf::from("/a/tests")],
    };

    // Act
    let cloned = ctx.clone();

    // Assert
    assert_eq!(cloned.name, ctx.name);
    assert_eq!(cloned.manifest_dir, ctx.manifest_dir);
    assert_eq!(cloned.source_roots, ctx.source_roots);
}

#[test]
fn package_context_holds_provided_values() {
    // Arrange & Act
    let ctx = PackageContext {
        name: "my-crate".to_string(),
        manifest_dir: PathBuf::from("/home/project"),
        source_roots: vec![PathBuf::from("/home/project/src")],
    };

    // Assert
    assert_eq!(ctx.name, "my-crate");
    assert_eq!(ctx.manifest_dir, PathBuf::from("/home/project"));
    assert_eq!(ctx.source_roots, vec![PathBuf::from("/home/project/src")]);
}
