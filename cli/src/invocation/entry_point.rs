// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;
use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;
use iceberg4rust::invocation::runner::Runner;
use iceberg4rust::reporting::text_report_renderer::TextReportRenderer;
use iceberg4rust::traits::report_renderer::ReportRenderer;

use crate::invocation::args::Args;
use crate::system::cargo_workspace_reader::CargoWorkspaceReader;
use crate::system::json_report_renderer::JsonReportRenderer;
use crate::system::walkdir_source_tree::WalkdirSourceTree;

const OFFENDERS_FOUND: u8 = 2;

// The composition root: the command line in, the report out on stdout, an exit
// code back. The one place cargo metadata, the real file system, the report's
// format and stdout are chosen; everything it composes is the core's, tested
// without them.
#[derive(Debug, Clone, Copy, Default)]
pub struct EntryPoint;

impl EntryPoint {
    pub fn new() -> Self {
        Self
    }

    pub fn run<I>(&self, argv: I) -> Result<ExitCode>
    where
        I: IntoIterator<Item = String>,
    {
        let args = Args::parse_from(Args::without_cargo_subcommand(argv));
        let workspace_reader =
            CargoWorkspaceReader::new(args.manifest_path().map(Path::to_path_buf));
        let source_tree = WalkdirSourceTree::new();
        let json = JsonReportRenderer::new();
        let text = TextReportRenderer::new();
        let renderer: &dyn ReportRenderer = if args.json() { &json } else { &text };
        let outcome = Runner::new(&workspace_reader, &source_tree, renderer).run(&args.config())?;

        print!("{}", outcome.output);

        // 0 clean, 2 offenders, 1 for an error via the `Err` path. A distinct
        // code for offenders lets a caller that keeps its own allow-list tell
        // "the tool failed" from "the tool found something".
        Ok(if outcome.has_offenders {
            ExitCode::from(OFFENDERS_FOUND)
        } else {
            ExitCode::SUCCESS
        })
    }
}
