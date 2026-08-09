# Compile-fail evidence: what the compiler refuses

A passing test shows that a legal program works. It cannot show that an illegal
program is rejected, because an illegal program does not compile and therefore
cannot be a test. This crate closes that half of the evidence.

## What it establishes

[`src/lib.rs`](src/lib.rs) exposes the public API the rejections are written
against: a verified email type whose evidence constructor is private, a
`Connection` whose `send` exists only in the open state, a `Payment` whose
`capture` exists only after authorization, and a `Transaction` whose `commit`
consumes the handle.

[`tests/ui.rs`](tests/ui.rs) runs `trybuild` over `ui/*.rs`. Each case is a
program that has to fail, paired with a `.stderr` snapshot of the exact
diagnostic. Nine cases are recorded:

| Case                                                                         | Refusal                                                    | Error   |
| ---------------------------------------------------------------------------- | ---------------------------------------------------------- | ------- |
| [capture before authorize](ui/capture_before_authorize.rs)                   | capturing a payment that was never authorized              | `E0308` |
| [send using closed connection](ui/send_using_closed_connection.rs)           | sending through a handle still in the closed state         | `E0308` |
| [reuse consumed transaction](ui/reuse_consumed_transaction.rs)               | staging a mutation after `commit` consumed the handle      | `E0382` |
| [construct verified email directly](ui/construct_verified_email_directly.rs) | calling the private evidence constructor                   | `E0624` |
| [construct verified email fields](ui/construct_verified_email_fields.rs)     | building the verified type from its private field          | `E0451` |
| [skip protocol stage](ui/skip_protocol_stage.rs)                             | handing a canonicalized stage straight to the policy stage | `E0308` |
| [reuse consumed stage](ui/reuse_consumed_stage.rs)                           | advancing twice from a stage one transition already moved  | `E0382` |
| [clone stage to duplicate](ui/clone_stage_to_duplicate.rs)                   | duplicating a stage value to run a branch twice            | `E0599` |
| [forge stage evidence](ui/forge_stage_evidence.rs)                           | constructing consent evidence outside the issuing stage    | `E0451` |

The last four are written against
[`staged-protocol`](../staged-protocol/README.md); the rest against this crate's
own API. Each `Error` column entry is the code the paired snapshot currently
records.

## What it does not establish

A `.stderr` snapshot pins the message the current compiler emits, not the reason
it emits it. A compiler upgrade can change the wording without changing the
refusal, and regenerating a snapshot after such a change is routine — but
regenerating one after a _behavior_ change silently discards the evidence.
Review a `.stderr` diff for which error is reported, not only for whether the
suite is green.

## Evidence

One unit test asserts the legal programs still compile and run, so a refusal
cannot be manufactured by breaking the API for everyone. The nine rejections run
under `trybuild` in the same command:

```text
cargo test --locked -p doctrine-compile-fail
```

Snapshots regenerate with `TRYBUILD=overwrite` prefixed to that command.

## Doctrine

Cited by [RUST-DOC-0008](../../doctrines/0008-testing-and-evidence/README.md),
[RUST-DOC-0001](../../doctrines/0001-invalid-states/README.md),
[RUST-DOC-0003](../../doctrines/0003-ownership-and-capabilities/README.md),
[RUST-DOC-0005](../../doctrines/0005-persistence-boundaries/README.md), and
[RUST-DOC-0010](../../doctrines/0010-staged-protocols/README.md).
