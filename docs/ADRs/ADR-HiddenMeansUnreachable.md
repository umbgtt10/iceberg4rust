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

A function counts toward `P` only if it is genuinely unreachable from outside.
That cuts in two directions, and both follow from the same test — can something
outside this file get at it?

Methods of a trait implementation are **excluded** even though they carry no
visibility modifier. Anyone holding the trait can call them.

Restricted visibility is **included**. `pub(crate)`, `pub(super)`,
`pub(in path)` and `pub(self)` are all unreachable from outside the crate, so
they are hidden by the same test that excludes trait methods. Only `pub` escapes.

Trait-impl methods are still collected. Their complexity counts toward
`sum(C_i)` and the private structs they use count toward `D` and `B`. Breadth of
a trait is free; logic inside one is not.

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

The symmetric error is worth stating because it was made. Reading only
`Visibility::Inherited` excludes trait-impl methods correctly by accident, and
excludes `pub(crate)` incorrectly by the same accident. Observed on a consensus
codebase: a validator extracted into its own file with eleven `pub(super)`
methods scored **zero** and vanished from the report entirely, while the file it
came from dropped 26.89 to 9.00. Nineteen complexity left the metric without
becoming reachable by anything. Under a project that forbids `#[cfg(test)]`
tests, prefixing every private function with `pub(crate)` would zero a file
outright — no relocation, no tests, no design change.

## Rejected alternatives

- **Trusting `Visibility::Inherited` alone.** Simple, syntactic, and wrong twice
  over: wrong for every trait impl in existence, and wrong for every restricted
  function, which it silently treats as public.
- **Weighting restricted visibility below private.** A second tunable constant
  to justify, for a distinction that does not exist from outside the crate:
  `pub(crate)` and `fn` are equally unreachable there.
- **A flag to choose the reading.** Defensible for a codebase that unit-tests
  inside the crate, where `pub(crate)` really is test-reachable. Not built: the
  threshold is already the place to express disagreement, and an unexercised
  option is worse than a stated position.
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
toward `P` while summing complexity over all of them. `is_private_item` matches
`Visibility::Inherited | Visibility::Restricted(_)`, so `pub` is the single exit.
`tests/private_function_collector_tests.rs` pins that a trait-impl method is not
hidden, an inherent one is, a free function is, a trait-impl method still carries
its complexity, and a file holding both marks each correctly. It also pins each
restricted form one by one — `pub(crate)`, `pub(super)` inside a module,
`pub(in path)`, `pub(self)` — and that a file mixing all four with a `pub`
function counts exactly the four. `tests/analyzer_tests.rs` pins the rule that
matters: the same source scores identically whether a helper is written `fn` or
`pub(crate) fn`.
`tests/analyzer_tests.rs` pins the two ends: a 63-method no-op trait impl scores
zero, and a trait impl carrying branching still scores.

## Related

- `ADR-FileScopeIsTheWholeSubject.md` — why the unit is the file.
