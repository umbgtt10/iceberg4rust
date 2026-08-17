# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
