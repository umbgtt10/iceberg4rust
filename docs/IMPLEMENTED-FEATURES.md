# Implemented Features

This document describes the feature set currently shipped by
`cargo-iceberg4rust`. For the scoring policy these implement, see
[FORMULA.md](FORMULA.md); for released versions, see
[CHANGELOG.md](../CHANGELOG.md).

## Version 0.1.0

### Analysis

- File-level risk score:
  `FileRisk = (log2(1 + L) / 10) * (P + 0.5·sum(C_i) + 0.5·D + 2.0·B)`
- `P` counts free functions and **inherent** impl methods with inherited
  visibility; trait-impl methods are excluded, because a trait method is
  reachable by anyone holding the trait
- Trait-impl complexity still counts toward `sum(C_i)`, and the private structs
  such methods use still count toward `D` and `B`
- Helper structs are classified as data-only or behavioural, weighted `0.5` and
  `2.0` respectively
- Effective lines exclude blank lines and comments
- `#[cfg(test)]` and other test-tagged items are stripped before scoring
- The zero baseline is derived from the formula, never short-circuited on `P = 0`
- Files scoring zero are dropped before becoming reports
- Analysis runs on a parsed `syn::File` with no type resolution and no build
  step, so it works on a tree that does not compile

### Resolution

- Manifest and package selection through cargo workspace metadata
- Repeatable `--package`; a workspace of several members requires it
- Only production targets contribute source roots — `test`, `bench`, `example`
  and `custom-build` are excluded, so a package's own tests are never scored

### Reporting

- Summary table: package, file, effective lines, private functions, complexity
  sum, data and behavioural struct counts, risk
- Per-offender detail: every private function by name, line and individual
  complexity, ordered by complexity, with a `(trait)` marker on methods outside
  the private-function tally
- Named helper structs, shown only when there are any
- `--json` carrying `threshold`, `scored_files`, `visible_files`, `total_risk`
  and `files[]`, each with `functions[]`, `data_structs[]` and
  `behavioral_structs[]`
- JSON ordering is stated by the contract, descending by risk with ties broken on
  file name, and applies no `--top` limit
- `--top` limits table rows only and never affects the verdict

### Gate contract

- Exit `0` when no file is at or above the threshold
- Exit `2` when offenders are found, distinct from `1` for a tool error, so a
  caller with its own allow-list can tell the two apart
- The exit code uses the same selection the table shows

### Packaging

- Publishes as `cargo-iceberg4rust` with library `iceberg4rust`
- Invoked as `cargo iceberg4rust`; the subcommand name cargo inserts at `argv[1]`
  is stripped conditionally, so direct invocation is unaffected and a package
  legitimately named `iceberg4rust` survives as a `--package` value
