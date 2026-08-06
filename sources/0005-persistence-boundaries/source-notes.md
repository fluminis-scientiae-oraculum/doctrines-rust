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

[Berenson et al., "A Critique of ANSI SQL Isolation
Levels"](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/tr-95-51.pdf)
provides the snapshot-isolation and write-skew taxonomy. PostgreSQL's current
documentation supplies the product-specific qualification: its Repeatable Read
level uses snapshot isolation and can admit serialization anomalies, while
Serializable can abort transactions that would produce them.

## Accepted ideas

Schema constraints, transactions, locks, and isolation can reinforce domain
invariants and arbitrate concurrent writers. Fallible conversions can keep
physical storage models separate from trusted domain models. Stable versions
and enum encodings support durable evolution.

Unique constraints are stronger concurrency arbiters than application-only
"check then insert." Optimistic version predicates make lost updates visible.
Transactions can atomically combine local rows inside their actual resource and
configured guarantees.

Per-row version predicates are accepted only for the anomaly they actually
control. They do not arbitrate disjoint-row write skew unless the shared
invariant is encoded in the compared predicate or another mechanism.

## Refined ideas

"The database is trusted" is refined: the database may be operationally trusted
while each row remains untrusted as current domain evidence. Old binaries,
imports, repairs, restores, and migrations can produce physical values that
violate current invariants. Drivers establish SQL decoding, not domain truth.

"Use a transaction" is refined into a mapping from a specific
invariant/anomaly to constraints, query shape, locking, version checks, and
isolation. Transaction syntax alone does not prove serializable business
behavior. The mapping now names both prevented and residual anomalies against a
defined, product-qualified taxonomy.

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
ledgers, and fifty-one operational persistence gates.

## Source-to-rule application

RUST-DOC-0005 uses `TryFrom` as the ordinary checked bridge but does not require
one ORM or driver. Constraint and isolation rules require the product mechanism
that matches the invariant. R009 additionally requires the controlled and
residual anomaly sets so a lost-update mechanism cannot be presented as
write-skew protection. Enum and version rules recognize durable encodings as
protocols rather than source implementation details. Migration rules add
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
transaction behavior after upgrades, including whether named levels prevent
phantoms, write skew, and other serialization anomalies. Revisit compatibility
whenever enum tags, column nullability, format versions, alternate writers,
backup/restore, or deployment ordering changes. New direct SQL or
administrative tools enter the construction-path audit.

> [!TIP]
> [attribution](attribution.md) · **source notes**
> Index: [all source packages](../README.md).
> Doctrine: [`doctrines/0005-persistence-boundaries/`](../../doctrines/0005-persistence-boundaries/README.md).
