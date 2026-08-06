# Changelog

All notable changes are documented here. Repository releases follow semantic versioning while
the corpus is pre-1.0: patch releases preserve normative meaning, minor releases may add
compatible normative requirements, and major releases may change doctrine contracts.

## [0.6.0] — 2026-08-06

A declared field that enforced nothing became real, and the corpus became navigable.
`manifest/agents.yaml` has always declared a `maximum_verbosity` per agent pack; the
bundler had never read it, and the measured pack sizes ran contrary to the order it
declared, with the pack labelled most focused thirty-five percent larger than one
labelled less focused. RFC-0005 gives the field an effect and reserves its widest
tier, which is what lets the corpus gain reading aids without any generated pack
growing. No normative rule changes, so this is a minor release for distribution
restructuring rather than for added obligations.

### Added in 0.6.0

- A verbosity annotation for a Markdown section, parsed once in `doctrine-manifest`
  and used by both tools. A generated output receives a section only when its ceiling
  is at least the annotation's tier, and every annotation is stripped from generated
  output. Comments inside fenced code are ignored, so a document can show the syntax
  without the example being read as an instruction.
- An `## Assembly` note in every filtered distribution, naming the ceiling applied,
  where it is declared, and how much it withheld. It is emitted even when nothing was
  withheld, so a reader can distinguish that from a forgotten disclosure. Each
  withheld run is replaced in place by a receipt naming the sections and their tiers.
- `doctrines/map.md`, a generated coverage map showing which doctrine each role pack
  hydrates. It is the transpose of six `doctrine_selections` lists, which the corpus
  carried but no document presented.
- Breadcrumb navigation on the fifty case-study and source-note leaves, which
  previously had no path to their own index, and live links throughout the root
  README reading paths and the package `Related material` sections.
- Orientation pages for twenty-one directories that held content and no index:
  each case study, each source package, `templates/`, `manifest/`, and
  `manifest/schema/`. They carry only what is not stated elsewhere — the
  subject, an ordered path through the directory, and a pointer to the single
  description of what each file holds — because the four-file arc and the
  provenance file roles are each already described once, and repeating them per
  directory is the duplication `RUST-DOC-0011-R017` asks a review to remove.
  None reaches a generated pack: no manifest references these directories, so
  they are reader-facing by location rather than by annotation.
- A closed callout vocabulary. An alert marks a distinction the corpus already draws
  rather than restating a claim, and `doctrine-lint` rejects any other alert name.

### Changed in 0.6.0

- The `auditor` pack moves from `exhaustive` to `detailed`. The widest tier is now
  reserved from every pack: a pack at that ceiling withholds nothing by definition, so
  it would absorb the whole of any canonical growth and get strictly larger.
- Double quotes are normalized to the corpus majority and eighteen em dashes gain the
  spacing the other two hundred and seventy-eight already use. Shell blocks under
  `agents/` are fenced as `bash` rather than `text`.

### Fixed in 0.6.0

- Agent overlays are obligation documents and are now untierable. An earlier
  classifier covered only doctrine normative files, the foundations, and the
  review checklists, so an annotation under `## Boundary obligations` in
  `agents/shared.md` was accepted by both tools and replaced those rules with a
  withholding receipt in every role pack. Because the widest tier is reserved
  from every pack, the obligation then reached none of them. Found in review.
  The consequence is stated rather than absorbed: every source any role pack
  lists now falls into an untierable class, so the ceiling removes nothing from
  a role pack today, and a test asserts that against the real manifests.
- The coverage map listed only active doctrines while `build_role_pack` hydrates
  any selected doctrine regardless of status, so a deprecated but selected
  doctrine would vanish from the map while the pack carrying it was unchanged.
  The map now covers every doctrine that is active or selected, and states each
  one's status.

- The per-doctrine rule counts in `EVIDENCE.md` were eleven machine-derivable integers
  maintained by hand with nothing checking them. `check_stated_counts` could not see
  them, because it only matches an integer immediately before one of three literal
  phrases and these sit alone in a table cell. They are recomputed and compared now.

## [0.5.0] — 2026-08-06

The corpus's own Rust code was audited against the corpus. The defects below are
the divergences that reproduced. Writing up the first of them exposed a defect in
the doctrine rather than the code: no rule could be cited against it. RFC-0004
amends `RUST-DOC-0001-R002` accordingly, which moves that doctrine to `0.2.0` and
makes this a minor release rather than a patch.

### Changed in 0.5.0

- `RUST-DOC-0001-R002` gains a second obligation. Its applicability named string
  discriminants while its statement reached only contradictory field combinations
  carrying state-specific data, so a lone `status` field over four unit states
  satisfied the applicability and the statement asked nothing of it. The rule was
  applicable and vacuous, which is why the manifest defect below could not be
  reported against it. A single field whose value selects among a closed, known
  set of mutually exclusive alternatives must now be decoded into a type that
  cannot hold a value outside that set. A new allowed exception routes a
  vocabulary too large or too volatile to enumerate to a validated newtype whose
  rejection of an unknown value is tested, so the rule does not require
  enumerating every constrained string.

  The rule keeps its identifier, title, and position, so existing citations
  resolve. No rule is added, removed, or weakened, and the corpus keeps 207
  normative rules. A system whose closed vocabulary reaches the domain as an
  unconstrained scalar was conforming before and is not conforming now, which is
  the minor version change. See `rfcs/accepted/RFC-0004-closed-vocabulary-discriminants.md`.

### Fixed in 0.5.0

- `bundle-agent-context` decoded the manifest `status` field as a `String` and
  never validated the manifest against `manifest/schema/doctrine.schema.json`,
  which constrains it to four values. A misspelled status therefore matched no
  filter and silently excluded that doctrine from `dist/full-doctrine.md`,
  `dist/compact-doctrine.md`, and every agent pack, while `generate` reported
  success and exited zero. Seeding `status: activ` on the first entry removed
  1901 lines from the bundles without a diagnostic. The vocabulary is now an
  enum decoded once, so the same input fails to parse in every consumer. This
  is `RUST-DOC-0001-R006`: deserialization must not bypass documented
  validation.
- `doctrine-lint` restated two schema-owned vocabularies in Rust: the six agent
  role identifiers and the four `maximum_verbosity` values, each a hand-written
  copy of an `enum` array in `manifest/schema/agent-pack.schema.json` that the
  same run had already validated. Both copies are removed; the values decode
  into types, and three tests assert those types against the schema files, so a
  vocabulary change fails the build instead of drifting. This is
  `RUST-DOC-0011-R017` applied to the tool that enforces it elsewhere.
- The manifest types, the status vocabularies, and the front-matter parser were
  each declared twice, once per tool, because the two binaries shared no
  library. The two `front_matter` implementations had already diverged, and the
  copy specialized to say "README" was being used to report malformed decision
  records. All of it now lives in a new `doctrine-manifest` crate.
- `doctrine-lint` treated a directory it could not read as an empty directory.
  An unreadable repository root would have scanned no root documents and let
  every root-document check pass while still reporting the repository valid.
  Directory walks now report a directory that exists and cannot be
  enumerated; an absent directory stays silent, because absence is genuinely
  empty and is already reported against its manifest entry.
- `bundle-agent-context` validated its command argument and then matched the
  raw string a second time, needing an `unreachable!` arm to restate a
  guarantee the first check had established. The argument is parsed into a
  `Command` and the dispatch is exhaustive.
- `unsafe-evidence` declared its escape hatch from the workspace's
  `unsafe_code = "forbid"` twice, in `Cargo.toml` and as an inner attribute.
  The manifest declaration is kept and named as the scoped exception under
  `RUST-DOC-0001-R016`.
- `RfcMetadata.status` was the one closed vocabulary still decoded as a string
  after the sweep above. It is now a type, checked by test against the `rfcs/`
  state directories that own the lifecycle. This is the amended
  `RUST-DOC-0001-R002` applied to the corpus's own code, so the repository
  conforms to the rule it publishes in the same release that publishes it.
- `doctrine-lint` skipped a file it could not read during the repository-wide
  forbidden-marker scan. Only directory-enumeration failures were reported, so a
  single unreadable file left `check` exiting zero and calling the repository
  valid. The read is now reported. Content that is not UTF-8 stays silent, since
  the markers are Markdown filler and a binary file carries none.
- `bundle-agent-context` turned a directory it could not enumerate into an empty
  list, and discarded per-entry read failures. With `rfcs/accepted` unlistable,
  `generate` rewrote the accepted-RFC index from its three rows to "no accepted
  RFCs" and exited zero. The walk is now fallible and aborts generation. This is
  the same class as the `doctrine-lint` fix above, which had been applied to only
  one of the two tools.
- Both tools classified directory entries with `Path::is_dir` and `Path::is_file`,
  which follow symbolic links and report `false` for a metadata error. A link was
  therefore treated as whatever it pointed at, and a classification failure was
  indistinguishable from an entry that was neither. A symbolic link added under
  `rfcs/accepted` made `generate` publish a second index row for a target the
  repository does not contain, and exit zero. Entries are now classified with
  `DirEntry::file_type`, a link is reported and not followed, and a classification
  failure is reported. The linter also reported no per-entry read failure, which
  `filter_map(Result::ok)` discarded; those are reported too.
- `doctrines/0001-invalid-states/README.md` stated "All rules are version `0.1.0`
  and active", which the `R002` amendment made false and which bundle generation
  propagated into `dist/full-doctrine.md`. The sentence restated a version the
  file's own front matter and the manifest already carry, so it is replaced with
  the stable-identifier wording the other doctrine READMEs use, which names no
  version and cannot go stale. That is `RUST-DOC-0011-R004` applied to a package
  README.
- `rfcs/accepted/RFC-0004-closed-vocabulary-discriminants.md` carried a prose
  paragraph where `rfcs/template.md` requires a decision record with decision,
  date, decision owners, rationale, conditions, and supersession. The record now
  states all six, so the acceptance authority for a normative change is on the
  record rather than implied.

## [0.4.1] — 2026-08-05

Documentation coherency only. No normative rule, statement, allowed exception,
or doctrine version changes.

### Fixed in 0.4.1

- The root `README.md` doctrine index listed nine doctrines while claiming to be
  synchronized with `manifest/doctrines.yaml`, which by then held eleven. That
  table was a third representation of a fact the manifest owns and
  `doctrines/README.md` already presented, and it also propagated into
  `dist/full-doctrine.md`. The duplicate is removed; the root README now links to
  the doctrine index instead of repeating it.
- `doctrine-lint` now checks the surviving doctrine index against the manifest:
  every active doctrine needs a row naming its identifier and its exact manifest
  title on one line, and the index may not list a doctrine the manifest does not
  carry as active. A prose cross-reference does not satisfy the row requirement,
  which is how the shipped index concealed a missing row.
- The root `README.md` claimed repository version `0.2.0`. It now names no
  version number at all and points at the manifest, so the same defect cannot
  recur there.
- `doctrines/README.md` said the repository release and every doctrine were at
  `0.1.0`; repository and doctrine versions have since diverged.
- The generated-file set was described as `dist/` alone in `README.md`,
  `CONTRIBUTING.md`, `agents/shared.md`, and the pull-request template, although
  `rfcs/accepted/README.md` became generated in 0.4.0. All four now name both,
  and the documented drift-gate command covers both paths.
- `decisions/` was absent from the canonical-root list and the architecture
  section of `README.md`.
- The reading paths in `README.md` did not mention RUST-DOC-0011, the executable
  narrative review, or the maintainer's decision-record obligations.
- `doctrines/0004-concurrency-and-async/README.md` referred to "the 0.1.0
  workspace".
- `EVIDENCE.md` was absent from the linter's root-document list, so it had never
  been scanned for normative-term scope, and the new counted-claim check silently
  skipped the corpus document carrying the most derived numbers. Every Markdown
  file at the repository root is now scanned.

### Added in 0.4.1

- A counted-claim check. Prose stating a number of normative rules, doctrine
  packages, or active doctrines is compared against the recomputed corpus, so a
  hand-maintained number cannot silently disagree with the manifest.
- A rule-citation check. Every `RUST-DOC-NNNN-RNNN` cited in maintained canonical
  Markdown must resolve to a rule a doctrine defines. The corpus already required
  each rule to appear in its own review standard; nothing checked the reverse, so
  a renamed or removed rule left dangling citations that read as authoritative.
- Both checks exempt dated records, which state the contract as it stood when a
  decision was taken and are not maintained afterwards, as
  `RUST-DOC-0011-R011` and `RUST-DOC-0011-R019` permit. Rewriting them to satisfy
  a linter would destroy the record. The exemption follows artifact lifecycle
  rather than directory membership: the RFC documents themselves in any lifecycle
  state, archived decision records, and `CHANGELOG.md`. Maintained governance
  around them stays scanned, including `rfcs/README.md`, the state-directory
  READMEs, and `rfcs/accepted/overview.md`, which cites live rule identifiers.
- A duplicated-validation-sequence check. The local validation sequence was
  carried in full by `README.md`, `AGENTS.md`, and the pull-request template at
  once, so a change to any gate had to be made in three places correctly with
  nothing announcing a miss. `README.md` now owns it, the other two link to it,
  and `doctrine-lint` counts fenced blocks carrying three or more validation
  commands and requires exactly one, in the owner.
- Root Markdown files are discovered from the directory rather than listed in a
  constant, so a future root document is covered on the day it is added rather
  than when someone remembers the inventory.

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
- A generated accepted-RFC index. `rfcs/accepted/README.md` is now produced by
  `bundle-agent-context` from `rfcs/accepted/overview.md` and each RFC's own
  front matter, and is drift-checked; it had omitted RFC-0002 while
  hand-maintained.

### Fixed before release in 0.4.0

Review of the initial revision found three defects. All were corrected before
merge:

- The decision-record registry duplicated the metadata each record already
  carries in its front matter, and the linter validated only the registry row.
  A record could omit or contradict its own owner, scope, authorities, or
  triggers while the build stayed green, which both overstated the guarantee and
  created the competing copy RUST-DOC-0011-R004 prohibits, inside the
  enforcement for that rule. The registry now enumerates membership only; the
  linter opens each listed record and validates RUST-DOC-0011-R007 from its
  front matter. The one overlapping fact, list membership versus declared
  `status`, is compared rather than trusted, and a mismatch test covers it.
- The accepted-RFC index was left hand-maintained on the stated ground that a
  generator would need another hand-written list. That reason was wrong: the
  directory supplies the file set and each RFC's front matter supplies its
  identifier, title, and status, so the view is generated with no competing
  source.
- RFC-0003 said ten existing rule statements were preserved. RUST-DOC-0010 has
  twenty-two rules and only R022 changes, so twenty-one are preserved in that
  package and 165 in the nine others.

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
