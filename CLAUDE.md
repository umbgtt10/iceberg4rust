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

A workspace of three members, with a virtual root that holds no package:

| member | what it is |
|---|---|
| `core/` | the analysis: package `iceberg4rust-core`, library `iceberg4rust`. `#![no_std]`, `#![forbid(unsafe_code)]`, no technology -- see "Technology-free core" below. Published. |
| `cli/` | the command: package `cargo-iceberg4rust`, library `iceberg4rust_cli`, binary `cargo-iceberg4rust`. `Args`, the composition root `EntryPoint`, and every adapter. Published. |
| `xtask/` | gate orchestration, run as `cargo xtask`. `publish = false`. |

All three are gated. The split is not cosmetic: with a single root package the
package directory *is* the repository, so `xtask/tests/` was read as source
belonging to the root package and every test in it broke `test-free-source`.
Siblings under a virtual root each get their own package directory, which is
what lets `xtask` carry its own `tests/` and be measured like any other crate.

Shared dependency versions and metadata live in the root `[workspace.dependencies]`
and `[workspace.package]`; members inherit with `{ workspace = true }`, which is
what `workspace-dependencies` enforces. `core` and `cli` share the workspace's
`version`. `[workspace.lints.rust]` forbids `unsafe_code` in every member and
every target; `core` says so at its crate root as well. `LICENSE` stays at the root and is not
copied into the members, matching the rest of the tool family; `license = "MIT"`
in each manifest is what declares it.

## Quality Gates

### Mandatory after every change to any member's `src/` or `tests/`

Run gates:

`just stage1`
`just stage2`

If either gate is not green, the work is not complete.

Stage 1 is formatting, clippy and tests -- cargo built-ins only, so it works on
a fresh checkout. Stage 2 is `cargo xtask stage2`, which orchestrates seven
gates in this order:

| gate | asks |
|---|---|
| Technology-free core | does `core` still touch no technology |
| `cargo stern4rust` | do the house coding rules hold |
| `cargo shear` | is every declared dependency used, in the section that uses it |
| `cargo dry4rust` | did this change add duplicated code |
| `cargo crap4rust` | is any function complex and untested |
| `cargo twin4rust` | does every source file have a mirrored test file |
| `cargo iceberg4rust` | is any file's private implementation risk too high |

The technology-free gate runs **first**. It is the xtask's own -- no tool to
install -- and asks three things of `core`: that its manifest names only
`libm`, `proc-macro2` and `syn`, in any section (read from `cargo metadata
--no-deps`; a package missing from the metadata fails); that `core/src/lib.rs`
carries `#![no_std]` and `#![forbid(unsafe_code)]`, which hand `core/src` to the
compiler; and that no file under `core/src` or `core/tests` reaches
`std::{env,fs,io,net,os,process}`, a printing macro, a file-system method of
`Path` or `extern crate std`, read with syn so a string literal is data. The
`core/src` scan is a backstop to the compiler; `core/tests`, where std is
linked, depends on it. Calls inside a macro's arguments are not seen, and a
method of one's own named like a file-system one would be flagged. Its fix is
moving code into `cli`, which changes what every gate behind it measures, and
it takes a second.

stern4rust runs **second** because its corrections are renames, file moves and
directory splits: a layout it is about to reject is a layout the others would
have measured for nothing. Its findings are also the cheapest to act on.

All twenty-four of its rules are enforced, with nothing skipped, nothing
selected and nothing unconfigured: `stern4rust.toml` carries no `skip` and no
`rules` list, and there is no stern baseline. That includes `returned-mutations`
(0.14.0): no `&mut` parameter in `src/`, so a helper returns what it built
rather than writing it through an argument. `docs/header.txt` holds the
three-line header every `.rs` file carries and `stern4rust.toml` names it -- in
the config rather than the gate script, so a hand-run of `cargo stern4rust`
checks exactly what the gate checks. `spdx-matches-manifest` reads `license`
from each member's manifest, inherited from `[workspace.package]`.

cargo-shear runs **third**: its corrections edit manifests only -- a
dependency removed, or moved to the section that uses it -- so they change
nothing the gates behind it measure, and it takes a second. It runs `--locked`,
reading the lock file and never writing it, and `--deny-warnings`, so an
optional dependency no feature's code uses fails as an unused one does. A
dependency it cannot see used -- linked only for what it provides, declared
only to switch on a feature, used only by generated code -- goes in
`[package.metadata.cargo-shear] ignored`, or the workspace's, and the gate
refuses an entry without its reason in the same table's `reasons`. The one
today is `proc-macro2`, declared for its `span-locations` feature: core reads
line numbers from syn's spans, and only that feature gives a span its line.
Pinned at 1.14.0, as CI installs it.

dry4rust runs **fourth** for the same reason stern4rust runs early: removing a
duplicate moves code between files, which changes what the three behind it
measure. It makes one run over the workspace with the trees it must not judge
excluded -- `xtask`, which is not published, and every `tests/`, which repeat
their arrangement by design -- each by its full path, since dry4rust matches an
exclusion as a substring of a file's path and a bare `tests` would also match a
checkout under a directory of that name. It checks with zero ceilings against
`dry4rust-baseline.json`, the duplication
already there when the gate arrived. So it fails on what a change adds, not on
what it inherited; a duplicate removed is admitted, a copy added to a recorded
group is not.

The baseline is empty. The two exact groups the first measurement found -- two
`Visit` methods recording a path's last segment, and three two-character
comment-marker tests in `CommentStripper` -- were shared rather than recorded,
so any duplicated unit at or above the floor fails the gate.

It counts only code units of 25 AST nodes or more, the floor the rest of the
tool family uses. Below that sit one-line accessors and trait methods whose
sameness is the signature's rather than a copy.

Re-record the baseline only to drop groups that are gone, never to admit new
ones, and at the same floor -- a baseline matches only at the floor it was
recorded at: `cargo dry4rust --path "$PWD" --exclude "$PWD/xtask" --exclude "$PWD/core/tests" --exclude "$PWD/cli/tests" --min-nodes 25 --baseline "$PWD/dry4rust-baseline.json" baseline`.

`cargo install just`
`cargo install cargo-stern4rust`
`cargo install cargo-dry4rust`
`cargo install cargo-crap4rust`
`cargo install cargo-twin4rust`
`cargo install --locked cargo-shear --version 1.14.0`
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

A ruleset already exists, so the usual operation is an update rather than a
create. POST is for the first application only — running it again makes a
second ruleset rather than replacing the first, and both then apply. Find the
id and update in place:

```sh
gh api repos/umbgtt10/iceberg4rust/rulesets --jq '.[] | "\(.id)\t\(.name)"'
gh api repos/umbgtt10/iceberg4rust/rulesets/<id> --method PUT --input .github/rulesets/main.json
```

Only if the repository has no ruleset at all:

```sh
gh api repos/umbgtt10/iceberg4rust/rulesets --method POST --input .github/rulesets/main.json
```

The file round-trips, so applying an unchanged one is a no-op. Server-generated
fields — `id`, `node_id`, timestamps, `_links`, `source`, `current_user_can_bypass`
— are stripped, because they are per-instance and would go stale in git. That is
also why the id is looked up rather than recorded here.

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

## Technology-free core

`core` touches no technology -- no cargo, file system, process, environment,
argument parsing, serde or stdout -- so mutation testing it needs nothing but
its own tests, which run in about a hundredth of a second.
`.cargo/mutants.toml` scopes cargo-mutants to `core/src`, each mutant tested by
core's tests only.

Whatever technology core needs is a port: a trait in `core/src/traits/`
(`SourceTree`, `WorkspaceReader`, `ReportRenderer`), the data it speaks in
`core/src/workspace/` or beside its user, an in-memory fake for core's tests in
`core/tests/support/`, and the real adapter in `cli/src/system/`, composed in
`cli`'s `EntryPoint`. Paths in core are `SourcePath`s, text whose only
separator is `/`; `cli`'s `PortablePath` converts at the edge, and a `.rs` path
that is not UTF-8 fails the run rather than being skipped. core's failures are
typed (`RunFailure` and the records it wraps) and keep their causes, so `cli`'s
`main` prints the same chain anyhow always printed. `log2` is `libm`'s: core
cannot reach the platform's.

## Publishing

`cli` publishes as `cargo-iceberg4rust` so cargo resolves `cargo iceberg4rust`,
matching `cargo-crap4rust` and `cargo-dry4rust`. The library `iceberg4rust`
publishes as `iceberg4rust-core`, with a short README of its own.

A release publishes **both**, `iceberg4rust-core` first, since `cli` names it by
version: `cargo publish --workspace` orders them and leaves `xtask` out. Their
shared version moves once, in `[workspace.package]` and in the root's
`iceberg4rust-core` dependency together.

`cli`'s `EntryPoint` strips the repeated subcommand name that cargo inserts as
`argv[1]`; running the binary directly does not repeat it, so the strip is
conditional. `Args::without_cargo_subcommand` owns that rule and is tested.

Before publishing, `cargo publish --workspace --dry-run` must succeed.

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

- one object per file; records where record-home puts them
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
