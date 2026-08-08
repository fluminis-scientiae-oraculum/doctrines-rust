---
id: RUST-DOC-0007
slug: unsafe-rust
title: Unsafe Rust as a Proof Obligation
status: active
version: 0.1.3
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

- Patterns: [opaque newtypes](../../patterns/opaque-newtypes.md),
  [capability types](../../patterns/capability-types.md), and
  [consuming transitions](../../patterns/consuming-transitions.md).
- Boundaries: FFI and filesystem.
- Reviews: [pre-implementation](../../reviews/pre-implementation.md), boundary, domain model, and
  final audit.
- Doctrines: ownership/capabilities, concurrency/async, testing/evidence, and
  performance/measurement.

## Reading order

Read the normative rules before authoring a safety comment. Use the decision
framework to challenge whether unsafe is needed. Apply every relevant review
gate, then use the anti-pattern catalogue as an adversarial second pass.

## Compact doctrine summary

Every unsafe operation needs a safety invariant and a local argument showing
why all required preconditions hold. Unsafe surface is minimized and
encapsulated. A safe public API must be sound for all safe callers. `unsafe fn`
documents caller obligations; `unsafe impl Send` or `Sync` includes a concurrency
proof. FFI defines ABI, representation, ownership, nullability, lifetime,
threading, allocator, and unwind behavior. Partial initialization accounts for
drop. Layout and provenance are never guessed. Miri, sanitizers, model checking,
fuzzing, and target testing provide complementary evidence but do not replace
reasoning.

## Executable evidence status

The workspace forbids unsafe code by default. The narrowly isolated
`unsafe-evidence` crate opts out locally to exercise a panic-safe
`MaybeUninit<[T; N]>` initializer. Its five unit tests cover success, builder
error, builder panic, an empty array, and zero-sized element drop accounting;
the dedicated CI job reruns them under Miri on a pinned nightly toolchain. The
crate documents each unsafe operation, safe-API proof, construction boundary,
and residual limits.

This evidence supports only that abstraction under the exercised interpreter
and inputs. It is not sanitizer, FFI-target, fuzzing, concurrent-unsafe, or
universal provenance evidence, and it does not replace the safety argument.

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
