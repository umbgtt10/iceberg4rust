// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use xtask::process::command_outcome::CommandOutcome;

#[test]
fn new_holds_the_values_it_was_given() {
    // Arrange & Act
    let outcome = CommandOutcome::new(Some(2), String::from("out"), String::from("err"));

    // Assert
    assert_eq!(outcome.exit_code, Some(2));
    assert_eq!(outcome.stdout, "out");
    assert_eq!(outcome.stderr, "err");
}

#[test]
fn new_with_no_exit_code_holds_none() {
    // Arrange & Act
    let outcome = CommandOutcome::new(None, String::new(), String::new());

    // Assert
    assert_eq!(outcome.exit_code, None);
}
