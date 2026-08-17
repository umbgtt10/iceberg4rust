// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use iceberg4rust::analyzer::Analyzer;

#[test]
fn analyze_source_single_struct_zero_private_functions_returns_zero_risk() {
    // Arrange
    let source = r#"
pub struct FileType;

impl FileType {
    pub fn run(&self) {}
}
"#;

    // Act
    let metrics = Analyzer::new().analyze_source(source).expect("metrics");

    // Assert
    assert_eq!(metrics.private_function_count, 0);
    assert_eq!(metrics.data_private_struct_count, 0);
    assert_eq!(metrics.behavioral_private_struct_count, 0);
    assert!(metrics.risk_score.abs() < f64::EPSILON);
}

#[test]
fn analyze_source_private_data_and_behavioral_structs_are_counted_separately() {
    // Arrange
    let source = r#"
pub struct Worker;

struct Helper;
struct State;

impl Helper {
    fn build() -> Self {
        Helper
    }
}

impl Worker {
    fn compute(&self, helper: Helper) -> State {
        if true {
            State
        } else {
            let shadow: Helper = helper;
            let _other = Helper::build();
            let _ = shadow;
            State
        }
    }
}
"#;

    // Act
    let metrics = Analyzer::new().analyze_source(source).expect("metrics");

    // Assert
    assert_eq!(metrics.private_function_count, 2);
    assert!(metrics.private_complexity_sum > 0);
    assert_eq!(metrics.data_private_struct_count, 1);
    assert_eq!(metrics.behavioral_private_struct_count, 1);
    assert!(metrics.risk_score > 0.0);
}

#[test]
fn compute_file_risk_larger_file_same_private_logic_returns_higher_score() {
    // Arrange
    let smaller = Analyzer::compute_file_risk(20, 2, 10, 1, 1);
    let larger = Analyzer::compute_file_risk(200, 2, 10, 1, 1);

    // Act
    let comparison = larger > smaller;

    // Assert
    assert!(comparison);
}

#[test]
fn compute_file_risk_behavioral_structs_weigh_more_than_data_structs() {
    // Arrange
    let data_heavy = Analyzer::compute_file_risk(100, 2, 10, 2, 0);
    let behavioral_heavy = Analyzer::compute_file_risk(100, 2, 10, 0, 2);

    // Act
    let comparison = behavioral_heavy > data_heavy;

    // Assert
    assert!(comparison);
}

#[test]
fn analyze_source_private_data_struct_used_by_private_function_is_counted() {
    // Arrange
    let source = r#"
pub struct Manager;

struct Config;

impl Manager {
    fn configure(&self) -> Config {
        Config
    }
}
"#;

    // Act
    let metrics = Analyzer::new().analyze_source(source).expect("metrics");

    // Assert
    assert_eq!(metrics.private_function_count, 1);
    assert_eq!(metrics.data_private_struct_count, 1);
    assert_eq!(metrics.behavioral_private_struct_count, 0);
}

#[test]
fn analyze_source_empty_source_returns_zero_metrics() {
    // Arrange
    let source = "";

    // Act
    let metrics = Analyzer::new().analyze_source(source).expect("metrics");

    // Assert
    assert_eq!(metrics.private_function_count, 0);
    assert_eq!(metrics.private_complexity_sum, 0);
    assert_eq!(metrics.data_private_struct_count, 0);
    assert_eq!(metrics.behavioral_private_struct_count, 0);
    assert!(metrics.risk_score.abs() < f64::EPSILON);
}

#[test]
fn compute_file_risk_zero_private_functions_returns_zero() {
    // Arrange & Act
    let risk = Analyzer::compute_file_risk(100, 0, 0, 0, 0);

    // Assert
    assert!(risk.abs() < f64::EPSILON);
}

#[test]
fn compute_file_risk_increasing_complexity_increases_score() {
    // Arrange
    let low_complexity = Analyzer::compute_file_risk(100, 2, 2, 0, 0);
    let high_complexity = Analyzer::compute_file_risk(100, 2, 20, 0, 0);

    // Act
    let comparison = high_complexity > low_complexity;

    // Assert
    assert!(comparison);
}

#[test]
fn analyze_source_cfg_test_module_is_ignored() {
    // Arrange
    let source = r#"
pub struct Worker;

struct Helper;

impl Helper {
    fn build() -> Helper {
        Helper
    }
}

#[cfg(test)]
mod tests {
    fn internal_helper() {}
}
"#;

    // Act
    let metrics = Analyzer::new().analyze_source(source).expect("metrics");

    // Assert
    assert_eq!(metrics.private_function_count, 1);
    assert_eq!(metrics.data_private_struct_count, 0);
    assert_eq!(metrics.behavioral_private_struct_count, 1);
}

// A wide trait implemented with empty bodies. This shape scored 49.19 when
// every trait method counted toward P.
#[test]
fn analyze_source_wide_no_op_trait_impl_scores_zero_risk() {
    // Arrange
    let mut source = String::from("pub struct NoOpObserver;\n\nimpl Observer for NoOpObserver {\n");
    for index in 0..63 {
        source.push_str(&format!("    fn on_event_{index}(&self) {{}}\n"));
    }
    source.push_str("}\n");

    // Act
    let metrics = Analyzer::new().analyze_source(&source).expect("metrics");

    // Assert
    assert_eq!(metrics.private_function_count, 0);
    assert_eq!(metrics.private_complexity_sum, 0);
    assert!(metrics.risk_score.abs() < f64::EPSILON);
}

// The counterpart: breadth is free, but logic inside a trait impl is not.
#[test]
fn analyze_source_trait_impl_carrying_logic_still_scores_risk() {
    // Arrange
    let source = r#"
pub struct Router;

impl Dispatching for Router {
    fn dispatch(&self, kind: u8, flag: bool) {
        if flag {
            for _ in 0..kind {
                send();
            }
        } else {
            drop_it();
        }
    }
}
"#;

    // Act
    let metrics = Analyzer::new().analyze_source(source).expect("metrics");

    // Assert
    assert_eq!(metrics.private_function_count, 0);
    assert!(metrics.private_complexity_sum > 0);
    assert!(metrics.risk_score > 0.0);
}

#[test]
fn analyze_source_inherent_private_methods_are_still_counted() {
    // Arrange
    let source = r#"
pub struct Worker;

impl Worker {
    fn first(&self) {}
    fn second(&self) {}
}
"#;

    // Act
    let metrics = Analyzer::new().analyze_source(source).expect("metrics");

    // Assert
    assert_eq!(metrics.private_function_count, 2);
    assert!(metrics.risk_score > 0.0);
}

#[test]
fn analyze_source_reports_each_private_function_by_name_line_and_complexity() {
    // Arrange
    let source = r#"
pub struct Worker;

impl Worker {
    fn simple(&self) {}

    fn branching(&self, flag: bool) {
        if flag {
            act();
        }
    }
}
"#;

    // Act
    let metrics = Analyzer::new().analyze_source(source).expect("metrics");

    // Assert
    assert_eq!(metrics.functions.len(), 2);
    assert_eq!(metrics.functions[0].name, "simple");
    assert_eq!(metrics.functions[0].line, 5);
    assert_eq!(metrics.functions[0].complexity, 0);
    assert_eq!(metrics.functions[1].name, "branching");
    assert!(metrics.functions[1].complexity > 0);
}

#[test]
fn analyze_source_names_the_helper_structs_it_counted() {
    // Arrange
    let source = r#"
pub struct Primary;

struct DataHelper {
    value: u32,
}

struct BehaviouralHelper {
    seed: u32,
}

impl BehaviouralHelper {
    fn act(&self) {}
}

impl Primary {
    fn work(&self) {
        let data = DataHelper { value: 1 };
        let behaviour = BehaviouralHelper { seed: 2 };
        behaviour.act();
    }
}
"#;

    // Act
    let metrics = Analyzer::new().analyze_source(source).expect("metrics");

    // Assert
    assert_eq!(metrics.data_structs, vec!["DataHelper".to_string()]);
    assert_eq!(
        metrics.behavioral_structs,
        vec!["BehaviouralHelper".to_string()]
    );
    assert_eq!(
        metrics.data_private_struct_count,
        metrics.data_structs.len()
    );
}

// A type whose whole implementation is pub(crate) is untestable from outside the
// crate, so it carries the same risk as one built from bare `fn` helpers.
#[test]
fn analyze_source_a_type_built_from_pub_crate_helpers_scores_risk() {
    // Arrange
    let source = r#"
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

    // Act
    let metrics = Analyzer::new().analyze_source(source).expect("metrics");

    // Assert
    assert_eq!(metrics.private_function_count, 2);
    assert_eq!(metrics.private_complexity_sum, 2);
    assert!(metrics.risk_score > 0.0);
}

// Restricting a private helper must not change the score: pub(crate) is not an
// escape hatch from the measurement, which is the whole point of counting it.
#[test]
fn analyze_source_scores_pub_crate_helpers_the_same_as_private_ones() {
    // Arrange
    let private_source = r#"
pub struct Worker;

impl Worker {
    pub fn run(&self, v: u32) -> u32 { Self::step(v) }

    fn step(v: u32) -> u32 {
        if v > 1 { 2 } else { 3 }
    }
}
"#;
    let restricted_source = private_source.replace("    fn step", "    pub(crate) fn step");

    // Act
    let private = Analyzer::new()
        .analyze_source(private_source)
        .expect("metrics");
    let restricted = Analyzer::new()
        .analyze_source(&restricted_source)
        .expect("metrics");

    // Assert
    assert_eq!(
        private.private_function_count,
        restricted.private_function_count
    );
    assert!((private.risk_score - restricted.risk_score).abs() < f64::EPSILON);
}
