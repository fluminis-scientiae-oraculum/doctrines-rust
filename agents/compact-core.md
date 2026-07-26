# Compact doctrine core

## Thesis

Rust quality requires discovering important invariants, encoding those that are
structurally enforceable, constraining legal states and transitions, preserving
validation at trust boundaries, modeling external failure honestly,
representing distributed uncertainty explicitly, and keeping type complexity
proportional to risk removed.

## Invariant classification

Classify value, state, transition, authority, lifecycle, boundary, cross-entity,
temporal, environmental, and distributed invariants. Record statement, scope,
owner, enforcement, boundary, evidence, consequence, and residual uncertainty.
Distinguish invariant, precondition, postcondition, policy, assumption,
observation, and desired outcome.

## Boundary pipeline

```text
raw input → parse → structural value → validate → trusted domain value
          → execute fallible effect → observe/reconcile
          → confirmed evidence or explicit uncertainty
```

Validation moves to protected construction and boundaries; it never disappears.

## Mechanism selection

Use an enum for mutually exclusive state, opaque newtype for one stable local
invariant, validated wrapper for a collection invariant, consuming transition
or typestate for a small locally controlled sequence, capability for authority,
runtime enum/service for dynamic or persisted state, and runtime validation for
external/cross-entity facts. Use plain code when machinery costs more than the
risk it removes.

## Guarantee honesty

For each claim record establishment, construction protection, boundary
preservation, escape hatches, non-proofs, mutable external facts, runtime
failures, and indeterminate outcomes. Type names describe evidence, not
aspiration.

## Core audit

- Can any constructor, decoder, row mapper, migration, or feature forge trusted
  state?
- Can mutation invalidate a wrapper?
- Can authority be cloned, serialized, or used after revocation?
- Does local typestate claim remote liveness?
- Does timeout become rejection?
- Does retry reuse one operation identity and satisfy idempotency?
- Are duplicates, order scope, acknowledgement loss, and reconciliation
  explicit?
- Are unsafe obligations complete?
- Do tests cover prohibited and partial-failure paths?
- Are performance claims measured under a defined workload?
