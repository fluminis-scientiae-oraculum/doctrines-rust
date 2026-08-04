# Changelog

All notable changes are documented here. Repository releases follow semantic versioning while
the corpus is pre-1.0: patch releases preserve normative meaning, minor releases may add
compatible normative requirements, and major releases may change doctrine contracts.

## [0.4.0] — 2026-08-04

### Changed in 0.4.0

- Restated RUST-DOC-0010-R022. It keeps its identifier and its non-waivable
  status, and now states an authority partition rather than a blanket
  governance precedence: the executable protocol is authoritative for the
  ordering, successor constraints, construction restrictions, and negative
  capabilities it mechanically enforces; external systems are authoritative for
  durable and remote facts; governing records are authoritative for rationale,
  non-guarantees, waivers, and change authority; and no artifact is maintained
  as a competing source for another's class of claim. The previous text
  contradicted RUST-DOC-0010-R018 and RUST-DOC-0010-R019 in the same package.
  RUST-DOC-0010 advances to 0.2.0.
- Corrected the source provenance for RUST-DOC-0010. The earlier notes recorded
  the originating document as claiming "code as sufficient contract" and
  rejected it; the document had made no such claim. The corrected
  classification records executable authority as accepted and refined, records
  the source's stronger last-resort stance on decision records, and states the
  earlier error in place rather than deleting it.
- Made CT³ visible where the mechanism is taught. `patterns/successor-capabilities.md`
  now states the local name and explains what each of its words carries,
  including the containment that "telescopic" names; the package glossary entry
  carries the same explanation. The term remains local vocabulary with its
  established families attributed, per RUST-DOC-0010-R021.
- Added the authority-partition gates S58, S61, and S62 to the RUST-DOC-0010
  review standard, and a second failure direction to its rationale and
  anti-pattern catalogue.
- Selected RUST-DOC-0011 for every agent pack, added the new review procedure to
  five of them, and gave each role file its obligation-placement section.

### Added in 0.4.0

- RUST-DOC-0011 "Executable Narrative and Minimal Decision Records", an
  eleventh doctrine governing where an architectural obligation lives, which
  artifact settles which class of claim, how a derived view is kept from
  drifting, and when a manually maintained decision record earns its permanent
  cost. It adds twenty rules covering claim classification, executable
  authority, the prohibition on a competing manually maintained copy,
  generation over synchronization, the decision-record last-resort test and
  lifecycle, the historical-veto prohibition, rationale honesty, external
  authority, representation counting, and agent hydration.
- A forty-gate review standard for the new doctrine, and the
  `reviews/executable-narrative-review.md` procedure.
- The `patterns/executable-narrative.md` pattern guide.
- A `decisions/` tree holding the record template and two worked examples: one
  decision whose residue justifies a narrow record, and one that should not be
  written at all. Both are illustrations and are absent from the registry.
- `manifest/decision-records.yaml` and its Draft 2020-12 schema, enumerating the
  active and archived record sets. The active set is empty, which is the outcome
  RUST-DOC-0011-R006 predicts for this repository.
- Six `doctrine-lint` checks for the registry, each with a unit test: owner,
  revalidation trigger, obsolescence condition and resolvable executable
  authorities on an active record; the directory each status is filed under;
  identifier uniqueness; the archival marking; and the prohibition on an agent
  pack hydrating an archived record. Each check was observed failing on a seeded
  violation before being accepted as evidence.
- RFC-0003 recording the accepted decision, including the previous position, the
  corrected position, and the reason the earlier wording was wrong.
- RFC-0002 and RFC-0003 in the accepted-RFC index, which had omitted RFC-0002.

## [0.3.0] — 2026-08-04

### Added in 0.3.0

- RUST-DOC-0010 "Staged Protocols and Successor Capabilities", a tenth doctrine
  governing in-process multi-stage protocols whose capabilities expose their
  legal successor as an associated type bounded by the next capability. It adds
  twenty-two rules covering successor bounds, branch and recovery edges,
  per-stage failure identity, construction bypass, effect disclosure, stage
  granularity, erasure boundaries, and the limit at which a local transition
  stops being evidence of a durable one.
- A fifty-eight gate review standard for the new doctrine, and a
  staged-protocol gate group in the typestate review procedure.
- The `patterns/successor-capabilities.md` pattern guide.
- A `registration-onboarding` case study covering two entry paths, an expiring
  availability observation, branch and recovery edges, and the durable limit.
- The `staged-protocol` example crate with ten unit tests and an executable
  topology assertion that pins every documented protocol edge, plus three
  compiler-rejection cases for stage skipping, consumed-stage reuse, and stage
  evidence forgery.
- RFC-0002 recording the accepted decision to add the doctrine.

### Changed in 0.3.0

- Selected RUST-DOC-0010 for the planner, implementer, reviewer, and auditor
  agent packs, and cross-referenced the new pattern from RUST-DOC-0001,
  RUST-DOC-0003, RUST-DOC-0004, RUST-DOC-0005, and RUST-DOC-0006. Their
  normative text is unchanged.
- Updated the repository evidence map for 187 normative rules, 40 unit tests,
  and nine compiler-rejection cases, and recorded which of the new rules have
  no executable evidence in this repository.

### Fixed before release in 0.3.0

Review of the initial RUST-DOC-0010 revision found five defects in the shipped
package. All were corrected before merge:

- `RUST-DOC-0010-R005` now makes non-duplicability part of the consumption
  obligation. Nonterminal stages previously derived `Clone`, so a caller could
  copy a stage and advance every copy while every consuming signature remained
  satisfied. Added a compiler-rejection case for duplication and gate S59.
- `RUST-DOC-0010-R019` now requires a contract assertion that derives the
  successor capability from the stage capability alone. The original assertions
  restated the bound themselves, so deleting `type Next: CheckIdentity` from a
  capability left the suite green.
- `RUST-DOC-0010-R007` now applies to fallible transitions only, and forbids
  declaring a failure type that is never constructed. `prepare_persistence` is
  infallible and returns its successor directly, consistent with
  RUST-DOC-0001-R013. Added gate S60.
- The conflict-resolution edge preserves origin evidence. A revised submission
  is now parameterized by the original origin, so an invited attempt stays
  invited instead of being reported as self-service under a fabricated
  challenge identifier.
- The registration case study carries a guarantee-ledger row per stage, as
  `RUST-DOC-0010-R020` and gate S55 require, including duplicability and the
  recovery stages.

## [0.2.0] — 2026-07-27

### Changed in 0.2.0

- Strengthened RUST-DOC-0005-R009 so concurrency designs name the anomaly class
  they prevent and every residual anomaly class they permit; added isolation
  terminology, a mechanism map, and a critical review gate.
- Strengthened RUST-DOC-0006-R014 so time-based authority defines its clock
  source, clock kind, timing bounds, and behavior when assumptions fail.
- Clarified RUST-DOC-0004-R004 coverage of poisoning and non-poisoning lock
  APIs, including the pinned toolchain's experimental `std::sync::nonpoison`
  module.
- Documented the deliberate expanded-versus-tabular review procedure policy.

### Added in 0.2.0

- A repository-level evidence map relating all 165 normative rules to current
  executable evidence classes and known gaps.
- An isolated, panic-safe `MaybeUninit` array initializer with a local unsafe
  lint exception, five unit tests, a safety argument, and pinned-nightly Miri
  CI.
- RFC-0001 recording the accepted normative isolation and time-assumption
  changes.

## [0.1.0] — Initial doctrine corpus

### Added

- Shared foundations for normative language, invariant classification, evidence levels, trust
  boundaries, guarantee honesty, and type-system complexity budgeting.
- Nine active doctrine packages: invalid states, error modeling, ownership and capabilities,
  concurrency and async, persistence boundaries, distributed uncertainty, unsafe Rust,
  testing and evidence, and performance measurement.
- Pattern guides for sum types, opaque newtypes, smart constructors, typestate, capabilities,
  consuming transitions, validated collections, hybrid state machines, and explicit
  uncertainty.
- Boundary guides for Serde, database decoding, HTTP/RPC, messaging, configuration,
  filesystems, and FFI.
- Operational review procedures covering design preparation, domain models, boundaries,
  typestate, distributed effects, and final correctness audits.
- Shared, planner, implementer, reviewer, auditor, and maintainer agent role contracts with
  deterministic generated hydration packs.
- End-to-end invoice, payment, transaction, message delivery, authenticated session, and UI
  workflow case studies.
- Stable Rust examples for domain modeling, validated newtypes, typestate, boundary
  validation, and distributed outcomes, plus compiler-rejection tests.
- Machine-readable doctrine and agent manifests with Draft 2020-12 schemas.
- A substantive doctrine linter, deterministic bundle generator, RFC governance, reusable
  authoring templates, and source-provenance packages.
- Generated full, compact, and role-specific distributions with drift detection.
- Read-only GitHub workflows for doctrine validation, stable/MSRV example testing, and
  Markdown formatting, linting, and link checking; structured issue and pull-request forms.
- Reproducible Markdown quality tooling with pinned Node.js, Prettier, and markdownlint-cli2
  versions, a patched transitive override, deterministic formatting policy, advisory audit,
  and a distinct pull-request gate.
- Dual licensing: CC BY 4.0 for documentation and MIT OR Apache-2.0 for code.
