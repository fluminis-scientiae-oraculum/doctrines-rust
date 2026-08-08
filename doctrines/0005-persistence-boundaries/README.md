---
id: RUST-DOC-0005
slug: persistence-boundaries
title: Persistence Boundaries and Domain Integrity
status: active
version: 0.2.1
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

- Patterns: [opaque newtypes](../../patterns/opaque-newtypes.md),
  [smart constructors](../../patterns/smart-constructors.md),
  [sum types](../../patterns/sum-types.md),
  [validated collections](../../patterns/validated-collections.md),
  [hybrid state machines](../../patterns/hybrid-state-machines.md), and
  [explicit uncertainty](../../patterns/explicit-uncertainty.md).
- Boundaries: [database decoding](../../boundaries/database-decoding.md),
  [Serde](../../boundaries/serde.md), messaging, configuration, and filesystems.
- Reviews: domain model, boundary, distributed effects, and final audit.
- Case studies: database transaction, payment lifecycle, invoice, and message delivery.

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
boundary and isolation semantics. Concurrency designs name the anomaly they
prevent and any anomaly they still permit; a per-row version check can prevent
lost updates while leaving cross-row write skew possible. Persistence plus
messaging requires durable coordination such as an outbox or an explicit
reconciliation design. Historical invalid data is quarantined or migrated; it
is never forged into a trusted value for convenience.

## Package contents

| File                                             | What it carries                                               |
| ------------------------------------------------ | ------------------------------------------------------------- |
| [`doctrine.md`](doctrine.md)                     | the normative rules, under stable rule identifiers            |
| [`rationale.md`](rationale.md)                   | why the rules take this shape, and what was rejected          |
| [`decision-framework.md`](decision-framework.md) | the operational path from a problem to a chosen mechanism     |
| [`review-standard.md`](review-standard.md)       | the auditable checks a reviewer records against this doctrine |
| [`anti-patterns.md`](anti-patterns.md)           | the bypass shapes this doctrine exists to catch               |
| [`glossary.md`](glossary.md)                     | terms as this package uses them                               |
| [`references.md`](references.md)                 | external sources, with the scope each one actually supports   |
