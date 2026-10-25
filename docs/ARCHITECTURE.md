# Architecture

How a `cargo iceberg4rust` invocation flows through the code, crate by crate
and module by module.

## Two crates

`core/` (`iceberg4rust-core`, library `iceberg4rust`) is the analysis, and
touches no technology: it is `#![no_std]`, depends on `libm`, `proc-macro2` and
`syn` only, and reaches cargo, the file system and the report's format through
ports it declares. `cli/` (`cargo-iceberg4rust`) is the command: it parses the
arguments, implements the ports with the real technology and composes the run.
`cli` depends on `core`, never the reverse. A stage-2 gate keeps it that way;
see [ADRs/ADR-TechnologyFreeCore.md](ADRs/ADR-TechnologyFreeCore.md).

## The pipeline

```text
cli/src/main.rs
  └─ EntryPoint::run(argv)                        cli
      ├─ Args (clap), cargo's argv[1] stripped    cli
      ├─ CargoWorkspaceReader   WorkspaceReader   cli: cargo metadata --no-deps
      ├─ WalkdirSourceTree      SourceTree        cli: walkdir + fs
      ├─ JsonReportRenderer     ReportRenderer    cli: serde, for --json
      │  or TextReportRenderer  ReportRenderer    core
      └─ Runner::new(reader, tree, renderer).run(&config)        core
          1. ManifestResolver   the workspace, described -> Vec<PackageContext>
             a. select the requested packages, or the root package
             b. SourceRootCollector derives source roots from production targets
          2. PackageAnalyzer::analyze_package(), per package
             a. SourceTree::rust_files lists .rs files under each source root
             b. per file: SourceTree::read_source, then Analyzer::analyze_source:
                CommentStripper counts effective lines,
                PrivateHelperClassifier splits helper structs into data and
                behavioural, PrivateFunctionCollector gathers private functions
                with their name, line, complexity and struct usage
             c. compute_file_risk() turns those into a score
             d. files scoring zero are dropped before becoming reports
          3. ReportSelection builds the RiskReport: every offender ranked,
             the top of them, the totals; the renderer turns it into text
          4. RunOutcome { output, has_offenders }
      prints the output; exit code 2 on offenders, 0 otherwise, 1 on an error
```

## Analysis layer (core)

`Analyzer` (`invocation/analyzer.rs`) owns the per-file computation and the
formula itself. `compute_file_risk` takes plain numbers and no context, so every
term's direction is testable without parsing anything. Its logarithm is
`libm`'s, since `core` cannot reach the platform's.

`PrivateFunctionCollector` (`private_function_collector.rs`) walks items and
records each private function as `PrivateFunctionMetrics` — name, line,
cognitive complexity, the private structs it touches, and whether it is *hidden*.
Hidden means unreachable from outside: free functions and inherent impl methods,
never trait-impl methods. See
[ADRs/ADR-HiddenMeansUnreachable.md](ADRs/ADR-HiddenMeansUnreachable.md).

`ComplexityScorer` (`complexity_scorer.rs`) scores a single block.
`PrivateHelperClassifier` (`private_helper_classifier.rs`) decides which helper
structs are data-only and which carry behaviour — the difference is a factor of
four in the formula. `StructUsageCollector` (`struct_usage_collector.rs`) finds
which of them a given function actually uses.

`CommentStripper` (`comment_stripper.rs`) computes effective lines: non-blank,
non-comment, production only.

Analysis runs on a parsed `syn::File` with no type resolution and no build step,
so the tool works on any syntactically valid source whether or not the crate
compiles or its dependencies are vendored.

`PackageAnalyzer` (`invocation/package_analyzer.rs`) walks one package through
the `SourceTree` it was given; a file outside the production tree
(`tests/`, `examples/`, `benches/`, `build.rs`) or scoring zero yields no report.

## Ports and their data (core)

| port, `traits/` | answers | data | failure |
|---|---|---|---|
| `SourceTree` | the `.rs` files under a root, and a file's text | `SourcePath` | `SourceReadFailure` |
| `WorkspaceReader` | the workspace's packages and targets | `WorkspaceDescription`, `PackageDescription`, `TargetDescription` | `WorkspaceReadFailure` |
| `ReportRenderer` | a report as text | `RiskReport` | `RenderFailure` |

`SourcePath` (`workspace/source_path.rs`) is a path held as text with `/` as its
only separator, compared by component; `cli`'s `PortablePath` converts the
platform's paths to it. `RunFailure` (`invocation/run_failure.rs`) is the run's
error, a variant per way to fail; a failure it passes on keeps its own message
and cause. Core's tests implement every port in memory, in `core/tests/support/`.

## Resolution layer (core)

`ManifestResolver` (`manifest_resolver.rs`) asks its `WorkspaceReader` for the
workspace and turns a package selection into `PackageContext` values.
`SourceRootCollector` (`source_root_collector.rs`) contributes each production
target's parent directory as a source root; `test`, `bench`, `example` and
`custom-build` targets are excluded, so a package's own test tree is never
scored, and a package with none falls back to its `src`.

## Reporting layer (core)

- `ReportSelection` (`report_selection.rs`) — `has_offenders`, which decides
  the verdict from the same selection the report shows so the two can never
  disagree, `select_visible`, and `report`, the `RiskReport` a renderer is
  handed: every file at or above the threshold ranked, the `top` of them, and
  the totals across every scored file. It ranks before it truncates, because
  `Runner` appends one package's reports after another and taking `--top` off
  that order would cut by package rather than by score.
- `TextReportRenderer` (`text_report_renderer.rs`) — the summary table, the
  offender detail and the summary line, as one string.
- `OffenderDetailRenderer` (`offender_detail_renderer.rs`) — per-offender
  function detail.
- `FileRiskReport` (`file_risk_report.rs`) is the shape every surface projects.
  `RiskOrdering` (`risk_ordering.rs`) is the single comparator the ranking
  uses — descending score, ties broken on file name, total even when a score
  will not compare.

## cli

`Args` (`invocation/args.rs`, `clap::Parser`) parses argv; `without_cargo_subcommand`
strips the subcommand name cargo inserts at `argv[1]`, and `config` is the
`Config` the core reads. `EntryPoint` (`invocation/entry_point.rs`) is the
composition root and the only place that prints or decides an exit code;
`main.rs` hands it the process's argv. The adapters live in `system/`:
`CargoWorkspaceReader` (`cargo metadata --no-deps`, translated into core's
records; the manifest path stays here), `WalkdirSourceTree` (a `.rs` path that
is not UTF-8 fails the listing), `JsonReportRenderer` (the machine surface, with
serialisable records of its own, every offender and no `--top` limit) and
`PortablePath`.

## Related

- [FORMULA.md](FORMULA.md) — every scoring term, and what to do about a breach.
- [ADRs/](ADRs/) — why the codebase is shaped this way.
- [ROADMAP.md](ROADMAP.md) — what's shipped and what's planned.
- [history/](history/) — implemented plans, kept as they landed.
