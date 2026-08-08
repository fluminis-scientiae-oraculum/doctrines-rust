---
id: RFC-0007
title: Every rule names what enforces it, and every gate says who decides it
author: doctrines-rust maintainers
status: accepted
created: 2026-08-08
affected_doctrines:
  - RUST-DOC-0001
  - RUST-DOC-0002
  - RUST-DOC-0003
  - RUST-DOC-0004
  - RUST-DOC-0005
  - RUST-DOC-0006
  - RUST-DOC-0007
  - RUST-DOC-0008
  - RUST-DOC-0009
  - RUST-DOC-0010
  - RUST-DOC-0011
---

# RFC-0007: Every rule names what enforces it, and every gate says who decides it

## Summary

`RUST-DOC-0011-R002` requires an enforceable obligation to live in its enforcing mechanism.
Applied to this corpus, nothing checked whether a rule had one: all 208 rules could lack a
golden, a lint, or an example and no gate noticed.

This adds a per-rule `**Enforcement.**` field and a per-gate `Check` column, gives the three
review standards that had no stable gate identifier one, and removes the rationale material
`RUST-DOC-0011-R012` already forbids. No rule statement, applicability, exception, or identifier
changes, and the corpus stays at 208 rules.

The headline is the split, not the mechanism:

| Population      | Total | Enforceable         | Not enforceable    |
| --------------- | ----: | ------------------- | ------------------ |
| Normative rules |   208 | 80 name an artifact | 128 state a waiver |
| Review gates    |   486 | 32 mechanical       | 454 judgment       |

Most rules are not mechanisable and most gates are judgment. The purpose of this change is to
make that countable rather than to imply otherwise.

## Motivation

Three filed issues describe one problem from three directions.

**Issue #12 — no rule is linked to what enforces it.** Measured before this change: zero of
eleven `doctrine.md` files referenced `examples/`, two of eleven package `README.md` files linked
an example crate, and one example source back-referenced a rule identifier. `doctrine-lint` ran
about twenty checks, and every one guarded the documents' internal consistency. The interlock
instinct was real but pointed inward.

**Issue #14 — review standards used three gate formats.** `RUST-DOC-0001` used `## Gate N`
headings, `0002` and `0003` used tables keyed by a prose name, and `0004` through `0011` used
coded identifiers. A gate that cannot be named cannot be cited from a waiver, a review record, or
a CI job, which blocked the `Check` column outright.

**Issue #13 — `R012`'s own prohibition was audited nowhere.** `R012` requires rationale that
cannot be reconstructed from executable artifacts, and forbids restating the operational
topology, interface, or invariant set as an independent contract. The second half had no audit.

## Proposed change

### The enforcement field lives on the rule, not in the manifest

Issue #12 proposed a manifest field. A per-rule manifest entry would copy all 208 rule
identifiers into a second maintained file, which is exactly the competing copy
`RUST-DOC-0011-R004` prohibits. The issue's proposed design is refused by the issue's own
doctrine, so the field sits in `doctrine.md` beside the rule it describes.

A value either names artifacts, each of which must exist, or opens with `Unenforceable:` and
states why. `doctrine-lint` rejects a rule carrying neither.

### The waiver is the normal outcome, not an oversight marker

Sixty-two per cent of rules carry a waiver. That is not a backlog. `RUST-DOC-0009` is 0 of 20
because the repository ships no benchmark, and `RUST-DOC-0004` is 1 of 20 because it ships no
async runtime. Many rules are design obligations no artifact can carry — state what a claim does
not prove, publish guarantees beside non-guarantees.

`RUST-DOC-0008-R022` is itself a waiver. It is the best-resourced rule in the corpus, shipping
three executable tests, and no mechanism decides whether an arbitrary absence assertion carries a
control. A design that marked it as a gap would be measuring the wrong thing.

### Gate identifiers, and whether renaming one is normative

Issue #14 left this open deliberately. It is settled here: **a gate identifier is part of the
review contract**, because the issue's own problem statement is that a gate must be citable from
a waiver, a review record, or a CI job. Renaming one therefore breaks citations.

Nothing in this corpus cites a gate identifier outside its own review standard, so nothing here
breaks. External citations cannot be checked from inside the repository, so `RUST-DOC-0001`,
`0002`, and `0003` take a minor version rather than a patch. The other eight take a patch: they
gain a column and a field, and no identifier moves.

Prefixes follow the existing convention. `0001` becomes `I01` through `I18`, `0002` becomes `F01`
through `F18`, and `0003` becomes `O01` through `O18`. Each package's heading title or prose name
carried subject information its question omits — `Authority` names what `Are issuance, scope,
clone, transfer, serialization, expiry, revocation, and use count defined?` is about — so it is
folded into the question as a bold lead-in rather than dropped.

### The rationale audit removes one class and protects the other

Forty sections across eleven files restated rules, types, or manifests. `rationale.md` falls from
1,787 lines to 1,383. Material recording a rejected alternative, an accepted risk, or a
constraint no artifact holds was excluded by the audit and stays. That is the class `R012`
protects, and the originating external review's proposal to delete all 1,787 lines would have
removed precisely it.

Per-file outcomes, so a later reader knows the audit happened rather than guessing it was
skipped:

| Package                            | Sections removed | Lines removed |
| ---------------------------------- | ---------------: | ------------: |
| `0001-invalid-states`              |                5 |           138 |
| `0002-error-modeling`              |                1 |            15 |
| `0003-ownership-and-capabilities`  |                3 |            29 |
| `0004-concurrency-and-async`       |                4 |            52 |
| `0005-persistence-boundaries`      |                4 |            15 |
| `0006-distributed-uncertainty`     |                6 |            48 |
| `0007-unsafe-rust`                 |                5 |            33 |
| `0008-testing-and-evidence`        |                3 |            22 |
| `0009-performance-and-measurement` |                6 |            19 |
| `0010-staged-protocols`            |                1 |            13 |
| `0011-executable-narrative`        |                2 |            20 |

### The tooling is held to the corpus it enforces

Three findings in this repository's own Rust, all fixed here.

`check_rule_enforcement` first parsed its field by walking lines while a boolean remembered
whether one was open. That makes "inside a field" and "no field read yet" separately
representable and able to contradict each other, which is the shape gate `I02` rejects. It now
splits a document into rule sections and reads a field from a section, so the state cannot be
inconsistent.

`bundle-agent-context` reported stale bundles and an unreadable tree as the same string and the
same exit code, so CI could not distinguish "run generate" from "the tool broke". A typed
`CommandFailure` now separates them, with distinct exit codes and a test that matches the variant
rather than the message.

Note what was deliberately not done. `Result<_, String>` appears nineteen times in that binary,
and gate `F02` names that type as its failure example. But `RUST-DOC-0002-R002` applies to
reusable crates and module boundaries with multiple operational outcomes, and those are private
functions in a binary with no library target whose sole caller responds identically to every
failure. Refactoring them in `R002`'s name would apply a rule to a class its statement excludes,
which is the failure RFC-0006 warned about.

Two corpus-wide tests asserted absence with no positive control, so an empty file list would have
satisfied them while examining nothing. Both now assert a non-zero observation first.

## Affected artifacts

Every `doctrine.md`, `review-standard.md`, and `rationale.md`; `tools/doctrine-lint` and
`tools/bundle-agent-context`; `manifest/doctrines.yaml`; each package `README.md` front matter;
`EVIDENCE.md`; `CHANGELOG.md`; the workspace version with its versioned internal path
dependencies; and every regenerated distribution.

## Guarantee ledger impact

| Claim                                                       | Before          | After                                           |
| ----------------------------------------------------------- | --------------- | ----------------------------------------------- |
| Every rule names an enforcement artifact or a stated waiver | not established | established, over all 208 rules                 |
| A named artifact exists in the repository                   | not established | established, by path and link-target resolution |
| Every gate declares whether a command decides it            | not established | established, over all 486 gates                 |
| Every gate can be cited by a stable identifier              | false for 54    | established for all 486                         |
| Rationale restates no rule, type, or manifest               | not established | established by audit, not by a check            |
| A stated waiver reason is true                              | not established | **still not established, and not claimed**      |
| A declared command actually decides its gate                | not established | **still not established, and not claimed**      |

The last two rows are the honest limit. Nothing verifies that a waiver's reason is accurate, only
that one was written. Nothing runs a gate's declared command to confirm it decides that gate;
all nine distinct commands were checked by hand against `rust-toolchain.toml`, the pinned Miri
nightly, and the real package names, and none of that is mechanised. Making both mechanical is
the natural follow-up and is deliberately not attempted here.

## Compatibility

No rule is added, amended, or removed, so no consumer of a rule identifier is affected and
nothing that passed review before now fails it. Gate identifiers change in three packages, which
take a minor version. The repository takes a minor version because the corpus gains a maintained
field in every package.

## Migration

A review record or waiver citing a `RUST-DOC-0001`, `0002`, or `0003` gate by its old heading
number or prose name should be updated to the coded identifier. The mapping is recoverable: every
gate kept its title as the lead-in of its question, in its original order.

## Alternatives

**Put the field in the manifest, as issue #12 proposed.** Rejected: it copies 208 identifiers
into a second maintained file, which `RUST-DOC-0011-R004` prohibits.

**Annotate only rules that already have artifacts, and ratchet the rest.** Rejected. A partial
annotation cannot answer "what fraction is enforceable", which is the question that justified the
work.

**Leave `0001` in heading form and teach the checker two shapes.** Rejected: a corpus arguing
against competing representations should not keep three shapes for one artifact. The objection
that a table would read worse was tested rather than assumed — `MD013` exempts tables, and
`0011/rationale.md` already carries a thousand-character row.

**Delete `rationale.md` wholesale**, as the originating external review first proposed. Withdrawn
by that reviewer, and rejected here: it removes the protected class `R012` exists to preserve.

## Security impact

Neutral. No trust boundary and no escape hatch is added. The field makes it harder for a
security-relevant rule to sit with no artifact and no acknowledgement that it has none.

## Complexity impact

Two lint checks, one field per rule, and one column per gate table. Against that, `rationale.md`
loses 404 lines. The review surface does not grow: no gate is added, and the `Check` column
classifies gates that already existed.

## Evidence plan

- `check_rule_enforcement` and `check_gate_check_column`, each positive-controlled against a
  violation seeded in the real corpus rather than only in a fixture;
- five seeded failure modes confirmed reported and then cleared: a missing field, an artifact path
  that does not exist, a waiver with no reason, a gate declaring neither value, and a review
  standard with no `Check` column;
- a typed-variant test for `CommandFailure`, asserting the variant and its exit code rather than
  the wording of its message.

## Source provenance

No external material. Derived from issues #12, #13, and #14, themselves triaged from an external
review, and from measurement of this repository.

## Decision record

- Decision: accepted
- Date: 2026-08-08
- Decision owners: doctrines-rust maintainers
- Rationale: `RUST-DOC-0011-R002` went unenforced against the corpus that states it. Making it
  enforceable required a stable identifier for every gate first, which is why issue #14 blocked
  issue #12. The measured result — 38 per cent of rules with an artifact, 6.6 per cent of gates
  command-decidable — is the finding, and a mechanism that hid it would be worse than none.
- Conditions: Keep the field on the rule rather than in the manifest, so no second file maintains
  the rule identifiers. Treat the waiver as the normal outcome and count it. Settle that a gate
  identifier is part of the review contract, and take a minor version in the three packages whose
  identifiers change. Remove only rationale that restates rules, types, or manifests, and record
  the outcome per file. State plainly that neither a waiver's truth nor a declared command's
  adequacy is mechanically established.
- Supersedes / superseded by: none. RFC-0001 through RFC-0006 are unaffected.
