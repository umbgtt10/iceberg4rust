# iceberg4rust

## Meaning

`iceberg4rust` is a cargo subcommand that scores how much private
implementation a Rust source file hides beneath its public surface — the mass
below the waterline.

It is not a function-level metric. A file can pass every complexity and coverage
gate and still breach this one, because the subject is accumulation across a
file rather than the worst function in it. It answers one question: does this
file carry more hidden implementation than the contract allows?

A score over the threshold is a contract breach, not a design verdict.

`docs/FORMULA.md` is the canonical policy description, covering both the scoring
terms and what to do about a breach. If the formula or the remediation guidance
changes, update `docs/FORMULA.md`, the tool logic and the integration tests
together. `docs/ARCHITECTURE.md` maps the code; `docs/ADRs/` holds the
load-bearing decisions.

It is self-contained.

## Boundary Rule

This repository is **SELF-CONTAINED**.

The LLM **SHALL NOT cross its boundaries without asking**.

That means:
- do not inspect, edit, or rely on files outside `iceberg4rust/` unless the user explicitly asks
- do not pull assumptions from sibling repositories or crates
- do not propose cross-repository changes by default

## Layout

A workspace of two members, with a virtual root that holds no package:

| member | what it is |
|---|---|
| `core/` | the published crate. Package `cargo-iceberg4rust`, library `iceberg4rust`. |
| `xtask/` | gate orchestration, run as `cargo xtask`. `publish = false`. |

Both are gated. The split is not cosmetic: with a single root package the
package directory *is* the repository, so `xtask/tests/` was read as source
belonging to the root package and every test in it broke `test-free-source`.
Siblings under a virtual root each get their own package directory, which is
what lets `xtask` carry its own `tests/` and be measured like any other crate.

Shared dependency versions and metadata live in the root `[workspace.dependencies]`
and `[workspace.package]`; members inherit with `{ workspace = true }`, which is
what `workspace-dependencies` enforces. `LICENSE` stays at the root and is not
copied into the members, matching the rest of the tool family; `license = "MIT"`
in each manifest is what declares it.

## Quality Gates

### Mandatory after every change to any member's `src/` or `tests/`

Run gates:

`just stage1`
`just stage2`

If either gate is not green, the work is not complete.

Stage 1 is formatting, clippy and tests -- cargo built-ins only, so it works on
a fresh checkout. Stage 2 is `cargo xtask stage2`, which orchestrates four
gates in this order:

| gate | asks |
|---|---|
| `cargo stern4rust` | do the house coding rules hold |
| `cargo crap4rust` | is any function complex and untested |
| `cargo twin4rust` | does every source file have a mirrored test file |
| `cargo iceberg4rust` | is any file's private implementation risk too high |

stern4rust runs **first** because its corrections are renames, file moves and
directory splits: a layout it is about to reject is a layout the others would
have measured for nothing. Its findings are also the cheapest to act on.

All twenty-one of its rules are enforced, with nothing skipped and nothing
unconfigured. `docs/header.txt` holds the three-line header every `.rs` file
carries and `stern4rust.toml` names it -- in the config rather than the gate
script, so a hand-run of `cargo stern4rust` checks exactly what the gate checks.

`cargo install just`
`cargo install cargo-stern4rust`
`cargo install cargo-crap4rust`
`cargo install cargo-twin4rust`
`cargo install cargo-llvm-cov`
`rustup component add llvm-tools`

`cargo-llvm-cov` is crap4rust's own coverage backend rather than a house tool.
A missing install surfaces only once stage 2 runs, as
`cargo llvm-cov failed with exit code Some(101)`.

The gates are a `justfile` plus an `xtask` workspace member, not scripts: one
entry point that behaves the same on Linux, Windows and macOS, and gate
orchestration in Rust rather than shell text-parsing. `.github/workflows/ci.yml`
runs both stages on all three on every push and pull request.

### Branch protection

`main` requires a pull request with all six CI jobs green; repository admins
bypass both. That is enforced by a GitHub ruleset, which is server-side
configuration rather than anything cargo or git reads — nothing in a clone or a
fork carries it, and deleting it leaves no history.

`.github/rulesets/main.json` is the copy of record. It is applied, not merely
documentation:

```sh
gh api repos/umbgtt10/iceberg4rust/rulesets --method POST --input .github/rulesets/main.json
```

Use `--method PUT` against `.../rulesets/<id>` to update an existing one; the
file round-trips, so re-applying an unchanged file is a no-op. Server-generated
fields — `id`, `node_id`, timestamps, `_links`, `source`, `current_user_can_bypass`
— are stripped, because they are per-instance and would go stale in git.

The file is the state GitHub actually holds, including two defaults GitHub
added on its own: `require_extra_approval_for_unattributed_changes` and the
`allowed_merge_methods` list. Capturing them faithfully is the point — a
setting nobody chose is worth having visible in review.

Editing the ruleset in the web UI puts the file out of date, with nothing to
catch the drift. Change the file and apply it.

The last gate runs `iceberg4rust` against itself. A tool that enforces a bound
it does not respect is not worth installing.

The self-gate ceiling is a ratchet set just above the current worst file, not at
the shipped default — a default-sized bound would never fire on a crate this
small. Lower it when the score improves; never raise it to turn a red build
green.

## Publishing

The crate publishes as `cargo-iceberg4rust` so cargo resolves `cargo iceberg4rust`,
matching `cargo-crap4rust` and `cargo-dry4rust`. The library is `iceberg4rust`.

`core/src/main.rs` strips the repeated subcommand name that cargo inserts as
`argv[1]`; running the binary directly does not repeat it, so the strip is
conditional. `Args::without_cargo_subcommand` owns that rule and is tested.

Before publishing, `cargo publish --dry-run` must succeed.

## Orthogonality, trait surface and cognitive complexity

**When changing productive code, always maximize orthogonality and testable surface through traits, and minimize cognitive complexity.**

Specifically:
- prefer extracting behavior behind traits so individual pieces can be tested and swapped independently
- prefer small, focused methods with a single responsibility over large methods with many branches
- prefer named structs with methods over free functions operating on external state
- when `crap4rust` or a reviewer flags a function as too complex, reduce it by extracting internal structs with methods and adding integration coverage — not by extracting standalone helper functions
- never increase cognitive complexity to pass a test; find the root cause and fix it there
- make constructors depend on traits, not directly on concrete implementations
- ALL dependencies are injected through the SINGLE constructor and stored in the struct
- apply the same split recursively to nested dependencies: trait first, state/data model second, concrete implementation third

## User coding standards

- one struct per file
- no unnecessary comments in code
- unit tests are not allowed. Only integration tests are
- consolidate scattered functions inside structs as appropriate
- no `&mut` input parameters; prefer return values
- only use `pub mod` in `mod.rs` and `lib.rs`
- split test files so there is one test file per source file, named `<source file name>_tests.rs`
- in `all_tests.rs`, reference test files one by one without `#[path = ...]`
- apply AAA (`Arrange`, `Act`, `Assert`) structure to tests with blank-line separation between the three sections
- use `// Arrange & Act` if there is no separate `Arrange`
- use `// Act & Assert` if there is no separate `Act`
- add the repository copyright and license header to every Rust source file
- tests should be named as follows `<method under test>_<test description>_<result>`
- do not use fully qualified paths; use `use` imports instead
