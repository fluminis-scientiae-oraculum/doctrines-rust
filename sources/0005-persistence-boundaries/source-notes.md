# RUST-DOC-0005 source notes

## Primary sources

The standard-library [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)
contract provides the fallible conversion mechanism used for raw-row to domain
mapping. Serde's
[`try_from` container attribute](https://serde.rs/container-attrs.html#try_from)
supports checked deserialization through an intermediate representation.

PostgreSQL documentation for
[constraints](https://www.postgresql.org/docs/current/ddl-constraints.html),
[transaction isolation](https://www.postgresql.org/docs/current/transaction-iso.html),
[explicit locking](https://www.postgresql.org/docs/current/explicit-locking.html),
and [enumerated types](https://www.postgresql.org/docs/current/datatype-enum.html)
provides concrete database mechanics. The doctrine does not generalize those
product-specific details to every database.

## Accepted ideas

Schema constraints, transactions, locks, and isolation can reinforce domain
invariants and arbitrate concurrent writers. Fallible conversions can keep
physical storage models separate from trusted domain models. Stable versions
and enum encodings support durable evolution.

Unique constraints are stronger concurrency arbiters than application-only
"check then insert." Optimistic version predicates make lost updates visible.
Transactions can atomically combine local rows inside their actual resource and
configured guarantees.

## Refined ideas

"The database is trusted" is refined: the database may be operationally trusted
while each row remains untrusted as current domain evidence. Old binaries,
imports, repairs, restores, and migrations can produce physical values that
violate current invariants. Drivers establish SQL decoding, not domain truth.

"Use a transaction" is refined into a mapping from a specific invariant/anomaly
to constraints, query shape, locking, version checks, and isolation. Transaction
syntax alone does not prove serializable business behavior.

Persistence success does not include a message, payment, or email outside the
database. A transactional outbox establishes durable intent with the domain
commit; it does not establish unique delivery.

## Rejected ideas

The doctrine rejects unchecked ORM construction of trusted newtypes, silent
defaults for invalid history, source-variant spelling as an accidental permanent
protocol, blind last-write-wins, skipped invalid rows, and commit errors all
mapped to rollback. It rejects migration defaults that invent verification or
authorization evidence.

## Repository additions

The repository adds invariant-aware migration records, quarantine, row truth
tables, boundary resource limits, raw/domain model decision criteria, ambiguous
commit outcomes, outbox/inbox review, durability/configuration guarantee
ledgers, and fifty operational persistence gates.

## Source-to-rule application

RUST-DOC-0005 uses `TryFrom` as the ordinary checked bridge but does not require
one ORM or driver. Constraint and isolation rules require the product mechanism
that matches the invariant. Enum and version rules recognize durable encodings
as protocols rather than source implementation details. Migration rules add
complete precondition/postcondition evidence so shape changes cannot invent
domain facts.

Outbox guidance is deliberately scoped: one local transaction can couple a
domain write and publication intent. Publisher acknowledgement, broker
delivery, consumer effect, and external calls remain later fallible boundaries.
Commit ambiguity likewise depends on driver/protocol evidence and cannot be
classified universally from a Rust I/O error.

## Maintenance triggers

Replace PostgreSQL examples with authoritative sources for the selected
database and verify configured isolation/durability. Recheck driver error and
transaction behavior after upgrades. Revisit compatibility whenever enum tags,
column nullability, format versions, alternate writers, backup/restore, or
deployment ordering changes. New direct SQL or administrative tools enter the
construction-path audit.
