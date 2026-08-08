---
id: RUST-DOC-0004
slug: concurrency-and-async
title: Concurrency and Async Correctness
status: active
version: 0.1.2
normative: true
applies_to:
  - planning
  - implementation
  - review
  - audit
risk_domains:
  - concurrency
  - async
  - cancellation
  - resource-management
supersedes: []
superseded_by: null
---

# Concurrency and Async Correctness

## Scope

This package governs Rust code in which work overlaps in time, execution can be
interleaved, or progress depends on synchronization. It applies to operating
system threads, asynchronous tasks, channels, locks, atomics, actor-like
components, background workers, and shutdown coordination. It also applies when
otherwise sequential code calls a concurrent dependency whose cancellation,
ordering, or retry behavior affects correctness.

The doctrine treats concurrency as a protocol-design problem. Rust can prevent
many memory races, but `Send`, `Sync`, successful compilation, and use of an
async runtime do not establish freedom from deadlock, starvation, lost wakeups,
unbounded queues, retry amplification, incomplete cancellation, or incorrect
ordering assumptions.

## Out of scope

This package does not prescribe one async runtime, one channel implementation,
or one locking primitive. It does not define distributed idempotency in full;
that belongs to RUST-DOC-0006. It does not replace performance measurement under
RUST-DOC-0009, unsafe-code review under RUST-DOC-0007, or boundary validation
under RUST-DOC-0005. Runtime-specific guarantees remain those of the selected
runtime and version.

## Intended readers

- planners defining task ownership, capacity, cancellation, and shutdown;
- implementers writing concurrent or asynchronous components;
- reviewers tracing interleavings, lock scope, and overload behavior;
- auditors searching for detached work, unbounded growth, and hidden retries;
- maintainers changing runtimes, channel types, or supervision structure.

## Normative status

[`doctrine.md`](doctrine.md) is normative. Rule identifiers remain stable within
the doctrine version. Rationale, examples, and decision aids explain intended
application but become normative only when a rule incorporates them. An
approved, scoped waiver may document an exception; convenience is not evidence
for one.

## Prerequisite foundations

Read these documents before applying the rules:

1. [`../../foundations/invariants.md`](../../foundations/invariants.md) for
   lifecycle, temporal, and authority invariants;
2. [`../../foundations/trust-boundaries.md`](../../foundations/trust-boundaries.md)
   for external observations and effects;
3. [`../../foundations/guarantee-honesty.md`](../../foundations/guarantee-honesty.md)
   for exact claims about local and external state;
4. [`../../foundations/complexity-budget.md`](../../foundations/complexity-budget.md)
   for synchronization and abstraction costs;
5. [`../../foundations/evidence.md`](../../foundations/evidence.md) for the
   evidence represented by handles, guards, and acknowledgements.

## Related material

- Patterns: [capability types](../../patterns/capability-types.md),
  [consuming transitions](../../patterns/consuming-transitions.md),
  [hybrid state machines](../../patterns/hybrid-state-machines.md), and
  [explicit uncertainty](../../patterns/explicit-uncertainty.md).
- Boundaries: messaging, [HTTP/RPC](../../boundaries/http-and-rpc.md),
  [database decoding](../../boundaries/database-decoding.md), filesystems, and FFI.
- Reviews: [pre-implementation](../../reviews/pre-implementation.md),
  [distributed-effects](../../reviews/distributed-effects-review.md), and
  [final correctness](../../reviews/final-correctness-audit.md) audit.
- Case studies: [message delivery](../../case-studies/message-delivery/), payment lifecycle,
  database transaction, and authenticated session.

## Reading order

Read the normative rules first, then the rationale. Use the decision framework
to select an ownership and coordination model. Apply the review standard before
merge. Use the anti-pattern catalogue during adversarial review, then consult
the glossary and primary references for disputed terminology.

## Compact doctrine summary

Every concurrent component needs a defined owner for mutable state, tasks, queues,
and shutdown. Capacity and backpressure are deliberate. Each suspension
point inside a partial operation requires cancellation analysis. Blocking work
must be isolated from executor workers. Lock ordering, channel closure, task
failure, retry layering, and external ordering claims require explicit
contracts. Detached work is exceptional and observable. Atomics require an
ordering argument tied to a synchronization invariant. Graceful shutdown means
bounded, observable completion behavior; it does not mean every external effect
can be rolled back.

## Executable evidence status

The example workspace demonstrates ownership-consuming transitions, a fallible
connection protocol, and compiler rejection of sending through a locally closed
connection. It does not include an async-runtime integration, cancellation
harness, deadlock detector, Loom model, or backpressure load test. Systems
applying this doctrine supply evidence for those runtime-specific claims; the
current examples do not establish them.

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
