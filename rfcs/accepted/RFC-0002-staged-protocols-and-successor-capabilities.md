---
id: RFC-0002
title: Add a doctrine for staged protocols and successor capabilities
author: doctrines-rust maintainers
status: accepted
created: 2026-08-04
affected_doctrines:
  - RUST-DOC-0010
---

# RFC-0002: Add a doctrine for staged protocols and successor capabilities

## Summary

A tenth doctrine, `RUST-DOC-0010` "Staged Protocols and Successor Capabilities", will govern
in-process multi-stage protocols whose stage capabilities expose their legal successor as an
associated type bounded by the next capability. It adds twenty-two rules, a fifty-eight gate
review standard, a pattern guide, a worked case study, and an executable example with a topology
assertion and three compiler-rejection cases. No existing rule identifier, statement, allowed
exception, or doctrine version changes. The repository advances to 0.3.0.

## Motivation

The corpus governs the neighbouring ground and leaves one mechanism unowned. `RUST-DOC-0001`
governs legal transitions and unrepresentable states. `RUST-DOC-0003` governs consuming custody.
`patterns/typestate.md` covers marker generics with state-specific inherent implementations, and
`patterns/consuming-transitions.md` covers ownership transfer. None of them abstracts the
successor relationship through a trait, so none of them can express the contract "whatever this
stage produces, it is something the next capability accepts."

That gap has a concrete cost. A protocol with two entry paths carrying different evidence, such
as a self-service and an invited registration, cannot be served by a hardcoded successor type. The
available workarounds are one widened successor carrying each proof as an optional field, which
reintroduces exactly the contradictory combinations `RUST-DOC-0001` exists to remove, or a
duplicated protocol per entry path.

Three further failures motivated rules that no existing doctrine states:

A protocol edge can be deleted by a one-line refactor. Changing `type Next: CheckIdentity` to
`type Next` looks like a generics simplification, leaves every existing negative test passing, and
removes the guarantee entirely. Compile-fail evidence does not detect it, because the programs
those tests reject remain rejected while the edge they protected no longer exists.

A conversion can manufacture a later stage. A `From<RawSubmission> for AcceptedRegistration`
implementation added for a fixture asserts every proof that stage represents, having performed
none of them. `RUST-DOC-0001` protects newtype construction against decoding bypass; it does not
address conversions between protocol stages.

A local move can be read as a durable one. Consuming a Rust value proves the caller cannot use
that value again. A stored row is read into a value and can be read again by another worker, so
two workers can each hold a consumed handle for the same row. Designs that map stages onto
persisted lifecycle states and conclude the durable advance happened once are reasoning from the
strongest local guarantee available to a distributed claim it cannot support.

The originating material for this doctrine is an internal working document that proposed the
mechanism under a coined name. Absorbing it required refining its scope, rejecting its scoring
rubric and its claim that code is a sufficient contract, and adding the governance this repository
requires. That classification is recorded in the source notes rather than left implicit.

## Proposed normative changes

All twenty-two rules are new. No existing rule is altered.

| Doctrine/rule      | Current meaning | Proposed meaning                                                                                       | Reason                                                                    |
| ------------------ | --------------- | ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------- |
| RUST-DOC-0010-R001 | none            | Inventory stages, edges, evidence, failures, and effects before introducing stage types                | Prevents a type graph derived from existing functions rather than proofs  |
| RUST-DOC-0010-R002 | none            | Name each stage for the fact its construction establishes                                              | Keeps the graph readable as proofs rather than processing steps           |
| RUST-DOC-0010-R003 | none            | Expose a legal successor as an associated type bounded by the next capability                          | The defining mechanism; makes the protocol edge checkable                 |
| RUST-DOC-0010-R004 | none            | Bounds name only capability actually established and are not widened to make code compile              | Closes the one-line deletion of a protocol edge                           |
| RUST-DOC-0010-R005 | none            | Consume the stage on transition where reuse is invalid                                                 | Scopes the `RUST-DOC-0003` posture to stage-to-stage transitions          |
| RUST-DOC-0010-R006 | none            | Carry forward exactly what successors need; drop superseded raw representations                        | Prevents raw values being mistaken for checked ones after their stage     |
| RUST-DOC-0010-R007 | none            | Keep failures stage-identifying inside the protocol, including undetermined outcomes                   | Preserves which proof failed, and keeps unknown distinct from rejection   |
| RUST-DOC-0010-R008 | none            | Model material branches as named sums over distinct successors                                         | Prevents a branch degrading into optional fields                          |
| RUST-DOC-0010-R009 | none            | Name retry, revision, and recovery edges                                                               | Makes the recovery half as reviewable as the success path                 |
| RUST-DOC-0010-R010 | none            | Prohibit conversions, derives, and public constructors that produce a later stage                      | Closes the bypass that makes a stage graph decorative                     |
| RUST-DOC-0010-R011 | none            | Restrict, own, and inventory the trusted construction paths that remain                                | Keeps necessary paths from becoming ambient bypasses                      |
| RUST-DOC-0010-R012 | none            | Keep stage granularity at proof boundaries and assess it against the complexity budget                 | Guards both over-fragmentation and stages hiding several responsibilities |
| RUST-DOC-0010-R013 | none            | Disclose durable and external effects per stage; check-named stages perform no durable write           | Keeps the collapsed chain an accurate summary of what the protocol does   |
| RUST-DOC-0010-R014 | none            | A local transition is not durable evidence; durable advancement re-checks identity, state, and a token | Separates the local guarantee from the distributed claim                  |
| RUST-DOC-0010-R015 | none            | Persisted or multi-actor lifecycle stays a runtime model; typed stages cover one in-process pass       | Narrows the mechanism to where it is sound                                |
| RUST-DOC-0010-R016 | none            | State cancellation, retry safety, idempotency identity, and acknowledgment ordering per async stage    | Keeps an interrupted transition from yielding an unearned successor       |
| RUST-DOC-0010-R017 | none            | Erase the protocol only at a named boundary                                                            | Keeps the graph checkable for its whole length                            |
| RUST-DOC-0010-R018 | none            | Provide compile-fail evidence for each claimed impossibility                                           | Keeps a claimed impossibility from silently becoming possible             |
| RUST-DOC-0010-R019 | none            | Assert the documented stage graph executably                                                           | Detects a redirected type or widened bound that negative tests cannot     |
| RUST-DOC-0010-R020 | none            | Record a guarantee ledger row per stage                                                                | Makes honesty auditable at the granularity claims are made                |
| RUST-DOC-0010-R021 | none            | Do not present local vocabulary as standardized external terminology                                   | Keeps a local coinage from borrowing external authority                   |
| RUST-DOC-0010-R022 | none            | The executable protocol does not replace doctrine, review evidence, or the decision process            | Keeps an accurate observation from becoming an overreaching claim         |

## Affected doctrine IDs and artifacts

The implementation adds the `RUST-DOC-0010` package with its eight files, source notes and
attribution under `sources/0010-staged-protocols/`, the pattern guide
`patterns/successor-capabilities.md`, the `registration-onboarding` case study, and the
`staged-protocol` example crate with three cases under `examples/compile-fail/ui/`.

It edits `manifest/doctrines.yaml` for the new entry and the repository version,
`manifest/agents.yaml` to select the doctrine for the planner, implementer, reviewer, and auditor
packs, the doctrine, pattern, and case-study indexes, `reviews/typestate-review.md` for a
staged-protocol gate group, `tools/bundle-agent-context/src/main.rs` for the curated pattern
inventory, `examples/src/lib.rs` and `Cargo.toml` for workspace membership and the version bump,
`.github/workflows/rust-examples.yml` for the new crate, and `EVIDENCE.md` and `CHANGELOG.md`.
Generated bundles under `dist/` are regenerated.

`RUST-DOC-0001`, `RUST-DOC-0003`, `RUST-DOC-0004`, `RUST-DOC-0005`, and `RUST-DOC-0006` gain
cross-references to the new pattern in their manifest entries. Their normative text is unchanged.

## Guarantee ledger impact

| Claim                                               | Established by                                           | Protected construction                 | Boundary preservation                                | Escape hatches                      | Does not prove                                      | Residual runtime risk                                    |
| --------------------------------------------------- | -------------------------------------------------------- | -------------------------------------- | ---------------------------------------------------- | ----------------------------------- | --------------------------------------------------- | -------------------------------------------------------- |
| The in-process protocol ran in the documented order | consuming transitions plus bounded associated successors | private stage fields, no constructors  | untrusted input canonicalized at the first stage     | restricted trusted paths under R011 | that any durable or remote effect occurred          | a stage reached through an unreviewed trusted path       |
| A stage's successor satisfies the next capability   | associated-type bound checked by the compiler            | bound may not be widened under R004    | restoration issues typed stages through checked code | none                                | that the successor's evidence is externally current | a bound relaxed without the topology assertion           |
| The documented graph is the compiled graph          | the executable topology assertion under R019             | assertion covers every documented edge | assertion runs in the ordinary test suite            | waiver under the waiver section     | that the graph is correct for the domain            | an edge added to code and omitted from the assertion     |
| Durable state advanced exactly once                 | identity, stored state, and concurrency token re-checked | the authoritative query or procedure   | the durable model is a runtime representation        | administrative repair paths         | that the local protocol observed the advance        | lost update, stale read, or an unfenced competing writer |

## Compatibility

No consuming Rust API, persisted value, wire format, schema, rule identifier, or existing allowed
exception changes. The nine existing doctrines keep their versions; only the repository version
advances, to 0.3.0, because a doctrine is added.

Adoption is not retroactive. Existing designs in consuming systems become subject to the new rules
when they are next reviewed under this corpus version, and a design that satisfied `RUST-DOC-0001`
and `RUST-DOC-0003` before this RFC continues to satisfy them.

Older generated agent bundles lack the new selections and are regenerated rather than mixed with
the new manifest. The workspace version bump requires the one versioned internal path dependency,
`validated-newtypes` in `examples/boundary-validation`, to advance coherently.

## Migration

1. Add the example crate, its topology assertion, and the compiler-rejection cases; confirm the
   fixtures match on both the pinned toolchain and the minimum supported version.
2. Author the doctrine package, fixing rule identifiers before the review standard cites them.
3. Author the pattern guide, source notes, attribution, and case study.
4. Update the manifests, indexes, review gate group, bundler inventory, workspace membership,
   versions, evidence map, and changelog.
5. Format canonical Markdown, regenerate bundles, inspect the generated difference, and run the
   complete validation set on both toolchains.

Rollback before publication reverts the additions and regenerates bundles; because no existing
rule changes, removal is clean. After adoption, forward repair is preferred: a design that cannot
satisfy a rule records a waiver on the terms the doctrine states rather than weakening the rule.

## Alternatives

- **A pattern document with no normative rules.** Rejected as under-absorption. The mechanism's
  most valuable obligations, the bound that may not be widened, the conversion prohibition, and
  the durable-advancement limit, would be non-binding guidance that no review standard checks.
- **Extending `RUST-DOC-0001` and `RUST-DOC-0003` with a few rules.** Seriously considered, and
  rejected on scope. The additions span protocol topology, branch modelling, effect disclosure,
  erasure boundaries, async stage contracts, and the durable limit. Distributing them across two
  doctrines whose scopes are value representation and ownership authority would blur both, and
  rule identifiers would no longer indicate which concern governs.
- **Folding the mechanism into `patterns/typestate.md`.** Rejected because that pattern documents
  marker generics with inherent implementations. Adding a trait-abstracted successor mechanism to
  it would make one document describe two mechanisms with different costs and different failure
  modes.
- **Adopting the source document's scoring rubric.** Rejected. A numeric total lets strong scores
  in cheap categories offset a critical failure in an expensive one, which is what the severity
  model exists to prevent.
- **Requiring the mechanism wherever a protocol has stages.** Rejected. A concrete successor
  return is simpler and equally safe when a capability has one implementation, and
  `RUST-DOC-0010-R012` requires the comparison rather than presuming the mechanism wins.

## Security impact

The doctrine reduces integrity risk from bypassed protocol stages, forged stage evidence, and
local transitions presented as durable ones. The conversion prohibition and the trusted-path
inventory directly target authority-bearing stages such as authorization and consent.

It adds no dependency beyond an existing internal path dependency between two example crates, no
secret material, no unsafe code, and no network or database access. The example crate uses an
in-memory collaborator and generic example identifiers.

One residual risk is worth naming: a doctrine that makes ordering visibly enforced can encourage a
reader to trust a stage name over its construction. `RUST-DOC-0010-R014` and `RUST-DOC-0010-R020`
exist to keep the claims narrow, and the review gates treat an overclaiming ledger row as critical.

## Complexity impact

The corpus grows from 165 to 187 normative rules and from nine to ten packages. Reviewers acquire
one more review standard, and the typestate review gains a gate group.

For consuming systems the cost is real: longer signatures, worse first-encounter diagnostics
because a mismatch appears as an unsatisfied bound, generic parameters travelling into helper and
test code, and monomorphization growing with stages multiplied by implementations. The doctrine
answers this by making the comparison against a runtime enum and against ordinary sequenced
functions a required step, with two explicit exits into simpler designs in the decision tree.

Build cost in this repository is one additional small crate and three additional trybuild cases.

## Evidence plan

- The `staged-protocol` crate exercises both entry paths, both branches, both recovery edges, the
  undetermined failure, stale consent, malformed input, and canonical-value survival.
- A topology assertion pins every documented edge, so a redirected associated type or widened
  bound fails the build.
- Three compiler-rejection cases cover stage skipping, consumed-stage reuse, and evidence forgery,
  with committed diagnostics that match on both the pinned toolchain and the minimum supported
  version.
- Doctrine lint checks package structure, rule identifiers, review-standard citation of every
  rule, front-matter agreement with the manifest, and normative-term scope.
- Markdown formatting, linting, link checking, deterministic bundle generation and drift
  detection, dependency policy, and diff hygiene apply as for any change.

Limits: no database, broker, network, or clock fault is exercised. `RUST-DOC-0010-R014`,
`RUST-DOC-0010-R015`, and `RUST-DOC-0010-R016` are supported by argument and review gates, not by
executed evidence in this repository. The evidence map records this rather than implying coverage.

## Source provenance

- An internal working document supplied the mechanism and a catalogue of failure modes. Its scope
  claims are narrowed, its scoring rubric and its "code is the contract" claim are rejected, and
  its coined vocabulary is recorded as local. Details are in
  `sources/0010-staged-protocols/source-notes.md`.
- The Rust Reference and the Rust Book are accepted for associated items, trait bounds, move
  semantics, visibility, and monomorphization.
- Strom and Yemini, and Aldrich and colleagues, are cited for the typestate and
  typestate-oriented-programming families. Honda and colleagues are cited for session types as a
  related but distinct formalism.
- PostgreSQL documentation is accepted for multiversion concurrency behavior, which is the
  mechanical basis for the durable-advancement limit. Gray and Cheriton are cited for
  time-bounded distributed authority, consistent with RFC-0001.
- The conversion prohibition, the topology-assertion requirement, the bound-widening prohibition,
  per-stage effect disclosure, the undetermined-outcome requirement, the per-stage ledger row, and
  the complete gate set are repository governance additions.

## Decision record

- Decision: accepted
- Date: 2026-08-04
- Decision owners: doctrines-rust maintainers
- Rationale: The corpus governed the surrounding ground but could not express a checked successor
  relationship, and three consequential failures, edge deletion by refactor, stage manufacture by
  conversion, and a local move read as a durable one, had no owning rule. The mechanism is narrow
  enough to state precisely and consequential enough to enforce.
- Conditions: Preserve all existing rule identifiers and exceptions. Keep the typed protocol
  scoped to one in-process pass. Ship the topology assertion and the compiler-rejection evidence
  with the doctrine rather than after it. Record in the evidence map which rules have no executable
  evidence here. Regenerate bundles and pass the complete validation set on both toolchains.
- Supersedes / superseded by: none
