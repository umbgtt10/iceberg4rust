// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use iceberg4rust::invocation::config::Config;

#[derive(Debug, Parser)]
#[command(name = "cargo-iceberg4rust")]
#[command(bin_name = "cargo iceberg4rust")]
#[command(version)]
#[command(about = "Measure how much private implementation a Rust source file hides")]
pub struct Args {
    #[arg(long)]
    manifest_path: Option<PathBuf>,

    #[arg(long = "package")]
    packages: Vec<String>,

    #[arg(long, default_value_t = 20.0)]
    threshold: f64,

    #[arg(long, default_value_t = 20)]
    top: usize,

    #[arg(long)]
    json: bool,
}

impl Args {
    pub fn config(&self) -> Config {
        Config {
            manifest_path: self.manifest_path.clone(),
            packages: self.packages.clone(),
            threshold: self.threshold,
            top: self.top,
            json: self.json,
        }
    }

    pub fn json(&self) -> bool {
        self.json
    }

    pub fn manifest_path(&self) -> Option<&Path> {
        self.manifest_path.as_deref()
    }

    pub fn packages(&self) -> &[String] {
        &self.packages
    }

    pub fn threshold(&self) -> f64 {
        self.threshold
    }

    pub fn top(&self) -> usize {
        self.top
    }

    // Cargo invokes `cargo iceberg4rust ...` as `cargo-iceberg4rust iceberg4rust
    // ...`, inserting the subcommand name at argv[1]. Dropping it lets the same
    // binary be run directly and through cargo with identical arguments.
    pub fn without_cargo_subcommand<I>(args: I) -> Vec<String>
    where
        I: IntoIterator<Item = String>,
    {
        let args: Vec<String> = args.into_iter().collect();
        if args.get(1).map(String::as_str) != Some("iceberg4rust") {
            return args;
        }
        let mut forwarded = Vec::with_capacity(args.len() - 1);
        forwarded.extend(args.iter().take(1).cloned());
        forwarded.extend(args.into_iter().skip(2));
        forwarded
    }
}
