# Message delivery: remaining uncertainty

## Deduplication horizon

The inbox suppresses repeats only while identity remains retained and reachable.
Broker replay, backup restoration, or an operator import after pruning can
repeat an old effect. Retention must cover known replay horizons and the
business cost of duplication. Permanent retention has storage and privacy cost,
so expiry behavior needs explicit risk acceptance.

A unique inbox row also assumes all cooperating consumers use the same
database/namespace. Another deployment, tenant-scope bug, or direct side-effect
caller can bypass it.

## Broker evidence

Broker acknowledgement and offset semantics depend on product, client library,
configuration, transaction feature, and failover. The consumer protocol must be
verified against these specifics. At-least-once delivery can still lose messages
through producer behavior, retention expiry, administrative deletion, or an
incorrect acknowledgement policy.

Partition ordering does not create global order. Repartitioning and producer
retries may change observed sequences. Aggregate versions provide application
evidence but missing versions can require a separate authoritative source.

## External notification

The outbox ensures local intent, not provider execution. Provider idempotency
may expire or be scoped differently. An accepted response does not prove end-user
delivery. Unknown notification may remain unresolved. Support may authorize a
repeat with acknowledged duplicate risk; that action is new intent.

## Poison and compatibility

Quarantine prevents one bad message from hot-looping but moves work into an
operational queue. It can grow, expose sensitive payloads, and become forgotten.
Age, volume, access, and resolution need monitoring. Repair can change
semantics, so provenance and authorization matter.

Ignoring unknown event types helps forward compatibility only when omission is
safe. An unknown command cannot generally be ignored without losing requested
work. Compatibility tooling validates schemas, not every semantic assumption.

## Concurrency and shutdown

Bounded concurrency prevents unbounded resource growth but can delay processing.
Database contention, downstream slowdown, and retry storms can saturate limits.
Backpressure may increase broker lag until retention is threatened. Capacity
plans and alerts are operational evidence, not compile-time guarantees.

Shutdown after database commit but before acknowledgement is safe only because
inbox replay handling remains available. Shutdown during external publication
leaves durable intent or unknown state. Forced termination can still postpone
cleanup.

## Final statement

The improved consumer provides a defensible one-time local mutation within one
database and retention boundary. It explicitly retains at-least-once delivery,
scoped ordering, external-effect ambiguity, compatibility, retention, and
operational backlog as remaining system responsibilities.
