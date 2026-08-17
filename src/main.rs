// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::process::ExitCode;

use anyhow::Result;
use iceberg4rust::args::Args;
use iceberg4rust::runner::Runner;

fn main() -> Result<ExitCode> {
    let args = Args::parse_args();
    Runner::run(args)
}
