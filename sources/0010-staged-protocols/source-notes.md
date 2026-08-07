# RUST-DOC-0010 source notes

## Originating internal document

This package absorbs an internal working document that proposed a design vocabulary for
consuming stage traits whose associated output type is bounded by the next legal capability. The
document coined the name "Chainable Telescopic Typestate Traits," abbreviated CT³, and
explicitly stated that the term was a proposed project-level name rather than a standardized
academic one.

The document is not reproduced here. Its durable content is rewritten as the normative rules,
rationale, decision framework, review gates, anti-patterns, and executable example in this
package. The classification below records what this repository accepted, refined, rejected, and
added, so that borrowed ideas do not arrive carrying more authority than their argument.

## Accepted core

The central mechanism is accepted. A stage capability that has a legal successor benefits from
naming that successor as an associated type bounded by the capability the successor must
satisfy. This is genuinely stronger than returning one concrete successor type, and the
repository did not previously express it: `patterns/typestate.md` covers marker generics with
state-specific inherent implementations, and `patterns/consuming-transitions.md` covers
ownership transfer, but neither abstracts the successor relationship through a trait. It became
`RUST-DOC-0010-R003`.

The reason the abstraction earns its cost is also accepted: one capability may have several
implementations producing different successor evidence, and without an associated successor type
that requires either a widened successor carrying every proof as an option, or a duplicated
protocol. The example crate exercises exactly this with two entry stages.

Also accepted, within scope: stages carry evidence rather than being empty markers; transitions
consume the prior stage; failures stay stage-specific inside the protocol; branches and recovery
paths are named and typed rather than implied; illegal orderings deserve compile-fail evidence;
and escape hatches are restricted rather than ambient.

## Refined claims

**Scope of application.** The source lists payment settlement, registration and onboarding,
workflow engines, database migration phases, and deployment promotion as fits for the mechanism.
Most of those are durable, multi-actor lifecycles. This repository's existing position, in
`AGENTS.md` and `foundations/guarantee-honesty.md`, restricts type-level protocol enforcement to
locally controlled, small, static protocols and prefers a runtime representation for dynamic,
persisted, heterogeneous, or externally chosen state. The claim is narrowed rather than adopted:
`RUST-DOC-0010-R015` scopes the typed protocol to one in-process pass and keeps the durable model
at runtime, and `patterns/hybrid-state-machines.md` remains the mechanism for combining them.

**Persistence and nominal database types.** The source proposes extending nominal domain
semantics into PostgreSQL base types so that comparing two identifier species fails without an
explicit cast. That is a reasonable persistence technique and it belongs to RUST-DOC-0005, which
already governs decoding and schema-reinforced invariants; this package does not restate it.

What this package does take from that section is the correction beneath it. A consuming Rust
transition moves a local value. A row is read into a value and can be read again by another
worker, so the move consumes nothing durable and two workers can each hold a consumed handle for
the same row. Compiling typestate into database procedures does not change this; the multiversion
concurrency model supplies linearity, not the Rust move. `RUST-DOC-0010-R014` states the limit
and requires identity, stored state, and a concurrency token to be re-checked where durable state
advances.

**Async transitions.** The source's async section is accepted and narrowed. Cancellation
mechanics belong to RUST-DOC-0004; `RUST-DOC-0010-R016` requires only the per-stage contract:
cancellation behavior, retry safety, the identity under which a retry deduplicates, and whether
the successor proof exists only after a durable acknowledgment.

**Guidance for agents.** The source ends with an interpretation contract and a copy-and-paste
prompt intended to calibrate another agent. The repository already carries agent obligations in
`agents/` with deterministic generated hydration packs selected through `manifest/agents.yaml`.
The content is therefore delivered through that mechanism rather than as free-floating prompt
text, so that agent instructions stay versioned, reviewable, and consistent with the doctrine
they cite.

**Terminology.** The source's own caution about its coinage is accepted and made binding.
`RUST-DOC-0010-R021` requires local vocabulary to travel with its family attribution; the
glossary records CT³ as local vocabulary, and `patterns/successor-capabilities.md` states the term
and explains what each of its words carries, so older internal documents remain readable.

**Executable authority.** The source argues that an enforceable architectural obligation should
live in the mechanism that enforces it rather than survive only as prose. The repository accepts
that claim for current operational truth: legal ordering, available capabilities, construction
restrictions, and negative guarantees.

The claim does not extend to external reality, rationale that cannot be recovered from the
implementation, accepted risk, or change authority. Those remain separate authorities.

The repository therefore adopts an authority partition. It rejects both "code explains
everything" and "documentation has blanket precedence". `RUST-DOC-0010-R022` states the partition
for staged protocols and `RUST-DOC-0011` governs it generally.

The source also treats manually maintained decision records as a last resort, because they
duplicate the system, drift independently, and can harden obsolete decisions into barriers to
improvement. That stance is stronger than "documentation should not be the only enforcement", is
accepted as stated, and became `RUST-DOC-0011-R006` through `RUST-DOC-0011-R010`.

**Correction of an earlier entry in this file.** Until repository version 0.4.0 this section's
neighbouring "Rejected claims" list carried an item headed "Code as sufficient contract". It
summarized the source as arguing that the executable protocol is the authoritative live contract
and that documentation should not be the only enforcement, accepted the second half, and rejected
the first. The source had not made the rejected claim, and the rejection was then encoded as a
blanket governance precedence in `RUST-DOC-0010-R022`, which contradicted `RUST-DOC-0010-R018`
and `RUST-DOC-0010-R019` in the same package. RFC-0003 restates the rule and this entry replaces
the mischaracterization. The correction is recorded here rather than applied silently, because a
provenance file whose errors are edited out teaches nothing about how they happened.

## Rejected claims

**The scored review rubric.** The source proposes rating a design zero to two across ten
categories and reading the total as a verdict, with bands from "ordinary runtime workflow" to
"exemplary." This repository records each gate as pass, fail, not applicable, or waiver
reference, and `reviews/README.md` states that blank status is not approval. A numeric total lets
strong scores in cheap categories offset a critical failure in an expensive one, which is
precisely what the severity model exists to prevent. The rubric is not adopted in any form.

**Stage-per-step granularity.** Several of the source's worked flows introduce a stage for
transformations that establish no fact a later stage consumes. `RUST-DOC-0010-R012` requires a
stage to be a proof boundary and requires the count to be assessed against the complexity budget.

## Repository additions

These are not derived from the source and are this repository's governance:

- the prohibition on conversion, derive, and public constructor paths that manufacture a later
  stage, together with the requirement that remaining trusted paths carry an owner, a stated
  caller obligation, and a ledger entry (`RUST-DOC-0010-R010`, `RUST-DOC-0010-R011`);
- the executable topology assertion, which detects a redirected associated type or a widened
  bound that leaves every existing negative test passing (`RUST-DOC-0010-R019`);
- the requirement that a bound may not be widened to make an implementation compile
  (`RUST-DOC-0010-R004`);
- per-stage effect disclosure and the prohibition on durable writes in stages named for checks
  (`RUST-DOC-0010-R013`);
- the requirement that an undetermined outcome stays distinguishable from both branches
  (`RUST-DOC-0010-R007`, gate S25);
- the per-stage guarantee ledger row (`RUST-DOC-0010-R020`);
- the complete review gate set, severities, and waiver terms.

## Established families

The mechanism is a refinement within typestate-oriented programming and behavioral types rather
than a new formalism. Strom and Yemini introduced typestate as a compile-time discipline; Aldrich
and colleagues developed state as a first-class unit with state-specific members and the framing
of legal call sequences as an object protocol. Session types are a close relative and are the
better frame when communication duality, send and receive sequencing, and participant
compatibility are central; this doctrine governs an object's or command's internal protocol
instead. Citations are in `references.md`.

## Version and date scope

Language mechanics were checked against current Rust documentation for the pinned toolchain
1.97.1 and the minimum supported version 1.85.0 on 2026-08-04. The `trybuild` behavior recorded
in this package corresponds to version 1.0.118. Compiler diagnostic rendering differs between
those two toolchains; the compile-fail fixtures in this package were shaped so that the same
committed diagnostic matches both, and that shaping is a property of the fixtures rather than of
the doctrine.

> [!TIP]
> [attribution](attribution.md) · **source notes**
> Index: [all source packages](../README.md).
> Doctrine: [`doctrines/0010-staged-protocols/`](../../doctrines/0010-staged-protocols/README.md).
