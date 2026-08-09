# Validated newtypes: private values and verifier-owned evidence

[`src/lib.rs`](src/lib.rs) separates two claims that ordinary string types
conflate: that a value parsed, and that someone proved they own it.

## What it establishes

`EmailAddress` and `BoundedName` hold private `String` fields with checked
constructors, so a caller cannot skip the check by building the struct
literally. Both implement `TryFrom<String>`, which makes the fallible path the
idiomatic one. Both implement `Debug` by hand, so a log line cannot leak the raw
value.

`VerifiedEmailAddress` is a distinct type that only `EmailVerifier::accept` can
produce, and `accept` takes an `OwnershipProof` the verifier alone constructs.
Parsing an address therefore cannot be mistaken for verifying it: the two facts
have two types, and the stronger one has a single gate.

## What it does not establish

The parser is an example syntax policy, not a complete RFC 5321 or RFC 5322
implementation. It rejects shapes a real mailbox may legitimately use, and
accepts shapes a real provider may reject. `VerifiedEmailAddress` proves this
example verifier accepted a proof; it is not evidence of mailbox ownership,
deliverability, or continued control. A production verifier authenticates the
external evidence before constructing the proof.

## Evidence

Four unit tests cover the separator cases a weak email check accepts, whitespace
normalization, the empty and over-long name rejections, and the fact that only
the verifier consumes an ownership proof. The compiler-rejection half — that the
verified type cannot be forged from outside — lives in
[`doctrine-compile-fail`](../compile-fail/README.md).

```text
cargo test --locked -p validated-newtypes
```

## Doctrine

Cited by [RUST-DOC-0001](../../doctrines/0001-invalid-states/README.md) and
[RUST-DOC-0008](../../doctrines/0008-testing-and-evidence/README.md), and by the
[smart constructors](../../patterns/smart-constructors.md),
[opaque newtypes](../../patterns/opaque-newtypes.md), and
[validated collections](../../patterns/validated-collections.md) patterns.
