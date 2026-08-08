---
id: <Replace with RUST-DOC-NNNN>
slug: <Replace with lowercase-hyphenated-slug>
title: <Replace with doctrine title>
status: draft
version: 0.1.0
normative: true
applies_to:
  - <Replace with planning, implementation, review, or audit>
risk_domains:
  - <Replace with risk domain>
supersedes: []
superseded_by: null
---

# <Replace with doctrine title>

This directory is a reusable eight-file doctrine package. Text enclosed in
angle brackets is a template instruction and must be replaced when copied.
Remove this paragraph from the resulting doctrine.

## Scope

<Define the systems, APIs, risks, and change classes governed by the doctrine.
Name concrete boundaries and explain why they require shared rules.>

## Out of scope

<State adjacent concerns the doctrine deliberately does not govern. Link the
canonical doctrine or foundation that owns them.>

## Intended readers

<List relevant planner, implementer, reviewer, auditor, maintainer, and human
engineering roles.>

## Normative status

State that [`doctrine.md`](doctrine.md) is normative, identify version/status, and explain
waiver boundaries. Do not let rationale create hidden requirements.

## Prerequisite foundations

<Link only foundations necessary to interpret the rules, including normative
language, invariant classification, guarantee honesty, and complexity budget as
appropriate.>

## Related material

<Link related patterns, boundary guides, review procedures, case studies,
doctrines, and executable examples.>

## Reading order

<Give a short operational order through the eight files.>

## Compact doctrine summary

<Summarize the normative posture, exact guarantee discipline, boundary
preservation, evidence expectation, and central non-guarantee without copying
every rule.>

## Package completion check

- metadata agrees with [`manifest/doctrines.yaml`](../../manifest/doctrines.yaml) and its JSON Schema;
- rule IDs use `RUST-DOC-NNNN-RNNN`;
- all eight files contain domain-specific substance;
- references and source notes distinguish external facts from repository
  governance;
- relevant examples and agent selections are linked;
- generated bundles reproduce after manifest updates.
