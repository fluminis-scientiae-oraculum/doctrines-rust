# RUST-DOC-0001 source notes

## Originating teaching source

The initial pedagogical prompt was inspired by the YouTube video
[“How to write peak Rust”](https://www.youtube.com/watch?v=IVcPPT799_A).
The repository does not contain the video or a transcript. The useful teaching
sequence is:

```text
enums → validated newtypes → typestate
```

That sequence helps move from contradictory records, to refined values, to
locally legal operation order. It is a teaching progression, not a ranking in
which typestate is always the final or superior design.

## Accepted core lesson

The doctrine accepts the central lesson that consequential invalidity should
move out of ordinary business operations and into deliberate construction and
state design. Mutually exclusive cases benefit from enums with associated data.
Stable local value invariants benefit from private newtypes and checked
constructors. Small locally owned protocols can benefit from consuming
transitions or typestate.

This aligns with the Rust language's enum model in
[The Rust Reference: enumerations](https://doc.rust-lang.org/reference/items/enumerations.html),
privacy in
[The Rust Reference: visibility](https://doc.rust-lang.org/reference/visibility-and-privacy.html),
and ownership/moves described in
[The Rust Book](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html).
Those sources establish mechanics. Repository rules determine when the mechanics
are proportionate and which evidence reviewers require.

## Refined claims

**Validation is relocated, not deleted.** A private newtype centralizes
validation when values enter the trusted domain. HTTP, messages, databases,
files, configuration, and FFI remain untrusted boundaries. Derived decoding
must not bypass construction.

**Integer representation does not eliminate monetary rounding.** Integer minor
units avoid binary floating-point representation error for those units.
Currency scale, tax, FX, percentage allocation, division, and remainder policy
still need explicit rules. `u64` permits zero; `NonZeroU64` proves only nonzero.

**Email syntax is not ownership.** A parser can establish only its documented
syntax and normalization policy. Mailbox control needs a verification process;
deliverability can change later.

**Typestate is local protocol evidence.** `Connection<Open>` can establish that
the local connect transition returned success. It cannot guarantee the peer
remains reachable at the next instruction. Sending remains fallible.

**Persistence favors runtime state.** Stored and heterogeneous values generally
need stable runtime enums. Rehydrating typestate requires checked current
evidence; marker spelling is not proof.

## Rejected overextensions

The doctrine rejects "make every invalid state unrepresentable" as an absolute.
Cross-entity, temporal, environmental, and distributed facts often require
runtime checks. It rejects typestate as a universal replacement for enums,
unchecked deserialization into trusted types, infallible external effects, and
timeouts treated as confirmed failure.

## Repository additions

The repository adds:

- invariant classification and inventory format;
- evidence-accurate type naming;
- trust-boundary decoding integrity for Serde and databases;
- authority/capability selection;
- explicit unknown/reconciliation states for distributed outcomes;
- guarantee ledgers with non-guarantees;
- public escape-hatch review;
- compile-fail evidence for prohibited programs;
- a proportional complexity budget.

These additions connect the local type-design lesson to persistence,
distributed systems, operations, and agent review.
