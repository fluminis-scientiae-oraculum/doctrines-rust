# RUST-DOC-0011 source notes

## Originating internal document

This package absorbs the second half of the internal working document whose first half became
RUST-DOC-0010. That half argued about where an architectural obligation should live, and it was
mishandled during the earlier absorption.

The document is not reproduced here. The classification below records what this repository
accepted, refined, rejected, and added, and it also records what the earlier absorption recorded
inaccurately, because a correction that quietly replaces the old text teaches nothing about how
the error happened.

## Correction of the earlier record

[`sources/0010-staged-protocols/source-notes.md`](../0010-staged-protocols/source-notes.md) originally listed, under
"Rejected claims", an item headed "Code as sufficient contract". It summarized the source as arguing "that the
executable protocol is the authoritative live contract and that documentation should not be the only enforcement",
accepted the second half, and rejected the first.

That summary attributed a claim the source did not make. The source did not argue that code
explains external constraints, accepted risk, or change authority. It argued that an obligation
which a mechanism can enforce belongs in that mechanism rather than surviving only as prose. The
rejection of the misattributed claim was then encoded as `RUST-DOC-0010-R022`, a blanket
governance precedence over the executable protocol, which contradicted `RUST-DOC-0010-R018` and
`RUST-DOC-0010-R019` in the same package. RFC-0003 restates that rule and the source notes for
RUST-DOC-0010 now carry the accurate classification.

## Accepted core

**Executable authority.** The source argues that an enforceable architectural obligation should
live in the mechanism that enforces it rather than survive only as prose. The repository accepts
that claim for current operational truth: legal ordering, available capabilities, construction
restrictions, and negative guarantees. It became `RUST-DOC-0011-R002` and `RUST-DOC-0011-R003`.

The claim does not extend to external reality, rationale that cannot be recovered from the
implementation, accepted risk, or change authority. Those remain separate authorities, and the
repository therefore adopts an authority partition rather than a precedence rule. It rejects both
"code explains everything" and "documentation has blanket precedence".

**Decision records as a last resort.** The source treats manually maintained architecture
decision records as noise by default, because they duplicate the system, drift independently of
it, and can harden obsolete decisions into barriers to improvement. This is a stronger claim than
"documentation should not be the only enforcement", and reducing it to that weaker form was part
of the earlier mischaracterization. It is accepted as stated and became `RUST-DOC-0011-R006`
through `RUST-DOC-0011-R010`.

**Improvement cost as the measure.** The source's argument for minimizing representations is that
each additional maintained description raises the cost of a future improvement, so the number of
representations is itself an architectural quantity. Accepted, and made a stated review quantity
rather than a sentiment, in `RUST-DOC-0011-R017`.

**Duplication as liability.** The source treats a prose artifact that repeats executable truth as
a cost rather than a documentation achievement, listing another source to update, another
interpretation boundary, another stale artifact, another review surface, and another opportunity
for an agent to infer outdated intent. Accepted and narrowed to enforced claims in
`RUST-DOC-0011-R004`.

## Refined claims

**Generation.** The source prefers generated views over manual synchronization. Accepted, with
one addition the source does not make: a generator fed by a hand-maintained description of the
same claim is the competing copy under another name. `RUST-DOC-0011-R005` states that case
explicitly, because it is the failure a team reaches for first when a view resists derivation.

**Historical gravity.** The source observes that a record of the form "we chose A over B at time
T because of constraints C" becomes discoverable authority after C disappears. Accepted, and split
into two obligations, because they fail independently: `RUST-DOC-0011-R009` ends the record when
its reason ends, and `RUST-DOC-0011-R010` governs the citation even while the record is still
active, since a record can be current and still be cited without checking whether it applies to
the change at hand.

**Agent context.** The source argues that current authority should be easier to find than historical decisions. Accepted
and delivered through this repository's existing mechanism: [`manifest/agents.yaml`](../../manifest/agents.yaml) selects
canonical sources and the bundler produces deterministic hydration packs, so `RUST-DOC-0011-R018` states the obligation
in terms of that mechanism rather than as a general recommendation about repository layout.

**Rationale boundaries.** The source's position on rationale is mostly negative, saying what
should not be written. The repository adds the positive obligation: an absent reason is recorded
as unknown, and an inference is labelled as one. `RUST-DOC-0011-R013` exists because deleting bad
rationale and inventing plausible rationale are the two failure modes of the same cleanup, and the
source only warns about the first.

## Rejected claims

**Documentation as noise in general.** The source's framing is sharpest when it treats
manually maintained records as noise by default. Applied to decision records that duplicate
enforced claims, the framing is accepted. Extended to documentation as such, it is rejected: an
external mandate, a rejected alternative, an accepted residual risk, and a non-guarantee are
carried by nothing else, and the same argument that makes a duplicate costly makes an
irrecoverable reason valuable. `RUST-DOC-0011-R012` and the anti-pattern "Deleting rationale in
the name of executable architecture" record the limit.

**Code as the whole account of a system.** Where the source's language implies that a
sufficiently expressive type system removes the need for any recorded reasoning, the repository
does not follow. `RUST-DOC-0011-R016` requires the enforcing artifact to be legible, which is the
strongest version of that claim this repository will make, and it is a requirement on the artifact
rather than a licence to stop writing.

## Repository additions

These are not derived from the source and are this repository's governance:

- the five-class authority partition and the obligation to classify a claim before citing an
  authority for it (`RUST-DOC-0011-R001`);
- the obligation to state the unenforced part of a partially enforced claim separately
  (`RUST-DOC-0011-R003`);
- the machine-readable active registry, its schema, and the linter validation of owner,
  revalidation trigger, obsolescence condition, and resolvable executable authorities
  (`RUST-DOC-0011-R007`);
- the requirement that a record answer one question and state what it does not govern
  (`RUST-DOC-0011-R008`);
- the retirement of an implemented proposal from operational authority
  (`RUST-DOC-0011-R011`);
- the external-authority naming obligation (`RUST-DOC-0011-R014`);
- the labelling obligation for an unenforced compatibility promise (`RUST-DOC-0011-R015`);
- the five recorded terms for every exception (`RUST-DOC-0011-R020`);
- the complete review gate set, severities, and waiver terms.

## Version and date scope

Language and tooling mechanics were checked against current documentation for the pinned
toolchain 1.97.1 and the minimum supported version 1.85.0 on 2026-08-04. The decision-record
registry schema targets JSON Schema Draft 2020-12, consistent with the doctrine and agent-pack
manifests already in this repository.

> [!TIP]
> [attribution](attribution.md) · **source notes**
> Index: [all source packages](../README.md).
> Doctrine: [`doctrines/0011-executable-narrative/`](../../doctrines/0011-executable-narrative/README.md).
