# Distributed outcomes: the third answer a remote call can give

[`src/lib.rs`](src/lib.rs) models the outcome a two-valued `Result` cannot
carry: the call neither succeeded nor failed, because the answer was lost.

## What it establishes

`OperationOutcome` has a third case beside success and failure, and
`AttemptObservation` records what the caller actually saw rather than what it
inferred. `decide_retry` turns an observation into a `RetryDecision`, and the
decision is explicit about reconciliation: an undetermined attempt is not a
failed one, so it is not retried blindly.

`AttemptIdentity` pairs an `OperationId` with an `IdempotencyKey`, and a retry
reuses both. That is the point of the type — a fresh key requests a second
effect, so identity has to survive the retry for the retry to be safe.
`ReconciliationToken` carries the provider reference needed to ask the remote
system what actually happened.

## What it does not establish

There is no network, no clock, and no provider. The crate proves the decision
function distinguishes the three observations and preserves identity across a
retry; it does not prove any particular provider honors an idempotency key,
returns a stable reference, or reconciles within a bounded time. Timeout
selection, backoff policy, and the durability of the reconciliation record are
all outside it.

## Evidence

Four unit tests cover an ambiguous non-idempotent attempt routed to
reconciliation, a retry that reuses the same operation and idempotency key, an
unknown outcome that does not collapse into a rejection, and identity
normalization.

```text
cargo test --locked -p distributed-outcomes
```

## Doctrine

Cited by [RUST-DOC-0006](../../doctrines/0006-distributed-uncertainty/README.md),
[RUST-DOC-0001](../../doctrines/0001-invalid-states/README.md),
[RUST-DOC-0002](../../doctrines/0002-error-modeling/README.md), and
[RUST-DOC-0008](../../doctrines/0008-testing-and-evidence/README.md), and by the
[explicit uncertainty](../../patterns/explicit-uncertainty.md) pattern.
