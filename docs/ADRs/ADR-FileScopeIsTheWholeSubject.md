# ADR — File scope is the whole subject

## Status

Accepted.

## Context

Rust tooling measures functions. Cyclomatic and cognitive complexity, CRAP,
coverage, lint counts — all of them take a function as the unit and report the
worst one. That catches a function nobody should have written.

It does not catch a file nobody should have to read. A file can hold twenty-seven
private helpers, none above complexity 2, and every function-level gate passes.
Measured on a consensus implementation of roughly 76 KLOC, average complexity per
private function across the seven worst files ranged from **0.6 to 3.4** — an
order of magnitude below any usable function-level threshold. Those files were
wrong at a scope nothing was measuring.

## Decision

The unit is the file, and the subject is private implementation: how much
machinery sits beneath the public surface, not how bad any single piece of it is.

```text
FileRisk = (log2(1 + L) / 10) * (P + 0.5 * sum(C_i) + 0.5 * D + 2.0 * B)
```

Breadth and depth both count. `P` charges for how many private things exist,
`sum(C_i)` for how involved they are, `D` and `B` for the private types they drag
along. A file scores badly for having a great deal of ordinary machinery, which is
exactly the case a per-function maximum cannot express.

## Forcing constraints / Evidence

The composite is not a decoration on its inputs. Ranking the same 41 files by each
input separately and comparing the top seven against the top seven by score:

| ranked by | overlap | missed |
|---|---:|---|
| `private_fns` | 6/7 | `protocol.rs` |
| `effective_loc` | 5/7 | two files |
| `sum(C_i)` | 5/7 | two files |

`protocol.rs` carries only 9 private functions and would never surface on a count,
but holds the highest `sum(C_i)` in the tree at 31 — half of it in one function.
No single column finds it.

## Rejected alternatives

- **Extending a function-level metric to report per-file maxima.** The worst
  function in a file of 27 trivial helpers is still trivial. A maximum cannot
  express accumulation.
- **Counting lines alone.** Correlates with the score but misses both the file
  dense in private helpers and the short file with one heavy function; five of
  seven overlap is not a substitute.
- **Calibrating the weights empirically.** There is no labelled corpus of
  "files that should have been split", and inventing one would encode a single
  project's taste as arithmetic. The constants are judgment and are documented as
  such.

## Consequences

The threshold is an agreed bound, not a discovered boundary. Nothing here claims
a file above it is badly designed — only that it exceeds what was agreed, which
is a fact rather than an opinion.

Because breadth counts, a file can breach without any individual part being
objectionable, and the right remedy is sometimes to *inline* helpers rather than
extract more. `docs/FORMULA.md` says so, so that the reflex to extract does not
become automatic.

`P` and `sum(C_i)` overlap deliberately: a complex private function is charged
twice, once for existing and once for its depth. That is intended, and stated in
the README's limitations rather than hidden.

## Enforcement

`Analyzer::compute_file_risk` takes plain numbers and no context, so the formula
is testable in isolation; `tests/analyzer_tests.rs` pins each term's direction —
larger files score higher for the same logic, behavioural structs outweigh
data-only ones, and increasing complexity increases the score.

## Related

- `ADR-HiddenMeansUnreachable.md` — why trait-impl methods are excluded from `P`.
- `ADR-CargoSubcommandPackaging.md` — how the crate is published and invoked.
