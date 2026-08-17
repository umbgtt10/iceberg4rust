# cargo-iceberg4rust

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
| `--top <N>` | Show at most `N` offenders in the table. Display only — never affects the verdict. Default `20`. |
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

## License

MIT — see [`LICENSE`](LICENSE).
