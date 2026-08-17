// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::helper_struct_kind::HelperStructKind;

#[test]
fn data_variant_exists() {
    // Arrange & Act
    let kind = HelperStructKind::Data;

    // Assert
    assert!(matches!(kind, HelperStructKind::Data));
}

#[test]
fn behavioral_variant_exists() {
    // Arrange & Act
    let kind = HelperStructKind::Behavioral;

    // Assert
    assert!(matches!(kind, HelperStructKind::Behavioral));
}

#[test]
fn data_and_behavioral_are_not_equal() {
    // Arrange
    let data = HelperStructKind::Data;
    let behavioral = HelperStructKind::Behavioral;

    // Act & Assert
    assert_ne!(data, behavioral);
}

#[test]
fn same_variants_are_equal() {
    // Arrange
    let a = HelperStructKind::Data;
    let b = HelperStructKind::Data;

    // Act & Assert
    assert_eq!(a, b);
}

// The derived Clone impl is the subject here, so the call is deliberate even
// though the type is Copy and the two are the same operation.
#[test]
#[allow(clippy::clone_on_copy)]
fn clone_produces_equal_copy() {
    // Arrange
    let original = HelperStructKind::Behavioral;

    // Act
    let cloned = original.clone();

    // Assert
    assert_eq!(original, cloned);
}

#[test]
fn debug_format_includes_variant_name() {
    // Arrange & Act
    let data = format!("{:?}", HelperStructKind::Data);
    let behavioral = format!("{:?}", HelperStructKind::Behavioral);

    // Assert
    assert_eq!(data, "Data");
    assert_eq!(behavioral, "Behavioral");
}
