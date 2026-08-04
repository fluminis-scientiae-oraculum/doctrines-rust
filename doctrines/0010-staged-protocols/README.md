---
id: RUST-DOC-0010
slug: staged-protocols
title: Staged Protocols and Successor Capabilities
status: active
version: 0.1.0
normative: true
applies_to:
  - planning
  - implementation
  - review
  - audit
risk_domains:
  - protocol-design
  - state-machines
  - api-design
  - persistence
supersedes: []
superseded_by: null
---

# Staged Protocols and Successor Capabilities

## Scope

A staged protocol is an in-process sequence in which each stage establishes a fact that later
stages depend on: canonicalize, then check, then authorize, then prepare, then hand off. This
doctrine governs how such a protocol is represented so that its ordering is enforced rather than
remembered.

Its distinctive concern is the protocol edge. A stage capability names its legal successor as an
associated type bounded by the capability that successor must satisfy. The edge therefore lives
in the contract, and a stage that stops leading anywhere legal fails to compile. The doctrine
also governs branch and recovery edges, per-stage failure identity, stage granularity, effect
disclosure, the boundary at which the protocol may be erased, and the point at which a local
transition stops being evidence of a durable one.

## Out of scope

It does not govern value validation or newtype construction, which belong to RUST-DOC-0001. It
does not design an error taxonomy, which belongs to RUST-DOC-0002. It does not define custody,
capability issuance, or revocation, which belong to RUST-DOC-0003. It does not define
cancellation mechanics, which belong to RUST-DOC-0004. It does not govern durable decoding,
migration, or transactions, which belong to RUST-DOC-0005, nor distributed ambiguity and
reconciliation, which belong to RUST-DOC-0006. It states no soundness obligation, which belongs
to RUST-DOC-0007, defines no evidence class, which belongs to RUST-DOC-0008, and makes no cost
claim, which belongs to RUST-DOC-0009.

It does not claim that every ordered sequence deserves a stage type, and it does not make a
typed protocol a substitute for a durable workflow engine.

## Intended readers

Planners inventory stages, edges, evidence, and effects before types exist. Implementers build
the capability traits, the successor bounds, and the topology assertion. Reviewers test whether
the documented graph is the compiled graph and whether each stage claims only what its
construction establishes. Auditors search for conversion bypasses, forged stage evidence, and
local transitions presented as durable ones. Maintainers keep stage identity stable across
versions.

## Normative status

`doctrine.md` is normative and carries the stable rule identifiers. This package is version
0.1.0 with status active. Rationale, decision framework, anti-patterns, glossary, and references
are informative and cannot create an obligation that `doctrine.md` does not state.

Rules `RUST-DOC-0010-R012`, `RUST-DOC-0010-R016`, and `RUST-DOC-0010-R019` permit a waiver on
the terms recorded in the normative waiver section. The rules governing successor bounds,
construction bypass, durable claims, the guarantee ledger, terminology, and governance
precedence do not.

## Prerequisite foundations

Read [normative language](../../foundations/normative-language.md) for requirement levels and
waiver structure, [invariants](../../foundations/invariants.md) for classifying the facts stages
prove, [evidence](../../foundations/evidence.md) for what each evidence class establishes,
[guarantee honesty](../../foundations/guarantee-honesty.md) for the ledger discipline used
throughout, and [complexity budget](../../foundations/complexity-budget.md) for the granularity
assessment required by `RUST-DOC-0010-R012`.

## Related material

Patterns: [successor capabilities](../../patterns/successor-capabilities.md) is the mechanism
this doctrine governs; [typestate](../../patterns/typestate.md) and
[consuming transitions](../../patterns/consuming-transitions.md) are its foundation;
[sum types](../../patterns/sum-types.md) carry its branches; and
[hybrid state machines](../../patterns/hybrid-state-machines.md) carry the durable half that
`RUST-DOC-0010-R015` requires.

Boundaries: [HTTP and RPC](../../boundaries/http-and-rpc.md) for the untrusted input that enters
the first stage, [database decoding](../../boundaries/database-decoding.md) for restoration, and
[messaging](../../boundaries/messaging.md) for published effects.

Reviews: [typestate review](../../reviews/typestate-review.md) covers proportionality and adds a
staged-protocol gate group. Case studies:
[registration onboarding](../../case-studies/registration-onboarding/) is the worked protocol;
[payment lifecycle](../../case-studies/payment-lifecycle/) and
[authenticated session](../../case-studies/authenticated-session/) show the durable and authority
halves this doctrine defers.

Executable evidence lives in [`examples/staged-protocol`](../../examples/staged-protocol/src/lib.rs)
with compiler-rejection cases under [`examples/compile-fail/ui/`](../../examples/compile-fail/ui/).

## Reading order

Start with this file for scope, then `doctrine.md` for the obligations. Read `rationale.md` for
the failure modes and the guarantee ledger, then `decision-framework.md` before committing to a
stage graph. Use `review-standard.md` during review, `anti-patterns.md` when a design feels
close to one of the known failures, `glossary.md` for terms whose local meaning is narrower than
ordinary usage, and `references.md` for provenance.

## Compact doctrine summary

Inventory the protocol before typing it. Name each stage for the fact it proves. Put the legal
successor in the contract as an associated type bounded by the next capability, and never widen
that bound to make an implementation compile. Consume the stage on transition, carry forward
exactly the evidence successors need, and keep each failure identifiable by stage. Model
branches as named alternatives over distinct successors, and name every retry and recovery edge.
Allow no conversion, derive, or public constructor that produces a later stage without its
transition, and restrict and inventory the trusted paths that remain. Disclose durable and
external effects per stage. Erase the protocol only at a named boundary.

The central non-guarantee: reaching a later stage proves that the in-process protocol ran in
order, and nothing more. A move consumes a local value; stored facts are read, copied, and
replayed, so no local move consumes them. Durable advancement re-checks identity, stored state,
and a concurrency token, and persisted lifecycle stays a runtime model.

## Package completion check

- metadata agrees with `manifest/doctrines.yaml` and its JSON Schema;
- rule IDs use `RUST-DOC-0010-RNNN` and every one appears in `review-standard.md`;
- all eight files carry domain-specific substance;
- references and source notes separate external facts from repository governance, and record
  which vocabulary is local;
- the example crate, its topology assertion, and the compiler-rejection cases are linked;
- generated bundles reproduce after the manifest update.
