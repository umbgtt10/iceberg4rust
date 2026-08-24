// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::gates::gate::Gate;

pub struct Stage2<'a> {
    gates: Vec<&'a dyn Gate>,
}

impl<'a> Stage2<'a> {
    pub fn new(gates: Vec<&'a dyn Gate>) -> Self {
        Self { gates }
    }

    // Stops at the first failure rather than collecting every one: stern4rust
    // runs first because its corrections are renames and file moves, so a
    // layout it is about to reject is a layout the later gates would have
    // measured for nothing.
    pub fn run(&self) -> Result<(), String> {
        for gate in &self.gates {
            println!("{}...", gate.label());

            if let Err(reason) = gate.run() {
                return Err(format!("{} ({reason})", gate.label()));
            }
        }

        Ok(())
    }
}
