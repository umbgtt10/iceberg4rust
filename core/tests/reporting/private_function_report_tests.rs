// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// Mirrors src/reporting/private_function_report.rs.

use iceberg4rust::reporting::private_function_report::PrivateFunctionReport;

#[test]
fn new_keeps_every_field_it_was_given() {
    // Arrange & Act
    let report = PrivateFunctionReport::new(String::from("decode"), 42, 7, true);

    // Assert
    assert_eq!(report.name(), "decode");
    assert_eq!(report.line(), 42);
    assert_eq!(report.complexity(), 7);
    assert!(report.is_hidden());
}
