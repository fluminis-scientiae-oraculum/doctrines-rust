---
id: RUST-DOC-0005
slug: persistence-boundaries
title: Persistence Boundaries and Domain Integrity
status: active
version: 0.1.0
normative: true
applies_to:
  - planning
  - implementation
  - review
  - audit
  - maintenance
risk_domains:
  - persistence
  - migrations
  - transactions
  - serialization
supersedes: []
superseded_by: null
---

# Persistence Boundaries and Domain Integrity

## Scope

This package governs data crossing between persistent representations and
trusted Rust domain types. It covers relational rows, document records,
key-value entries, event payloads, snapshots, migration code, transaction
boundaries, optimistic concurrency, and durable intent for later external
effects.

Persisted bytes are evidence that some writer stored a representation under
some historical rules. They are not automatic evidence that current domain
invariants hold. Database drivers can decode valid SQL values that are invalid
domain values. Old binaries, manual repair, incomplete migrations, relaxed
constraints, replication, and corruption can all produce records a trusted type
must reject.

## Out of scope

This doctrine does not prescribe a database product, object-relational mapper,
or event-sourcing architecture. It does not claim that schema constraints alone
prove application behavior. Distributed delivery and unknown external outcomes
are governed more fully by RUST-DOC-0006. SQL isolation and durability claims
must follow the chosen database's documented configuration and observed
behavior.

## Intended readers

- planners defining storage/domain separation and transaction scope;
- implementers writing row conversions, repositories, and migrations;
- reviewers tracing every decoding and update path;
- auditors searching for forged trusted values and lost updates;
- maintainers evolving schemas and persisted enums.

## Normative status

[`doctrine.md`](doctrine.md) is normative. Rules use stable identifiers.
Rationale and examples clarify application but do not silently create new
requirements. Waivers require scope, evidence, owner, and expiry.

## Prerequisite foundations

Read invariant classification, trust boundaries, evidence levels, guarantee
honesty, and complexity budget under [`../../foundations/`](../../foundations/).
Persistence work particularly depends on the distinction between a parsed
representation, a policy-accepted value, a persisted fact, and a fact that
remains true in mutable external reality.

## Related material

- Patterns: opaque newtypes, smart constructors, sum types, validated
  collections, hybrid state machines, and explicit uncertainty.
- Boundaries: database decoding, Serde, messaging, configuration, and
  filesystems.
- Reviews: domain model, boundary, distributed effects, and final audit.
- Case studies: database transaction, payment lifecycle, invoice, and message
  delivery.

## Reading order

Read normative rules, then rationale. Use the decision framework when designing
storage shape, migration, or transaction scope. Apply the review gates to every
read and write path. Use the anti-pattern catalogue for adversarial bypass
search.

## Compact doctrine summary

Storage models and domain models should be distinct when their invariants or
evolution pressures differ. Every path from storage to a trusted type needs to
validate current invariants. Schema constraints reinforce but do not replace
domain construction. Transactions protect only operations within their actual
boundary and isolation semantics. Version checks prevent silent lost updates.
Persistence plus messaging requires durable coordination such as an outbox or
an explicit reconciliation design. Historical invalid data is quarantined or
migrated; it is never forged into a trusted value for convenience.
