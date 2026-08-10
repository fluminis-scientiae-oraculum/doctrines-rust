# Staged protocol: successor capabilities as part of each stage contract

[`src/lib.rs`](src/lib.rs) is the largest example in the workspace. It models a
registration flow in which each stage names its own legal successor in the type
system, so the stage graph cannot drift away from its documentation without a
compiler error.

## What it establishes

Every stage trait — `Canonicalize`, `CheckIdentity`, `AcceptPolicy`,
`PreparePersistence`, `ResolveConflict` — declares its successor as an
associated type bounded by the capability that successor has to satisfy. The
bound is the protocol edge. Removing it, or pointing it at a different stage,
changes what the compiler accepts rather than only what the prose says.

Two entry stages implement the same first capability and produce _different_
concrete successors carrying different evidence. That is the property a
hardcoded successor return type cannot express, and it is why the successor is
an associated type rather than a fixed struct.

`IdentityOutcome` and `Recovery` make the branching explicit: an identity check
yields either an available or a conflicting successor, and a rejected stage
offers a revision path rather than a dead end. Consent is a `ConsentProof` tied
to a `PolicyVersion`, so accepting policy version 3 is not evidence of accepting
version 4.

## What it does not establish

The protocol is one in-process pass. It ends at a persistable value and
deliberately does not claim a durable write: a consuming Rust transition moves a
local value, and moving a local value does not consume a stored fact. There is
no database, no competing writer, no restoration path, and no async
cancellation. Availability is modeled as undetermined rather than known.

## Evidence

Twelve unit tests exercise both entry paths, both branches, both recovery edges,
an invited revision carried through to the terminal stage, undetermined
availability, stale consent, and malformed input failing at canonicalization.
Contract assertions derive each successor capability from its trait alone; edge
assertions pin every concrete edge against the documented topology. Four
compiler-rejection cases in
[`doctrine-compile-fail`](../compile-fail/README.md) cover stage skipping,
consumed-stage reuse, stage duplication, and evidence forgery.

```text
cargo test --locked -p staged-protocol
```

## Doctrine

Cited by [RUST-DOC-0010](../../doctrines/0010-staged-protocols/README.md),
[RUST-DOC-0001](../../doctrines/0001-invalid-states/README.md), and
[RUST-DOC-0002](../../doctrines/0002-error-modeling/README.md); by the
[successor capabilities](../../patterns/successor-capabilities.md) and
[executable narrative](../../patterns/executable-narrative.md) patterns; and
worked end to end in the
[registration and onboarding case study](../../case-studies/registration-onboarding/README.md).
