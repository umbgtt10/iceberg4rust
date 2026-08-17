// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use clap::Parser;
use iceberg4rust::args::Args;

#[test]
fn parse_args_defaults() {
    // Arrange & Act
    let args = Args::parse_from(["cargo-iceberg4rust"]);

    // Assert
    assert!(args.manifest_path.is_none());
    assert!(args.packages.is_empty());
    assert!((args.threshold - 20.0).abs() < f64::EPSILON);
    assert_eq!(args.top, 20);
    assert!(!args.json);
}

#[test]
fn parse_args_manifest_path() {
    // Arrange & Act
    let args = Args::parse_from(["cargo-iceberg4rust", "--manifest-path", "Cargo.toml"]);

    // Assert
    assert_eq!(args.manifest_path.unwrap().to_string_lossy(), "Cargo.toml");
}

#[test]
fn parse_args_single_package() {
    // Arrange & Act
    let args = Args::parse_from(["cargo-iceberg4rust", "--package", "foo"]);

    // Assert
    assert_eq!(args.packages, vec!["foo"]);
}

#[test]
fn parse_args_multiple_packages() {
    // Arrange & Act
    let args = Args::parse_from([
        "cargo-iceberg4rust",
        "--package",
        "foo",
        "--package",
        "bar",
        "--package",
        "baz",
    ]);

    // Assert
    assert_eq!(args.packages, vec!["foo", "bar", "baz"]);
}

#[test]
fn parse_args_threshold() {
    // Arrange & Act
    let args = Args::parse_from(["cargo-iceberg4rust", "--threshold", "15.5"]);

    // Assert
    assert!((args.threshold - 15.5).abs() < f64::EPSILON);
}

#[test]
fn parse_args_top() {
    // Arrange & Act
    let args = Args::parse_from(["cargo-iceberg4rust", "--top", "10"]);

    // Assert
    assert_eq!(args.top, 10);
}

#[test]
fn parse_args_all_options() {
    // Arrange & Act
    let args = Args::parse_from([
        "cargo-iceberg4rust",
        "--manifest-path",
        "my/Cargo.toml",
        "--package",
        "pkg_a",
        "--package",
        "pkg_b",
        "--threshold",
        "8.0",
        "--top",
        "5",
    ]);

    // Assert
    assert_eq!(
        args.manifest_path.unwrap().to_string_lossy(),
        "my/Cargo.toml"
    );
    assert_eq!(args.packages, vec!["pkg_a", "pkg_b"]);
    assert!((args.threshold - 8.0).abs() < f64::EPSILON);
    assert_eq!(args.top, 5);
}

fn raw(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|part| (*part).to_string()).collect()
}

#[test]
fn without_cargo_subcommand_drops_the_name_cargo_inserts() {
    // Arrange
    let argv = raw(&["cargo-iceberg4rust", "iceberg4rust", "--threshold", "20"]);

    // Act
    let forwarded = Args::without_cargo_subcommand(argv);

    // Assert
    assert_eq!(forwarded, raw(&["cargo-iceberg4rust", "--threshold", "20"]));
}

#[test]
fn without_cargo_subcommand_leaves_a_direct_invocation_untouched() {
    // Arrange
    let argv = raw(&["cargo-iceberg4rust", "--threshold", "20"]);

    // Act
    let forwarded = Args::without_cargo_subcommand(argv.clone());

    // Assert
    assert_eq!(forwarded, argv);
}

// The subcommand name is only dropped at argv[1]. A package that happens to be
// called iceberg4rust must survive as a --package value.
#[test]
fn without_cargo_subcommand_keeps_a_package_that_happens_to_be_named_iceberg4rust() {
    // Arrange
    let argv = raw(&["cargo-iceberg4rust", "--package", "iceberg4rust"]);

    // Act
    let forwarded = Args::without_cargo_subcommand(argv.clone());

    // Assert
    assert_eq!(forwarded, argv);
}

#[test]
fn without_cargo_subcommand_handles_being_given_nothing() {
    // Arrange
    let argv: Vec<String> = Vec::new();

    // Act
    let forwarded = Args::without_cargo_subcommand(argv);

    // Assert
    assert!(forwarded.is_empty());
}
