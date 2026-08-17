# Architecture Decision Records

Each file records one decision that shaped this codebase, with the forces that
made it and the alternatives it displaced. Unlike the priority-tiered ADRs of
larger ecosystem repos, these are not tiered: `iceberg4rust` is a single-crate
CLI with a small enough decision surface that a flat list is sufficient.

## Index

| ADR | Decision |
|---|---|
| [ADR-FileScopeIsTheWholeSubject](ADR-FileScopeIsTheWholeSubject.md) | The unit is the file and the subject is private implementation — a file can pass every function-level gate and still be unreadable, and a maximum cannot express accumulation. |
| [ADR-HiddenMeansUnreachable](ADR-HiddenMeansUnreachable.md) | A function counts as hidden only if it is unreachable from outside, so trait-impl methods are excluded from `P` while their complexity still counts. |
| [ADR-CargoSubcommandPackaging](ADR-CargoSubcommandPackaging.md) | The crate publishes as `cargo-iceberg4rust` with library `iceberg4rust`, and strips the subcommand name cargo re-inserts at `argv[1]`. |

## Template

```markdown
# ADR-<Name>

## Status
Accepted / Superseded by <ADR>.

## Context
The forces and tension this resolves.

## Decision
The choice, in one quotable sentence.

## Forcing constraints / Evidence
Why this was forced, not freely chosen — the real evidence. `N/A` if none.

## Rejected alternatives
What we did not do, and why.

## Consequences
What it commits us to; what it costs; obligations pushed onto consumers.

## Enforcement
The specific test, gate, or structural mechanism that keeps it true.
`N/A` if purely structural.

## Related
Links to other ADRs and architecture docs.
```

Fields that do not apply are marked `N/A` rather than padded. Each ADR is a
snapshot of the decision as it stands today, not a changelog — state the current
shape as fact, don't narrate what an earlier version of this document used to say.
