# cargo-iceberg4rust

[![CI](https://github.com/umbgtt10/iceberg4rust/actions/workflows/ci.yml/badge.svg)](https://github.com/umbgtt10/iceberg4rust/actions/workflows/ci.yml)

**How much is your file hiding below the waterline?**

`cargo-iceberg4rust` is a static analysis tool that measures **file-level private
implementation** — how much machinery a Rust source file conceals behind its
public surface. It produces a risk score per file, ranks the offenders, and names
every private function, its line and its complexity.

---

## The problem

Function-level metrics answer "is this function too complex?" A file can pass
every one of them and still be a problem:

- 27 private helpers, none above complexity 2, all in one file
- one private function carrying half the file's total complexity
- eleven pure builders tangled with four functions that reach storage
- a decoder and an encoder that share nothing but a filename

None of those files contains a bad *function*. They are wrong at a scope that
function-level tools do not measure, which is why nothing else reports them.

---

## The formula, briefly

```
FileRisk = (log2(1 + L) / 10) × (P + 0.5·ΣCᵢ + 0.5·D + 2.0·B)
```

`L` effective lines, `P` private functions, `Cᵢ` their cognitive complexity,
`D` and `B` the private data-only and behavioural helper structs they use.

Only `pub` keeps a function out of `P`. `pub(crate)`, `pub(super)` and the other
restricted forms count as hidden, because nothing outside the crate — including
an integration test, which is a separate crate — can reach them.

Trait-implementation methods do **not** count toward `P` — they are reachable by
anyone holding the trait, so they are contract rather than hidden machinery. Their
complexity still counts. Full derivation, every term and every weight:
**[`docs/FORMULA.md`](docs/FORMULA.md)**.

---

## Documentation

| Doc | What's in it |
|---|---|
| [`docs/FORMULA.md`](docs/FORMULA.md) | Every scoring term, the thresholding rule, and what to do about a breach. |
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | How an invocation flows through the code, module by module. |
| [`docs/ADRs/`](docs/ADRs/) | Why the codebase is shaped the way it is. |
| [`docs/ROADMAP.md`](docs/ROADMAP.md) | What's shipped, what's next. |
| [`docs/OPEN_POINTS.md`](docs/OPEN_POINTS.md) | Known gaps, deliberately deferred. |
| [`CHANGELOG.md`](CHANGELOG.md) | Release history. |

---

## Installation

```sh
cargo install cargo-iceberg4rust
```

## Usage

```sh
cargo iceberg4rust [OPTIONS]
```

| Option | Description |
|---|---|
| `--manifest-path <PATH>` | Manifest to analyse. Defaults to the working directory. |
| `--package <NAME>` | Package to analyse; repeatable. Required for a workspace of several members. |
| `--threshold <N>` | Files scoring at or above `N` are reported as offenders. Default `20`. |
| `--top <N>` | Show at most `N` offenders in the table, keeping the `N` riskiest across every scanned package. Display only — never affects the verdict. Default `20`. |
| `--json` | Emit the report as JSON. |
| `-h`, `--help` | Print help |
| `-V`, `--version` | Print version |

### Exit codes

| Code | Meaning |
|---|---|
| `0` | No file at or above the threshold |
| `2` | Offenders found |
| `1` | The tool failed — bad manifest, unreadable source |

`2` is distinct from `1` so a caller keeping its own allow-list can tell "found
something" from "broke". A caller that wants neither can simply check for
non-zero.

---

## Output

```
cargo-iceberg4rust report

package             file                                 loc    private_fns  private_complexity  data_structs  behavioral_structs        risk
------------------  ---------------------------------  -----  -------------  ------------------  ------------  ------------------  ----------
cargo-iceberg4rust  src/private_helper_classifier.rs     100              7                  14             0                   0        9.32
cargo-iceberg4rust  src/private_function_collector.rs     85              8                  10             0                   0        8.35

Offender detail:

  cargo-iceberg4rust  src/private_helper_classifier.rs  (risk 9.32, 7 private fns, complexity 14)
      line  complexity  function
        32           3  collect_structs
        69           3  upgrade_impls
        79           3  try_upgrade_impl
        59           2  recurse_collect_structs
       100           2  recurse_upgrade_impls
        46           1  try_insert_data_struct
       111           0  impl_target_name

summary: scored_files=11 visible_files=2 threshold=8.00 top=2 total_risk=37.93
```

(`iceberg4rust`'s own source, analysed by itself at `--threshold 8` — the shipped
default of `20` reports nothing on a crate this small.)

The summary table says which file is over; the detail says what is in it, so
there is somewhere to go without opening the file first. Functions are ordered by
complexity, because that is where an extraction usually starts. A `(trait)`
marker flags a method that does not count toward `private_fns`, so the list never
appears to contradict the tally.

`--json` carries the same data with `functions[]`, `data_structs[]` and
`behavioral_structs[]` per file, ordered by descending risk.

---

## What a breach means

A score over the threshold is a **contract breach, not a design verdict**. It says
one file carries more private implementation than the contract allows. Clearing it
means moving, removing or reducing real implementation — never making the number
smaller.

The usual fix is relocation: extract what is not this file's own subject, then
compose it back — moved to the owning type, simply composed, monomorphised, or
injected behind a seam. Sometimes the answer is not relocation at all: delete,
replace with a library facility, inline over-decomposition, or simplify.

Two things lower the number without moving any implementation and are **not**
remediation: widening visibility, and rehousing logic in a trait impl. Both are
named explicitly in [`docs/FORMULA.md`](docs/FORMULA.md#what-is-not-remediation),
because an automated consumer will find them before it finds the real fix.

---

## Limitations

- **The constants are hand-picked.** The weights and the size factor reflect
  judgment, not measured correlation with anything. The threshold is an agreed
  bound, not a claim about where "bad" begins.
- **No type resolution.** Analysis runs on a parsed `syn` file with no build step,
  so it works on a tree that does not compile — and cannot see through a macro or
  a type alias.
- **`P` and `ΣCᵢ` overlap.** A complex private function contributes to both, so
  complexity is weighted twice: once by existing, once by depth.
- **Struct usage detection is syntactic.** See
  [`docs/OPEN_POINTS.md`](docs/OPEN_POINTS.md).

---

## Development

The repository is a workspace of two members: `core/` is the published crate
(package `cargo-iceberg4rust`, library `iceberg4rust`), and `xtask/` is the
gate orchestration behind `cargo xtask`. Both are gated.

Mandatory after every change to either member's `src/` or `tests/`:

```sh
just stage1
just stage2
```

Stage 1 is formatting, clippy and tests — cargo built-ins only, so it works on
a fresh checkout. Stage 2 runs `cargo xtask stage2`, which runs, in this
order: `cargo stern4rust` (house coding rules), `cargo crap4rust` (complexity
vs. coverage), `cargo twin4rust` (every source file has a mirrored test
file), and `cargo-iceberg4rust` against itself — a tool that enforces a bound
it does not respect is not worth installing.

Required tools, none of which come with `cargo` itself:

| Tool | Install |
|---|---|
| [`just`](https://github.com/casey/just) | `cargo install just` |
| `cargo-stern4rust` | `cargo install cargo-stern4rust` |
| `cargo-crap4rust` | `cargo install cargo-crap4rust` |
| `cargo-twin4rust` | `cargo install cargo-twin4rust` |
| [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov) | `cargo install cargo-llvm-cov` |
| `llvm-tools` rustup component | `rustup component add llvm-tools` |

`cargo-llvm-cov` is `cargo crap4rust`'s own coverage backend, not a house
tool — it isn't visible until stage2 actually runs, at which point a missing
install shows only as `cargo llvm-cov failed with exit code Some(101)`.

`cargo-iceberg4rust` itself needs no separate install for the self-gate —
`cargo xtask stage2` builds and runs it straight from this checkout.

CI (`.github/workflows/ci.yml`) runs both stages on Ubuntu, Windows and macOS
on every push and pull request. `main` requires a pull request with those six
jobs green; the ruleset that enforces it is kept in
[`.github/rulesets/main.json`](.github/rulesets/main.json) and applied with
`gh api`, since GitHub holds it as server-side config that no clone or fork
would otherwise carry.

---

## License

MIT — see [`LICENSE`](LICENSE).
