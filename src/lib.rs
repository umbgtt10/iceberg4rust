// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub mod analyzer;
pub mod args;
pub mod comment_stripper;
pub mod complexity_scorer;
pub mod config;
pub mod file_metrics;
pub mod file_risk_report;
pub mod helper_struct_kind;
pub mod json_report_renderer;
pub mod manifest_resolver;
pub mod offender_detail_renderer;
pub mod package_context;
pub mod private_function_collector;
pub mod private_function_metrics;
pub mod private_function_report;
pub mod private_helper_classifier;
pub mod report_printer;
pub mod risk_ordering;
pub mod runner;
pub mod source_file_walker;
pub mod source_root_collector;
pub mod struct_usage_collector;
pub mod test_attr_checker;
