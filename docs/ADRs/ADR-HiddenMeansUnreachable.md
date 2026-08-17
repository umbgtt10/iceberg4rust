# ADR — Hidden means unreachable, not un-annotated

## Status

Accepted.

## Context

`P` counts a file's private functions. The obvious implementation reads
`syn`'s visibility: a function with `Visibility::Inherited` — no `pub` — is
private.

That is wrong for trait implementations. A method inside `impl Trait for Type`
never carries a visibility modifier, because the trait's visibility governs it.
Read syntactically, every method of every trait impl looks private. Read
semantically, none of them is: anyone holding the trait can call them.

The consequence was not subtle. A no-op observer implementing a 63-method trait
with empty bodies scored **49.19** — the highest file in the tree — on nothing but
method count. Zero complexity, zero helper structs, no branch anywhere in the
file. The least risky shape a file can have sat at the top of the list, and the
gate that should have acted on it named it as the worst offender in the codebase.

## Decision

A function counts toward `P` only if it is genuinely unreachable from outside:
free functions and **inherent** impl methods with inherited visibility. Methods of
a trait implementation are excluded.

They are still collected. Their complexity counts toward `sum(C_i)` and the
private structs they use count toward `D` and `B`. Breadth of a trait is free;
logic inside one is not.

## Forcing constraints / Evidence

The two readings diverge exactly where the metric's purpose is at stake. The
subject is *hidden implementation* — what a reader cannot get at from the outside
and therefore must read the file to understand. A trait method is part of the
type's contract; it is documented, discoverable and callable. Counting it charges
a file for having a wide interface, which is a different property entirely and one
this tool has no opinion about.

Excluding them outright would swap one error for another: a trait impl can carry
real logic, and a heavy `next` or `poll` is genuine burden. Keeping their
complexity while dropping their count separates the two things that were conflated
— how many hidden things exist, and how involved the file's behaviour is.

## Rejected alternatives

- **Trusting `Visibility::Inherited` alone.** Simple, syntactic, and wrong for
  every trait impl in existence.
- **Excluding trait impls entirely, complexity included.** Loses a genuinely heavy
  implementation, and would make "move it into a trait impl" a complete escape
  rather than a partial one.
- **Excluding methods with empty bodies instead.** Fixes the observed symptom
  without fixing the cause: a wide trait implemented with one-line bodies would
  still dominate the ranking.

## Consequences

Moving a method into a trait implementation removes it from `P`. Where the trait
is the right abstraction that is good design; where it is done to satisfy the
gate it is not, and no structural rule can tell the two apart. `docs/FORMULA.md`
names it under "What is not remediation" rather than leaving it as a loophole to
be discovered.

Removing the count without removing the complexity meant the score could no longer
be short-circuited on `P = 0`. A file whose only behaviour lives in a trait impl
has `P = 0` and positive `sum(C_i)`, and that has to survive into the score. The
zero baseline is therefore derived from the formula rather than enforced ahead of
it.

## Enforcement

`PrivateFunctionCollector::process_impl` reads `item_impl.trait_.is_none()` into
`PrivateFunctionMetrics::is_hidden`; `Analyzer` counts only hidden functions
toward `P` while summing complexity over all of them.
`tests/private_function_collector_tests.rs` pins that a trait-impl method is not
hidden, an inherent one is, a free function is, a trait-impl method still carries
its complexity, and a file holding both marks each correctly.
`tests/analyzer_tests.rs` pins the two ends: a 63-method no-op trait impl scores
zero, and a trait impl carrying branching still scores.

## Related

- `ADR-FileScopeIsTheWholeSubject.md` — why the unit is the file.
