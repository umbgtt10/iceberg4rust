# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

The tool's behaviour, command line and scoring are unchanged -- its stdout,
stderr and exit codes are compared byte for byte against 0.3.1 -- with two
exceptions. A `.rs` file whose path is not UTF-8 now fails the run, naming the
file, where 0.3.1 scored it under a lossy name; such a file cannot be a `mod`
(identifiers are UTF-8), only a `#[path]` target. And `log2` comes from `libm`,
since the core has no `std`: for 54 file sizes up to 100,000 effective lines,
the first at 1,374, its last bit differs from the platform's, which can change
the last digits of `risk_score` and `total_risk` in `--json`; no text report
changes. The
repository splits into a technology-free core and a cli crate
(`docs/CORE_CLI_SPLIT.md`), which changes how the library is packaged and
much of its API. One `pub` free function became an associated function, for
stern4rust 0.17.0's new `owned-functions` rule -- a file with a subject holds
no free function. `Args` and `FileRiskReport` no longer expose their fields,
for stern4rust's coming `records-and-objects` rule.

### Changed (packages)
- `cargo-iceberg4rust` is now the command-line crate (`cli/`): the
  `cargo-iceberg4rust` binary and a library, `iceberg4rust_cli`. The analysis
  library `iceberg4rust` is published by a new package, `iceberg4rust-core`
  (`core/`); a dependent replaces `cargo-iceberg4rust` with
  `iceberg4rust-core`, and paths under `iceberg4rust::` stay for what remains
  there. Both crates share the workspace's version.
- `invocation::args::Args` moves to `iceberg4rust_cli::invocation::args::Args`
  and gains `config()`, the `Config` the core reads.
- `invocation::runner::Runner::run(Args) -> Result<ExitCode>` is
  `Runner::new(&dyn WorkspaceReader, &dyn SourceTree, &dyn ReportRenderer)`
  then `run(&self, &Config) -> Result<invocation::run_outcome::RunOutcome, RunFailure>`:
  the report as text and the verdict, `has_offenders`. The argv fix-up, the
  parsing, the choice of cargo, the real file system and the report's format,
  printing, and the exit code are
  `iceberg4rust_cli::invocation::entry_point::EntryPoint`'s.
- The core no longer reads cargo metadata: the new port
  `traits::workspace_reader::WorkspaceReader` describes the workspace in the
  core's own records -- `workspace::{workspace_description::WorkspaceDescription,
  package_description::PackageDescription, target_description::TargetDescription}`
  -- and fails with `workspace::workspace_read_failure::WorkspaceReadFailure`.
  `iceberg4rust_cli::system::cargo_workspace_reader::CargoWorkspaceReader`
  implements it over `cargo metadata --no-deps`; its `translate` maps
  cargo_metadata's `Metadata`. `ManifestResolver::new(Config)` is
  `new(&dyn WorkspaceReader)`, `resolve_packages(&self)` is
  `resolve_packages(&self, &Config)`, `is_production_target` takes a
  `TargetDescription`, `select_packages` a `WorkspaceDescription` (returning
  `PackageDescription`s), and `SourceRootCollector::collect_from_targets` a
  `&[TargetDescription]`.
- `Config` loses `manifest_path`: the manifest path is
  `CargoWorkspaceReader::new`'s and never enters the core, so a manifest path
  that is not UTF-8 works as before.
- `analysis::source_file_walker::SourceFileWalker` is gone: the new port
  `traits::source_tree::SourceTree` (`rust_files`, `read_source`) is
  implemented on disk by `iceberg4rust_cli::system::walkdir_source_tree::WalkdirSourceTree`.
- `invocation::analyzer::Analyzer::analyze_package` moves to
  `invocation::package_analyzer::PackageAnalyzer::new(&dyn SourceTree, Analyzer).analyze_package`.
- Paths in the library are `workspace::source_path::SourcePath`, text with `/`
  as its only separator: `PackageContext`'s `manifest_dir` and `source_roots`,
  and `ManifestResolver::relative_file(&SourcePath, &SourcePath)`. A file the
  tree cannot list or read fails with `workspace::source_read_failure::SourceReadFailure`.
- The library is `#![no_std]` (with `alloc`) and `#![forbid(unsafe_code)]`,
  and depends on `libm`, `proc-macro2` and `syn` only -- no anyhow, serde,
  cargo_metadata, clap or walkdir. Its failures are typed:
  `Analyzer::analyze_source` returns `invocation::parse_failure::ParseFailure`,
  and `ManifestResolver::resolve_packages` and `select_packages`,
  `PackageAnalyzer::analyze_package` and `Runner::run` return
  `invocation::run_failure::RunFailure`, whose variants keep their cause, so
  the chain the binary prints is unchanged.
- `reporting::report_printer::ReportPrinter` splits in two.
  `reporting::report_selection::ReportSelection` keeps `new(threshold, top)`,
  `has_offenders` and `select_visible`, and builds the
  `reporting::risk_report::RiskReport` a renderer is handed: every offender
  ranked, the `top` of them, and the totals. `reporting::text_report_renderer::TextReportRenderer`
  implements the new port `traits::report_renderer::ReportRenderer`; `print`,
  which wrote to stdout, is `render`, which returns the same text.
- `reporting::json_report_renderer::JsonReportRenderer` moves to
  `iceberg4rust_cli::system::json_report_renderer::JsonReportRenderer`, another
  `ReportRenderer`, with serialisable records of its own: `FileRiskReport` and
  `PrivateFunctionReport` no longer derive `Serialize`, and the library no
  longer depends on serde or serde_json. A renderer that fails says
  `failed to render the report` (`reporting::render_failure::RenderFailure`);
  0.3.1's `failed to render report as JSON` could not be reached.

### Added
- Stage 2 gains a first gate, "Technology-free core": the core's manifest may
  name only `libm`, `proc-macro2` and `syn`, in any section; its crate root
  must carry `#![no_std]` and `#![forbid(unsafe_code)]`; and no file under
  `core/src` or `core/tests` may reach `std::{env,fs,io,net,os,process}`, the
  printing macros, the file-system methods of `Path` or `extern crate std`.
  dry4rust makes one run over both published crates' sources.

### Changed
- `analysis::struct_usage_collector::collect_used_private_structs` is
  `StructUsageCollector::collect_used_private_structs`, on a now-public
  `StructUsageCollector`. Three private helpers -- `is_private_item`,
  `impl_target_name` and `first_struct_name` -- move into the impl blocks of
  the types that call them.
- `Args` and `FileRiskReport` keep their fields private behind getters: they
  have behaviour, and stern4rust's coming `records-and-objects` rule asks an
  object to hide its fields. Each field is read through a getter named after
  it. A `FileRiskReport` is built from a file's metrics with
  `FileRiskReport::new(package_name, relative_file, metrics)`; its JSON is
  unchanged.
- `Config`, `PackageContext`, `FileMetrics`, `PrivateFunctionMetrics` and
  `PrivateFunctionReport` are records, as in 0.3.1: their fields stay `pub`,
  with no getters, constructors or `with_` steps, for stern4rust's coming
  `records-and-objects` rule. `FileMetrics` derives `Default`, so a literal
  can name only the fields it sets and take the rest from
  `..Default::default()`.
- All twenty-four stern4rust rules apply to every workspace member.

## [0.3.1] - 2026-10-10

No change to the tool's behaviour, its CLI or its scoring: same output, same
exit codes, and a file scores exactly what it scored on 0.3.0. Checked by
running the 0.3.0 binary and this one side by side over this repository, a
sibling tool's workspace and a fixture of awkward comments and struct uses,
in text and `--json`: every byte and every exit code matched.

A patch rather than a minor release because no `pub` item of the
`iceberg4rust` library changed signature. Every source change below is to a
private function.

### Changed

- The workspace satisfies all twenty-two `stern4rust` 0.14.0 rules, with
  nothing skipped, selected, unconfigured or baselined. The new
  `returned-mutations` rule found four `&mut` parameters, all fixed rather than
  skipped: `CommentStripper::skip_block_comment_end` returns the advanced index,
  and the three `OffenderDetailRenderer` helpers return the text they build,
  appended in the order they used to write it.
- Duplicated code is shared rather than copied. `dry4rust` found two exact
  groups in `core/src` at 25 AST nodes: two `Visit` methods that each recorded
  a path's last segment, now one `record_if_private_struct` taking the path;
  and three two-character comment-marker tests in `CommentStripper`, now one
  `pair_at` with named pairs. None remain.
- Stage 2 gains a fifth gate, `cargo dry4rust`, second after `stern4rust`: no
  duplicated code in `core/src` beyond `dry4rust-baseline.json`, which records
  nothing, counting units of 25 AST nodes or more. CI installs
  `cargo-dry4rust` alongside the other stage-2 tools.

## [0.3.0] - 2026-08-24

No change to the tool's behaviour, its API or its scoring: a file scores exactly
what it scored on 0.2.0, and no threshold needs revisiting. Everything below is
how the repository builds and gates itself.

### Changed

- The repository is a workspace of two members: `core/` holds the published
  crate, `xtask/` holds the gate orchestration. The root manifest is virtual.

  A single root package made the package directory the whole repository, so
  `xtask/tests/` was judged as source belonging to `core` and every test in it
  broke `test-free-source`. Siblings under a virtual root each own their
  directory, which is what lets `xtask` carry its own `tests/`.

  `cargo publish` now runs from `core/`. The published crate is unchanged:
  same name, version, metadata and contents.

- The gates are a `justfile` plus `cargo xtask` rather than two PowerShell
  scripts, so one entry point serves Linux, Windows and macOS. `cargo xtask
  stage2` reads crap4rust's `--output-format json` instead of parsing its
  human-readable table, and names every offending function when it fails.

- Both members are gated, so the crate that enforces the bar is now held to
  it: 110 functions measured where 77 were before.

### Added

- `.github/workflows/ci.yml` runs both stages on Ubuntu, Windows and macOS for
  every push and pull request.

## [0.2.0] - 2026-08-17

Scores rise. A gate that passes on 0.1.1 may fail on 0.2.0 without the code
having changed, so treat this as a threshold review rather than a drop-in
upgrade. No API changed; the scoring did.

### Fixed

- Restricted visibility now counts toward `P` and `sum(C_i)`. `is_private_item`
  matched only `Visibility::Inherited`, so `pub(crate)`, `pub(super)`,
  `pub(in path)` and `pub(self)` were treated exactly like `pub` and contributed
  nothing at all.

  Nothing outside a crate can reach a restricted function, including an
  integration test, which is a separate crate. Prefixing every private function
  in a file with `pub(crate)` therefore took its score to zero — no relocation,
  no tests, no design change. That is the escape hatch `FORMULA.md` warns about
  under "What is not remediation", except the tool could not see it.

  Found on a consensus codebase: a validator extracted into its own file with
  eleven `pub(super)` methods scored zero and did not appear in the report at
  all, while the file it came from fell 26.89 to 9.00. Nineteen complexity left
  the metric without becoming reachable by anything.

  `ADR-HiddenMeansUnreachable` already stated the governing test — can something
  outside get at it? — and excludes trait-impl methods on exactly that basis.
  Reading only `Visibility::Inherited` got trait impls right by accident and
  restricted visibility wrong by the same accident.

### Changed

- The effect is monotonic: no file scores lower than it did on 0.1.1. A codebase
  with no restricted visibility is unaffected, so the four `*4rust` tools and
  iceberg4rust itself score identically.

### Notes

- A `pub` item inside a private inline `mod` is still treated as reachable. Same
  defect class, deliberately not bundled so that this release's repricing can be
  attributed to one cause. Recorded in `docs/OPEN_POINTS.md`.

## [0.1.1] - 2026-08-17

### Fixed

- The console table and the per-offender detail now rank files across every
  scanned package instead of within each package. `Runner` appends one package's
  reports after another, and `ReportPrinter::select_visible` filtered and applied
  `--top` without ranking first, so the table showed the first N offenders in
  `--package` order rather than the N riskiest.

  On a multi-package workspace this could hide the highest-risk file in the scan
  entirely. Found against a five-package tree where `--top 18`, with the largest
  package listed first, returned eighteen rows from that one package and omitted
  the riskiest file in the repository — which sat in a package named later on the
  command line. Reordering the `--package` flags changed which files appeared.

  `--json` was never affected: it states its own ordering and applies no `--top`.
  Neither was the exit code, which is computed from the threshold across all
  reports and never from the table selection.

### Changed

- `--top` now cuts the lowest-scoring rows rather than whichever package came
  last. Output for single-package scans is unchanged.

### Added

- `RiskOrdering` (`risk_ordering.rs`), the single comparator the table, the JSON
  surface and the per-package sort now share. The rule had been written out three
  times; the copy the table needed was the one that did not exist.

## [0.1.0] - 2026-08-17

First standalone release. The tool previously lived inside a private workspace as
`etheram-file-risk`; this extracts it as a general-purpose cargo subcommand with
no ties to the repository it grew up in.

### Added

- `cargo iceberg4rust` scores every production source file in the selected
  packages for hidden private implementation, and reports the files at or above
  a threshold.
- `--manifest-path` and repeatable `--package` selection, resolved through cargo
  workspace metadata. Only production targets contribute source roots; `test`,
  `bench`, `example` and `custom-build` targets are excluded.
- `--threshold` (default `20`), and `--top` to limit table rows. `--top` is
  display only and never affects the verdict.
- `--json`, emitting the full report with `functions[]`, `data_structs[]` and
  `behavioral_structs[]` per file, ordered by descending risk with ties broken on
  file name so runs are reproducible.
- Per-offender detail in the console report: every private function by name, line
  and individual complexity, ordered by complexity, with a `(trait)` marker on
  methods that do not count toward the private-function tally.
- Exit codes as a gate contract: `0` clean, `2` offenders found, `1` tool error.
  A distinct code for offenders lets a caller with its own allow-list tell
  "found something" from "broke".
- `docs/FORMULA.md` documents what a breach means and how to clear it, including
  the two changes that lower the score without moving implementation — widening
  visibility and rehousing logic in a trait impl — which are named as
  non-remedies rather than left as discoverable loopholes.

### Changed

- Relicensed from Apache-2.0 to MIT, matching the other published tools in this
  family.
- Trait-implementation methods no longer count toward `P`. A trait-impl method is
  reachable by anyone holding the trait, and its inherited visibility reflects the
  trait governing it rather than privacy. Counting them made a wide trait
  implemented with empty bodies the highest-risk shape in a tree, which inverts
  what the metric is for. Their complexity still counts toward `sum(C_i)`, and the
  private structs they use still count toward `D` and `B`.
- The zero baseline is derived rather than enforced. The score is no longer
  short-circuited on `P = 0`, because a file whose only behaviour lives in a trait
  implementation has `P = 0` with positive complexity, and that has to survive
  into the score.
