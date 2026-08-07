# Message delivery: improved design

## Versioned envelope

```rust
pub struct RawEnvelope {
    id: String,
    source: String,
    kind: String,
    schema_version: u16,
    aggregate_id: String,
    aggregate_version: u64,
    correlation_id: String,
    causation_id: Option<String>,
    payload: Vec<u8>,
}
```

The broker adapter enforces frame, decompression, payload, header, and batch
limits. It parses the raw envelope, verifies source identity/permission, parses
stable IDs and versions, then decodes a raw payload DTO. Domain construction
validates values and command policy. Unknown schema or command type follows an
explicit reject/quarantine policy.

Message identity is namespaced by authoritative source. A canonical request
fingerprint binds identity to type, aggregate, version, and payload. Same
identity with a different fingerprint is an integrity/security conflict.

## Inbox and local effect

The consumer begins one database transaction:

1. insert/claim inbox row under unique scoped message ID;
2. if completed with matching fingerprint, return recorded local outcome;
3. if concurrently processing, apply bounded wait/retry policy;
4. load aggregate and compare expected version;
5. perform domain transition;
6. record new aggregate version and audit causality;
7. insert durable external-effect/outbox intent if needed;
8. mark inbox local outcome;
9. commit.

Inbox claim, local effect, aggregate version, audit, and outbox intent share one
actual transaction. Redelivery after commit sees completion and does not repeat
the local mutation. A crash before commit leaves none of those changes durable.

This is a scoped exactly-once local mutation claim for retained identity in one
database. It is not an exactly-once broker or external effect claim.

## Ordering and gaps

Each command carries aggregate version or expected predecessor. If a duplicate
version is already applied with matching identity, return its outcome. If a
newer version arrives while a predecessor is missing, place it in a bounded
waiting/reconciliation state or request current authoritative aggregate data.
If an older conflicting command arrives, reject it as stale.

Ordering is per aggregate/version. No global FIFO is claimed. Commutative command
types can use a separate policy rather than unnecessary strict sequencing.

## External effect

The outbox record creates an `NotificationOperationId` distinct from message
identity but correlated to it. A bounded publisher sends with one provider
idempotency key and records attempts. Outcomes are:

```rust
pub enum NotificationOutcome {
    ConfirmedAccepted(ProviderReceipt),
    Rejected(NotificationRejection),
    Unknown { reconciliation: NotificationReconciliation },
}
```

Broker message acknowledgement need not wait for external confirmation because
durable intent and inbox completion are already committed. External worker
failure leaves recoverable outbox state. Timeout after possible send becomes
unknown. Reconciliation observes provider evidence before unsafe repeat.

## Acknowledgement

After confirmed database commit, the consumer acknowledges the broker message.
If acknowledgement is lost, redelivery hits the inbox and returns recorded
outcome. If acknowledgement fails repeatedly, task supervision and broker
contract decide reconnect or shutdown; no busy loop fabricates success.

The acknowledgement itself does not prove broker retention changed until the
broker's evidence says so, but duplicate handling makes uncertainty safe within
retention.

## Poison and replay

Malformed, unauthorized, unsupported, or repeatedly failing messages move to a
quarantine/dead-letter record with original identity, source, schema, safe
fingerprint, failure category, attempt history, and correlation. Payload access
is restricted and retention matches sensitivity. Queue capacity and retry rate
are bounded.

Repair either changes the message under a new logical intent explicitly linked
to the original, or replays the identical identity after the underlying issue
is corrected. Replay never silently generates a new ID to evade history.

## Evidence

Contract tests cover envelope versions and producer compatibility. Integration
tests use the real codec, database, and broker where feasible. Fault tests crash
at every inbox/local/outbox/ack boundary. Tests deliver duplicates concurrently,
same ID with different payload, out-of-order versions, gaps, poison data,
consumer restart, dead-letter replay, and retention expiry.

External-effect doubles support execution followed by response loss.
Reconciliation tests keep unknown until authoritative evidence arrives.
Overload tests verify bounded fetch, worker concurrency, database pool, and
outbox publisher.

## Guarantee ledger

| Claim                                           | Established by                              | Protected construction | Boundary preservation          | Escape hatches                   | Does not prove                              | Residual runtime risk      |
| ----------------------------------------------- | ------------------------------------------- | ---------------------- | ------------------------------ | -------------------------------- | ------------------------------------------- | -------------------------- |
| trusted command passed current validation       | envelope/source/DTO/domain pipeline         | private command fields | replay uses same pipeline      | audited repair tool              | producer truth or current authority forever | schema/policy change       |
| local effect occurs once per retained identity  | unique inbox claim plus same DB transaction | repository handler     | restart/redelivery reads inbox | direct DB edit, retention expiry | one broker delivery or remote effect        | database failure           |
| aggregate order is checked per key              | expected version transition                 | repository API         | version persisted              | administrative override          | global order                                | missing predecessor        |
| notification intent is durable                  | outbox shares local commit                  | repository transaction | publisher uses operation ID    | direct DB mutation               | provider acceptance                         | publisher outage           |
| notification is unknown after ambiguous timeout | explicit outcome/token                      | outcome constructor    | durable record                 | operator decision                | sent or unsent                              | provider query unavailable |
| poison data is isolated                         | bounded quarantine transition               | consumer supervisor    | original identity retained     | privileged deletion              | later repair correctness                    | backlog/sensitive exposure |

> [!TIP]
> [problem](problem.md) · [naive design](naive.md) · **improved design** · [remaining uncertainty](remaining-uncertainty.md)
> Index: [all case studies](../README.md).
> Executable mechanics live under [`../../examples/`](../../examples/).
