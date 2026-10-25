// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::invocation::package_context::PackageContext;
use iceberg4rust::workspace::source_path::SourcePath;

#[test]
fn package_context_clone_produces_independent_copy() {
    // Arrange
    let ctx = PackageContext {
        name: "pkg".to_string(),
        manifest_dir: SourcePath::new(String::from("/a")),
        source_roots: vec![
            SourcePath::new(String::from("/a/src")),
            SourcePath::new(String::from("/a/tests")),
        ],
    };

    // Act
    let cloned = ctx.clone();

    // Assert
    assert_eq!(cloned.name, ctx.name);
    assert_eq!(cloned.manifest_dir, ctx.manifest_dir);
    assert_eq!(cloned.source_roots, ctx.source_roots);
}
