---
id: RUST-DOC-0006
slug: distributed-uncertainty
title: Distributed Effects, Uncertainty, and Reconciliation
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
  - distributed-systems
  - retries
  - idempotency
  - reconciliation
supersedes: []
superseded_by: null
---

# Distributed Effects, Uncertainty, and Reconciliation

## Scope

This package governs operations crossing process, host, service, database,
broker, or administrative boundaries where communication can fail separately
from execution. It covers timeouts, lost acknowledgements, retries,
idempotency, duplicate delivery, ordering, reconciliation, compensation,
causality, and external observations that may be stale.

Distributed APIs often return less evidence than local control flow suggests.
A caller can know that it stopped waiting without knowing whether the server
executed. A consumer can process a message while its acknowledgement is lost. A
database can commit before its client connection fails. Honest models preserve
confirmed success, confirmed rejection, local pre-execution failure, and unknown
outcome as distinct states when operations require different action.

## Out of scope

This doctrine does not promise one universal delivery or consensus protocol. It
does not prescribe a broker or database. It does not use Rust types to claim
that remote state remains current. Local concurrency rules are in
RUST-DOC-0004; persistence coordination is in RUST-DOC-0005; error categories
are in RUST-DOC-0002.

## Intended readers

- planners inventorying effects, retries, and reconciliation;
- implementers building clients, consumers, workers, and operation trackers;
- reviewers checking idempotency scope and timeout semantics;
- auditors searching for collapsed uncertainty and unsafe replay;
- maintainers changing external protocols or retention.

## Normative status

[`doctrine.md`](doctrine.md) is normative. Rule identifiers are stable within
this version. Examples provide possible representations, not mandatory generic
shapes. Approved waivers must preserve operational visibility and carry a
bounded risk decision.

## Prerequisite foundations

Read trust boundaries, evidence levels, guarantee honesty, invariants, and
complexity budget under [`../../foundations/`](../../foundations/). An
externally acknowledged value and a reconciled external outcome are distinct
evidence levels.

## Related material

- Patterns: explicit uncertainty, consuming transitions, capability types, and
  hybrid state machines.
- Boundaries: HTTP/RPC, messaging, database decoding, and filesystems.
- Reviews: distributed effects, boundary, and final correctness audit.
- Case studies: payment lifecycle, message delivery, database transaction,
  invoice, and UI workflow.

## Reading order

Read the rules and rationale before assigning a binary `Result` to an external
effect. Use the decision framework to classify each failure point. Apply the
review standard to operation identity, retry, retention, and reconciliation.

## Compact doctrine summary

A timeout MUST NOT imply non-execution. Retrying an effect requires explicit
idempotency analysis. An idempotency key needs scope, uniqueness, retention,
payload-binding, and replay semantics. Outcome models distinguish confirmed
success, confirmed rejection, local failure before dispatch, and unknown
execution. Unknown states carry enough evidence for reconciliation. At-least-once
delivery means duplicates are expected. Ordering claims name their exact scope.
Compensation is a new fallible action, not rollback. Audit trails preserve
operation identity, attempts, correlation, causality, observations, and final
resolution.
