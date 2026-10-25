# A technology-free core and a cli crate

Status: **in progress** -- one commit per slice, in the order below.

## Why

Mutation testing pays for every mutant with a build and a test run. A mutant in
code that runs `cargo metadata`, walks a disk or prints is slow to kill, and a
test that needs a workspace to run in is slow to run. So the scoring, the
selection and the text report move into a crate that touches no technology at
all, and mutation testing is scoped to that crate alone.

## The shape

The workspace grows a third member:

| member | package | library | what it holds |
|---|---|---|---|
| `core/` | `iceberg4rust-core` | `iceberg4rust` | `#![no_std]`, `#![forbid(unsafe_code)]`; the analysis, the formula, package selection, report selection and the text report. Depends on `libm`, `proc-macro2` and `syn` only |
| `cli/` | `cargo-iceberg4rust` | `iceberg4rust_cli` | the binary, `Args` (clap), the composition root `EntryPoint`, and every adapter: cargo metadata, the file system, the JSON report |
| `xtask/` | `xtask` | `xtask` | the gates |

`cli` depends on `core`, never the reverse. Both published crates share one
workspace version.

### Ports

Core declares a trait for everything technological it needs, implemented in
`cli/src/system/` and, for core's own tests, by in-memory fakes under
`core/tests/support/`:

| port (`core/src/traits/`) | what it answers | cli adapter |
|---|---|---|
| `SourceTree` | the `.rs` files under a root, and a file's text | `WalkdirSourceTree` |
| `WorkspaceReader` | the workspace's packages and targets | `CargoWorkspaceReader` (`cargo metadata --no-deps`) |
| `ReportRenderer` | a report as text | `TextReportRenderer` lives in core; `JsonReportRenderer` (serde) in cli |

Core works on its own records (`workspace/`: `WorkspaceDescription`,
`PackageDescription`, `TargetDescription`), on `SourcePath` -- a path held as
text with `/` as its only separator, which cli converts to at the edge -- and
fails with typed errors (`SourceReadFailure`, `WorkspaceReadFailure`,
`RenderFailure`, `ParseFailure`, `RunFailure`). It returns its output as a
string; printing it and turning the verdict into an exit code is `EntryPoint`'s.

The binary's behaviour does not change: stdout, stderr and exit codes are
compared byte for byte against 0.3.1 after every slice that touches them.
`log2` comes from `libm`, since `core` cannot reach the platform's; the two
disagree in the last bit for a few file sizes (the first at 1,374 effective
lines), visible only in `--json`'s `risk_score` and `total_risk`.

### The gate

A stage-2 gate, first in the run, keeps core technology-free: its manifest may
name only its allowed crates in any section, its crate root must carry
`#![no_std]` and `#![forbid(unsafe_code)]`, and no file under `core/src` or
`core/tests` may reach `std::{env,fs,io,net,os,process}`, the printing macros,
the file-system methods or `extern crate std`.

### Mutation testing

`.cargo/mutants.toml` scopes cargo-mutants to `core/src`; each mutant runs
core's tests only.

## Slices

| # | slice | status |
|---|---|---|
| S0 | this record | **landed** (`6bf2acd`) |
| S1 | `DryGate`: one run over the workspace, with the trees it must not judge excluded by full path | **landed** (`49a92c8`) |
| S2 | the split: `cli` takes `main.rs`, `Args` and a new `EntryPoint`; core is renamed; one workspace version; `[workspace.lints]` forbids unsafe code | **landed** (`8e45b08`). `Config` gains `json` until S5 takes the format into the renderer |
| S3 | the `SourceTree` port, `SourcePath`, `PackageAnalyzer`; the walker becomes cli's `WalkdirSourceTree` | **landed** (`13d4ee8`). Until S4 the resolver itself turns cargo's paths into `SourcePath`s |
| S4 | the `WorkspaceReader` port and core's workspace records; `CargoWorkspaceReader` in cli | **landed** (`0fbeaf1`). The manifest path stays in cli: `Config` loses it |
| S5 | the `ReportRenderer` port: report selection in core, the text report in core, the JSON report in cli; serde leaves core | **landed** (`e18ff8f`). `Config` loses `json`: the renderer is the format |
| S6 | `#![no_std]`: alloc imports, `libm`, typed errors; anyhow leaves core | **landed** (`bd9397d`) |
| S7 | the technology-free gate, first in stage 2 | **landed** (`1da1b5d`). A crate declared in two sections is named once |
| S8 | `.cargo/mutants.toml` | **landed**. `cargo mutants --list`: 325 mutants, all in `iceberg4rust-core` |
| S9 | docs, and this record moves to `docs/history/` | |

## Tests that move

No test is deleted. Tests whose subject moves go with it:

| from | to |
|---|---|
| `core/tests/invocation/args_tests.rs` (11) | `cli/tests/invocation/args_tests.rs` |
| `core/tests/invocation/runner_tests.rs` (2, argv-driven) | `cli/tests/invocation/entry_point_tests.rs`, same names |
| `core/tests/analysis/source_file_walker_tests.rs` (6, `walk_*`) | `cli/tests/system/walkdir_source_tree_tests.rs`, renamed `rust_files_*` |
| `analyzer_tests::analyze_package_*` (2) | `core/tests/invocation/package_analyzer_tests.rs`, same names, in-memory |
| `manifest_resolver_tests::resolve_packages_against_this_package_returns_it_with_its_source_root` | `cli/tests/system/cargo_workspace_reader_tests.rs`, same name |
| `core/tests/reporting/json_report_renderer_tests.rs` (7) | `cli/tests/system/json_report_renderer_tests.rs`, same names |
| `report_printer_tests`: `has_offenders_*` (5), `select_visible_*` (6) | `core/tests/reporting/report_selection_tests.rs`, same names |

Two tests are rewritten because the method they called no longer exists: the
printer's `print_report_no_visible_files_does_not_panic` and
`print_report_with_reports_does_not_panic` become
`render_with_no_visible_files_says_none_was_found` and
`render_with_a_report_lists_it_in_the_table_the_detail_and_the_summary`, which
assert the exact text where the old ones asserted only that nothing panicked.
`config_clone_produces_independent_copy` loses its `manifest_path` line with the
field, which moves into `CargoWorkspaceReader`.
