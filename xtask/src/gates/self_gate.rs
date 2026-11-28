// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serge4rust::gates::gate::Gate;
use serge4rust::gates::gate_failure::GateFailure;
use serge4rust::process::command_runner::CommandRunner;

const OFFENDERS_FOUND: i32 = 2;

pub struct SelfGate<'a> {
    runner: &'a dyn CommandRunner,
    manifest_path: String,
    binary: String,
    packages: Vec<String>,
    threshold: String,
}

impl<'a> SelfGate<'a> {
    pub fn new(
        runner: &'a dyn CommandRunner,
        manifest_path: String,
        binary: String,
        packages: Vec<String>,
        threshold: String,
    ) -> Self {
        Self {
            runner,
            manifest_path,
            binary,
            packages,
            threshold,
        }
    }
}

impl Gate for SelfGate<'_> {
    fn label(&self) -> String {
        String::from("File risk (self-analysis)")
    }

    fn run(&self) -> Result<(), GateFailure> {
        let mut args = vec![
            String::from("run"),
            String::from("--quiet"),
            String::from("--bin"),
            self.binary.clone(),
            String::from("--"),
            String::from("--manifest-path"),
            self.manifest_path.clone(),
        ];
        // A virtual workspace has no root package, so the packages to analyse
        // have to be named: without them the tool cannot pick a default and
        // exits rather than scanning.
        for package in &self.packages {
            args.push(String::from("--package"));
            args.push(package.clone());
        }
        args.push(String::from("--threshold"));
        args.push(self.threshold.clone());

        // 2 is the tool's own "offenders found"; anything else non-zero, or a
        // signal, is a failure to run at all.
        match self.runner.run_streaming("cargo", &args)? {
            Some(0) => Ok(()),
            Some(OFFENDERS_FOUND) => Err(GateFailure::AboveCeiling {
                ceiling: self.threshold.clone(),
            }),
            code => Err(GateFailure::CouldNotRun { exit_code: code }),
        }
    }
}
