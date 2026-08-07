# Database transaction: remaining uncertainty

## Protocol and product semantics

The application can classify commit only as precisely as the driver, database
protocol, and topology permit. Connection loss timing observed by a client may
not reveal server processing. Failover, replication, synchronous-commit
configuration, and proxy behavior change durability and visibility. These
premises belong in the deployed guarantee ledger and must be rechecked after
upgrade or topology change.

An operation record inside the transaction often makes reconciliation practical,
but absence can remain ambiguous during replica lag, failover, or an incomplete
lookup. Query the authoritative node or use a documented consistency level.

## Isolation

Optimistic versioning prevents silent overwrite of one row. It does not prevent
every predicate anomaly, write skew, phantom, or cross-system race. The domain
invariant determines whether a constraint, explicit row/range lock,
serializable isolation, commutative update, or later reconciliation is needed.

Serializable implementations can abort transactions and require bounded retry.
Retry must reuse logical operation identity and respect the caller deadline.
Long transactions increase lock and version-conflict pressure.

## Rollback and cleanup

Consuming `rollback(self)` prevents local handle reuse and can confirm the
driver-reported outcome. Drop-based cleanup is best effort according to the
library and connection state. A process crash cannot execute Rust destructors.
The database normally cleans abandoned sessions, but timing and lock release are
external observations.

Explicit rollback can itself fail. The connection may need quarantine from the
pool. The service should monitor active duration, rollback failures, pool
discard, and lock wait without claiming metrics prove no leak.

## External effects

The outbox closes one gap: confirmed database commit contains durable intent.
It does not guarantee publication time, unique delivery, consumer success, or
external side-effect uniqueness. Publisher acknowledgement can be lost.
Consumers need identity and durable handling. Dead-letter and replay procedures
must preserve it.

If the transaction follows an already completed external payment or file
operation, local rollback cannot undo it. The workflow needs external operation
identity and reconciliation, possibly compensation. Calling the database
transaction complete does not make the whole business transaction atomic.

## Administrative and historical paths

Direct SQL, restores, migrations, and privileged repair can bypass repository
types and operation identities. Access controls, constraints, migration checks,
and audit reduce the risk but cannot make alternate writers impossible.
Invalid-history quarantine preserves honesty at the cost of partial
availability.

Caches and read replicas introduce another evidence age. A caller that reads an
old version after confirmed commit may incorrectly infer the transaction was
lost. Read-your-writes routing, version-aware cache invalidation, or waiting for
a documented replication position can improve the observation. None makes
every later read globally current; responses must preserve version and source
semantics where decisions depend on freshness.

## Final statement

The improved design proves useful local lifecycle and database-scoped atomicity.
It explicitly refuses to turn client I/O failure into remote rollback evidence.
Isolation anomalies, failover visibility, process crash cleanup, message
delivery, and prior external effects remain bounded runtime and operational
problems.

> [!TIP]
> [problem](problem.md) · [naive design](naive.md) · [improved design](improved.md) · **remaining uncertainty**
> Index: [all case studies](../README.md).
> Executable mechanics live under [`../../examples/`](../../examples/).
