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

## Quality Gates

### Mandatory after every change to `src/` or `tests/`

Run gates:

`powershell -File scripts\run_stage_1.ps1`
`powershell -File scripts\run_stage_2.ps1`

If either gate is not green, the work is not complete.

Stage 2 runs three gates: CRAP, mirrored tests, and `iceberg4rust` against
itself. A tool that enforces a bound it does not respect is not worth
installing.

The self-gate ceiling is a ratchet set just above the current worst file, not at
the shipped default — a default-sized bound would never fire on a crate this
small. Lower it when the score improves; never raise it to turn a red build
green.

## Publishing

The crate publishes as `cargo-iceberg4rust` so cargo resolves `cargo iceberg4rust`,
matching `cargo-crap4rust` and `cargo-dry4rust`. The library is `iceberg4rust`.

`src/main.rs` strips the repeated subcommand name that cargo inserts as
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
