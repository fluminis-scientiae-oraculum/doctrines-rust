# Invoice: naive design

## Representation

```rust
struct Invoice {
    paid: bool,
    failed: bool,
    receipt: Option<String>,
    failure_reason: Option<String>,
    recipient: String,
    amount: f64,
    currency: String,
    email_sent: bool,
}
```

This compiles and can pass happy-path tests. It also represents paid and failed
simultaneously, paid without receipt, pending with a failure reason, and
`email_sent = true` before provider acceptance. `f64` admits negative, zero,
non-finite, and binary representation behavior unsuitable as the sole money
policy. Currency is disconnected from arithmetic.

Recipient validation occurs in one request handler:

```rust
if !invoice.recipient.contains('@') {
    return Err("invalid email".to_owned());
}
```

Other paths — database load, administrative import, replay, and tests — construct
the struct directly. The check accepts many meaningless strings and its name
still tempts code to call the address verified.

## Operation

The handler starts a database transaction, marks `email_sent = true`, calls the
provider, commits, and returns success. On any error it sets
`failure_reason = Some(error.to_string())`. A timeout is treated as failure and
the job framework retries with a new request ID.

Several failures follow:

1. The provider accepts the email, the response is lost, and the timeout marks
   delivery failed.
2. Retry sends the same invoice again because the new request ID is not linked
   to the first logical operation.
3. The database update rolls back after provider acceptance; local state says
   no email attempt exists.
4. The database commits `email_sent = true`, but process loss occurs before the
   provider call.
5. A malformed historical recipient bypasses the request check.
6. Error strings erase rejection, availability, timeout, and unknown outcome.

The transaction cannot include the provider. Holding it open across a network
call increases contention but does not create atomicity. `email_sent` collapses
requested, dispatched, accepted, delivered, bounced, rejected, and unknown into
one dishonest bit.

## Evidence weakness

Unit tests create a valid object, mock the provider with immediate success, and
assert `email_sent`. The mock never delays after execution, never loses an
acknowledgement, and never accepts duplicate keys. No test loads invalid rows or
competes concurrent sends. High line coverage would still miss the protocol
gaps.

The implementation makes broad statements:

- floating-point is "fine because Rust";
- `contains('@')` means valid email;
- provider call inside a transaction is atomic;
- timeout means unsent;
- boolean sent means delivered.

None follows from the code. The weak model centralizes no invariant and gives
reviewers no stable claim to audit.

## Operational consequence

Support cannot distinguish duplicate deliveries from an initial failure. A
customer may receive two notices. Operators lack one operation ID across API,
outbox, provider, and reconciliation logs. Retrying every generic error
amplifies provider load. Invalid database records appear as ordinary invoices,
so later failure is attributed to the provider rather than data integrity.

> [!TIP]
> [problem](problem.md) · **naive design** · [improved design](improved.md) · [remaining uncertainty](remaining-uncertainty.md)
> Index: [all case studies](../README.md).
> Executable mechanics live under [`../../examples/`](../../examples/).
