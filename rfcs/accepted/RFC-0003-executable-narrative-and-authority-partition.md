---
id: RFC-0003
title: Partition architectural authority and make decision records a last resort
author: doctrines-rust maintainers
status: accepted
created: 2026-08-04
affected_doctrines:
  - RUST-DOC-0010
  - RUST-DOC-0011
---

# RFC-0003: Partition architectural authority and make decision records a last resort

## Summary

An eleventh doctrine, `RUST-DOC-0011` "Executable Narrative and Minimal Decision Records", will
govern where an architectural obligation lives, which artifact settles which class of claim, and
when a manually maintained decision record is justified. It adds twenty rules, a forty-gate
review standard, a pattern guide, a review procedure, a machine-readable decision-record
registry with linter enforcement, and worked examples of a justified and an unjustified record.

`RUST-DOC-0010-R022` keeps its identifier and is restated. Its current text asserts a blanket
governance precedence over the executable protocol. That is replaced by an explicit partition:
the executable protocol is authoritative for what it mechanically enforces, external systems are
authoritative for durable and remote facts, and governing records are authoritative for
rationale, non-guarantees, waivers, and change authority.

The repository advances to 0.4.0 and `RUST-DOC-0010` to 0.2.0. No other rule identifier,
statement, or allowed exception changes.

## Motivation

### The corrected position is neither of the two positions currently on the record

The source material absorbed by `RUST-DOC-0010` argued that an enforceable architectural
obligation should live in the mechanism that enforces it rather than survive only as prose.
`sources/0010-staged-protocols/source-notes.md` recorded that claim under "Rejected claims" as
"code as sufficient contract" and rejected its first half. The rejection answered a claim the
source did not make.

The source did not claim that code explains external constraints, accepted risk, or who may
change a contract. It claimed that an obligation a mechanism can enforce belongs in that
mechanism. Recording it as "code explains everything" and rejecting it produced the opposite
overreach: a rule stating that documentation and process hold precedence over the executable
protocol, without saying precedence over which class of claim.

Both framings share a defect. They assume one artifact must win. The accurate structure is a
partition. A question about what the program currently permits and a question about who accepted
a residual risk are different questions, and answering both from one artifact is what produces
either a decorative type system or a documentation corpus that drifts from the code it describes.

### A blanket precedence rule has a cost this corpus already pays

`RUST-DOC-0010-R019` requires an executable topology assertion precisely because prose cannot
detect a redirected associated type. `RUST-DOC-0010-R018` requires compile-fail evidence
precisely because a claimed impossibility stated in prose can silently become possible. Both
rules exist because the executable artifact is the authority for the claim, and both sit in a
package whose closing rule says the executable protocol does not settle what the system is
obliged to do. The package argues the partition and then denies it.

### Nothing in the corpus governs the second copy

The corpus has no rule against maintaining a hand-written table of a protocol's stages beside
the compiled graph, no rule requiring a derived view to be generated, and no rule about when a
decision record is worth its permanent cost. A record of the form "we chose A over B at time T
because of constraint C" stays discoverable after constraint C disappears, and a reviewer or an
agent can then cite it as authority. Improvement becomes an argument against a document that no
longer describes anything true.

The repository has already accumulated one instance. `rfcs/accepted/README.md` indexes RFC-0001
and omits RFC-0002, so the one hand-maintained index of accepted proposals was wrong within a
single release. That is the failure mode in miniature, and it argues for generation and for
fewer maintained representations rather than for more diligence.

### Local vocabulary was recorded and then made invisible

The originating document coined "Chainable Telescopic Typestate Traits", abbreviated CT³, and
disclaimed standardization itself. `RUST-DOC-0010-R021` made that caution binding, and the term
was recorded in the package glossary. It appears nowhere in the pattern guide that teaches the
mechanism, so a reader arriving from an internal document that uses the term cannot connect it to
the material that supersedes it. Recording vocabulary as local is an attribution obligation, not
a reason to hide it.

## Proposed normative changes

### Restated rule

| Doctrine/rule      | Current meaning                                                                                                                                      | Proposed meaning                                                                                                                                                                                                                                                                                                               | Reason                                                                                 |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| RUST-DOC-0010-R022 | The executable protocol is authoritative for in-process ordering and does not replace doctrine obligations, review evidence, or the decision process | Each protocol claim is classified and assigned to one authority: the executable protocol for what it mechanically enforces, external systems for durable and remote facts, governing records for rationale, non-guarantees, waivers, and change authority; no artifact is maintained as a competing source for another's class | The current text answers a claim the source did not make and contradicts R018 and R019 |

The rule keeps its identifier, its position in the package, and its status as non-waivable. Its
title becomes "Partition protocol authority explicitly".

### New rules

All twenty `RUST-DOC-0011` rules are new. None alters an existing rule.

| Rule               | Obligation                                                                                         | Reason                                                                        |
| ------------------ | -------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| RUST-DOC-0011-R001 | Classify a claim before citing an authority for it                                                 | Ends precedence arguments by making the class explicit                        |
| RUST-DOC-0011-R002 | Represent a mechanically enforceable obligation in the mechanism that enforces it                  | Keeps an enforceable rule from surviving only as prose                        |
| RUST-DOC-0011-R003 | Treat the enforcing artifact as the operational authority for what it enforces                     | Names the artifact a reader should trust for current behavior                 |
| RUST-DOC-0011-R004 | Keep no manually maintained competing copy of an enforced claim                                    | A second editable source drifts and is the cause of stale architecture        |
| RUST-DOC-0011-R005 | Generate a derived view and declare its source                                                     | Removes the synchronization obligation instead of assigning it                |
| RUST-DOC-0011-R006 | Create a decision record only for the residue that cannot live elsewhere                           | Makes the record the exception rather than the default artifact               |
| RUST-DOC-0011-R007 | State the last-resort justification, owner, revalidation, obsolescence, and executable authorities | Makes an active record auditable and removable                                |
| RUST-DOC-0011-R008 | Keep a decision record narrow and state what it does not govern                                    | Prevents a record becoming a general architecture description                 |
| RUST-DOC-0011-R009 | Expire a record whose reason has ended                                                             | Survival by inattention is how a record becomes false                         |
| RUST-DOC-0011-R010 | Confirm applicability before citing a record as a constraint                                       | Removes the silent veto a historical choice otherwise acquires                |
| RUST-DOC-0011-R011 | Retire an implemented proposal from operational authority                                          | An accepted RFC is decision history, not a current specification              |
| RUST-DOC-0011-R012 | Record only rationale that cannot be recovered from the artifacts                                  | Confines prose to what prose alone can carry                                  |
| RUST-DOC-0011-R013 | Do not invent rationale for an existing constraint                                                 | An inferred reason presented as governing rationale is a fabricated authority |
| RUST-DOC-0011-R014 | Keep an external claim outside the executable authority                                            | The partition's external leg, applied beyond one protocol                     |
| RUST-DOC-0011-R015 | Make a compatibility or migration promise executable, or state that nothing enforces it            | A promise with no mechanism is a claim, and is labelled as one                |
| RUST-DOC-0011-R016 | Keep the enforced structure readable as its domain story                                           | An illegible authority recreates the prose duplicate it replaced              |
| RUST-DOC-0011-R017 | Count and reduce the maintained representations of a claim                                         | Makes duplication a reviewable quantity rather than a matter of taste         |
| RUST-DOC-0011-R018 | Hydrate agents from current authority, excluding archived records                                  | Keeps obsolete decisions out of the context agents reason from                |
| RUST-DOC-0011-R019 | Govern a change without duplicating what it changes                                                | Preserves the RFC process without making it a second specification            |
| RUST-DOC-0011-R020 | Record the terms of a prose-only obligation or an active record                                    | Every exception carries an owner, a trigger, and a removal condition          |

## Affected doctrine IDs and artifacts

The implementation adds the `RUST-DOC-0011` package with its eight files, source notes and
attribution under `sources/0011-executable-narrative/`, the pattern guide
`patterns/executable-narrative.md`, the review procedure
`reviews/executable-narrative-review.md`, and a `decisions/` tree holding the record template,
a justified worked example, and a rejected worked example.

It adds `manifest/decision-records.yaml` and its JSON Schema, and extends `doctrine-lint` to
validate the registry: identifier shape and uniqueness, owner, scope, non-empty executable
authorities that resolve to real files, non-empty revalidation and obsolescence triggers, the
directory each status is filed under, the archival marker required on a record that is no longer
current authority, and the prohibition on an archived record appearing in an agent pack.

It restates `RUST-DOC-0010-R022`, updates that package's README, rationale, and review standard
for the restatement, corrects `sources/0010-staged-protocols/source-notes.md`, and makes CT³
visible in `patterns/successor-capabilities.md` and its package glossary.

It edits `manifest/doctrines.yaml` for the new entry, the `RUST-DOC-0010` version, and the
repository version; `manifest/agents.yaml` to select the new doctrine for every role and add the
new review procedure to the reviewer and auditor packs; the doctrine, pattern, and review
indexes; `AGENTS.md` for the new canonical root; `tools/bundle-agent-context/src/main.rs` for
the curated pattern and review inventories; `Cargo.toml` and `examples/boundary-validation`
for the version bump; and `EVIDENCE.md` and `CHANGELOG.md`. Bundles under `dist/` are
regenerated.

## Guarantee ledger impact

| Claim                                                                 | Established by                                       | Protected construction                             | Boundary preservation                                 | Escape hatches                          | Does not prove                                             | Residual runtime risk                                       |
| --------------------------------------------------------------------- | ---------------------------------------------------- | -------------------------------------------------- | ----------------------------------------------------- | --------------------------------------- | ---------------------------------------------------------- | ----------------------------------------------------------- |
| Every active decision record carries owner, triggers, and authorities | linter validation of the registry against its schema | the registry is the only active-record entry point | records outside the registry are not active authority | none                                    | that the recorded justification is a good one              | a record whose justification is stated but wrong            |
| An archived record is absent from agent context                       | linter check plus deterministic bundle generation    | packs name canonical sources explicitly            | generated packs are drift-checked                     | none                                    | that a reader will not open the archive directly           | a reader citing an archived record from outside the packs   |
| A generated view cannot drift from its source                         | bundle generation with drift detection               | banner declares the canonical roots                | `dist/` is never hand-edited                          | none                                    | that the canonical source is correct                       | a canonical source that is itself a stale second copy       |
| An enforceable obligation lives in its enforcing mechanism            | review gates and the executability test              | none; this is a review obligation                  | none                                                  | recorded assessment under R002 and R020 | that every obligation in a consuming system was classified | an obligation nobody classified, so nobody found unenforced |

## Compatibility

No consuming Rust API, persisted value, wire format, schema, or example behavior changes. Every
existing rule identifier is preserved. Exactly one existing rule, `RUST-DOC-0010-R022`, changes
meaning and keeps its identifier; the other twenty-one rules in that package, and all 165 rules in
the nine other packages, are unchanged in statement and allowed exceptions.

The restatement is a narrowing in one direction and a widening in another. A design that
previously satisfied R022 by citing a governing decision record for its rationale continues to
satisfy it. A design that cited doctrine as the authority for what the program currently permits
no longer does, and moves that claim to the executable artifact. Nothing in the corpus depended
on the old reading.

`RUST-DOC-0010` advances to 0.2.0 because a rule changes meaning. The repository advances to
0.4.0 for the same reason and because a doctrine is added; the one versioned internal path
dependency advances with it.

Adoption is not retroactive. Existing decision records in consuming systems become subject to
`RUST-DOC-0011-R007` and `RUST-DOC-0011-R009` when they are next reviewed under this corpus
version. A system with no decision records satisfies the doctrine trivially, which is the
intended common case.

## Migration

1. Author the `RUST-DOC-0011` package, fixing rule identifiers before the review standard cites
   them.
2. Add the registry, its schema, and the linter checks with unit tests, before any record exists,
   so the first record is validated on creation rather than retrofitted.
3. Restate `RUST-DOC-0010-R022` and reconcile the package README, rationale, and review standard
   with the restatement.
4. Correct the source notes and restore CT³ visibility.
5. Add the pattern guide, the review procedure, and the worked examples.
6. Update manifests, indexes, agent obligations, bundler inventories, versions, evidence map, and
   changelog.
7. Format canonical Markdown, regenerate bundles, inspect the generated difference, and run the
   complete validation set on both toolchains.

Rollback before publication reverts the additions and restores the previous R022 text.
After adoption, forward repair is preferred: a design that cannot satisfy a rule records the
terms `RUST-DOC-0011-R020` requires rather than weakening the rule.

## Alternatives

- **Leave R022 and add the new doctrine beside it.** Rejected. The corpus would then state a
  blanket precedence in one package and a partition in another, and a reviewer would have no way
  to decide which governs a staged protocol.
- **Delete R022 and let `RUST-DOC-0011` carry the whole obligation.** Rejected. Rule identifiers
  are stable and a removal is a compatibility event; restating the rule preserves traceability
  for anyone who cited it. The staged-protocol package also needs the partition stated in the
  terms of stages and successors, not only in general terms.
- **Prohibit decision records outright.** Rejected as dishonest. An external mandate, an
  irreversible commitment, and an accepted residual risk are real classes of fact that no
  executable artifact carries. A prohibition would push them into commit messages and issue
  threads, which are worse locations with no owner and no expiry.
- **A non-normative guidance document.** Rejected as under-absorption, for the same reason
  RFC-0002 gave. The obligations that matter, that a competing copy is prohibited and that an
  active record must carry an owner and an expiry, are checkable, and guidance no review standard
  checks changes nothing.
- **Enforce record policy only by review, with no linter support.** Rejected. The failure mode is
  a record surviving because nobody revisited it, which is exactly the failure a periodic human
  review also suffers from. The mechanical checks that are cheap and stable were implemented, and
  the judgment-bearing ones were left to review gates.
- **Ship a protocol-graph generator to demonstrate generated views.** Considered and not done in
  this change; see "Evidence plan".

## Security impact

The change reduces the risk that an agent or reviewer acts on an obsolete constraint recorded as
current authority, and the risk that a local guarantee is presented as evidence of an external
fact. The registry makes the active decision set enumerable, which is a prerequisite for auditing
it.

It adds no dependency, no secret material, no unsafe code, and no network or database access. The
linter additions read repository files that are already read. The worked examples are labelled as
examples, name no real organization, and record no obligation of this repository.

One residual risk is worth naming. A doctrine that treats prose as a liability can be misread as
licence to delete rationale that was never recoverable from the code.
`RUST-DOC-0011-R012` and `RUST-DOC-0011-R013` exist to prevent that, and the review procedure
treats an inferred rationale presented as governing as a critical failure.

## Complexity impact

The corpus grows from 187 to 207 normative rules and from ten to eleven packages. Reviewers
acquire one review standard and one review procedure. Authors acquire one registry file and one
schema.

For consuming systems the cost falls mainly on review: classifying a claim before citing an
authority for it, and counting the maintained representations of a claim, are both new steps. The
doctrine offsets this by removing work, since the common outcome of the executability test is
that no record is written at all.

Build cost in this repository is the linter additions and their unit tests. No new crate, no new
compile-fail case, and no new example crate is added.

## Evidence plan

- Doctrine lint validates the registry: schema conformance, identifier shape and uniqueness,
  required owner and scope, executable-authority paths that resolve, non-empty revalidation and
  obsolescence triggers, status-to-directory agreement, the archival marker, and the prohibition
  on an archived record appearing in an agent pack. Each check has a unit test.
- The registry ships with an empty active set, which is the state the doctrine predicts for this
  repository and which the linter validates as such.
- Existing package structure, rule identifier, review-standard citation, front-matter agreement,
  normative-scope, forbidden-marker, and generated-banner checks apply to the new package.
- Deterministic bundle generation and drift detection cover the regenerated distributions.
- Markdown formatting, linting, link checking, dependency policy, and diff hygiene apply as for
  any change.

Limits, stated rather than implied. `RUST-DOC-0011-R002`, `RUST-DOC-0011-R003`,
`RUST-DOC-0011-R012`, `RUST-DOC-0011-R013`, `RUST-DOC-0011-R016`, and `RUST-DOC-0011-R017`
are judgment obligations supported by review gates, not by executed evidence; no linter can
decide whether an obligation could have been made executable. `RUST-DOC-0011-R005` is
demonstrated by this repository's own generated bundles and their drift check, which is real
evidence for the mechanism and not evidence that any particular view elsewhere is generated.

No protocol-graph generator is shipped. A generator that derived the stage graph from the trait
definitions would be genuine evidence; a generator fed by a hand-written edge list would be the
second maintained representation this doctrine prohibits, wearing the word "generated". The
pattern guide illustrates the generated-view leg rather than claiming this repository ships one,
and the evidence map records the gap.

## Source provenance

- The originating internal working document supplied the executable-authority claim and the
  stance that manually maintained decision records are a last resort. Its position is recorded
  accurately in `sources/0011-executable-narrative/source-notes.md`, and the earlier
  mischaracterization in `sources/0010-staged-protocols/source-notes.md` is corrected there and
  is not silently deleted.
- The decision-record form is attributed to its established origin as an architectural practice.
  This doctrine restricts that practice rather than introducing it, and does not claim the
  practice's authors endorse the restriction.
- The authority partition, the registry, the expiry obligation, the historical-veto prohibition,
  the representation count, and the complete gate set are repository governance additions.

## Decision record

- Decision: accepted
- Date: 2026-08-04
- Decision owners: doctrines-rust maintainers
- Rationale: The corpus recorded a source claim inaccurately and then encoded the
  mischaracterization as a normative rule that contradicts two other rules in the same package.
  Correcting the record required stating the partition the corpus already relies on, and the
  partition is only actionable if the second-copy and decision-record obligations are stated with
  it.
- Conditions: Preserve every rule identifier, including the restated one. Ship the registry and
  its linter checks with the doctrine rather than after it. Record in the evidence map which
  rules are judgment obligations with no executable evidence. Do not fabricate an active decision
  record to demonstrate the registry. Regenerate bundles and pass the complete validation set on
  both toolchains.
- Supersedes / superseded by: none. RFC-0002 is not superseded; this decision corrects the
  provenance recorded during its implementation and restates one of its rules.
