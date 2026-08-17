# Open Points

Known gaps, deliberately recorded rather than silently assumed correct. Each
entry states what was actually observed, not what is suspected.

## A unit struct used by bare name is not detected

`StructUsageCollector` visits struct literals (`Expr::Struct`), constructor calls
(`ExprCall`), type paths and return types. A unit struct referenced by its bare
name — `let helper = BehaviouralHelper;` — is an `Expr::Path`, which is not
visited, so the struct does not count toward `D` or `B`.

Observed while writing `analyze_source_names_the_helper_structs_it_counted`: a
fixture using a unit struct reported zero behavioural structs where one was
expected. The fixture was changed to a struct with a field, which is detected;
the gap was left in place rather than widening detection inside an unrelated
change.

The effect is under-reporting, which is the wrong direction for a gate — a file
using several unit-struct helpers is charged less than one using the same number
of fielded helpers. Not started. The fix is a `visit_expr_path` arm, with care
taken that a path naming a *type* in expression position is distinguished from an
ordinary variable of the same name.

## `P` and `sum(C_i)` charge complexity twice

A complex private function contributes to `P` by existing and to `sum(C_i)` by its
depth. This is deliberate — breadth and depth are both intended to count — but it
means the two terms are not independent, and a file's score responds more sharply
to one heavy function than a reading of the formula suggests.

No action planned. Recorded so that anyone recalibrating the weights knows the
terms overlap before treating them as orthogonal.

## The constants are judgment, not measurement

`0.5` for complexity, `0.5` for data structs, `2.0` for behavioural structs, and
`log2(1 + L) / 10` for size. None is derived from a corpus. The ratio between the
struct weights encodes a belief — that a private type carrying behaviour costs a
reader roughly four times what a plain data holder does — which is plausible and
unverified.

No action planned without a labelled corpus of files that should have been split,
which does not exist. Building one from a single project would encode that
project's taste as arithmetic, which is the alternative
`ADR-FileScopeIsTheWholeSubject.md` rejects.

## The threshold has no empirical basis

`20` is the default because it sat above the 90th percentile of a real codebase at
the time it was chosen, not because files above it are known to be bad. It is an
agreed bound, and the ratchet is the contract; nothing here claims to have located
where "too big" begins.

Recorded so the default is not mistaken for a finding.

## Score comparisons across packages are not meaningful

`total_risk` sums every scored file in the run, so it grows with the size of the
selection. Two runs over different package sets cannot be compared, and the number
is useful only as a before-and-after within one selection.

No action planned; the per-file scores are the intended unit.

## A `pub` item inside a private inline module is treated as reachable

`is_private_item` reads an item's own visibility. `process_mod` recurses into an
inline `mod` without tracking whether that module is itself reachable, so a `pub`
function inside a private `mod` is excluded from `P` even though nothing outside
can call it.

This is the same defect class as the restricted-visibility gap fixed in 0.2.0 —
an item that looks reachable syntactically but is not — and the same test
applies: can something outside the crate get at it?

Left in place deliberately. Correcting it means threading module reachability
through the recursion, including a public module nested inside a private one,
and shipping it alongside the visibility change would have made the repricing of
existing codebases impossible to attribute to either cause. The observed impact
is small where `pub mod` is confined to `mod.rs` and `lib.rs`, which is the
convention in the trees this was measured against.

The effect is under-reporting, the wrong direction for a gate.
