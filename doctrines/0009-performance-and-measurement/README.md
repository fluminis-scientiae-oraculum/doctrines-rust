---
id: RUST-DOC-0009
slug: performance-and-measurement
title: Performance Claims Require Measurement
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
  - performance
  - capacity
  - latency
  - resource-cost
supersedes: []
superseded_by: null
---

# Performance Claims Require Measurement

## Scope

This package governs performance objectives, measurements, optimizations, and
claims about Rust software. It covers workload definition, benchmark design,
profiling, latency distributions, throughput, CPU and wall-clock time,
allocations, cache locality, contention, batching, backpressure, serialization,
system calls, database and network limits, binary size, monomorphization, and
compile time.

Rust enables low-level control and strong optimization, but compilation does not
establish speed. A result is meaningful only relative to an environment, input
distribution, concurrency level, system state, measurement method, and
correctness contract. Faster output that violates an invariant is not an
optimization.

## Out of scope

This doctrine does not select one benchmark framework, profiler, allocator, or
runtime. It does not prescribe optimization of code without an objective. It
does not treat a microbenchmark as end-to-end evidence or use unsafe code as a
default performance technique.

## Intended readers

- planners defining objectives and workloads;
- implementers profiling and changing hot paths;
- reviewers checking methodology and correctness preservation;
- auditors challenging broad or irreproducible claims;
- maintainers controlling regressions, build cost, and environment drift.

## Normative status

[`doctrine.md`](doctrine.md) is normative. A performance statement in code,
documentation, review, or release notes is a claim subject to these rules.
Waivers may accept an unmeasured low-risk cleanup, but cannot convert intuition
into a measured claim.

## Prerequisite foundations

Read complexity budget, guarantee honesty, invariants, and evidence under
[`../../foundations/`](../../foundations/). Also apply RUST-DOC-0004 to
concurrency, RUST-DOC-0007 to unsafe optimization, and RUST-DOC-0008 to the
difference between benchmark and correctness evidence.

## Related material

- Patterns: validated collections, opaque newtypes, typestate, and hybrid state
  machines all have runtime and compile-time costs.
- Boundaries: serialization, database, HTTP/RPC, messaging, and filesystem.
- Reviews: pre-implementation, typestate, distributed effects, and final audit.
- Case studies: performance choices remain subordinate to each guarantee
  ledger.

## Reading order

Read the rules, then the rationale. Use the decision framework to design
measurement before changing code. Apply the review standard to benchmark
artifacts and claims. Use anti-patterns to detect attractive but unsupported
stories.

## Compact doctrine summary

Optimization begins with a defined objective and representative workload.
Claims name environment, inputs, system state, sample method, and uncertainty.
Profile before optimizing. Benchmarks defend against dead-code elimination,
setup contamination, unstable machines, and invalid comparisons. Report
latency distributions, not only averages. Measure allocation, copies, syscalls,
contention, and size rather than inferring them. Async concurrency is not
parallel speedup. Zero-copy claims identify exactly which copies are removed and
which lifetime or retention costs are introduced. Regression thresholds are
automated only for sufficiently stable signals. All changes preserve invariants.
