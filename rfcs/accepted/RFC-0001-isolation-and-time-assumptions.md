---
id: RFC-0001
title: Make isolation anomalies and time assumptions enforceable
author: doctrines-rust maintainers
status: accepted
created: 2026-07-27
affected_doctrines:
  - RUST-DOC-0005
  - RUST-DOC-0006
---

# RFC-0001: Make isolation anomalies and time assumptions enforceable

## Summary

RUST-DOC-0005-R009 will require a concurrency design to name the anomaly class
its mechanism prevents and every residual anomaly class it permits.
RUST-DOC-0006-R014 will require time-based authority to name its clock source,
clock kind, accepted timing bounds, and failure behavior. Existing rule IDs and
allowed exceptions remain stable.

## Motivation

Both packages already ask reviewers for this evidence, but their rule statements
do not make it enforceable. Under snapshot isolation, two transactions can read
one shared predicate, update disjoint rows, avoid a write-write conflict, and
commit a state that violates a cross-row invariant. A per-row version predicate
therefore does not establish protection against write skew.

Likewise, a lease can appear valid to a paused or skewed worker after another
coordinator has acquired authority. Fencing may reject the stale write, but a
design that depends on time must still state which clock and bounds justify its
authority decision.

## Proposed normative changes

| Doctrine/rule      | Current meaning                                                                                                                                            | Proposed meaning                                                                                                                                   | Reason                                                                                      |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| RUST-DOC-0005-R009 | Cross-entity invariants use a capable isolation mechanism or explicit coordination; anomaly analysis appears only in review evidence                       | Preserve that obligation and require named prevented and residual anomaly classes                                                                  | Make isolation claims auditable and expose write skew hidden by disjoint-row version checks |
| RUST-DOC-0006-R014 | Concurrent coordinators use ownership, fencing, compare-and-set, leadership, or effect-level idempotency; clock assumptions appear only in review evidence | Preserve those mechanisms and require clock source, clock kind, skew/pause/renewal bounds, and failure behavior when time contributes to authority | Prevent undocumented timing assumptions from becoming split-brain authority                 |

## Affected doctrine IDs and artifacts

The implementation changes RUST-DOC-0005 and RUST-DOC-0006 normative
statements, rationale, review evidence, source notes, references, and package
versions. It adds persistence anomaly terminology and one review gate. The
doctrine manifest, repository version, changelog, generated bundles, and
aggregate evidence map change with them. RUST-DOC-0004, RUST-DOC-0007,
RUST-DOC-0008, review-format documentation, and the unsafe example receive
separate non-normative evidence clarifications in the same repository release.

## Guarantee ledger impact

| Claim                                                                              | Established by                                                                                               | Protected construction                                                                  | Boundary preservation                                                               | Escape hatches                                  | Does not prove                                                                                         | Residual runtime risk                                                                                      |
| ---------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------- |
| Selected concurrency mechanism protects the named invariant from the named anomaly | invariant/anomaly mapping, database contract, query or coordination design, and competing-operation evidence | transaction, constraint, lock, compare-and-set, version predicate, or explicit protocol | configured database isolation and every alternate writer use the reviewed mechanism | documented eventual convergence under R009      | absence of unnamed anomalies, external-effect atomicity, or future configuration stability             | abort, deadlock, write skew, phantom, stale read, or product-specific anomaly outside the stated mechanism |
| Time-based authority is valid only within stated assumptions                       | clock contract, timing bounds, renewal protocol, and overlap evidence                                        | lease or coordinator API plus effect-level fence where stale writes can harm            | every protected effect validates the authority or fencing token                     | commutative duplicate-safe execution under R014 | synchronized clocks, process progress, permanent liveness, or revocation of an already accepted effect | skew, pause, partition, delayed renewal, stale owner, or unavailable clock                                 |

## Compatibility

The two normative changes alter no consuming Rust API, persisted value, wire
format, schema, rule ID, or existing allowed exception. Review records that
previously passed R009 or R014 may need additional evidence; this is a
compatible normative strengthening and advances the affected doctrines to
0.2.0 and the repository to 0.2.0. The same release adds one unpublished
example API and advances workspace crates coherently to 0.2.0. Older generated
agent bundles lack the new obligations and must be regenerated rather than
mixed with the new manifest.

## Migration

1. Add isolation definitions and the product-qualified mechanism map.
2. Strengthen R009 and add the residual-anomaly review gate.
3. Strengthen R014 and raise missing clock-contract evidence to critical.
4. Update rationale, provenance, package versions, manifest, and changelog.
5. Format canonical Markdown, regenerate bundles, inspect generated changes,
   and run the complete pinned-toolchain validation set.

Rollback before publication reverts the canonical changes and regenerates
bundles. After consumers adopt version 0.2.0, forward repair is preferred:
supply the missing anomaly or clock contract rather than silently weakening the
rules.

## Alternatives

- Keep the requirements only in review evidence. Rejected because a review
  checklist cannot create a normative obligation.
- Require serializable isolation for every cross-entity invariant. Rejected
  because constraints, locks, atomic mutations, and explicit coordination can
  provide narrower sufficient mechanisms, while product semantics differ.
- Ban leases and deadlines for authority. Rejected because fenced leases can be
  appropriate; the required contract should expose their assumptions and
  failure modes.
- Create new rule IDs. Rejected because both changes tighten the exact concern
  already owned by R009 and R014; preserving IDs keeps traceability intact.

## Security impact

The normative changes reduce integrity risk from concurrent invariant violation
and stale coordinator authority. They add no secret material, dependency, or
boundary bypass. The separate unsafe evidence crate has no dependencies and
contains its proof and lint exception locally. Timing and anomaly records must
use generic operational bounds and identifiers rather than credentials or
private production data.

## Complexity impact

Design and review gain a small terminology and evidence cost. Runtime cost
depends on the selected mechanism: stronger isolation, locks, fencing, or
retries may add contention and aborts. The RFC does not mandate a type-level
protocol or one database product. Diagnostics improve because a conflict or
residual anomaly has a named class and owner.

## Evidence plan

- Doctrine lint and schema validation check rule and manifest structure.
- Markdown format, lint, and link checks validate the canonical documentation.
- Competing-operation and fault evidence remain required in consuming systems;
  this repository does not claim a live database or distributed lease test.
- Deterministic bundle generation and drift checks prove agent projections
  contain the accepted statements.
- Workspace formatting, Clippy, tests, MSRV checks, dependency policy, and diff
  hygiene protect the accompanying executable evidence without proving the
  distributed claims.

## Source provenance

- PostgreSQL transaction-isolation documentation is accepted for its
  product-specific Repeatable Read and Serializable behavior, including
  serialization failures and retry requirements.
- Berenson et al., "A Critique of ANSI SQL Isolation Levels," is accepted for
  the snapshot-isolation and write-skew taxonomy.
- Gray and Cheriton's "Leases" is accepted for time-bounded distributed
  authority.
- The explicit prevented/residual-anomaly and clock-failure contract is a
  repository governance addition.

## Decision record

- Decision: accepted
- Date: 2026-07-27
- Decision owners: doctrines-rust maintainers
- Rationale: Existing review evidence identified correctness-critical facts
  that consuming designs could omit without violating the rule statements.
- Conditions: Preserve rule IDs and exceptions; qualify product-specific
  isolation semantics; regenerate bundles; pass the complete validation suite.
- Supersedes / superseded by: none
