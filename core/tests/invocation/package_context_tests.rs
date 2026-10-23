// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::invocation::package_context::PackageContext;
use std::path::PathBuf;

#[test]
fn package_context_clone_produces_independent_copy() {
    // Arrange
    let ctx = PackageContext::new(
        "pkg".to_string(),
        PathBuf::from("/a"),
        vec![PathBuf::from("/a/src"), PathBuf::from("/a/tests")],
    );

    // Act
    let cloned = ctx.clone();

    // Assert
    assert_eq!(cloned.name(), ctx.name());
    assert_eq!(cloned.manifest_dir(), ctx.manifest_dir());
    assert_eq!(cloned.source_roots(), ctx.source_roots());
}

#[test]
fn package_context_holds_provided_values() {
    // Arrange & Act
    let ctx = PackageContext::new(
        "my-crate".to_string(),
        PathBuf::from("/home/project"),
        vec![PathBuf::from("/home/project/src")],
    );

    // Assert
    assert_eq!(ctx.name(), "my-crate");
    assert_eq!(ctx.manifest_dir(), PathBuf::from("/home/project"));
    assert_eq!(ctx.source_roots(), vec![PathBuf::from("/home/project/src")]);
}
