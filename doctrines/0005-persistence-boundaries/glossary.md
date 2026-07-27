# Glossary

**Ambiguous commit**
: A commit attempt whose client-side result does not establish whether the
database made the transaction durable.

**Data migration**
: A controlled transformation of stored values, including the evidence and
invariant meaning attached to them.

**Durable intent**
: A persisted record that an external action should be attempted, written so
process failure cannot silently forget the obligation.

**Expand-and-contract**
: A rollout sequence that first adds a representation compatible with old code,
migrates readers and writers, then removes the old representation.

**Historical invalid data**
: A stored representation that exists but cannot establish current trusted
domain invariants.

**Inbox**
: Durable consumer-side recording used to recognize and coordinate repeated
message delivery.

**Lost update**
: An anomaly in which concurrent operations derive writes from overlapping
state and one committed write silently replaces or erases another.

**Optimistic concurrency**
: A strategy that performs an update only if a previously observed version or
predicate remains current.

**Outbox**
: A durable publication-intent record written in the same local transaction as
the associated domain transition.

**Persistence model**
: A representation designed for storage shape, compatibility, and physical
decoding, not automatically a trusted domain entity.

**Phantom**
: A concurrency phenomenon in which repeating a predicate query observes a
different qualifying row set because another transaction committed inserts,
deletes, or updates. Exact prevention semantics are product-specific.

**Quarantine**
: Isolation of invalid stored evidence with identity and diagnostics so it can
be audited and repaired without entering trusted operations.

**Schema constraint**
: A database-enforced predicate such as nullability, uniqueness, referential
integrity, or a check expression.

**Serializable**
: An isolation contract under which committed transaction effects are
equivalent to some serial execution. It can require abort and retry and does not
by itself establish real-time ordering, external-effect atomicity, or future
liveness.

**Serialization anomaly**
: A committed result that cannot be explained by any serial ordering of the
participating transactions.

**Snapshot isolation**
: An isolation model in which a transaction reads from a consistent snapshot
and concurrent write-write conflicts are rejected under the product's rules.
It can still permit write skew when transactions update disjoint rows.

**Storage discriminator**
: A stable encoded value selecting one variant or lifecycle state in a durable
representation.

**Write skew**
: A serialization anomaly in which transactions read a shared predicate or
invariant, update disjoint rows, and both commit because no direct write-write
conflict exposes the combined violation.

## Isolation-analysis map

This map supports RUST-DOC-0005-R009 analysis; it is not a substitute for the
selected database's primary documentation.

| Mechanism or level                   | Typical protection                                                            | Residual analysis still required                                                    |
| ------------------------------------ | ----------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| per-row version or compare-and-set   | detects a lost update on the guarded row or predicate                         | disjoint-row write skew, phantoms, unguarded alternate writers                      |
| snapshot isolation                   | stable transaction snapshot and product-defined write-write conflict handling | write skew and other serialization anomalies                                        |
| predicate or range lock              | protects the locked predicate or key range from specified interference        | complete lock scope, deadlock, alternate access paths, and product behavior         |
| serializable isolation               | rejects or blocks executions that would not have a serial explanation         | retry handling, product configuration, external effects, and real-time-order claims |
| schema constraint or atomic mutation | arbitrates the encoded predicate at the database boundary                     | invariants not encoded by that constraint or mutation                               |

PostgreSQL currently implements its Repeatable Read level using snapshot
isolation: it prevents the phantom reads described by its documentation but
still permits serialization anomalies such as write skew. PostgreSQL
Serializable adds detection and may abort a transaction, so applications must
retry the complete transaction. Other products can assign different guarantees
to similarly named levels.
