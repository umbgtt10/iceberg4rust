// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_source_tree::InMemorySourceTree;
use crate::support::in_memory_workspace_reader::InMemoryWorkspaceReader;
use crate::support::in_memory_workspace_reader::library_package;
use crate::support::recording_report_renderer::RecordingReportRenderer;
use iceberg4rust::invocation::config::Config;
use iceberg4rust::invocation::runner::Runner;
use iceberg4rust::reporting::text_report_renderer::TextReportRenderer;
use iceberg4rust::workspace::workspace_description::WorkspaceDescription;

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

fn config(threshold: f64, top: usize) -> Config {
    Config {
        packages: vec![String::from("alpha"), String::from("beta")],
        threshold,
        top,
    }
}

fn two_package_workspace() -> InMemoryWorkspaceReader {
    InMemoryWorkspaceReader::new(WorkspaceDescription {
        packages: vec![
            library_package("alpha", "/w/alpha"),
            library_package("beta", "/w/beta"),
        ],
        root_package: None,
    })
}

#[test]
fn run_above_every_score_reports_no_offenders() {
    // Arrange
    let reader = two_package_workspace();
    let tree = InMemorySourceTree::new().with_file("/w/alpha/src/lib.rs", HIDES_PRIVATE_WORK);

    // Act
    let outcome = Runner::new(&reader, &tree, &TextReportRenderer::new())
        .run(&config(50.0, 20))
        .expect("run");

    // Assert
    assert!(!outcome.has_offenders);
    assert_eq!(
        outcome.output,
        "cargo-iceberg4rust report\n\nNo files with private implementation risk at or above 50.00 were found.\n"
    );
}

#[test]
fn run_below_every_score_reports_offenders_from_every_package() {
    // Arrange
    let reader = two_package_workspace();
    let tree = InMemorySourceTree::new()
        .with_file("/w/alpha/src/lib.rs", HIDES_PRIVATE_WORK)
        .with_file("/w/beta/src/lib.rs", HIDES_PRIVATE_WORK);

    // Act
    let outcome = Runner::new(&reader, &tree, &TextReportRenderer::new())
        .run(&config(0.5, 20))
        .expect("run");

    // Assert
    assert!(outcome.has_offenders);
    assert!(outcome.output.starts_with("cargo-iceberg4rust report\n\n"));
    assert!(outcome.output.contains("\nalpha    src/lib.rs "));
    assert!(outcome.output.contains("\nbeta     src/lib.rs "));
}

#[test]
fn run_hands_the_renderer_every_offender_and_the_top_of_them() {
    // Arrange
    let reader = two_package_workspace();
    let tree = InMemorySourceTree::new()
        .with_file("/w/alpha/src/lib.rs", HIDES_PRIVATE_WORK)
        .with_file("/w/beta/src/lib.rs", HIDES_PRIVATE_WORK);
    let renderer = RecordingReportRenderer::new();

    // Act
    let outcome = Runner::new(&reader, &tree, &renderer)
        .run(&config(0.5, 1))
        .expect("run");

    // Assert
    assert_eq!(outcome.output, "rendered");
    assert_eq!(
        renderer.handed(),
        vec![(
            vec![
                String::from("alpha src/lib.rs"),
                String::from("beta src/lib.rs")
            ],
            vec![String::from("alpha src/lib.rs")]
        )]
    );
}

#[test]
fn run_with_a_renderer_that_fails_returns_its_failure() {
    // Arrange
    let reader = two_package_workspace();
    let tree = InMemorySourceTree::new();
    let renderer = RecordingReportRenderer::failing();

    // Act
    let result = Runner::new(&reader, &tree, &renderer).run(&config(1.0, 20));

    // Assert
    assert_eq!(
        result.expect_err("failing renderer").to_string(),
        "failed to render the report"
    );
}

#[test]
fn run_with_a_workspace_that_cannot_be_read_returns_the_error() {
    // Arrange
    let reader = InMemoryWorkspaceReader::failing();
    let tree = InMemorySourceTree::new();

    // Act
    let result = Runner::new(&reader, &tree, &TextReportRenderer::new()).run(&config(1.0, 20));

    // Assert
    assert_eq!(
        result.expect_err("unreadable").to_string(),
        "failed to read Cargo metadata"
    );
}
