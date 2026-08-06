---
id: RFC-0005
title: Make the declared verbosity ceiling real, and reserve its widest tier for readers
author: doctrines-rust maintainers
status: accepted
created: 2026-08-06
affected_doctrines:
  - RUST-DOC-0011
---

# RFC-0005: Make the declared verbosity ceiling real, and reserve its widest tier for readers

## Summary

`manifest/agents.yaml` declares a `maximum_verbosity` for every agent pack. The schema constrains
it, the shared crate decodes it, and the bundler has never read it. This proposal gives the field
an effect: a section of canonical Markdown can carry a verbosity annotation, and a generated output
receives that section only when its ceiling is at least the annotation's tier.

The widest tier becomes reserved. No agent pack may declare it, so material annotated with it
reaches `dist/full-doctrine.md` and no pack. That reservation is what lets the corpus gain reading
aids without any generated pack growing.

No normative rule is added, removed, or amended. The corpus keeps 207 normative rules across
eleven packages. The repository moves to `0.6.0` because generated distributions change shape.

## Motivation

Two problems, one mechanism.

### A declared ceiling that capped nothing

`tools/doctrine-manifest/src/lib.rs` documents `Verbosity` as the verbosity ceiling for a generated
role pack. `manifest/schema/agent-pack.schema.json` enumerates four tiers. `manifest/agents.yaml`
sets one per pack. And searching the bundler for the field returned no matches: it was decoded and
dropped.

It was not merely inert. It was contradicted. The declared order runs from `focused` to
`exhaustive`, while the packs measured:

| Pack          | Declared ceiling |   Bytes |
| ------------- | ---------------- | ------: |
| `maintainer`  | `operational`    | 145,328 |
| `shared`      | `operational`    | 147,300 |
| `planner`     | `focused`        | 195,765 |
| `implementer` | `detailed`       | 222,840 |
| `auditor`     | `exhaustive`     | 229,016 |
| `reviewer`    | `detailed`       | 246,832 |

The pack declared most focused was thirty-five percent larger than one declared less focused.

`foundations/guarantee-honesty.md` opens by stating that a guarantee is a claim backed by an
enforcement mechanism, and that the discipline exists to prevent documentation and generated agent
context from becoming stronger than the implementation. A field named for a ceiling, described as a
ceiling, and enforcing nothing is that failure in the repository's own manifest.

### Reader-facing material cannot be added without taxing every agent

The corpus is hard to navigate. Of 222 canonical Markdown files, 190 carry no outbound link; the
root `README.md` carries two live links against twenty-two inert backticked paths; nineteen
directories hold content and no index.

Repairing that is additive, and `append_source` copies canonical bodies verbatim into generated
outputs. The distributions total 2.31 MB projected from 1.34 MB of canonical Markdown, an
amplification of about 1.73. Every byte added for a human reader therefore costs roughly 1.7 bytes
of agent context, and several packs carry the same file. Adding navigation, cross-references, and
orientation is arithmetically opposed to keeping hydration cheap, unless the projection can tell
the two audiences apart.

## Proposed change

### Annotation grammar

A section annotation is a line that is exactly this, for a tier the schema declares:

```text
<!-- verbosity: detailed -->
```

It sits directly after the heading it governs, which is a heading of level two to six. Its scope
runs to the next heading of the same or a higher level. A nested annotation may not be weaker than
the section enclosing it, so a section's effective tier is the annotation a reader can see rather
than a value assembled from ancestry.

A generated output emits the section when its ceiling is at least the tier. Annotation lines are
stripped at every ceiling, so none reaches a generated file.

Three properties are deliberate.

**The sentinel is the comment opener, not the word.** Any HTML comment in maintained canonical
Markdown that is not a well-formed annotation is an error. A near miss is reported rather than
silently doing nothing. This costs the corpus nothing today: it carries exactly one HTML comment
outside the pull-request template, the banner of the generated accepted-RFC index, and a test
asserts that this stays true.

**Anchoring to a heading rather than fencing a block.** A block-fenced form has four malformation
shapes; this has one, and it is a property of a single line. It cannot split a table or a list. It
cannot leave an empty heading behind, because the heading is inside its own scope and leaves with
it. Deleting a closing fence in the other design silently widens what is withheld, which is the
failure direction this repository has been caught by before.

**Comments inside fenced code are ignored.** A document has to be able to show this syntax without
the example being read as an instruction, which is how this RFC quotes the grammar it proposes.
`doctrine-lint` already skips fenced content when scanning for uppercase requirement terms. The
failure direction is deliberate: an ignored annotation emits a section that might have been
withheld, never the reverse.

### What may never be annotated

`RUST-DOC-0011-R018` requires generated agent context to be built from current authority. A ceiling
able to withhold a rule statement would remove an obligation from an agent's view while the receipt
described it as detail. Three classes are therefore projected whole, and an annotation in one of
them stops generation:

- every doctrine's `normative_path`, which states rules directly;
- everything under `foundations/`, which states them in lowercase prose. The nine questions
  `foundations/guarantee-honesty.md` requires every type-level design to answer are an obligation
  that no scan for uppercase requirement terms detects;
- every path any pack lists in `review_checklists`, which states the evidence a gate demands.

The set is derived from the manifests rather than from file names, and both tools call one
implementation of the test.

### The reserved tier

No agent pack may declare the widest tier the schema permits. `auditor` moves from `exhaustive` to
`detailed`, and `doctrine-lint` rejects any pack that declares the reserved tier.

Without this the mechanism has a hole. A pack at the widest ceiling withholds nothing by
definition, so it would absorb the whole of any future canonical growth and get strictly larger,
which is the opposite of the intent. With the reservation, material annotated at the widest tier
reaches `dist/full-doctrine.md` and nothing else, so reading aids are free for every pack.

### Disclosure

Every filtered output carries an `## Assembly` section naming the ceiling it applied, where that
ceiling is declared, and how much it withheld. It is emitted even when nothing was withheld,
because a reader cannot otherwise distinguish an output that had nothing to withhold from one whose
disclosure was forgotten.

Each maximal run of withheld sections is replaced in place by a receipt naming the headings, their
tiers, and the canonical file. Locality is the point. Without it, a reader who meets sections one
through eight and then ten concludes that nine was never written.

## Affected artifacts

`tools/doctrine-manifest/src/lib.rs`, `tools/bundle-agent-context/src/main.rs`,
`tools/doctrine-lint/src/main.rs`, `manifest/agents.yaml`, `CHANGELOG.md`, the workspace version
and its versioned internal path dependencies, and every regenerated distribution.

No doctrine package changes. No rule identifier, statement, applicability, exception, or review
evidence changes.

## Guarantee ledger impact

| Claim                                                        | Before                                               | After                                                          |
| ------------------------------------------------------------ | ---------------------------------------------------- | -------------------------------------------------------------- |
| A pack's declared ceiling bounds what that pack receives     | not established; the field was decoded and discarded | established, by comparison at projection time                  |
| Obligations reach every pack regardless of ceiling           | trivially true; nothing was ever withheld            | established, by the untierable set and a generation-time error |
| A reader can tell whether an output withheld anything        | not established                                      | established, by the assembly note and in-place receipts        |
| Reader-facing material can be added without growing any pack | false                                                | established, by the reserved tier                              |
| An annotated tier honestly describes the material it covers  | not applicable                                       | still not established, and cannot be mechanically              |

The last row is the honest limit. No mechanism distinguishes a worked application from a
load-bearing constraint. The three untierable classes are protected structurally; everything else
rests on an author's judgement reviewed in a diff.

## Compatibility

Generated distributions change. Every pack gains an assembly note, and `auditor` regenerates at a
narrower ceiling. Canonical Markdown is unchanged by this proposal, and no consumer of a rule
identifier is affected.

`foundations/normative-language.md` reserves minor releases for added normative requirements. No
normative requirement is added here, but `CONTRIBUTING.md` classes significant distribution
restructuring as requiring an RFC, and the shape of every generated pack changes. The repository
moves to `0.6.0` on that basis, and no doctrine package version moves.

## Migration

None for consumers of canonical Markdown. A consumer that pinned the byte layout of a generated
pack will see an added section near the top of each one.

## Alternatives

**Curate `canonical_sources` instead.** The manifest already selects sources per pack at file
granularity, already schema-validated and already drift-checked. Dropping one foundation from one
pack is a measurable cut with no new machinery. Not rejected, and this proposal does not displace
it. It is a different lever: it removes existing content from a pack, while this one keeps new
content out. Both are wanted, and curation is deliberately left out of this change set so that a
change to what agents already receive is reviewed on its own terms rather than inside a
presentation pass.

**Split canonical files by audience.** Move the reader-facing half of a document into a sibling
file and let `canonical_sources` do the rest. This needs no grammar and no new failure modes, and
`validate_curated_inventory` would enforce the split the day it landed. Rejected as the primary
mechanism because it fragments documents that read as one argument, and because the split point
would have to be chosen once for all six packs rather than per pack. It remains the better answer
wherever a document genuinely contains two documents.

**Deduplicate across packs.** The distributions carry 2.31 MB of text of which roughly 700 KB is
distinct; `RUST-DOC-0011` is inlined into all six packs. Deduplication would cut packs by half or
more, which is an order of magnitude beyond what this proposal achieves on existing content.
Rejected here because it costs pack self-containment, which is the property that makes a pack
usable as a single hydration input. It is recorded as the largest remaining opportunity.

**Declare a per-output byte budget.** Require each output to stay under a declared size and fail
`check` when it grows. Rejected as a substitute because it reports the symptom without giving an
author any way to act on it, but it composes with this proposal and is worth revisiting.

**Delete the field.** Remove `maximum_verbosity` from the schema, the manifest, and the shared
crate, on the grounds that an unenforced claim should be retracted rather than fulfilled. This is
the honest alternative to the whole proposal and was seriously considered. Rejected because the
declared ceilings encode a real editorial judgement about how much each role needs, and because
deletion would leave the presentation problem with no mechanism at all.

## Security impact

Neutral to slightly favourable. Nothing is withheld from an agent that carries an obligation, and
the untierable set is enforced by a generation-time error rather than by convention. The mechanism
adds no trust boundary and no escape hatch.

One risk is created and disclosed: an author can move material out of narrow packs by annotating
it, and no check judges whether the tier is honest. The mitigation is structural rather than
mechanical, and is stated in the guarantee ledger above rather than left implicit.

## Complexity impact

The grammar is roughly two hundred lines in the shared crate, called by both tools, with no new
dependency. The alternative that avoids it entirely is splitting files, and its cost is paid in
document structure instead of in code.

The limit is worth stating plainly, because the proposal would otherwise be read as a general
mechanism for shrinking packs. It is not. A doctrine's normative file is between a half and three
quarters of every pack and can never be annotated; extending the untierable set to the foundations
and the review checklists, which honesty requires, leaves under a tenth of existing pack content
eligible. This proposal makes new reader-facing material free. Shrinking what packs already carry
is the work of the first alternative above.

## Evidence plan

- Eighteen tests in `doctrine-manifest` cover the grammar, including each malformation that
  survives Prettier unchanged, a tilde fence closed by inner backticks, non-monotone nesting,
  idempotence, and the rejection of every tier in a file that states obligations;
- one test asserts that the annotation parser accepts exactly the tiers the schema declares, beside
  the existing test that asserts the variants against the schema itself;
- `bundle-agent-context` tests that a missing link target still fails generation when it sits
  inside a section the ceiling withholds, which pins the ordering that makes link validation
  independent of what any pack receives;
- `doctrine-lint` tests that only the widest tier is reserved, that the alert vocabulary is closed,
  and that the corpus carries no HTML comment outside the generated index;
- positive controls were run against the real files each check protects: an annotation seeded into
  a doctrine's normative file, a foundation, and a review checklist was rejected by both tools with
  distinct messages, and one annotation in `agents/shared.md` was observed withheld from the four
  outputs below its tier and present in the four at or above it.

## Source provenance

No external material. The dead field was found by auditing this repository's generated outputs
against the manifest that describes them. The reasoning is recorded as an internally derived
refinement.

## Decision record

- Decision: accepted
- Date: 2026-08-06
- Decision owners: doctrines-rust maintainers
- Rationale: `maximum_verbosity` was declared in the schema, set in the manifest, decoded by the
  shared crate, and never read, while the measured pack sizes ran contrary to the order it
  declared. Under `foundations/guarantee-honesty.md` that is a claim without a mechanism, in the
  repository's own manifest. Separately, the corpus needs reading aids it cannot afford to add
  while every canonical byte is copied into generated packs at an amplification of about 1.73. One
  mechanism answers both.
- Conditions: Annotate nothing that states an obligation, enforced from the manifests in both tools
  rather than by file name, so a doctrine's normative file, the foundations, and every review
  checklist are projected whole. Reserve the widest tier from every pack, so reader-facing material
  cannot grow a pack. Rewrite links before withholding anything, so link validation stays
  independent of what any output receives. Disclose the applied ceiling in every filtered output
  and mark each withheld run in place. Land the mechanism with no annotation anywhere in the
  corpus, so the first regeneration shows only the disclosure and the ceiling change. State the
  limit rather than implying a general size reduction. Move the repository to `0.6.0` and no
  doctrine package version. Pass the complete validation set on both toolchains.
- Supersedes / superseded by: none. RFC-0001 through RFC-0004 are unaffected, and no rule they
  introduced changes.
