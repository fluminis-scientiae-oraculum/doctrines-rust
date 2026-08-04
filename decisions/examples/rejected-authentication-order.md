---
id: ADR-EXAMPLE-0002
title: Example of a rejected record — authentication before authorization
status: example
scope: example-rejected-record
created: 2026-08-04
---

# Example: a record that should not be written

**This is an illustration, not a record.** It records no obligation of this repository, and it is
absent from `manifest/decision-records.yaml`. It shows a proposal failing the test in
[RUST-DOC-0011](../../doctrines/0011-executable-narrative/), and what happened instead.

## The proposal

> **ADR: Authentication must happen before authorization.**
>
> Context: several handlers were found calling the authorization check on unauthenticated
> requests. Decision: authentication always precedes authorization. Consequences: handlers are
> restructured; reviewers check the ordering.

The decision is correct. The record is the wrong artifact for it.

## Where it fails the test

**Which exact fact cannot be executable?** None was named, and none exists. The ordering is
expressible as a successor bound, so the claim can be enforced rather than described.

**Why would a generated view be insufficient?** The question was never reached, because the
enforceable form is not a view at all.

**Which future mistake does the record prevent?** The stated mistake is a handler calling
authorization on an unauthenticated request, which the compiler can reject outright. A record
prevents that mistake only by being read at the right moment by the right person.

**What event makes it obsolete?** No condition can be stated, because the obligation is not
contingent on anything external. A record with no obsolescence condition cannot be expired, and
under `RUST-DOC-0011-R009` it would survive indefinitely with no mechanism to remove it.

**Is it onboarding prose in decision form?** Largely. Its useful content is an explanation of the
sequence, which belongs with the code that enforces the sequence.

## What was done instead

The obligation moved into the mechanism that rejects violations of it:

```rust
pub trait Authenticate: Sized {
    /// Authorization is the only capability an authenticated request leads to.
    type Next: Authorize;
    type Error;

    fn authenticate(self) -> Result<Self::Next, Self::Error>;
}
```

A committed compiler rejection preserves the negative guarantee, so the claim that the ordering
cannot be skipped is evidenced by a program that fails to compile rather than by a sentence. A
contract assertion derives `Authorize` from `Authenticate` alone, so deleting the bound breaks
the build instead of leaving every existing test green.

The reason the order exists, which is short and genuinely useful, stayed as a doc comment on the
trait and a cross-reference to the governing doctrine. That is one sentence beside the mechanism,
not a discoverable artifact competing with it.

## The residue

There was none, so nothing was recorded. Had part of this decision been contingent on an
external fact, for example a certification requiring authentication events to be retained for a
stated period, that part alone would have become a narrow record, and the ordering would still
have become a bound.
