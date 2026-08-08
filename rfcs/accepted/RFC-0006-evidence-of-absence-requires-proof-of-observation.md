---
id: RFC-0006
title: Evidence of absence requires proof of observation
author: doctrines-rust maintainers
status: accepted
created: 2026-08-08
affected_doctrines:
  - RUST-DOC-0008
---

# RFC-0006: Evidence of absence requires proof of observation

## Summary

An assertion that expects nothing — an empty collection, a zero count, an unset value, an
uncalled double — passes identically whether the condition was searched for and absent or the
search matched nothing. RUST-DOC-0008 governs twenty-one aspects of evidence and does not govern
that one.

This adds `RUST-DOC-0008-R022`, requiring a runtime absence assertion to establish that its
predicate can observe the condition. The corpus grows from 207 normative rules to 208.
RUST-DOC-0008 moves to `0.2.0` and the repository to `0.7.0`.

## Motivation

The compile-fail instance of this principle is already covered. `RUST-DOC-0008-R005` requires
inspecting whether a prohibited program still fails for the intended reason, and gate `T18` names
the exact failure — "missing import causes pass". That is the same defect, and the corpus treats
it as critical.

The runtime instance is not covered anywhere. Sweeping all of RUST-DOC-0008 for vacuity and
positive-control vocabulary returns one adjacent item in the anti-pattern catalogue — "absence of
incidents is cited as proof a protocol is correct" — which is about production telemetry rather
than about a check whose predicate selected nothing.

The gap is not hypothetical. Four independent occurrences are on record:

- A catalog test asserting that no generated type carried an operator class passed for weeks with
  a pattern matching zero types. It would have passed identically with the invariant violated on
  all 178. Its sibling, expecting 178, failed loudly with the same defect; only the pairing
  exposed it.
- A `doctrine-lint` check in this repository passed its first positive control because its file
  list omitted the target file.
- `check_path_references`, added in the change immediately before this one, reported the
  repository valid while examining almost nothing: a callout marker consumed the scanned prose,
  and separately every root-relative mention failed to resolve. Two independent defects, one
  silent result.
- A rule whose applicability named a class its statement could not reach was applicable and
  vacuous, discovered only when something cited it.

Three of the four are in this repository. The principle is not novel practice being invented; the
three remedies the rule names were all already in use somewhere. What is missing is the
obligation to use one.

## Proposed change

One rule, one anti-pattern entry, one review gate, and executable evidence.

### The rule

`RUST-DOC-0008-R022` — Prove the observer looked before accepting absence.

The rule requires an absence assertion to establish that its predicate can observe the condition,
through a self-validating predicate that fails when its subject is missing, a positive control
asserted alongside it, or a paired assertion whose expected count is non-zero. The normative
wording lives in `doctrine.md`; this proposal does not restate it as an independent contract.

Applicability is deliberately bounded to runtime assertions whose expected result is empty, in
tests and in checks that gate a build. It does not reach compile-fail evidence, which `R005` and
`T18` already govern, and it does not reach production telemetry, which the existing anti-pattern
covers.

The stated exception is the one case where a control is redundant: a test that observes the
condition present and then removes it has already proved the observation, and the transition is
the evidence.

### Why the applicability is narrow

A rule whose applicability names a class its statement cannot reach is applicable and vacuous —
which would be an unusually poor failure for this rule in particular. "Absence" is restricted to
an expected-empty runtime result rather than to any negative claim, so every case the rule reaches
is a case the three named remedies can actually serve.

## Affected artifacts

`doctrines/0008-testing-and-evidence/doctrine.md`, its `anti-patterns.md` and `review-standard.md`,
the package front matter and `manifest/doctrines.yaml` version, `EVIDENCE.md` counts, `examples/`
evidence, `CHANGELOG.md`, the workspace version with its versioned internal path dependencies, and
every regenerated distribution.

No other doctrine changes. No rule identifier, applicability, exception, or review evidence
outside RUST-DOC-0008 changes.

## Guarantee ledger impact

| Claim                                                            | Before                                | After                                               |
| ---------------------------------------------------------------- | ------------------------------------- | --------------------------------------------------- |
| A compile-fail case must fail for the intended reason            | established, by `R005` and gate `T18` | unchanged                                           |
| A runtime absence assertion must prove its predicate can observe | not established                       | established, by `R022` and gate `T61`               |
| The corpus carries executable evidence of the failure mode       | not established                       | established, by the test trio in the examples crate |
| Every absence assertion in this repository satisfies the rule    | not established                       | still not established, and not claimed              |

The last row is the honest limit. The rule creates a review obligation and ships a demonstration;
it does not audit the existing suite, and no mechanical check finds an absence assertion whose
control is missing. Making that mechanical is the natural follow-up and is deliberately not
attempted here.

## Compatibility

A new normative rule is an added obligation, so RUST-DOC-0008 takes a minor version and the
repository follows. `foundations/normative-language.md` reserves minor releases for exactly this.
No existing rule is amended, so no consumer of a rule identifier is affected, and nothing that
passed review before now fails it retroactively except where a reviewer applies the new gate.

## Migration

None required for existing evidence. `T61` applies at the next review of a suite containing an
absence assertion.

## Alternatives

**Fold it into `R005`.** `R005` is about committed compiler diagnostics and their `.stderr`
snapshots. Widening it to runtime assertions would give one rule two enforcement stories and blunt
the compile-fail obligation that is currently sharp. Rejected.

**State it as an anti-pattern only.** The catalogue is informative. An anti-pattern entry
describes a shape to avoid; it creates no review gate and no obligation, and the defect's whole
character is that nothing signals it. Rejected, though the entry is added as well.

**Make it mechanical instead of normative.** A lint that finds absence assertions lacking controls
would be stronger than a rule. It is also not writable in general — recognising "this assertion
expects emptiness" across arbitrary test code is undecidable in the cases that matter. Recorded as
the follow-up above rather than as a substitute.

## Security impact

Neutral. The rule adds no trust boundary and no escape hatch. It slightly favours honesty in
security-relevant checks, since "no forbidden pattern found" is exactly the shape of assertion
this defect hides in.

## Complexity impact

One rule, one catalogue entry, one gate, and three tests. No tooling, no new dependency, no
generated-output shape change. The review surface grows by one gate.

## Evidence plan

- Three tests in the examples inventory crate, run against one registry that violates the
  invariant: the vacuous form passing on it, a positive control separating a blind predicate from
  a seeing one, and a non-zero pair naming the violation the vacuous form missed;
- gate `T61` in the RUST-DOC-0008 review standard;
- the anti-pattern entry, so the shape is catalogued where reviewers look for shapes.

## Source provenance

No external material. Derived from four recorded incidents, three of them in this repository, and
from the observation that the corpus already governs the compile-fail half of the same principle.

## Decision record

- Decision: accepted
- Date: 2026-08-08
- Decision owners: doctrines-rust maintainers
- Rationale: The corpus governs the compile-fail instance of "prove the observer looked" through
  `R005` and gate `T18`, and does not govern the runtime instance at all. Four recorded incidents,
  three of them here, show the runtime form passing while examining nothing. The three remedies
  were already in use; the obligation to use one was missing.
- Conditions: Bound applicability to runtime assertions whose expected result is empty, so the
  rule cannot itself be applicable and vacuous. Leave compile-fail evidence to `R005` and `T18`
  and production telemetry to the existing anti-pattern. Ship executable evidence with the rule
  rather than a description of it. State plainly that the existing suite is not audited by this
  change and that no mechanical check enforces the rule. Move RUST-DOC-0008 to `0.2.0` and the
  repository to `0.7.0`, and no other doctrine version.
- Supersedes / superseded by: none. RFC-0001 through RFC-0005 are unaffected.
