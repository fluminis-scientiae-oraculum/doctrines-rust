# Payment lifecycle: remaining uncertainty

## Provider and network

A local `AuthorizedPayment` proves a checked authorization response was
accepted and bound to the payment under observed policy. It cannot make the
provider reachable, prevent provider-side expiration, or prove capture will
succeed. Network failure can always occur after local checks.

Provider idempotency narrows duplicate risk only within its documented target,
payload binding, concurrency, and retention. A provider defect, manual provider
action, operation after retention, or different merchant account can escape
that boundary. Audit records and reconciliation remain.

## Capture finality

An unknown capture may never resolve if the provider loses searchable history
or gives inconsistent observations. The application can keep it pending,
escalate to support, accept external documentary evidence, or authorize a new
operation with acknowledged duplicate risk. None of these choices proves the
initial attempt failed.

Confirmed capture is evidence at a provider boundary. Settlement, funding,
chargeback, dispute, fraud review, and regulatory reversal remain later states.
The type and UI must avoid calling capture final payment finality.

## Concurrency

Optimistic versioning and work claims prevent cooperating workers from silently
updating one row concurrently. They do not stop direct provider calls,
privileged database edits, or a stale worker whose effect resource ignores
fencing. Effect-level idempotency protects the last boundary.

Policy can change between validation, authorization, and capture. Rechecking can
reduce stale decisions but adds new failure and latency. The product must define
which policy version is binding and when reauthorization is required.

## Settlement and event delivery

At-least-once provider events duplicate and can reorder. Inbox retention must
cover replay. A missing event does not prove no settlement; periodic
reconciliation or provider queries may be necessary. An authenticated event can
still contain a provider error or correction.

Clock timestamps are observations, not global order. Aggregate/provider sequence
numbers are stronger when their scope and gap behavior are documented.

## Reversal and compensation

A reversal does not erase customer-visible capture, interim balance effects,
fees, notifications, or audit history. It may use a different exchange rate or
settle on a different day. Partial reversal policy and rounding need independent
domain rules. Reversal timeout creates another unknown state that can coexist
with previously confirmed capture.

## Operational limits

Reconciliation queues, provider rate limits, outbox lag, and manual escalation
can exceed objectives. Bounded workers prevent system collapse but leave older
unknowns waiting. Monitoring must surface age and business exposure, not merely
error count. Secrets and regulated data in provider evidence require strict
minimization and access.

## Final statement

The hybrid design prevents many local sequence errors and records distributed
truth more honestly. It does not turn payment processing into one atomic local
transaction. External authority, provider behavior, settlement, compensation,
and permanently missing evidence remain explicit runtime and operational
responsibilities.
