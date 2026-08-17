# Architecture

How a `cargo iceberg4rust` invocation flows through the code, module by module.

## The pipeline

```text
main.rs
  └─ Args::parse_args()            argv, with the cargo subcommand name stripped
      └─ Runner::run(args)
          1. ManifestResolver      cargo metadata -> Vec<PackageContext>
             a. select the requested packages, or the single root package
             b. TargetRootCollector derives source roots from production targets
          2. Analyzer::analyze_package(), per package
             a. SourceFileWalker enumerates .rs files under each source root
             b. per file: CommentStripper counts effective lines,
                PrivateHelperClassifier splits helper structs into data and
                behavioural, PrivateFunctionCollector gathers private functions
                with their name, line, complexity and struct usage
             c. compute_file_risk() turns those into a score
             d. files scoring zero are dropped before becoming reports
          3. output
             a. --json  -> JsonReportRenderer
             b. default -> ReportPrinter, then OffenderDetailRenderer
          4. ReportPrinter::has_offenders() decides the exit code
```

## Analysis layer

`Analyzer` (`analyzer.rs`) owns the per-file computation and the formula itself.
`compute_file_risk` takes plain numbers and no context, so every term's direction
is testable without parsing anything.

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

## Resolution layer

`ManifestResolver` (`manifest_resolver.rs`) turns a manifest path and a package
selection into `PackageContext` values. `TargetRootCollector`
(`source_root_collector.rs`) contributes each production target's parent
directory as a source root; `test`, `bench`, `example` and `custom-build` targets
are excluded, so a package's own test tree is never scored.

`SourceFileWalker` (`source_file_walker.rs`) enumerates `.rs` files beneath a
root.

## Reporting layer

Three renderers, each returning or printing one thing:

- `ReportPrinter` (`report_printer.rs`) — the summary table, the trailing
  summary line, and `has_offenders`, which decides the exit code from the same
  selection the table shows so the two can never disagree. `select_visible`
  ranks before it truncates, because `Runner` appends one package's reports
  after another and taking `--top` off that order would cut by package rather
  than by score.
- `OffenderDetailRenderer` (`offender_detail_renderer.rs`) — per-offender
  function detail. Returns a `String`, so it is testable without capturing
  stdout.
- `JsonReportRenderer` (`json_report_renderer.rs`) — the machine surface. States
  its own ordering rather than inheriting the caller's, and applies no `--top`
  limit, because a consumer wants every offender.

`FileRiskReport` (`file_risk_report.rs`) is the shape both surfaces project.
`RiskOrdering` (`risk_ordering.rs`) is the single comparator all three ranking
sites share — descending score, ties broken on file name, total even when a
score will not compare.

## CLI layer

`Args` (`args.rs`, `clap::Parser`) parses argv and strips the subcommand name
cargo inserts at `argv[1]`. `Config` (`config.rs`) is the plain-data form the
resolver consumes. `Runner` (`runner.rs`) is the only place that decides an exit
code; `main.rs` is a thin entry point.

## Related

- [FORMULA.md](FORMULA.md) — every scoring term, and what to do about a breach.
- [ADRs/](ADRs/) — why the codebase is shaped this way.
- [ROADMAP.md](ROADMAP.md) — what's shipped and what's planned.
