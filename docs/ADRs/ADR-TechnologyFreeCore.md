# ADR-TechnologyFreeCore

- **Status:** Accepted
- **Date:** 2026-10-25

## Context

The tool is meant to be mutation-tested, and mutation testing pays for every
mutant with a build and a test run. While one crate held everything, a mutant
anywhere rebuilt a crate that linked cargo_metadata, clap, walkdir and serde,
and its tests ran `cargo metadata` and wrote temporary directories. The scoring,
which is what deserves mutating, was mixed with code whose only job is to talk
to the machine.

## Decision

The analysis lives in a technology-free crate, `iceberg4rust-core`: `#![no_std]`
and `#![forbid(unsafe_code)]`, depending on `libm`, `proc-macro2` and `syn`
only, declaring a trait for every piece of technology it needs, and returning
its report as a string; a separate crate, `cargo-iceberg4rust`, implements those
traits with the real technology, parses the arguments, prints, and decides the
exit code.

## Forcing constraints / Evidence

Measured on the same machine, one expression changed in `CommentStripper::strip`
as cargo-mutants changes it: building the crate went from 0.39 s to 0.18 s,
building and running its tests from 0.72 s to 0.51 s, and a clean build of its
tests -- what each cargo-mutants worker pays first -- from 13.4 s to 5.3 s. The
core's 227 tests run in about a hundredth of a second and touch no disk.
`cargo mutants --list` names 325 mutants, all in the core.

The binary's behaviour did not move: stdout, stderr and exit codes were compared
byte for byte against 0.3.1 on twelve invocations after every step.

`f64::log2` is not in `core`, and calling the platform's would need `unsafe`, so
the size factor uses `libm`. It disagrees with glibc in the last bit for 54 file
sizes up to 100,000 effective lines (the first at 1,374), which can move the
last digits of `risk_score` in `--json`; no text report changes.

## Rejected alternatives

- **One crate, with cargo-mutants' `exclude_globs` on the technology.** Scopes
  the mutants but not the build or the tests, and nothing would stop the next
  change reaching for `std::fs` inside the analysis.
- **A port for the logarithm.** Bit-exact on every platform, but a trait for
  arithmetic the core owns; the last-bit difference was measured and accepted.
- **An output port for stdout.** The core returns a string; printing it is the
  composition root's one line.
- **`clippy.toml`'s `disallowed-*` lists.** Type-resolved, but blind to
  manifests, and with `#![no_std]` the compiler already guards the source.

## Consequences

Two published crates, `iceberg4rust-core` first, sharing one version. A new
technology need is a trait in `core/src/traits/`, an in-memory fake in
`core/tests/support/` and an adapter in `cli/src/system/`, composed in
`EntryPoint`. Paths in the core are `SourcePath`s with `/` as their only
separator, converted by `cli` at the edge; a `.rs` file whose path is not UTF-8
fails the run instead of being scored under a lossy name. The core's errors are
typed and keep their causes.

## Enforcement

The "Technology-free core" gate, first in stage 2: core's manifest names only
`libm`, `proc-macro2` and `syn` in any section; `core/src/lib.rs` carries
`#![no_std]` and `#![forbid(unsafe_code)]`; no file under `core/src` or
`core/tests` reaches `std::{env,fs,io,net,os,process}`, a printing macro, a
file-system method of `Path` or `extern crate std`. `.cargo/mutants.toml` scopes
cargo-mutants to `core/src`.

## Related

- [ADR-CargoSubcommandPackaging](ADR-CargoSubcommandPackaging.md)
- [`../ARCHITECTURE.md`](../ARCHITECTURE.md)
- [`../history/[IMPLEMENTED] CORE_CLI_SPLIT.md`](../history/%5BIMPLEMENTED%5D%20CORE_CLI_SPLIT.md)
