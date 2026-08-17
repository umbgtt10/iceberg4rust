# Roadmap

What has shipped, and what is under consideration. Phases that were never built
are not listed; this is a record of the tool as it stands, not of intentions.

## Shipped

### 0.1.0 — first standalone release

- File-level scoring: `FileRisk = (log2(1 + L) / 10) * (P + 0.5·sum(C_i) + 0.5·D + 2.0·B)`
- Hidden means unreachable — trait-impl methods excluded from `P`, their
  complexity retained
- Workspace resolution through cargo metadata, with repeatable `--package`
- `--threshold` with a default of `20`, `--top` for table rows only
- `--json` with per-file `functions[]`, `data_structs[]`, `behavioral_structs[]`
- Console offender detail: name, line and complexity per private function, with a
  `(trait)` marker
- Exit-code contract: `0` clean, `2` offenders, `1` error
- Remediation documented, including the two non-remedies

## Under consideration

### Detect unit structs used by bare name

The one correctness gap currently recorded in
[OPEN_POINTS.md](OPEN_POINTS.md). Under-reporting is the wrong direction for a
gate, so this ranks above anything additive.

### Per-package summaries

`total_risk` is currently one number across the whole run, which cannot be
compared between runs over different package sets. A per-package subtotal would
make the figure meaningful for a workspace.

### `--baseline` for ratcheting

The threshold is a fixed bound. A recorded baseline would let a repository ratchet
downward — accepting today's offenders while refusing new ones — without
maintaining an allow-list in a shell script, which is where that logic currently
lives for every consumer.

### A machine-readable schema version in `--json`

The JSON surface is a contract with automated consumers. It carries no version
field, so a consumer cannot tell a shape change from a data change. Cheap to add
before there are consumers; awkward afterwards.

## Explicitly not planned

- **Type resolution.** Would improve struct classification and see through
  aliases, at the cost of requiring a compiling crate with vendored dependencies
  — which loses the ability to run on a broken tree, exactly when a structural
  gate is most useful.
- **Calibrating the weights against a corpus.** See
  [OPEN_POINTS.md](OPEN_POINTS.md); no such corpus exists, and building one from a
  single project would encode its taste as arithmetic.
- **Per-function output as a primary mode.** That is a function-level tool's job.
  The detail exists to say what is inside an offending file, not to rank functions
  across a codebase.
