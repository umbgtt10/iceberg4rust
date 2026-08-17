# ADR-CargoSubcommandPackaging

- **Status:** Accepted
- **Date:** 2026-08-16

## Context

The tool takes `--manifest-path`, resolves packages through `cargo metadata`,
and is meant to sit in a build gate beside `cargo fmt`, `cargo clippy` and
`cargo crap4rust`. It could ship either as a plain binary invoked by name or as
a cargo subcommand.

The repository is named `iceberg4rust`, which is not necessarily the crate name.

## Decision

The crate publishes as `cargo-iceberg4rust` with library name `iceberg4rust`, so
cargo resolves `cargo iceberg4rust`, and `Args::without_cargo_subcommand` strips
the subcommand name cargo re-inserts at `argv[1]`.

## Forcing constraints / Evidence

Cargo discovers subcommands by looking for a `cargo-<name>` binary on `PATH`.
The package name is therefore fixed by the invocation we want, not chosen
freely.

The sibling tools already resolved this the same way: repo `crap4rust` publishes
`cargo-crap4rust`, repo `dry4rust` publishes `cargo-dry4rust`. Matching them means
the gate scripts invoke all of them identically.

The library name departs from those siblings deliberately. `crap4rust` and
`grip4rust` shorten theirs to `crap` and `grip`; the equivalent here would be
`iceberg`, which is taken on crates.io by the Apache Iceberg Rust implementation
with over 1.8 million downloads. The library is therefore `iceberg4rust` in full.
The `4rust` suffix exists to disambiguate, and this is the case that needed it —
`cargo-iceberg4rust` and `iceberg4rust` were both free, `iceberg` was not.

Cargo passes the subcommand name through as the first argument, so
`cargo iceberg4rust --package foo` reaches the binary as
`cargo-iceberg4rust iceberg4rust --package foo`. clap rejects that extra argument.
Running the binary directly does not repeat the name, so the strip must be
conditional rather than unconditional — stripping always would eat the user's
first real argument.

## Rejected alternatives

- **Publish as plain `iceberg4rust`.** Simpler, no shim, and how `slotgate` ships.
  Rejected because a tool whose entire input is a Cargo manifest belongs behind
  `cargo`, and because it would leave gate scripts invoking one of three
  sibling tools differently from the other two.
- **Strip `argv[1]` unconditionally.** Breaks every direct invocation.
- **Match the name anywhere in argv.** Silently drops a package legitimately
  named `iceberg4rust`.

## Consequences

The binary is `cargo-iceberg4rust` and the crate is `cargo-iceberg4rust`, so
`cargo install cargo-iceberg4rust` is what users type while `cargo iceberg4rust` is
what they run. That mismatch is conventional for cargo subcommands but does
need saying once in the README.

`--version` and help text carry `bin_name = "cargo iceberg4rust"` so usage strings
show the invocation rather than the binary.

## Enforcement

`tests/args_tests.rs` covers all four cases: the name present at `argv[1]` is
stripped, a direct invocation is untouched, a package *named* `iceberg4rust` at
`argv[2]` survives, and an empty argv does not panic.

## Related

- `docs/ARCHITECTURE.md`
