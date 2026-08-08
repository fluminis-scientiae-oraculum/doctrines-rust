---
id: RUST-DOC-0003
slug: ownership-and-capabilities
title: Ownership as Authority and Lifecycle
status: active
version: 0.2.0
normative: true
applies_to:
  - planning
  - implementation
  - review
  - audit
risk_domains:
  - authority
  - resource-lifecycle
  - secrets
  - concurrency
supersedes: []
superseded_by: null
---

# Ownership as Authority and Lifecycle

## Scope

Ownership can express more than memory management. This doctrine governs its use for exclusive
authority, resource custody, lifecycle completion, single use, transfer, borrowing, secrets,
and inter-task handoff. It covers capabilities, transaction guards, tokens, leases, file
locks, shutdown permits, and locally owned resources.

## Out of scope

It does not claim Rust ownership is authorization by itself, make external rollback infallible,
or require a capability for every method. It does not replace synchronization, distributed
lease protocols, operating-system access control, or secret-management systems.

## Readers, status, and prerequisites

Planners map authority and lifecycle. Implementers design issuance and transfer. Reviewers
inspect clones, borrows, interior mutability, destruction, and revocation. Auditors search for
forged or leaked authority. `doctrine.md` is normative.

Read foundations on [invariants](../../foundations/invariants.md),
[evidence](../../foundations/evidence.md), [boundaries](../../foundations/trust-boundaries.md),
[guarantee honesty](../../foundations/guarantee-honesty.md), and
[complexity](../../foundations/complexity-budget.md). Related material:
[capability types](../../patterns/capability-types.md),
[consuming transitions](../../patterns/consuming-transitions.md),
[typestate](../../patterns/typestate.md), [filesystem](../../boundaries/filesystem.md) and
[FFI](../../boundaries/ffi.md) guides,
[concurrency doctrine](../0004-concurrency-and-async/), and
[authenticated-session](../../case-studies/authenticated-session/) and
[transaction](../../case-studies/database-transaction/) case studies.

## Summary

- Use ownership to model exclusive custody when the domain is exclusive.
- Borrow only the authority needed and for no longer than required.
- Restrict capability issuance and operation surface.
- Specify clone, transfer, serialization, expiry, revocation, and destruction.
- Use RAII for local cleanup; model external rollback or compensation as fallible.
- Keep secrets hard to format, clone, serialize, and retain.
- Do not claim complete zeroization without accounting for copies, allocators, compiler
  behavior, and external storage.
- Do not default to `Arc<Mutex<T>>`; define ownership and synchronization first.
- Use lifetimes only for real borrowing relationships.

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
