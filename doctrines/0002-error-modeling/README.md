---
id: RUST-DOC-0002
slug: error-modeling
title: Error Modeling as Domain Design
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
  - api-design
  - failure-semantics
  - operations
  - security
supersedes: []
superseded_by: null
---

# Error Modeling as Domain Design

## Scope

This doctrine governs errors exposed by Rust libraries, applications, protocols, boundaries,
and operations. It treats failure as domain evidence: callers may need to distinguish
validation, rejection, conflict, cancellation, timeout, local resource failure, and an
indeterminate external effect.

It covers structured public errors, application reports, source chains, context, retry and
recovery guidance, panic boundaries, conversion, redaction, observability, and compatibility.

## Out of scope

The package does not prescribe one error crate, require a public variant for every internal
cause, or promise that typed errors make recovery possible. It does not replace distributed
outcome modeling, security response policy, or protocol specifications.

## Readers and status

Planners define the failure vocabulary before APIs. Implementers preserve actionable
categories and sources. Reviewers trace conversion and retry. Auditors search for hidden
indeterminacy, panic on external input, secret leakage, and category erasure. `doctrine.md` is
normative under `foundations/normative-language.md`; other package files explain and
operationalize it.

## Prerequisites and related material

Read invariant, evidence, trust-boundary, and guarantee-honesty foundations. Related doctrines
are 0001, 0004, 0005, 0006, and 0008. Related guides include sum types, explicit uncertainty,
HTTP/RPC, messaging, database decoding, and distributed-effects review.

## Reading order and summary

Read normative rules, rationale, decision framework, review standard, anti-patterns, glossary,
and references. Core obligations:

- model operationally distinct failures as distinct structured cases;
- preserve source errors and machine-actionable context;
- state recoverability and retryability rather than infer them from transport labels;
- preserve cancellation, timeout, rejection, and unknown outcome;
- use panic only for violated internal assumptions or unrecoverable programmer faults;
- justify production `unwrap` and `expect`;
- redact secrets at every recipient boundary;
- and treat public error shape as compatibility surface.
