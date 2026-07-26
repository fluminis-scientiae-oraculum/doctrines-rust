---
id: RUST-DOC-0001
slug: invalid-states
title: Making Invalid States Unrepresentable
status: active
version: 0.1.0
normative: true
applies_to:
  - planning
  - implementation
  - review
  - audit
risk_domains:
  - domain-modeling
  - state-machines
  - trust-boundaries
  - distributed-effects
supersedes: []
superseded_by: null
---

# Making Invalid States Unrepresentable

## Scope

This doctrine governs discovery, classification, representation, construction, transition,
decoding, and review of consequential invariants in Rust systems. It covers mutually exclusive
domain state, refined values, collection rules, locally controlled protocols, authority,
persistence, external effects, and distributed uncertainty.

Its core question is not “Can Rust encode this in a type?” The question is “Which invalid
programs are consequential, which facts are structurally enforceable, where must runtime
validation remain, and what evidence supports the resulting claim?”

## Out of scope

This package does not specify a universal domain architecture, require typestate, define
complete email or monetary policy, guarantee external service behavior, or replace specialist
doctrines for errors, concurrency, persistence, distributed systems, unsafe code, testing, and
performance. It does not assert that every rule should become a type.

## Intended readers

Planners use this package before selecting structs or state markers. Implementers use it to
protect construction and transitions. Reviewers trace every bypass and distinguish local proof
from external observation. Auditors use it to challenge evidence-inaccurate names and certainty.
Architects use it with the complexity budget to choose proportionate mechanisms.

## Normative status

`doctrine.md` is normative. Requirements use the interpretation in
`foundations/normative-language.md` and stable IDs beginning `RUST-DOC-0001-R`. Rationale,
examples, anti-patterns, glossary, and references are informative unless a rule incorporates
them.

All rules are version `0.1.0` and active. Waivers follow the repository waiver contract. A new
escape hatch, weakened obligation, or changed rule meaning requires an RFC.

## Prerequisite foundations

Read, in order:

1. `foundations/normative-language.md`;
2. `foundations/invariants.md`;
3. `foundations/evidence.md`;
4. `foundations/trust-boundaries.md`;
5. `foundations/guarantee-honesty.md`;
6. `foundations/complexity-budget.md`.

## Related material

Related patterns are sum types, opaque newtypes, smart constructors, validated collections,
consuming transitions, typestate, capability types, hybrid state machines, and explicit
uncertainty. Primary boundary guides are Serde, database decoding, HTTP/RPC, and messaging.
Operational reviews are domain-model, boundary, typestate, distributed-effects, and final
correctness review.

Executable examples live under `examples/domain-modeling`, `examples/validated-newtypes`,
`examples/typestate`, `examples/boundary-validation`, `examples/distributed-outcomes`, and
`examples/compile-fail`. Case studies apply the doctrine to invoices, payments, transactions,
message delivery, authenticated sessions, and UI workflows.

## Reading order

Read this file, `doctrine.md`, `rationale.md`, and `decision-framework.md` before design.
Implement against applicable pattern and boundary guides. Use `review-standard.md` during
review, then inspect `anti-patterns.md` for bypass shapes. Consult the glossary and references
when terms or source authority matter.

## Compact summary

- Discover and inventory invariants before choosing representation.
- Use enums for mutually exclusive dynamic state, opaque newtypes for stable local value
  invariants, validated wrappers for collection invariants, capabilities for authority, and
  runtime services or transactions for cross-entity facts.
- Use typestate or consuming transitions only for proportionate, locally controlled sequencing.
- Keep trusted fields private and make smart constructors complete.
- Preserve validation through Serde, databases, caches, migrations, FFI, and every alternate
  constructor.
- Name types for evidence actually established.
- Keep network, storage, and other external effects fallible.
- Do not convert an ambiguous timeout into confirmed failure.
- Represent unknown outcomes with reconciliation identity.
- Test accepted and rejected construction, important forbidden programs, boundaries, and
  distributed failure.
- Publish guarantees beside non-guarantees and residual uncertainty.

The desired result is not the greatest amount of type machinery. It is a legible system in
which consequential invalid states and transitions are hard or impossible to express, runtime
truth remains validated, and external uncertainty is reported honestly.
