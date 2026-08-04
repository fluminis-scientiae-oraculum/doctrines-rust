# Successor capabilities

## 1. Problem

A protocol has several stages, and more than one way to enter it. A self-service signup and an
invitation-based signup both reach the same availability check, but each carries different
evidence: a challenge identifier in one case, an invitation and its issuing account in the other.

Plain typestate returns one concrete successor. Serving both entry paths then requires either one
successor type carrying every possible proof as an option, which reintroduces the contradictory
combinations the stages were built to remove, or a duplicated protocol per entry path. Neither
expresses the actual contract: whatever the entry stage produces, it must be something the
availability check accepts.

## 2. Forces

The protocol has few stable stages, controlled by one owner within one process. A capability may
have several implementations whose successors differ in evidence but agree on what comes next.
Transitions may perform fallible work. The stage graph will be refactored by people who did not
design it. Diagnostics, monomorphization, and the reach of generic parameters into helper and
test code all matter.

## 3. Weak representation

```rust
pub trait Canonicalize {
    /// Returns a value that can then be identity-checked.
    fn canonicalize(self) -> Result<CanonicalRegistration, CanonicalizeError>;
}
```

The successor is hardcoded, so a second implementation cannot carry different evidence. Worse,
the sentence that makes this a protocol lives in a doc comment: nothing checks that
`CanonicalRegistration` can in fact be identity-checked, and nothing notices when it stops being
able to.

## 4. Improved representation

```rust
pub trait Canonicalize: Sized {
    type Next: CheckIdentity;
    type Error;

    fn canonicalize(self) -> Result<Self::Next, Self::Error>;
}

impl Canonicalize for SelfServiceSubmission {
    type Next = CanonicalRegistration<SelfServiceOrigin>;
    type Error = CanonicalizeError;
    // ...
}

impl Canonicalize for InvitedSubmission {
    type Next = CanonicalRegistration<InvitedOrigin>;
    type Error = CanonicalizeError;
    // ...
}
```

The successor is now part of the contract and bounded by the capability it must satisfy. Two
implementations produce different successors carrying different origin evidence, and both are
statically required to lead into the identity check.

**Local name.** This repository's local name for the chainable trait-oriented form is _Chainable
Telescopic Typestate Traits_, abbreviated CT³. A chain gives order, `A → B → C`. A telescope gives
containment: A holds the controlled opening into B, and B holds the controlled opening into C. The
associated successor type is that opening, which is why the present capability carries both proof
of a completed history and permission for a constrained future. The term is local vocabulary and
not standardized terminology; the established families it refines are typestate-oriented
programming, behavioral types, and object protocols, and `RUST-DOC-0010-R021` requires that
attribution to travel with the name. Prefer the descriptive terms in new material, and use the
abbreviation only where a reader arriving from an older internal document needs the bridge.

A branching stage names one successor per outcome:

```rust
pub trait CheckIdentity: Sized {
    type Available: AcceptPolicy;
    type Conflicting: ResolveConflict;
    type Error;

    fn check_identity(
        self,
        directory: &IdentityDirectory,
    ) -> Result<IdentityOutcome<Self::Available, Self::Conflicting>, Self::Error>;
}
```

## 5. Exact guarantee gained

Safe code cannot advance a stage through a capability that stage does not implement, cannot reuse
a stage after a consuming transition, and cannot construct a later stage's evidence when its
fields are private and no public constructor exists. The associated bound additionally guarantees
that every implementation's successor satisfies the next capability, which a hardcoded return
type cannot state and a doc comment cannot enforce.

The bound is the protocol edge in checkable form. A refactor that redirects `type Next` elsewhere
does not merely change a type: it either fails to satisfy the bound, or it silently changes the
graph, which is why the pattern is completed by an executable topology assertion.

## 6. Guarantees not gained

Reaching a later stage proves the in-process protocol ran in order and nothing else. An
availability observation proves no conflict was visible to one reader at one moment, not that the
identity is still free when a row is written. A consent proof records that an offered version
matched the version in force during the check, not that the policy is unchanged at write time.

Most importantly, a consuming transition does not consume a durable fact. A move ends the
caller's use of a local value; a stored row is read into a value and can be read again by another
worker, so two workers can each hold a consumed handle for the same row. Durable advancement
needs identity, stored state, and a version or fencing token re-checked at the authoritative
store.

## 7. Boundary considerations

Untrusted input enters at the first stage and is canonicalized before any stage claims a checked
value. Do not derive a decoder that produces a later stage: a decoded value asserts every proof
that stage represents, and the decoder performed none of them. Where restoration is genuinely
needed, a checked service inspects stored state and issues a typed stage whose claim is scoped to
what it actually verified.

Erase the protocol at one named boundary. A round trip through a map or a dynamic context in the
middle of the graph ends static enforcement for every stage after it, while leaving the
appearance of a typed protocol intact.

## 8. Persistence considerations

Persist a runtime representation, not the stage type. Stage markers are compile-time artifacts;
their spelling in a column is not protocol evidence, and heterogeneous rows do not fit generic
stage types comfortably. A hybrid design is the norm: a runtime enum owns the durable lifecycle,
and the typed protocol covers one in-process pass issued through checked construction. See
[hybrid state machines](hybrid-state-machines.md).

## 9. Testing evidence

Unit-test each transition on success and failure, each branch variant, each recovery edge, and
the survival of canonical values from first stage to last. Add compile-fail cases for the
orderings the protocol claims are unrepresentable: skipping a stage, reusing a consumed stage,
and constructing stage evidence from a literal.

Then assert the topology executably, because compile-fail cases alone do not cover it. A
redirected associated type can leave every existing negative test passing while the edge it
protected no longer exists.

Two assertions are needed, and the difference between them is easy to get wrong. A **contract**
assertion knows only the stage capability and demands that its associated successor satisfies the
next one. Nothing in the helper supplies that bound, so it compiles only while the trait declares
it:

```rust
fn assert_canonicalize_contract<S: Canonicalize>() {
    fn requires_check_identity<T: CheckIdentity>() {}
    requires_check_identity::<S::Next>();
}
```

An **edge** assertion additionally pins the concrete successor:

```rust
fn assert_canonicalize_edge<S, N>()
where
    S: Canonicalize<Next = N>,
    N: CheckIdentity,
{
}
```

The edge form cannot replace the contract form. Its own `N: CheckIdentity` bound silently
supplies whatever the trait lost, so deleting `type Next: CheckIdentity` from `Canonicalize`
leaves a suite of edge assertions entirely green. Write both: contract assertions for the trait
obligations, edge assertions for the concrete graph.

## 10. Costs

Bounds make signatures longer and first-encounter diagnostics worse: a mismatch is reported as an
unsatisfied bound rather than a plain type error. Generic stage parameters travel into helper
functions, mocks, and sometimes public APIs. Each stage adds a type, a failure type, a ledger
row, and an assertion. Monomorphization grows with stages multiplied by implementations.

## 11. When not to use it

Do not use it when one capability will only ever have one implementation, where a concrete
successor return is simpler and equally safe. Do not use it for advisory ordering, for state
determined externally or chosen at runtime, for durable multi-actor lifecycle, or where callers
must hold heterogeneous stages in one collection. Do not add a stage for a transformation that
establishes no fact a later stage consumes. A short pipeline of ordinary functions is often the
better answer.

## 12. Related doctrines

RUST-DOC-0010 governs this mechanism, its bounds, branches, effect disclosure, erasure boundary,
and the limit at which a local transition stops being durable evidence. RUST-DOC-0001 governs
legal transitions and unrepresentable states generally. RUST-DOC-0002 governs the error taxonomy
the stage-specific failures map into. RUST-DOC-0003 governs custody of the values being advanced.
RUST-DOC-0004 governs cancellation of async transitions. RUST-DOC-0005 and RUST-DOC-0006 govern
the durable and ambiguous halves this pattern defers. RUST-DOC-0011 governs which artifact is
authoritative for each claim the protocol makes, and prohibits maintaining a prose copy of the
graph beside the traits that enforce it; see
[executable narrative](executable-narrative.md).

## 13. Executable example

See [`../examples/staged-protocol/src/lib.rs`](../examples/staged-protocol/src/lib.rs) for the
capability traits, two entry implementations with different successors, the branch and recovery
edges, and the topology assertion. Compiler-rejection cases are under
[`../examples/compile-fail/ui/`](../examples/compile-fail/ui/).

## 14. Worked application

A registration protocol canonicalizes a submission, checks identity availability, records policy
consent, and prepares a persistable value. Two entry stages produce canonical registrations
carrying different origin evidence; both satisfy the identity check.

The check branches. Availability leads to the policy stage; conflict leads to a resolution stage
whose revision edge is bounded back to the first capability, so a revised submission re-enters at
canonicalization rather than skipping ahead. A directory that cannot be reached is neither
branch: it is a stage-identifying failure carrying the address, so an operator can look the
attempt up.

Origin evidence is erased to a runtime discriminant exactly once, at the persistence boundary,
and the protocol stops at a persistable value. It does not claim the row was written. That
remains a durable operation which re-checks identity and state under its own concurrency control.

## 15. Review prompts

- Does each nonterminal capability name its successor as a bounded associated type?
- Does any bound name a capability the successor does not actually establish?
- Was a bound widened or removed to make an implementation compile?
- Will more than one implementation produce a different successor, or is a concrete return simpler?
- Is each material branch a named sum over distinct successors?
- Does a revision edge re-enter at the correct stage?
- Is an undetermined outcome distinguishable from both branches?
- Can any conversion, derive, or public constructor produce a later stage?
- Does any nonterminal stage derive `Clone` or `Copy`, allowing a copy to advance separately?
- Does the documented graph have a contract assertion, not only edge assertions?
- Does deleting a successor bound actually break the build?
- Is any local transition being presented as durable evidence?
