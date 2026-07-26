---
id: RUST-DOC-0003
slug: ownership-and-capabilities
title: Ownership as Authority and Lifecycle
status: active
version: 0.1.0
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

Read foundations on invariants, evidence, boundaries, guarantee honesty, and complexity.
Related material: capability types, consuming transitions, typestate, filesystem and FFI
guides, concurrency doctrine, and authenticated-session and transaction case studies.

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
