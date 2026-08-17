# Formula

Every scoring term, in full, kept in sync with `src/`.

`iceberg4rust` computes a file-level score for how much private implementation a
Rust source file hides beneath its public surface — the mass below the waterline.

It is deliberately separate from a CRAP-style metric. That one is function-scoped
and combines complexity with coverage; this one is file-scoped and asks a
different question, which is why a file can be clean by every function-level
measure and still be over this bound. See
[ADRs/ADR-FileScopeIsTheWholeSubject.md](ADRs/ADR-FileScopeIsTheWholeSubject.md).

## The score

For a file, define:

- `L`: effective file size in non-blank, non-comment lines
- `P`: number of private functions and private methods in the file
- `C_i`: cognitive complexity of private function `i`
- `D`: number of unique additional private data-only structs used by at least one private function
- `B`: number of unique additional private behavioral structs used by at least one private function

The file risk score is:

```text
FileRisk = (log2(1 + L) / 10) * (P + 0.5 * sum(C_i) + 0.5 * D + 2.0 * B)
```

## Zero Baseline

The score must be `0` when the file has one struct and zero private functions.

That follows directly from the formula because `P = 0`, `sum(C_i) = 0`, `D = 0`, and `B = 0`.

It is a derived property, not a special case: the score is never short-circuited
on `P = 0` alone. A file whose only behaviour lives in a trait implementation has
`P = 0` while `sum(C_i)` may be positive, and that complexity must survive into
the score.

## Definitions

### Effective File Size

`L` is computed from production source only and counts non-blank, non-comment lines.

### Private Functions

`P` counts:

- free functions with inherited visibility
- **inherent** impl methods with inherited visibility

It excludes methods of a trait implementation. Such a method is reachable by
anyone holding the trait, so it is the type's contract rather than its hidden
machinery — inherited visibility there reflects the trait's visibility governing
it, not privacy. Counting them made a wide trait implemented with empty bodies
the highest-risk shape in the tree, which inverts what this metric is for.

Their complexity is still counted in `sum(C_i)`, and the private structs they
use still count toward `D` and `B`. Breadth of a trait is free; logic inside one
is not.

It excludes test-only code paths such as:

- files under `tests/`, `examples/`, and `benches/`
- `build.rs`
- items inside `#[cfg(test)]` or other test-tagged items

### Additional Private Structs

The file's primary struct is the first top-level struct declared in the file.

Additional private structs are split into two classes.

#### Data-Only Private Structs

`D` counts unique private structs other than the primary struct when they are used by at least one private function and do not define methods.

#### Behavioral Private Structs

`B` counts unique private structs other than the primary struct when they are used by at least one private function and define one or more non-test methods.

Usage is detected through:

- parameter or return types
- explicit local variable types
- struct literal expressions
- tuple struct constructor calls

Each matching private struct is counted once per file, not once per use site.

## Output

Two surfaces, carrying the same data.

The console report opens with a summary row per offender — package, file,
effective LOC, private function count, complexity sum, data-only and behavioural
struct counts, and the risk score — sorted by descending risk, ties broken on
file name.

Beneath it, per-offender detail names every private function in that file with
its line and its individual complexity, ordered by descending complexity, because
that is where an extraction usually starts. A method that does not count toward
`P` carries a `(trait)` marker, so the list never appears to contradict the
tally. Helper structs are named when the file has any, and the line is omitted
when it does not.

`--json` emits `threshold`, `scored_files`, `visible_files`, `total_risk` and
`files[]`, each file carrying the same counts plus `functions[]` (`name`, `line`,
`complexity`, `is_hidden`), `data_structs[]` and `behavioral_structs[]`. The JSON
states its own ordering rather than inheriting the caller's, and applies no
`--top` limit — a machine consumer wants every offender.

`scored_files` counts files that produced a non-zero score, not files walked. A
file scoring zero is dropped before it becomes a report.

## Thresholding

`--threshold` sets the bound; the default is `20`.

- every production source file is analysed
- only files with `FileRisk >= threshold` are reported
- `--threshold 0` shows every file that scored above zero
- `--top` limits how many rows the table prints. It is display only and never
  affects which files are offenders or what the process exits with

The threshold is an agreed bound, not a discovered boundary. Nothing here claims
a file above it is badly designed — only that it exceeds what was agreed.

### Exit codes

| Code | Meaning |
|---|---|
| `0` | No file at or above the threshold |
| `2` | Offenders found |
| `1` | The tool failed — bad manifest, unreadable source |

`2` is distinct from `1` so a caller keeping its own allow-list can tell "found
something" from "broke". A caller wanting neither distinction can test for
non-zero. The exit code is decided from the same selection the table shows, so
the two can never disagree.

## Remediation

A score over threshold is a contract breach, not a design verdict. It says one
file carries more private implementation than the contract allows. Stage 1
proves correctness; this is stage 2, proving structure stays within agreed
bounds. Clearing it never means "make the number smaller" — it means moving,
removing or reducing real implementation.

### Relocation — the usual fix

Extract the logic that is not this file's own subject into one or more new
files, then compose it back. Four forms, in rough order of preference:

1. **Move to the owning type.** No new file. A private helper that mostly
   manipulates another type belongs on that type. `valid_transaction_gas(&self,
   tx: &Transaction)` is `Transaction`'s business.
2. **Simply composed.** The extracted type becomes a field, constructed
   directly. Correct when there is one implementation and nothing to fake.
3. **Monomorphised.** A generic parameter where the caller varies by type but
   dispatch need not be dynamic.
4. **Injected behind a seam.** A trait in `traits/`, passed to the single
   constructor. Reserve this for a dependency that is slow, non-deterministic,
   or reaches hardware, storage or the network — a seam concentrates untestable
   code, and at a boundary with no logic behind it there is nothing to
   concentrate.

### When relocation is not the answer

- **Delete.** Dead helpers, or logic duplicated in a sibling. Always try first.
- **Replace with a library facility.** A hand-rolled encoder cluster is often a
  `serde` impl. This removes `P` and `sum(C_i)` together.
- **Inline over-decomposition.** `P` counts helpers regardless of size, so a
  file of many trivial single-use helpers scores high with no complexity behind
  it. Where a helper exists only to name three lines used once, folding it back
  is a genuine simplification. Where the name carries meaning, it is not —
  judge by whether the name survives being read aloud.
- **Reduce `sum(C_i)`.** Early returns, table-driven dispatch instead of a long
  match. Leaves `P` alone but lowers the score and the reading cost.

### What is not remediation

These lower the number without moving any implementation. They are the ways an
automated consumer will be tempted to satisfy the gate, so they are named here:

- **Widening visibility.** A `pub` function leaves `P` by definition. Correct
  only when the privacy was accidental and the function is genuinely part of
  the contract; otherwise it trades a structural problem for an API one.
- **Rehousing logic in a trait impl.** Trait-impl methods do not count toward
  `P`. Implementing a trait because the abstraction is right is good design;
  implementing one to move methods out of the tally is not.
- **Splitting a file at an arbitrary line.** Two files each under the bound, cut
  through the middle of one idea, is worse than the file that was over it.

The check is the same in each case: after the change, is there less hidden
implementation in this package, or only in this file?

### Worked examples

Observed on a Byzantine-fault-tolerant consensus implementation of roughly
76 KLOC, at `threshold = 20`.

`src/ibft/ibft_protocol/protocol.rs` — 9 private fns, `sum(C_i)` 31, risk 20.87.
Depth, not breadth: `try_peer_discovery_phase` (line 253) carries complexity 15,
half the file's total, and `try_rejoin_ping_phase` (line 290) is its sibling.
Extract a join-phase driver holding both, with the `is_rejoin_ping_message`
(318) and `is_readiness_adaptation_message` (322) predicates that classify for
them. Simply composed — one implementation, nothing to fake.

`src/tiny_evm/tiny_evm_storage_slot_analyzer.rs` — 27 private fns, `sum(C_i)`
15, risk 27.60. The inverse: breadth with almost no depth, max complexity 2.
The five `apply_*_opcode` arms (78–132) are one dispatch family and the stack
operations under them are another. Two extractions, composed. Note this is also
the shape where inlining deserves consideration first — check whether each
helper's name earns its place before moving it.

`src/context/snapshot_context_builder.rs` — 18 private fns, risk 22.42. Eleven
pure `build_*_context` shapers tangled with four `load_*` functions that reach
storage. The only one of the current offenders that wants a seam: a trait over
the four loaders, leaving the eleven shapers testable against a fake.
