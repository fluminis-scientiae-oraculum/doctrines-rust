---
id: RUST-DOC-0011
slug: executable-narrative
title: Executable Narrative and Minimal Decision Records
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
  - architecture-governance
  - documentation-drift
  - decision-records
  - agent-context
supersedes: []
superseded_by: null
---

# Executable Narrative and Minimal Decision Records

## Scope

An architectural obligation that a mechanism can enforce belongs in that mechanism. This doctrine
governs where an obligation lives, which artifact settles which class of claim, how a derived
view is kept from drifting, and when a manually maintained decision record earns its permanent
cost.

Its distinctive concern is the second copy. A type, a schema, a manifest, or a test changes when
the system changes and fails when it is contradicted. A hand-maintained description of the same
obligation changes only when someone remembers, survives the constraint that produced it, and is
still discoverable when a reader treats it as current authority. The doctrine names the artifact
that is authoritative for each class of claim, prohibits a competing editable copy, prefers
generation to synchronization, and makes a decision record the exception rather than the default
artifact.

## Out of scope

It does not decide which invariants a domain has, which belongs to RUST-DOC-0001, nor how errors
are modeled, which belongs to RUST-DOC-0002. It does not define custody or authority, which
belong to RUST-DOC-0003, or cancellation, which belongs to RUST-DOC-0004. It does not govern
durable decoding, migration, or transactions, which belong to RUST-DOC-0005, nor distributed
ambiguity, which belongs to RUST-DOC-0006. It states no soundness obligation, which belongs to
RUST-DOC-0007. It does not define evidence classes or their strength, which belong to
RUST-DOC-0008, and it makes no cost claim, which belongs to RUST-DOC-0009. It does not design a
staged protocol, which belongs to RUST-DOC-0010; it supplies the authority partition that
doctrine's `RUST-DOC-0010-R022` applies to stages.

It does not claim that documentation is worthless, that code explains an external constraint, or
that a system with no decision records has no unrecorded constraints.

## Intended readers

Planners assess whether an obligation can be enforced before deciding how to describe it, and
propose a decision record only after that assessment fails. Implementers encode the obligation,
add the negative evidence, generate the derived views, and link the rare record to the artifacts
that remain authoritative. Reviewers reject an unnecessary record, find the competing copy, and
test whether an unenforced part of a claim was stated or assumed. Auditors enumerate the active
records, find those whose reason has ended, and find historical records being cited as current
authority. Maintainers archive what expired, revalidate what survived, and keep generated context
free of architecture archaeology.

## Normative status

`doctrine.md` is normative and carries the stable rule identifiers. This package is version
0.1.0 with status active. Rationale, decision framework, anti-patterns, glossary, and references
are informative and cannot create an obligation that `doctrine.md` does not state.

Rules `RUST-DOC-0011-R002`, `RUST-DOC-0011-R005`, `RUST-DOC-0011-R015`, `RUST-DOC-0011-R016`,
and `RUST-DOC-0011-R017` permit a waiver on the terms recorded in the normative waiver section.
The rules governing claim classification, operational authority, competing copies, record
necessity and lifecycle, rationale honesty, external claims, agent hydration, and exception terms
do not.

## Prerequisite foundations

Read [normative language](../../foundations/normative-language.md) for requirement levels and
waiver structure, [evidence](../../foundations/evidence.md) for what each artifact class
establishes, [guarantee honesty](../../foundations/guarantee-honesty.md) for the discipline that
separates a claim from its limits,
[complexity budget](../../foundations/complexity-budget.md) for the assessment
`RUST-DOC-0011-R002` requires before an obligation is left prose-carried, and
[invariants](../../foundations/invariants.md) for classifying the obligations being placed.

## Related material

Patterns: [executable narrative](../../patterns/executable-narrative.md) is the mechanism this
doctrine governs; [successor capabilities](../../patterns/successor-capabilities.md),
[typestate](../../patterns/typestate.md), and
[opaque newtypes](../../patterns/opaque-newtypes.md) are three of the mechanisms an obligation
can move into.

Boundaries: [database decoding](../../boundaries/database-decoding.md) and
[Serde](../../boundaries/serde.md) carry obligations across persistence and wire boundaries, and
[configuration](../../boundaries/configuration.md) carries operational policy.

Reviews: [executable narrative review](../../reviews/executable-narrative-review.md) is the
procedure for this doctrine, and
[final correctness audit](../../reviews/final-correctness-audit.md) aggregates it. Case studies:
[registration onboarding](../../case-studies/registration-onboarding/) shows an obligation
carried by types rather than by a record, and
[payment lifecycle](../../case-studies/payment-lifecycle/) shows the durable and external claims
this doctrine keeps outside the executable authority.

Decision records, their template, and the worked examples live under
[`decisions/`](../../decisions/README.md); the active set is enumerated in
[`manifest/decision-records.yaml`](../../manifest/decision-records.yaml) and validated by
`doctrine-lint`.

## Reading order

Start with this file for scope, then `doctrine.md` for the obligations and the authority
partition. Read `rationale.md` for the failure modes this doctrine answers, then
`decision-framework.md` before writing either a mechanism or a record. Use `review-standard.md`
during review, `anti-patterns.md` when a proposed document feels close to a known failure,
`glossary.md` for terms whose local meaning is narrower than ordinary usage, and `references.md`
for provenance.

## Compact doctrine summary

Classify a claim before citing an authority for it. Put an enforceable obligation in the
mechanism that enforces it, and treat that mechanism as authoritative for what it enforces. State
the part it does not enforce rather than letting the enforced part imply it. Keep no second
manually maintained copy of an enforced claim; generate a derived view, declare its source, and
check it for drift. Name the external system authoritative for every durable or remote fact.

Write a decision record only for the residue that no artifact can carry, and then only with an
owner, a revalidation trigger, an obsolescence condition, and links to the artifacts that remain
authoritative for current behavior. Retire a record when its reason ends, and confirm a record
still applies before citing it against a change. Record rationale that cannot be recovered, and
record an absent rationale as unknown rather than inferring one.

The central non-guarantee: moving an obligation into a mechanism proves the obligation is now
enforced, not that it is the right obligation. A generated view is current, not correct. An empty
decision-record set is evidence about the record set, not about the constraints a system is
under.

## Package completion check

- metadata agrees with `manifest/doctrines.yaml` and its JSON Schema;
- rule IDs use `RUST-DOC-0011-RNNN` and every one appears in `review-standard.md`;
- all eight files carry domain-specific substance;
- references and source notes separate external facts from repository governance, and record the
  originating claim accurately;
- the decision-record registry, its schema, and the linter checks are linked;
- generated bundles reproduce after the manifest update.

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
