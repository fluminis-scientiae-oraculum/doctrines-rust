---
id: RUST-DOC-0008
slug: testing-and-evidence
title: Testing as Layered Evidence
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
  - testing
  - verification
  - concurrency
  - distributed-systems
supersedes: []
superseded_by: null
---

# Testing as Layered Evidence

## Scope

This package governs how tests and related verification evidence support Rust
engineering claims. It covers compiler rejection, type checking, unit tests,
property tests, compile-fail tests, integration and contract tests, concurrency
testing, fault injection, model checking, unsafe-code tools, coverage,
snapshots, benchmarks, production telemetry, and incident evidence.

Tests are observations under defined conditions. Different classes expose
different failures and leave different blind spots. A constructor rejection
test supports a value invariant; it does not prove all integrations use the
constructor. A compile-fail test supports an API prohibition; it does not prove
runtime network behavior. A production metric observes deployed workloads; it
does not automatically reveal silent corruption.

## Out of scope

This doctrine does not mandate one testing framework or coverage percentage. It
does not treat benchmarks as correctness tests or passing tools as a universal
proof. Performance measurement belongs to RUST-DOC-0009, while unsafe-specific
proof obligations belong to RUST-DOC-0007.

## Intended readers

- planners producing an invariant-linked evidence plan;
- implementers creating positive, negative, and fault-path tests;
- reviewers assessing whether evidence matches claims;
- auditors locating untested bypasses and suppressed failures;
- maintainers evolving fixtures, snapshots, and compiler diagnostics.

## Normative status

[`doctrine.md`](doctrine.md) is normative. Rules require proportionate evidence,
not mechanical use of every test class. Waivers state the uncovered risk,
alternative evidence, owner, and review date.

## Prerequisite foundations

Read [`../../foundations/evidence.md`](../../foundations/evidence.md),
[`../../foundations/invariants.md`](../../foundations/invariants.md), and
[`../../foundations/guarantee-honesty.md`](../../foundations/guarantee-honesty.md).
Evidence strength depends on the claim, scope, environment, and completeness of
the observed set.

## Related material

- Patterns: every pattern's testing-evidence section.
- Boundaries: each boundary guide's positive and adversarial cases.
- Reviews: all operational checklists, especially final correctness audit.
- Case studies: guarantee ledgers connect design claims to evidence.

## Reading order

Read the normative rules, then the hierarchy in the rationale. Use the decision
framework to map invariants and failure risks to test classes. Apply the review
standard before updating snapshots, compiler diagnostics, or flaky-test policy.

## Compact doctrine summary

Every consequential test traces to an invariant or failure risk. Constructor
tests include rejection. Property tests cover generative or algebraic spaces
where examples are weak. Compile-fail tests preserve important prohibited
programs. Integration and contract tests cross real boundaries where feasible.
Concurrency evidence controls schedules; distributed evidence injects duplicate,
delay, reordering, partial failure, and unknown outcomes. Test doubles preserve
critical failure semantics. Snapshots receive semantic review. Flakiness is
evidence of uncontrolled behavior, not noise to retry away. Coverage supports
gap discovery but does not replace invariant coverage.

## Executable evidence status

The 0.1.0 workspace includes positive and negative unit tests, checked boundary
conversion, deterministic generator tests, and compiler-rejection cases through
`trybuild`. It does not include property-based generation, fault injection,
schedule exploration, contract testing against a deployed service, Miri, or
production telemetry. Those classes remain conditional tools whose value
depends on the claim; the existing suite establishes only the behaviors it
executes.
