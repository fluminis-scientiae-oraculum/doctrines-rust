# Registration onboarding: naive design

## A design that compiles

```rust
#[derive(Default)]
pub struct Registration {
    address: String,
    display_name: String,
    challenge_id: Option<String>,
    invite_code: Option<String>,
    inviting_account: Option<String>,

    canonicalized: bool,
    identity_checked: bool,
    existing_account: Option<String>,
    consent_version: Option<u32>,
    account_id: Option<String>,
}

impl Registration {
    pub fn canonicalize(&mut self) -> Result<(), Error> { /* ... */ }
    pub fn check_identity(&mut self) -> Result<(), Error> { /* ... */ }
    pub fn accept_policy(&mut self, version: u32) -> Result<(), Error> { /* ... */ }
    pub fn persist(&mut self) -> Result<(), Error> { /* ... */ }
    pub fn notify(&mut self) -> Result<(), Error> { /* ... */ }
}
```

Callers write the sequence they were told to write:

```rust
registration.canonicalize()?;
registration.check_identity()?;
registration.accept_policy(3)?;
registration.persist()?;
registration.notify()?;
```

This is a normal design. It is readable, it is easy to test on the happy path, and every failure
below is available to any caller who writes the calls in a different order.

## What the type permits

**Every order.** `persist` before `check_identity` compiles. So does calling `notify` alone. The
sequence is a convention enforced by whoever reviews the call site.

**Contradictory combinations.** The flag fields admit states the process does not have:
`identity_checked = true` with `existing_account = Some(_)` and `account_id = Some(_)` describes
an attempt that found a conflict and allocated an identity anyway. Nothing rejects it.

**Both entry paths at once.** `challenge_id` and `invite_code` are independent options, so a
registration can carry both, or neither. The type says these are four possible worlds; the domain
has two.

**Repetition.** Because the methods take `&mut self`, `persist` can run twice. The second call
sees `account_id` already set and either writes again or silently returns, depending on which
branch someone wrote.

**Raw strings in checked positions.** After `canonicalize` returns, `address` is a `String`, the
same type it was before. A later method cannot tell canonical from raw, so it either re-derives
or trusts a flag.

## The defect this produces

The availability check is a read, and `persist` is a write. Between them there is a window. Two
concurrent attempts for the same address both observe availability, both set
`identity_checked = true`, and both call `persist`. Unless the database has a unique constraint,
both succeed.

Adding a constraint catches it, but the naive design has no place to put the resulting failure:
`persist` returns the same `Error` as everything else, so the caller cannot distinguish "this
identity was taken while you were deciding" from "the database was unreachable" and cannot offer
the applicant a revision.

## Why the obvious repairs are not enough

**Adding assertions.** Beginning each method with a flag check converts a silent misordering into
a runtime error. It is a real improvement, and it still ships the illegal call to production
before anyone learns about it. Every caller must now handle a failure that a stronger
representation would not have had.

**A `state` enum field.** Replacing the flags with `enum Stage { Raw, Canonical, Checked, ... }`
removes the contradictory combinations, which is worth doing, and it is the right choice for the
durable record. It does not remove the illegal calls from the API: `persist` is still callable on
a `Raw` registration and must still reject it at runtime.

**Splitting into free functions.** `persist(check_identity(canonicalize(raw)?)?)?` gets the
ordering from data flow, which is a genuine improvement. It stops working as soon as the check
branches, because the conflicting outcome and the available outcome are different values that
lead to different places.

**Documenting the order.** The sequence goes into a design note. The note and the code then drift
independently, with no signal when they do.

## What is actually missing

None of the repairs addresses the underlying problem: the type of a registration is the same
before and after each step, so it cannot express what has been established. The stages exist in
the developer's head and in the flags, and the compiler has no access to either.

The improved design gives each established fact its own type, makes each transition consume the
previous one, and puts the legal successor in the contract so that the ordering survives a
refactor by someone who never read the design note.
