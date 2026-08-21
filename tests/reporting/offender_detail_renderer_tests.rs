// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::reporting::file_risk_report::FileRiskReport;
use iceberg4rust::reporting::offender_detail_renderer::OffenderDetailRenderer;
use iceberg4rust::reporting::private_function_report::PrivateFunctionReport;

fn function(name: &str, line: usize, complexity: u32, is_hidden: bool) -> PrivateFunctionReport {
    PrivateFunctionReport {
        name: name.to_string(),
        line,
        complexity,
        is_hidden,
    }
}

fn report(file: &str, functions: Vec<PrivateFunctionReport>) -> FileRiskReport {
    FileRiskReport {
        functions,
        data_structs: Vec::new(),
        behavioral_structs: Vec::new(),
        package_name: "node".to_string(),
        relative_file: file.to_string(),
        effective_loc: 366,
        private_function_count: 9,
        private_complexity_sum: 31,
        data_private_struct_count: 0,
        behavioral_private_struct_count: 0,
        risk_score: 20.87,
    }
}

#[test]
fn render_covers_every_offender_it_is_given() {
    // Arrange
    let first = report("src/first.rs", vec![function("a", 1, 1, true)]);
    let second = report("src/second.rs", vec![function("b", 2, 2, true)]);

    // Act
    let rendered = OffenderDetailRenderer::render(&[&first, &second]);

    // Assert
    assert!(rendered.contains("src/first.rs"));
    assert!(rendered.contains("src/second.rs"));
}

#[test]
fn render_lists_every_function_with_its_line_and_complexity() {
    // Arrange
    let offender = report(
        "src/protocol.rs",
        vec![
            function("sync_height", 244, 2, true),
            function("flush_wal_into", 310, 7, true),
        ],
    );

    // Act
    let rendered = OffenderDetailRenderer::render(&[&offender]);

    // Assert
    assert!(rendered.contains("sync_height"));
    assert!(rendered.contains("244"));
    assert!(rendered.contains("flush_wal_into"));
    assert!(rendered.contains("310"));
}

#[test]
fn render_marks_a_function_that_does_not_count_toward_the_private_surface() {
    // Arrange
    let offender = report(
        "src/observer.rs",
        vec![
            function("helper", 10, 3, true),
            function("on_event", 20, 4, false),
        ],
    );

    // Act
    let rendered = OffenderDetailRenderer::render(&[&offender]);

    // Assert
    let trait_line = rendered
        .lines()
        .find(|line| line.contains("on_event"))
        .expect("trait method listed");
    assert!(trait_line.contains("trait"));
}

#[test]
fn render_names_the_file_and_its_risk() {
    // Arrange
    let offender = report(
        "src/protocol.rs",
        vec![function("sync_height", 244, 2, true)],
    );

    // Act
    let rendered = OffenderDetailRenderer::render(&[&offender]);

    // Assert
    assert!(rendered.contains("src/protocol.rs"));
    assert!(rendered.contains("20.87"));
}

#[test]
fn render_names_the_helper_structs_when_present() {
    // Arrange
    let mut offender = report("src/builder.rs", vec![function("build", 10, 2, true)]);
    offender.data_structs = vec!["Payload".to_string()];
    offender.behavioral_structs = vec!["Encoder".to_string()];

    // Act
    let rendered = OffenderDetailRenderer::render(&[&offender]);

    // Assert
    assert!(rendered.contains("Payload"));
    assert!(rendered.contains("Encoder"));
}

#[test]
fn render_omits_the_struct_line_when_there_are_none() {
    // Arrange
    let offender = report("src/plain.rs", vec![function("work", 10, 2, true)]);

    // Act
    let rendered = OffenderDetailRenderer::render(&[&offender]);

    // Assert
    assert!(!rendered.contains("helper structs"));
}

#[test]
fn render_orders_functions_by_descending_complexity() {
    // Arrange
    let offender = report(
        "src/protocol.rs",
        vec![
            function("cheap", 10, 1, true),
            function("expensive", 900, 12, true),
            function("middling", 50, 5, true),
        ],
    );

    // Act
    let rendered = OffenderDetailRenderer::render(&[&offender]);

    // Assert
    let expensive = rendered.find("expensive").expect("expensive listed");
    let middling = rendered.find("middling").expect("middling listed");
    let cheap = rendered.find("cheap").expect("cheap listed");
    assert!(expensive < middling);
    assert!(middling < cheap);
}

#[test]
fn render_with_no_offenders_returns_empty() {
    // Arrange & Act
    let rendered = OffenderDetailRenderer::render(&[]);

    // Assert
    assert!(rendered.is_empty());
}
