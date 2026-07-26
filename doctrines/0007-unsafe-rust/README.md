---
id: RUST-DOC-0007
slug: unsafe-rust
title: Unsafe Rust as a Proof Obligation
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
  - memory-safety
  - ffi
  - concurrency
  - dependency-risk
supersedes: []
superseded_by: null
---

# Unsafe Rust as a Proof Obligation

## Scope

This package governs Rust `unsafe` blocks and functions, unsafe traits and
implementations, raw pointers, foreign-function interfaces, manual allocation,
layout-dependent code, uninitialized memory, and safe APIs whose implementation
contains unsafe operations. It also governs decisions to depend on crates whose
unsafe internals materially affect the system's risk.

`unsafe` does not disable Rust's safety contract. It identifies operations for
which the compiler cannot verify all preconditions and transfers the missing
proof to authors and reviewers. A sound safe API must remain memory-safe for
every safe caller, including adversarial call sequences and panics.

## Out of scope

This doctrine is not a collection of clever pointer techniques. It does not
teach exploit development or promise that passing Miri proves universal
soundness. Platform ABI, allocator, provenance, and concurrency claims must be
grounded in the relevant primary specification and deployed target.

## Intended readers

- planners deciding whether unsafe code is justified;
- implementers isolating and documenting proof obligations;
- reviewers checking safety arguments line by line;
- auditors inventorying unsafe code and dependencies;
- maintainers responding to compiler, platform, or dependency changes.

## Normative status

[`doctrine.md`](doctrine.md) is normative. Safety requirements cannot be waived
merely for performance or borrow-checker convenience. Any accepted exception
must still preserve Rust's safety contract and document evidence.

## Prerequisite foundations

Read invariants, evidence, trust boundaries, guarantee honesty, and complexity
budget under [`../../foundations/`](../../foundations/). Unsafe work additionally
requires the Rust Reference and Rustonomicon material cited in
[`references.md`](references.md).

## Related material

- Patterns: opaque newtypes, capability types, and consuming transitions.
- Boundaries: FFI and filesystem.
- Reviews: pre-implementation, boundary, domain model, and final audit.
- Doctrines: ownership/capabilities, concurrency/async, testing/evidence, and
  performance/measurement.

## Reading order

Read the normative rules before authoring a safety comment. Use the decision
framework to challenge whether unsafe is needed. Apply every relevant review
gate, then use the anti-pattern catalogue as an adversarial second pass.

## Compact doctrine summary

Every unsafe operation MUST have a safety invariant and a local argument showing
why all required preconditions hold. Unsafe surface is minimized and
encapsulated. A safe public API must be sound for all safe callers. `unsafe fn`
documents caller obligations; `unsafe impl Send` or `Sync` includes a concurrency
proof. FFI defines ABI, representation, ownership, nullability, lifetime,
threading, allocator, and unwind behavior. Partial initialization accounts for
drop. Layout and provenance are never guessed. Miri, sanitizers, model checking,
fuzzing, and target testing provide complementary evidence but do not replace
reasoning.
