// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;
use clap::Parser;
use iceberg4rust::invocation::args::Args;
use iceberg4rust::invocation::runner::Runner;
use std::env::args;
use std::process::ExitCode;

// Reading the real process argv is the one thing no test can reach, so it is
// all this binary does. The fixup it feeds is public and tested.
fn main() -> Result<ExitCode> {
    let forwarded = Args::without_cargo_subcommand(args());
    Runner::run(Args::parse_from(forwarded))
}
