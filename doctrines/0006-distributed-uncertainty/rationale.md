# Rationale

## Communication failure is not execution evidence

In a request-response exchange, the request can be lost, execute and return, or
execute while the response is lost. A caller observing a timeout cannot
distinguish all of these cases without additional protocol evidence. Local
cancellation affects the future being awaited; it does not retract bytes
already received by another system.

Binary `Result<Success, Failure>` is suitable only when the error side retains
the distinctions callers need. Consequential effects often benefit from a
domain-specific shape:

Local failures known to occur before dispatch can remain an outer error or an
additional variant. The important property is not the generic spelling but
that confirmed rejection and unknown execution cannot be confused.

## Idempotency is a protocol

A random string alone proves none of these. Client-generated keys can be
reliable identities when generation occurs once per logical intent and every
attempt reuses them. Generating a key inside the retry loop defeats the
protocol.

## Naturally idempotent is contextual

Setting a resource to a complete desired value can be idempotent at that
resource boundary, while triggering notifications or audit records on every
call remains non-idempotent. Deleting an already absent object may be
idempotent in state but return different observations. An operation's
idempotency claim must name the effect set and response semantics.

Commutativity is different: two increments may commute but replaying one still
changes the total. Deduplication is different again: it recognizes a repeated
identity and suppresses or replays the prior result.

## At-least-once consumers

A consumer that acts before acknowledging can crash after the effect but before
the acknowledgement. Redelivery follows. A consumer that acknowledges first
can crash before the effect and lose work. An inbox stored atomically with a
local database effect can close this gap for that database, but cannot
automatically include a remote payment or email.

Deduplication retention must cover the broker's replay horizon, operational
replay policy, and worst outage. Removing the key reopens the operation. A
unique inbox entry without atomic effect coordination can also record completion
before the effect occurred.

## Reconciliation is normal execution

Reconciliation queries can fail and observations can be stale. A response
saying "not found" may be definitive only after the provider's processing and
retention windows. The state machine must allow repeated observation without
turning uncertainty into imagined certainty.

## Ordering is scoped

One producer can assign monotonically increasing sequence numbers for one
aggregate. A partitioned broker can preserve partition order. Parallel
consumers, retries, and dead-letter replay can still change processing order.
Global total order usually requires stronger coordination and may reduce
availability or throughput.

## Exactly once requires a boundary

Some stream processors can coordinate input offsets and output state in one
transaction. A database unique constraint can make one operation identity apply
once to that database mutation. These are valuable guarantees, but they do not
automatically encompass an HTTP call, human action, email delivery, or an
uncoordinated database.

Anything outside that sentence remains subject to duplicates, loss, or
uncertainty.

## Compensation is not time reversal

Refunding a payment does not make capture unoccur. Releasing inventory later
does not guarantee the same customer experience or price. Deleting a created
resource may fail because another party now depends on it. Compensation is a
new command under current reality, with authorization, idempotency, timeout,
and reconciliation of its own.

Saga state must retain both forward and compensating outcomes. A failed
compensation may require manual resolution rather than pretending the original
transaction rolled back.

## Leases, clocks, and stale owners

A lease grants authority for a bounded period according to some clock and
renewal protocol. A paused or partitioned worker may continue after another
worker acquires a new lease. Fencing tokens let the protected resource reject
operations from older owners. A distributed lock that cannot fence the effect
may reduce overlap likelihood without preventing stale-owner execution.

Clock skew, process pauses, renewal delay, and resource support belong in the
guarantee ledger. Rust ownership can prevent cloning a local lease handle; it
cannot revoke authority already accepted by a remote system.

Tests then force overlap, delayed renewal, and bound failure at the protocol
seam. They provide scoped evidence for the implementation; they do not prove
that production clocks or processes always stay within the bounds.

## Audit without secret replication

Incident reconstruction needs stable IDs, timestamps, target identity,
fingerprints, attempt outcomes, and causal links. It rarely needs raw
credentials, full payment data, or message secrets. Hashes used as fingerprints
must be chosen with awareness of low-entropy values and correlation risk.
Access and retention should match the evidence's sensitivity.

## Guarantee ledger

| Claim                                         | Established by                             | Protected construction        | Boundary preservation    | Escape hatches          | Does not prove                                         | Residual runtime risk          |
| --------------------------------------------- | ------------------------------------------ | ----------------------------- | ------------------------ | ----------------------- | ------------------------------------------------------ | ------------------------------ |
| operation has stable identity                 | generated once and persisted               | private operation constructor | reused across attempts   | administrative replay   | effect executed once                                   | identity collision, misuse     |
| provider confirmed capture                    | authenticated response or reconciled event | outcome transition            | evidence retained        | operator override       | later settlement                                       | provider reversal, stale event |
| capture is unknown                            | timeout after possible dispatch            | explicit variant              | token persists           | destructive manual edit | success or rejection                                   | delayed observation            |
| duplicate local DB effect is suppressed       | unique inbox plus atomic mutation          | repository transaction        | durable identity         | retention expiry        | remote side effect uniqueness                          | late replay                    |
| worker currently holds time-bounded authority | checked acquisition plus clock contract    | non-clone authority           | fencing sent with writes | raw backend access      | synchronized clocks or exclusive remote action forever | pause, skew, partition, expiry |

## Proportionality

Not every telemetry ping needs a reconciliation worker. Best-effort actions may
accept loss or duplication when the product contract says so. The design still
states that choice. Consequential financial, authorization, provisioning, and
user-visible effects usually justify durable identities and explicit unknown
states. Type and storage complexity should track the cost of an incorrect
repeat, lost action, or false status.
