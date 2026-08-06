<!--
GENERATED FILE. DO NOT EDIT DIRECTLY.
Canonical sources live under /foundations, /doctrines, /patterns,
 /boundaries, /reviews, and /agents.
-->

# Full Rust doctrine corpus

## Assembly

Ceiling `exhaustive`, the widest the schema declares, applied by `bundle-agent-context`. A section annotated above that ceiling is withheld here. Nothing was withheld at this ceiling.

Obligations are never withheld. A doctrine's normative file, every foundation, every agent overlay, and every review checklist carry no annotation, and generation rejects one. Canonical sources carry every section, and `dist/full-doctrine.md` carries the corpus with no ceiling applied.

---

## Source: `README.md`

# doctrines-rust

`doctrines-rust` is the canonical Rust engineering doctrine corpus for AI agents and human
engineers. It emphasizes invariants, evidence, correctness boundaries, explicit uncertainty,
and proportionate use of Rust's type system. The corpus is designed to guide planning,
implementation, review, audit, and long-term maintenance without reducing engineering quality
to successful compilation, avoidance of `unwrap`, or a green test run.

Rust prevents broad classes of memory errors, but a memory-safe program can still encode an
impossible invoice, forge authority, skip validation while decoding a database row, retry an
ambiguous payment twice, deadlock under load, expose an unsound safe API, or make an
unmeasured performance claim. This repository treats those failures as design concerns with
named owners, explicit mechanisms, and reviewable evidence.

## Audience

The primary readers are AI planning, implementation, reviewing, auditing, and maintenance
agents; Rust architects and developers; code reviewers; and system designers. Documents use
stable identifiers, repeatable package anatomy, machine-readable manifests, and operational
checklists so an agent can load only the context required for its role. The same material
remains ordinary Markdown that a human can inspect, challenge, and revise through review.

Readers are expected to know basic Rust syntax. The corpus explains why a representation is
chosen, what it proves, how its construction is protected, and where runtime uncertainty
remains. It does not attempt to teach ownership syntax or replace the Rust language
documentation.

## Non-goals

This repository is not:

- a beginner Rust tutorial or a general-purpose Rust book;
- a crate recommendation catalogue;
- a formatting guide that duplicates `rustfmt`;
- a catalogue of Clippy lints;
- an unsafe-code cookbook;
- a prompt archive;
- a collection of clever type tricks;
- or a claim that compile-time representation can freeze mutable external reality.

Libraries appear when they support executable evidence or repository tooling. Their presence
is not a universal endorsement. Examples are deliberately bounded: an example email parser
establishes its documented syntax policy, not deliverability; integer money avoids binary
floating-point representation error, not tax, foreign-exchange, allocation, or rounding
policy.

## Core thesis

Rust engineering quality is not merely memory safety, successful compilation, avoidance of
`unwrap`, or passing tests. A design must discover important invariants, encode the invariants
that are structurally enforceable, constrain legal states and transitions, preserve validation
at trust boundaries, model external failure honestly, represent distributed uncertainty
explicitly, and keep type-system complexity proportional to the risk it removes.

The repository therefore starts before code. It asks which states are meaningful, who owns
each fact, where representations enter from less-trusted systems, which transitions are under
local control, and which observations can become stale. It then chooses among runtime enums,
opaque validated values, consuming transitions, typestate, capabilities, transactional
validation, ordinary runtime rules, and explicit reconciliation states. Typestate has no
privileged rank: persisted, heterogeneous, externally determined, or runtime-inspected state
usually belongs in an enum or a runtime state machine.

The reasoning lifecycle is:

```text
requirements
    ↓
invariant discovery
    ↓
invariant classification
    ↓
trust-boundary identification
    ↓
representation choice
    ↓
legal construction and transition design
    ↓
external failure and uncertainty modelling
    ↓
executable evidence
    ↓
review and guarantee audit
```

Skipping a stage creates predictable blind spots. Starting with structs can preserve
contradictory fields. Starting with typestate can encode a local protocol while ignoring
persistence. Starting with tests can exercise an API whose public constructors already
permit invalid values. Starting with retry policy can collapse an unknown external outcome
into a fictional failure.

## Repository architecture

Canonical content has distinct responsibilities:

- `foundations/` defines shared language for invariants, evidence, boundaries, guarantees,
  and complexity.
- `doctrines/` contains versioned normative packages. Every normative rule has a stable
  `RUST-DOC-####-R###` identifier, intent, applicability, exception policy, and expected
  evidence.
- `patterns/` describes reusable representation choices, their exact guarantees, their
  limitations, and their costs.
- `boundaries/` explains how untrusted HTTP, database, message, configuration, filesystem,
  Serde, and FFI representations become trusted domain evidence.
- `reviews/` contains pass/fail/not-applicable/waiver-oriented procedures.
- `agents/` provides role overlays that select canonical doctrine instead of manually
  duplicating it.
- `case-studies/` follows complete problems from weak representations through improved
  designs and residual uncertainty.
- `examples/` provides positive tests, compiler-rejection evidence, and an
  isolated unsafe abstraction checked under Miri.
- `sources/` records provenance, accepted ideas, refinements, and doctrine additions.
- `decisions/` holds the architecture decision records that survive the last-resort test in
  RUST-DOC-0011, with their template and worked examples. The active set is currently empty.
- `templates/` and `rfcs/` govern future doctrine work.
- `manifest/` exposes doctrine and agent-pack discovery through YAML validated against Draft
  2020-12 JSON Schemas.
- `tools/` contains the doctrine linter, the deterministic context bundler, and the
  `doctrine-manifest` crate both decode the manifests through, so one schema-owned
  vocabulary has one decoder rather than one per tool.

Definitions flow into doctrine; doctrine constrains patterns, boundary guides, reviews, and
agent work. A case study may demonstrate doctrine but cannot silently redefine it. Source
notes explain provenance but are non-normative. Executable examples provide evidence for
specific claims without becoming the only expression of a rule.

[`EVIDENCE.md`](../EVIDENCE.md) inventories the repository's current executable
evidence by doctrine and names the material gaps that remain.

## Canonical and generated content

Canonical doctrine lives under `foundations/`, `doctrines/`, `patterns/`, `boundaries/`,
`reviews/`, `agents/`, `case-studies/`, `decisions/`, `templates/`, `rfcs/`, and `sources/`.

Three kinds of file are generated and are never edited by hand:

- everything under [`dist/`](README.md), the hydration bundles projected from canonical
  sources;
- [`rfcs/accepted/README.md`](../rfcs/accepted/README.md), the accepted-RFC index, built from
  `rfcs/accepted/overview.md` and the front matter of each accepted RFC;
- [`doctrines/map.md`](../doctrines/map.md), the doctrine coverage map, built from
  `doctrines/map-overview.md` and the two manifests.

Every generated file carries a banner naming the sources it was built from. After a canonical
change, run:

```bash
cargo run -p bundle-agent-context -- generate
cargo run -p bundle-agent-context -- check
```

`check` reconstructs expected bytes in memory and fails if any generated file differs, whether
or not it lives under `dist/`. This separation permits compact agent loading, and a current
index, without creating a second manually maintained doctrine.

## Reading paths

The role packs under `agents/` state exact required inputs and outputs. The generated versions
under `dist/agents/` combine those overlays with selected canonical rules.

- A planner begins with [`foundations/invariants.md`](../foundations/invariants.md),
  [`foundations/trust-boundaries.md`](../foundations/trust-boundaries.md),
  [`foundations/complexity-budget.md`](../foundations/complexity-budget.md),
  [`agents/planner.md`](../agents/planner.md), and
  [`reviews/pre-implementation.md`](../reviews/pre-implementation.md). Its output is an invariant
  inventory, boundary map, state graph, authority map, effect and uncertainty inventory,
  persistence choice, complexity decision, and evidence plan.
- An implementer reads the relevant doctrine package,
  [`agents/implementer.md`](../agents/implementer.md), the applicable
  [boundary guide](../boundaries/README.md), and the matching [example crate](../examples/README.md).
  It protects constructors, retains external fallibility, and supplies positive, negative, and
  compiler-rejection evidence where useful.
- A reviewer reads [`foundations/guarantee-honesty.md`](../foundations/guarantee-honesty.md),
  [`agents/reviewer.md`](../agents/reviewer.md), the relevant doctrine review standard, and the
  [operational review documents](../reviews/README.md). It inspects construction paths and bypasses
  rather than approving merely idiomatic syntax.
- An auditor loads [`agents/auditor.md`](../agents/auditor.md), all applicable
  [boundary guides](../boundaries/README.md), and
  [`reviews/final-correctness-audit.md`](../reviews/final-correctness-audit.md). It searches
  adversarially for forged evidence, unchecked decoding, authority leakage, unsafe retry, and
  misleading claims.
- A maintainer reads [`foundations/normative-language.md`](../foundations/normative-language.md),
  [`agents/maintainer.md`](../agents/maintainer.md), [`rfcs/README.md`](../rfcs/README.md), and the
  affected [source notes](../sources/README.md) before changing meaning, versions, manifests, or
  generated outputs. It also revalidates or expires active decision records under
  [`decisions/`](../decisions/README.md).
- Any role about to write a document first applies
  [RUST-DOC-0011](../doctrines/0011-executable-narrative/README.md) and
  [`reviews/executable-narrative-review.md`](../reviews/executable-narrative-review.md): classify
  the claim, name the single artifact authoritative for it, prefer moving an enforceable
  obligation into the mechanism that enforces it, and write a decision record only for a fact no
  artifact can carry. The most common correct outcome is that no document is added.
- A human architect normally reads all [foundations](../foundations/README.md),
  [Doctrine 0001](../doctrines/0001-invalid-states/README.md), the doctrines relevant to the
  system's risks, the decision frameworks, and one structurally similar
  [case study](../case-studies/README.md).

> [!TIP]
> [`doctrines/map.md`](../doctrines/map.md) shows which doctrine each role pack hydrates. A doctrine
> a pack omits is one that is not available from that pack alone and has to be loaded from its
> canonical source; the exclusions are the interesting cells.

For constrained context windows, `dist/compact-doctrine.md` contains the shared thesis,
classification and boundary pipelines, every active doctrine's normative rules, the central
decision tree, and the core audit gates. `dist/full-doctrine.md` retains the repository identity,
foundations, every file in each active doctrine package, patterns, boundary guides, operational
reviews, and shared agent obligations in stable order. Case studies, source notes, RFCs,
templates, and role-specific overlays remain canonical but are deliberately outside that
hydration bundle.

## Doctrine index

`manifest/doctrines.yaml` is the machine-readable discovery source, and
[`doctrines/README.md`](../doctrines/README.md) is the reader-facing index derived from it. This
file deliberately does not repeat that table. It previously did, and the copy was wrong within
one release: two doctrines were added and the table still listed nine while claiming to be
synchronized with the manifest. `doctrine-lint` now checks the surviving index against the
manifest, so a doctrine cannot be added without the index following.

That is `RUST-DOC-0011-R004` and `RUST-DOC-0011-R017` applied to this repository's own front
page: one authority, one mechanically checked view, and no third copy to keep in step.
[`doctrines/map.md`](../doctrines/map.md) is a second view of the same manifests, and it is
generated for the same reason rather than written.

The corpus does not claim `1.0` semantic stability, and doctrines version independently of the
repository and of each other. Patch releases clarify without changing normative meaning; minor
releases add normative requirements or substantial compatible material; major releases may make
normative contracts incompatible. The current repository version is `repository_version` in
`manifest/doctrines.yaml`, which `doctrine-lint` holds equal to the workspace package version;
per-doctrine versions and status live in the same manifest and in each package's front matter.
This file names no version number, for the same reason it no longer repeats the index.

## Local validation

The pinned development toolchain is Rust 1.97.1. The workspace MSRV is Rust 1.85.0, the first
stable release supporting Edition 2024; selected dependencies declare compatibility with that
floor. Markdown tooling uses the exact Node.js, Prettier, and markdownlint-cli2 versions pinned
by `.node-version` and `package-lock.json`. Run from the repository root:

```bash
npm ci --ignore-scripts --no-audit
npm audit --audit-level=high
npm run check:markdown-format
npm run lint:markdown
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p doctrine-lint -- check
cargo run -p bundle-agent-context -- generate
git diff --exit-code -- dist/ rfcs/accepted/README.md doctrines/map.md
cargo run -p bundle-agent-context -- check
cargo deny check
lychee --no-progress '**/*.md'
git diff --check
```

Run `npm run format:markdown` to format canonical and repository-governance Markdown before
regenerating bundles. Prettier deliberately ignores every generated file, `dist/` and the
accepted-RFC index alike; generated Markdown changes only through `bundle-agent-context`. The
workspace test command includes the `trybuild` compile-fail suite. CI exposes Markdown
dependency audit, format, and lint as a distinct pull-request gate,
then repeats Rust formatting, Clippy, tests, schema validation, bundle drift detection,
dependency policy, and link checks with read-only repository permissions. CI confirms locally
discovered behavior; it is not the first compiler, linter, or formatter.

## Contributing and evolution

Read `CONTRIBUTING.md` and repository `AGENTS.md` before changing content. Corrections that
preserve normative meaning may use an ordinary pull request. New doctrines, normative rule
additions or weakening, new escape hatches, supersession, normative-term changes, significant
bundle restructuring, license changes, and MSRV policy changes require an RFC under `rfcs/`.

Every normative change identifies affected rule IDs, updates manifests and source provenance,
states compatibility and migration consequences, and regenerates `dist/`. A reviewer requires
a guarantee ledger separating what a mechanism establishes from what remains unproved. If a
claim is stronger than its constructors, boundary decoding, or external evidence, use the
guarantee-overclaim issue form.

## License

Documentation — including Markdown doctrine, manifests, schemas, and generated doctrine
bundles — is licensed under Creative Commons Attribution 4.0 International; see
`LICENSE-DOCS`. Rust source, test fixtures, repository tools, and reusable workflow or
configuration code are available under MIT OR Apache-2.0 at the recipient's option; see
`LICENSE-CODE`. External quotations and linked source material remain subject to their
respective rights.

---

## Source: `foundations/README.md`

# Foundations

Foundations define the vocabulary and reasoning contracts used by every doctrine in this
repository. They are separate because a doctrine should not privately redefine "invariant,"
"evidence," "trusted," "guarantee," or the force of `MUST`. A shared definition lets rule IDs
compose across domain modeling, errors, persistence, distributed effects, unsafe code,
testing, and performance.

## Dependency direction

The dependency direction is intentionally one-way:

```text
foundations
    ↓
normative doctrines
    ↓
patterns and boundary guides
    ↓
review procedures and agent workflows
    ↓
case studies and executable evidence
```

A foundation describes concepts and interpretation. A doctrine imposes requirements using
those concepts. A pattern presents a reusable mechanism that may satisfy one or more
requirements. A boundary guide specializes doctrine for a place where representations or
authority cross systems. A review procedure asks for evidence. A case study demonstrates a
coherent application and records what remains uncertain.

This direction matters during disagreement. A case study cannot establish that typestate is
universally preferred merely because it uses typestate. A pattern cannot weaken a doctrine's
constructor rule for convenience. A source note cannot become normative through repetition.
If a shared definition must change, the change is governed explicitly because every
downstream document may be affected.

## Reading order

Read the foundation documents in this order:

1. `normative-language.md` defines the force and scope of requirements, informative text,
   examples, exceptions, and waivers.
2. `invariants.md` distinguishes invariants from preconditions, policies, assumptions,
   observations, and desired outcomes. It supplies the inventory that precedes representation
   choice.
3. `evidence.md` describes the evidence carried by a value or capability and requires names
   to match what construction actually established.
4. `trust-boundaries.md` maps untrusted representations through parse and validation into
   trusted domain values, then through effects into observations or explicit uncertainty.
5. `guarantee-honesty.md` requires a ledger for proofs, protected construction, boundary
   preservation, escape hatches, non-guarantees, and residual risk.
6. `complexity-budget.md` keeps structural enforcement proportional to consequence and
   prevents type machinery from obscuring the system.

The order is a reasoning sequence, not a ranking. Complexity does not override a safety
invariant, and an important invariant does not authorize an inaccurate guarantee. The design
must make a risk-owned tradeoff and preserve evidence at every boundary.

## Document classes

### Definition

A definition assigns a stable meaning to a term. Definitions are shared interpretation, not
an implementation command by themselves. For example, a transition invariant is a condition
that must hold across a permitted state change; this does not yet choose an enum, transaction,
or consuming method.

### Doctrine

A doctrine is a versioned normative contract. It contains identified requirements, intent,
applicability, permitted exceptions, and review evidence. Compliance is evaluated against
rule intent and system behavior, not a copied syntax fragment.

### Pattern

A pattern relates a recurring problem to a mechanism under stated forces. It identifies the
exact guarantee gained and not gained, boundary and persistence implications, evidence, cost,
and conditions where it should not be used. Patterns are selections, not mandates unless a
doctrine rule requires one in a defined case.

### Boundary guide

A boundary guide starts with less-trusted data, authority, or effects. It asks what is parsed,
what is validated, how trusted construction is protected, how failures and unknown values are
represented, and what remains mutable or uncertain. Boundaries include more than network
inputs: database rows, cached bytes, environment variables, filesystem state, and FFI values
are representations from outside the current domain proof.

### Review procedure

A review procedure is operational. Each item must be recorded as pass, fail, not applicable,
or a waiver reference. A prose assertion such as "validation looks adequate" is weaker than
evidence naming every constructor and decoding route. Review does not manufacture proof; it
checks whether the mechanism and evidence support the claim.

### Case study

A case study follows one domain through problem, weak model, improved model, and residual
uncertainty. It makes tradeoffs concrete without turning an example domain into a universal
rule. Its guarantee ledger must say what the design cannot establish.

## Using foundations during work

A planner begins by writing an invariant inventory and trust-boundary map. Each inventory row
names the owner, enforcement classification, failure consequence, evidence, and residual
uncertainty. The planner then chooses representation and records why a simpler or more complex
mechanism is proportionate.

An implementer traces every public construction path, decoding path, mutation path, and state
transition to the inventory. A reviewer attempts bypasses and distinguishes compiler-enforced
facts from runtime observations. An auditor checks whether type names or documentation have
silently grown stronger than their proofs. A maintainer rechecks the dependency direction
before changing a definition.

The foundations share a single discipline: a claim is not its mechanism, and a mechanism is
not evidence that it ran. `NonZeroU64` is a mechanism for excluding zero; the private field and
compiler rejection protect ordinary construction; tests demonstrate selected constructor
behavior; none of these establishes tax correctness. An external verification response may
establish evidence at one time, but it cannot guarantee that external reality remains
unchanged.

## Change policy

Changes that clarify wording without changing meaning use normal review. A change to
normative-term meaning, invariant classification, evidence interpretation, boundary trust
model, guarantee ledger obligations, or complexity policy can alter every doctrine and
requires the RFC process. The proposal must enumerate downstream contracts, compatibility,
migration, and evidence.

Foundations should remain domain-neutral but not vague. They use examples to expose
distinctions, name failure semantics, and state limitations. They avoid universal mechanisms:
structural enforcement is preferred when it removes consequential invalid programs, while
runtime validation remains necessary for external, temporal, cross-entity, and distributed
facts.

---

## Source: `foundations/normative-language.md`

# Normative language

This corpus uses five uppercase requirement terms deliberately: **MUST**, **MUST NOT**,
**SHOULD**, **SHOULD NOT**, and **MAY**. Their purpose is to make obligations, recommended
engineering judgment, and permitted choices reviewable. Casual emphasis uses ordinary
lowercase language.

The terms follow the general interpretation established by
[RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and clarified by
[RFC 8174](https://www.rfc-editor.org/rfc/rfc8174), with repository-specific governance
defined here.

## Requirement levels

**MUST** states an absolute requirement within the rule's applicability. A conforming design
satisfies it or carries an approved, explicit waiver where the rule permits waiver. Difficulty,
legacy cost, or compiler inconvenience does not silently lower the requirement.

**MUST NOT** states an absolute prohibition within applicability. An apparent workaround that
recreates the prohibited risk under another name is still nonconforming. For example, a rule
against public trusted-value construction is not satisfied by a public `from_raw` method that
performs no validation.

**SHOULD** states the recommended choice when its applicability holds. A different choice can
be conforming only when the work records a concrete reason, analyzes the resulting risk,
identifies compensating evidence where needed, and survives review. "Preference" alone is
insufficient.

**SHOULD NOT** marks a normally prohibited choice whose exceptional use requires the same
documented judgment. The exception must explain why the usual failure mode does not apply or
how another control contains it.

**MAY** grants permission or identifies a genuinely optional mechanism. It does not remove
obligations from other rules. A design MAY use typestate in a suitable local protocol, but it
must still model transition failure honestly and preserve persistence boundaries.

## Vocabulary calibration

Normative vocabulary is selected by meaning, not by a target distribution. A doctrine does
not need to contain every term. `SHOULD NOT` is appropriate only for a normally prohibited
choice that can remain conforming after an explicit risk argument; a strict prohibition with
bounded applicability instead uses `MUST NOT` and its allowed-exceptions field. `MAY` marks a
permission that would otherwise be unclear. Lowercase "may" can still describe uncertainty or
possibility without granting a new permission.

Reviewers examine whether each chosen force matches consequence and available exceptions.
They do not rebalance counts mechanically. Replacing one normative term with another can
change the set of conforming systems and therefore follows the doctrine-change process unless
the edit demonstrably preserves meaning.

## Scope and applicability

Every doctrine rule states applicability. The normative term governs only within that scope,
but applicability is evaluated by system behavior rather than file layout or labels. A
database adapter that constructs a domain value is a trusted-construction path even if it is
placed in an infrastructure crate. A background task that captures payment is an external
effect even if called from a method named `advance`.

"Not applicable" is a review result, not an omission. The reviewer records why the triggering
conditions do not exist. If the system later changes, the applicability decision must be
revisited.

The strongest applicable rule controls when rules overlap. If one doctrine recommends
structured errors and another requires an explicit unknown outcome for an ambiguous effect,
an opaque report alone cannot erase the unknown state. Conflicts between normative rules are
reported as corpus defects and resolved through governance rather than private
interpretation.

## Normative and informative material

Uppercase requirement terms in `doctrine.md` are normative. Stable rule IDs are the citation
unit. Package metadata records whether the doctrine is normative and its lifecycle state.
Repository governance contracts such as `AGENTS.md`, `CONTRIBUTING.md`, and `rfcs/README.md`
may use the same vocabulary for repository operations; those obligations are governance, not
unnumbered doctrine rules. Definition documents may mention the uppercase terms as terms.
Other informative material uses ordinary lowercase language or cites the governing doctrine
rule instead of creating a hidden obligation.

Rationale, glossary entries, source notes, anti-pattern explanation, and ordinary examples are
informative unless a normative rule explicitly incorporates them. Informative material can
clarify intent and expected evidence but cannot create a hidden obligation. Conversely, a
normative rule cannot be evaded by reading its example as the only permitted syntax.

Examples are illustrative unless marked as required evidence. An enum example teaches
mutually exclusive state; it does not require the same variant names in every domain. A
`TryFrom` example teaches checked conversion; a complete smart constructor with equivalent
protection can comply.

Compliance requires satisfying a rule's intent, not merely copying syntax. A private field
does not protect an invariant if public deserialization bypasses the constructor. A consuming
method does not prevent duplicate effects if callers can clone the authority-bearing value.
A test named after a rule is not evidence when it never exercises the violation path.

## Exceptions

A rule's "allowed exceptions" section defines conditions under which its default statement
does not apply or a reviewed deviation may be accepted. Exceptions must be narrow enough to
test. They should name the changed threat or domain assumption, not merely state that the
implementation is special.

If a rule lists no exception, ordinary review cannot invent one. A new escape hatch or
normative weakening requires an RFC. If an emergency requires a temporary deviation, the
waiver records the breach and remediation rather than pretending compliance.

## Waivers

A waiver is explicit, reviewed, scoped, time-aware, and documented. It includes:

- affected doctrine and rule IDs;
- exact code, component, boundary, or deployment scope;
- owner authorized to accept the risk;
- reason compliance is currently impracticable;
- failure consequence and affected users or systems;
- compensating controls and their evidence;
- expiration or reconsideration trigger;
- remediation or removal plan;
- reviewer and approval reference.

Silence, an inline allow attribute, a generic "legacy" label, or a passing CI job is not a
waiver. A waiver does not change the doctrine for other work. Repeated waivers can reveal that
a rule is wrong or adoption is blocked; that observation should trigger doctrine review, not
automatic normalization.

## Rule writing

A normative rule uses one stable ID such as `RUST-DOC-0001-R004` and includes:

- **Statement:** one testable obligation or tightly related contract.
- **Intent:** the failure mode or invariant protected.
- **Applicability:** the systems, paths, or conditions that trigger it.
- **Allowed exceptions:** bounded conditions or "none."
- **Review evidence:** artifacts and observations sufficient to assess it.

The applicability and review-evidence fields use capitalized noun-phrase lists consistently.
This register keeps machine extraction predictable while the statement, intent, and exception
fields carry complete propositions.

Avoid combining unrelated requirements merely to reduce rule count. Avoid vague verbs such as
"handle appropriately" without defining outcomes. Name owners and failure semantics. A rule
about timeouts should say whether the result is confirmed failure, cancellation, or
indeterminate effect; a rule about validation should name construction and decoding paths.

## Versioning consequences

A patch doctrine version may correct grammar or clarify meaning without changing the set of
conforming systems. A minor version may add normative rules or compatible obligations. A
major version may weaken, remove, or incompatibly reinterpret obligations. While the
repository is pre-1.0, changes still record their semantic category and migration impact;
pre-1.0 is not permission for silent contract change.

Normative language makes review sharper, not automatic. Judgment remains necessary for
classification, applicability, consequence, proportionality, and evidence quality. The terms
ensure that judgment is visible and accountable.

---

## Source: `foundations/invariants.md`

# Invariants

An invariant is a statement that must remain true throughout a defined scope while the system
is considered valid. Its value comes from precision: the statement identifies which states or
histories are legal, who owns the truth, where it may be established, and what evidence can
support it.

"The invoice is correct" is not a useful invariant. "A paid invoice carries a receipt issued
for that invoice" is a state invariant. "Capture can occur only after authorization" is a
transition invariant. "Only a capability created by the authorization service permits
capture" adds an authority invariant. Each can receive a different enforcement mechanism and
different evidence.

## Invariant categories

### Value invariants

A value invariant constrains one value independent of other entities at a defined time.
Examples include non-zero minor units, a bounded display name, or an identifier with a
specified grammar. Opaque newtypes and smart constructors often fit stable, local value
invariants. A type should not claim more: `NonZeroU64` excludes zero but says nothing about
currency, account balance, tax policy, or origin.

### State invariants

A state invariant constrains a whole domain state. Mutually exclusive cases should usually be
represented by an enum with variant-specific data. An invoice cannot be both pending and paid;
a paid variant can require a receipt while a failed variant requires a reason. This removes
contradictory field combinations from ordinary construction.

### Transition invariants

A transition invariant constrains movement between legal states. It may require a prior
state, evidence, authority, or atomic update. Consuming transitions can prevent local reuse;
typestate can prevent calling an operation in the wrong local phase. Neither proves an
external effect succeeded. Persisted and externally driven transitions generally need runtime
state validation and concurrency control.

### Authority invariants

An authority invariant states who or what may cause an effect. A capability value can make
authority possession explicit, limit available operations, and use constructor visibility to
resist forgery. Authority also needs issuance, transfer, cloning, revocation, expiry, and
leakage semantics. Ownership of a Rust value is evidence of local custody, not proof that an
external administrator has not revoked permission.

### Lifecycle invariants

A lifecycle invariant spans acquisition, use, handoff, shutdown, and release. A transaction
handle may be consumed by commit so local code cannot reuse it. RAII can release local
resources during unwinding. External rollback, compensation, and durable cleanup remain
fallible and need their own states.

### Boundary invariants

A boundary invariant states how a less-trusted representation becomes trusted domain
evidence. Every public constructor, deserializer, database decoder, FFI conversion, cache
loader, and migration path must preserve it. Validation is centralized at entry; it has not
disappeared.

### Collection invariants

A collection invariant describes the collection as a whole: non-empty, bounded, unique,
sorted, capacity-limited, or containing compatible currencies. Mutation methods and iterator
construction can invalidate these properties unless the wrapper controls every change.

### Cross-entity invariants

A cross-entity invariant relates records or aggregates, such as total allocation equaling an
invoice amount or an account version matching the update's expected version. A single newtype
cannot normally prove it. Domain services, database constraints, transactions, optimistic
concurrency, or reconciliation enforce it at runtime.

### Temporal invariants

A temporal invariant constrains ordering or time: a lease must be unexpired when used, a
session must be revoked after a security event, or a retry must not outlive an idempotency
record. Time readings, clock assumptions, races, and stale caches matter. A type created at
time A cannot guarantee the fact remains true at time B without observation or bounded
validity.

### Environmental assumptions

An environmental assumption is a fact the design relies on but does not control: filesystem
rename semantics, database isolation, protocol limits, clock behavior, allocator agreement,
or remote idempotency retention. Assumptions must be named, versioned where relevant, and
tested or monitored. Calling an assumption an invariant would falsely assign enforcement to
the local design.

### Distributed invariants

A distributed invariant spans independent failure domains, such as "at most one capture is
accepted for an idempotency key" or "every committed outbox record is eventually attempted."
The precise boundary matters. Network partitions, duplicate delivery, partial failure, and
concurrent actors often prevent a simple global proof. Protocol, durable identity, atomic
local transactions, deduplication, reconciliation, and audit trails provide bounded
guarantees.

## Related but distinct statements

An **invariant** must remain true throughout its scope. A **precondition** must be true before
an operation and defines caller or environment obligations. A **postcondition** is promised
after a particular successful result. A **policy** selects desired behavior and may change by
configuration or authority. An **assumption** is relied upon but enforced elsewhere or not
enforced. An **observation** is evidence gathered at a time and may become stale. A **desired
outcome** is a goal, not a guarantee.

Consider `Connection<Open>`. "The local connect transition returned success" is historical
evidence encoded by the state. "The remote peer is reachable now" is a mutable observation,
not a lasting invariant of the value. "`send` is called only after local connection" is a
sequencing invariant. "The next send succeeds" is a desired outcome and must remain fallible.

Confusing these categories creates false guarantees. A successful authentication observation
does not establish perpetual authorization. A database schema constraint does not prove old
data passed the newest policy unless migration verified it. A timeout does not establish that
the remote side did nothing.

## Invariant inventory

Before representation choice, record each consequential statement using this format:

```text
ID
Statement
Scope
Owner
Classification
Enforcement mechanism
Trust boundary
Evidence
Failure consequence
Residual uncertainty
```

**ID** is stable within the design or review. **Statement** is falsifiable. **Scope** names the
value, aggregate, operation, boundary, component, or history. **Owner** is accountable for the
truth or enforcement. **Classification** uses the categories above. **Enforcement mechanism**
states compiler, visibility, constructor, enum, transaction, constraint, service,
synchronization, protocol, monitoring, or another control. **Trust boundary** identifies
entry and decoding. **Evidence** names compiler rejection, tests, schema inspection,
transactional result, telemetry, reconciliation, or audit records. **Failure consequence**
drives severity. **Residual uncertainty** prevents the mechanism from becoming a broader
claim.

An example row:

| Field                 | Content                                                                                          |
| --------------------- | ------------------------------------------------------------------------------------------------ |
| ID                    | INV-PAY-004                                                                                      |
| Statement             | A locally requested capture references an accepted authorization for the same payment and amount |
| Scope                 | capture command construction                                                                     |
| Owner                 | payment domain                                                                                   |
| Classification        | transition and cross-entity invariant                                                            |
| Enforcement mechanism | verifier-issued capability plus runtime amount comparison                                        |
| Trust boundary        | provider authorization response and persisted reload                                             |
| Evidence              | constructor tests, compiler rejection before authorization, integration contract test            |
| Failure consequence   | unauthorized or wrong-amount capture                                                             |
| Residual uncertainty  | provider may reject, time out, or accept without returning acknowledgement                       |

## Discovery method

Start from failure, not from a favorite Rust feature. Ask:

- Which combinations would be contradictory?
- Which values are meaningless or dangerous?
- Which operations require earlier evidence?
- Which actor has authority, and can authority be forged or copied?
- Which facts cross process, storage, network, or FFI boundaries?
- Which facts can change after construction?
- Which updates must be atomic across entities?
- Which effect can succeed without acknowledgement?
- Which duplicate, reorder, cancellation, or concurrent execution breaks the story?
- What is the consequence if the statement is false?

Trace requirements, examples, incidents, protocol specifications, schema constraints, and
operational recovery. Negative cases are often more revealing than success paths.

## Classification drives representation

Classification narrows choices without making them automatic:

| Invariant shape                   | Usual first mechanism                        |
| --------------------------------- | -------------------------------------------- |
| Mutually exclusive state          | enum with variant-specific data              |
| Stable local scalar rule          | opaque validated newtype                     |
| Whole-collection rule             | validated collection wrapper                 |
| Small locally controlled sequence | consuming transition or typestate            |
| Authority possession              | capability type                              |
| Dynamic or persisted lifecycle    | runtime enum and validated state machine     |
| External input                    | parse and runtime validation                 |
| Cross-entity fact                 | domain service plus transactional validation |
| External effect result            | structured `Result`                          |
| Ambiguous distributed effect      | explicit unknown state and reconciliation    |

These are starting points. A complex system often uses several: a runtime persisted payment
status, a consuming local authorization capability, an opaque idempotency key, and an
explicit capture outcome.

## Ownership and change

An invariant without an owner is an aspiration. Ownership names the component or authority
responsible for construction, transition, or observation. The owner also defines change:
which migrations transform historical data, which version changes policy, and which runtime
monitor detects assumption failure.

Not every desired property should become a type. If enforcement requires external observation
or cross-entity synchronization, forcing it into a local type can create stale or forged
evidence. If misuse is low-impact and immediately checked, ordinary runtime code may be the
clearest mechanism. The inventory makes that proportional choice explicit.

## Evidence and review

Review asks whether the statement is complete, the owner can enforce it, every construction
and decoding path preserves it, and evidence tests violation as well as success. Compiler
rejection proves selected programs do not type-check; it does not prove all runtime input is
valid. A database constraint proves the database rejected or accepted according to its
current schema; it does not prove an external side effect.

Invariants evolve. A new policy may make historical values invalid, a protocol may add a
variant, or an authority model may gain revocation. Version and migration planning are part of
the invariant, not clerical aftermath.

---

## Source: `foundations/evidence.md`

# Evidence

Trusted domain values are evidence-carrying values. Their representation and construction
record that a particular check, transition, observation, or authority grant occurred. The
strength of the type's name and documentation must not exceed that evidence.

Evidence is not metaphysical proof. It is a scoped claim established by a mechanism under
assumptions. A private `NonZeroU64` field can make zero unconstructible through safe public
paths. It cannot establish that the amount is affordable, belongs to a particular currency,
or follows a correct tax calculation. An ownership-verification response can justify
`VerifiedEmailAddress`; it cannot guarantee continued mailbox control or future
deliverability.

## Evidence levels

The following progression is common but not universal:

```text
raw input
    ↓
parsed value
    ↓
syntactically valid value
    ↓
policy-accepted value
    ↓
externally verified value
    ↓
authorized capability
    ↓
persisted fact
    ↓
reconciled external outcome
```

Each arrow is a fallible evidence-producing operation. Systems may branch or omit levels, but
they must not silently rename a lower level as a higher one.

### Raw input

Raw input is bytes, text, loosely typed JSON, a database row, an environment variable, an FFI
pointer, or another representation not yet interpreted by the domain. Size limits and
resource controls may be required before parsing. Raw input is not "bad"; it is simply
untrusted for domain use.

### Parsed value

Parsing establishes structural interpretation: text became an integer, JSON became a request
DTO, or bytes became a protocol frame. Parsing may reject malformed representation but does
not necessarily enforce domain policy. A parsed integer can still be zero or outside an
account limit.

### Syntactically valid value

Syntax validation establishes a documented grammar or local shape. An `EmailAddress` example
might require one `@`, non-empty local and domain parts, bounded length, and a dotted domain.
That is not RFC-complete validity, deliverability, or ownership. The validation policy must
be named and tested.

### Policy-accepted value

Policy acceptance applies current domain rules: allowed country, permitted currency, password
strength, configured amount bounds, or product availability. Policy can change and may depend
on configuration. Persisted evidence should record policy version or be revalidated when
current acceptance matters.

### Externally verified value

External verification relies on another authority or observation: a mailbox challenge was
completed, an identity provider authenticated a principal, or a bank confirmed an account.
The value should carry verification identity, time, issuer, scope, or expiry where those
affect use. Network and provider failures remain runtime failures.

### Authorized capability

A capability indicates local possession of authority to request an operation. Constructor
visibility, unforgeable tokens, limited methods, and non-clonability can strengthen it.
Revocation, expiry, leakage, serialization, and external enforcement remain part of the
contract. A capability is not the result of exercising authority.

### Persisted fact

Persistence establishes that a representation was accepted by a specific storage operation
under a schema and transaction. It may provide version or commit identity. It does not make
data forever current, preserve new invariants automatically, or include external effects
outside the transaction.

### Reconciled external outcome

Reconciliation establishes a later observation about an effect whose immediate result was
unknown. It ties an operation identifier or provider reference to a confirmed outcome.
Reconciliation evidence should record authority, observation time, and causality. Even then,
the claim has a boundary: a confirmed capture does not establish later settlement.

## Evidence-accurate naming

Names are public claims. A useful progression is:

```text
EmailInput
EmailAddress
DeliverableEmailAddress
VerifiedEmailAddress
```

`EmailInput` says only where the representation came from. `EmailAddress` should document its
syntax policy. `DeliverableEmailAddress` would require evidence that a delivery route accepted
or is expected to accept the address; the exact mechanism and time matter.
`VerifiedEmailAddress` requires an ownership-verification process and protected construction.
These types are not interchangeable.

Avoid aspirational names such as `SafePath`, `AuthorizedUser`, or `CommittedTransaction`
unless constructors and boundaries establish the stated evidence. Prefer narrower names:
`NormalizedRelativePath`, `AuthenticatedPrincipal`, `CaptureCapability`, or
`CommitAcknowledged`, as appropriate.

## Establishing evidence

For each evidence-carrying type, record:

- claim established;
- input and preconditions;
- producer or authority;
- validation or transition algorithm;
- policy or protocol version;
- time and expiry when relevant;
- protected constructor location;
- persistence and deserialization path;
- error and indeterminate outcomes;
- revocation or invalidation;
- evidence tests;
- non-guarantees.

The producer matters. A public `VerifiedEmailAddress::new(String)` cannot establish external
verification because any caller can invoke it without proof. A verifier-owned proof token
whose field is private and whose constructor is restricted can make the transition harder to
forge. If the token is `Clone`, serializable, or valid forever, those semantics need explicit
justification.

## Preserving evidence

Private fields protect only ordinary struct construction. Evidence can be lost or forged
through:

- derived `Deserialize` that writes fields directly;
- unchecked `From<String>`;
- database `FromRow` construction without validation;
- public `from_raw` or `new_unchecked`;
- broad `unsafe` constructors;
- mutation methods that no longer validate;
- public enum variants carrying trusted inner data;
- `Default` values that do not satisfy the claim;
- cloning or serialization of authority;
- migration scripts that write impossible historical data;
- FFI values accepted without layout or semantic checks;
- stale cache reloads under newer policy.

Every boundary uses the protected constructor or an explicitly reviewed equivalent. Where a
bypass is necessary for trusted internal performance, scope it narrowly, state preconditions,
make misuse visible, and test the safe façade.

## Evidence composition

Evidence can be composed only when scopes align. `PositiveMoney<USD>` plus a verified account
does not prove sufficient funds. An authenticated principal plus an authorization policy
decision can produce a scoped capability, but only for the resource, operation, tenant, and
time covered. A persisted authorization plus a current capture request must still compare
payment identity and amount.

Composition often belongs in a domain service or transactional operation. Encoding every
cross-entity fact in generic parameters can create stale evidence and state explosion.
Structural types should carry stable local facts; runtime services should establish temporal
and relational facts.

## Evidence decay and revocation

Some evidence is immutable history: a compiler accepted a revision, a challenge completed, or
a database acknowledged a commit. The implications can still decay. Authorization may be
revoked, a policy version superseded, a certificate expire, or an external status change.

Types cannot freeze mutable external reality. Designs use expiry, version fields, observation
timestamps, revocation checks, leases, or forced revalidation. The type name should distinguish
historical evidence from current authority when that difference matters.

## Evidence in failures

Errors and unknown outcomes also carry evidence. A rejection can include a provider decision
code; a validation error identifies which policy failed; an unknown capture carries operation
and reconciliation identifiers. Collapsing all failures to text loses machine-actionable
evidence. Collapsing timeout to rejection invents evidence the system does not have.

## Review

Review follows the evidence chain from every producer to every consumer. It attempts direct
construction, alternate deserialization, invalid historical rows, clones, expired tokens, and
wrong-entity composition. Tests should demonstrate accepted and rejected values, while
compile-fail tests demonstrate important prohibited programs.

Evidence supports precise confidence. Passing tests are evidence for selected behavior on the
tested revision and environment. They do not prove universal correctness. Honest naming and
boundary preservation keep limited evidence useful rather than turning it into false
certainty.

---

## Source: `foundations/trust-boundaries.md`

# Trust boundaries

A trust boundary is a point where data, authority, control flow, or effect evidence enters
from a context whose invariants the current domain cannot assume. Boundaries exist inside a
single process as well as across networks. A database row, cached serialized value, plugin
callback, environment variable, or public constructor can be a boundary.

The central pipeline is:

```text
untrusted representation
    ↓ parse
structural representation
    ↓ validate
trusted domain representation
    ↓ execute
external side effect
    ↓ observe / reconcile
new trusted evidence or explicit uncertainty
```

Validation is relocated and centralized; it is not eliminated. After a trusted type is
constructed, ordinary domain operations may omit repeated local checks only because every
construction and mutation path preserves the documented invariant.

## What "untrusted" means

Untrusted does not mean malicious. It means the representation is not covered by the current
proof. A row may have been written before a migration, by another service, through manual
repair, or under an older policy. An internal message may be duplicated or reordered. A file
may change between inspection and use. A UI may be honest but compromised. A type-safe SDK
may still deliver a stale or ambiguous external response.

Trust is claim-specific. JSON can be structurally valid but unauthorized. An authenticated
principal can be unauthorized for a resource. A database constraint can establish non-null
but not a current cross-service fact. A successful TLS connection authenticates according to
its configuration but does not prove the next business operation will succeed.

## Common boundaries

### HTTP and RPC

Request methods, paths, headers, bodies, query strings, peer identity, and size are untrusted.
Parsing produces DTOs. Validation establishes domain values. Authentication establishes a
principal; authorization produces a scoped decision or capability. Idempotency, retries,
versioning, correlation, and response evidence must be explicit. A client disconnect does not
prove server-side cancellation.

### Message brokers

Messages can be malformed, duplicated, delayed, reordered, replayed, or delivered after
schema evolution. Consumer code validates envelopes and payloads, handles unknown versions,
deduplicates using durable scope, and defines acknowledgement timing. A lost acknowledgement
can cause redelivery after the effect completed.

### Databases

Rows are persistence representations, not trusted domain objects. Decode into a raw row,
validate through `TryFrom`, quarantine invalid historical data, and align transactions with
cross-entity invariants. Schema constraints reinforce domain rules but do not replace
constructors. A successful commit does not include a remote API call unless a protocol
explicitly coordinates both.

### Files and filesystems

Paths, metadata, file content, permissions, symlinks, and directory entries can change.
Canonicalization alone does not defeat time-of-check/time-of-use races. Bound size, resist
traversal, use appropriate atomic replacement and durability semantics, and avoid treating a
successful write call as durable storage without the required flush and directory protocol.

### Environment variables and configuration

Strings require parsing into typed durations, sizes, addresses, and policy enums. Defaults are
policy. Cross-field combinations need whole-configuration validation. Reloads create
concurrency and partial-application questions. Secrets must not appear in ordinary debug or
error output.

### CLI arguments

CLI parsers establish syntax, not necessarily authorization, path safety, or domain policy.
Non-interactive automation may supply stale or conflicting flags. Normalize once, reject
invalid combinations, and retain evidence of the chosen operation.

### FFI

Pointers, lengths, layout, ownership, nullability, string encoding, callbacks, allocator
identity, unwinding, and thread restrictions cross the compiler's safe boundary. A safe Rust
wrapper must validate caller-independent conditions and document obligations that cannot be
checked. Every `unsafe` operation carries a local proof obligation.

### Operating-system resources

Sockets, processes, file descriptors, locks, credentials, clocks, and signals can change
independently. RAII can manage local handle lifetime but cannot guarantee a remote peer,
process, or durable effect remains available.

### External services

Responses establish only the protocol evidence returned by the service. Timeouts, connection
loss, rate limits, and inconsistent reads can make results unknown. Retries require
idempotency analysis. External state observations can become stale immediately.

### User interfaces

Frontend validation improves feedback but does not authorize backend operations. UI state is
a local projection. Navigation, refresh, concurrent devices, expired sessions, and uncertain
submissions require server authority and reconciliation. Preserve user input across retriable
or unknown states.

### Cached serialized values

Caches may outlive code, policy, schema, credentials, or authoritative data. Treat cache bytes
as versioned input. Validate on load and define invalidation. A cache hit is an observation,
not proof of current truth.

## Boundary contract

Every boundary documents:

1. representation and threat or drift sources;
2. size and resource limits applied before expensive work;
3. parsing and structural errors;
4. normalization rules and whether original form is retained;
5. domain validation and policy version;
6. trusted constructor and its visibility;
7. authentication and authorization when relevant;
8. unknown and future values;
9. error categories and sensitive-data handling;
10. operation identity, retries, and ambiguity;
11. evidence tests;
12. residual uncertainty and revalidation.

The contract names an owner. "Serde validates it" is not enough when the derive writes private
fields directly. "The database enforces it" is not enough when replicas, old rows, or migration
scripts use a different schema.

## Parsing, validation, and normalization

Parsing answers whether a representation has a structure. Validation answers whether the
structured value satisfies a domain invariant or policy. Normalization selects a canonical
representation. These operations can interact but should not be mislabeled.

Normalization may be lossy or security-sensitive. Unicode case folding, path resolution,
email casing, and identifier trimming require domain policy. Validate either before or after
normalization according to the intended claim, and test collisions. Preserve raw input when
audit or user correction needs it.

## Protected construction

A trusted type has private fields and fallible construction. Boundary adapters call that
construction rather than reproducing a weaker subset of checks. If several adapters need the
same validation, put the complete policy in one constructor and translate boundary-specific
errors without discarding cause.

Unchecked construction is an escape hatch. If required for verified internal data, restrict
visibility, document all preconditions, and keep it absent from ordinary boundary code. An
unsafe constructor transfers proof responsibility; it does not waive the invariant.

## Unknown and future values

Protocols and schemas evolve. An unknown enum value can be rejected, retained as an explicit
unknown variant, or passed through as raw data depending on compatibility and security policy.
Rejecting unknown fields can harden closed control protocols but can break additive evolution;
it is a policy choice, not a universal default.

Never map unknown external state to the nearest known state merely to satisfy an enum. Preserve
the raw discriminant or quarantine the record. Downstream behavior must be safe under absence
of interpretation.

## Effects and uncertainty

After trusted input drives an external effect, local types cannot guarantee outcome. A
confirmed response can establish success or rejection according to protocol. A pre-send local
failure may establish non-execution only if the request was definitely not transmitted. A
timeout or connection loss after transmission may produce an unknown outcome.

Unknown outcomes carry operation identity, provider reference if known, timestamps, and a
reconciliation strategy. Retry decisions distinguish safe retry, unsafe retry, and
reconcile-before-retry. This is a second trust boundary: external observation becomes new
domain evidence.

## Evidence

Boundary evidence includes parser and constructor rejection tests, fuzz or property tests,
payload limit tests, invalid historical row tests, schema evolution tests, authentication and
authorization cases, duplicate and reorder tests, fault injection, and reconciliation tests.
Integration tests should cross a real boundary where feasible; mocks must not erase ambiguity
or provider-specific failure categories.

Review enumerates all alternate inputs and constructors. The goal is not to call data trusted
early. It is to make the transition from representation to evidence explicit, narrow, and
auditable.

---

## Source: `foundations/guarantee-honesty.md`

# Guarantee honesty

A guarantee is a claim backed by an enforcement mechanism and evidence within a stated scope.
Guarantee honesty prevents type names, API documentation, reviews, and generated agent context
from becoming stronger than the implementation.

The discipline separates four things:

1. **Claim:** what the design says is true.
2. **Mechanism:** how the design attempts to establish or preserve it.
3. **Evidence:** what was observed about a specific revision, configuration, or runtime.
4. **Residual risk:** what can still fail, change, or remain unknown.

A private field is a mechanism. Compiler rejection of direct construction is evidence for one
class of program. Neither proves database decoding uses the constructor. A passing integration
test is evidence for tested behavior; it does not prove all schedules or external histories.

## Required questions

Every type-level design, capability, state machine, boundary conversion, and external-outcome
model must answer:

1. **What does the type prove?** State the narrow invariant, transition history, authority,
   or observation represented.
2. **How is the proof established?** Name constructor, parser, verifier, transaction,
   protocol response, reconciliation, or compiler rule.
3. **How is construction protected?** Enumerate visibility, private fields, sealed proof
   tokens, non-clonability, consuming APIs, and mutation controls.
4. **How does decoding preserve the proof?** Trace Serde, database, cache, migration, FFI, and
   versioned representation paths.
5. **Which escape hatches exist?** Name unchecked, unsafe, privileged, test-only, feature-gated,
   or migration paths and their review contracts.
6. **What does the type not prove?** List adjacent facts a reader may mistakenly infer.
7. **Which facts can change externally?** Include revocation, expiry, liveness, balance,
   policy, topology, or provider state.
8. **Which failures remain runtime failures?** Include I/O, resource exhaustion, rejection,
   cancellation, contention, and provider behavior.
9. **Which outcomes may be indeterminate?** Include transmitted requests without
   acknowledgement, ambiguous commit, lost messages, and stale observation.

If an answer is absent, narrow the claim or complete the design.

## Guarantee ledger

Use this ledger for major types, case studies, review, and pull requests:

| Claim                                                     | Established by                                                | Protected construction                                | Boundary preservation                                          | Escape hatches                          | Does not prove                                                  | Residual runtime risk                           |
| --------------------------------------------------------- | ------------------------------------------------------------- | ----------------------------------------------------- | -------------------------------------------------------------- | --------------------------------------- | --------------------------------------------------------------- | ----------------------------------------------- |
| `PositiveMoney` is non-zero                               | `NonZeroU64` accepted by a fallible constructor               | private field; no unchecked public constructor        | DTO and row conversions call constructor                       | scoped migration conversion if reviewed | sufficient funds, correct FX, tax or allocation policy          | overflow on later arithmetic, currency mismatch |
| `VerifiedEmailAddress` passed ownership verification      | verifier-only proof token after completed challenge           | private fields and restricted proof-token constructor | persisted issuer, scope, time, and address revalidated on load | administrative import with audit        | future deliverability, continued control, RFC-complete validity | revocation, expiry, provider error              |
| `Connection<Open>` completed local connection transition  | consuming `connect` returned `Ok`                             | state marker and constructor visibility               | not normally serialized; restoration requires a new connection | test transport factory                  | remote liveness at next send                                    | immediate network failure, peer closure         |
| `AuthorizedPayment` passed local authorization transition | accepted authorization response and identity/amount checks    | consuming transition; capability not freely cloneable | row decode validates status and authorization reference        | repair tool with scoped authorization   | capture success, settlement, absence of provider reversal       | timeout, expiry, provider rejection             |
| `UnknownCapture` has reconciliation identity              | explicit outcome constructor after ambiguous transport result | private operation and token fields                    | durable row stores operation identity and provider scope       | manual reconciliation record with audit | whether capture succeeded or failed                             | delayed visibility, concurrent reconciliation   |

Ledger rows should identify exact project types and methods during review. Generic examples
teach structure but are not evidence for an implementation.

## Construction audit

List every path that can create or change the trusted value:

- public and crate-visible constructors;
- struct literals and enum variants;
- `Default`, `From`, `TryFrom`, `FromStr`, builders, and macros;
- `Deserialize`, custom visitors, and remote adapters;
- database row mappings and ORM derives;
- migration and administrative repair code;
- cloning, copying, mutation, and collection insertion;
- test utilities and feature-gated APIs;
- FFI imports and raw-pointer wrappers;
- unsafe and unchecked functions.

The documented invariant is complete only if all paths establish or explicitly assume it. A
private field plus derived `Deserialize` can be dishonest. A complete `new` plus weaker
`From<String>` is dishonest. A capability that derives `Clone` may turn exclusive authority
into duplicable authority.

## Boundary preservation

Trusted memory does not make persisted or serialized bytes trusted. Decode into a raw
representation, then validate through the canonical constructor. If the wire format needs a
stable shape, use Serde's `try_from` or a manual implementation. Database adapters should use
`TryFrom<Row>`, return invalid historical data as a distinct failure, and provide quarantine
or repair policy.

Versioning is part of the proof. A value accepted under policy version 1 may not satisfy
version 2. Either retain evidence that v1 remains acceptable, migrate it, revalidate it, or
represent its legacy state honestly.

## Escape hatches

Some systems require a bypass for trusted constants, bulk migration, FFI, or measured hot
paths. The escape hatch must be:

- visibly named;
- narrower in visibility than ordinary construction;
- documented with complete preconditions;
- owned by a specific module or operational role;
- excluded from generic boundary adapters;
- covered by tests of the safe interface;
- discoverable by audit;
- and reviewed under the doctrine governing its risk.

`unsafe` means the compiler cannot verify the proof; it does not mean the invariant is
optional. A safe `from_raw_unchecked` is often worse because it looks ordinary while
transferring proof responsibility.

## External reality

Rust types describe local program evidence. They cannot freeze a network, user, database,
clock, credential issuer, remote service, or physical resource.

`Connection<Open>` records a local successful transition. The peer may close immediately.
`AuthenticatedPrincipal` records an authentication result; the session may expire or be
revoked. `AuthorizedCapability` records a decision under a policy and resource scope; policy
or ownership can change. `Persisted<T>` records a storage acknowledgement; a concurrent actor
may update the row. Documentation must state observation time, validity bounds, and required
rechecks.

## Failure and indeterminacy

External effects remain fallible after every compile-time sequencing check. Error categories
must preserve operational distinctions: rejection, validation failure, cancellation,
conflict, timeout, local resource failure, and unknown external outcome.

A timeout is not necessarily failure. If a request may have reached the remote system, the
result is unknown unless protocol guarantees otherwise. The type should carry operation
identity and reconciliation instructions. Automatically retrying may duplicate a payment,
message, or provisioning action.

Database commit can also be ambiguous when the connection fails around acknowledgement. The
application must use database-specific evidence, idempotent operation identity, or
reconciliation rather than report fictional rollback.

## Evidence quality

Evidence is bound to scope:

- the compiler rejects a selected forbidden program under a named API and toolchain;
- a unit test observes selected constructor behavior;
- a property test samples a generated input model;
- an integration test crosses a configured boundary;
- model checking explores a bounded state space;
- telemetry observes deployed histories;
- an incident falsifies an assumption.

More tests do not expand the claim automatically. Review asks what violation each test could
detect, which environment it ran against, and what it cannot observe. Updating snapshots or
compiler diagnostics without semantic inspection weakens evidence.

## Language discipline

Prefer "establishes," "prevents through safe public construction," "records that," and "was
observed" over absolute terms such as "ensures forever." Pair a guarantee with its
non-guarantee in the same section. If a type name repeatedly invites a stronger inference,
rename it rather than relying on distant caveats.

Honesty is not pessimism. Narrow guarantees compose. A type that accurately proves one fact is
more useful than a type that vaguely claims a whole business outcome. Explicit uncertainty
lets systems recover without corrupting their own account of reality.

---

## Source: `foundations/complexity-budget.md`

# Complexity budget

Rust can encode rich protocols and refined values, but every type parameter, wrapper, trait
bound, macro, state marker, and conversion consumes a complexity budget. The goal is not
maximum type cleverness. The goal is the simplest representation that prevents consequential
invalid programs without obscuring the system.

Complexity is justified by risk removed, not by elegance in isolation. A two-state local
builder used by many callers may benefit from typestate. A persisted workflow with dozens of
states, external transitions, dynamic inspection, and evolving schema is usually clearer as
a runtime enum plus validated transitions. A bounded amount can be an opaque newtype; a
cross-account balance rule belongs in transactional runtime logic.

## Budget inputs

Assess the following before selecting additional type machinery.

### State and transition shape

Count meaningful states, legal transitions, conditional transitions, and state-specific data.
Estimate growth. Typestate works best when the graph is small, locally controlled, and static.
Generic states can explode public APIs when transitions depend on runtime policy or external
responses.

### Misuse frequency and impact

Ask how often the invalid action is plausible and what happens if it occurs. A wrong-state
payment capture deserves stronger prevention than a harmless display preference. Repeated
production incidents, security consequences, financial loss, or irreversible external effects
increase the budget.

### Public API surface

Structural enforcement is more valuable at a widely reused library boundary because callers
cannot share all local context. It also increases compatibility obligations. Public marker
types, error enums, and generics become part of the API. Consider evolution, downstream
diagnostics, semver, and language bindings.

### Serialization and persistence

Generic typestate does not naturally serialize a heterogeneous runtime state. Persisted
records need stable discriminants, unknown-variant policy, migration, and runtime decoding.
A hybrid can use a runtime enum for storage and consuming transitions for one local operation.
Duplicated representations require explicit conversion and evidence.

### Dynamic dispatch and heterogeneity

Collections of mixed states, plugins, trait objects, user-driven workflows, and runtime
inspection often favor enums or trait objects. Encoding every state in a distinct concrete
type can force boxing, erasure, or large match layers that remove the expected benefit.

### Async and external effects

An async transition can fail, be cancelled, or become ambiguous. A consuming method that
loses the prior value on failure may make recovery awkward. The API may need to return the
previous state with the error, store durable intent before awaiting, or use a runtime state
machine. Types cannot turn a timeout into certainty.

### Trait-bound readability

Complex generic constraints can hide the business rule and produce diagnostics remote from
the caller's mistake. Measure whether a new team member can understand construction,
transition, and recovery without reverse-engineering type algebra. Good compiler rejection is
part of usability evidence.

### Compile time and monomorphization

Generic states and combinatorial trait implementations can increase compilation, incremental
rebuild time, generated code, and binary size. These are measurement questions. Do not claim
zero-cost solely because the state markers are zero-sized; monomorphization and API
duplication can still cost.

### Team familiarity and maintenance

A mechanism that only one maintainer understands has operational risk. Documentation and
examples can reduce that cost, but should not be used to justify needless abstraction.
Migration, debugging, on-call diagnosis, and incident repair count alongside authoring.

### Interoperation

FFI, databases, Serde, RPC schemas, and other languages often need runtime representations.
If every boundary erases and reconstructs type state, verify that the proof is real and the
conversion cost is worthwhile.

## Mechanism ladder

Choose the lowest-cost mechanism that adequately contains the failure:

1. clear ordinary code and a named runtime check;
2. enum for mutually exclusive runtime state;
3. opaque newtype for a stable local value invariant;
4. validated collection wrapper;
5. consuming method to prevent immediate local reuse;
6. capability type for authority;
7. typestate for small locally controlled sequencing;
8. hybrid runtime and compile-time state model;
9. more elaborate type-level proof only for exceptional, evidenced risk.

This is not a universal ranking. A capability may be simpler than a complex runtime
permission object. An enum may be both cheaper and stronger for contradictory state. The
ladder prompts an explanation for skipping simpler options.

## Complexity decision record

For a representation choice, record:

| Dimension      | Observation                           | Cost or risk                  | Evidence                        |
| -------------- | ------------------------------------- | ----------------------------- | ------------------------------- |
| Invalid action | Which misuse is prevented             | Consequence and frequency     | incidents, threat model, review |
| State graph    | State and transition count            | explosion or clarity          | state diagram                   |
| Control        | local, external, or shared            | stale proof and runtime need  | boundary map                    |
| Persistence    | format and migration                  | conversion and compatibility  | schema tests                    |
| API            | caller count and stability            | semver and diagnostics        | compile-fail tests              |
| Runtime        | dispatch, allocation, synchronization | latency and contention        | benchmarks or profiles          |
| Build          | generics and macros                   | compile time and binary size  | measured builds                 |
| Team           | familiarity and support               | maintenance and incident cost | review exercise                 |
| Alternative    | simpler mechanism                     | residual invalidity           | comparative prototype           |

The decision also states a removal trigger. If state count grows beyond the usable limit,
persistence becomes necessary, diagnostics degrade, or measurement shows material cost, the
design should be reconsidered.

## Typestate budget

Typestate is proportionate when:

- sequencing is locally controlled;
- state count and transition graph are small;
- ownership can naturally consume prior state;
- callers benefit from compiler rejection;
- state-specific methods are stable;
- objects do not require heterogeneous storage or routine serialization;
- async failure can return usable recovery state or is otherwise designed;
- and compile diagnostics are understandable.

Prefer a runtime enum when state is externally chosen, persisted, replayed, discovered at
runtime, held in mixed collections, inspected by UI or operations, or expected to evolve
frequently. Prefer a consuming transition without a generic state parameter when only reuse
prevention matters.

Do not use a phantom marker to suggest external liveness or authorization that can be revoked.
The complexity buys local protocol evidence only.

## Newtype budget

Opaque newtypes are usually low-cost for stable scalar invariants. Costs still include
conversion, formatting, borrowing, Serde adapters, database decoding, error types, and policy
versioning. Avoid creating a distinct wrapper for every conceptual label when values have no
different invariant or accidental interchange consequence.

A wrapper must control mutation. A `NonEmptyVec` that exposes unrestricted `Vec::clear` is
dishonest. An email wrapper with a public tuple field buys only naming.

## Capability budget

Capability types are valuable when possession should grant a narrow operation. Review issuance,
clone, transfer, serialization, expiry, and revocation. If authority is checked centrally on
every operation and local possession offers no stable right, a runtime authorization decision
may be clearer. If revocation is essential, a capability may carry an identifier and require
online validation rather than claim perpetual authority.

## Runtime simplicity is legitimate

Plain runtime validation is not a design failure. External input, mutable reality,
cross-entity facts, configurable policy, concurrency, and distributed outcomes require
runtime checks. An explicit function with a structured error can be more honest than a type
whose proof becomes stale.

The important questions are whether validation is centralized, every boundary uses it,
failure is represented, and evidence covers violation. "Compile time" is not automatically
stronger when the fact exists only at runtime.

## Measuring complexity

Do not replace one unmeasured claim with another. Measure compile time, binary size,
allocations, latency, or diagnostic quality when those costs drive the decision. Use a
representative workload and compare against a simpler design. Record toolchain, features,
target, and inputs. A zero-sized marker does not prove zero system cost.

Qualitative costs can also be tested: ask a reviewer to trace a failed transition, have a new
maintainer add a state, inspect compiler messages from incorrect calls, and simulate migration
from a stored record. These exercises produce evidence about operability.

## Review questions

Review asks:

- Which consequential invalid program becomes impossible?
- Could an enum, newtype, runtime check, or consuming method achieve the same result more
  clearly?
- Does the mechanism preserve evidence across every boundary?
- Is the proof local while the name implies an external fact?
- What state or policy growth breaks the design?
- How does failure, cancellation, or uncertainty return usable information?
- Are compiler diagnostics better than runtime errors for intended callers?
- What compatibility surface and build cost are added?
- Is the mechanism understood and testable by the maintenance team?
- Which observation would trigger simplification?

Complexity budgeting is not permission to leave severe risk uncontained. It is the discipline
that makes protection sustainable. Strong design spends complexity where invalidity is
consequential and keeps the rest legible.

---

## Source: `doctrines/0001-invalid-states/README.md`

---
id: RUST-DOC-0001
slug: invalid-states
title: Making Invalid States Unrepresentable
status: active
version: 0.2.0
normative: true
applies_to:
  - planning
  - implementation
  - review
  - audit
risk_domains:
  - domain-modeling
  - state-machines
  - trust-boundaries
  - distributed-effects
supersedes: []
superseded_by: null
---

# Making Invalid States Unrepresentable

## Scope

This doctrine governs discovery, classification, representation, construction, transition,
decoding, and review of consequential invariants in Rust systems. It covers mutually exclusive
domain state, refined values, collection rules, locally controlled protocols, authority,
persistence, external effects, and distributed uncertainty.

Its core question is not "Can Rust encode this in a type?" The question is "Which invalid
programs are consequential, which facts are structurally enforceable, where must runtime
validation remain, and what evidence supports the resulting claim?"

## Out of scope

This package does not specify a universal domain architecture, require typestate, define
complete email or monetary policy, guarantee external service behavior, or replace specialist
doctrines for errors, concurrency, persistence, distributed systems, unsafe code, testing, and
performance. It does not assert that every rule should become a type.

## Intended readers

Planners use this package before selecting structs or state markers. Implementers use it to
protect construction and transitions. Reviewers trace every bypass and distinguish local proof
from external observation. Auditors use it to challenge evidence-inaccurate names and certainty.
Architects use it with the complexity budget to choose proportionate mechanisms.

## Normative status

`doctrine.md` is normative. Requirements use the interpretation in
`foundations/normative-language.md` and stable IDs beginning `RUST-DOC-0001-R`. Rationale,
examples, anti-patterns, glossary, and references are informative unless a rule incorporates
them.

Rule identifiers remain stable within the doctrine version, which this file's front matter and
`manifest/doctrines.yaml` record. Waivers follow the repository waiver contract. A new escape
hatch, weakened obligation, or changed rule meaning requires an RFC.

## Prerequisite foundations

Read, in order:

1. `foundations/normative-language.md`;
2. `foundations/invariants.md`;
3. `foundations/evidence.md`;
4. `foundations/trust-boundaries.md`;
5. `foundations/guarantee-honesty.md`;
6. `foundations/complexity-budget.md`.

## Related material

Related patterns are [sum types](../patterns/sum-types.md),
[opaque newtypes](../patterns/opaque-newtypes.md),
[smart constructors](../patterns/smart-constructors.md),
[validated collections](../patterns/validated-collections.md),
[consuming transitions](../patterns/consuming-transitions.md), typestate,
[capability types](../patterns/capability-types.md),
[hybrid state machines](../patterns/hybrid-state-machines.md), and
[explicit uncertainty](../patterns/explicit-uncertainty.md). Primary boundary guides are
[Serde](../boundaries/serde.md), [database decoding](../boundaries/database-decoding.md),
[HTTP/RPC](../boundaries/http-and-rpc.md), and messaging. Operational reviews are
[domain-model](../reviews/domain-model-review.md), boundary, typestate,
[distributed-effects](../reviews/distributed-effects-review.md), and
[final correctness](../reviews/final-correctness-audit.md) review.

Executable examples live under [`examples/domain-modeling`](../examples/domain-modeling/),
[`examples/validated-newtypes`](../examples/validated-newtypes/),
[`examples/typestate`](../examples/typestate/),
[`examples/boundary-validation`](../examples/boundary-validation/),
[`examples/distributed-outcomes`](../examples/distributed-outcomes/), and
[`examples/compile-fail`](../examples/compile-fail/). Case studies apply the doctrine to
[invoices](../case-studies/invoice/), [payments](../case-studies/payment-lifecycle/),
[transactions](../case-studies/database-transaction/),
[message delivery](../case-studies/message-delivery/),
[authenticated sessions](../case-studies/authenticated-session/), and
[UI workflows](../case-studies/ui-workflow/).

## Reading order

Read this file, `doctrine.md`, `rationale.md`, and `decision-framework.md` before design.
Implement against applicable pattern and boundary guides. Use `review-standard.md` during
review, then inspect `anti-patterns.md` for bypass shapes. Consult the glossary and references
when terms or source authority matter.

## Compact summary

- Discover and inventory invariants before choosing representation.
- Use enums for mutually exclusive dynamic state, opaque newtypes for stable local value
  invariants, validated wrappers for collection invariants, capabilities for authority, and
  runtime services or transactions for cross-entity facts.
- Use typestate or consuming transitions only for proportionate, locally controlled sequencing.
- Keep trusted fields private and make smart constructors complete.
- Preserve validation through Serde, databases, caches, migrations, FFI, and every alternate
  constructor.
- Name types for evidence actually established.
- Keep network, storage, and other external effects fallible.
- Do not convert an ambiguous timeout into confirmed failure.
- Represent unknown outcomes with reconciliation identity.
- Test accepted and rejected construction, important forbidden programs, boundaries, and
  distributed failure.
- Publish guarantees beside non-guarantees and residual uncertainty.

The desired result is not the greatest amount of type machinery. It is a legible system in
which consequential invalid states and transitions are hard or impossible to express, runtime
truth remains validated, and external uncertainty is reported honestly.

---

## Source: `doctrines/0001-invalid-states/doctrine.md`

# Normative doctrine

## RUST-DOC-0001-R001 — Inventory invariants before representation

**Statement.** A design MUST identify consequential invariants, their owners,
classifications, trust boundaries, enforcement mechanisms, evidence, failure consequences,
and residual uncertainty before selecting domain representations.

**Intent.** Prevent a favorite mechanism or an initial struct shape from deciding the domain
before contradictory states, authority, temporal facts, and external ambiguity are known.

**Applicability.** New domain models, substantial lifecycle changes, new external effects,
boundary integrations, and repairs caused by invariant failure.

**Allowed exceptions.** Pure mechanical refactoring whose behavior and construction surface
are demonstrably unchanged.

**Review evidence.** An invariant inventory using the foundation format, plus a state and
boundary map appropriate to the risk.

## RUST-DOC-0001-R002 — Represent mutually exclusive state as a sum type

**Statement.** Contradictory field combinations MUST be replaced by an enum or equivalent sum
type when domain states are mutually exclusive and carry state-specific data. A single field
whose value selects among a closed, known set of mutually exclusive alternatives MUST likewise
be decoded into a type that cannot hold a value outside that set, rather than retained as an
unconstrained string or integer.

**Intent.** Remove combinations such as `is_paid = true` with no receipt or simultaneous paid
and failed flags from ordinary construction, and remove the unconstrained discriminant, whose
out-of-vocabulary values survive decoding to be compared against literals at every use.

**Applicability.** Booleans, nullable fields, option groups, string discriminants, or structs
whose validity depends on exclusive combinations. A scalar constrained only in format, an open
vocabulary, and a value that selects among no alternatives are outside the second obligation.

**Allowed exceptions.** A foreign persistence or wire DTO may retain its external shape if it
is untrusted and converted into a validated domain enum before use. A vocabulary too large or
too volatile to enumerate may use a validated newtype that rejects an unknown value at
construction, provided the rejection is tested.

**Review evidence.** State table, exhaustive matching, invalid-combination rejection at the
boundary, decoding rejection of an unknown discriminant value, and persistence evolution
policy.

## RUST-DOC-0001-R003 — Protect trusted newtype representation

**Statement.** A trusted validated newtype MUST keep its representation private from callers
that are not authorized to assume or establish its invariant.

**Intent.** Make possession of the type meaningful evidence rather than an advisory wrapper.

**Applicability.** Scalars, identifiers, names, money amounts, tokens, and other values whose
type name asserts validation or authority.

**Allowed exceptions.** Transparent public wrappers whose documented purpose is nominal
distinction only and whose name does not assert validation.

**Review evidence.** Visibility audit covering fields, constructors, macros, derives,
features, tests, and re-exports.

## RUST-DOC-0001-R004 — Enforce the complete documented invariant

**Statement.** Every safe constructor for a trusted type MUST enforce the complete invariant
documented for that type, or require an evidence object that establishes the missing part.

**Intent.** Prevent a strong type name from being backed by one partial check or by different
policies across constructors.

**Applicability.** `new`, `parse`, `FromStr`, `TryFrom`, builders, collection constructors,
verifier transitions, and safe boundary conversions.

**Allowed exceptions.** A constructor may establish a deliberately narrower type whose name
and documentation reflect that evidence level.

**Review evidence.** Constructor matrix, positive and negative tests, policy version where
relevant, and proof-token construction audit.

## RUST-DOC-0001-R005 — Name the evidence accurately

**Statement.** A type, variant, method, or field name MUST NOT imply stronger evidence than its
construction establishes.

**Intent.** Prevent syntax validation from being mistaken for ownership, local transition from
external liveness, persistence acknowledgement from durable business completion, or timeout
from rejection.

**Applicability.** All evidence-carrying types and lifecycle variants.

**Allowed exceptions.** None for public claims. Domain-standard abbreviations MAY be used when
their exact repository meaning is documented.

**Review evidence.** Guarantee ledger linking each name to producer, scope, time, and
non-guarantees.

## RUST-DOC-0001-R006 — Preserve invariants through deserialization

**Statement.** Deserialization MUST NOT write a trusted representation in a way that bypasses
its documented validation.

**Intent.** Treat serialized bytes as untrusted regardless of whether they came from an
internal service or cache.

**Applicability.** Serde, custom formats, caches, files, message payloads, and RPC adapters.

**Allowed exceptions.** An explicitly versioned, cryptographically authenticated internal
format may use a privileged decoder only when its authenticity, invariant version, and bypass
preconditions are reviewed and tested.

**Review evidence.** `try_from` or manual decoding path, malformed and policy-invalid cases,
size limits, and unknown-version behavior.

## RUST-DOC-0001-R007 — Validate database decoding

**Statement.** Database decoding MUST NOT silently forge trusted domain values. Raw rows MUST
be checked against current or explicitly versioned invariants before trusted use.

**Intent.** Account for historical data, migrations, manual repair, schema drift, and writes
from other applications.

**Applicability.** ORM derives, row decoders, repositories, event stores, snapshot loaders, and
migrations.

**Allowed exceptions.** A database-native scalar whose complete invariant is enforced by the
database and whose decoder cannot represent an invalid value MAY map directly, provided that
the equivalence is documented.

**Review evidence.** Raw-row/domain separation, checked conversion, invalid-history test,
constraint inspection, quarantine or repair policy, and migration compatibility.

## RUST-DOC-0001-R008 — Preserve collection invariants after construction

**Statement.** A validated collection wrapper MUST control every mutation and construction
route that could violate non-empty, bounded, sorted, unique, capacity, or member-compatibility
invariants.

**Intent.** Prevent a valid initial wrapper from becoming invalid through unrestricted inner
access, iterator collection, clearing, or replacement.

**Applicability.** Domain collections whose whole-value property carries evidence.

**Allowed exceptions.** Immutable wrappers MAY expose read-only slices, iterators, and
borrowing that cannot violate the invariant.

**Review evidence.** Mutation API audit, boundary conversion tests, empty and overflow tests,
and iterator construction behavior.

## RUST-DOC-0001-R009 — Consume prior state when reuse is invalid

**Statement.** State-transition APIs SHOULD consume the prior state, token, transaction, or
capability when its reuse would violate a lifecycle or authority invariant.

**Intent.** Make local double commit, double use, wrong-order capture, or continued use after
close unavailable through ordinary safe code.

**Applicability.** Single-use tokens, transaction completion, shutdown permits, local protocol
states, and authority consumed by an operation.

**Allowed exceptions.** Runtime state guarded by durable concurrency control, externally
shared state, or transitions requiring retry from the same handle may use mutable/runtime
validation when consuming ownership would make recovery less correct.

**Review evidence.** Transition signatures, clone audit, compile-fail test for significant
reuse, and failure return semantics.

## RUST-DOC-0001-R010 — Use typestate proportionately

**Statement.** Typestate MUST be reserved for locally controlled operation sequencing where
state count, ownership, API shape, diagnostics, persistence, and evolution costs are justified
by the invalid programs prevented.

**Intent.** Avoid state explosion and false claims that compile-time local state describes
external or persisted reality.

**Applicability.** Generic marker states, `PhantomData`, state-specific impl blocks, and
builders that move through compile-time phases.

**Allowed exceptions.** None to the proportionality analysis; a small internal experiment MAY
be used to gather diagnostic and complexity evidence.

**Review evidence.** State graph, local-control argument, runtime-enum comparison, persistence
plan, async failure design, compile diagnostics, and complexity budget.

## RUST-DOC-0001-R011 — Use runtime state for dynamic reality

**Statement.** Dynamic, persisted, heterogeneous, externally determined, runtime-inspected, or
frequently evolving state SHOULD use an enum or explicit runtime state machine.

**Intent.** Preserve honest inspection, serialization, migration, and unknown-value handling
without encoding mutable external facts in static type parameters.

**Applicability.** Database status, UI state, message workflow, external provider lifecycle,
mixed-state collections, and replay.

**Allowed exceptions.** A hybrid design MAY convert a validated runtime state into a local
typestate operation when construction and staleness are controlled.

**Review evidence.** Persistence schema, transition validator, concurrency policy, unknown
variant plan, and hybrid conversion contract if used.

## RUST-DOC-0001-R012 — Represent authority as restricted capability

**Statement.** When possession should authorize an operation, a capability type MUST protect
issuance and expose no broader authority than intended; cloning, transfer, serialization,
expiry, and revocation MUST be specified.

**Intent.** Prevent forgery or accidental amplification of authority.

**Applicability.** Authorization grants, transaction rights, shutdown permits, verifier proof
tokens, secret access, and single-use operations.

**Allowed exceptions.** A centralized runtime authorization check MAY be clearer when
authority is mutable and must be revalidated on every use.

**Review evidence.** Issuer visibility, operation surface, clone/serialize audit, scope fields,
revocation and expiry behavior, and misuse tests.

## RUST-DOC-0001-R013 — Keep external effects fallible

**Statement.** Network, database, filesystem, process, device, and other external effects MUST
remain fallible even when local types prove legal sequencing and input invariants.

**Intent.** Prevent compile-time state from being misrepresented as control over independent
systems or resources.

**Applicability.** Connect, send, close, commit, capture, persist, publish, delete, and similar
operations.

**Allowed exceptions.** A pure in-memory transition with no observable external dependency MAY
be infallible if allocation and panic behavior are outside the API's promised failure model.

**Review evidence.** Structured result types, error categories, cancellation behavior,
resource-failure tests, and stated non-guarantees.

## RUST-DOC-0001-R014 — Do not collapse ambiguous timeout into failure

**Statement.** A timeout, disconnect, cancellation, or acknowledgement loss MUST NOT be
reported as confirmed non-execution when the external effect may have occurred.

**Intent.** Avoid duplicate payments, messages, commits, or provisioning caused by invented
failure evidence.

**Applicability.** Any request that may cross an external commitment point before local
certainty is lost.

**Allowed exceptions.** A protocol can establish non-execution when it specifies and
implements a verifiable pre-commit cancellation or rejection boundary.

**Review evidence.** Protocol commitment analysis, fault injection around send and
acknowledgement, outcome type, and retry decision table.

## RUST-DOC-0001-R015 — Model distributed uncertainty explicitly

**Statement.** When an external outcome can be uncertain, the domain MUST include an explicit
`Unknown`, `Indeterminate`, or reconciliation state carrying enough identity and evidence to
resolve or safely manage the outcome.

**Intent.** Preserve truth during partial failure rather than force every history into success
or failure.

**Applicability.** Payment capture, message acknowledgement, ambiguous commit, remote
provisioning, email submission, and similar distributed effects.

**Allowed exceptions.** None when ambiguity is possible and consequential.

**Review evidence.** Outcome variants, operation and idempotency identity, durable storage,
reconciliation procedure, audit trail, and tests that unknown never becomes confirmed failure
without new evidence.

## RUST-DOC-0001-R016 — Make escape hatches explicit

**Statement.** Every public or privileged construction bypass MUST be visibly named,
documented, scoped, owned, and reviewed; ordinary boundary adapters MUST NOT use it.

**Intent.** Keep migrations, trusted constants, or performance paths from silently becoming
general invariant-forging APIs.

**Applicability.** `unchecked`, raw, privileged, feature-gated, administrative, test, and
migration constructors.

**Allowed exceptions.** Test-only constructors MAY have broader convenience when confined to
non-production builds and incapable of leaking into public APIs.

**Review evidence.** Search inventory, visibility and feature analysis, precondition
documentation, call-site list, and safe-interface tests.

## RUST-DOC-0001-R017 — Scope unsafe constructors narrowly

**Statement.** An unsafe constructor MUST state the complete caller proof obligation and MUST
be no broader than the invariant that safe code cannot verify.

**Intent.** Treat unsafe construction as transferred proof responsibility, not permission to
skip validation.

**Applicability.** Raw pointers, FFI wrappers, unchecked UTF or layout conversion, and
performance-sensitive trusted construction.

**Allowed exceptions.** None to documentation or soundness. Avoid unsafe when a checked safe
constructor is practical.

**Review evidence.** RUST-DOC-0007 review, safety section, encapsulation, invalid-input
analysis, Miri or sanitizer evidence where applicable, and all call sites.

## RUST-DOC-0001-R018 — Prove important prohibited programs

**Statement.** Compile-fail tests SHOULD demonstrate compiler rejection of important direct
construction, wrong-state operations, forged authority, or reuse after consumption.

**Intent.** Bind a type-level claim to executable evidence and detect accidental public API
weakening.

**Applicability.** Public or reusable APIs whose primary benefit is compiler prevention.

**Allowed exceptions.** Runtime-only invariants or unstable diagnostics may use API compile
tests plus other structural evidence when a compile-fail harness would be brittle without
adding meaningful confidence.

**Review evidence.** Minimal UI case, reviewed diagnostic, pinned toolchain, and positive
counterpart test.

## RUST-DOC-0001-R019 — Publish guarantees and non-guarantees

**Statement.** Every major trusted type and state transition MUST document its exact guarantee
beside its non-guarantees, escape hatches, boundary preservation, and residual runtime risk.

**Intent.** Stop local evidence from expanding into claims about external liveness, business
policy, distributed certainty, or universal correctness.

**Applicability.** Public domain types, capabilities, typestate APIs, persisted states, and
case-study designs.

**Allowed exceptions.** Trivial private wrappers MAY rely on a nearby module-level guarantee
ledger if every constructor and use is covered.

**Review evidence.** Completed guarantee ledger traced to code, tests, boundaries, and effect
outcomes.

## RUST-DOC-0001-R020 — Keep cross-entity and temporal facts at runtime

**Statement.** Cross-entity, temporal, policy-dependent, and externally mutable invariants MUST
be revalidated by the owning runtime service or transaction when current truth is required.

**Intent.** Avoid stale types that claim balance, authorization, uniqueness, liveness, or
policy acceptance after the underlying fact may change.

**Applicability.** Account funds, inventory, tenant membership, session revocation, uniqueness,
foreign exchange, and multi-record totals.

**Allowed exceptions.** Immutable snapshots MAY carry historical evidence when the name and
API make the observation time and scope explicit.

**Review evidence.** Owner, transaction or observation boundary, concurrency controls,
staleness policy, failure type, and race tests.

## RUST-DOC-0001-R021 — Model money without false arithmetic guarantees

**Statement.** Monetary types MUST carry currency and enforce the documented amount invariant;
arithmetic MUST check currency compatibility and MUST NOT claim that integer representation
eliminates tax, foreign-exchange, allocation, or rounding policy.

**Intent.** Prevent zero/negative amounts where prohibited, accidental currency mixing, binary
floating-point representation error, and overstatement of what minor units solve.

**Applicability.** Prices, invoices, payments, fees, balances, allocations, and settlement.

**Allowed exceptions.** A domain with exactly one fixed currency MAY bind currency at the
aggregate or module level if accidental mixing is structurally impossible and documented.

**Review evidence.** `u64`/`NonZeroU64` semantics, overflow behavior, same-currency tests,
rounding and allocation policy location, and non-guarantee statement.

## RUST-DOC-0001-R022 — Separate email syntax from ownership

**Statement.** An email-address type MUST document its actual syntax policy; mailbox ownership
or external verification MUST require separate verifier-produced evidence.

**Intent.** Prevent checks such as `contains('@')` from being represented as meaningful
deliverability or ownership proof.

**Applicability.** User contact, authentication, notification, and account-recovery addresses.

**Allowed exceptions.** A raw contact string MAY remain unrefined when the system does not
claim email semantics and safely treats delivery failure.

**Review evidence.** Syntax policy tests, private representation, verifier-only proof path,
expiry or revocation considerations, and deliverability non-guarantee.

---

## Source: `doctrines/0001-invalid-states/rationale.md`

# Rationale

## Why compilation is insufficient

Rust compiles many logically contradictory models. A struct can contain `paid: true`,
`failed: true`, `receipt: None`, and a negative floating-point amount represented through
convention. The borrow checker protects memory relationships, not an application's business
meaning. The purpose of this doctrine is to move consequential invalidity out of ordinary
business operations and into explicit construction, state, authority, and boundary design.

The move is selective. External and temporal facts remain runtime concerns. A design is
stronger when it encodes stable local invariants and openly validates everything else than
when it wraps mutable reality in a confident type name.

## State before fields

Consider an invoice:

```rust
struct Invoice {
    paid: bool,
    failed: bool,
    receipt: Option<String>,
    failure_reason: Option<String>,
}
```

The number of representable combinations exceeds the meaningful states. Every consumer must
repeat a validity condition, and one forgotten branch permits contradiction. An enum makes the
association between state and data structural:

```rust
enum InvoiceState {
    Pending,
    Paid { receipt: Receipt },
    Failed { reason: FailureReason },
}
```

Exhaustive matching exposes evolution. Persistence still needs versioning and unknown-variant
policy. A public enum also exposes variant construction, so inner values must themselves be
trusted or construction must be restricted at a higher aggregate boundary.

The same principle helps UI state. Separate booleans such as `is_valid`, `is_submitting`,
`submitted`, and `has_error` admit impossible combinations. A runtime enum can represent
`Draft`, `Validated`, `Submitting`, `Submitted`, `Rejected`, and `Unknown`, carrying the form
or operation identity appropriate to each case. Frontend state does not authorize the backend;
the server remains the authority.

## Refined values and exact claims

An opaque newtype reduces repeated validation when its invariant is stable and local. Positive
minor-unit money can use `NonZeroU64`. This establishes non-zero only. A plain `u64` permits
zero. `NonZeroU64` does not establish a business maximum, sufficient funds, currency agreement,
correct tax, or correct allocation.

Integer minor units avoid binary floating-point representation error for values exactly
expressible in that scale. Monetary systems still need policy for fractional taxes, discounts,
foreign exchange, pro-rata allocation, cash rounding, and overflow. Currency must be carried
or fixed by a scope that makes mixing impossible. Addition should reject different currencies.

Email illustrates evidence levels. `contains('@')` accepts empty local parts, empty domains,
multiple separators, control characters, and many other unusable forms. A bounded example
parser can establish a documented syntax subset; it should be named `EmailAddress`, not
`VerifiedEmailAddress`. Ownership requires a challenge or equivalent external process. Even
verified ownership at one time does not guarantee future control or delivery.

Private fields matter because public construction turns the type into a comment. Complete
constructors matter because a private field with several inconsistent builders provides
different meanings under one name. Boundary preservation matters because a derived
deserializer or ORM can write the field without calling `new`.

## Legal transitions

Ownership can represent local lifecycle. A transaction consumed by `commit(self)` cannot be
committed or rolled back through the same value afterward. An authorized payment can be the
only type exposing `capture`. A closed connection can expose `connect`, while an open
connection exposes `send` and `close`.

This is valuable local evidence. It does not erase failure:

```text
Connection<Closed>
    → Result<Connection<Open>, ConnectError>
    → send(...) → Result<Receipt, SendError>
    → close() → Result<Connection<Closed>, CloseError>
```

`Connection<Open>` means the local connection transition returned success. It cannot guarantee
the remote peer remains reachable. The network may fail immediately after the transition or
during the next `send`. `close` can also fail or become ambiguous depending on protocol.

Consuming APIs need recovery design. If an async transition consumes a value and returns only
an error, the caller may lose state needed to reconcile or retry. An error can return the
previous state, a durable operation identifier, or an explicit unknown state. Ownership
prevents local reuse; it does not decide distributed history.

## Typestate is a tool, not a hierarchy

Typestate can provide clear compiler diagnostics for a small, static protocol under local
ownership. Marker zero-sized types and state-specific impl blocks are implementation
mechanisms. Their cost includes generic API surface, monomorphization, diagnostics, async
recovery, dynamic dispatch, serialization, and migration.

Persisted payment state is dynamic reality. It must be inspected after restart, decoded from a
schema, updated transactionally, and evolved as providers add outcomes. A runtime enum is the
honest primary representation. A hybrid design may create a short-lived `AuthorizedPayment`
capability for one local capture call while retaining a persisted `PaymentStatus`.

State explosion is a stop condition. If a workflow has many orthogonal dimensions — validation,
authorization, fraud review, capture, settlement, reversal, dispute, provider state — generic
cross-products can obscure rather than protect. Runtime state plus validated transition
functions and transactional constraints can be simpler and stronger.

## Authority is distinct from state

Knowing that an object is in a state does not necessarily grant permission to act. A
capability type represents possession of authority and exposes only permitted methods.
Constructor visibility can prevent forgery; non-clonability can preserve single-use or
exclusive authority.

Capabilities still require a contract. A clone can amplify authority. Serialization can leak
it. Revocation can make local possession stale. Transfer across tasks changes custody.
External enforcement may recheck authority. An `AuthorizedPayment` should identify payment,
amount, provider scope, and expiry where those facts constrain capture.

## Persistence and boundary integrity

Serialized or persisted representations are not trusted merely because the local program
wrote them once. Old versions, alternate writers, corrupted storage, migration errors, manual
repairs, and changed policy can violate current invariants.

Serde supports checked adapters such as `try_from`; manual `Deserialize` can parse a raw DTO
then invoke canonical construction. Database code can decode a `RawInvoiceRow` and implement
`TryFrom<RawInvoiceRow> for Invoice`. Invalid historical records should produce a distinct
error and quarantine path rather than be coerced into a nearby valid state.

Schema constraints reinforce domain invariants and protect other writers. They cannot replace
domain validation because the application must reject before effect, provide domain errors,
handle old schema versions, and enforce facts spanning services or external systems.

## External effects and honest uncertainty

A legal local transition can reach an external system and lose certainty. Consider payment
authorization and capture:

```text
draft → validated → authorized → capture requested
```

If the provider returns an accepted capture, the system has confirmed evidence. If it returns
a definitive rejection, the system has rejection evidence. If the request was transmitted and
the connection timed out, success may have occurred. Reporting `Failed` invents non-execution;
blind retry may double the effect.

An explicit outcome records:

```rust
enum CaptureOutcome {
    Confirmed(CaptureReceipt),
    Rejected(CaptureRejection),
    Unknown {
        operation_id: OperationId,
        reconciliation: ReconciliationToken,
    },
}
```

The exact domain type may differ, but the semantics must not. Unknown carries durable identity,
safe next actions, and audit correlation. A reconciliation worker queries or observes the
provider, then produces new evidence. Compensation is a later effect, not rollback of history.

Message delivery has the same shape. A broker may accept a message and lose the acknowledgement.
At-least-once delivery means duplicates must be expected. An idempotency key and durable inbox
can constrain effects, but claims must define scope and retention. "Exactly once" is meaningful
only at a precise boundary with a mechanism.

Database commit can be ambiguous around connection loss. The transaction handle being consumed
prevents local reuse; it does not prove rollback. Database-specific recovery, unique operation
identity, and read-back may be necessary.

## Why alternatives are weaker

Scattered `if` statements repeat rules and allow one path to omit them. Comments and naming do
not protect construction. A giant struct with optional fields admits contradictions. Raw
strings erase evidence levels. Public tuple fields permit forgery. Derived decoding can bypass
complete constructors. Boolean success collapses rejection, local failure, and uncertainty.
Universal typestate can make persistence and evolution harder while still failing to control
external reality.

Runtime checks are not inherently weak. They are the correct mechanism for external, mutable,
cross-entity, and temporal facts. Their strength comes from centralized ownership, transaction
or protocol semantics, structured errors, complete boundary use, and evidence. The doctrine
rejects both under-modeling and type-system overreach.

## Cost of application

Stronger representations add conversion, error types, adapters, test cases, and review work.
Public enums and error variants create compatibility surfaces. Typestate can enlarge compiled
code. Versioned boundaries require migrations. Explicit unknown states require operational
reconciliation.

Those costs are justified when they prevent consequential failure. They are not justified for
every label or harmless transient. The complexity budget asks frequency, impact, control,
persistence, diagnostics, team familiarity, migration, and measured build/runtime cost.

## Evidence limits

Compiler rejection proves selected invalid programs do not type-check against the reviewed
API. Constructor tests show selected inputs are accepted or rejected. Property tests explore a
model. Integration tests cross configured boundaries. None proves universal business
correctness, remote liveness, or future policy.

Guarantee honesty keeps these evidence layers useful. A type should say exactly what it
establishes, how construction is protected, how decoding preserves it, which escape hatches
exist, what changes externally, which failures remain, and where outcomes become unknown.

---

## Source: `doctrines/0001-invalid-states/decision-framework.md`

# Decision framework

## Start with the invariant

Do not begin with `struct`, `enum`, `PhantomData`, or a library. Write a falsifiable invariant,
its owner, classification, boundary, consequence, and residual uncertainty. Then ask whether
the fact is stable and local, dynamic and persisted, relational, authoritative, or external.

The first selection table is:

| Problem                               | Preferred mechanism                                |
| ------------------------------------- | -------------------------------------------------- |
| Mutually exclusive state              | `enum`                                             |
| Validated scalar or identifier        | opaque newtype                                     |
| Non-empty or bounded collection       | validated wrapper                                  |
| Locally controlled operation sequence | typestate or consuming transition                  |
| Authority to perform an operation     | capability type                                    |
| Dynamic or persisted state            | runtime enum/state machine                         |
| External input                        | runtime parse and validation                       |
| Cross-entity business rule            | domain service or transactional runtime validation |
| External success/failure              | `Result`                                           |
| Indeterminate distributed outcome     | explicit unknown/reconciliation state              |

"Preferred" means first candidate, not automatic answer. Multiple mechanisms often compose.

## Operational decision tree

```text
Is the problem mutually exclusive states?
├─ yes → enum / sum type
└─ no
   Is it a single value with a stable local invariant?
   ├─ yes → opaque validated newtype
   └─ no
      Is it a collection invariant?
      ├─ yes → validated collection wrapper
      └─ no
         Is it locally controlled operation sequencing?
         ├─ yes
         │  Is state count small and API static?
         │  ├─ yes → typestate or consuming transitions
         │  └─ no → runtime state machine
         └─ no
            Is it authority?
            ├─ yes → capability type
            └─ no
               Is it external or mutable reality?
               ├─ yes → runtime observation + Result
               │        + explicit uncertainty where needed
               └─ no → ordinary runtime rule or domain service
```

Before accepting the leaf, apply complexity and honesty checks:

- Can the mechanism's proof become stale?
- Does persistence need a runtime discriminant?
- Can an external effect occur without acknowledgement?
- Does an enum already remove the contradiction more simply?
- Does a consuming method prevent the actual misuse without generic state?
- Can callers understand compiler diagnostics?
- Are all constructors and boundary decoders protected?
- What does the mechanism not prove?

## State decision

Use an enum when cases are mutually exclusive, data differs by case, state is inspected at
runtime, or persistence matters. Put only state-relevant data in each variant. Decide unknown
and future variant behavior at external boundaries.

Use a runtime transition service when legality depends on current database facts,
authorization, concurrent version, or external state. Validate expected prior status and
cross-entity invariants transactionally.

Consider typestate when all answers are favorable:

| Question                          | Favorable evidence                                               |
| --------------------------------- | ---------------------------------------------------------------- |
| Is sequencing locally controlled? | Current owner chooses every transition                           |
| Is the graph small?               | Few states and stable transitions                                |
| Is ownership natural?             | Prior state can be consumed without harming recovery             |
| Is storage unnecessary?           | Value is short-lived or runtime conversion is explicit           |
| Are callers static?               | No routine heterogeneous collection or dynamic dispatch          |
| Is failure designed?              | Transition returns prior state, error evidence, or unknown state |
| Are diagnostics usable?           | Compile-fail examples point to the domain mistake                |
| Is cost proportionate?            | Consequence exceeds API/build/maintenance cost                   |

If several answers are unfavorable, stop and use a runtime enum or consuming method.

## Value decision

Use an opaque newtype when the invariant:

- concerns one value;
- is stable enough to name;
- can be checked without mutable external state;
- is valuable after construction;
- and can be preserved through all mutations and boundaries.

Choose the evidence level before the name. For email, decide whether the type means parsed
input, a documented syntax subset, policy acceptance, ownership verification, or delivery
observation. Do not compress those levels into one `ValidEmail`.

For money, decide:

- signed, non-negative, positive, or bounded amount;
- minor-unit scale;
- currency representation;
- overflow policy;
- same-currency arithmetic;
- rounding and allocation owner;
- foreign-exchange boundary.

`u64` includes zero. `NonZeroU64` excludes zero but does not encode policy maximum or currency.

## Collection decision

Identify whether the invariant is non-empty, bounded, sorted, unique, capacity-limited, or
relational among members. A wrapper is only valid if it controls:

- vector or set creation;
- extension and insertion;
- removal and clearing;
- mutable slice or inner-container access;
- `FromIterator`;
- deserialization and database decoding.

If mutation is broad and invariant checks are cheap, an ordinary collection plus an
operation-level validator may be clearer.

## Authority decision

Use a capability when local possession should enable a narrow operation and forgery must be
hard. Record:

- issuer and protected constructor;
- resource, tenant, operation, and amount scope;
- whether cloning is valid;
- transfer and task ownership;
- serialization policy;
- expiry and revocation;
- use count;
- external recheck;
- secret handling.

If authority changes frequently and every use must query a policy engine, model the runtime
authorization decision explicitly rather than claiming durable authority.

## Boundary decision

For each HTTP DTO, message, row, file, configuration, or FFI value:

1. bound size and resource use;
2. parse into a structural representation;
3. normalize according to explicit policy;
4. validate using canonical constructors;
5. authenticate and authorize separately;
6. preserve unknown versions safely;
7. convert errors without erasing category;
8. retain operation identity for effects;
9. test invalid and historical values;
10. state what remains uncertain.

If a derive constructs private fields directly, it is a bypass unless the derive delegates to
checked conversion.

## Effect and outcome decision

Mark the external commitment point. Then classify outcomes:

| Observation                              | Domain meaning         | Retry posture                        |
| ---------------------------------------- | ---------------------- | ------------------------------------ |
| Request definitely not sent              | local failure          | retry may be safe after policy check |
| Definitive accepted response             | confirmed success      | do not repeat effect                 |
| Definitive rejected response             | confirmed rejection    | retry only if rejection is retriable |
| Sent, response lost or timed out         | unknown                | reconcile or use proven idempotency  |
| Cancellation before commitment confirmed | cancelled/non-executed | retry may be safe                    |
| Cancellation acknowledgement absent      | unknown                | reconcile                            |

Define idempotency key scope, uniqueness, retention, and replay response. If those properties
are unknown, do not label retry safe.

## Choosing a simpler mechanism

Choose ordinary runtime validation instead of a new type when:

- the fact is used once at the boundary;
- it depends on mutable external or cross-entity state;
- the invalid state has low consequence and an immediate structured error;
- a wrapper would expose unrestricted mutation;
- persistence or dynamic inspection erases the type on every operation;
- or the team cannot maintain the abstraction safely.

Choose a consuming method instead of full typestate when only double use matters. Choose an
enum instead of boolean combinations. Choose a capability instead of a broad service object
when authority surface is the concern.

## Stop conditions

Stop adding type machinery when it no longer removes a named consequential invalid program,
when state combinations multiply faster than the domain, when compiler errors cease to express
the mistake, when serialization requires pervasive erasure, or when external reality makes the
proof stale immediately.

Stop simplifying when a public bypass remains, the same invariant is scattered across
callers, wrong-state effects are plausible and severe, or an unknown outcome is still forced
into success/failure.

The final decision includes a guarantee ledger, boundary map, evidence plan, and a trigger for
revisiting the representation.

---

## Source: `doctrines/0001-invalid-states/review-standard.md`

# Review standard

Record each gate as pass, fail, not applicable, or a waiver reference. "Looks idiomatic" is
not evidence.

## Gate 1 — Invariant inventory

**Question.** Are consequential value, state, transition, authority, boundary, cross-entity,
temporal, and distributed invariants identified with owners?

**Pass evidence.** Inventory links each statement to enforcement, boundary, consequence, and
residual uncertainty.

**Failure examples.** Types were selected first; ownership or timeout semantics are absent.

**Severity.** Critical when effects or authority are involved; otherwise major.

**Remediation.** Complete discovery and reconsider representation.

## Gate 2 — Mutually exclusive states

**Question.** Can booleans, options, or discriminants express contradictory states, and can a
field carrying a closed vocabulary hold a value outside it?

**Pass evidence.** Enum variants carry only relevant data; external DTO contradiction is
rejected during conversion; a closed vocabulary is decoded into a type that cannot hold an
unknown value, or into a validated newtype whose rejection of one is tested.

**Failure examples.** Paid without receipt; failed and submitted simultaneously; a status field
decoded as a string and compared against literals, so a misspelled value matches no branch and
is silently treated as absent.

**Severity.** Major.

**Remediation.** Introduce a sum type and migration plan; decode the vocabulary at the boundary
and replace literal comparisons with matching.

## Gate 3 — Construction protection

**Question.** Can untrusted callers construct a trusted value without complete validation?

**Pass evidence.** Private fields, complete fallible constructors, restricted proof issuance,
and no weaker `From`, builder, default, macro, or re-export.

**Failure examples.** Public tuple field; public `from_raw`; builder skips one rule.

**Severity.** Critical for authority or safety; major otherwise.

**Remediation.** Restrict representation and consolidate construction.

## Gate 4 — Evidence-accurate names

**Question.** Does every type and state name match the evidence actually established?

**Pass evidence.** Guarantee ledger maps names to producers and non-guarantees.

**Failure examples.** `VerifiedEmail` from syntax parser; `Open` documented as remote liveness.

**Severity.** Major; critical when it drives security or external retry.

**Remediation.** Narrow name or strengthen evidence and protected construction.

## Gate 5 — Serde and format decoding

**Question.** Does every deserializer preserve the canonical invariant?

**Pass evidence.** Raw DTO plus `TryFrom`, Serde `try_from`, or equivalent manual validation;
invalid and oversized input tests.

**Failure examples.** Derived `Deserialize` writes private field; unknown variant maps to
default.

**Severity.** Critical at untrusted boundaries.

**Remediation.** Decode structurally, validate canonically, define version behavior.

## Gate 6 — Database decoding

**Question.** Can historical or alternate-writer rows forge domain values?

**Pass evidence.** Checked row conversion, schema constraints, invalid-history quarantine,
migration and version tests.

**Failure examples.** ORM derive directly constructs trusted type; invalid row is coerced.

**Severity.** Critical for financial, authority, or safety data; otherwise major.

**Remediation.** Separate persistence representation and validate.

## Gate 7 — Mutation preservation

**Question.** Can mutation, dereferencing, iteration, cloning, or collection conversion erode
the invariant or authority?

**Pass evidence.** Controlled methods, read-only borrowing, clone rationale, negative tests.

**Failure examples.** `NonEmptyVec` exposes `clear`; capability derives `Clone` without scope.

**Severity.** Major or critical by consequence.

**Remediation.** Narrow API or move validation to every mutation.

## Gate 8 — Transition legality

**Question.** Do APIs prevent significant wrong-order or repeated local operations?

**Pass evidence.** Consuming transition or transactional runtime validator; compile-fail or
concurrency tests.

**Failure examples.** Capture accepts draft payment; transaction can commit twice.

**Severity.** Critical for irreversible effects; major otherwise.

**Remediation.** Encode prior evidence or validate atomically.

## Gate 9 — Typestate proportionality

**Question.** Is typestate locally controlled, small, static, recoverable, and cheaper than
runtime alternatives?

**Pass evidence.** Complexity record, state graph, diagnostics, persistence decision, async
failure semantics.

**Failure examples.** Generic state mirrors dozens of persisted provider statuses.

**Severity.** Major maintainability concern; critical if it creates false certainty.

**Remediation.** Simplify to runtime enum, consuming method, or hybrid.

## Gate 10 — Authority

**Question.** Are issuance, scope, clone, transfer, serialization, expiry, revocation, and use
count defined?

**Pass evidence.** Capability construction and call-site audit plus misuse tests.

**Failure examples.** Public capability constructor; serializable admin token; stale grant.

**Severity.** Critical.

**Remediation.** Restrict issuance and define mutable authority checks.

## Gate 11 — External fallibility

**Question.** Do local state proofs leave every external effect fallible?

**Pass evidence.** Structured results distinguish expected categories and preserve sources.

**Failure examples.** `send` returns receipt without error; destructor claims external rollback.

**Severity.** Critical when failure would corrupt state; otherwise major.

**Remediation.** Restore fallible API and recovery semantics.

## Gate 12 — Commitment and timeout

**Question.** Can failure of acknowledgement occur after external commitment?

**Pass evidence.** Protocol analysis identifies commitment point and maps timeout to explicit
unknown where necessary.

**Failure examples.** Timeout becomes `Failed`; automatic retry has no idempotency analysis.

**Severity.** Critical.

**Remediation.** Add unknown outcome, identity, and reconciliation.

## Gate 13 — Unknown outcome durability

**Question.** Can uncertainty survive process restart and be resolved without losing causality?

**Pass evidence.** Durable operation ID, idempotency key, provider scope, reconciliation token,
audit correlation, and worker tests.

**Failure examples.** Unknown exists only in memory; restart retries blindly.

**Severity.** Critical.

**Remediation.** Persist uncertainty before or atomically with operation progress.

## Gate 14 — Money contract

**Question.** Are amount, currency, overflow, arithmetic compatibility, and rounding/allocation
policy represented honestly?

**Pass evidence.** Non-zero or bounded constructor, same-currency tests, policy owner and
non-guarantees.

**Failure examples.** `u64` called positive; integers described as eliminating rounding.

**Severity.** Critical for movement of funds; major for display-only calculations.

**Remediation.** Refine amount and currency; document policy boundaries.

## Gate 15 — Email evidence

**Question.** Are syntax, policy acceptance, ownership verification, and deliverability kept
distinct?

**Pass evidence.** Documented parser, verifier-owned transition, expiry or revocation policy,
and delivery failure handling.

**Failure examples.** `contains('@')`; syntax result named verified.

**Severity.** Critical for authentication or recovery; major otherwise.

**Remediation.** Separate evidence levels and constructors.

## Gate 16 — Escape hatches

**Question.** Are all unchecked, unsafe, administrative, migration, test, and feature-gated
bypasses explicit and scoped?

**Pass evidence.** Search inventory and reviewed call sites with documented preconditions.

**Failure examples.** Ordinary decoder calls `new_unchecked`; test helper is enabled in
production.

**Severity.** Critical.

**Remediation.** Remove, restrict, or govern under RUST-DOC-0007.

## Gate 17 — Compiler evidence

**Question.** Are important prohibited programs tested and their diagnostics semantically
reviewed?

**Pass evidence.** Minimal compile-fail cases plus positive tests on pinned stable.

**Failure examples.** UI test fails for unused import; diagnostics overwritten after upgrade.

**Severity.** Major.

**Remediation.** Target the intended rejection and inspect `.stderr`.

## Gate 18 — Guarantee ledger

**Question.** Are guarantees, construction, decoding, escape hatches, non-guarantees, and
residual risk complete?

**Pass evidence.** Ledger entries trace to exact code and tests.

**Failure examples.** Documentation states "impossible" while public bypass or external
mutation exists.

**Severity.** Critical for misleading safety/security claims; otherwise major.

**Remediation.** Narrow claim or strengthen mechanism and evidence.

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0001-R001`, `RUST-DOC-0001-R002`, `RUST-DOC-0001-R003`, `RUST-DOC-0001-R004`
- `RUST-DOC-0001-R005`, `RUST-DOC-0001-R006`, `RUST-DOC-0001-R007`, `RUST-DOC-0001-R008`
- `RUST-DOC-0001-R009`, `RUST-DOC-0001-R010`, `RUST-DOC-0001-R011`, `RUST-DOC-0001-R012`
- `RUST-DOC-0001-R013`, `RUST-DOC-0001-R014`, `RUST-DOC-0001-R015`, `RUST-DOC-0001-R016`
- `RUST-DOC-0001-R017`, `RUST-DOC-0001-R018`, `RUST-DOC-0001-R019`, `RUST-DOC-0001-R020`
- `RUST-DOC-0001-R021`, `RUST-DOC-0001-R022`

---

## Source: `doctrines/0001-invalid-states/anti-patterns.md`

# Anti-patterns

## Boolean state soup

**Weak example.** A struct contains `is_paid`, `is_failed`, `is_sending`, optional receipt,
and optional failure reason.

**Why it fails.** Independent fields represent contradictory combinations and force repeated
checks.

**Risk.** Invalid persistence, unreachable UI branches, missing state-specific evidence.

**Improved direction.** Use an enum whose variants carry receipt, reason, form, or operation
identity.

**Justified appearance.** An untrusted legacy DTO may retain the shape solely for checked
conversion and migration.

## Public validated tuple field

**Weak example.** `pub struct PositiveMoney(pub u64);`

**Why it fails.** Any caller can construct zero; the wrapper proves only nominal distinction.

**Risk.** Business code omits checks because the name implies validation.

**Improved direction.** Private `NonZeroU64`, fallible constructor, currency-aware aggregate,
and checked arithmetic.

**Justified appearance.** A public wrapper can be appropriate when it deliberately asserts no
validation and the name does not imply one.

## Partial smart constructor

**Weak example.** `EmailAddress::new` checks only `contains('@')`.

**Why it fails.** The check does not establish a meaningful documented syntax policy, and
another constructor may be weaker still.

**Risk.** Invalid contact data, inconsistent boundary behavior, misleading verification
claims.

**Improved direction.** Define a bounded syntax policy, test it, and separate ownership
verification.

**Justified appearance.** A permissive raw input type is fine when it is named raw and all
delivery failure remains expected.

## Derive bypass

**Weak example.** A trusted private-field newtype derives `Deserialize` directly.

**Why it fails.** The decoder may populate representation without calling the constructor.

**Risk.** Network, cache, or file input forges trusted evidence.

**Improved direction.** Deserialize a raw representation and delegate through `TryFrom` or a
manual visitor.

**Justified appearance.** Direct derive is safe for a representation whose Rust-valid values
all satisfy the complete invariant.

## Trusted database myth

**Weak example.** ORM row mapping returns `VerifiedEmailAddress` from a text column without
verification metadata or validation.

**Why it fails.** Historical rows, other writers, manual repair, and policy change are outside
the current proof.

**Risk.** Forged evidence and irreparable migration ambiguity.

**Improved direction.** Decode a raw row, validate syntax and evidence fields, quarantine
invalid history, and version policy.

**Justified appearance.** Direct mapping can be valid for database-native values whose entire
claim is exactly enforced by the schema and decoder.

## Typestate everywhere

**Weak example.** Every persisted payment provider status becomes a generic marker type and
all combinations become separate concrete types.

**Why it fails.** External state is dynamic, persisted, heterogeneous, and mutable. Generic
states create explosion without external control.

**Risk.** Erasure at every boundary, stale proof, poor diagnostics, migration friction.

**Improved direction.** Runtime status enum plus transactional transition validation; add a
short-lived local capability where useful.

**Justified appearance.** A small local builder or connection protocol may benefit from
typestate.

## Infallible open connection

**Weak example.** `Connection<Open>::send` returns a receipt directly because the type is open.

**Why it fails.** The type records a local historical transition; the peer can close
immediately.

**Risk.** Panic, hidden retry, data loss, or false success.

**Improved direction.** Return structured `Result` and document remote-liveness
non-guarantee.

**Justified appearance.** A pure in-memory mock may be infallible if its type does not claim
network semantics and is not substituted where failure behavior matters.

## Timeout means failure

**Weak example.** Any payment-provider timeout transitions persisted state to `Failed`.

**Why it fails.** The request may have committed and only the response was lost.

**Risk.** Duplicate capture on retry and a false audit record.

**Improved direction.** Store `UnknownCapture` with operation and reconciliation identity.

**Justified appearance.** A protocol-defined pre-commit timeout may establish non-execution
when that guarantee is verified.

## Integer money solves rounding

**Weak example.** Documentation states that minor-unit integers eliminate rounding.

**Why it fails.** Tax, foreign exchange, discounts, allocation, and currencies with differing
scales still produce fractions and policy choices.

**Risk.** Silent bias, reconciliation differences, accounting defects.

**Improved direction.** State representation guarantee separately from calculation,
allocation, and rounding policy.

**Justified appearance.** An operation that only stores and adds same-scale whole minor units
may require no rounding at that step; document the narrow scope.

## Clone the capability

**Weak example.** An exclusive capture token derives `Clone` for convenience.

**Why it fails.** Local authority becomes duplicable and consuming one clone does not consume
the others.

**Risk.** Repeated irreversible effects or authority leakage.

**Improved direction.** Remove clone, issue operation-scoped identity, and make provider
idempotency a separate explicit control.

**Justified appearance.** Read-only shared capability can be cloneable when duplication is
part of the authority contract and revocation is handled.

## Unchecked constructor as ordinary API

**Weak example.** `pub fn new_unchecked(value: String) -> Self` is safe and used by all
adapters for speed.

**Why it fails.** Proof responsibility is invisible and boundaries bypass the canonical
policy.

**Risk.** Invariant erosion spreads through apparently valid types.

**Improved direction.** Remove it, restrict visibility, or make the exceptional precondition
and ownership explicit; measure before adding a fast path.

**Justified appearance.** A narrow unsafe constructor can support verified FFI or a measured
internal path under complete documented obligations and safe encapsulation.

## Type-level cross-entity truth

**Weak example.** `FundedAccount` is constructed once and later treated as proof that funds
remain sufficient.

**Why it fails.** Concurrent withdrawals, holds, expiry, or external updates change the fact.

**Risk.** Overspend or authorization bypass.

**Improved direction.** Treat the type as a timestamped observation or revalidate within the
transaction that spends funds.

**Justified appearance.** An immutable snapshot can carry historical evidence when its name
and API make staleness explicit.

## Test names as proof

**Weak example.** A test called `invalid_states_are_impossible` constructs only valid values.

**Why it fails.** The violation path and alternate constructors remain untested.

**Risk.** Review mistakes a label for evidence.

**Improved direction.** Add negative constructor tests, boundary tests, and compile-fail cases
for the precise prohibited programs.

**Justified appearance.** Broad scenario names are harmless when assertions and adjacent
documentation clearly state evidence scope.

---

## Source: `doctrines/0001-invalid-states/glossary.md`

# Glossary

**Capability type**

A value whose possession grants a bounded operation. Its authority depends on protected
issuance, scope, clone and transfer semantics, expiry, and revocation.

**Consuming transition**

A method taking ownership of the prior state or authority so ordinary safe code cannot reuse
it after transition.

**Contradictory state**

A representable combination that the domain declares impossible, such as paid and failed
simultaneously.

**Evidence-accurate name**

A name whose implied claim does not exceed what constructors, transitions, or observations
establish.

**Escape hatch**

A construction or mutation path that assumes rather than checks the ordinary invariant,
including unchecked, unsafe, administrative, migration, or privileged paths.

**Hybrid state machine**

A design using runtime state for persistence or external observation and compile-time state or
capabilities for a bounded local operation.

**Indeterminate outcome**

A state in which the system lacks evidence to classify an external effect as confirmed
success or confirmed non-execution/rejection.

**Opaque newtype**

A wrapper with representation hidden from ordinary callers so validated construction gives
the type evidentiary meaning.

**Protected construction**

Visibility, proof tokens, fallible constructors, and controlled mutation that prevent
untrusted code from forging the claimed evidence.

**Reconciliation identity**

Durable operation, provider, correlation, or token data sufficient to observe and resolve an
indeterminate external outcome.

**Runtime state machine**

An explicit state representation and validated transition function evaluated at runtime,
often required for persistence, dynamic inspection, concurrency, or external state.

**Smart constructor**

A fallible constructor that parses, normalizes, validates, or requires proof before producing
a trusted type.

**Sum type**

An enum or equivalent representation where a value is exactly one of several variants, each
with its own data.

**Typestate**

Compile-time representation of a local object's protocol phase through distinct concrete
types, generic marker states, or state-specific implementations.

**Trusted domain representation**

A value whose documented invariant has been established through reviewed construction. Trust
is scoped to that invariant and does not imply external certainty.

**Validated collection**

A wrapper that establishes and preserves a whole-collection property such as non-empty,
bounded, sorted, unique, or compatible members.

---

## Source: `doctrines/0001-invalid-states/references.md`

# References

Primary and authoritative sources:

- [The Rust Reference: types](https://doc.rust-lang.org/reference/types.html) defines language
  type forms and their semantics.
- [The Rust Reference: visibility and privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html)
  defines module-scoped access used to protect construction.
- [`std::num::NonZeroU64`](https://doc.rust-lang.org/std/num/type.NonZeroU64.html) documents
  the exact non-zero integer guarantee and niche behavior.
- [`std::convert::TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) defines
  fallible value-to-value conversion used by boundary adapters.
- [Rust API Guidelines: type safety](https://rust-lang.github.io/api-guidelines/type-safety.html)
  discusses newtypes and static enforcement.
- [Serde container attributes](https://serde.rs/container-attrs.html) documents `try_from`,
  `from`, `into`, and representation controls relevant to checked decoding.
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) explains proof obligations that arise
  when unsafe code performs construction the compiler cannot verify.
- [trybuild documentation](https://docs.rs/trybuild/) defines the stable UI-test harness used
  for compiler-rejection evidence.
- [RFC 2008: non-exhaustive types](https://rust-lang.github.io/rfcs/2008-non-exhaustive.html)
  informs public enum evolution and unknown future cases.
- [Gray and Cheriton, "Leases: An Efficient Fault-Tolerant Mechanism for Distributed File
  Cache Consistency"](https://dl.acm.org/doi/10.1145/74850.74870) is a foundational treatment
  of time-bounded distributed authority and uncertainty.
- [RFC 9110: HTTP Semantics](https://www.rfc-editor.org/rfc/rfc9110) defines method semantics,
  retries, and idempotency terminology used at HTTP boundaries.

Pedagogical provenance for the originating video and the doctrine's accepted, refined, and
added claims is recorded under `sources/0001-invalid-states/`. The video is not a language
specification; normative claims in this package are bounded by the primary sources and the
explicit contracts above.

---

## Source: `doctrines/0002-error-modeling/README.md`

---
id: RUST-DOC-0002
slug: error-modeling
title: Error Modeling as Domain Design
status: active
version: 0.1.0
normative: true
applies_to:
  - planning
  - implementation
  - review
  - audit
  - maintenance
risk_domains:
  - api-design
  - failure-semantics
  - operations
  - security
supersedes: []
superseded_by: null
---

# Error Modeling as Domain Design

## Scope

This doctrine governs errors exposed by Rust libraries, applications, protocols, boundaries,
and operations. It treats failure as domain evidence: callers may need to distinguish
validation, rejection, conflict, cancellation, timeout, local resource failure, and an
indeterminate external effect.

It covers structured public errors, application reports, source chains, context, retry and
recovery guidance, panic boundaries, conversion, redaction, observability, and compatibility.

## Out of scope

The package does not prescribe one error crate, require a public variant for every internal
cause, or promise that typed errors make recovery possible. It does not replace distributed
outcome modeling, security response policy, or protocol specifications.

## Readers and status

Planners define the failure vocabulary before APIs. Implementers preserve actionable
categories and sources. Reviewers trace conversion and retry. Auditors search for hidden
indeterminacy, panic on external input, secret leakage, and category erasure. `doctrine.md` is
normative under `foundations/normative-language.md`; other package files explain and
operationalize it.

## Prerequisites and related material

Read the [invariant](../foundations/invariants.md), [evidence](../foundations/evidence.md),
[trust-boundary](../foundations/trust-boundaries.md), and
[guarantee-honesty](../foundations/guarantee-honesty.md) foundations. Related doctrines are
[0001](../doctrines/0001-invalid-states/), [0004](../doctrines/0004-concurrency-and-async/),
[0005](../doctrines/0005-persistence-boundaries/), [0006](../doctrines/0006-distributed-uncertainty/), and
[0008](../doctrines/0008-testing-and-evidence/). Related guides include
[sum types](../patterns/sum-types.md),
[explicit uncertainty](../patterns/explicit-uncertainty.md),
[HTTP/RPC](../boundaries/http-and-rpc.md), [messaging](../boundaries/messaging.md),
[database decoding](../boundaries/database-decoding.md), and
[distributed-effects review](../reviews/distributed-effects-review.md).

## Reading order and summary

Read normative rules, rationale, decision framework, review standard, anti-patterns, glossary,
and references. Core obligations:

- model operationally distinct failures as distinct structured cases;
- preserve source errors and machine-actionable context;
- state recoverability and retryability rather than infer them from transport labels;
- preserve cancellation, timeout, rejection, and unknown outcome;
- use panic only for violated internal assumptions or unrecoverable programmer faults;
- justify production `unwrap` and `expect`;
- redact secrets at every recipient boundary;
- and treat public error shape as compatibility surface.

---

## Source: `doctrines/0002-error-modeling/doctrine.md`

# Normative doctrine

## RUST-DOC-0002-R001 — Define a failure inventory

**Statement.** APIs with consequential failure MUST identify failure categories, caller
actions, commitment semantics, recipients, and evidence before selecting an error type.

**Intent.** Prevent implementation details or string messages from becoming the accidental
contract.

**Applicability.** Public libraries, service operations, external effects, persistence, and
security-sensitive flows.

**Allowed exceptions.** Trivial private helpers MAY reuse the enclosing operation's inventory.

**Review evidence.** Failure table mapping causes to variants, recovery, retry, logging,
protocol status, and uncertainty.

## RUST-DOC-0002-R002 — Use structured library errors

**Statement.** Library APIs MUST NOT use opaque string errors as their primary public contract
when callers can respond differently to failure categories.

**Intent.** Preserve machine-actionable meaning independently of human wording.

**Applicability.** Reusable crates and module boundaries with multiple operational outcomes.

**Allowed exceptions.** An opaque non-exhaustive error object MAY be used when no stable
category can be promised, provided callers have documented inspection or reporting semantics.

**Review evidence.** Public enum or equivalent typed interface, match examples, and stability
policy.

## RUST-DOC-0002-R003 — Distinguish actionable categories

**Statement.** Validation failure, policy rejection, authorization denial, conflict,
cancellation, timeout, resource exhaustion, local I/O failure, and indeterminate outcome MUST
remain distinguishable when they require different caller or operator action.

**Intent.** Prevent unsafe retry, misleading user messages, and loss of reconciliation.

**Applicability.** Any operation where at least two listed outcomes differ operationally.

**Allowed exceptions.** Categories MAY be coarsened at an outer recipient boundary when the
recipient cannot act differently and observability retains safe internal detail.

**Review evidence.** Outcome-to-action matrix and conversion tests.

## RUST-DOC-0002-R004 — Preserve sources

**Statement.** Error wrapping and conversion SHOULD preserve the originating error through a
source chain when doing so is safe and useful for diagnosis.

**Intent.** Retain causal evidence while adding domain context.

**Applicability.** I/O, parsing, serialization, database, protocol, and dependency errors.

**Allowed exceptions.** Security, privacy, compatibility, or cross-process boundaries MAY
replace the exposed source with a sanitized internal correlation record.

**Review evidence.** `source()` chain tests or report inspection, plus redaction review.

## RUST-DOC-0002-R005 — Add context without erasing category

**Statement.** Application context SHOULD identify the failed operation and relevant
non-sensitive identity without replacing machine-actionable categories with formatted text.

**Intent.** Make diagnosis specific while retaining programmatic action.

**Applicability.** Layered application operations, job processing, and boundary adapters.

**Allowed exceptions.** A terminal application boundary MAY use an opaque report after all
control decisions have been made.

**Review evidence.** Context chain, correlation ID, structured fields, and user-facing
redaction.

## RUST-DOC-0002-R006 — State recoverability

**Statement.** Recoverability MUST be explicit at the decision point; callers MUST NOT infer
that every `Err` leaves state unchanged or reusable.

**Intent.** Account for partial mutation, consumed authority, cancellation, ambiguous commit,
and external side effects.

**Applicability.** Stateful, consuming, transactional, asynchronous, and external operations.

**Allowed exceptions.** Pure functions MAY document the conventional no-side-effect error
contract once at module level.

**Review evidence.** Post-error state contract, returned recovery value or token, and tests.

## RUST-DOC-0002-R007 — Type retry guidance

**Statement.** Retryability MUST NOT be inferred solely from a generic transport class,
status family, or error string. Retry policy MUST account for operation semantics,
idempotency, attempt budget, backoff, and external commitment.

**Intent.** Prevent duplicates, retry storms, and repeated permanent rejection.

**Applicability.** Network, database, broker, and other transient-looking errors.

**Allowed exceptions.** None where the operation can cause a consequential effect.

**Review evidence.** Typed retry decision, idempotency analysis, budget, jitter, and fault
tests.

## RUST-DOC-0002-R008 — Preserve indeterminate outcomes

**Statement.** Error conversion MUST NOT convert an indeterminate external effect into
confirmed rejection or non-execution.

**Intent.** Keep the system's account of reality honest and enable reconciliation.

**Applicability.** Timeout, acknowledgement loss, ambiguous commit, cancellation race, or
connection loss after possible send.

**Allowed exceptions.** A protocol-proven pre-commit failure MAY be classified as
non-execution.

**Review evidence.** Commitment analysis, explicit unknown type, reconciliation identity, and
conversion tests.

## RUST-DOC-0002-R009 — Bound panic to programmer faults

**Statement.** Panics MUST be reserved for violated internal invariants or unrecoverable
programmer errors, not expected external, user, configuration, or data failure.

**Intent.** Keep expected failure in the declared control-flow and cleanup model.

**Applicability.** Production library and application paths.

**Allowed exceptions.** Process startup MAY deliberately abort on invalid required
configuration after producing a clear sanitized diagnostic, when continued operation is
unsafe and no caller can recover.

**Review evidence.** Panic-site inventory, unwind/abort policy, and boundary failure tests.

## RUST-DOC-0002-R010 — Justify `unwrap` and `expect`

**Statement.** `unwrap` and `expect` in production paths MUST have a locally evident invariant
or explicit justification showing why failure is a programmer defect rather than expected
input or environment.

**Intent.** Prevent hidden panic contracts.

**Applicability.** Non-test Rust code.

**Allowed exceptions.** Tests and examples MAY use them when the panic is not the behavior
being taught and failure location remains clear.

**Review evidence.** Search results, invariant comments where not obvious, and negative tests
for external input.

## RUST-DOC-0002-R011 — Preserve security and reconciliation evidence

**Statement.** Error conversion MUST NOT erase security-relevant denial, authentication
failure, operation correlation, provider reference, or reconciliation identity needed for
safe action and audit.

**Intent.** Avoid turning an authorization event or ambiguous effect into an undifferentiated
internal error.

**Applicability.** Security, financial, distributed, and regulated workflows.

**Allowed exceptions.** Details MAY be withheld from an untrusted recipient while retained in
a protected correlated record.

**Review evidence.** Internal/external mapping, audit fields, access control, and redaction.

## RUST-DOC-0002-R012 — Prevent secret disclosure

**Statement.** Error display, debug, source chains, protocol responses, logs, and telemetry
MUST NOT expose secrets or sensitive internal data to unauthorized recipients.

**Intent.** Ensure diagnosis does not create a confidentiality breach.

**Applicability.** Credentials, tokens, personal data, SQL, paths, provider payloads, and
security decisions.

**Allowed exceptions.** Restricted forensic storage MAY retain necessary evidence under
explicit access and retention policy.

**Review evidence.** Recipient map, redaction tests, debug implementations, and sample logs.

## RUST-DOC-0002-R013 — Govern public error compatibility

**Statement.** Public error categories and inspection behavior MUST be treated as API
compatibility surface; evolution MUST account for exhaustive matching, non-exhaustive design,
error codes, and downstream recovery behavior.

**Intent.** Avoid breaking callers or forcing unstable implementation details into permanent
variants.

**Applicability.** Published crates, versioned protocols, and stable internal platform APIs.

**Allowed exceptions.** Private application errors MAY evolve with coordinated callers.

**Review evidence.** Semver analysis, non-exhaustive strategy, code stability, and migration
notes.

## RUST-DOC-0002-R014 — Log once at an ownership boundary

**Statement.** Errors SHOULD be logged by the layer that owns the final handling decision,
rather than at every propagation layer.

**Intent.** Prevent duplicate events, contradictory severity, and noisy alerts.

**Applicability.** Layered services, jobs, and request handlers.

**Allowed exceptions.** A lower layer MAY emit a distinct metric or trace event when it adds
unique timing or state evidence and correlation prevents double counting.

**Review evidence.** Error path trace, log ownership, event IDs, and alert mapping.

---

## Source: `doctrines/0002-error-modeling/rationale.md`

# Rationale

Errors describe the states in which an operation did not produce its ordinary success value.
That makes them part of domain and protocol design. `Result<T, String>` can carry prose, but
it cannot reliably tell a caller whether to ask the user for correction, retry with the same
idempotency key, reconcile before retry, refresh authorization, or stop permanently.

Structured enums preserve distinctions:

```rust
enum CaptureError {
    Validation(ValidationError),
    Rejected(ProviderRejection),
    Conflict { current_version: u64 },
    LocalTransport(TransportError),
    Unknown { reconciliation: ReconciliationToken },
}
```

The exact shape depends on the API. The important property is that an ambiguous timeout is
not a `Rejected` variant, and an authorization denial is not hidden as transport failure.

Source chains and context serve different readers. A domain category supports control flow.
An underlying I/O or database error supports diagnosis. Context says which operation failed.
An outer report can format that chain for an operator after decisions are complete. Erasing
the category to add a sentence forces later code to parse human text, while exposing every
dependency error directly freezes implementation detail into public API.

`thiserror`-style derives can implement typed errors with sources; the doctrine does not
require that crate. `anyhow`-style opaque reports are useful at an application boundary where
the process will report or terminate and no reusable caller needs stable variants. They are a
poor primary library contract when callers need action.

Retry is semantic. A connection refusal before sending a request may be retriable. A timeout
after sending a non-idempotent capture may require reconciliation. A conflict may be retriable
only after reloading state. A validation error is generally repaired, not retried unchanged.
A provider rejection may carry its own retry window. Generic "transient" labels are evidence
only when defined by the operation's protocol.

Panics express a different contract: safe continuation through the current call stack is not
expected. They fit impossible internal states caused by programmer error, not malformed JSON,
missing files, provider timeouts, or database conflicts. Even an internal invariant panic
needs consideration of unwind versus abort, locks, FFI, and process supervision.

`expect` can document a proof close to code — for example, a regex literal compiled once when
the literal is fixed and known valid — but it should not replace a fallible path for
configuration or user input. "Cannot fail" requires an invariant, not optimism.

Errors also have recipients. A user needs safe corrective information. An operator needs
correlation and category. Telemetry needs bounded fields. A security audit may need protected
detail. Returning raw database or provider messages can disclose tokens, schema, queries,
personal data, or internal topology. Redaction should retain a correlation key rather than
destroy diagnostic evidence.

Public error variants can become semver commitments. A library should expose stable domain
categories, use `#[non_exhaustive]` or an opaque strategy where evolution requires it, and
avoid making every dependency error a top-level variant. Stable codes can support protocols,
but codes must map to documented semantics and must not become strings with unknown meaning.

Logging at each `?` boundary produces several records for one failure, often at different
severity. The handling owner should decide final log level, response, metric, and retry.
Lower layers can attach structured context or trace spans without claiming the event was
unhandled.

Error types do not prove recovery. A transaction error may have consumed a guard. An async
operation may have partially mutated local state. A remote effect may be unknown. Each variant
must state post-error state and safe next actions. This keeps errors as honest evidence rather
than a bucket for everything undesirable.

## Boundary translation

One domain failure can have several recipient-specific representations without losing its
identity. A validation variant may become an HTTP client error with field codes, a CLI
diagnostic with usage help, and a job result that is permanently rejected. The internal
category remains validation. The mapping should be exhaustive and tested so a newly added
variant cannot silently become a generic server error.

Authentication and authorization deserve particular care. Public policy may intentionally
coarsen "resource absent" and "resource forbidden" to avoid disclosure. Internally, the audit
record still needs to distinguish missing resource, invalid credential, denied capability,
and policy failure. Coarsening for one recipient is not permission to discard evidence
globally.

Provider codes can be valuable but are not automatically domain contracts. A boundary adapter
should map documented stable provider categories into domain meaning and retain the raw code
for protected diagnosis. Unknown codes need an explicit safe fallback. Matching the provider's
human message couples behavior to wording and translation.

## Cancellation and partial work

Async cancellation is a control-flow event with state consequences. Dropping a future can
occur at any `await`; local mutation, reserved capacity, locks, durable intent, or transmitted
requests may already exist. A cancellation variant is honest only when the operation defines
what cleanup completed and whether external work may continue. Otherwise cancellation can
produce the same unknown outcome as a timeout.

Consuming APIs should consider returning the original value with a pre-commit error. After a
possible commitment, returning the original authority as if unused can enable repetition.
The error shape may instead carry a reconciliation handle. The type design and error design
therefore form one lifecycle contract.

## Operational stability

Error display text serves humans and may improve without a semver change; variants, stable
codes, retry hints, and source behavior may be relied upon by programs. Documentation should
say which layer is stable. A non-exhaustive public enum lets a library add categories, but
callers still need a safe catch-all action. A stable opaque type can expose methods such as
`kind()`, `is_retryable_under(&operation)`, or `code()` without exposing dependency layout.

Metrics should count categories at the handling boundary and separate attempts from logical
operations. Three retries are one user operation with multiple attempts, not necessarily
three incidents. Unknown effects should remain visible until reconciled; deleting the initial
error after later success loses latency and reliability evidence.

The design should also define equality and cloning deliberately. Many errors contain sources
that are not comparable or cloneable. Forcing `Clone` merely to satisfy a queue may erase the
source into text. A durable job record should store a stable failure category, safe fields,
attempt metadata, and correlation — not pretend to serialize an arbitrary in-memory error
object.

---

## Source: `doctrines/0002-error-modeling/decision-framework.md`

# Decision framework

## Choose the contract boundary

Ask who receives the error and what decisions remain:

| Boundary                  | Preferred form                                                | Reason                                      |
| ------------------------- | ------------------------------------------------------------- | ------------------------------------------- |
| Reusable library          | structured domain enum or stable opaque typed error           | callers need action and compatibility       |
| Internal domain service   | structured enum                                               | preserves business outcomes                 |
| Application orchestration | typed errors until final decision, then opaque report         | combines action with rich context           |
| CLI/process entry         | formatted report and exit code after classification           | no upstream Rust caller                     |
| HTTP/RPC                  | internal typed error mapped to stable public status/code/body | separates protocol recipient from internals |
| Background job            | typed retry/reconcile decision plus correlated report         | scheduler action must be safe               |

Use `thiserror` or equivalent when derivation reduces mechanical implementations without
changing the model. Use an `anyhow`-style report where control decisions are already complete
and arbitrary context is more valuable than public matching. Do not expose an application
report as a library's only error if callers need categories.

## Classify outcomes

For each failure, answer:

1. Was input invalid?
2. Did policy or authority reject it?
3. Did current state conflict?
4. Was the operation cancelled, and at what commitment point?
5. Did a timeout occur before or after possible external execution?
6. Is local state reusable, consumed, or partially changed?
7. Can the same request be retried safely?
8. Must state be reloaded or reconciled first?
9. Which recipient may see details?
10. Which stable code or variant should callers use?

## Retry guidance

Return or compute a typed decision:

```text
DoNotRetry
RetrySameOperation { after, remaining_budget }
RetryAfterRefresh
ReconcileBeforeRetry { operation_id }
Escalate
```

The error alone need not own policy, but the decision must use its structured evidence.
Backoff, jitter, attempt budget, overall deadline, idempotency retention, and nested retry
layers are part of the design.

## Fatal errors and panic

Use an ordinary fatal process error when startup or service continuation is unsafe but the
program can produce a controlled diagnostic and exit. Panic only when a programmer violated
an internal invariant and the chosen process policy treats that as unrecoverable or supervised.

Before panic, ask whether the input came from HTTP, configuration, storage, clock, filesystem,
network, user, or another process. If yes, expected external failure should normally be
returned.

## Coarsening

Errors may be coarsened outward when the recipient cannot act on internal categories.
Preserve:

- security-safe public wording;
- stable public code;
- internal category and source;
- correlation;
- reconciliation identity;
- retry headers or guidance where valid.

Never coarsen confirmed rejection and unknown effect into one public response if the client
must choose different behavior.

## Stop conditions

Stop adding variants when distinctions expose unstable implementation without changing action.
Stop coarsening when a caller must parse strings, retry becomes unsafe, security evidence is
lost, or an unknown outcome disappears. Review the resulting compatibility and redaction
surface.

---

## Source: `doctrines/0002-error-modeling/review-standard.md`

# Review standard

Record pass, fail, not applicable, or waiver for each gate.

| Gate          | Question                                                                            | Pass evidence                  | Failure example                         | Severity | Remediation                       |
| ------------- | ----------------------------------------------------------------------------------- | ------------------------------ | --------------------------------------- | -------- | --------------------------------- |
| Inventory     | Are causes, post-error state, actions, recipients, and commitment semantics listed? | failure matrix                 | error enum copied from dependency       | major    | model domain failures first       |
| Structure     | Can callers distinguish actionable outcomes without parsing text?                   | typed matching tests           | `Result<T, String>`                     | major    | introduce stable categories       |
| Source        | Is causal detail preserved safely?                                                  | source-chain test              | formatted source discarded              | major    | wrap with `source`                |
| Context       | Does context identify operation without erasing category?                           | structured context             | all errors become one sentence          | major    | attach context separately         |
| Validation    | Is invalid input distinct from internal failure?                                    | boundary tests                 | malformed request returns 500           | major    | map validation explicitly         |
| Authorization | Is denial preserved and redacted?                                                   | security mapping               | denial becomes not-found internally too | critical | retain protected audit category   |
| Cancellation  | Is cancellation distinct and commitment-aware?                                      | cancellation tests             | cancelled task assumed rolled back      | critical | define post-cancel state          |
| Timeout       | Can timeout mean unknown execution?                                                 | commitment analysis            | timeout maps to rejection               | critical | add unknown outcome               |
| Retry         | Is retry typed by semantics, idempotency, budget, and backoff?                      | retry table and fault tests    | retry every transport error             | critical | add decision policy               |
| Recovery      | Does each variant state whether values or handles remain usable?                    | API docs and tests             | consumed guard silently lost            | major    | return recovery evidence          |
| Panic         | Are expected external failures returned?                                            | panic inventory                | panic on user JSON                      | critical | make boundary fallible            |
| Unwrap        | Does each production unwrap follow a local invariant?                               | reviewed search                | unwrap database result                  | major    | propagate structured error        |
| Secrets       | Are display, debug, source, response, and logs recipient-safe?                      | redaction tests                | token in provider error                 | critical | sanitize and correlate            |
| Conversion    | Do `From` and mapping preserve security/reconciliation data?                        | conversion tests               | provider reference dropped              | critical | retain fields or protected record |
| Compatibility | Is public evolution deliberate?                                                     | semver/non-exhaustive analysis | downstream exhaustive match breaks      | major    | stabilize or document migration   |
| Logging       | Is final handling logged once with correlation?                                     | trace of one failure           | same error logged four times            | minor    | assign log owner                  |
| Codes         | Are public codes stable and documented?                                             | code catalogue tests           | message text used as code               | major    | introduce semantic code           |
| Evidence      | Do tests cover action distinctions, not only display strings?                       | variant and fault tests        | snapshot-only evidence                  | major    | test semantics                    |

A critical failure blocks approval unless an explicit doctrine waiver permits it. Redacting a
public message does not justify erasing protected internal evidence.

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0002-R001`, `RUST-DOC-0002-R002`, `RUST-DOC-0002-R003`, `RUST-DOC-0002-R004`
- `RUST-DOC-0002-R005`, `RUST-DOC-0002-R006`, `RUST-DOC-0002-R007`, `RUST-DOC-0002-R008`
- `RUST-DOC-0002-R009`, `RUST-DOC-0002-R010`, `RUST-DOC-0002-R011`, `RUST-DOC-0002-R012`
- `RUST-DOC-0002-R013`, `RUST-DOC-0002-R014`

---

## Source: `doctrines/0002-error-modeling/anti-patterns.md`

# Anti-patterns

## `Result<T, String>`

**Weak example.** A library returns formatted prose for validation, I/O, and conflict.
**Why it fails.** Callers parse unstable wording. **Risk.** Wrong response or retry.
**Improved direction.** Structured domain categories with `Display` for humans.
**Justified appearance.** A tiny private helper may return text consumed immediately by one
formatting layer when no action distinction exists.

## One giant `AppError`

**Weak example.** Every subsystem variant is placed in one global enum and propagated
everywhere. **Why it fails.** It couples unrelated layers and makes any dependency a public
concern. **Risk.** Unstable API and meaningless catch-all handling. **Improved direction.**
Use bounded errors per contract and convert at ownership boundaries. **Justified appearance.**
A process entrypoint can have one final report type after domain decisions.

## Retry every error

**Weak example.** Middleware retries any `Err` three times. **Why it fails.** Validation,
denial, conflict, and unknown effects have different semantics. **Risk.** Duplicate effects
and load amplification. **Improved direction.** Typed retry/reconcile decision with budget and
jitter. **Justified appearance.** A proven idempotent read may use a narrow transport policy.

## Log and discard

**Weak example.** A lower layer logs an error and returns success or `None`. **Why it fails.**
The caller cannot respond and telemetry contradicts state. **Risk.** Silent data loss.
**Improved direction.** Return the error; log at final handling. **Justified appearance.**
Best-effort cleanup may record and continue when failure is explicitly non-critical and
observable.

## Double logging

**Weak example.** Every propagation layer logs the same source. **Why it fails.** One event
looks like many incidents. **Risk.** alert noise and false counts. **Improved direction.**
Attach context during propagation and log once at the handling owner. **Justified appearance.**
Distinct trace events can measure layer timing when correlated and not counted as failures.

## Panic on malformed input

**Weak example.** JSON parsing uses `unwrap`. **Why it fails.** External failure is expected.
**Risk.** denial of service and lost cleanup. **Improved direction.** Return validation or
parse error. **Justified appearance.** Fixed compile-time literals can use a locally proven
expectation.

## Timeout becomes rejection

**Weak example.** Provider timeout maps to `PaymentDeclined`. **Why it fails.** No rejection
evidence exists. **Risk.** duplicate capture and false user message. **Improved direction.**
Unknown outcome with reconciliation. **Justified appearance.** Only a protocol-proven
pre-commit timeout.

## Hide the source

**Weak example.** `map_err(|_| DomainError::Storage)` drops database evidence. **Why it fails.**
Diagnosis loses cause. **Risk.** slow repair and misclassification. **Improved direction.**
Preserve a source or correlated protected diagnostic. **Justified appearance.** Withhold a
sensitive source from an untrusted boundary while retaining it internally.

## Expose internal secrets

**Weak example.** Raw provider body or SQL appears in an HTTP error. **Why it fails.**
Diagnostic detail crosses recipient scope. **Risk.** credential or data disclosure.
**Improved direction.** Stable public code, safe message, protected correlated detail.
**Justified appearance.** Restricted forensic storage under access and retention control.

## String-based retry

**Weak example.** Code retries when `message.contains("temporary")`. **Why it fails.** Wording
and locale change. **Risk.** unsafe or missed retry. **Improved direction.** Structured
category or protocol code. **Justified appearance.** Compatibility parsing of a broken legacy
protocol may be isolated, versioned, and heavily tested.

---

## Source: `doctrines/0002-error-modeling/glossary.md`

# Glossary

**Application report** — An opaque, context-rich error intended for final application
handling rather than stable library matching.

**Error category** — A machine-actionable class with shared caller behavior.

**Fatal process error** — A returned failure after which the process deliberately exits
because safe service cannot begin or continue.

**Indeterminate outcome** — Absence of evidence to classify a possibly committed external
effect as success or non-execution.

**Recovery contract** — The state of inputs, handles, local mutation, and safe next actions
after an error.

**Retry guidance** — A structured decision derived from operation semantics, not merely a
transport label.

**Source chain** — Causal errors retained through wrapping for diagnosis.

**Recipient boundary** — The point at which an error is translated for a user, client,
operator, telemetry system, or protected audit store.

---

## Source: `doctrines/0002-error-modeling/references.md`

# References

- [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html) defines Rust's
  standard source-chain interface.
- [`std::result::Result`](https://doc.rust-lang.org/std/result/) documents explicit
  success/error control flow.
- [Rust API Guidelines: dependability](https://rust-lang.github.io/api-guidelines/dependability.html)
  covers useful error traits and predictable behavior.
- [The Rust Book: recoverable errors with
  `Result`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
  distinguishes recoverable errors and panics.
- [RFC 2008: non-exhaustive types](https://rust-lang.github.io/rfcs/2008-non-exhaustive.html)
  informs evolvable public error enums.
- [HTTP Semantics, status codes](https://www.rfc-editor.org/rfc/rfc9110#name-status-codes)
  defines protocol response classes without making them universal retry policy.
- [Tokio tutorial: spawning and `'static`](https://tokio.rs/tokio/tutorial/spawning) and
  runtime documentation provide primary runtime context for task error handling.
- [Common Weakness Enumeration CWE-209](https://cwe.mitre.org/data/definitions/209.html)
  documents information exposure through error messages.

---

## Source: `doctrines/0003-ownership-and-capabilities/README.md`

---
id: RUST-DOC-0003
slug: ownership-and-capabilities
title: Ownership as Authority and Lifecycle
status: active
version: 0.1.0
normative: true
applies_to:
  - planning
  - implementation
  - review
  - audit
risk_domains:
  - authority
  - resource-lifecycle
  - secrets
  - concurrency
supersedes: []
superseded_by: null
---

# Ownership as Authority and Lifecycle

## Scope

Ownership can express more than memory management. This doctrine governs its use for exclusive
authority, resource custody, lifecycle completion, single use, transfer, borrowing, secrets,
and inter-task handoff. It covers capabilities, transaction guards, tokens, leases, file
locks, shutdown permits, and locally owned resources.

## Out of scope

It does not claim Rust ownership is authorization by itself, make external rollback infallible,
or require a capability for every method. It does not replace synchronization, distributed
lease protocols, operating-system access control, or secret-management systems.

## Readers, status, and prerequisites

Planners map authority and lifecycle. Implementers design issuance and transfer. Reviewers
inspect clones, borrows, interior mutability, destruction, and revocation. Auditors search for
forged or leaked authority. `doctrine.md` is normative.

Read foundations on [invariants](../foundations/invariants.md),
[evidence](../foundations/evidence.md), [boundaries](../foundations/trust-boundaries.md),
[guarantee honesty](../foundations/guarantee-honesty.md), and
[complexity](../foundations/complexity-budget.md). Related material:
[capability types](../patterns/capability-types.md),
[consuming transitions](../patterns/consuming-transitions.md),
[typestate](../patterns/typestate.md), [filesystem](../boundaries/filesystem.md) and
[FFI](../boundaries/ffi.md) guides,
[concurrency doctrine](../doctrines/0004-concurrency-and-async/), and
[authenticated-session](../case-studies/authenticated-session/) and
[transaction](../case-studies/database-transaction/) case studies.

## Summary

- Use ownership to model exclusive custody when the domain is exclusive.
- Borrow only the authority needed and for no longer than required.
- Restrict capability issuance and operation surface.
- Specify clone, transfer, serialization, expiry, revocation, and destruction.
- Use RAII for local cleanup; model external rollback or compensation as fallible.
- Keep secrets hard to format, clone, serialize, and retain.
- Do not claim complete zeroization without accounting for copies, allocators, compiler
  behavior, and external storage.
- Do not default to `Arc<Mutex<T>>`; define ownership and synchronization first.
- Use lifetimes only for real borrowing relationships.

---

## Source: `doctrines/0003-ownership-and-capabilities/doctrine.md`

# Normative doctrine

## RUST-DOC-0003-R001 — Map authority and custody

**Statement.** A design MUST identify who owns each resource, who may borrow it, which
operations possession authorizes, how custody transfers, and how authority ends.

**Intent.** Prevent memory ownership from being confused with business permission or lifecycle
completion.

**Applicability.** Resources, tokens, sessions, transactions, locks, permits, secrets, and task
handoffs.

**Allowed exceptions.** Pure immutable data without authority or lifecycle meaning.

**Review evidence.** Authority map, lifecycle diagram, and ownership signatures.

## RUST-DOC-0003-R002 — Encode exclusive authority with ownership

**Statement.** Ownership SHOULD express exclusive authority when only one actor may legally
exercise or complete an operation.

**Intent.** Prevent duplicated commit, shutdown, claim, or single-use token consumption.

**Applicability.** Exclusive domain actions with natural transfer or consumption.

**Allowed exceptions.** Durable external coordination MAY require runtime exclusivity when
multiple processes or persisted actors participate.

**Review evidence.** Non-cloneable type, consuming operation, and concurrency or compile-fail
tests.

## RUST-DOC-0003-R003 — Bound borrowed authority

**Statement.** A borrowed reference MUST NOT accidentally grant mutation, ownership transfer,
serialization, or authority beyond the documented borrow scope.

**Intent.** Keep read access from becoming lasting or privileged access.

**Applicability.** References, guards, views, callbacks, and borrowed service handles.

**Allowed exceptions.** Interior mutability MAY permit mutation when that aliasing contract is
the explicit design and synchronization is correct.

**Review evidence.** Method receiver audit, returned-lifetime analysis, and mutation tests.

## RUST-DOC-0003-R004 — Restrict capability issuance and surface

**Statement.** Capability constructors MUST be restricted to authorized issuers, and a
capability MUST expose only the operations and scope it grants.

**Intent.** Make capabilities hard to forge and consistent with least privilege.

**Applicability.** Authorization, verification proof, shutdown, transaction, secret, and
resource capabilities.

**Allowed exceptions.** None for security-relevant authority.

**Review evidence.** Visibility, fields, re-exports, operation methods, and issuer tests.

## RUST-DOC-0003-R005 — Justify cloning authority

**Statement.** Cloning or copying an authority-bearing value MUST require explicit
justification consistent with exclusivity, use count, scope, and revocation.

**Intent.** Prevent convenience derives from amplifying authority.

**Applicability.** Capabilities, tokens, guards, handles, and credentials.

**Allowed exceptions.** A shareable read capability MAY be cloneable when duplication is part
of the documented authority model.

**Review evidence.** `Clone`/`Copy` audit, clone semantics, and duplicate-use tests.

## RUST-DOC-0003-R006 — Define transfer and revocation

**Statement.** Tokens, sessions, transaction guards, leases, and resource handles MUST define
transfer, expiry, revocation, and post-revocation behavior when those concepts apply.

**Intent.** Prevent local possession from being treated as perpetual external permission.

**Applicability.** Mutable authority, leased resources, sessions, and cross-task custody.

**Allowed exceptions.** Irrevocable process-local values MAY state that revocation is not part
of their contract.

**Review evidence.** State transitions, clocks or versions, revocation check, and stale-use
tests.

## RUST-DOC-0003-R007 — Treat RAII as local cleanup

**Statement.** RAII SHOULD release locally owned resources, but destruction MUST NOT be
described as proving fallible external rollback, commit, compensation, or durable cleanup.

**Intent.** Distinguish deterministic local drop from effects whose failure cannot be returned
by `Drop`.

**Applicability.** Transactions, locks, temporary files, sockets, remote leases, and sessions.

**Allowed exceptions.** Infallible local memory bookkeeping MAY be completed entirely in
`Drop`.

**Review evidence.** Explicit completion methods, drop fallback, error observability, and
failure tests.

## RUST-DOC-0003-R008 — Protect secret-bearing types

**Statement.** Secret-bearing types MUST minimize accidental `Debug`, `Display`, cloning,
serialization, logging, and long-lived borrowing; exposure MUST be explicit and scoped.

**Intent.** Reduce unintended copies and recipient leakage.

**Applicability.** Passwords, tokens, private keys, session secrets, and decrypted material.

**Allowed exceptions.** None for ordinary formatting. Controlled serialization MAY be
required for a protected secret store under a distinct API.

**Review evidence.** Trait implementation audit, redaction tests, exposure call sites, and
storage contract.

## RUST-DOC-0003-R009 — Limit zeroization claims

**Statement.** Zeroization claims MUST state the exact owned buffer cleared and MUST NOT imply
removal of compiler-created copies, allocator remnants, swap, logs, external stores, or prior
serialization unless those paths are controlled and evidenced.

**Intent.** Prevent a local overwrite mechanism from becoming a universal secrecy guarantee.

**Applicability.** Secret memory and cryptographic material.

**Allowed exceptions.** None to claim accuracy.

**Review evidence.** Ownership and copy analysis, drop path, memory-locking policy where used,
and explicit non-guarantees.

## RUST-DOC-0003-R010 — Design before `Arc<Mutex<T>>`

**Statement.** `Arc<Mutex<T>>` MUST NOT be the default substitute for identifying ownership,
task responsibility, mutation protocol, lock scope, and shutdown.

**Intent.** Avoid shared mutable bags that compile but hide contention, deadlock, and authority.

**Applicability.** Concurrent shared state and service handles.

**Allowed exceptions.** It MAY be the simplest correct mechanism after the ownership and
synchronization contract is documented.

**Review evidence.** Owner, lock invariant, contention and poisoning policy, alternatives, and
tests.

## RUST-DOC-0003-R011 — Justify interior mutability

**Statement.** Interior mutability MUST be justified by a required aliasing contract and MUST
preserve the domain's synchronization and authority invariants.

**Intent.** Prevent `Cell`, `RefCell`, locks, or atomics from bypassing a better ownership
design.

**Applicability.** Mutation through shared references.

**Allowed exceptions.** Local caching or instrumentation MAY use it when invisible to domain
semantics and reentrancy is safe.

**Review evidence.** Aliasing rationale, borrow/panic behavior, synchronization, and reentrancy
tests.

## RUST-DOC-0003-R012 — Use lifetimes for real relationships

**Statement.** Lifetime parameters SHOULD express actual borrowing or validity relationships,
not ornamental complexity or an inaccurate claim that an external resource remains valid.

**Intent.** Keep APIs readable and prevent local borrow duration from implying remote
liveness.

**Applicability.** Borrowed views, guards, transactions, callbacks, and FFI.

**Allowed exceptions.** Internal generic abstraction MAY carry a lifetime required by a
dependency, with its relationship documented.

**Review evidence.** Referent and duration explanation, escape analysis, and simpler owned
alternative.

## RUST-DOC-0003-R013 — Define cross-task ownership

**Statement.** Transfer of authority or resources across tasks MUST identify the new owner,
completion signal, cancellation behavior, shutdown responsibility, and behavior if the task
is dropped or panics.

**Intent.** Prevent detached custody and resources with no accountable closer.

**Applicability.** Spawned tasks, worker actors, channels carrying handles, and supervisors.

**Allowed exceptions.** Truly process-lifetime services MAY be owned by the process supervisor.

**Review evidence.** Task tree, join/abort contract, channel closure, and shutdown tests.

## RUST-DOC-0003-R014 — Keep external authority revalidation explicit

**Statement.** A local capability MUST NOT claim current external authority when revocation,
expiry, tenant membership, or resource ownership can change without local control; current
use MUST revalidate or carry a bounded lease.

**Intent.** Prevent stale authorization.

**Applicability.** Sessions, identity-provider grants, distributed locks, and policy decisions.

**Allowed exceptions.** Immutable operation-scoped grants MAY remain valid for their defined
commit window.

**Review evidence.** Lease or recheck boundary, stale-state handling, and revocation race
tests.

---

## Source: `doctrines/0003-ownership-and-capabilities/rationale.md`

# Rationale

Rust ownership is a strong vocabulary for custody: a value has an owner, can be moved, can be
borrowed, and is dropped. Domain design can align those facts with exclusive authority or a
single lifecycle. A transaction guard consumed by `commit(self)` cannot be committed again
through the same value. A shutdown permit moved to a supervisor identifies who may initiate
shutdown. A single-use token without `Clone` makes local duplication unavailable.

This alignment is useful only when the domain is actually exclusive and local. A process-local
`FileLock` handle records that an operating-system lock acquisition succeeded and that this
handle owns release. It does not prove another host follows the same locking convention or
that a network filesystem implements expected semantics. A distributed lease needs identity,
expiry, fencing, clock assumptions, and server enforcement.

Capabilities separate authority from ambient service access. Rather than pass a broad payment
service plus an ID, a validated authorization step can produce `CaptureCapability` scoped to
payment, amount, provider, and expiry. Only it exposes capture. Constructor privacy prevents
ordinary forgery. Yet cloning, serialization, and stale revocation can defeat the story, so
capability design includes the whole lifecycle.

Borrowing can express temporary access. An immutable borrowed view supports inspection without
ownership transfer. A mutable borrow grants exclusive mutation for its duration. Returning a
reference tied to a guard can prevent use after the guard ends. Lifetimes prove local reference
validity; they do not prove a remote lease, session, or socket remains accepted.

RAII is effective for local release because `Drop` runs when an owned value leaves scope during
ordinary unwinding. `Drop` cannot return an error. A database rollback, remote lease release,
or durable file sync can fail. Provide explicit `commit`, `rollback`, `close`, or `release`
methods whose failures are visible; use drop as a best-effort fallback and make fallback
failure observable where possible. Compensation is a new effect, not automatic rollback.

Secrets need restrictive traits. An ordinary derived `Debug` can print a token into logs.
`Clone` multiplies copies. Serde can place plaintext into an intermediate buffer. Borrowed
exposure can outlive the intended operation if returned or captured. A secret wrapper should
redact debug, omit display, avoid clone, and expose bytes only through a scoped closure or
deliberate method.

Zeroization narrows memory exposure but is often overclaimed. Clearing the owned buffer does
not clear copies made before wrapping, formatting buffers, allocator pages, swap, crash dumps,
remote logs, or serialized records. Compiler optimization and memory model details matter.
Accurate documentation states what buffer and lifecycle are controlled.

`Arc<Mutex<T>>` is sometimes correct. It provides shared ownership and mutually exclusive
runtime mutation. It does not identify which task owns progress, bound lock duration, prevent
deadlock, provide backpressure, define poisoning recovery, or stop a caller from performing an
external effect while holding the lock. Actor ownership, message passing, partitioned state,
or a single supervisor may fit better.

Interior mutability is similarly a contract, not an escape from the borrow checker. `RefCell`
moves borrow failure to runtime and can panic on reentrancy. A mutex introduces blocking,
poisoning, and contention. Atomics require ordering arguments. The mechanism should match the
aliasing relationship actually required.

Task transfer completes the ownership story. Moving a resource into a spawned task makes that
task responsible for it, but detached tasks can outlive request scope and lose error
reporting. Structured supervision, join handles, cancellation tokens, channel closure, and
graceful shutdown identify who reclaims resources and observes failure.

Examples compose these ideas:

- an authorization capability is issuer-created, operation-scoped, non-forgeable, and
  revalidated if revocable;
- a transaction guard is consumed by commit or rollback and reports ambiguous commit;
- a secret wrapper redacts formatting and controls exposure;
- a single-use token is non-cloneable and consumed;
- a leased resource carries expiry and fencing identity;
- a file lock states local operating-system semantics and releases on drop;
- a shutdown permit moves to one supervisor;
- a task-owned handle returns completion through a join path.

Ownership removes certain local invalid programs. It does not create external truth. The
guarantee ledger must keep those scopes separate.

## Guarantee ledger examples

| Claim                                                     | Established by                                          | Does not prove                                                         |
| --------------------------------------------------------- | ------------------------------------------------------- | ---------------------------------------------------------------------- |
| `CommitPermit` has not been consumed through this value   | private, non-cloneable ownership and consuming `commit` | database commit will succeed or acknowledgement will arrive            |
| `FileLock` owns one acquired operating-system lock handle | successful acquisition and owned handle                 | every other process or host honors the same locking protocol           |
| `SessionCapability` was issued for a principal and scope  | protected issuer and signed or server-side grant        | session remains unexpired, unrevoked, or sufficient for changed policy |
| `SecretBytes` redacts ordinary formatting                 | custom trait implementations and absent display         | copies never existed or memory is absent from swap and crash dumps     |
| `ShutdownPermit` has one process-local owner              | private non-cloneable value                             | all external workers will acknowledge shutdown                         |

These entries show why a capability's constructor, methods, and documentation must be reviewed
together. A name such as `ExclusiveLease` is dishonest if the server does not reject stale
holders with a fencing value. A name such as `RolledBackTransaction` is dishonest when drop
only sent a best-effort request.

## Transaction guards

A transaction guard often borrows a connection or owns a pooled connection. The lifetime can
ensure the guard does not outlive the local connection borrow. Consuming `commit` prevents
continued local mutation through the same guard. The database remains an independent system:
isolation level, server failover, connection loss, and ambiguous acknowledgement determine the
actual outcome.

An API can return `CommitOutcome::Confirmed`, a definite pre-commit failure with a reusable or
released guard, or an unknown outcome with transaction identity. It should not return the
original guard after possible commit as though the transaction were safely reusable. Drop may
attempt rollback only while local protocol evidence still supports it.

## Leases and file locks

Leases require more than an expiry timestamp stored in a Rust struct. State the clock source,
skew assumption, renewal protocol, server enforcement, fencing rule, and behavior after
renewal uncertainty. A task that pauses past expiry can still hold the local value; the
resource owner must reject stale fencing tokens.

File locks need a declared scope: process, host, mount, or network filesystem. Advisory locks
work only among cooperating actors. Paths can alias through links or mounts. Inheritance
across fork, duplication of descriptors, and close semantics can affect ownership. The wrapper
should claim only the documented operating-system behavior.

## Shutdown and cancellation

One shutdown permit can serialize initiation, while a broadcast cancellation token can notify
many tasks. These are different authorities. Notification does not prove completion. The
supervisor owns the task registry, waits within a deadline, records stragglers, and chooses
forced termination behavior. Dropping a permit should not silently initiate a destructive
shutdown unless that surprising contract is unavoidable and prominent.

Transferring a handle through a channel moves custody only when send succeeds. A failed send
returns the value, leaving the sender responsible. Once received, channel closure and task
panic determine recovery. Tests should cover send failure, receiver cancellation, and
supervisor shutdown — not only the successful handoff.

## Choosing less machinery

Not every right needs a new capability type. If a private function is called only after one
obvious authorization branch and no value crosses asynchronous or module boundaries, a
separate capability can add ceremony without reducing risk. Conversely, a capability is
valuable when authority would otherwise travel as a boolean, broad context object, or repeated
comment.

The complexity decision should compare an owned token, runtime policy check, narrow trait,
closure-based authority, and ordinary parameter passing. Select the mechanism that keeps
issuance, use, and end of authority visible with the least misleading surface.

---

## Source: `doctrines/0003-ownership-and-capabilities/decision-framework.md`

# Decision framework

## Determine the authority shape

Ask:

1. Is authority exclusive, shareable, or consumable?
2. Is it local or enforced externally?
3. Who issues it and can callers forge it?
4. Can it be cloned, delegated, serialized, or moved across tasks?
5. Does it expire or revoke?
6. What happens on cancellation, drop, panic, or process loss?
7. Does completion have a fallible external effect?

| Shape                                    | First mechanism                  |
| ---------------------------------------- | -------------------------------- |
| Exclusive local custody                  | owned non-cloneable value        |
| One-time operation                       | consuming token or capability    |
| Temporary read access                    | immutable borrow                 |
| Temporary exclusive mutation             | mutable borrow or scoped guard   |
| Shareable immutable authority            | cloneable scoped capability      |
| Mutable external permission              | runtime recheck or bounded lease |
| Single task owns mutable state           | actor/task ownership             |
| Shared state with short critical section | documented lock                  |

## Capability design

Record issuer, scope fields, operations, transfer, clone, serialization, expiry, revocation,
use count, and audit identity. Prefer separate capability types for materially different
authority instead of a boolean `is_admin` inside a broad context object. Avoid encoding claims
the enforcing service will not honor.

## RAII decision

Use drop for infallible local bookkeeping and best-effort cleanup. Add explicit completion when
failure matters. If forgetting explicit completion is dangerous, mark returned guards
`#[must_use]`, provide consuming methods, test drop fallback, and supervise leaked external
state.

## Shared state decision

Before `Arc<Mutex<T>>`, compare:

- move ownership into one worker;
- partition state by key;
- send commands through a bounded channel;
- use immutable snapshots;
- use a lock with explicit invariant and ordering;
- use atomics for one measured simple state.

Choose the simplest mechanism whose failure, shutdown, and contention behavior is clear.

## Secret decision

Decide who may expose the secret, in what representation, for how long, and to which API. Audit
every trait derive and serialization path. If zeroization is used, enumerate all known copies
and state the uncovered channels.

## Stop conditions

Do not add a capability when every use must perform the same mutable external authorization and
local possession adds no stable evidence. Do not use ownership to imply distributed
exclusivity. Do not add lifetimes that only make signatures harder. Do not share state merely
to avoid choosing an owner.

---

## Source: `doctrines/0003-ownership-and-capabilities/review-standard.md`

# Review standard

Record pass, fail, not applicable, or waiver.

| Gate                | Question                                              | Pass evidence               | Failure example                 | Severity | Remediation            |
| ------------------- | ----------------------------------------------------- | --------------------------- | ------------------------------- | -------- | ---------------------- |
| Authority map       | Is every resource and operation owner named?          | custody/lifecycle map       | ambient service access          | major    | assign ownership       |
| Exclusivity         | Does exclusive domain authority have one local owner? | non-cloneable value         | copied commit token             | critical | consume or coordinate  |
| Issuance            | Can unauthorized code construct capability?           | restricted constructor      | public capability field         | critical | restrict issuer        |
| Least privilege     | Does capability expose only scoped operations?        | narrow methods/scope        | admin service inside token      | critical | split capabilities     |
| Clone               | Is every authority clone deliberate?                  | clone contract/test         | derive for convenience          | critical | remove or redefine     |
| Transfer            | Is delegation explicit and auditable?                 | move/delegation record      | hidden clone across tasks       | major    | model transfer         |
| Revocation          | Can stale authority be used?                          | recheck or lease            | perpetual session capability    | critical | bound validity         |
| Borrow              | Does borrowed access grant only required rights?      | receiver/lifetime audit     | read view exposes mutation      | major    | narrow borrow          |
| RAII                | Is drop limited to local/best-effort cleanup?         | explicit fallible close     | drop claims rollback            | critical | expose completion      |
| Secret debug        | Are formatting paths redacted?                        | trait/redaction tests       | derived debug token             | critical | custom debug           |
| Secret copies       | Are clone and serialization minimized?                | call-site inventory         | secret freely cloneable         | critical | scope exposure         |
| Zeroization         | Is claim limited to controlled buffers?               | guarantee ledger            | "all traces removed"            | major    | narrow claim           |
| Shared state        | Was ownership designed before lock choice?            | alternatives/lock invariant | global `Arc<Mutex<_>>`          | major    | choose owner           |
| Lock scope          | Are external awaits/effects outside lock?             | code trace/test             | network call under lock         | critical | split critical section |
| Interior mutability | Is aliasing need explicit?                            | reentrancy/sync analysis    | `RefCell` to appease compiler   | major    | redesign ownership     |
| Lifetime            | Does each lifetime express a real referent relation?  | signature explanation       | ornamental generics             | minor    | simplify               |
| Task owner          | Who joins, cancels, and closes?                       | supervision tree            | detached resource task          | critical | structure tasks        |
| External truth      | Are local and external authority claims separated?    | non-guarantees              | local lease implies global lock | critical | revalidate/fence       |

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0003-R001`, `RUST-DOC-0003-R002`, `RUST-DOC-0003-R003`, `RUST-DOC-0003-R004`
- `RUST-DOC-0003-R005`, `RUST-DOC-0003-R006`, `RUST-DOC-0003-R007`, `RUST-DOC-0003-R008`
- `RUST-DOC-0003-R009`, `RUST-DOC-0003-R010`, `RUST-DOC-0003-R011`, `RUST-DOC-0003-R012`
- `RUST-DOC-0003-R013`, `RUST-DOC-0003-R014`

---

## Source: `doctrines/0003-ownership-and-capabilities/anti-patterns.md`

# Anti-patterns

## Cloneable single-use token

**Weak example.** A capture token derives `Clone`. **Why it fails.** Consumption of one copy
does not consume authority. **Risk.** duplicate effect. **Improved direction.** Remove clone
and add idempotency separately. **Justified appearance.** None for truly single-use authority.

## Public capability constructor

**Weak example.** Any caller can create `AdminCapability`. **Why it fails.** The type is
forgeable. **Risk.** authorization bypass. **Improved direction.** Restrict issuance to policy
owner and embed scope. **Justified appearance.** A nominal marker with no authority claim.

## RAII equals rollback

**Weak example.** Transaction drop is documented as guaranteeing rollback. **Why it fails.**
External rollback can fail and `Drop` cannot report it. **Risk.** false state and lost
diagnostics. **Improved direction.** explicit fallible rollback; drop as observed fallback.
**Justified appearance.** Infallible in-memory restoration.

## Secret derives everything

**Weak example.** Token derives `Debug`, `Clone`, `Serialize`. **Why it fails.** Ordinary
tooling leaks and copies it. **Risk.** credential exposure. **Improved direction.** redacted
debug, no display, scoped exposure, protected store adapter. **Justified appearance.**
Serialization only through a separate encrypted-store contract.

## Universal zeroization

**Weak example.** Clearing one `Vec<u8>` is claimed to remove the secret. **Why it fails.**
Copies and external traces remain. **Risk.** false security assurance. **Improved direction.**
state exact cleared buffer and uncovered paths. **Justified appearance.** Narrow buffer-level
claim.

## `Arc<Mutex<T>>` architecture

**Weak example.** Every service shares one mutable application state. **Why it fails.** No
progress owner, broad lock scope, hidden authority. **Risk.** deadlock and contention.
**Improved direction.** task ownership, bounded commands, partitioning, or narrow lock.
**Justified appearance.** Small shared cache with documented invariant and measured contention.

## `RefCell` to silence borrowing

**Weak example.** Interior mutability replaces a difficult ownership choice. **Why it fails.**
Borrow failure moves to runtime and reentrancy may panic. **Risk.** latent failure.
**Improved direction.** decide owner or document aliasing requirement. **Justified appearance.**
single-threaded cache with controlled reentrancy.

## Lifetime as liveness

**Weak example.** A borrow lifetime is described as proving a remote session stays active.
**Why it fails.** Lifetimes govern references, not external systems. **Risk.** skipped
revalidation. **Improved direction.** explicit lease/expiry and fallible use. **Justified
appearance.** Lifetime can tie a local guard to its resource.

## Detached custody

**Weak example.** A handle moves into a spawned task and its join handle is dropped. **Why it
fails.** No owner observes failure or shutdown. **Risk.** leaked resource and hidden effects.
**Improved direction.** supervisor owns join and cancellation. **Justified appearance.**
Process-lifetime telemetry task with explicit supervisor and loss policy.

## Broad borrowed service

**Weak example.** A function receives `&mut AppContext` to perform one authorized action.
**Why it fails.** It can mutate unrelated state and exercise ambient authority. **Risk.**
confused-deputy behavior. **Improved direction.** pass narrow capability and required data.
**Justified appearance.** tightly scoped internal orchestration where context is the explicit
transaction owner.

---

## Source: `doctrines/0003-ownership-and-capabilities/glossary.md`

# Glossary

**Authority** — Permission to cause a domain effect; distinct from memory access alone.

**Capability** — Protected value whose possession grants bounded operations.

**Custody** — Responsibility for a resource's use, transfer, completion, and release.

**Fencing token** — Monotonic or otherwise ordered lease evidence used by the resource owner
to reject stale actors.

**Interior mutability** — Mutation through a shared reference using runtime borrowing,
synchronization, or atomics.

**Lease** — Time- or version-bounded authority that may expire independently.

**RAII** — Resource acquisition bound to object lifetime, with local cleanup triggered by
destruction.

**Revocation** — Withdrawal of authority before or independent of local value destruction.

**Scoped exposure** — Deliberate temporary access to secret material or authority with bounded
recipient and lifetime.

**Single-use token** — Non-forgeable authority intended to be consumed by one operation.

---

## Source: `doctrines/0003-ownership-and-capabilities/references.md`

# References

- [Rust Book: ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
  defines moves, borrowing, and slices.
- [Rust Reference: destructors](https://doc.rust-lang.org/reference/destructors.html) defines
  destruction scope and drop behavior.
- [`std::ops::Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html) documents Rust's
  destructor hook and its non-fallible signature.
- [`std::sync::Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) and
  [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) document shared ownership,
  locking, and poisoning.
- [`std::cell`](https://doc.rust-lang.org/std/cell/) documents interior mutability and runtime
  borrow checking.
- [The Rustonomicon: Send and Sync](https://doc.rust-lang.org/nomicon/send-and-sync.html)
  explains concurrency marker obligations.
- [Dennis and Van Horn, "Programming Semantics for Multiprogrammed
  Computations"](https://dl.acm.org/doi/10.1145/360303.360308) is a foundational capability
  reference.
- [RFC 6819, OAuth threat model](https://www.rfc-editor.org/rfc/rfc6819) describes token
  leakage, replay, and lifecycle threats relevant to bearer capabilities.

---

## Source: `doctrines/0004-concurrency-and-async/README.md`

---
id: RUST-DOC-0004
slug: concurrency-and-async
title: Concurrency and Async Correctness
status: active
version: 0.1.1
normative: true
applies_to:
  - planning
  - implementation
  - review
  - audit
risk_domains:
  - concurrency
  - async
  - cancellation
  - resource-management
supersedes: []
superseded_by: null
---

# Concurrency and Async Correctness

## Scope

This package governs Rust code in which work overlaps in time, execution can be
interleaved, or progress depends on synchronization. It applies to operating
system threads, asynchronous tasks, channels, locks, atomics, actor-like
components, background workers, and shutdown coordination. It also applies when
otherwise sequential code calls a concurrent dependency whose cancellation,
ordering, or retry behavior affects correctness.

The doctrine treats concurrency as a protocol-design problem. Rust can prevent
many memory races, but `Send`, `Sync`, successful compilation, and use of an
async runtime do not establish freedom from deadlock, starvation, lost wakeups,
unbounded queues, retry amplification, incomplete cancellation, or incorrect
ordering assumptions.

## Out of scope

This package does not prescribe one async runtime, one channel implementation,
or one locking primitive. It does not define distributed idempotency in full;
that belongs to RUST-DOC-0006. It does not replace performance measurement under
RUST-DOC-0009, unsafe-code review under RUST-DOC-0007, or boundary validation
under RUST-DOC-0005. Runtime-specific guarantees remain those of the selected
runtime and version.

## Intended readers

- planners defining task ownership, capacity, cancellation, and shutdown;
- implementers writing concurrent or asynchronous components;
- reviewers tracing interleavings, lock scope, and overload behavior;
- auditors searching for detached work, unbounded growth, and hidden retries;
- maintainers changing runtimes, channel types, or supervision structure.

## Normative status

[`doctrine.md`](../doctrines/0004-concurrency-and-async/doctrine.md) is normative. Rule identifiers remain stable within
the doctrine version. Rationale, examples, and decision aids explain intended
application but become normative only when a rule incorporates them. An
approved, scoped waiver may document an exception; convenience is not evidence
for one.

## Prerequisite foundations

Read these documents before applying the rules:

1. [`../../foundations/invariants.md`](../foundations/invariants.md) for
   lifecycle, temporal, and authority invariants;
2. [`../../foundations/trust-boundaries.md`](../foundations/trust-boundaries.md)
   for external observations and effects;
3. [`../../foundations/guarantee-honesty.md`](../foundations/guarantee-honesty.md)
   for exact claims about local and external state;
4. [`../../foundations/complexity-budget.md`](../foundations/complexity-budget.md)
   for synchronization and abstraction costs;
5. [`../../foundations/evidence.md`](../foundations/evidence.md) for the
   evidence represented by handles, guards, and acknowledgements.

## Related material

- Patterns: [capability types](../patterns/capability-types.md),
  [consuming transitions](../patterns/consuming-transitions.md),
  [hybrid state machines](../patterns/hybrid-state-machines.md), and
  [explicit uncertainty](../patterns/explicit-uncertainty.md).
- Boundaries: messaging, [HTTP/RPC](../boundaries/http-and-rpc.md),
  [database decoding](../boundaries/database-decoding.md), filesystems, and FFI.
- Reviews: [pre-implementation](../reviews/pre-implementation.md),
  [distributed-effects](../reviews/distributed-effects-review.md), and
  [final correctness](../reviews/final-correctness-audit.md) audit.
- Case studies: [message delivery](../case-studies/message-delivery/), payment lifecycle,
  database transaction, and authenticated session.

## Reading order

Read the normative rules first, then the rationale. Use the decision framework
to select an ownership and coordination model. Apply the review standard before
merge. Use the anti-pattern catalogue during adversarial review, then consult
the glossary and primary references for disputed terminology.

## Compact doctrine summary

Every concurrent component needs a defined owner for mutable state, tasks, queues,
and shutdown. Capacity and backpressure are deliberate. Each suspension
point inside a partial operation requires cancellation analysis. Blocking work
must be isolated from executor workers. Lock ordering, channel closure, task
failure, retry layering, and external ordering claims require explicit
contracts. Detached work is exceptional and observable. Atomics require an
ordering argument tied to a synchronization invariant. Graceful shutdown means
bounded, observable completion behavior; it does not mean every external effect
can be rolled back.

## Executable evidence status

The example workspace demonstrates ownership-consuming transitions, a fallible
connection protocol, and compiler rejection of sending through a locally closed
connection. It does not include an async-runtime integration, cancellation
harness, deadlock detector, Loom model, or backpressure load test. Systems
applying this doctrine supply evidence for those runtime-specific claims; the
current examples do not establish them.

---

## Source: `doctrines/0004-concurrency-and-async/doctrine.md`

# Normative doctrine

## RUST-DOC-0004-R001 — Define the concurrency model

**Statement.** Every component with shared mutable state or overlapping work
MUST document its ownership and synchronization model.

**Intent.** Make custody, mutation authority, and coordination visible before
interleavings obscure them.

**Applicability.** Threads, async tasks, callbacks, actors, shared caches,
worker pools, and foreign callbacks.

**Allowed exceptions.** A leaf function using no shared state may rely on the
surrounding component contract.

**Review evidence.** An ownership map identifies state owner, permitted
mutators, synchronization primitive, task owner, and shutdown authority.

## RUST-DOC-0004-R002 — Protect invariants, not fields

**Statement.** Synchronization boundaries MUST cover the complete invariant
they protect. Related fields MUST NOT be independently locked when an operation
requires their values to change atomically as a group.

**Intent.** Prevent logically torn state even when every individual memory
access is race-free.

**Applicability.** Multi-field state, counters paired with collections, status
paired with resources, and cross-index data.

**Allowed exceptions.** Independent locks are permitted when the invariant
inventory proves the fields are independent or a documented reconciliation
protocol tolerates temporary divergence.

**Review evidence.** Invariant-to-lock mapping plus tests or model evidence for
multi-step updates.

## RUST-DOC-0004-R003 — Bound lock scope

**Statement.** Lock scope MUST be minimized to the protected operation and MUST
be documented on latency-sensitive or contention-sensitive paths. A blocking
call or `.await` MUST NOT occur while holding a synchronous lock unless a
specific correctness argument and bounded behavior justify it.

**Intent.** Reduce deadlock, convoying, executor blockage, and accidental
serialization.

**Applicability.** Mutexes, read-write locks, semaphore permits, and
transaction-like guards.

**Allowed exceptions.** A deliberately serialized critical section may include
a bounded non-suspending operation when splitting it would violate an
invariant.

**Review evidence.** Critical-section boundaries, timing assumptions, and
contention measurements where performance matters.

## RUST-DOC-0004-R004 — Review lock order and poisoning

**Statement.** Components that may acquire more than one lock MUST define a
global acquisition order or another deadlock-avoidance protocol. Lock poisoning
behavior MUST be chosen consciously rather than inherited without review.

**Intent.** Make cyclic waits and post-panic state handling explicit.

**Applicability.** Nested locks, callbacks under locks, standard-library
poisoning locks, version-appropriate `std::sync::nonpoison` APIs when available,
and libraries with different poisoning semantics.

**Allowed exceptions.** A proof that locks cannot overlap may replace a global
order.

**Review evidence.** Lock graph, callback analysis, and documented recovery,
fail-stop, or invariant-rebuild policy after panic.

## RUST-DOC-0004-R005 — Isolate blocking work

**Statement.** Potentially blocking filesystem, process, DNS, compression,
cryptographic, database, or CPU-intensive work MUST NOT run on async executor
worker threads without deliberate isolation.

**Intent.** Preserve progress for unrelated tasks and keep runtime scheduling
assumptions honest.

**Applicability.** Async services and libraries running work whose latency is
not cooperatively yielded.

**Allowed exceptions.** Operations proven bounded below the component's
documented scheduling budget may remain inline.

**Review evidence.** Classification of blocking calls, isolation mechanism,
pool capacity, cancellation behavior, and overload limits.

## RUST-DOC-0004-R006 — Analyze cancellation at every suspension point

**Statement.** Every `.await` or equivalent suspension point inside a partial
operation MUST be reviewed for cancellation safety.

**Intent.** Prevent abandoned state mutations, lost data, leaked authority, and
unobserved external effects when a future is dropped.

**Applicability.** Select loops, timeouts, request handlers, retries, and
multi-stage operations.

**Allowed exceptions.** A suspension point may be classified cancellation-safe
when dropping the future cannot lose progress or violate an invariant; the
classification still requires evidence.

**Review evidence.** A cancellation table showing state before suspension,
drop effect, cleanup owner, resumability, and external uncertainty.

## RUST-DOC-0004-R007 — Define cancellation cleanup

**Statement.** Resources and partial state created before a cancellable
suspension MUST have a defined cleanup, commit, compensation, or reconciliation
path.

**Intent.** Ensure that future destruction is part of the protocol instead of
an invisible control-flow edge.

**Applicability.** Locks, permits, temporary files, transactions, leases,
messages, and external requests.

**Allowed exceptions.** Abandonment is permitted only when the resource is
designed to expire or be collected and the delay and capacity consequences are
acceptable.

**Review evidence.** Drop behavior, explicit cleanup calls, expiry bounds,
reconciliation identifiers, and cancellation tests.

## RUST-DOC-0004-R008 — Bound concurrency

**Statement.** Concurrency SHOULD be bounded by a reviewed resource limit.
Unbounded task spawning requires explicit justification and an overload
analysis.

**Intent.** Prevent memory growth, connection exhaustion, scheduler collapse,
and downstream overload.

**Applicability.** Per-request tasks, batch fan-out, consumer loops, and retry
workers.

**Allowed exceptions.** A finite, statically bounded input set may establish a
safe upper bound without a runtime semaphore.

**Review evidence.** Capacity source, queue bound, rejection or waiting policy,
and stress evidence at and above capacity.

## RUST-DOC-0004-R009 — Make backpressure explicit

**Statement.** Producers and consumers MUST define what happens when demand
exceeds service capacity: wait, reject, shed, coalesce, persist, or degrade.

**Intent.** Replace accidental memory buffering with an operational contract.

**Applicability.** Channels, queues, streams, batching, connection pools, and
API ingress.

**Allowed exceptions.** None for an open-ended producer. A fixed small batch
may document its finite bound.

**Review evidence.** Capacity values, overflow behavior, fairness, metrics, and
caller-visible failure semantics.

## RUST-DOC-0004-R010 — Handle channel closure

**Statement.** Send and receive paths MUST handle channel closure as a normal
protocol event. Closure MUST NOT be converted silently into an endless retry,
busy loop, or fabricated success.

**Intent.** Make owner departure, shutdown, and worker failure observable.

**Applicability.** Bounded and unbounded channels, watch streams, broadcast
channels, and actor mailboxes.

**Allowed exceptions.** A process-terminating invariant breach may escalate
closure after recording adequate context.

**Review evidence.** Closure branches, sender/receiver ownership, drain policy,
and tests for last-sender and receiver-drop behavior.

## RUST-DOC-0004-R011 — Structure task ownership

**Statement.** Every spawned task MUST have an owner responsible for observing
completion, failure, cancellation, and shutdown.

**Intent.** Prevent invisible task failure and work that outlives its authority
or dependencies.

**Applicability.** Runtime tasks, threads, worker pools, and background
maintenance loops.

**Allowed exceptions.** Process-lifetime infrastructure may be supervised by a
top-level owner rather than joined by the immediate caller.

**Review evidence.** Task tree, join or supervision strategy, failure
propagation, restart limits, and shutdown trigger.

## RUST-DOC-0004-R012 — Restrict detached tasks

**Statement.** Detached tasks MUST be exceptional, named, observable, bounded,
and documented with their process-lifetime contract.

**Intent.** Avoid fire-and-forget work whose success, failure, or resource use
cannot be accounted for.

**Applicability.** Telemetry flushers, cache refreshers, cleanup work, and
best-effort notifications.

**Allowed exceptions.** A deliberately lossy best-effort action may detach if
loss is acceptable, resource use is bounded, and failures are measured.

**Review evidence.** Owner rationale, task name, metrics, capacity, panic
handling, and termination behavior.

## RUST-DOC-0004-R013 — Define graceful shutdown

**Statement.** Concurrent services MUST define admission stop, cancellation,
queue drain, resource release, deadline, forced termination, and observability
semantics for shutdown.

**Intent.** Turn shutdown into a tested lifecycle transition.

**Applicability.** Services, consumers, worker pools, and long-running tools.

**Allowed exceptions.** Short-lived pure computations may rely on process
completion when they own no persistent or external effect.

**Review evidence.** Ordered shutdown procedure, time budget, outstanding-work
accounting, and tests for idle and loaded shutdown.

## RUST-DOC-0004-R014 — State ordering guarantees precisely

**Statement.** Ordering claims MUST identify their scope, key, producer set,
buffering boundary, and behavior during retry or failover.

**Intent.** Prevent local FIFO behavior from being described as global order.

**Applicability.** Channels, brokers, actor mailboxes, logs, and concurrent
state updates.

**Allowed exceptions.** None when callers depend on order.

**Review evidence.** Ordering contract plus tests that include multiple
producers, retries, and closure where relevant.

## RUST-DOC-0004-R015 — Justify atomic ordering

**Statement.** Every nontrivial atomic operation MUST document why its memory
ordering is sufficient for the associated synchronization invariant.

**Intent.** Prevent atomics from becoming unexplained race-free but incorrect
protocols.

**Applicability.** `Atomic*`, fences, lock-free structures, and unsafe
concurrency.

**Allowed exceptions.** A simple standalone statistics counter may use relaxed
ordering when no other memory is synchronized through it.

**Review evidence.** Happens-before argument, invariant, permitted
interleavings, Loom or equivalent model evidence where tractable, and
RUST-DOC-0007 review if unsafe code is present.

## RUST-DOC-0004-R016 — Preserve failure and ordering through supervision

**Statement.** Task supervision MUST distinguish normal completion,
cancellation, panic, retryable failure, permanent failure, and exhausted
restart policy when those outcomes require different action.

**Intent.** Prevent restart loops and silent partial service.

**Applicability.** Actors, consumers, background workers, and service task
trees.

**Allowed exceptions.** Outcomes may be combined only when no caller or
operator acts differently and diagnostic evidence remains adequate.

**Review evidence.** Supervision decision table, restart budget, backoff,
jitter, terminal-state reporting, and panic policy.

## RUST-DOC-0004-R017 — Review async abstraction costs

**Statement.** Async traits, boxed futures, dynamic dispatch, and generated
state machines MUST be evaluated for allocation, object-safety, API stability,
diagnostic, and monomorphization tradeoffs.

**Intent.** Keep async abstraction proportional to actual polymorphism needs.

**Applicability.** Public traits, plugin boundaries, high-volume paths, and
generic middleware.

**Allowed exceptions.** Local low-volume code may choose the clearest interface
without benchmark evidence when its cost is immaterial.

**Review evidence.** Required dispatch mode, allocation expectations, public
API consequences, and measurements for performance claims.

## RUST-DOC-0004-R018 — Coordinate timeouts and retries

**Statement.** Timeout and retry layers MUST be inventoried end to end.
Independent layers MUST NOT multiply attempts or synchronize retries without a
documented load and idempotency analysis.

**Intent.** Prevent retry storms, thundering herds, duplicated effects, and
latency that exceeds caller budgets.

**Applicability.** Clients, middleware, proxies, services, databases, brokers,
and supervisors.

**Allowed exceptions.** Nested retries may exist when attempt budgets compose
within one deadline and each layer has distinct, proven safe semantics.

**Review evidence.** Attempt equation, total deadline, backoff and jitter,
idempotency classification, downstream capacity, and unknown-outcome handling.

## RUST-DOC-0004-R019 — Separate concurrency safety from external correctness

**Statement.** A race-free local transition MUST NOT be claimed to establish
remote liveness, durable completion, unique execution, or current external
state.

**Intent.** Keep local synchronization evidence distinct from mutable external
reality.

**Applicability.** Network connections, acknowledgements, leases, distributed
locks, and database commits.

**Allowed exceptions.** None.

**Review evidence.** Guarantee ledger identifying local proof, observation
time, runtime failures, timeout ambiguity, and reconciliation path.

## RUST-DOC-0004-R020 — Test adverse schedules and overload

**Statement.** Evidence for a consequential concurrent protocol MUST include
adverse scheduling, closure, cancellation, overload, and shutdown behavior,
using model checking or fault control when ordinary tests cannot reliably
exercise the interleavings.

**Intent.** Test the protocol edges that happy-path scheduling conceals.

**Applicability.** Shared-state protocols, supervisors, queues, and
cancellation-sensitive operations.

**Allowed exceptions.** Trivial immutable parallel computation may document why
these hazards are absent.

**Review evidence.** Invariant-linked tests, stress or model results, failure
injection, and known evidence limits.

---

## Source: `doctrines/0004-concurrency-and-async/rationale.md`

# Rationale

## Memory-race freedom is one layer

Rust's ownership rules and `Send`/`Sync` traits reject important classes of
unsafe sharing. They do not decide whether the program chose the right
synchronization boundary. Two fields can each be protected correctly while
their relationship is observed in an impossible combination. A channel can be
memory-safe while its unbounded queue consumes all memory. A mutex can be
correctly implemented while two call paths acquire locks in opposite order.
An async task can compile while never yielding during a long computation.

The doctrine therefore begins with a protocol description: state ownership,
mutation authority, synchronization, task custody, queue capacity, and
shutdown. Primitives are selected after the invariant is known.

## Ownership model choices

Single-owner designs remove many interleavings. An actor or dedicated worker
can own mutable state and accept commands through a bounded channel. This makes
mutation order local, but introduces mailbox capacity, response cancellation,
owner failure, and shutdown questions. Shared-lock designs can reduce message
overhead and allow direct reads, but require a lock graph and careful critical
sections. Immutable snapshots can simplify reads while making update and memory
retention costs visible. Atomics can serve narrow protocols, but their compact
syntax hides a demanding memory-order proof.

No model is uniformly superior. The correct question is which model makes the
important invariant and overload behavior easiest to establish and audit.

## Cancellation is control flow

An async future may be dropped whenever its owner abandons it, a timeout wins,
or a selection chooses another branch. Drop is therefore an ordinary
control-flow edge. If an operation removed an item from a queue before waiting
to write it, cancellation may lose the item. If it sent an external request
before awaiting the response, cancellation cannot establish that the request
did not execute. If it acquired a permit through an RAII guard, drop may
correctly release local capacity, but it cannot undo an external effect.

Cancellation analysis records:

| Question                               | Required answer                                       |
| -------------------------------------- | ----------------------------------------------------- |
| What changed before suspension?        | local and external mutations                          |
| What happens if the future is dropped? | destructor, abandonment, or no action                 |
| Who owns recovery?                     | current task, supervisor, lease expiry, or reconciler |
| Can the operation resume safely?       | cursor, transaction, or idempotency evidence          |
| Can success be unknown?                | explicit reconciliation state and identity            |

Cancellation-safe does not mean infallible. It means that dropping the future
at the specified point does not violate its documented protocol.

## Backpressure is part of the API

When producers can outpace consumers, some resource accumulates: memory,
threads, file descriptors, database rows, broker depth, or caller latency.
Calling a channel "internal" does not remove this fact. A bounded channel makes
capacity observable, but the choice at capacity still matters. Waiting
propagates pressure upstream. Rejection preserves the service but requires a
caller policy. Shedding or coalescing is valid only when the lost distinctions
are unimportant. Persisting creates a durable queue with its own retention and
replay contract.

Capacity should derive from resource budgets and service objectives rather than
an arbitrary large number. Stress evidence must include behavior after the
limit, not only throughput before it.

## Structured task ownership

A spawned task creates a lifecycle. Someone must observe its completion and
failure, decide whether to cancel siblings, and stop it during shutdown. Merely
retaining a runtime handle is insufficient if the handle is never awaited or
supervised. Structured ownership forms a task tree whose children do not
silently outlive the authority, configuration, or resources of their parent.

Some process-lifetime tasks cannot be lexically scoped to a request. They still
need a top-level supervisor, name, health signal, restart budget, and shutdown
path. Detachment describes an implementation relationship; it must not erase
operational accountability.

## Locks and suspension

Holding a synchronous lock across `.await` can block an executor worker and
allow the suspended task to retain exclusive access for an unbounded duration.
Even an async-aware mutex can create long convoys or deadlocks if callbacks and
other resources form a cycle. The appropriate remedy is usually to extract the
needed state, release the guard, perform the fallible or slow work, then
reacquire and validate that assumptions still hold. Sometimes the invariant
requires serialization across the slow operation; then a dedicated owner task
or explicit operation queue often expresses the design better.

Poisoning is also policy, not proof. A panic while holding a standard-library
mutex marks possible invariant damage. Blind recovery may expose corrupt state;
blind termination may be excessive when state can be rebuilt. The component
must choose.

Choosing a non-poisoning lock removes the poison signal, not the possibility
that a panic interrupted a multi-step invariant update. The same review must
therefore define whether unwinding can expose partial state and how the
component repairs or abandons it. In the pinned Rust 1.97.1 documentation,
`std::sync::nonpoison` is present but nightly-only and experimental; consumers
must check the documentation for their actual toolchain rather than infer
stability from the namespace.

## Blocking isolation is bounded too

Moving blocking work to a blocking pool protects async workers only if that pool
has capacity, admission control, and cancellation semantics. CPU-heavy work can
still saturate all cores. Blocking calls may not stop when their async wrapper
is cancelled. A detached blocking job can continue consuming resources after
the request disappears. Isolation moves contention to an explicit subsystem; it
does not delete it.

## Retry amplification

Suppose a client tries three times, a proxy tries twice, and the service worker
tries four times. One logical request can create twenty-four downstream
attempts, before broker or database retries are counted. If every layer starts
retrying after a common timeout, the load spike becomes synchronized. Backoff,
jitter, deadlines, retry budgets, and idempotency must be designed as one
system.

Retry safety depends on operation semantics. A transport error before a response
does not reveal whether the remote effect occurred. RUST-DOC-0006 governs the
resulting unknown outcome and reconciliation. Concurrency limits without retry
coordination can still admit a sustained overload loop.

## Ordering claims

A single channel receiver may observe messages from each individual sender in
order while interleaving multiple senders nondeterministically. A broker may
order within a partition but not across partitions. Restart, redelivery, retry,
and parallel consumption can change visible order. An ordering contract must
therefore name the key, producer set, scope, and exceptional behavior.

Global order is expensive and often unnecessary. Per-aggregate sequencing or
commutative operations may provide the actual business guarantee with less
coordination.

## Atomics and proof cost

Atomicity prevents torn access to the atomic value. Memory ordering determines
how that access relates to other memory. A relaxed counter is suitable for
independent telemetry because it does not publish other data. A readiness flag
that publishes initialized memory needs a happens-before relationship.
Copying an ordering from nearby code is not an argument.

Small lock-free protocols benefit from model checking because ordinary tests
sample schedules. Unsafe lock-free code also inherits the proof obligations of
RUST-DOC-0007. A mutex is often the lower-risk choice when measured contention
does not justify the atomic protocol.

## Complexity and operational truth

Async traits, boxed futures, generic middleware, and elaborate actor systems can
improve composition, but they add allocation, diagnostics, dynamic dispatch,
monomorphization, and lifecycle complexity. The type system cannot encode every
schedule, queue depth, deadline, or external state. Runtime metrics and
supervision remain necessary.

The simplest correct design may be sequential code with a bounded worker pool.
Concurrency is justified by workload and latency evidence, not by language
capability. Its complexity budget includes failure analysis, tests, operator
visibility, and future maintenance — not only source-line count.

---

## Source: `doctrines/0004-concurrency-and-async/decision-framework.md`

# Decision framework

## Start with artifacts

Before selecting a primitive, produce:

1. an invariant inventory for shared and task-local state;
2. an ownership map naming every mutator;
3. a task tree with completion and shutdown owners;
4. an external-effect list with cancellation points;
5. a capacity table for queues, pools, permits, and downstream services;
6. a lock graph or a statement that no locks overlap;
7. a retry-and-timeout inventory;
8. an evidence plan covering adverse schedules and overload.

If these artifacts cannot be stated, implementation is premature.

## Choose the state model

| Need                                | Initial choice            | Main checks                                         |
| ----------------------------------- | ------------------------- | --------------------------------------------------- |
| One sequential owner, many commands | actor or owner task       | mailbox bound, response cancellation, supervision   |
| Short shared reads and updates      | lock-protected state      | complete invariant, lock order, contention          |
| Read-mostly immutable views         | snapshot or copy-on-write | update cost, stale reads, retention                 |
| Fixed parallel pure work            | bounded worker pool       | input bound, result order, panic propagation        |
| Narrow flag or counter protocol     | atomic                    | happens-before argument, ordering, model evidence   |
| External durable coordination       | runtime protocol          | lease, fencing, expiry, split brain, reconciliation |

Prefer one owner when shared mutation is complex. Prefer a lock when operations
are short and the protected invariant is clear. Prefer ordinary sequential code
when measured workload does not need overlap.

## Suspension-point decision

For each `.await`:

```text
Has local or external state changed before suspension?
├─ no → verify drop releases acquired resources
└─ yes
   Can drop leave the invariant valid and progress recoverable?
   ├─ yes → document recovery owner and resume identity
   └─ no
      Can the mutation move after the suspension?
      ├─ yes → reorder operation
      └─ no
         Can a guard/owner task finish it despite caller cancellation?
         ├─ yes → supervise bounded completion
         └─ no → add compensation or explicit reconciliation
```

A timeout around the future adds another cancellation edge. It does not prove a
remote request was not executed.

## Capacity decision

For each producer-consumer boundary, calculate:

- maximum admitted concurrent work;
- average and peak arrival rate assumptions;
- service-time distribution;
- per-item memory and resource cost;
- downstream capacity;
- wait deadline;
- overflow policy;
- fairness requirements;
- queue-age and rejection metrics.

Choose among:

| Policy   | Use when                                                | Cost                                   |
| -------- | ------------------------------------------------------- | -------------------------------------- |
| Wait     | caller can absorb latency and pressure should propagate | deadline and head-of-line blocking     |
| Reject   | caller can retry or degrade safely                      | visible failure and retry coordination |
| Shed     | work is explicitly disposable                           | information loss                       |
| Coalesce | newest or aggregate value is sufficient                 | intermediate history lost              |
| Persist  | work must survive process loss                          | storage, replay, deduplication         |

An open-ended source with an unbounded in-memory queue fails the decision gate.

## Lock decision

Use a lock only after answering:

- Which invariant does it protect?
- Can code call unknown callbacks while holding it?
- Can it acquire another lock?
- Can it suspend or block?
- What is the acquisition order?
- What happens after panic?
- Can ownership transfer remove the shared mutation?
- Does measured contention require a different design?

Stop and redesign if the lock guards a broad component merely because ownership
was not decided.

## Task and shutdown decision

For every spawn, record:

| Field         | Required content                                   |
| ------------- | -------------------------------------------------- |
| Task name     | stable operational identity                        |
| Owner         | task or component that observes it                 |
| Completion    | join, result channel, or supervised state          |
| Failure       | propagation, restart, degradation, or process stop |
| Cancellation  | trigger and cleanup behavior                       |
| Capacity      | maximum instances                                  |
| Shutdown      | admission stop, drain, deadline, abort             |
| Observability | active count, failures, age, queue depth           |

Detach only when loss is acceptable and the work remains bounded and observable.

## Retry composition

Compute the maximum attempt multiplication across all layers. Establish a
single end-to-end deadline and allocate sub-budgets. For each failure category,
choose:

- no retry;
- retry within remaining budget;
- reconcile before retry;
- terminal rejection;
- supervisor restart.

Use exponential backoff with jitter only where repeated attempts are safe.
Backoff is not a substitute for a hard attempt budget. If an operation may
already have taken effect, route to RUST-DOC-0006.

## Stop conditions

Choose a simpler or sequential design when:

- ownership is harder to explain than the workload benefit;
- the task tree has no clear failure owner;
- queue capacity has no defensible basis;
- lock ordering cannot be made acyclic;
- cancellation cannot preserve or reconcile partial work;
- an atomic ordering argument cannot be stated;
- benchmark evidence does not support the added parallelism;
- diagnostics and type complexity exceed the risk removed.

## Evidence selection

Use deterministic unit tests for closure and state transitions, controlled
timeouts for cancellation cleanup, stress tests for overload, and model checking
for small synchronization protocols. Measure queue depth, task count, lock wait,
latency distribution, rejection, restart, and shutdown duration in realistic
integration tests. State which schedules and failures remain untested.

---

## Source: `doctrines/0004-concurrency-and-async/review-standard.md`

# Review standard

Record each gate as **pass**, **fail**, **not applicable**, or an approved
**waiver reference**. A critical failure blocks merge.

| Gate | Question                                                | Pass evidence                        | Failure example                                  | Severity | Remediation                            |
| ---- | ------------------------------------------------------- | ------------------------------------ | ------------------------------------------------ | -------- | -------------------------------------- |
| C01  | Is mutable-state ownership explicit?                    | ownership map                        | several tasks mutate shared state by convention  | critical | assign owner and synchronization       |
| C02  | Does each synchronization primitive name its invariant? | invariant-to-primitive mapping       | mutex exists only because sharing was convenient | high     | define protected relationship          |
| C03  | Are related fields updated under one protocol?          | atomic grouped transition            | status and handle use separate locks             | critical | combine state or coordinate transition |
| C04  | Is aliasing authority minimal?                          | narrow borrowed or message interface | broad mutable handle escapes                     | high     | restrict interface                     |
| C05  | Is every task represented in a task tree?               | spawn inventory                      | untracked spawn                                  | high     | add owner and completion path          |
| C06  | Are task failures observed?                             | join or supervisor result handling   | join handle dropped                              | critical | propagate or supervise failure         |
| C07  | Are panics handled according to policy?                 | panic branch and telemetry           | worker silently disappears                       | high     | define fail, restart, or degrade       |
| C08  | Are restart attempts bounded?                           | restart budget                       | permanent failure loops forever                  | high     | cap and expose terminal state          |
| C09  | Does restart use backoff and jitter where needed?       | policy and tests                     | all workers restart simultaneously               | high     | stagger bounded retries                |
| C10  | Is concurrency bounded?                                 | semaphore, pool, or finite proof     | spawn per unbounded input                        | critical | impose reviewed limit                  |
| C11  | Is queue capacity justified?                            | resource calculation                 | arbitrary enormous buffer                        | high     | derive from budget                     |
| C12  | Is overload behavior explicit?                          | wait/reject/shed/persist contract    | memory grows until failure                       | critical | add backpressure                       |
| C13  | Is overload visible?                                    | queue age/depth/rejection metrics    | saturation is silent                             | medium   | instrument capacity signals            |
| C14  | Is fairness considered?                                 | scheduling policy                    | one tenant monopolizes permits                   | medium   | partition or schedule fairly           |
| C15  | Is each channel closure branch handled?                 | closure tests                        | receive loop spins after closure                 | high     | terminate or transition                |
| C16  | Does receiver loss reach senders?                       | send error policy                    | sends reported successful after owner exit       | critical | preserve closure failure               |
| C17  | Is drain behavior defined?                              | shutdown contract                    | queue is silently discarded                      | high     | drain, persist, or document loss       |
| C18  | Is lock scope bounded?                                  | visible narrow guard lifetime        | guard spans whole request                        | high     | extract and release                    |
| C19  | Is `.await` absent under synchronous locks?             | code inspection                      | network call while mutex held                    | critical | split phase or use owner task          |
| C20  | Are blocking calls classified?                          | blocking-work inventory              | synchronous filesystem call on executor          | high     | isolate deliberately                   |
| C21  | Is blocking-pool capacity bounded?                      | pool configuration                   | isolation pool grows without limit               | high     | add capacity and admission             |
| C22  | Can cancelled blocking work continue?                   | explicit accounting                  | request cancellation assumed to stop syscall     | high     | supervise remaining work               |
| C23  | Is a lock acquisition order documented?                 | acyclic lock graph                   | paths acquire A/B and B/A                        | critical | establish order or redesign            |
| C24  | Are callbacks excluded from lock scope?                 | call graph evidence                  | arbitrary callback under lock                    | high     | release before callback                |
| C25  | Is poisoning policy explicit?                           | recovery or fail-stop rationale      | poisoned lock blindly unwrapped                  | high     | validate or terminate                  |
| C26  | Is every suspension point cancellation-reviewed?        | cancellation table                   | timeout drops half-finished operation            | critical | reorder, guard, or reconcile           |
| C27  | Does cancellation release local resources?              | RAII or explicit cleanup test        | permit leak                                      | high     | add owned guard                        |
| C28  | Are external effects before cancellation recorded?      | outcome state                        | timeout becomes definitive rejection             | critical | represent uncertainty                  |
| C29  | Can partial progress resume safely?                     | cursor/idempotency evidence          | restart repeats unsafe effect                    | critical | persist progress or reconcile          |
| C30  | Are timeout budgets end-to-end?                         | deadline allocation                  | nested layers each use full timeout              | high     | propagate remaining budget             |
| C31  | Is retry multiplication calculated?                     | attempt equation                     | client, proxy, worker each retry blindly         | critical | centralize budgets                     |
| C32  | Are retries idempotency-classified?                     | operation table                      | mutation retried after lost response             | critical | reconcile or use idempotency           |
| C33  | Are retry waves desynchronized?                         | jitter policy                        | thundering herd at fixed intervals               | high     | add jitter and admission               |
| C34  | Is ordering scope precise?                              | key/producer/partition contract      | FIFO called global order                         | high     | narrow claim                           |
| C35  | Are retry and failover effects on order stated?         | scenario tests                       | redelivery reorders unnoticed                    | high     | version or tolerate reorder            |
| C36  | Does shutdown stop admission first?                     | ordered procedure                    | new work enters while draining                   | high     | close ingress                          |
| C37  | Does shutdown have a deadline?                          | time budget                          | waits forever                                    | high     | define forced termination              |
| C38  | Are in-flight effects classified at shutdown?           | work accounting                      | process exit loses ambiguous request             | critical | persist or reconcile                   |
| C39  | Are resources released on all exits?                    | tests for normal/error/cancel        | permits survive task failure                     | high     | use guards and cleanup                 |
| C40  | Are detached tasks exceptional and named?               | approved inventory                   | anonymous fire-and-forget                        | high     | supervise or document exception        |
| C41  | Are detached resources bounded?                         | count and queue limits               | cleanup task accumulation                        | critical | bound and shed                         |
| C42  | Are detached failures observable?                       | metrics and logs                     | background failure invisible                     | high     | report health                          |
| C43  | Does each atomic name its invariant?                    | code comment and design note         | atomic chosen for speed                          | critical | define protocol                        |
| C44  | Is memory ordering justified?                           | happens-before argument              | copied `Relaxed` ordering                        | critical | prove or use lock                      |
| C45  | Is unsafe concurrency separately reviewed?              | RUST-DOC-0007 evidence               | manual `Send` with no proof                      | critical | perform unsafe audit                   |
| C46  | Are small protocols model-tested where valuable?        | Loom or equivalent result            | rare schedule only stress-tested                 | medium   | add controlled schedule tests          |
| C47  | Are async trait costs understood?                       | dispatch/allocation analysis         | boxed future on hot path by accident             | medium   | simplify or measure                    |
| C48  | Are latency claims distributions, not anecdotes?        | p50/p95/p99 under load               | one local timing                                 | medium   | benchmark correctly                    |
| C49  | Does local state avoid remote-liveness claims?          | guarantee ledger                     | connected state promises next send               | critical | narrow guarantee                       |
| C50  | Are evidence limits documented?                         | residual-risk section                | tests presented as universal proof               | high     | state untested schedules               |

## Review outcome

Approval requires all critical gates to pass or carry a time-bounded waiver with
owner, scope, compensating evidence, and removal condition. High-severity
failures require remediation or explicit risk acceptance. Medium findings may
be scheduled only when they cannot mask correctness or overload failures.

The review record attaches or references the ownership map, task tree, capacity
table, cancellation table, retry inventory, shutdown procedure, and guarantee
ledger. Code style alone is not sufficient evidence.

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0004-R001`, `RUST-DOC-0004-R002`, `RUST-DOC-0004-R003`, `RUST-DOC-0004-R004`
- `RUST-DOC-0004-R005`, `RUST-DOC-0004-R006`, `RUST-DOC-0004-R007`, `RUST-DOC-0004-R008`
- `RUST-DOC-0004-R009`, `RUST-DOC-0004-R010`, `RUST-DOC-0004-R011`, `RUST-DOC-0004-R012`
- `RUST-DOC-0004-R013`, `RUST-DOC-0004-R014`, `RUST-DOC-0004-R015`, `RUST-DOC-0004-R016`
- `RUST-DOC-0004-R017`, `RUST-DOC-0004-R018`, `RUST-DOC-0004-R019`, `RUST-DOC-0004-R020`

---

## Source: `doctrines/0004-concurrency-and-async/anti-patterns.md`

# Anti-pattern catalogue

## `Arc<Mutex<T>>` as architecture

**Weak example.** Every service clones one `Arc<Mutex<State>>`, and each method
locks whichever fields it needs.

**Why it fails.** Shared access exists without a defined state owner, invariant
boundary, lock order, or contention model.

**Risk.** Deadlock, broad authority, torn logical transitions, and accidental
serialization.

**Improved direction.** Define ownership first. Use a dedicated owner task,
narrow capability, immutable snapshot, or a lock protecting a named invariant.

**When justified.** A small, short-lived component with one clear invariant and
measured low contention may use this representation.

## Locking across `.await`

**Weak example.** A task holds a synchronous mutex guard while awaiting a
network response.

**Why it fails.** The guard duration becomes externally controlled and may
block executor progress or create a resource cycle.

**Risk.** Deadlock, latency spikes, and throughput collapse.

**Improved direction.** Copy or move required state out, release the guard,
perform the request, then reacquire and validate. For mandatory serialization,
route work through an owner task.

**When justified.** An async-aware lock may span a deliberately serialized
operation only with a bounded external contract and a reviewed cancellation
path.

## Spawn per item

**Weak example.** A loop spawns one task for every element from an open-ended
stream.

**Why it fails.** Task count becomes the queue and has no admission policy.

**Risk.** Memory exhaustion, connection exhaustion, scheduler overhead, and
downstream overload.

**Improved direction.** Use bounded concurrency, a worker pool, or a stream
buffer with a reviewed limit and overload behavior.

**When justified.** The complete input is finite and statically proven below a
safe resource limit.

## Unbounded channel for convenience

**Weak example.** A producer uses an unbounded channel to avoid handling
capacity errors.

**Why it fails.** It turns overload into hidden memory consumption and latency.

**Risk.** Process termination and stale work processed after it has value.

**Improved direction.** Use a bounded channel and choose wait, reject, shed,
coalesce, or durable persistence.

**When justified.** A truly finite, tightly bounded event set may use one when
the bound is documented independently.

## Fire-and-forget side effect

**Weak example.** A request handler spawns an email or payment task, drops the
handle, and returns success.

**Why it fails.** Completion and failure have no owner; caller success describes
admission rather than effect.

**Risk.** Silent loss, duplicate retry, and dishonest API semantics.

**Improved direction.** Return an accepted operation identity, supervise the
task, persist durable intent, and expose confirmed or unknown outcomes.

**When justified.** A bounded best-effort telemetry action may detach when loss
is part of its documented contract.

## Timeout means cancellation succeeded

**Weak example.** Timing out a future is recorded as though no external effect
occurred.

**Why it fails.** Dropping local waiting does not revoke a transmitted request.

**Risk.** Duplicate payment, duplicate message, or false rejection.

**Improved direction.** Classify the operation as unknown when execution may
have occurred and preserve reconciliation identity.

**When justified.** Definitive failure is valid only when the protocol supplies
evidence that execution could not have occurred.

## Retry at every layer

**Weak example.** Client, proxy, service, database adapter, and supervisor each
retry independently.

**Why it fails.** Attempts multiply and common timing synchronizes load.

**Risk.** Retry storm, thundering herd, budget overrun, and duplicate effects.

**Improved direction.** Inventory all layers, allocate one deadline and attempt
budget, add jitter, and classify idempotency.

**When justified.** Nested retry scopes may address distinct failures when
their composed maximum is proven and observable.

## Closure as exceptional impossibility

**Weak example.** A receive loop unwraps channel reads because the sender
"always exists."

**Why it fails.** Shutdown, panic, and owner drop make closure an ordinary
lifecycle event.

**Risk.** panic loops, missing shutdown, or invisible owner failure.

**Improved direction.** Handle closure as stop, degrade, restart, or terminal
failure according to protocol.

**When justified.** A process-level invariant may deliberately terminate after
recording why closure proves internal corruption.

## Detached supervisor

**Weak example.** A background task restarts failed workers forever but its own
handle and failures are unobserved.

**Why it fails.** Moving failure into a supervisor does not establish custody of
the supervisor.

**Risk.** silent service loss or endless restart load.

**Improved direction.** Root the supervisor in the service task tree, bound
restarts, and expose terminal health.

**When justified.** Process-lifetime infrastructure can be top-level, but must
still be observed by the process owner.

## Atomics by folklore

**Weak example.** Code uses relaxed atomics because they are faster, or
sequential consistency because it feels safe.

**Why it fails.** Neither choice substitutes for a synchronization invariant;
stronger ordering can hide design confusion without establishing algorithmic
correctness.

**Risk.** stale reads, missing publication, rare protocol violation, or needless
cost.

**Improved direction.** State the happens-before relationship, model relevant
interleavings, or use a lock.

**When justified.** Independent telemetry counters can commonly use relaxed
ordering because they publish no other memory.

## Blocking pool as infinite escape

**Weak example.** Every slow or CPU-heavy action is moved to a blocking pool
with no admission limit.

**Why it fails.** Contention moves rather than disappears, and cancellation may
not stop running jobs.

**Risk.** thread growth, CPU saturation, and resource use after callers leave.

**Improved direction.** Bound admission, partition CPU-heavy work, account for
orphaned jobs, and monitor queue age.

**When justified.** Runtime-managed blocking facilities are appropriate when
their documented capacity and cancellation semantics satisfy the workload.

## Sleep-based concurrency test

**Weak example.** A test inserts sleeps and assumes a desired interleaving will
occur.

**Why it fails.** Timing varies by host and successful runs do not establish
the schedule was exercised.

**Risk.** flaky evidence or false confidence.

**Improved direction.** Use barriers, controlled clocks, injectable scheduling,
model checking, and explicit event observation.

**When justified.** Sleeps may bound an integration test deadline, but should
not be the synchronization mechanism that establishes its assertion.

---

## Source: `doctrines/0004-concurrency-and-async/glossary.md`

# Glossary

**Actor ownership**
: A model in which one task owns mutable state and other tasks request changes
through messages. It simplifies mutation authority but still requires mailbox
capacity, supervision, and shutdown semantics.

**Backpressure**
: A policy that makes downstream capacity constrain upstream production through
waiting, rejection, shedding, coalescing, or durable buffering.

**Cancellation safety**
: The property that dropping an incomplete future at a stated suspension point
does not violate its documented invariant or lose unrecoverable progress.

**Channel closure**
: The protocol event produced when the relevant sender or receiver set has
disappeared. Meaning depends on channel kind and component lifecycle.

**Convoying**
: Delayed progress caused when many operations serialize behind a slow holder or
resource.

**Detached task**
: A task whose completion is not structurally awaited by the immediate caller.
It still requires an operational owner.

**Graceful shutdown**
: A bounded lifecycle protocol that stops admission, accounts for outstanding
work, releases resources, and defines behavior at its deadline.

**Happens-before**
: A synchronization relationship used to reason that memory effects become
observable in a required order. It is stronger and more precise than
wall-clock intuition.

**Lock graph**
: A directed graph in which an edge represents acquiring one lock while holding
another. A cycle indicates a potential deadlock protocol.

**Lock poisoning**
: A mechanism that records that a panic occurred while exclusive state was
guarded. It signals possible invariant damage; recovery policy remains a
component decision.

**Retry amplification**
: Multiplication of attempts when independent layers retry one logical
operation.

**Structured concurrency**
: Task lifecycle organization in which child work has an accountable owner and
bounded relationship to parent completion, failure, and cancellation.

**Task supervision**
: Observation and policy for task completion, panic, cancellation, failure,
restart, and terminal degradation.

**Thundering herd**
: A synchronized group of waiters or retries that simultaneously contend for a
recovering resource.

**Unknown outcome**
: A state in which local evidence cannot determine whether an external effect
occurred. It requires reconciliation rather than assumed rejection.

---

## Source: `doctrines/0004-concurrency-and-async/references.md`

# References

Primary and authoritative material:

- [Rust standard library: `std::marker::Send`](https://doc.rust-lang.org/std/marker/trait.Send.html)
  and [`Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html) define
  language-level transfer and sharing marker contracts. They do not claim
  application-level deadlock or protocol correctness.
- [Rust standard library: `std::sync::Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
  documents locking and poisoning behavior used by the review rules.
- [Rust 1.97.1 standard library:
  `std::sync::nonpoison`](https://doc.rust-lang.org/1.97.1/std/sync/nonpoison/index.html)
  documents the pinned toolchain's nightly-only experimental non-poisoning lock
  namespace. Its API status must be rechecked for each selected toolchain.
- [Rust standard library: atomic memory ordering](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html)
  defines the available orderings. The doctrine adds the requirement to connect
  every selection to an application invariant.
- [The Rustonomicon: concurrency](https://doc.rust-lang.org/nomicon/concurrency.html)
  discusses unsafe concurrency and the role of `Send` and `Sync`.
- [The Async Book](https://rust-lang.github.io/async-book/) explains Rust
  futures, executors, and async programming mechanics. This doctrine adds
  operational ownership, capacity, and evidence gates.
- [Tokio tutorial: spawning](https://tokio.rs/tokio/tutorial/spawning) and
  [Tokio topic: graceful shutdown](https://tokio.rs/tokio/topics/shutdown)
  provide runtime-specific task and shutdown guidance.
- [Tokio `select!` documentation](https://docs.rs/tokio/latest/tokio/macro.select.html)
  documents cancellation behavior and fairness considerations for that runtime
  construct.
- [Tokio `spawn_blocking` documentation](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html)
  documents blocking-task behavior, including cancellation limitations.
- [Loom documentation](https://docs.rs/loom/latest/loom/) describes controlled
  exploration of concurrent executions for small Rust protocols.

These sources establish language, library, or runtime mechanics. Requirements
for ownership maps, capacity tables, retry inventories, guarantee ledgers, and
review severities are repository governance added to make those mechanics
auditable.

---

## Source: `doctrines/0005-persistence-boundaries/README.md`

---
id: RUST-DOC-0005
slug: persistence-boundaries
title: Persistence Boundaries and Domain Integrity
status: active
version: 0.2.0
normative: true
applies_to:
  - planning
  - implementation
  - review
  - audit
  - maintenance
risk_domains:
  - persistence
  - migrations
  - transactions
  - serialization
supersedes: []
superseded_by: null
---

# Persistence Boundaries and Domain Integrity

## Scope

This package governs data crossing between persistent representations and
trusted Rust domain types. It covers relational rows, document records,
key-value entries, event payloads, snapshots, migration code, transaction
boundaries, optimistic concurrency, and durable intent for later external
effects.

Persisted bytes are evidence that some writer stored a representation under
some historical rules. They are not automatic evidence that current domain
invariants hold. Database drivers can decode valid SQL values that are invalid
domain values. Old binaries, manual repair, incomplete migrations, relaxed
constraints, replication, and corruption can all produce records a trusted type
must reject.

## Out of scope

This doctrine does not prescribe a database product, object-relational mapper,
or event-sourcing architecture. It does not claim that schema constraints alone
prove application behavior. Distributed delivery and unknown external outcomes
are governed more fully by RUST-DOC-0006. SQL isolation and durability claims
must follow the chosen database's documented configuration and observed
behavior.

## Intended readers

- planners defining storage/domain separation and transaction scope;
- implementers writing row conversions, repositories, and migrations;
- reviewers tracing every decoding and update path;
- auditors searching for forged trusted values and lost updates;
- maintainers evolving schemas and persisted enums.

## Normative status

[`doctrine.md`](../doctrines/0005-persistence-boundaries/doctrine.md) is normative. Rules use stable identifiers.
Rationale and examples clarify application but do not silently create new
requirements. Waivers require scope, evidence, owner, and expiry.

## Prerequisite foundations

Read invariant classification, trust boundaries, evidence levels, guarantee
honesty, and complexity budget under [`../../foundations/`](../foundations/).
Persistence work particularly depends on the distinction between a parsed
representation, a policy-accepted value, a persisted fact, and a fact that
remains true in mutable external reality.

## Related material

- Patterns: [opaque newtypes](../patterns/opaque-newtypes.md),
  [smart constructors](../patterns/smart-constructors.md),
  [sum types](../patterns/sum-types.md),
  [validated collections](../patterns/validated-collections.md),
  [hybrid state machines](../patterns/hybrid-state-machines.md), and
  [explicit uncertainty](../patterns/explicit-uncertainty.md).
- Boundaries: [database decoding](../boundaries/database-decoding.md),
  [Serde](../boundaries/serde.md), messaging, configuration, and filesystems.
- Reviews: domain model, boundary, distributed effects, and final audit.
- Case studies: database transaction, payment lifecycle, invoice, and message delivery.

## Reading order

Read normative rules, then rationale. Use the decision framework when designing
storage shape, migration, or transaction scope. Apply the review gates to every
read and write path. Use the anti-pattern catalogue for adversarial bypass
search.

## Compact doctrine summary

Storage models and domain models should be distinct when their invariants or
evolution pressures differ. Every path from storage to a trusted type needs to
validate current invariants. Schema constraints reinforce but do not replace
domain construction. Transactions protect only operations within their actual
boundary and isolation semantics. Concurrency designs name the anomaly they
prevent and any anomaly they still permit; a per-row version check can prevent
lost updates while leaving cross-row write skew possible. Persistence plus
messaging requires durable coordination such as an outbox or an explicit
reconciliation design. Historical invalid data is quarantined or migrated; it
is never forged into a trusted value for convenience.

---

## Source: `doctrines/0005-persistence-boundaries/doctrine.md`

# Normative doctrine

## RUST-DOC-0005-R001 — Treat persisted data as boundary input

**Statement.** Data read from persistence MUST be treated as an untrusted
representation until it has been decoded and validated against current domain
invariants.

**Intent.** Prevent storage provenance from forging domain evidence.

**Applicability.** Rows, documents, snapshots, cached values, event payloads,
and restored backups.

**Allowed exceptions.** None for a type whose name carries a validated
invariant. Trusted storage infrastructure may reduce threat likelihood but not
remove the construction obligation.

**Review evidence.** A complete read-path inventory and conversions that call
the trusted constructor.

## RUST-DOC-0005-R002 — Separate models when contracts differ

**Statement.** Persistence models and domain models SHOULD be separated when
their nullability, versioning, normalization, compatibility, or invariant
contracts differ.

**Intent.** Prevent storage evolution concerns from weakening the domain model.

**Applicability.** Most durable business entities and versioned records.

**Allowed exceptions.** One representation may serve both roles when field
contracts are demonstrably identical and decoding still preserves invariants.

**Review evidence.** Field mapping, rationale for shared or separate models,
and tests for invalid stored representations.

## RUST-DOC-0005-R003 — Validate trusted newtypes during decoding

**Statement.** Database and serialized decoding MUST construct trusted newtypes
through their validated public path. A driver mapping MUST NOT write private
representation bytes through an unchecked or unsafe path merely to satisfy an
interface.

**Intent.** Preserve one invariant gate across every construction source.

**Applicability.** SQL decoding traits, ORM hooks, Serde adapters, event
deserializers, and cache loaders.

**Allowed exceptions.** A narrowly scoped internal constructor may accept
evidence already validated in the same operation, with the proof documented and
tested.

**Review evidence.** `TryFrom`, parser, or smart-constructor calls and negative
decoding tests.

## RUST-DOC-0005-R004 — Reinforce invariants in the schema

**Statement.** Schema constraints SHOULD reinforce stable value and
cross-column invariants that the database can enforce without duplicating
volatile business policy.

**Intent.** Defend against alternate writers and narrow invalid-data ingress.

**Applicability.** Nullability, ranges, uniqueness, referential integrity,
discriminators, and state-related column combinations.

**Allowed exceptions.** A constraint may remain application-only when the
database cannot express it reliably, enforcement would create unacceptable
coupling, or rollout cannot yet guarantee compatibility.

**Review evidence.** Invariant mapping to domain constructor, schema constraint,
transactional validation, or explicit residual gap.

## RUST-DOC-0005-R005 — Avoid contradictory nullable records

**Statement.** Partial records, boolean flags, and nullable associated fields
MUST NOT represent mutually exclusive domain states without a checked
discriminator and a validation rule that rejects contradictory combinations.

**Intent.** Prevent rows such as "paid without receipt" or "failed with settled
timestamp."

**Applicability.** Lifecycle tables, optional payload columns, and soft-state
flags.

**Allowed exceptions.** A deliberately incomplete staging record may exist in a
separate type and table whose lifecycle never exposes it as the completed
domain entity.

**Review evidence.** Row-state truth table, schema checks where feasible, and
conversion tests for every invalid combination.

## RUST-DOC-0005-R006 — Make migrations invariant-aware

**Statement.** Every migration MUST state which invariants it preserves,
strengthens, weakens, or transforms, and MUST define handling for rows that do
not satisfy the target invariant.

**Intent.** Treat migration as a domain transition rather than only a shape
change.

**Applicability.** Schema, data, index, encoding, and enum migrations.

**Allowed exceptions.** A metadata-only operation may state that domain
invariants are unaffected, with evidence.

**Review evidence.** Precondition query, transformation, postcondition query,
rollback or forward-repair strategy, and representative migration test.

## RUST-DOC-0005-R007 — Version durable representations

**Statement.** Persisted formats that can outlive one release MUST be versioned
or have an explicit compatibility and migration strategy.

**Intent.** Keep old values decodable without silently assigning new meaning.

**Applicability.** JSON blobs, snapshots, event payloads, files, cache entries
that survive deployment, and database schemas.

**Allowed exceptions.** Ephemeral caches may be invalidated atomically when
version changes, if stale values cannot be interpreted.

**Review evidence.** Version field or schema version, supported-reader matrix,
unknown-version behavior, and fixture tests.

## RUST-DOC-0005-R008 — Plan enum evolution

**Statement.** Persistence of enums MUST define storage encoding, unknown or
future value behavior, rename policy, and downgrade compatibility.

**Intent.** Avoid making source-level variant spelling an accidental permanent
wire contract.

**Applicability.** SQL enums, text discriminators, integer tags, and serialized
sum types.

**Allowed exceptions.** A closed, disposable dataset may reject unknown values
and rebuild from canonical input.

**Review evidence.** Stable encoding table, unknown-value path, migration plan,
and old/new reader tests.

## RUST-DOC-0005-R009 — Align transactions with cross-entity invariants

**Statement.** A cross-entity invariant that requires atomic observation and
mutation MUST be enforced within a transaction boundary and isolation mechanism
capable of protecting that invariant, or through an explicit alternative
coordination protocol. The design MUST name the concurrency anomaly being
controlled and the residual anomaly set permitted by the selected mechanism,
database, and configuration.

**Intent.** Prevent application prechecks from racing concurrent writers.

**Applicability.** Balances, uniqueness, inventory, state transitions,
aggregate versions, and paired records.

**Allowed exceptions.** Eventual convergence is permitted when temporary
violation is a documented domain state with bounded detection and repair.

**Review evidence.** Transaction scope, isolation analysis against the package
taxonomy, locking or constraint mechanism, concurrent test, and named residual
anomaly set.

## RUST-DOC-0005-R010 — Prevent lost updates

**Statement.** Read-modify-write operations subject to concurrent writers MUST
use optimistic version checks, locking, commutative updates, or another explicit
lost-update prevention strategy.

**Intent.** Stop later writes from silently erasing changes based on stale
state.

**Applicability.** Mutable entities, counters with derived fields, and
administrative edits.

**Allowed exceptions.** Last-write-wins is allowed only when it is the explicit
business policy and discarded updates are acceptable and observable where
needed.

**Review evidence.** Version predicate or locking query, conflict error,
concurrency test, and caller conflict policy.

## RUST-DOC-0005-R011 — Preserve transaction-handle lifecycle

**Statement.** Transaction APIs SHOULD prevent use after commit or rollback
through consuming methods or an equivalent runtime lifecycle guard. Commit
failure MUST preserve the distinction between confirmed rollback, confirmed
commit, and ambiguous outcome when the driver or protocol permits ambiguity.

**Intent.** Prevent stale transaction reuse and dishonest commit status.

**Applicability.** Database clients, unit-of-work abstractions, and transactional
repositories.

**Allowed exceptions.** A library-owned mutable transaction handle may enforce
the same lifecycle at runtime when consuming APIs are incompatible with the
driver.

**Review evidence.** Handle transition tests, compile-fail evidence where
useful, and connection-loss behavior.

## RUST-DOC-0005-R012 — Do not extend database atomicity to external effects

**Statement.** Database transaction success MUST NOT be claimed to include a
message, payment, file, or network effect outside the transaction's actual
resource boundary.

**Intent.** Prevent fictional atomicity across independent systems.

**Applicability.** State changes coupled to publishing or external calls.

**Allowed exceptions.** A documented distributed transaction mechanism may
state only the boundary and failure model it actually provides.

**Review evidence.** Effect inventory, atomic boundary diagram, failure matrix,
and reconciliation path.

## RUST-DOC-0005-R013 — Coordinate persistence and messaging durably

**Statement.** When a domain transition and message publication must not be
silently separated, the design SHOULD use a transactional outbox, inbox, event
log, or equivalent durable coordination protocol.

**Intent.** Make retry and recovery possible after process or network failure.

**Applicability.** Event publication, job enqueueing, and integration messages.

**Allowed exceptions.** A best-effort notification may remain outside durable
coordination when loss is an accepted, documented outcome.

**Review evidence.** Atomic write, publisher retry, deduplication identity,
retention, ordering scope, and operational lag metrics.

## RUST-DOC-0005-R014 — Quarantine invalid historical data

**Statement.** A stored representation that fails current domain validation
MUST be rejected, quarantined, repaired through an audited migration, or exposed
as an explicit invalid-record type. It MUST NOT be forged into the trusted type.

**Intent.** Preserve the meaning of trusted domain values while allowing
operational recovery.

**Applicability.** Production reads, imports, restores, and migration scans.

**Allowed exceptions.** None for trusted construction.

**Review evidence.** Diagnostic classification, record identity, sensitive-data
handling, repair workflow, and metrics.

## RUST-DOC-0005-R015 — Preserve unknown fields and values deliberately

**Statement.** Readers MUST choose and document whether unknown fields or values
are rejected, ignored, retained, or mapped to an explicit unknown variant.

**Intent.** Make forward compatibility and security posture deliberate.

**Applicability.** Flexible records, events, snapshots, and rolling upgrades.

**Allowed exceptions.** None; the chosen policy may be implicit in a format only
if documented and tested.

**Review evidence.** Compatibility matrix and tests for extra fields, missing
fields, and unknown discriminators.

## RUST-DOC-0005-R016 — Bound stored-input resource use

**Statement.** Decoding durable values MUST enforce appropriate limits on
length, nesting, allocation, decompression, and batch size before constructing
trusted in-memory state.

**Intent.** Prevent validly encoded but hostile or corrupted records from
exhausting resources.

**Applicability.** Blobs, arrays, compressed payloads, large text, and batch
queries.

**Allowed exceptions.** A format with a proven small physical bound may rely on
that bound and document it.

**Review evidence.** Limits, streaming behavior, oversized fixtures, and failure
mapping.

## RUST-DOC-0005-R017 — Record persistence guarantees and non-guarantees

**Statement.** Persistence designs MUST document the exact durability,
consistency, isolation, freshness, and external-effect claims they rely on,
including configuration assumptions.

**Intent.** Prevent product names or successful calls from implying stronger
guarantees than deployed behavior.

**Applicability.** Every durable domain component.

**Allowed exceptions.** None.

**Review evidence.** Guarantee ledger linked to database documentation,
configuration, tests, monitoring, and residual failure modes.

---

## Source: `doctrines/0005-persistence-boundaries/rationale.md`

# Rationale

## Storage is historical evidence

A row proves that bytes were accepted by a storage path. It may have been
written by an older binary, another service, an administrative tool, a partial
migration, a restored backup, or an import. Even if the database faithfully
enforces its schema, that schema may encode fewer or different invariants than
the current domain model.

Consequently, `FromRow`-style convenience cannot be allowed to forge
`PositiveMoney`, `VerifiedEmailAddress`, or `AuthorizedPayment`. Decode raw
storage values first, then call the same checked constructor used at other trust
boundaries. Failed conversion is evidence of a data-integrity incident, not a
reason to weaken the type.

## Separate representations clarify evolution

Storage favors stable encodings, compatibility fields, nullable rollout stages,
and query efficiency. Domain types favor precise legal states, private
construction, and behavior-oriented data. Combining them can be appropriate for
a simple immutable record, but it often causes storage nullability to leak into
business operations or domain refactors to rewrite durable history.

A raw row can represent exactly what exists, including malformed historical
combinations. A fallible conversion then establishes the stronger domain
evidence. This separation also gives quarantine tooling a representation for
invalid data without pretending it is valid.

## Defense in depth

Domain constructors protect normal application paths. Database constraints
protect against alternate writers and concurrency races that application
prechecks cannot see. Neither makes the other unnecessary. A uniqueness
constraint can arbitrate concurrent inserts; the domain still needs a
machine-actionable conflict result. A check constraint can reject zero amounts;
the `PositiveMoney` constructor still protects values before storage and after
decoding.

Schema policy should remain stable enough to share across writers. Rapidly
changing eligibility rules may belong in a transactional domain service rather
than a migration for every policy adjustment.

## Migrations change meaning

Adding a non-null column, splitting one state into two, changing money scale, or
normalizing an identifier transforms domain evidence. A safe migration
identifies existing records that violate the target, decides how evidence will
be established, and verifies the postcondition. Inventing a default can be
dishonest: filling `verified_at` with migration time does not prove verification
occurred.

Large deployments may need expand-and-contract rollout. New readers first
understand both forms; writers populate both; data is backfilled and checked;
then old support is removed. The compatibility window and rollback direction
must be explicit.

## Persisted enums are protocols

Source variants are easy to rename. Durable discriminators are not. Text names
improve inspection but still need stable mapping. Integer tags avoid spelling
coupling but require a registry. Native database enums have product-specific
migration and compatibility behavior. Unknown variants can be rejected for
closed internal data, retained as raw evidence, or represented explicitly for
forward-compatible consumers.

The choice depends on rolling deployment, downgrade, replay, and public API
needs. Exhaustive matching in current Rust code does not make historical storage
closed forever.

## Transactions protect scoped invariants

A transaction is useful only relative to its isolation semantics and the
operations inside it. Reading availability, checking it in application code,
then later updating without a constraint or lock can race another writer.
Optimistic version predicates detect stale state and force a caller decision.
Pessimistic locking can serialize access but affects contention and deadlock.
Commutative database updates can avoid read-modify-write races for suitable
operations.

Lost-update protection is not a complete isolation argument. Suppose two
transactions each read that at least one of two operators remains on duty, then
each marks a different operator off duty. Per-row version predicates both
succeed because the writes are disjoint, yet the cross-row invariant is false
after both commits. This is write skew. Snapshot isolation can permit it;
serializable isolation, a predicate-level lock, an invariant-enforcing
constraint, or another coordination protocol may prevent it, subject to the
selected product's exact contract.

No generic statement such as "inside a transaction" establishes serializable
business behavior. Review must connect the invariant to actual queries,
constraints, locks, configured isolation, prevented anomaly, and residual
anomaly set.

## Commit can be ambiguous

A client may send a commit and lose the connection before receiving the result.
Depending on protocol and driver evidence, the client may not know whether the
database committed. Treating every commit error as rollback can duplicate later
work; treating it as success can hide lost data. The operation needs identity,
observation, and reconciliation appropriate to the database and application.

A consuming transaction handle prevents accidental local reuse after an
attempt. It does not itself prove the database outcome.

## Persistence and external effects

A database cannot normally roll back an email already sent or a payment already
captured. Calling the external service inside a database transaction also holds
resources while waiting and still permits ambiguous combinations:

- external success followed by database rollback;
- database commit followed by publication failure;
- timeout after the external service executed;
- process loss between two steps.

A transactional outbox stores domain change and publication intent atomically
in one database. A publisher later delivers with retries. This closes the
specific "committed state but forgotten intent" gap; it does not create
exactly-once delivery. Consumers still need deduplication or idempotent effects,
and operators need lag and poison-message handling.

## Invalid historical data

Quarantine preserves two truths: the stored bytes exist, and they do not satisfy
the trusted domain invariant. It should retain record identity, validation
diagnostics, source, and repair history while limiting sensitive-data exposure.
A repair must establish evidence, not merely call an unchecked constructor.

Availability pressure can make rejection uncomfortable, but weakening a trusted
type makes every downstream use uncertain. An explicit `InvalidHistoricalRow`
or degraded read model contains the uncertainty.

## Guarantee ledger

| Claim                                       | Established by                     | Protected construction | Boundary preservation       | Escape hatches          | Does not prove                      | Residual runtime risk             |
| ------------------------------------------- | ---------------------------------- | ---------------------- | --------------------------- | ----------------------- | ----------------------------------- | --------------------------------- |
| decoded email satisfies syntax policy       | checked row conversion             | private newtype field  | all readers use `TryFrom`   | audited internal import | ownership or deliverability         | policy changes, corrupt row       |
| update used current aggregate version       | version predicate affected one row | repository API         | conflict preserved          | administrative repair   | absence of all business races       | retry conflict, isolation anomaly |
| outbox intent shares domain commit          | same local transaction             | repository operation   | publisher reads durable row | direct DB write         | single delivery or consumer success | duplicate, delay, poison message  |
| transaction handle cannot be reused locally | consuming commit/rollback          | private fields         | API lifecycle               | driver internals        | definite remote commit result       | connection loss ambiguity         |

## Cost of overapplication

Separate models add conversion code. Version envelopes add fields. Constraints
and strict decoding complicate rollout. Transactions and locks reduce
concurrency. Outboxes require workers and retention. These costs are justified
by consequential invariant and recovery needs, not ritual. Ephemeral,
rebuildable, bounded caches may use simpler handling. The design still must say
why loss or incompatibility is acceptable.

---

## Source: `doctrines/0005-persistence-boundaries/decision-framework.md`

# Decision framework

## Inventory the persistence contract

For each durable representation record:

- owner and alternate writers;
- physical format and schema version;
- domain type constructed;
- fields whose meanings differ;
- maximum lifetime;
- rolling-upgrade and downgrade needs;
- constraints and transaction isolation;
- invalid-data behavior;
- external effects coupled to writes;
- recovery, backup, and replay paths.

## Select one or two models

Use one Rust model only when storage and domain contracts are identical:
nullability, valid values, normalization, compatibility, and public exposure all
match. Otherwise use:

```text
StoredRow
    ↓ decode physical types
RawRecord
    ↓ TryFrom + current invariant validation
DomainEntity
```

On conversion failure retain a diagnostic representation rather than partially
constructing the domain entity.

## Choose invariant enforcement

| Invariant                   | Primary mechanism            | Reinforcement                 |
| --------------------------- | ---------------------------- | ----------------------------- |
| positive scalar             | private newtype constructor  | SQL check constraint          |
| unique business key         | domain conflict type         | unique constraint             |
| valid foreign reference     | repository/domain rule       | foreign key                   |
| state-associated fields     | sum-type conversion          | discriminator checks          |
| cross-row balance           | transactional service        | isolation, locks, constraints |
| current-write version       | optimistic predicate         | version column                |
| volatile eligibility policy | transactional domain service | audit record                  |

If the database cannot enforce an invariant, state the race and repair model.

## Decode decision

```text
Does the target type carry an invariant?
├─ no → ordinary physical decoding may be sufficient
└─ yes
   Can the driver call a fallible checked constructor?
   ├─ yes → implement checked mapping
   └─ no
      Decode a raw storage type first
      then convert through TryFrom
```

Reject any solution that writes private fields through unsafe code for
convenience.

## Migration decision

Classify the change:

- representation-only;
- invariant preserving;
- invariant strengthening;
- invariant weakening;
- evidence transformation;
- state split or merge;
- destructive history change.

For strengthening, scan all rows before enforcement. For evidence
transformation, define how new evidence is established; do not synthesize it.
For rolling deployments, define old-reader/new-writer and new-reader/old-writer
compatibility. Prefer forward repair when rollback would reinterpret new data
incorrectly.

## Concurrency decision

For each read-modify-write:

1. identify concurrent writers;
2. name the anomaly that matters using the package glossary;
3. choose a constraint, atomic update, optimistic version, lock, or isolation
   level;
4. name the residual anomaly set for the selected product and configuration;
5. preserve conflict as a structured result;
6. test at least two competing operations;
7. define caller retry and deadline.

Last-write-wins requires explicit business acceptance, not absence of a version
column.

## Persistence plus effect decision

```text
Must the durable transition and effect intent stay coupled?
├─ no → document accepted loss or independent retry
└─ yes
   Can both occur in one actual transactional resource?
   ├─ yes → use that transaction and state its boundary
   └─ no
      Can durable intent be stored with the domain change?
      ├─ yes → outbox/event log + idempotent publisher
      └─ no → saga/reconciliation with explicit uncertainty
```

Never call compensation rollback. Compensation is a new fallible effect.

## Invalid-data decision

Choose among:

- reject and fail the operation;
- quarantine record with identity and diagnostics;
- expose a separate degraded/invalid read model;
- run an audited repair migration;
- restore from verified source.

Do not replace invalid evidence with a guessed default. If availability requires
partial reads, ensure the partial type cannot enter trusted business operations.

## Stop conditions

Stop the design if:

- a trusted type has any decoding bypass;
- migration creates evidence it did not observe;
- enum encoding depends on unstable source names without policy;
- transaction isolation is assumed rather than matched to an invariant;
- update conflicts disappear into affected-row count zero;
- external effects are described as database-atomic;
- commit errors are all mapped to rollback;
- outbox delivery is described as exactly once without boundary-specific proof.

---

## Source: `doctrines/0005-persistence-boundaries/review-standard.md`

# Review standard

Mark every gate **pass**, **fail**, **not applicable**, or with an approved
**waiver reference**.

| Gate | Question                                              | Pass evidence            | Failure example                          | Severity | Remediation               |
| ---- | ----------------------------------------------------- | ------------------------ | ---------------------------------------- | -------- | ------------------------- |
| P01  | Are all durable representations inventoried?          | storage map              | cache snapshot omitted                   | high     | enumerate sources         |
| P02  | Are alternate writers known?                          | writer list              | admin tool bypasses service              | high     | constrain or validate     |
| P03  | Is persisted data treated as boundary input?          | fallible conversion      | row directly becomes trusted type        | critical | add raw model             |
| P04  | Are private newtype constructors preserved?           | checked constructor call | ORM writes field internally              | critical | route through `TryFrom`   |
| P05  | Do invalid rows fail explicitly?                      | structured error         | invalid value normalized silently        | critical | reject or quarantine      |
| P06  | Are conversion diagnostics actionable?                | record ID and category   | opaque decode error                      | medium   | add safe context          |
| P07  | Are sensitive values absent from diagnostics?         | redaction tests          | token printed with row error             | critical | redact                    |
| P08  | Are storage and domain models separated where needed? | contract comparison      | nullable row leaks into domain           | high     | split models              |
| P09  | Are null combinations validated?                      | truth table              | paid row has no receipt                  | critical | sum conversion and checks |
| P10  | Are defaults evidence-honest?                         | provenance               | migration invents verification time      | critical | derive or keep unknown    |
| P11  | Does schema reinforce stable invariants?              | constraint map           | alternate writer can store zero          | high     | add constraint            |
| P12  | Are constraint failures structured?                   | conflict mapping         | all become internal error                | high     | preserve category         |
| P13  | Are cross-row rules transactionally protected?        | isolation proof          | check then update races                  | critical | lock/constraint/protocol  |
| P14  | Is isolation tied to the anomaly?                     | documented analysis      | transaction assumed sufficient           | critical | choose mechanism          |
| P15  | Are concurrent writers tested?                        | competing-operation test | only sequential test                     | high     | add concurrency evidence  |
| P16  | Are lost updates prevented?                           | version or atomic update | blind overwrite                          | critical | add concurrency control   |
| P17  | Is version conflict visible?                          | typed conflict           | zero rows treated as success             | critical | return conflict           |
| P18  | Is last-write-wins explicit if used?                  | policy approval          | accidental overwrite                     | high     | document or prevent       |
| P19  | Is enum encoding stable?                              | encoding table           | source variant name persisted casually   | high     | define tags               |
| P20  | Are unknown enum values handled?                      | explicit branch          | decoder panics                           | high     | reject/retain/unknown     |
| P21  | Is downgrade behavior considered?                     | compatibility matrix     | old reader misinterprets new state       | high     | stage rollout             |
| P22  | Are durable formats versioned?                        | version strategy         | old snapshot silently decoded anew       | critical | add version/migration     |
| P23  | Are unknown versions rejected safely?                 | fixture test             | version ignored                          | high     | preserve incompatibility  |
| P24  | Does migration name invariant effects?                | migration contract       | shape-only description                   | high     | add domain analysis       |
| P25  | Does strengthening scan old rows?                     | precondition query       | constraint fails mid-rollout             | critical | scan and repair           |
| P26  | Is postcondition verified completely?                 | authoritative query      | sampled rows only                        | critical | query full affected set   |
| P27  | Is rollback semantically safe?                        | compatibility reasoning  | old binary corrupts new meaning          | high     | prefer forward repair     |
| P28  | Is rollout order explicit?                            | expand/contract sequence | writer deploys before readers            | critical | stage compatibility       |
| P29  | Are decoding resource limits set?                     | limits and tests         | huge blob allocated blindly              | high     | bound or stream           |
| P30  | Are batches bounded?                                  | pagination policy        | full table loaded                        | high     | page and cap              |
| P31  | Is transaction handle lifecycle guarded?              | consuming/runtime state  | reused after commit                      | high     | consume or reject         |
| P32  | Is commit ambiguity considered?                       | driver/protocol analysis | any error means rollback                 | critical | reconcile unknown         |
| P33  | Are rollback errors preserved?                        | cleanup result           | rollback failure discarded               | high     | report residual state     |
| P34  | Is connection loss behavior documented?               | failure matrix           | retry assumes no commit                  | critical | identify outcome          |
| P35  | Are external effects outside DB atomicity?            | boundary diagram         | email called transactionally             | critical | add durable protocol      |
| P36  | Is durable intent used when loss matters?             | outbox/inbox design      | commit then best-effort publish          | critical | couple intent             |
| P37  | Is outbox write in the same transaction?              | query evidence           | separate connection writes               | critical | make atomic               |
| P38  | Is publisher retry idempotent?                        | operation identity       | duplicate external effect                | critical | deduplicate/reconcile     |
| P39  | Is outbox lag observable?                             | metrics/alerts           | stuck events invisible                   | high     | instrument                |
| P40  | Is retention defined?                                 | cleanup policy           | dedup/outbox grows forever               | medium   | bound safely              |
| P41  | Is ordering scope documented?                         | aggregate/partition rule | insertion order called global            | high     | narrow claim              |
| P42  | Is invalid history quarantined?                       | explicit path            | unchecked constructor used               | critical | contain invalid evidence  |
| P43  | Is repair audited?                                    | before/after evidence    | manual edit unrecorded                   | high     | record repair             |
| P44  | Are backup/restores validated?                        | restore test             | stale schema restored blindly            | high     | migrate and check         |
| P45  | Are replicas/freshness claims accurate?               | read-routing contract    | replica read called current              | high     | state staleness           |
| P46  | Are durability settings identified?                   | configuration evidence   | product default assumed                  | critical | document and monitor      |
| P47  | Are schema and domain tests linked?                   | invariant matrix         | tests cover only model                   | medium   | add boundary cases        |
| P48  | Are administrative escape paths reviewed?             | access and audit policy  | direct SQL silently allowed              | high     | restrict and validate     |
| P49  | Does guarantee ledger state non-guarantees?           | completed ledger         | persisted implies externally complete    | critical | narrow claim              |
| P50  | Are evidence limits stated?                           | residual risks           | tests presented as proof of all history  | high     | document limits           |
| P51  | Is residual anomaly set named against the taxonomy?   | product-qualified set    | version check assumed to stop write skew | critical | define and test residuals |

Critical failures block merge. A waiver must identify owner, affected paths,
compensating controls, monitoring, expiry, and a condition for removal.

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0005-R001`, `RUST-DOC-0005-R002`, `RUST-DOC-0005-R003`, `RUST-DOC-0005-R004`
- `RUST-DOC-0005-R005`, `RUST-DOC-0005-R006`, `RUST-DOC-0005-R007`, `RUST-DOC-0005-R008`
- `RUST-DOC-0005-R009`, `RUST-DOC-0005-R010`, `RUST-DOC-0005-R011`, `RUST-DOC-0005-R012`
- `RUST-DOC-0005-R013`, `RUST-DOC-0005-R014`, `RUST-DOC-0005-R015`, `RUST-DOC-0005-R016`
- `RUST-DOC-0005-R017`

---

## Source: `doctrines/0005-persistence-boundaries/anti-patterns.md`

# Anti-pattern catalogue

## The database already validated it

**Weak example.** A row's `String` is placed directly inside a trusted email
newtype because it came from a controlled database.

**Why it fails.** The database may enforce only a physical text type, and other
writers or old data may violate current policy.

**Risk.** Every downstream user receives forged evidence.

**Improved direction.** Decode a raw row and call the checked constructor.

**When justified.** Direct mapping is reasonable only for a type with no
stronger invariant than its physical representation.

## Domain type equals row type

**Weak example.** Domain fields become optional because a rolling migration
needs nullable columns.

**Why it fails.** Temporary storage compatibility becomes permanent invalid
domain state.

**Risk.** Scattered checks and contradictory objects.

**Improved direction.** Keep a raw persistence model and fallibly convert to the
complete domain type.

**When justified.** A simple immutable record may share a representation when
all contracts truly match.

## Derived decoding bypass

**Weak example.** A serialization or ORM derive assigns a private
representation without executing validation.

**Why it fails.** A second construction path establishes less evidence under
the same type name.

**Risk.** hostile or historical data bypasses policy.

**Improved direction.** Use a raw adapter plus `TryFrom` or a manual checked
decoder.

**When justified.** A derive is safe when it delegates to the complete
validated conversion and tests prove rejection.

## Default as invented evidence

**Weak example.** A migration fills missing `authorized_by` with `"system"` so a
new non-null constraint can be added.

**Why it fails.** The value states an event that was not observed.

**Risk.** audit corruption and unauthorized behavior.

**Improved direction.** retain an explicit legacy/unknown state, derive from
authoritative history, or quarantine.

**When justified.** A default is valid for a true policy default that makes no
historical claim.

## Application uniqueness check

**Weak example.** Code queries for an identifier, sees none, then inserts it.

**Why it fails.** Concurrent writers can both pass the check.

**Risk.** duplicate identity and ambiguous lookup.

**Improved direction.** use a unique constraint and map its conflict.

**When justified.** A precheck may improve error messages but cannot be the
enforcement mechanism.

## Transaction therefore safe

**Weak example.** Review approves a cross-row update solely because it uses a
transaction.

**Why it fails.** Isolation level and query shape may permit the relevant
anomaly.

**Risk.** lost updates, overspending, or invalid state.

**Improved direction.** connect the invariant to constraints, locking,
versions, or suitable isolation.

**When justified.** The claim is valid after the actual mechanism is documented
and tested.

## Blind upsert

**Weak example.** An upsert overwrites every column from a stale object.

**Why it fails.** It erases concurrent updates and hides conflicts.

**Risk.** silent data loss.

**Improved direction.** update intended fields with a version predicate or use
commutative operations.

**When justified.** Last-write-wins data such as replaceable cache material may
accept it explicitly.

## Persist Rust variant spelling

**Weak example.** An enum derives text serialization and the resulting variant
name becomes a permanent database value without policy.

**Why it fails.** source refactors become data migrations and unknown values
break older readers.

**Risk.** incompatible rollout and replay failure.

**Improved direction.** define stable external tags and unknown handling.

**When justified.** Disposable, version-locked data may use direct spelling.

## External call inside transaction

**Weak example.** Code sends a message or captures a payment before committing
the database transaction and calls the whole operation atomic.

**Why it fails.** The external effect cannot be rolled back with the database.

**Risk.** effect without state, long locks, and ambiguous retry.

**Improved direction.** persist intent atomically and deliver through a
retriable, observable protocol.

**When justified.** A true shared transaction manager may coordinate specific
resources, but its exact failure boundary must be stated.

## Commit error equals rollback

**Weak example.** Any commit I/O error is mapped to `NotCommitted`.

**Why it fails.** The server may have committed before the response was lost.

**Risk.** duplicate retry or inconsistent reconciliation.

**Improved direction.** classify the outcome according to driver evidence and
retain operation identity.

**When justified.** A protocol may provide definitive non-commit evidence; cite
that mechanism.

## Outbox means exactly once

**Weak example.** A transactional outbox is described as exactly-once message
delivery.

**Why it fails.** publisher acknowledgement can be lost and delivery can repeat.

**Risk.** consumers perform duplicate effects.

**Improved direction.** state the atomic-intent guarantee, use stable message
identity, and design consumer idempotency.

**When justified.** A narrower exactly-once claim may hold inside a specifically
defined transactional boundary with proof.

## Skip invalid rows

**Weak example.** A query iterator silently omits rows that fail domain
conversion.

**Why it fails.** Corruption becomes missing business data with no owner.

**Risk.** incorrect totals, incomplete processing, and prolonged integrity
failure.

**Improved direction.** fail, quarantine, or return an explicit mixed result
with diagnostics.

**When justified.** An administrative scan may continue collecting all invalid
rows, but must report every omission.

---

## Source: `doctrines/0005-persistence-boundaries/glossary.md`

# Glossary

**Ambiguous commit**
: A commit attempt whose client-side result does not establish whether the
database made the transaction durable.

**Data migration**
: A controlled transformation of stored values, including the evidence and
invariant meaning attached to them.

**Durable intent**
: A persisted record that an external action should be attempted, written so
process failure cannot silently forget the obligation.

**Expand-and-contract**
: A rollout sequence that first adds a representation compatible with old code,
migrates readers and writers, then removes the old representation.

**Historical invalid data**
: A stored representation that exists but cannot establish current trusted
domain invariants.

**Inbox**
: Durable consumer-side recording used to recognize and coordinate repeated
message delivery.

**Lost update**
: An anomaly in which concurrent operations derive writes from overlapping
state and one committed write silently replaces or erases another.

**Optimistic concurrency**
: A strategy that performs an update only if a previously observed version or
predicate remains current.

**Outbox**
: A durable publication-intent record written in the same local transaction as
the associated domain transition.

**Persistence model**
: A representation designed for storage shape, compatibility, and physical
decoding, not automatically a trusted domain entity.

**Phantom**
: A concurrency phenomenon in which repeating a predicate query observes a
different qualifying row set because another transaction committed inserts,
deletes, or updates. Exact prevention semantics are product-specific.

**Quarantine**
: Isolation of invalid stored evidence with identity and diagnostics so it can
be audited and repaired without entering trusted operations.

**Schema constraint**
: A database-enforced predicate such as nullability, uniqueness, referential
integrity, or a check expression.

**Serializable**
: An isolation contract under which committed transaction effects are
equivalent to some serial execution. It can require abort and retry and does not
by itself establish real-time ordering, external-effect atomicity, or future
liveness.

**Serialization anomaly**
: A committed result that cannot be explained by any serial ordering of the
participating transactions.

**Snapshot isolation**
: An isolation model in which a transaction reads from a consistent snapshot
and concurrent write-write conflicts are rejected under the product's rules.
It can still permit write skew when transactions update disjoint rows.

**Storage discriminator**
: A stable encoded value selecting one variant or lifecycle state in a durable
representation.

**Write skew**
: A serialization anomaly in which transactions read a shared predicate or
invariant, update disjoint rows, and both commit because no direct write-write
conflict exposes the combined violation.

## Isolation-analysis map

This map supports RUST-DOC-0005-R009 analysis; it is not a substitute for the
selected database's primary documentation.

| Mechanism or level                   | Typical protection                                                            | Residual analysis still required                                                    |
| ------------------------------------ | ----------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| per-row version or compare-and-set   | detects a lost update on the guarded row or predicate                         | disjoint-row write skew, phantoms, unguarded alternate writers                      |
| snapshot isolation                   | stable transaction snapshot and product-defined write-write conflict handling | write skew and other serialization anomalies                                        |
| predicate or range lock              | protects the locked predicate or key range from specified interference        | complete lock scope, deadlock, alternate access paths, and product behavior         |
| serializable isolation               | rejects or blocks executions that would not have a serial explanation         | retry handling, product configuration, external effects, and real-time-order claims |
| schema constraint or atomic mutation | arbitrates the encoded predicate at the database boundary                     | invariants not encoded by that constraint or mutation                               |

PostgreSQL currently implements its Repeatable Read level using snapshot
isolation: it prevents the phantom reads described by its documentation but
still permits serialization anomalies such as write skew. PostgreSQL
Serializable adds detection and may abort a transaction, so applications must
retry the complete transaction. Other products can assign different guarantees
to similarly named levels.

---

## Source: `doctrines/0005-persistence-boundaries/references.md`

# References

- [Rust standard library: `TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)
  defines fallible conversion used to separate raw rows from trusted domain
  values.
- [PostgreSQL documentation: constraints](https://www.postgresql.org/docs/current/ddl-constraints.html)
  documents checks, uniqueness, primary keys, and referential constraints.
- [PostgreSQL documentation: transaction isolation](https://www.postgresql.org/docs/current/transaction-iso.html)
  describes phenomena and guarantees for its isolation levels. Other databases
  require their own primary documentation.
- [Berenson et al., "A Critique of ANSI SQL Isolation
  Levels"](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/tr-95-51.pdf)
  defines snapshot isolation and write skew and distinguishes them from the
  original ANSI phenomena.
- [PostgreSQL documentation: explicit locking](https://www.postgresql.org/docs/current/explicit-locking.html)
  documents row and table lock behavior and deadlock considerations.
- [PostgreSQL documentation: enumerated types](https://www.postgresql.org/docs/current/datatype-enum.html)
  illustrates product-specific persisted-enum properties and evolution limits.
- [Serde enum representations](https://serde.rs/enum-representations.html) and
  [custom conversion attributes](https://serde.rs/container-attrs.html#try_from)
  document serialization mechanisms relevant to durable formats.
- [CloudEvents specification](https://github.com/cloudevents/spec/blob/main/cloudevents/spec.md)
  provides an example of a versioned event envelope and stable context
  attributes.

These sources establish mechanics for particular language, format, or database
boundaries. The repository adds invariant mapping, migration evidence,
quarantine, and guarantee-ledger review requirements.

---

## Source: `doctrines/0006-distributed-uncertainty/README.md`

---
id: RUST-DOC-0006
slug: distributed-uncertainty
title: Distributed Effects, Uncertainty, and Reconciliation
status: active
version: 0.2.0
normative: true
applies_to:
  - planning
  - implementation
  - review
  - audit
  - maintenance
risk_domains:
  - distributed-systems
  - retries
  - idempotency
  - reconciliation
supersedes: []
superseded_by: null
---

# Distributed Effects, Uncertainty, and Reconciliation

## Scope

This package governs operations crossing process, host, service, database,
broker, or administrative boundaries where communication can fail separately
from execution. It covers timeouts, lost acknowledgements, retries,
idempotency, duplicate delivery, ordering, reconciliation, compensation,
causality, and external observations that may be stale.

Distributed APIs often return less evidence than local control flow suggests.
A caller can know that it stopped waiting without knowing whether the server
executed. A consumer can process a message while its acknowledgement is lost. A
database can commit before its client connection fails. Honest models preserve
confirmed success, confirmed rejection, local pre-execution failure, and unknown
outcome as distinct states when operations require different action.

## Out of scope

This doctrine does not promise one universal delivery or consensus protocol. It
does not prescribe a broker or database. It does not use Rust types to claim
that remote state remains current. Local concurrency rules are in
RUST-DOC-0004; persistence coordination is in RUST-DOC-0005; error categories
are in RUST-DOC-0002.

## Intended readers

- planners inventorying effects, retries, and reconciliation;
- implementers building clients, consumers, workers, and operation trackers;
- reviewers checking idempotency scope and timeout semantics;
- auditors searching for collapsed uncertainty and unsafe replay;
- maintainers changing external protocols or retention.

## Normative status

[`doctrine.md`](../doctrines/0006-distributed-uncertainty/doctrine.md) is normative. Rule identifiers are stable within
this version. Examples provide possible representations, not mandatory generic
shapes. Approved waivers must preserve operational visibility and carry a
bounded risk decision.

## Prerequisite foundations

Read trust boundaries, evidence levels, guarantee honesty, invariants, and
complexity budget under [`../../foundations/`](../foundations/). An
externally acknowledged value and a reconciled external outcome are distinct
evidence levels.

## Related material

- Patterns: [explicit uncertainty](../patterns/explicit-uncertainty.md),
  [consuming transitions](../patterns/consuming-transitions.md),
  [capability types](../patterns/capability-types.md), and
  [hybrid state machines](../patterns/hybrid-state-machines.md).
- Boundaries: [HTTP/RPC](../boundaries/http-and-rpc.md), messaging,
  [database decoding](../boundaries/database-decoding.md), and filesystems.
- Reviews: distributed effects, boundary, and
  [final correctness](../reviews/final-correctness-audit.md) audit.
- Case studies: payment lifecycle, [message delivery](../case-studies/message-delivery/),
  database transaction, invoice, and UI workflow.

## Reading order

Read the rules and rationale before assigning a binary `Result` to an external
effect. Use the decision framework to classify each failure point. Apply the
review standard to operation identity, retry, retention, and reconciliation.

## Compact doctrine summary

A timeout does not imply non-execution. Retrying an effect requires explicit
idempotency analysis. An idempotency key needs scope, uniqueness, retention,
payload-binding, and replay semantics. Outcome models distinguish confirmed
success, confirmed rejection, local failure before dispatch, and unknown
execution. Unknown states carry enough evidence for reconciliation. At-least-once
delivery means duplicates are expected. Ordering claims name their exact scope.
Compensation is a new fallible action, not rollback. Audit trails preserve
operation identity, attempts, correlation, causality, observations, and final
resolution. Time-based authority names its clock, timing bounds, and behavior
when those assumptions fail.

---

## Source: `doctrines/0006-distributed-uncertainty/doctrine.md`

# Normative doctrine

## RUST-DOC-0006-R001 — Do not equate timeout with non-execution

**Statement.** A timeout MUST NOT be represented as confirmed failure when the
remote operation may have executed.

**Intent.** Preserve the distinction between stopping local waiting and learning
remote outcome.

**Applicability.** Network requests, database commit, broker acknowledgement,
filesystem operations over remote mounts, and subprocess protocols.

**Allowed exceptions.** A timeout may be definitive only when protocol evidence
establishes that execution could not have begun or was atomically cancelled.

**Review evidence.** Protocol timeline, cancellation semantics, and explicit
unknown-outcome path.

## RUST-DOC-0006-R002 — Model operationally distinct outcomes

**Statement.** Outcome types MUST distinguish confirmed success, confirmed
rejection, local failure before dispatch, and unknown outcome when callers
require different recovery.

**Intent.** Prevent transport symptoms from erasing domain knowledge.

**Applicability.** Consequential external operations.

**Allowed exceptions.** Categories may combine when no caller action, audit
meaning, security consequence, or reconciliation path differs.

**Review evidence.** Outcome decision table and exhaustive caller handling.

## RUST-DOC-0006-R003 — Carry reconciliation evidence

**Statement.** An unknown outcome MUST carry or reference sufficient evidence
to reconcile it, including stable operation identity and the external target.

**Intent.** Make uncertainty actionable and auditable.

**Applicability.** Payments, messages, provisioning, commits, and any effect that
cannot safely be repeated blindly.

**Allowed exceptions.** An explicitly irreconcilable best-effort action may
retain only audit evidence if business policy accepts permanent uncertainty.

**Review evidence.** Reconciliation token, operation ID, request fingerprint,
target, attempt history, and observation method.

## RUST-DOC-0006-R004 — Analyze before retry

**Statement.** Every retry policy MUST classify the operation as safe to retry,
unsafe to retry, or reconcile-before-retry for each relevant failure point.

**Intent.** Prevent duplicate effects and unsafe assumptions.

**Applicability.** Clients, consumers, publishers, schedulers, and operator
runbooks.

**Allowed exceptions.** Pure reads may use a simpler safe-retry classification
when staleness and load remain documented.

**Review evidence.** Failure-point matrix, idempotency mechanism, deadline, and
attempt budget.

## RUST-DOC-0006-R005 — Define idempotency-key semantics

**Statement.** An idempotency key MUST have defined uniqueness, caller and
resource scope, payload binding, retention, concurrency, replay, and conflict
semantics.

**Intent.** Prevent a string field from being mistaken for idempotent behavior.

**Applicability.** Mutable external APIs and durable commands.

**Allowed exceptions.** Naturally idempotent operations may omit keys when their
semantic identity and repeated-result behavior are established independently.

**Review evidence.** Key contract, storage constraint, same-key/same-payload and
same-key/different-payload tests, and expiry policy.

## RUST-DOC-0006-R006 — Reuse operation identity across attempts

**Statement.** Retries of one logical operation MUST reuse its operation and
idempotency identity. A new identity MUST mean a new requested effect.

**Intent.** Allow receivers and reconcilers to distinguish replay from new
intent.

**Applicability.** External API requests, published commands, and repair tools.

**Allowed exceptions.** A protocol-mandated new transport attempt identifier may
be added, but it MUST remain correlated to the stable logical operation.

**Review evidence.** Identity lifecycle and attempt log.

## RUST-DOC-0006-R007 — Expect duplicate delivery

**Statement.** Consumers in at-least-once systems MUST expect duplicate
delivery and MUST define whether repeated processing is deduplicated,
idempotent, commutative, or safely rejected.

**Intent.** Make acknowledgement loss and redelivery ordinary protocol paths.

**Applicability.** Brokers, job queues, webhook delivery, change feeds, and
replayed logs.

**Allowed exceptions.** A verified at-most-once boundary may accept loss instead
of duplicates, with that loss documented.

**Review evidence.** Duplicate test, stable message identity, and effect-level
handling.

## RUST-DOC-0006-R008 — Persist deduplication durably

**Statement.** Deduplication that protects a durable effect MUST itself use
durable state with atomic relationship to that effect, and MUST define
retention.

**Intent.** Prevent process restart or pruning from reopening duplicate effects.

**Applicability.** Consumer inboxes, payment commands, and webhook handlers.

**Allowed exceptions.** In-memory deduplication may protect only ephemeral
best-effort work whose duplicate cost is accepted.

**Review evidence.** Unique key, transaction boundary, retention calculation,
and replay-after-restart test.

## RUST-DOC-0006-R009 — State ordering scope

**Statement.** Ordering claims MUST identify key or partition, producer set,
consumer concurrency, retry behavior, failover behavior, and observation point.

**Intent.** Prevent partition-local or producer-local order from becoming a
false global guarantee.

**Applicability.** Brokers, streams, event logs, RPC sequencing, and replication.

**Allowed exceptions.** None when business behavior relies on order.

**Review evidence.** Ordering contract and tests for retries, multiple
producers, and failover.

## RUST-DOC-0006-R010 — Qualify exactly-once claims

**Statement.** Any "exactly once" claim MUST identify the precise boundary,
identity, transactional mechanism, failure assumptions, retention, and effects
included. It MUST NOT imply exactly-once behavior beyond that boundary.

**Intent.** Replace a broad slogan with an auditable scoped guarantee.

**Applicability.** Messaging, stream processing, payments, jobs, and APIs.

**Allowed exceptions.** None.

**Review evidence.** Guarantee ledger, protocol documentation, duplicate tests,
and excluded effects.

## RUST-DOC-0006-R011 — Coordinate acknowledgement with effect

**Statement.** A consumer MUST define the order and atomic relationship among
effect execution, durable progress, and acknowledgement.

**Intent.** Make the duplicate-versus-loss tradeoff visible.

**Applicability.** Message and job consumers.

**Allowed exceptions.** Best-effort consumers may acknowledge early only when
loss is accepted and measured.

**Review evidence.** Crash-point matrix and tests before and after each durable
step.

## RUST-DOC-0006-R012 — Treat compensation as a new effect

**Statement.** Sagas and compensating operations MUST NOT be described as
rollback. Each compensation MUST remain fallible, idempotency-analyzed, and
capable of an unknown outcome.

**Intent.** Preserve real-world irreversibility and changed conditions.

**Applicability.** Distributed workflows, reservations, payments, and
provisioning.

**Allowed exceptions.** A local database rollback may be called rollback within
its actual transaction boundary.

**Review evidence.** Forward/compensation pairs, business non-equivalence,
failure handling, and reconciliation.

## RUST-DOC-0006-R013 — Treat observations as time-scoped evidence

**Statement.** External observations MUST record or imply their observation
time and MUST NOT be presented as immutable current truth when the external
state can change.

**Intent.** Prevent stale reads from becoming permanent authority.

**Applicability.** Status queries, authorization, inventory, leases, and
reconciliation.

**Allowed exceptions.** Immutable append-only facts may remain stable when the
source contract establishes immutability.

**Review evidence.** Freshness policy, version or timestamp, cache behavior, and
revalidation trigger.

## RUST-DOC-0006-R014 — Address concurrent execution and split brain

**Statement.** Where multiple workers or coordinators can act on one logical
operation, the design MUST address concurrent execution using ownership,
leases with fencing, compare-and-set state, consensus-backed leadership, or an
effect-level idempotency mechanism. When a lease, expiry, or deadline
contributes to that authority, the design MUST define the clock source, whether
elapsed or wall time is used, accepted clock-skew, process-pause, and
renewal-delay bounds, and behavior when any timing assumption fails.

**Intent.** Prevent stale owners and duplicate coordinators from acting with
equal authority, including after a timing assumption ceases to hold.

**Applicability.** Reconciliation workers, schedulers, failover, distributed
locks, leases, and other time-based authority.

**Allowed exceptions.** Concurrent execution is allowed for commutative,
duplicate-safe operations with evidence.

**Review evidence.** Authority protocol, expiry, fencing token use, clock source
and kind, quantified timing bounds, assumption-failure behavior, and overlap
test.

## RUST-DOC-0006-R015 — Bound retries and reconciliation

**Statement.** Retry and reconciliation loops MUST have bounded concurrency,
attempt or time budgets, backoff where appropriate, terminal escalation, and
observability.

**Intent.** Prevent uncertainty from turning into permanent load or hidden
backlog.

**Applicability.** Retry queues, reconcilers, publishers, and operator repair.

**Allowed exceptions.** A durable obligation may remain pending indefinitely,
but each execution cycle still requires bounded work and visible age.

**Review evidence.** Queue capacity, schedule, age metrics, dead-letter or
manual escalation, and overload test.

## RUST-DOC-0006-R016 — Preserve correlation and causality

**Statement.** Audit trails MUST preserve stable operation identity, attempt
identity, triggering event, parent correlation, request fingerprint, outcome
observations, and reconciliation decisions where these affect accountability.

**Intent.** Reconstruct what was requested, attempted, observed, and resolved.

**Applicability.** Consequential distributed effects.

**Allowed exceptions.** Low-risk telemetry may use aggregated correlation when
individual reconstruction is unnecessary.

**Review evidence.** Event schema, trace propagation, redaction, and end-to-end
incident query.

## RUST-DOC-0006-R017 — Protect sensitive reconciliation data

**Statement.** Reconciliation and audit evidence MUST contain enough identity
to act without unnecessarily storing credentials, secret payloads, or sensitive
personal data.

**Intent.** Avoid turning operational evidence into a second secret database.

**Applicability.** Operation logs, dead-letter records, tracing, and support
tools.

**Allowed exceptions.** Required regulated evidence may be retained with
documented access, encryption, minimization, and deletion policy.

**Review evidence.** Field classification, redaction tests, access policy, and
retention.

## RUST-DOC-0006-R018 — Test failure points, not only final errors

**Statement.** Distributed-effect tests MUST inject loss, delay, duplication,
reordering, concurrent execution, and crash points between durable steps in
proportion to risk.

**Intent.** Exercise ambiguity and replay paths hidden by happy-path mocks.

**Applicability.** Integrations, consumers, publishers, and reconcilers.

**Allowed exceptions.** A low-risk pure read may narrow the matrix and state
why.

**Review evidence.** Fault matrix linked to invariants, test results, and
unexercised assumptions.

## RUST-DOC-0006-R019 — State residual uncertainty

**Statement.** Public and internal contracts MUST state which outcomes can
remain unknown, how long, who owns reconciliation, and what users or operators
may safely do meanwhile.

**Intent.** Make uncertainty an owned lifecycle state rather than an error
message.

**Applicability.** Every consequential effect with ambiguous execution.

**Allowed exceptions.** None.

**Review evidence.** State machine, service-level target, escalation path, and
guarantee ledger.

---

## Source: `doctrines/0006-distributed-uncertainty/rationale.md`

# Rationale

## Communication failure is not execution evidence

In a request-response exchange, the request can be lost, execute and return, or
execute while the response is lost. A caller observing a timeout cannot
distinguish all of these cases without additional protocol evidence. Local
cancellation affects the future being awaited; it does not retract bytes
already received by another system.

Binary `Result<Success, Failure>` is suitable only when the error side retains
the distinctions callers need. Consequential effects often benefit from a
domain-specific shape:

```rust
pub enum OperationOutcome<T, R, E> {
    Confirmed(T),
    Rejected(E),
    Unknown { reconciliation: R },
}
```

Local failures known to occur before dispatch can remain an outer error or an
additional variant. The important property is not the generic spelling but
that confirmed rejection and unknown execution cannot be confused.

## Idempotency is a protocol

An idempotency key has value only when a receiver stores and interprets it. The
contract must answer:

- uniqueness scope: global, account, resource, or endpoint;
- who generates the key and when;
- whether the same key binds to a request fingerprint;
- behavior for same key with different payload;
- behavior while the first attempt is still running;
- response replay semantics;
- retention duration and expiry;
- atomic relationship to the effect;
- behavior after retention expires.

A random string alone proves none of these. Client-generated keys can be
reliable identities when generation occurs once per logical intent and every
attempt reuses them. Generating a key inside the retry loop defeats the
protocol.

## Naturally idempotent is contextual

Setting a resource to a complete desired value can be idempotent at that
resource boundary, while triggering notifications or audit records on every
call remains non-idempotent. Deleting an already absent object may be
idempotent in state but return different observations. An operation's
idempotency claim must name the effect set and response semantics.

Commutativity is different: two increments may commute but replaying one still
changes the total. Deduplication is different again: it recognizes a repeated
identity and suppresses or replays the prior result.

## At-least-once consumers

A consumer that acts before acknowledging can crash after the effect but before
the acknowledgement. Redelivery follows. A consumer that acknowledges first
can crash before the effect and lose work. An inbox stored atomically with a
local database effect can close this gap for that database, but cannot
automatically include a remote payment or email.

Deduplication retention must cover the broker's replay horizon, operational
replay policy, and worst outage. Removing the key reopens the operation. A
unique inbox entry without atomic effect coordination can also record completion
before the effect occurred.

## Reconciliation is normal execution

An unknown payment capture should become a durable state with operation ID,
provider key, target account, amount fingerprint, attempt history, and a next
safe observation. A reconciler queries the authoritative provider or consumes a
signed event, compares evidence, and transitions to confirmed success,
confirmed rejection, still unknown, or human escalation.

Reconciliation queries can fail and observations can be stale. A response
saying "not found" may be definitive only after the provider's processing and
retention windows. The state machine must allow repeated observation without
turning uncertainty into imagined certainty.

## Ordering is scoped

One producer can assign monotonically increasing sequence numbers for one
aggregate. A partitioned broker can preserve partition order. Parallel
consumers, retries, and dead-letter replay can still change processing order.
Global total order usually requires stronger coordination and may reduce
availability or throughput.

Business logic should ask for the weakest sufficient order: per-account
sequence, causal predecessor, monotonic version, or commutative merge. An
explicit stale-version rejection may be more robust than relying on delivery
order.

## Exactly once requires a boundary

Some stream processors can coordinate input offsets and output state in one
transaction. A database unique constraint can make one operation identity apply
once to that database mutation. These are valuable guarantees, but they do not
automatically encompass an HTTP call, human action, email delivery, or an
uncoordinated database.

Review should rewrite every broad exactly-once claim into:

> For operation identities retained for the stated period, this mechanism
> atomically records the local effect and processed identity in the stated
> resource under the documented failure assumptions.

Anything outside that sentence remains subject to duplicates, loss, or
uncertainty.

## Compensation is not time reversal

Refunding a payment does not make capture unoccur. Releasing inventory later
does not guarantee the same customer experience or price. Deleting a created
resource may fail because another party now depends on it. Compensation is a
new command under current reality, with authorization, idempotency, timeout,
and reconciliation of its own.

Saga state must retain both forward and compensating outcomes. A failed
compensation may require manual resolution rather than pretending the original
transaction rolled back.

## Leases, clocks, and stale owners

A lease grants authority for a bounded period according to some clock and
renewal protocol. A paused or partitioned worker may continue after another
worker acquires a new lease. Fencing tokens let the protected resource reject
operations from older owners. A distributed lock that cannot fence the effect
may reduce overlap likelihood without preventing stale-owner execution.

Clock skew, process pauses, renewal delay, and resource support belong in the
guarantee ledger. Rust ownership can prevent cloning a local lease handle; it
cannot revoke authority already accepted by a remote system.

A reviewable time-based authority contract names:

- the component that supplies authority time;
- monotonic elapsed time versus civil wall time and any conversion between
  them;
- maximum accepted skew, process pause, scheduling delay, and renewal latency;
- the safety margin between renewal and expiry;
- the protected resource's fencing or rejection behavior; and
- the transition taken when a bound is exceeded or the clock is unavailable.

Tests then force overlap, delayed renewal, and bound failure at the protocol
seam. They provide scoped evidence for the implementation; they do not prove
that production clocks or processes always stay within the bounds.

## Audit without secret replication

Incident reconstruction needs stable IDs, timestamps, target identity,
fingerprints, attempt outcomes, and causal links. It rarely needs raw
credentials, full payment data, or message secrets. Hashes used as fingerprints
must be chosen with awareness of low-entropy values and correlation risk.
Access and retention should match the evidence's sensitivity.

## Guarantee ledger

| Claim                                         | Established by                             | Protected construction        | Boundary preservation    | Escape hatches          | Does not prove                                         | Residual runtime risk          |
| --------------------------------------------- | ------------------------------------------ | ----------------------------- | ------------------------ | ----------------------- | ------------------------------------------------------ | ------------------------------ |
| operation has stable identity                 | generated once and persisted               | private operation constructor | reused across attempts   | administrative replay   | effect executed once                                   | identity collision, misuse     |
| provider confirmed capture                    | authenticated response or reconciled event | outcome transition            | evidence retained        | operator override       | later settlement                                       | provider reversal, stale event |
| capture is unknown                            | timeout after possible dispatch            | explicit variant              | token persists           | destructive manual edit | success or rejection                                   | delayed observation            |
| duplicate local DB effect is suppressed       | unique inbox plus atomic mutation          | repository transaction        | durable identity         | retention expiry        | remote side effect uniqueness                          | late replay                    |
| worker currently holds time-bounded authority | checked acquisition plus clock contract    | non-clone authority           | fencing sent with writes | raw backend access      | synchronized clocks or exclusive remote action forever | pause, skew, partition, expiry |

## Proportionality

Not every telemetry ping needs a reconciliation worker. Best-effort actions may
accept loss or duplication when the product contract says so. The design still
states that choice. Consequential financial, authorization, provisioning, and
user-visible effects usually justify durable identities and explicit unknown
states. Type and storage complexity should track the cost of an incorrect
repeat, lost action, or false status.

---

## Source: `doctrines/0006-distributed-uncertainty/decision-framework.md`

# Decision framework

## Build an effect inventory

For every external operation record:

- logical intent and stable operation identity;
- target system and resource;
- point after which execution may have occurred;
- response and acknowledgement evidence;
- timeout and cancellation semantics;
- natural idempotency, key-based idempotency, or deduplication;
- retry owner and attempt budget;
- unknown-outcome representation;
- reconciliation source and owner;
- compensation, if any;
- audit and sensitive-data requirements.

## Classify each failure point

| Failure point                                   | Knowledge                                        | Default action                              |
| ----------------------------------------------- | ------------------------------------------------ | ------------------------------------------- |
| local validation before dispatch                | no request sent                                  | correct or reject                           |
| admission rejection with authenticated response | confirmed rejection                              | do not blind retry unless condition changes |
| connection failure before any bytes can be sent | likely local failure, subject to transport proof | safe retry if established                   |
| failure after request may be received           | execution unknown                                | reconcile or idempotent replay              |
| authenticated success response                  | confirmed at response boundary                   | persist evidence                            |
| acknowledgement loss after consumer effect      | effect may repeat                                | deduplicate on redelivery                   |

Do not infer exact transport timing without support from the library and
protocol.

## Outcome decision tree

```text
Did authoritative evidence confirm success?
├─ yes → Confirmed(success evidence)
└─ no
   Did authoritative evidence confirm rejection/non-execution?
   ├─ yes → Rejected(reason)
   └─ no
      Is it proven no request crossed the execution boundary?
      ├─ yes → LocalFailure(retry guidance)
      └─ no → Unknown(reconciliation evidence)
```

## Idempotency decision

Ask in order:

1. Does repeating the complete effect set yield the same state and acceptable
   response?
2. If not, can the receiver atomically bind an operation key to the effect?
3. Does the key cover concurrent duplicate attempts?
4. Is the request payload bound to the key?
5. Is retention longer than every replay and retry horizon?
6. Can external effects occur outside that atomic boundary?
7. What happens after expiry?

If any consequential effect remains outside the idempotent boundary, unknown
outcomes still require reconciliation.

## Retry decision

| Classification                       | Permitted behavior                                   |
| ------------------------------------ | ---------------------------------------------------- |
| safe retry                           | reuse operation identity within remaining budget     |
| unsafe retry                         | stop and escalate                                    |
| reconcile before retry               | observe authoritative state, then decide             |
| confirmed rejection                  | retry only after documented condition changes        |
| rate/overload response               | honor server guidance, backoff, jitter, cap attempts |
| authentication/authorization failure | repair authority; do not storm                       |

Calculate multiplication across callers, middleware, proxies, workers, and
libraries. One logical deadline constrains all layers.

## Consumer decision

Choose an acknowledgement position by examining crash points:

```text
receive
  ↓
claim/deduplicate
  ↓
perform local or external effect
  ↓
record outcome/progress
  ↓
acknowledge
```

For a local database effect, combine inbox claim, effect, and progress in one
transaction where possible. For an external effect, persist operation identity
and unknown state before attempting, then reconcile ambiguous outcomes.

## Ordering decision

Identify the required relationship:

- no order;
- per-producer order;
- per-aggregate version order;
- causal predecessor;
- partition order;
- total order.

Prefer versions and stale-write rejection when they express the invariant more
directly than delivery order. Define duplicate and gap behavior.

## Reconciliation design

A reconciliation record should contain:

| Field               | Purpose                                     |
| ------------------- | ------------------------------------------- |
| operation ID        | stable logical identity                     |
| external key        | provider lookup/deduplication               |
| request fingerprint | compare intent without unnecessary raw data |
| target              | select authoritative source                 |
| first/last attempt  | timeline                                    |
| observation cursor  | resume progress                             |
| next action time    | bounded scheduling                          |
| attempt count       | escalation                                  |
| current evidence    | explain state                               |
| owner               | operational accountability                  |

Define confirmed terminal transitions and a still-unknown path. Human override
must be audited as new evidence or a policy decision, never retroactive proof.

## Stop conditions

Stop and redesign when:

- timeout maps directly to rejection;
- retry generates a new idempotency key;
- same key with different payload has no conflict rule;
- deduplication is volatile but protects a durable effect;
- acknowledgement can precede required durable evidence without accepted loss;
- exactly-once language lacks a boundary;
- compensation is assumed infallible;
- lease ownership lacks fencing where stale workers can harm;
- unknown outcomes have no durable owner;
- audit data contains avoidable secrets.

---

## Source: `doctrines/0006-distributed-uncertainty/review-standard.md`

# Review standard

Each gate receives **pass**, **fail**, **not applicable**, or an approved
**waiver reference**.

| Gate | Question                                          | Pass evidence                       | Failure example                         | Severity | Remediation                   |
| ---- | ------------------------------------------------- | ----------------------------------- | --------------------------------------- | -------- | ----------------------------- |
| D01  | Are all external effects inventoried?             | effect map                          | notification hidden in callback         | critical | enumerate side effects        |
| D02  | Is logical operation identity stable?             | persisted ID                        | ID generated per retry                  | critical | generate once                 |
| D03  | Are transport attempts separately identified?     | attempt log                         | retries indistinguishable               | medium   | add attempt identity          |
| D04  | Is request fingerprint retained safely?           | canonical fingerprint               | same key accepts changed amount         | critical | bind payload                  |
| D05  | Does timeout avoid definitive failure?            | unknown branch                      | timeout becomes declined                | critical | preserve uncertainty          |
| D06  | Is pre-dispatch failure actually proven?          | transport evidence                  | connection error guessed early          | high     | narrow classification         |
| D07  | Are confirmed rejections authenticated?           | protocol evidence                   | proxy text treated as provider decision | high     | validate source               |
| D08  | Are outcomes operationally distinct?              | outcome table                       | one generic error                       | critical | structure outcomes            |
| D09  | Does unknown carry reconciliation identity?       | durable token                       | only error string remains               | critical | persist evidence              |
| D10  | Is reconciliation owner named?                    | service/runbook owner               | unknown state abandoned                 | critical | assign custody                |
| D11  | Is reconciliation source authoritative?           | provider query/event contract       | local cache decides                     | critical | use authoritative observation |
| D12  | Is observation freshness stated?                  | timestamp/version                   | stale read called current               | high     | record and revalidate         |
| D13  | Can still-unknown remain explicit?                | state transition                    | absence converted to failure too early  | critical | retain state                  |
| D14  | Is escalation bounded by age/attempt?             | policy                              | retries continue silently forever       | high     | escalate visibly              |
| D15  | Is idempotency scope defined?                     | account/resource/endpoint rule      | key meaning global by assumption        | critical | define namespace              |
| D16  | Is key uniqueness defined?                        | generator and collision analysis    | timestamp-only key                      | high     | use robust identity           |
| D17  | Is payload bound to key?                          | conflict test                       | same key changes request                | critical | store fingerprint             |
| D18  | Are concurrent same-key calls handled?            | atomic claim                        | both execute before record              | critical | serialize/constraint          |
| D19  | Is response replay behavior defined?              | stored terminal response            | duplicate gets unrelated response       | high     | define replay                 |
| D20  | Is key retention sufficient?                      | horizon calculation                 | key pruned before broker replay         | critical | extend or constrain replay    |
| D21  | Is post-expiry behavior documented?               | contract                            | old key silently executes again         | high     | reject or identify new intent |
| D22  | Are naturally idempotent claims scoped?           | effect set                          | repeated email called idempotent        | critical | analyze all effects           |
| D23  | Does every retry reuse identity?                  | attempt trace                       | retry loop regenerates key              | critical | move ID outside loop          |
| D24  | Is retry classification per failure point?        | matrix                              | retry every I/O error                   | critical | classify                      |
| D25  | Is total attempt budget bounded?                  | equation                            | layers multiply without cap             | high     | coordinate                    |
| D26  | Is one deadline propagated?                       | remaining-time budget               | each layer restarts timeout             | high     | propagate deadline            |
| D27  | Are backoff and jitter appropriate?               | policy                              | synchronized fixed retry                | high     | desynchronize                 |
| D28  | Is downstream overload honored?                   | rate-limit handling                 | immediate repeated retry                | critical | wait/shed                     |
| D29  | Are duplicates expected by consumers?             | duplicate test                      | duplicate panics or repeats charge      | critical | deduplicate/idempotent effect |
| D30  | Is dedup state durable when needed?               | inbox/store                         | in-memory set                           | critical | persist                       |
| D31  | Is claim atomic with local effect?                | transaction proof                   | inbox records before effect             | critical | coordinate                    |
| D32  | Is acknowledgement order explicit?                | crash matrix                        | ack timing incidental                   | critical | define protocol               |
| D33  | Are crash points before/after effect tested?      | fault tests                         | only happy path                         | high     | inject crashes                |
| D34  | Is poison-message handling defined?               | quarantine/dead-letter              | endless hot loop                        | high     | isolate and escalate          |
| D35  | Is replay policy explicit?                        | operator procedure                  | replay duplicates unknown effects       | high     | preserve identities           |
| D36  | Is ordering scope named?                          | key/partition contract              | global FIFO claim                       | critical | narrow                        |
| D37  | Are gaps handled?                                 | version policy                      | missing predecessor ignored             | high     | wait/reconcile/reject         |
| D38  | Are out-of-order events tested?                   | sequence fixtures                   | assumed broker order                    | high     | add versions                  |
| D39  | Are exactly-once claims bounded?                  | guarantee ledger                    | broad slogan                            | critical | specify mechanism             |
| D40  | Are excluded external effects named?              | boundary diagram                    | DB transaction includes email claim     | critical | list exclusions               |
| D41  | Is compensation called a new effect?              | saga state model                    | refund called rollback                  | high     | model separately              |
| D42  | Is compensation failure handled?                  | outcome and reconcile path          | compensation assumed successful         | critical | retain uncertainty            |
| D43  | Is compensation idempotency analyzed?             | repeat test                         | duplicate reversal                      | critical | stable identity               |
| D44  | Are concurrent coordinators controlled?           | lease/CAS protocol                  | two reconcilers both act                | critical | claim and fence               |
| D45  | Do leases use fencing where needed?               | monotonic token at resource         | expired owner still accepted            | critical | add fencing                   |
| D46  | Is the time-authority contract complete?          | source, clock kind, bounds, failure | wall clocks assumed identical           | critical | define bounds and failure     |
| D47  | Is audit causality preserved?                     | parent/trigger IDs                  | attempts cannot be reconstructed        | high     | enrich audit schema           |
| D48  | Are audit secrets minimized?                      | field classification                | raw credential logged                   | critical | redact/minimize               |
| D49  | Are retry/reconcile queues bounded?               | capacity and age metrics            | backlog consumes memory                 | critical | persist and bound workers     |
| D50  | Are fault tests representative?                   | loss/delay/duplicate/reorder matrix | mock returns only error                 | high     | inject protocol failures      |
| D51  | Does guarantee ledger state residual uncertainty? | completed ledger                    | types imply remote permanence           | critical | narrow claims                 |
| D52  | Can users act safely while outcome is unknown?    | UI/API contract                     | retry button duplicates effect          | critical | gate action or reconcile      |

Critical failures block merge. Waivers need a named owner, affected operations,
accepted consequence, compensating control, monitoring, expiry, and resolution
condition.

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0006-R001`, `RUST-DOC-0006-R002`, `RUST-DOC-0006-R003`, `RUST-DOC-0006-R004`
- `RUST-DOC-0006-R005`, `RUST-DOC-0006-R006`, `RUST-DOC-0006-R007`, `RUST-DOC-0006-R008`
- `RUST-DOC-0006-R009`, `RUST-DOC-0006-R010`, `RUST-DOC-0006-R011`, `RUST-DOC-0006-R012`
- `RUST-DOC-0006-R013`, `RUST-DOC-0006-R014`, `RUST-DOC-0006-R015`, `RUST-DOC-0006-R016`
- `RUST-DOC-0006-R017`, `RUST-DOC-0006-R018`, `RUST-DOC-0006-R019`

---

## Source: `doctrines/0006-distributed-uncertainty/anti-patterns.md`

# Anti-pattern catalogue

## Timeout equals failure

**Weak example.** A payment timeout becomes `PaymentFailed`, enabling a new
capture immediately.

**Why it fails.** The provider may have executed before the response was lost.

**Risk.** duplicate charge and false customer status.

**Improved direction.** transition to `CaptureUnknown` with provider key and
reconcile before new action.

**When justified.** A protocol-specific cancellation acknowledgement may prove
non-execution.

## Retry with a fresh key

**Weak example.** The retry helper creates an idempotency key on every attempt.

**Why it fails.** The receiver sees each attempt as new intent.

**Risk.** duplicate irreversible effects.

**Improved direction.** create and persist identity once before dispatch.

**When justified.** A fresh key is correct only for a genuinely new requested
effect.

## Key-shaped decoration

**Weak example.** An API accepts `Idempotency-Key` but stores nothing, ignores
payload conflicts, or expires keys immediately.

**Why it fails.** Syntax is present without replay semantics.

**Risk.** callers retry under a false guarantee.

**Improved direction.** define and test scope, binding, atomic claim, replay,
retention, and expiry.

**When justified.** None if the interface claims idempotency.

## Retry every transport error

**Weak example.** All I/O errors trigger exponential retry.

**Why it fails.** Some occur after execution and some represent persistent
authority or validation failure.

**Risk.** duplicate effects and amplified load.

**Improved direction.** classify by failure point and choose safe retry,
reconcile, or terminal handling.

**When justified.** Pure reads may broadly retry within a deadline when load and
staleness are controlled.

## In-memory deduplication

**Weak example.** A consumer remembers message IDs in a process-local set while
performing durable writes.

**Why it fails.** restart forgets identities and cannot coordinate the claim
with the effect.

**Risk.** duplicate durable mutation.

**Improved direction.** persist an inbox identity atomically with the local
effect.

**When justified.** Ephemeral best-effort work may accept duplication.

## Acknowledge then act

**Weak example.** A consumer acknowledges a command before performing a
required effect.

**Why it fails.** crash after acknowledgement loses the obligation.

**Risk.** silent missing work.

**Improved direction.** persist durable intent or complete and record the effect
before acknowledgement.

**When justified.** Lossy telemetry may explicitly prefer at-most-once
processing.

## Act then acknowledge without deduplication

**Weak example.** A consumer completes an effect, then acknowledges, with no
stable identity.

**Why it fails.** acknowledgement loss produces redelivery.

**Risk.** duplicate effect.

**Improved direction.** use idempotent effect identity or durable deduplication.

**When justified.** Repeated effect must be harmless under its complete
semantics.

## Exactly-once by branding

**Weak example.** A broker feature is cited as proof every downstream effect
happens once.

**Why it fails.** feature boundaries may cover broker state but not external
systems.

**Risk.** consumers omit duplicate protection.

**Improved direction.** state exact identity, transaction, resource, retention,
and excluded effects.

**When justified.** Boundary-scoped exactly-once terminology is acceptable with
the complete mechanism.

## Compensation as rollback

**Weak example.** A saga diagram shows an external refund arrow labeled
rollback.

**Why it fails.** refund is later, fallible, and not equivalent to no charge.

**Risk.** unresolved compensation disappears from state.

**Improved direction.** model compensation command, outcome, retry, and unknown
state independently.

**When justified.** The term rollback is appropriate inside one actual local
transaction.

## Distributed lock without fencing

**Weak example.** A lease expiry lets a new worker proceed, but the protected
resource accepts writes from the paused old worker.

**Why it fails.** ownership service and effect resource disagree about current
authority.

**Risk.** concurrent stale mutation.

**Improved direction.** attach monotonic fencing tokens that the resource
rejects when stale, or make the effect idempotent and versioned.

**When justified.** A lock may reduce duplicate low-risk work when overlap is
harmless.

## Not found means never happened

**Weak example.** The first reconciliation query returns no record and the
system declares rejection.

**Why it fails.** provider indexing or processing can lag.

**Risk.** unsafe retry.

**Improved direction.** follow provider-defined finality and retention windows;
remain unknown until evidence is definitive.

**When justified.** The provider contract may make an authoritative not-found
response final for the operation identity.

## Endless reconciler

**Weak example.** Unknown operations retry rapidly forever without age metrics
or escalation.

**Why it fails.** ownership exists only in code, not operations.

**Risk.** permanent load, cost, and invisible customer impact.

**Improved direction.** bound concurrency and attempts per cycle, back off,
measure age, and escalate terminally.

**When justified.** A durable obligation may remain pending, but execution and
visibility still need bounds.

---

## Source: `doctrines/0006-distributed-uncertainty/glossary.md`

# Glossary

**Acknowledgement ambiguity**
: Uncertainty caused when processing may have completed but acknowledgement was
lost or not durably coordinated.

**Compensation**
: A new action intended to mitigate a prior effect. It is not equivalent to
erasing history and may fail independently.

**Deduplication**
: Recognition of a previously seen logical identity to suppress repeated
processing or replay a stored result.

**Fencing token**
: A monotonically ordered authority value checked by the protected resource so
operations from stale lease holders can be rejected.

**Idempotency key**
: Stable identity used by a receiver to bind repeated attempts of one logical
operation to one scoped result.

**Logical operation**
: One domain intent, potentially represented by several transport attempts.

**Reconciliation**
: Acquisition and evaluation of authoritative evidence to resolve or continue
an unknown outcome.

**Replay horizon**
: The maximum period over which a prior message or request identity may
legitimately return.

**Saga**
: A distributed workflow of independently committed actions with explicit
follow-up or compensating actions.

**Unknown outcome**
: A state in which available evidence does not establish either execution
success or definitive rejection.

---

## Source: `doctrines/0006-distributed-uncertainty/references.md`

# References

- [RFC 9110, HTTP Semantics: idempotent methods](https://www.rfc-editor.org/rfc/rfc9110.html#name-idempotent-methods)
  defines HTTP method idempotency and explicitly scopes the concept to intended
  effect.
- [Apache Kafka design: delivery semantics](https://kafka.apache.org/documentation/#semantics)
  documents at-most-once, at-least-once, and Kafka's transaction-scoped
  exactly-once mechanisms.
- [CloudEvents specification](https://github.com/cloudevents/spec/blob/main/cloudevents/spec.md)
  provides standardized event identity and source context useful for
  correlation.
- [Stripe API: idempotent requests](https://docs.stripe.com/api/idempotent_requests)
  is an authoritative example of key retention, parameter comparison, and
  response replay semantics for one API.
- [Amazon Builders' Library: making retries safe with idempotent APIs](https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/)
  explains production API identity and retry design.
- [Gray and Cheriton, "Leases: An Efficient Fault-Tolerant Mechanism for
  Distributed File Cache Consistency"](https://dl.acm.org/doi/10.1145/74850.74870)
  is foundational literature on time-bounded distributed authority and its
  clock assumptions.
- [PostgreSQL documentation: transaction isolation](https://www.postgresql.org/docs/current/transaction-iso.html)
  grounds database observations and anomalies for relevant examples.

The doctrine adds an operational contract for explicit unknown states,
reconciliation evidence, scoped claims, audit causality, and review gates.

---

## Source: `doctrines/0007-unsafe-rust/README.md`

---
id: RUST-DOC-0007
slug: unsafe-rust
title: Unsafe Rust as a Proof Obligation
status: active
version: 0.1.1
normative: true
applies_to:
  - planning
  - implementation
  - review
  - audit
  - maintenance
risk_domains:
  - memory-safety
  - ffi
  - concurrency
  - dependency-risk
supersedes: []
superseded_by: null
---

# Unsafe Rust as a Proof Obligation

## Scope

This package governs Rust `unsafe` blocks and functions, unsafe traits and
implementations, raw pointers, foreign-function interfaces, manual allocation,
layout-dependent code, uninitialized memory, and safe APIs whose implementation
contains unsafe operations. It also governs decisions to depend on crates whose
unsafe internals materially affect the system's risk.

`unsafe` does not disable Rust's safety contract. It identifies operations for
which the compiler cannot verify all preconditions and transfers the missing
proof to authors and reviewers. A sound safe API must remain memory-safe for
every safe caller, including adversarial call sequences and panics.

## Out of scope

This doctrine is not a collection of clever pointer techniques. It does not
teach exploit development or promise that passing Miri proves universal
soundness. Platform ABI, allocator, provenance, and concurrency claims must be
grounded in the relevant primary specification and deployed target.

## Intended readers

- planners deciding whether unsafe code is justified;
- implementers isolating and documenting proof obligations;
- reviewers checking safety arguments line by line;
- auditors inventorying unsafe code and dependencies;
- maintainers responding to compiler, platform, or dependency changes.

## Normative status

[`doctrine.md`](../doctrines/0007-unsafe-rust/doctrine.md) is normative. Safety requirements cannot be waived
merely for performance or borrow-checker convenience. Any accepted exception
must still preserve Rust's safety contract and document evidence.

## Prerequisite foundations

Read invariants, evidence, trust boundaries, guarantee honesty, and complexity
budget under [`../../foundations/`](../foundations/). Unsafe work additionally
requires the Rust Reference and Rustonomicon material cited in
[`references.md`](../doctrines/0007-unsafe-rust/references.md).

## Related material

- Patterns: [opaque newtypes](../patterns/opaque-newtypes.md),
  [capability types](../patterns/capability-types.md), and
  [consuming transitions](../patterns/consuming-transitions.md).
- Boundaries: FFI and filesystem.
- Reviews: [pre-implementation](../reviews/pre-implementation.md), boundary, domain model, and
  final audit.
- Doctrines: ownership/capabilities, concurrency/async, testing/evidence, and
  performance/measurement.

## Reading order

Read the normative rules before authoring a safety comment. Use the decision
framework to challenge whether unsafe is needed. Apply every relevant review
gate, then use the anti-pattern catalogue as an adversarial second pass.

## Compact doctrine summary

Every unsafe operation needs a safety invariant and a local argument showing
why all required preconditions hold. Unsafe surface is minimized and
encapsulated. A safe public API must be sound for all safe callers. `unsafe fn`
documents caller obligations; `unsafe impl Send` or `Sync` includes a concurrency
proof. FFI defines ABI, representation, ownership, nullability, lifetime,
threading, allocator, and unwind behavior. Partial initialization accounts for
drop. Layout and provenance are never guessed. Miri, sanitizers, model checking,
fuzzing, and target testing provide complementary evidence but do not replace
reasoning.

## Executable evidence status

The workspace forbids unsafe code by default. The narrowly isolated
`unsafe-evidence` crate opts out locally to exercise a panic-safe
`MaybeUninit<[T; N]>` initializer. Its five unit tests cover success, builder
error, builder panic, an empty array, and zero-sized element drop accounting;
the dedicated CI job reruns them under Miri on a pinned nightly toolchain. The
crate documents each unsafe operation, safe-API proof, construction boundary,
and residual limits.

This evidence supports only that abstraction under the exercised interpreter
and inputs. It is not sanitizer, FFI-target, fuzzing, concurrent-unsafe, or
universal provenance evidence, and it does not replace the safety argument.

---

## Source: `doctrines/0007-unsafe-rust/doctrine.md`

# Normative doctrine

## RUST-DOC-0007-R001 — Justify the need for unsafe

**Statement.** Introduction or expansion of unsafe code MUST document the
required capability, safe alternatives considered, and why their cost or
limitations are unacceptable for the stated risk domain.

**Intent.** Prevent unsafe from becoming a convenience escape from design or
borrowing work.

**Applicability.** Every new unsafe block, function, trait implementation, or
FFI boundary.

**Allowed exceptions.** Mechanically generated binding declarations may share
one reviewed justification for a generated unit.

**Review evidence.** Required capability, safe alternatives, explicit scope,
and benchmark evidence when performance justifies the risk.

## RUST-DOC-0007-R002 — State the safety invariant

**Statement.** Every unsafe block MUST be associated with a `SAFETY:` argument
that states the relevant invariant and explains why each unsafe operation's
preconditions hold at that point.

**Intent.** Make transferred proof obligations inspectable beside the code.

**Applicability.** Explicit and compiler-required unsafe operations.

**Allowed exceptions.** Repeated operations inside one tightly bounded block may
share one complete argument when their obligations are identical.

**Review evidence.** The `SAFETY:` comment names the applicable aliasing,
validity, lifetime, alignment, provenance, initialization, concurrency, and
panic considerations.

## RUST-DOC-0007-R003 — Minimize and encapsulate unsafe

**Statement.** Unsafe operations MUST be kept in the smallest practical lexical
and API scope and encapsulated behind a safe abstraction whenever safe callers
can use the capability.

**Intent.** Reduce proof surface and prevent invariant-dependent values from
escaping unchecked.

**Applicability.** Low-level modules, FFI wrappers, containers, and optimized
algorithms.

**Allowed exceptions.** A public unsafe primitive may be appropriate when
callers must supply obligations that cannot be checked.

**Review evidence.** Unsafe inventory, module visibility, private fields, and
safe wrapper tests.

## RUST-DOC-0007-R004 — Make safe APIs sound for every safe caller

**Statement.** A safe public API implemented with unsafe code MUST uphold
memory-safety requirements for all values and call sequences constructible in
safe Rust, including reentrancy, panic, cancellation, and concurrent use allowed
by its traits.

**Intent.** Prevent hidden caller obligations from leaking through a safe
signature.

**Applicability.** All safe wrappers over unsafe internals.

**Allowed exceptions.** None.

**Review evidence.** Adversarial safe-call analysis, invariant ownership,
panic/drop paths, and executable evidence.

## RUST-DOC-0007-R005 — Document unsafe caller obligations

**Statement.** Every public or cross-module `unsafe fn` and unsafe trait MUST
have a `# Safety` section specifying complete caller obligations in testable,
non-circular terms.

**Intent.** Define exactly what the compiler no longer checks for the caller.

**Applicability.** Unsafe functions, methods, traits, and constructors.

**Allowed exceptions.** Private functions used once may state obligations at
the function or call site, but the proof chain MUST remain explicit.

**Review evidence.** Caller obligations name valid ranges, lifetime, ownership,
aliasing, initialization, thread, and provenance constraints as relevant.

## RUST-DOC-0007-R006 — Protect representation validity

**Statement.** Unsafe code MUST preserve Rust validity requirements for every
value that becomes observable as a typed value. It MUST NOT create invalid enum
discriminants, references, booleans, characters, nonzero values, or other
restricted representations.

**Intent.** Avoid undefined behavior before ordinary code can validate.

**Applicability.** Casts, reads, transmutation, FFI, serialization shortcuts,
and uninitialized memory.

**Allowed exceptions.** Bytes may remain untyped storage until validity is
established; they MUST NOT be observed through an invalid typed value.

**Review evidence.** Representation source, validation, layout reference, and
invalid-input tests.

## RUST-DOC-0007-R007 — Prove aliasing and lifetime

**Statement.** Creation or use of references from raw pointers MUST establish
non-nullness, alignment, dereferenceability, initialization, permitted aliasing,
and a lifetime no longer than the backing allocation and authority.

**Intent.** Prevent references from asserting guarantees the pointer does not
provide.

**Applicability.** Raw-pointer dereference, slices from raw parts, FFI pointers,
and self-referential structures.

**Allowed exceptions.** None; only the proof mechanism varies.

**Review evidence.** Allocation owner, mutation paths, reallocation analysis,
and borrow duration.

## RUST-DOC-0007-R008 — Respect provenance and bounds

**Statement.** Raw-pointer arithmetic and integer-pointer conversions MUST have
a documented provenance, allocation, element-bound, alignment, and one-past-end
argument consistent with the supported Rust model and target APIs.

**Intent.** Prevent address arithmetic from being treated as sufficient pointer
authority.

**Applicability.** Allocators, buffers, intrusive structures, memory maps, and
FFI.

**Allowed exceptions.** None.

**Review evidence.** Originating allocation, range proof, zero-sized-type
behavior, overflow handling, and Miri coverage where supported.

## RUST-DOC-0007-R009 — Handle partial initialization and drop

**Statement.** `MaybeUninit` and manual initialization MUST track exactly which
elements are initialized and MUST drop each initialized value exactly once on
success, error, and panic paths.

**Intent.** Prevent reads of uninitialized memory, leaks of owned resources, and
double drop.

**Applicability.** Arrays, FFI output buffers, custom collections, and
performance-sensitive construction.

**Allowed exceptions.** Trivially non-dropping byte storage still requires proof
against uninitialized typed reads.

**Review evidence.** Initialization counter or state, guard behavior, panic
injection, and destructor tests.

## RUST-DOC-0007-R010 — Require exceptional justification for transmute

**Statement.** `transmute` MUST require stronger justification than convenience:
source and destination size, alignment, validity, lifetime, ownership, and
layout compatibility MUST be established from authoritative contracts.

**Intent.** Expose the many simultaneous obligations hidden by one operation.

**Applicability.** Every transmute or equivalent bit reinterpretation.

**Allowed exceptions.** None; a narrower cast or conversion SHOULD be used when
it expresses fewer obligations.

**Review evidence.** Primary layout citation, static assertions where possible,
and tests across supported targets.

## RUST-DOC-0007-R011 — Define FFI representation and ABI

**Statement.** FFI declarations MUST specify the correct ABI and use
representations whose layout is defined for that boundary. Rust-native layout
MUST NOT be assumed stable without an applicable representation contract.

**Intent.** Prevent caller/callee disagreement about call convention and data
layout.

**Applicability.** Foreign functions, callbacks, shared structs, unions, and
opaque handles.

**Allowed exceptions.** Bindings generated from an authoritative interface may
derive declarations, but generated output and generator version remain reviewed
inputs.

**Review evidence.** Header/specification match, `repr` choice, target matrix,
and ABI tests.

## RUST-DOC-0007-R012 — Define FFI ownership and allocation

**Statement.** Every pointer crossing FFI MUST define nullability, length,
ownership transfer, lifetime, mutability, thread access, allocator of origin,
and the matching release operation.

**Intent.** Prevent double frees, leaks, allocator mismatch, and dangling
access.

**Applicability.** Buffers, strings, handles, callbacks, and allocated objects.

**Allowed exceptions.** None; an opaque handle still requires a lifecycle
contract.

**Review evidence.** Boundary table, constructor/destructor pairs, null and
length tests, and foreign-side documentation.

## RUST-DOC-0007-R013 — Control unwinding across FFI

**Statement.** Panic or foreign exception unwinding across an ABI boundary MUST
be prevented or handled according to an explicitly selected ABI and supported
runtime contract.

**Intent.** Avoid undefined behavior and uncontrolled process state.

**Applicability.** Exported Rust functions, imported callbacks, and foreign
exceptions.

**Allowed exceptions.** An unwind-capable ABI may be used only with documented
cross-language behavior and target support.

**Review evidence.** Catch/abort policy, destructor implications, and panic-path
test.

## RUST-DOC-0007-R014 — Prove unsafe `Send` and `Sync`

**Statement.** Every unsafe implementation of `Send` or `Sync` MUST state a
concurrency proof covering all contained state, aliasing, mutation,
destruction, callbacks, and foreign-library thread guarantees.

**Intent.** Ensure marker traits do not grant unsupported cross-thread
authority.

**Applicability.** Custom containers, raw handles, FFI wrappers, and
self-referential values.

**Allowed exceptions.** None.

**Review evidence.** Trait invariant, synchronization model, adverse schedule
tests, and upstream thread-safety contract.

## RUST-DOC-0007-R015 — Preserve panic safety

**Statement.** Unsafe abstractions MUST remain memory-safe if safe callbacks,
allocation, cloning, comparison, formatting, or destruction panic at any
permitted point.

**Intent.** Prevent partial mutation from violating assumptions later consumed
by unsafe code.

**Applicability.** Collections, sorting, initialization, callback-based APIs,
and guards.

**Allowed exceptions.** Logical corruption after panic may be allowed only if
memory safety remains intact and the object cannot be used as though valid.

**Review evidence.** Unwind-state analysis, guards, injected panics, and drop
accounting.

## RUST-DOC-0007-R016 — Use complementary dynamic evidence

**Statement.** Unsafe code SHOULD be exercised with Miri and relevant
sanitizers, fuzzing, model checking, or target-specific integration tests where
the tools support its behavior.

**Intent.** Detect violations that code review and ordinary tests can miss.

**Applicability.** Pointer, initialization, FFI, and concurrency code.

**Allowed exceptions.** Unsupported operations or targets may use alternative
evidence, with the limitation documented.

**Review evidence.** Exact commands, supported targets, findings resolved, and
known blind spots.

## RUST-DOC-0007-R017 — Review unsafe dependencies

**Statement.** Dependencies containing material unsafe code MUST be identified
and reviewed proportionally to reachability, privilege, input exposure,
maintenance, advisories, and substitutability.

**Intent.** Include transitive proof trust in the system risk model.

**Applicability.** FFI bindings, parsers, runtimes, allocators, cryptography, and
highly privileged libraries.

**Allowed exceptions.** Low-risk unreachable target-specific code may receive a
documented reduced review.

**Review evidence.** Dependency inventory, versions, advisory status, unsafe
surface, upstream audit evidence, and update policy.

## RUST-DOC-0007-R018 — Re-audit when assumptions change

**Statement.** Unsafe code MUST be re-reviewed when compiler behavior, target,
ABI, dependency, layout, allocation, synchronization, or surrounding safe API
assumptions change.

**Intent.** Keep proof obligations synchronized with their premises.

**Applicability.** Upgrades, ports, refactors, and feature changes.

**Allowed exceptions.** A change proven outside the unsafe dependency cone may
document that conclusion.

**Review evidence.** Assumption inventory, changed-premise analysis, repeated
dynamic evidence, and reviewer approval.

---

## Source: `doctrines/0007-unsafe-rust/rationale.md`

# Rationale

## The boundary of compiler proof

Safe Rust establishes a set of memory-safety properties for safe programs,
assuming sound compiler and library behavior. Unsafe operations are required
for capabilities such as dereferencing raw pointers or calling unsafe foreign
functions because the compiler lacks enough information to establish their
preconditions. The programmer does not receive permission to ignore those
preconditions; responsibility moves into a proof argument.

A safety comment that merely restates "this pointer is dereferenced here" adds
no evidence. A useful argument identifies where the pointer came from, why it is
aligned and in bounds, what initializes the bytes, which aliases exist, how long
the allocation lives, and why concurrent mutation cannot invalidate the access.

## Safe abstraction means adversarial safe callers

The soundness test for a safe wrapper is not whether intended callers use it
correctly. Safe callers may pass empty slices, zero-sized types, panicking
callbacks, unusual drop implementations, repeated methods, aliases allowed by
the signature, and concurrent calls allowed by `Send` or `Sync`. If one such
safe sequence causes undefined behavior, the safe abstraction is unsound.

Hidden documentation such as "do not call twice" cannot repair a safe signature.
The API must enforce the lifecycle, perform a runtime check, consume a value, or
be unsafe with explicit caller obligations.

## Lexical minimization improves proof locality

An entire function marked unsafe permits operations to be added later without a
local review cue. A narrow block shows which steps rely on external proof and
keeps parsing, range checking, and ownership setup in safe code. Encapsulation
protects private invariants so callers cannot manufacture a state that makes
the unsafe implementation invalid.

Small blocks are not automatically sound. They are useful because reviewers can
map each operation to its premises and because safe surrounding code carries
more of the argument.

## Validity precedes business validation

It is undefined behavior to create some invalid Rust typed values even if code
intends to check them immediately. Reading an arbitrary byte as `bool`, creating
a reference from null, or materializing an enum with an invalid discriminant
crosses the validity boundary before a match or condition can reject it.
Untrusted bytes should remain bytes or a representation that accepts every bit
pattern until validated.

This distinction mirrors domain construction: physical representation
validation must occur before trusted interpretation, but unsafe Rust adds
language validity obligations that cannot be repaired after violation.

## Aliasing, lifetime, and provenance

A raw numeric address does not establish ownership or provenance. A reference
asserts alignment, dereferenceability, validity, and aliasing properties for its
lifetime. Reallocation can invalidate addresses into vectors. Foreign libraries
may retain callbacks or buffers beyond the call. A slice length can overflow or
extend beyond its allocation even when the starting pointer is valid.

Proof should start from the allocation and follow custody to each use. Keeping
raw pointers raw until the shortest needed borrow often avoids claiming a long
lifetime. Foreign handles should remain opaque unless their contract explicitly
permits memory access.

## Partial initialization

`MaybeUninit<T>` avoids falsely claiming that bytes already contain a valid
`T`. It does not track which array elements are initialized or automatically
drop them on a panic. Construction code needs a progress count or guard whose
destructor drops exactly the completed prefix. Only after every element is
initialized may the buffer be converted to the complete typed value.

Leaking values may be memory-safe for some `T`, but can leak locks, file
descriptors, or secrets. Correct resource behavior remains part of the broader
contract even where language-level undefined behavior is absent.

## FFI combines several trust boundaries

FFI crosses language layout, ABI, allocator, unwind, lifetime, thread, and error
models simultaneously. `repr(C)` gives specified layout relationships for
supported field types; it does not make arbitrary Rust types portable to C.
Pointers require nullability and length conventions. Strings require encoding
and ownership. Objects allocated on one side generally require their matching
deallocator. Callbacks require a retention and threading contract.

A robust wrapper uses raw foreign types at the boundary, validates return codes
and lengths, converts to owned Rust values where practical, and exposes a safe
API only for obligations it can enforce.

## Unwinding and panic

Unsafe collection code can become unsound when a user-provided comparator or
clone panics mid-transition. Guards should ensure the object remains droppable
and no value is dropped twice. Exported Rust callbacks must prevent panic from
crossing an incompatible foreign ABI, commonly by catching it at the boundary
or using process abort according to policy. Catching panic does not guarantee
the foreign library remains logically usable; that is another contract.

## `Send` and `Sync` are authority declarations

Unsafe marker implementations tell the compiler that transfer or shared
reference across threads is safe. A raw pointer field can suppress automatic
traits, but manually restoring them requires reasoning about the pointed-to
allocation, foreign thread rules, mutation synchronization, callbacks, and
destruction. A mutex around one field does not establish the foreign handle is
thread-safe.

The proof must cover every safe method and drop, not only the operation that
motivated cross-thread use.

## Dynamic tools are evidence, not universal proof

Miri can detect many undefined behaviors in executions it explores under its
supported model. Sanitizers can detect target executions involving address,
thread, or memory errors. Fuzzing explores input space; Loom explores schedules
for modeled synchronization. None explores all code, inputs, compilers,
platforms, or foreign components.

Use several forms of evidence and record their limits. A clean run raises
confidence; it does not replace the safety argument.

## Performance is not presumed

Unsafe code often claims to remove checks, copies, or synchronization. The
change must be measured under a defined workload and must preserve correctness.
Compiler optimizations can make clear safe code equivalent. A slightly slower
safe implementation may be the correct complexity-budget choice when proof and
maintenance costs dominate.

## Guarantee ledger

| Claim                                   | Established by                    | Protected construction        | Boundary preservation                 | Escape hatches         | Does not prove                    | Residual runtime risk            |
| --------------------------------------- | --------------------------------- | ----------------------------- | ------------------------------------- | ---------------------- | --------------------------------- | -------------------------------- |
| slice references initialized allocation | bounds, alignment, lifetime proof | private wrapper               | raw input checked before slice        | unsafe internal helper | business validity of bytes        | allocator or FFI contract breach |
| FFI handle is released once             | ownership wrapper and `Drop`      | private field                 | constructor accepts only owned handle | raw binding layer      | remote resource cleanup succeeded | foreign destructor failure       |
| wrapper is safe to transfer             | complete `Send` proof             | no aliasing escape            | foreign thread contract checked       | direct bindings        | external library bug              | upstream version change          |
| array is fully initialized              | progress guard then conversion    | `MaybeUninit` remains private | errors drop initialized prefix        | manual raw access      | element semantic validity         | panicking foreign destructor     |

## Cost of overapplication

Avoiding unsafe at any cost can create excessive copying or make necessary FFI
impossible. The doctrine permits unsafe where capability and evidence justify
it. It rejects prestige, convenience, and unmeasured optimization as proof.
Centralizing a small reviewed kernel often produces a simpler whole system than
spreading workarounds or unchecked foreign assumptions.

---

## Source: `doctrines/0007-unsafe-rust/decision-framework.md`

# Decision framework

## Establish necessity

Ask:

1. What capability is unavailable in safe Rust?
2. Can ownership, borrowing, an enum, a checked index, or a maintained safe
   dependency supply it?
3. Is performance the reason, and is the bottleneck measured?
4. Can unsafe remain in one private module?
5. Who can review the relevant aliasing, ABI, or concurrency model?
6. Which targets and toolchains must be supported?
7. What happens when a premise changes?

If the need is only to silence a borrow error, redesign ownership first.

## Build the proof table

For every unsafe operation record:

| Obligation            | Evidence                                        |
| --------------------- | ----------------------------------------------- |
| allocation/provenance | originating allocation or foreign contract      |
| bounds                | checked range and overflow handling             |
| alignment             | type/layout contract or runtime check           |
| initialization        | construction state and exact initialized region |
| validity              | bit-pattern validation before typed observation |
| aliasing              | all references and mutation authority           |
| lifetime              | owner and destruction order                     |
| concurrency           | synchronization and thread contract             |
| panic/drop            | every partial state and destructor path         |
| target/ABI            | supported platforms and primary specification   |

Any unanswered applicable row blocks implementation.

## Choose the API boundary

```text
Can all safety preconditions be checked or enforced internally?
├─ yes → safe API over private unsafe implementation
└─ no
   Can ownership/type structure encode them?
   ├─ yes → redesign until safe
   └─ no → narrow unsafe API with complete caller obligations
```

Do not make an API safe by moving obligations into prose.

## FFI decision sequence

1. Generate or verify declarations against the authoritative headers.
2. Define ABI and target matrix.
3. Use stable boundary representations and opaque handles.
4. Record pointer nullability, length units, mutability, and ownership.
5. Pair every allocation with the correct deallocator.
6. Define string encoding and interior-null behavior.
7. Define callback retention, thread, and reentrancy.
8. translate foreign errors without losing categories;
9. prevent incompatible unwinding;
10. expose a safe wrapper only after obligations are enforced.

## Initialization decision

Prefer ordinary safe initialization. Use `MaybeUninit` only when required by
FFI, array construction, or measured cost. Track initialized elements with a
guard. Review zero-length, zero-sized, allocation-failure, element-constructor
failure, and panic paths. Convert to initialized `T` only once.

## Concurrency decision

Before unsafe `Send` or `Sync`, enumerate:

- all fields, raw targets, and aliases;
- allowed methods through shared and exclusive access;
- mutation synchronization;
- callback threads and reentrancy;
- destruction concurrency;
- foreign library guarantees;
- thread-local or affinity requirements;
- cancellation and panic effects.

If any upstream thread guarantee is missing, keep the wrapper non-`Send` or
non-`Sync`.

## Evidence matrix

| Risk                          | Useful evidence                            |
| ----------------------------- | ------------------------------------------ |
| pointer validity and aliasing | Miri, fuzzing, targeted tests              |
| address and bounds defects    | AddressSanitizer                           |
| data races                    | ThreadSanitizer, Loom for modeled code     |
| uninitialized reads           | MemorySanitizer where supported, Miri      |
| FFI layout                    | bindgen/layout tests, C-side assertions    |
| panic safety                  | injected panics and drop counters          |
| target assumptions            | cross-target CI or hardware tests          |
| performance justification     | benchmark and profiler under RUST-DOC-0009 |

Tool limitations must be recorded.

## Stop conditions

Stop when:

- safety explanation repeats syntax rather than premises;
- a safe caller must obey an undocumented rule;
- layout is inferred from current observation;
- a raw address is treated as provenance;
- `MaybeUninit` has no partial-drop plan;
- transmute replaces a narrower conversion;
- FFI ownership or allocator is unspecified;
- callback lifetime is guessed;
- unsafe `Send`/`Sync` lacks an upstream thread contract;
- performance improvement is unmeasured;
- reviewers cannot explain the proof.

---

## Source: `doctrines/0007-unsafe-rust/review-standard.md`

# Review standard

Mark each gate **pass**, **fail**, **not applicable**, or with an approved
**waiver reference**. Safety-contract failures cannot be waived into soundness.

| Gate | Question                                        | Pass evidence                       | Failure example                         | Severity | Remediation               |
| ---- | ----------------------------------------------- | ----------------------------------- | --------------------------------------- | -------- | ------------------------- |
| U01  | Is unsafe necessary?                            | safe alternatives and measured need | borrow checker bypass                   | critical | redesign safely           |
| U02  | Is unsafe inventory complete?                   | tool/search inventory               | macro-generated unsafe missed           | critical | enumerate                 |
| U03  | Is lexical scope minimal?                       | small block                         | whole function marked unsafe            | high     | narrow block              |
| U04  | Is API visibility minimal?                      | private module/helper               | raw constructor public                  | critical | encapsulate               |
| U05  | Does every block state invariant?               | `SAFETY:` argument                  | "pointer seems valid"                   | critical | write proof               |
| U06  | Does comment cover each operation?              | operation-to-premise mapping        | one generic comment                     | critical | split or expand           |
| U07  | Are safe callers adversarially considered?      | call-sequence analysis              | intended use only                       | critical | test full safe surface    |
| U08  | Are hidden caller obligations absent?           | signature enforces rules            | safe method says "must not call twice"  | critical | encode/check/mark unsafe  |
| U09  | Does unsafe API have `# Safety` docs?           | complete section                    | caller obligations omitted              | critical | document                  |
| U10  | Are obligations non-circular?                   | concrete predicates                 | "call only when safe"                   | critical | specify facts             |
| U11  | Is pointer origin known?                        | allocation/foreign provenance       | integer address guessed                 | critical | trace origin              |
| U12  | Is nullability checked?                         | check or non-null contract          | dereference nullable result             | critical | validate                  |
| U13  | Is alignment established?                       | layout or runtime check             | byte offset cast blindly                | critical | align/copy                |
| U14  | Are bounds and overflow checked?                | checked arithmetic                  | length multiplication wraps             | critical | checked operations        |
| U15  | Is dereferenceability established?              | live allocation range               | pointer only numerically in range       | critical | prove allocation          |
| U16  | Is initialization tracked?                      | progress state                      | assume-init before complete             | critical | guard                     |
| U17  | Is typed validity established?                  | bit-pattern validation              | arbitrary byte as bool                  | critical | remain untyped            |
| U18  | Are enum discriminants valid?                   | stable conversion                   | transmute integer to enum               | critical | checked match             |
| U19  | Is aliasing permitted?                          | reference graph                     | mutable and shared references overlap   | critical | shorten/restructure       |
| U20  | Is lifetime bounded by owner?                   | custody proof                       | forged static reference                 | critical | return owned/short borrow |
| U21  | Is reallocation considered?                     | capacity/pinning proof              | reference held across vector push       | critical | avoid movement            |
| U22  | Are zero-sized types handled?                   | explicit case                       | pointer increment assumes size          | high     | account for ZST           |
| U23  | Is one-past-end use valid?                      | arithmetic proof                    | dereference end pointer                 | critical | correct bounds            |
| U24  | Are integer-pointer conversions justified?      | supported provenance API            | round-trip assumed universally valid    | critical | use supported operations  |
| U25  | Is every initialized value dropped once?        | guard and counters                  | panic leaks/double drops                | critical | track prefix              |
| U26  | Is error cleanup sound?                         | failure tests                       | partial FFI output leaked               | critical | cleanup guard             |
| U27  | Is panic cleanup sound?                         | injected panic                      | callback panic corrupts collection      | critical | use repair guard          |
| U28  | Is transmute unavoidable?                       | narrower alternatives rejected      | convenience cast                        | critical | replace                   |
| U29  | Are transmute sizes/layouts proven?             | primary contract/assertion          | current compiler observation            | critical | establish or remove       |
| U30  | Is ownership preserved across bit cast?         | drop analysis                       | duplicated owned pointer                | critical | use safe conversion       |
| U31  | Is FFI ABI exact?                               | header/spec match                   | default ABI assumed                     | critical | correct declaration       |
| U32  | Is representation stable?                       | applicable `repr` contract          | Rust layout exported                    | critical | boundary type             |
| U33  | Are foreign lengths in correct units?           | bytes/elements contract             | byte length used as elements            | critical | convert checked           |
| U34  | Is FFI ownership explicit?                      | boundary table                      | unclear who frees                       | critical | define lifecycle          |
| U35  | Is allocator pairing correct?                   | matching free function              | Rust frees C allocation                 | critical | return to origin          |
| U36  | Is string encoding explicit?                    | conversion policy                   | UTF-8 assumed from C                    | high     | validate                  |
| U37  | Are callbacks lifetime-safe?                    | registration/unregistration proof   | stack context retained                  | critical | own context               |
| U38  | Are callback threads known?                     | foreign contract                    | thread-affine state accessed anywhere   | critical | marshal/synchronize       |
| U39  | Is reentrancy handled?                          | state-machine analysis              | callback reenters mutable borrow        | critical | guard/design              |
| U40  | Is unwind policy explicit?                      | catch/abort/ABI contract            | panic crosses C ABI                     | critical | contain unwind            |
| U41  | Is foreign exception behavior known?            | source contract                     | exception crosses Rust unknowingly      | critical | wrapper boundary          |
| U42  | Does unsafe `Send` cover all fields?            | concurrency proof                   | raw pointer ignored                     | critical | prove or remove           |
| U43  | Does unsafe `Sync` cover shared methods?        | alias/mutation proof                | foreign handle not thread-safe          | critical | restrict                  |
| U44  | Is drop thread behavior valid?                  | destruction contract                | must free on creator thread             | critical | enforce affinity          |
| U45  | Are atomics ordered by invariant?               | happens-before proof                | folklore ordering                       | critical | prove/use lock            |
| U46  | Does safe abstraction remain sound after panic? | state and drop evidence             | poison ignored                          | critical | isolate invalid state     |
| U47  | Has Miri run where supported?                   | command/result                      | no dynamic UB evidence                  | high     | run or explain            |
| U48  | Have relevant sanitizers run?                   | target results                      | concurrency code untested               | high     | add evidence              |
| U49  | Is fuzzing aimed at boundary invariants?        | corpus/property                     | only fixed examples                     | medium   | fuzz                      |
| U50  | Are tool blind spots recorded?                  | evidence limits                     | clean run called proof                  | high     | qualify                   |
| U51  | Are unsafe dependencies inventoried?            | dependency audit                    | transitive FFI crate ignored            | high     | review                    |
| U52  | Are advisories and maintenance current?         | audit evidence                      | abandoned critical crate                | high     | update/replace            |
| U53  | Are target assumptions tested?                  | target matrix                       | only developer architecture             | high     | cross-test                |
| U54  | Is performance justification measured?          | benchmark/profile                   | "faster" assertion                      | high     | measure                   |
| U55  | Is re-audit trigger documented?                 | assumption list                     | compiler upgrade ignored                | high     | define trigger            |
| U56  | Does guarantee ledger state non-guarantees?     | completed ledger                    | safe wrapper claims foreign correctness | critical | narrow claim              |

Approval requires a reviewer competent in the relevant unsafe domain. Tool
success cannot compensate for an incomplete safety argument.

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0007-R001`, `RUST-DOC-0007-R002`, `RUST-DOC-0007-R003`, `RUST-DOC-0007-R004`
- `RUST-DOC-0007-R005`, `RUST-DOC-0007-R006`, `RUST-DOC-0007-R007`, `RUST-DOC-0007-R008`
- `RUST-DOC-0007-R009`, `RUST-DOC-0007-R010`, `RUST-DOC-0007-R011`, `RUST-DOC-0007-R012`
- `RUST-DOC-0007-R013`, `RUST-DOC-0007-R014`, `RUST-DOC-0007-R015`, `RUST-DOC-0007-R016`
- `RUST-DOC-0007-R017`, `RUST-DOC-0007-R018`

---

## Source: `doctrines/0007-unsafe-rust/anti-patterns.md`

# Anti-pattern catalogue

## Unsafe to satisfy the borrow checker

**Weak example.** A lifetime is extended or alias created because restructuring
ownership is inconvenient.

**Why it fails.** The borrow error often identifies a real custody ambiguity.

**Risk.** dangling reference or aliasing violation.

**Improved direction.** change ownership, shorten borrows, use indices or owned
values, or encode lifecycle.

**When justified.** Self-referential or intrusive structures may require unsafe
after pinning and movement proofs are complete.

## "It seems safe"

**Weak example.** A safety comment expresses confidence without facts.

**Why it fails.** Reviewers cannot connect operation preconditions to evidence.

**Risk.** undocumented undefined behavior.

**Improved direction.** enumerate allocation, bounds, alignment,
initialization, validity, aliasing, lifetime, and concurrency premises.

**When justified.** Never as the complete argument.

## Syntax narration

**Weak example.** `SAFETY: dereferencing the pointer here`.

**Why it fails.** It says what happens, not why the operation is permitted.

**Risk.** false review signal.

**Improved direction.** state pointer origin, live allocation, checked range,
alignment, initialization, and aliases.

**When justified.** Syntax may orient a longer proof but cannot replace it.

## Whole unsafe function

**Weak example.** A large implementation is marked unsafe so any operation can
be used inside.

**Why it fails.** Safe computations and proof-requiring steps become
indistinguishable.

**Risk.** later edits add unchecked operations.

**Improved direction.** keep the function safe where possible and use narrow
unsafe blocks.

**When justified.** Caller-facing unsafe remains necessary when callers supply
uncheckable premises, but internals should still localize operations.

## Forge `'static`

**Weak example.** A borrowed reference is transmuted to `'static` because an
async task needs it.

**Why it fails.** type lifetime outlives the allocation contract.

**Risk.** use after free.

**Improved direction.** move owned data, scope the task, or redesign storage.

**When justified.** A genuinely process-lifetime allocation can produce a
long-lived reference through a direct ownership argument, usually without
transmute.

## Transmute an enum

**Weak example.** An integer from FFI is transmuted into a Rust enum.

**Why it fails.** unknown discriminants can be invalid typed values before
matching.

**Risk.** immediate undefined behavior.

**Improved direction.** match accepted integers into variants and retain an
unknown/error path.

**When justified.** No convenience justification makes arbitrary input valid.

## Assume observed layout

**Weak example.** A test checks current size and the code relies on unannotated
Rust field order.

**Why it fails.** observed compiler output is not a stable layout contract.

**Risk.** corruption on compiler, target, or optimization changes.

**Improved direction.** use an applicable `repr`, explicit boundary fields, and
primary ABI definitions.

**When justified.** An assertion reinforces a defined contract; it does not
create one.

## Reference across reallocation

**Weak example.** A pointer into a vector becomes a reference, then the vector
may grow.

**Why it fails.** growth can move the allocation.

**Risk.** dangling access.

**Improved direction.** use indices, reserve under proven bounds, stable
allocation, or pinning appropriate to the structure.

**When justified.** The allocation must be proven immovable for the complete
reference lifetime.

## `assume_init` after partial success

**Weak example.** An array is treated as initialized after a loop that can exit
early.

**Why it fails.** some elements may contain no valid `T`.

**Risk.** invalid reads and incorrect drop.

**Improved direction.** track initialized prefix with a cleanup guard and
convert only after completion.

**When justified.** Only when every element's initialization is proven.

## Manual `Send` because mutex

**Weak example.** A raw foreign handle gets unsafe `Send` because the wrapper
contains a mutex.

**Why it fails.** the library may require thread affinity, callbacks may race,
and drop may occur on the wrong thread.

**Risk.** foreign corruption and races.

**Improved direction.** establish the full foreign thread contract or keep
ownership on one task/thread.

**When justified.** The mutex participates in a complete proof covering all
operations and destruction.

## Catch panic and continue blindly

**Weak example.** FFI wrapper catches a panic and reports an error while leaving
foreign and Rust state unexamined.

**Why it fails.** containment of unwind does not prove logical state remains
usable.

**Risk.** later unsafe assumptions consume damaged state.

**Improved direction.** mark handle poisoned, rebuild, or terminate according to
the state invariant.

**When justified.** Continue only when cleanup guarantees a valid recoverable
state.

## Miri passed, therefore sound

**Weak example.** A clean test run replaces safety reasoning.

**Why it fails.** only executed paths and supported operations were checked.

**Risk.** untested targets or inputs retain undefined behavior.

**Improved direction.** combine proof, Miri, sanitizers, fuzzing, target tests,
and review.

**When justified.** Miri is strong supporting evidence, never universal proof.

---

## Source: `doctrines/0007-unsafe-rust/glossary.md`

# Glossary

**Aliasing invariant**
: The set of permitted simultaneous references and mutation paths for one
memory region.

**Fencing**
: Prevention of stale concurrent authority, commonly through a monotonically
checked token.

**Foreign-function interface**
: A boundary where Rust calls or is called by code governed by another ABI,
layout, allocation, error, or unwind model.

**Initialized**
: Containing a valid value for the relevant typed interpretation, not merely
allocated bytes.

**Provenance**
: The allocation and authority history associated with a pointer, beyond its
numeric address.

**Safe abstraction**
: An API whose safe callers cannot cause undefined behavior, even though its
private implementation uses unsafe operations.

**Safety invariant**
: A condition that must hold whenever unsafe code relies on it to satisfy
language or library preconditions.

**Soundness**
: Preservation of Rust's safety contract for all behavior available through a
safe interface.

**Validity**
: Requirements a bit pattern must satisfy to be observed as a particular Rust
type.

**Unwinding**
: Stack traversal caused by panic or a foreign exception, including destructor
execution across frames.

---

## Source: `doctrines/0007-unsafe-rust/references.md`

# References

- [The Rust Reference: unsafety](https://doc.rust-lang.org/reference/unsafe-keyword.html)
  defines unsafe functions, blocks, traits, and implementations.
- [The Rust Reference: behavior considered undefined](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
  provides the language's non-exhaustive undefined-behavior contract.
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) is the official book
  for unsafe Rust topics including aliasing, ownership, FFI, and concurrency.
- [`std::mem::MaybeUninit`](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html)
  documents initialization invariants and common patterns.
- [Rust standard library pointer module](https://doc.rust-lang.org/std/ptr/index.html)
  documents raw-pointer operations and safety requirements.
- [The Rust Reference: type layout](https://doc.rust-lang.org/reference/type-layout.html)
  defines representation guarantees and their limits.
- [The Rust Reference: function ABI](https://doc.rust-lang.org/reference/items/functions.html#extern-function-qualifier)
  documents external function qualifiers and ABI strings.
- [Miri](https://github.com/rust-lang/miri) is the Rust project's interpreter
  for detecting many undefined-behavior violations in executed code.
- [Rust Sanitizers](https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html)
  documents compiler sanitizer support and target limitations.

The doctrine adds repository requirements for necessity, local proof comments,
adversarial safe-call review, dependency inventory, evidence composition, and
re-audit triggers.

---

## Source: `doctrines/0008-testing-and-evidence/README.md`

---
id: RUST-DOC-0008
slug: testing-and-evidence
title: Testing as Layered Evidence
status: active
version: 0.1.1
normative: true
applies_to:
  - planning
  - implementation
  - review
  - audit
  - maintenance
risk_domains:
  - testing
  - verification
  - concurrency
  - distributed-systems
supersedes: []
superseded_by: null
---

# Testing as Layered Evidence

## Scope

This package governs how tests and related verification evidence support Rust
engineering claims. It covers compiler rejection, type checking, unit tests,
property tests, compile-fail tests, integration and contract tests, concurrency
testing, fault injection, model checking, unsafe-code tools, coverage,
snapshots, benchmarks, production telemetry, and incident evidence.

Tests are observations under defined conditions. Different classes expose
different failures and leave different blind spots. A constructor rejection
test supports a value invariant; it does not prove all integrations use the
constructor. A compile-fail test supports an API prohibition; it does not prove
runtime network behavior. A production metric observes deployed workloads; it
does not automatically reveal silent corruption.

## Out of scope

This doctrine does not mandate one testing framework or coverage percentage. It
does not treat benchmarks as correctness tests or passing tools as a universal
proof. Performance measurement belongs to RUST-DOC-0009, while unsafe-specific
proof obligations belong to RUST-DOC-0007.

## Intended readers

- planners producing an invariant-linked evidence plan;
- implementers creating positive, negative, and fault-path tests;
- reviewers assessing whether evidence matches claims;
- auditors locating untested bypasses and suppressed failures;
- maintainers evolving fixtures, snapshots, and compiler diagnostics.

## Normative status

[`doctrine.md`](../doctrines/0008-testing-and-evidence/doctrine.md) is normative. Rules require proportionate evidence,
not mechanical use of every test class. Waivers state the uncovered risk,
alternative evidence, owner, and review date.

## Prerequisite foundations

Read [`../../foundations/evidence.md`](../foundations/evidence.md),
[`../../foundations/invariants.md`](../foundations/invariants.md), and
[`../../foundations/guarantee-honesty.md`](../foundations/guarantee-honesty.md).
Evidence strength depends on the claim, scope, environment, and completeness of
the observed set.

## Related material

- Patterns: every pattern's testing-evidence section.
- Boundaries: each boundary guide's positive and adversarial cases.
- Reviews: all operational checklists, especially [final correctness](../reviews/final-correctness-audit.md) audit.
- Case studies: guarantee ledgers connect design claims to evidence.

## Reading order

Read the normative rules, then the hierarchy in the rationale. Use the decision
framework to map invariants and failure risks to test classes. Apply the review
standard before updating snapshots, compiler diagnostics, or flaky-test policy.

## Compact doctrine summary

Every consequential test traces to an invariant or failure risk. Constructor
tests include rejection. Property tests cover generative or algebraic spaces
where examples are weak. Compile-fail tests preserve important prohibited
programs. Integration and contract tests cross real boundaries where feasible.
Concurrency evidence controls schedules; distributed evidence injects duplicate,
delay, reordering, partial failure, and unknown outcomes. Test doubles preserve
critical failure semantics. Snapshots receive semantic review. Flakiness is
evidence of uncontrolled behavior, not noise to retry away. Coverage supports
gap discovery but does not replace invariant coverage.

## Executable evidence status

The workspace includes positive and negative unit tests, checked boundary
conversion, deterministic generator tests, compiler-rejection cases through
`trybuild`, and a dedicated Miri run for the isolated unsafe example. It does
not include property-based generation, fault injection, schedule exploration,
contract testing against a deployed service, sanitizers, fuzzing, or production
telemetry. Those classes remain conditional tools whose value depends on the
claim; the existing suite establishes only the behaviors it executes.

---

## Source: `doctrines/0008-testing-and-evidence/doctrine.md`

# Normative doctrine

## RUST-DOC-0008-R001 — Trace tests to invariants and risks

**Statement.** Tests MUST identify the invariant, contract, failure mode, or
regression risk they support.

**Intent.** Make suites evidence-oriented rather than collections of incidental
examples.

**Applicability.** All canonical tests and verification jobs.

**Allowed exceptions.** A compact regression test may reference an issue,
incident, or neighboring test module rather than repeat the full invariant.

**Review evidence.** Names, documentation, or manifest mapping from claim to
test.

## RUST-DOC-0008-R002 — Test constructor acceptance and rejection

**Statement.** Validated constructors MUST have positive and negative tests at
meaningful boundaries, including normalization and error categories.

**Intent.** Demonstrate both admitted and excluded value sets.

**Applicability.** Parsers, smart constructors, newtypes, collections, and
configuration.

**Allowed exceptions.** A constructor delegated entirely to a separately tested
primitive may cite that evidence and test its integration.

**Review evidence.** Boundary-value table and assertions on structured errors.

## RUST-DOC-0008-R003 — Use properties for generative invariants

**Statement.** Property-based tests SHOULD cover algebraic, round-trip,
ordering, normalization, parser, and collection invariants when a small list of
examples leaves substantial input space.

**Intent.** Explore classes of inputs and produce minimized counterexamples.

**Applicability.** Serialization, arithmetic, state-machine commands, parsers,
and collection operations.

**Allowed exceptions.** Exhaustive finite domains or directly proven simple
functions may use table tests.

**Review evidence.** Generator domain, shrinking behavior, seed retention, and
property statement.

## RUST-DOC-0008-R004 — Prove prohibited programs where valuable

**Statement.** Compile-fail tests SHOULD preserve important API prohibitions
whose guarantee depends on privacy, ownership, traits, or typestate.

**Intent.** Detect accidental widening of legal programs.

**Applicability.** Trusted construction, capability forgery, consumed handles,
state-specific operations, and trait bounds.

**Allowed exceptions.** Fragile diagnostics may be avoided when a stable API
surface check or compile test provides clearer evidence.

**Review evidence.** Minimal failing programs and reviewed compiler diagnostics.

## RUST-DOC-0008-R005 — Inspect compiler-diagnostic changes

**Statement.** Committed compile-fail `.stderr` or equivalent evidence MUST NOT
be rewritten mechanically without reviewing whether the prohibited program
still fails for the intended reason.

**Intent.** Prevent snapshot acceptance from hiding weakened construction or
transition rules.

**Applicability.** UI test suites implemented with `trybuild` or equivalent harnesses.

**Allowed exceptions.** Pure path, line, or diagnostic wording changes may be
accepted after semantic inspection.

**Review evidence.** Diff review and assertion that the intended error remains.

## RUST-DOC-0008-R006 — Cross real boundaries

**Statement.** Integration tests SHOULD cross the real parser, protocol,
database, filesystem, or process boundary when practical and consequential.

**Intent.** Exercise adapters and assumptions that unit tests omit.

**Applicability.** Boundary conversions and external integrations.

**Allowed exceptions.** Unavailable or costly systems may use faithful
emulators plus scheduled real-system evidence, with gaps documented.

**Review evidence.** Environment description, real components, setup isolation,
and cleanup.

## RUST-DOC-0008-R007 — Protect protocol contracts

**Statement.** Contract tests SHOULD verify request and response schemas,
semantic categories, compatibility, idempotency, versioning, and unknown-value
behavior relied on across independently deployed components.

**Intent.** Detect integration drift before deployment.

**Applicability.** HTTP/RPC, messages, FFI, durable events, and public libraries.

**Allowed exceptions.** One jointly released private component may rely on
end-to-end integration evidence when independent compatibility is irrelevant.

**Review evidence.** Provider/consumer contract, version matrix, and failure
fixtures.

## RUST-DOC-0008-R008 — Control concurrency evidence

**Statement.** Concurrency tests MUST use explicit synchronization, schedule
control, model checking, or observable events rather than sleeps as the primary
means of establishing an interleaving.

**Intent.** Avoid flaky timing guesses and unexercised schedules.

**Applicability.** Locks, channels, atomics, cancellation, and shutdown.

**Allowed exceptions.** A sleep may enforce an outer deadline but MUST NOT be
the evidence that an ordering occurred.

**Review evidence.** Barriers, controlled clock, Loom model, event trace, or
equivalent mechanism.

## RUST-DOC-0008-R009 — Test cancellation and cleanup

**Statement.** Async and concurrent operations MUST test cancellation at
consequential suspension points and verify resource, partial-state, and
external-outcome handling.

**Intent.** Exercise future-drop control flow.

**Applicability.** Partial writes, permits, transactions, external calls, and
task supervision.

**Allowed exceptions.** Pure cancellation-safe reads may share representative
evidence when the reasoning applies identically.

**Review evidence.** Controlled cancellation and postcondition assertions.

## RUST-DOC-0008-R010 — Inject partial failure

**Statement.** Fault-injection tests SHOULD exercise failures before, during,
and after durable or external steps in proportion to consequence.

**Intent.** Verify recovery rather than only returned errors.

**Applicability.** Persistence, messaging, payments, filesystems, and
multi-stage operations.

**Allowed exceptions.** Low-risk pure transformations may not need fault
injection.

**Review evidence.** Crash-point matrix, injected faults, resulting state, and
recovery.

## RUST-DOC-0008-R011 — Exercise distributed uncertainty

**Statement.** Distributed tests MUST exercise duplicate, delay, reordering,
lost acknowledgement, retry, and unknown outcomes when the production protocol
permits them.

**Intent.** Prevent perfect-network doubles from defining false behavior.

**Applicability.** Brokers, remote APIs, reconcilers, and distributed workflows.

**Allowed exceptions.** A protocol may exclude a scenario only with
authoritative evidence.

**Review evidence.** Scenario matrix and explicit terminal or unknown states.

## RUST-DOC-0008-R012 — Preserve failure modes in test doubles

**Statement.** Test doubles MUST NOT erase failure categories, cancellation,
latency, capacity, ordering, duplicate, or uncertainty behavior that is
material to the tested claim.

**Intent.** Keep tests faithful to the risk being evaluated.

**Applicability.** Mocks, fakes, emulators, in-memory repositories, and clocks.

**Allowed exceptions.** A narrow unit test may use a simpler double when the
omitted behavior is outside its claim and covered elsewhere.

**Review evidence.** Double-to-real contract comparison and gap ownership.

## RUST-DOC-0008-R013 — Review snapshots semantically

**Statement.** Snapshot changes MUST be reviewed as semantic output changes.
Bulk acceptance MUST NOT replace explanation of why each affected behavior is
correct.

**Intent.** Prevent expected-output updates from blessing regressions.

**Applicability.** Serialized output, diagnostics, UI, plans, and compiler UI
tests.

**Allowed exceptions.** Deterministic formatting-only migrations may group
equivalent changes with one documented rationale.

**Review evidence.** Focused diff, invariant impact, and reviewer sign-off.

## RUST-DOC-0008-R014 — Treat flakiness as evidence

**Statement.** A flaky test MUST be investigated as evidence of uncontrolled
time, state, environment, scheduling, isolation, or product behavior. Retries
MUST NOT be the sole resolution.

**Intent.** Prevent nondeterminism from being normalized.

**Applicability.** All test and benchmark automation.

**Allowed exceptions.** A temporary bounded retry may gather diagnostics while
the issue is owned and visible.

**Review evidence.** Failure signatures, root cause, deterministic fix, or
time-bounded quarantine with owner.

## RUST-DOC-0008-R015 — Do not substitute coverage for invariant evidence

**Statement.** Coverage percentages MUST NOT be used as the sole claim that
behavior or invariants are adequately tested.

**Intent.** Distinguish executed lines from asserted semantics and input space.

**Applicability.** Coverage gates and quality reports.

**Allowed exceptions.** Coverage may serve as a supplemental regression and gap
discovery metric.

**Review evidence.** Invariant-to-evidence matrix in addition to coverage.

## RUST-DOC-0008-R016 — Separate benchmarks from correctness

**Statement.** Benchmarks MUST NOT substitute for correctness tests, and
correctness assertions inside benchmark setup MUST remain independently
executable where feasible.

**Intent.** Prevent performance samples from becoming weak semantic evidence.

**Applicability.** Microbenchmarks, load tests, and profiling harnesses.

**Allowed exceptions.** A benchmark may validate setup defensively, but the
invariant still needs appropriate tests.

**Review evidence.** Corresponding correctness suite and benchmark methodology.

## RUST-DOC-0008-R017 — Use model checking proportionally

**Statement.** Small consequential concurrent protocols SHOULD be considered
for Loom or equivalent model checking, with the model's abstraction and bounds
documented.

**Intent.** Explore scheduler interleavings ordinary runs rarely reach.

**Applicability.** Atomics, locks, channels, once initialization, and ownership
handoff.

**Allowed exceptions.** Unsupported primitives or state explosion may use a
simplified model plus stress and reasoning.

**Review evidence.** Modeled invariant, bounds, results, and mismatch from
production code.

## RUST-DOC-0008-R018 — Exercise unsafe code with specialized tools

**Statement.** Unsafe code SHOULD run under Miri and relevant sanitizers,
fuzzing, or target-specific tests as required by RUST-DOC-0007.

**Intent.** Add dynamic evidence for memory-model and boundary violations.

**Applicability.** Unsafe internals and FFI wrappers.

**Allowed exceptions.** Tool incompatibility must be documented with
alternative evidence.

**Review evidence.** Commands, results, supported targets, and blind spots.

## RUST-DOC-0008-R019 — Use production evidence carefully

**Statement.** Production telemetry and incidents SHOULD refine tests and risk
models, but MUST NOT be treated as proof that unobserved failures cannot occur.

**Intent.** Learn from real workloads without confusing absence of observation
with absence of defects.

**Applicability.** Operational services and libraries with field data.

**Allowed exceptions.** None for universal claims.

**Review evidence.** Telemetry coverage, detection limits, incident-derived
regressions, and residual uncertainty.

## RUST-DOC-0008-R020 — Keep tests deterministic and isolated

**Statement.** Tests MUST control or uniquely scope mutable external state,
clocks, randomness, ports, files, and environment variables required for their
claim.

**Intent.** Make failures reproducible and parallel execution safe.

**Applicability.** Workspace tests and CI.

**Allowed exceptions.** Deliberate randomized or stress tests may vary inputs
but MUST record reproducible seeds and isolate effects.

**Review evidence.** Temporary resource strategy, seed capture, controlled
clock, and parallel-run results.

## RUST-DOC-0008-R021 — State evidence limits

**Statement.** Every consequential evidence plan MUST state what each selected
test class proves, what it does not prove, and which risks remain observed only
in production or external systems.

**Intent.** Preserve guarantee honesty.

**Applicability.** Feature plans, reviews, and release audits.

**Allowed exceptions.** Trivial local changes may reference an existing suite
contract.

**Review evidence.** Evidence ledger tied to invariant inventory.

---

## Source: `doctrines/0008-testing-and-evidence/rationale.md`

# Rationale

## Evidence hierarchy

The following layers are complementary rather than a strict ranking:

| Evidence             | Supports                                                        | Does not establish                                 |
| -------------------- | --------------------------------------------------------------- | -------------------------------------------------- |
| compiler rejection   | a specific program cannot type-check under tested API/toolchain | runtime correctness or all prohibited programs     |
| type checking        | accepted code satisfies language and trait constraints          | domain truth or external behavior                  |
| unit test            | local behavior for selected inputs                              | boundary integration or full input space           |
| property test        | a property over generated cases                                 | mathematical universality outside generation/model |
| compile-fail test    | important misuse remains rejected                               | runtime failure handling                           |
| integration test     | behavior across instantiated components                         | every deployment or failure                        |
| contract test        | agreed protocol examples and compatibility                      | provider implementation correctness everywhere     |
| fault injection      | recovery at selected failure points                             | all timing and correlated failures                 |
| model checking       | modeled schedules within stated bounds                          | unmodeled code, inputs, or unbounded executions    |
| production telemetry | observed deployed behavior                                      | invisible failures or workloads not seen           |
| incident evidence    | a real failure mechanism and consequence                        | absence of other mechanisms                        |

Tests become persuasive when their scope matches the claim and independent
layers agree.

## Invariants make test selection concrete

"Add tests" is underspecified. For a `PositiveMoney` invariant, test zero
rejection, valid construction, and arithmetic overflow/currency mismatch. For a
private verified-email constructor, add compile-fail evidence that external code
cannot construct it and runtime evidence that verifier proof is checked. For a
message consumer, test duplicates and acknowledgement loss. For a lock-free
protocol, combine invariant reasoning with model exploration.

An evidence plan therefore starts from the invariant inventory, not from test
framework preferences.

## Negative tests protect the boundary

Happy paths demonstrate admission, but trusted types are defined equally by
what they exclude. Boundary values, malformed encodings, contradictory row
states, unknown variants, oversized input, and unauthorized transitions should
produce structured rejection. Tests should assert the category callers use, not
fragile full wording unless wording is itself a contract.

Compile-fail tests are valuable when the claim is "this program cannot be
expressed through the public API." Their diagnostic snapshots are evidence that
must be interpreted. A changed compiler can alter wording while the prohibition
remains, or the program can still fail for an unrelated import error while the
actual protection vanished.

## Properties explore shape, not infinity

Property testing can generate many combinations and shrink failures to useful
examples. The generator defines the explored universe. If it never creates
Unicode edge cases, large lengths, or invalid state sequences, clean runs say
nothing about them. Properties also need an independent oracle; asserting that
encoding followed by the same flawed decoder returns something can preserve a
shared defect.

Strong properties include round-trip through independently specified formats,
normalization idempotence, arithmetic laws with stated overflow domain,
ordering/uniqueness after every mutation, and state-machine invariants over
generated commands.

## Real boundaries matter

An in-memory repository may accept values a database rejects, ignore isolation,
or provide instantaneous consistency. A mock HTTP client may never delay after
remote execution. A fake broker may deliver each message once in order. These
doubles are useful for local logic only if broader tests cover the omitted
semantics.

Contract tests verify schemas and semantic categories across deployment
boundaries. Real integration tests reveal encoding, configuration, driver, and
transaction behavior. Neither guarantees the remote service will behave
forever, so monitoring and compatibility ownership remain.

## Concurrency needs controlled schedules

Sleeping creates a hope that another task progressed. Slow or fast CI hosts can
violate that timing. Barriers, channels, deterministic executors, paused clocks,
and observed events make the desired ordering explicit. Loom can enumerate
possible schedules for a small modeled protocol and detect invariant failures
that stress tests seldom encounter.

Models require scrutiny. Replacing a production primitive with a simplified one
can omit behavior. State bounds may exclude long histories. Still, a carefully
matched small model supplies stronger schedule evidence than thousands of
uncontrolled repetitions.

## Fault injection targets the spaces between steps

Many distributed defects occur between successful steps: after effect before
acknowledgement, after domain commit before publish, after partial file write
before rename, or after request dispatch before response. Returning an error
from the first call in a mock does not exercise these states.

A crash-point matrix names each durable boundary and expected recovery. Tests
then stop or fail at each point, restart the component, and verify invariant,
duplicate handling, and unknown outcome. Delay and reordering tests expose
timeouts and stale-observation assumptions.

## Snapshots require semantic ownership

Snapshots are useful for large structured outputs and compiler diagnostics.
Their danger is an easy update command that converts all current output into
expected output. Review should classify changes: intended semantic change,
stable formatting migration, environment noise, or unexpected regression.
Nondeterministic IDs, timestamps, and paths should be normalized at the source
or represented deliberately.

Compiler UI snapshots demand particular care. The test passes if output matches,
even when the failing cause no longer demonstrates the intended privacy or type
rule. Inspect the actual diagnostic.

## Flakiness is a system observation

Flaky tests can reveal races, leaked state, clock assumptions, resource
exhaustion, unstable external dependencies, and insufficient isolation.
Automatic retries improve short-term pipeline throughput but destroy frequency
and signature evidence if used alone. A temporary quarantine needs a visible
owner and deadline, with captured seeds, traces, environment, and timing.

If the root cause is genuinely external instability, the product's behavior
under that instability may also need design work.

## Coverage and mutation

Line and branch coverage reveal code that tests did not execute. They do not
show whether assertions would detect a defect, whether input partitions are
meaningful, or whether concurrency schedules occurred. Mutation testing can
provide additional evidence that assertions detect selected changes, but the
mutation set is also a model.

Use coverage to find gaps after mapping invariants, not as the definition of
quality.

## Production and incidents

Telemetry can validate workload distributions, error frequencies, latency,
queue saturation, and reconciliation age. It is strongest when detection
mechanisms are themselves tested. Silent data corruption or missing events may
produce no metric. An incident supplies high-authority evidence that one
failure mechanism is real; it should produce a regression test, fault scenario,
or doctrine correction where appropriate.

## Evidence ledger example

| Claim                                          | Evidence                                    | Scope                         | Does not prove              | Residual risk                  |
| ---------------------------------------------- | ------------------------------------------- | ----------------------------- | --------------------------- | ------------------------------ |
| direct verified-email construction is blocked  | compile-fail test                           | public API on pinned compiler | verifier truth              | unsafe/internal future escape  |
| raw DB email is validated                      | integration fixtures plus constructor tests | tested schema/driver versions | all historical rows valid   | alternate writer or corruption |
| duplicate command does not repeat local effect | transactional integration test              | local database boundary       | remote effect uniqueness    | retention expiry               |
| atomic protocol preserves one-owner state      | Loom model plus reasoning                   | modeled bounds and primitives | unsupported target behavior | model mismatch                 |
| timeout remains unknown                        | fault injection after dispatch              | selected protocol points      | provider final state        | reconciliation outage          |

## Proportionality

Every test type has cost: environment maintenance, execution time, flaky
surface, and diagnostic work. A pure formatter does not need distributed fault
injection. A payment capture does. Select the smallest evidence portfolio that
addresses consequential failure risks, then state uncovered assumptions
honestly.

---

## Source: `doctrines/0008-testing-and-evidence/decision-framework.md`

# Decision framework

## Map invariant to evidence

For every invariant record:

1. enforcement mechanism;
2. legal construction or transition;
3. prohibited path;
4. boundary where evidence enters;
5. external failure points;
6. concurrency or persistence risks;
7. primary test class;
8. independent supporting evidence;
9. residual risk.

## Select test classes

| Claim shape                                  | Primary evidence                     |
| -------------------------------------------- | ------------------------------------ |
| public API cannot express misuse             | compile-fail test                    |
| constructor accepts/rejects specified values | unit/table tests                     |
| law holds across broad generated values      | property test                        |
| parser and serializer agree with format      | fixtures, properties, contract tests |
| database conversion preserves invariant      | real integration test                |
| consumer tolerates replay                    | duplicate/fault-injection test       |
| small concurrent protocol preserves state    | model checking plus unit tests       |
| unsafe operation respects memory rules       | proof plus Miri/sanitizers/fuzzing   |
| end-to-end workflow recovers from crash      | fault-injected system test           |
| deployed workload meets expectation          | telemetry plus performance evidence  |

Use more than one layer when a claim crosses layers.

## Boundary-value design

Partition the input domain:

- minimum and maximum accepted;
- just below and above each bound;
- empty and zero;
- malformed structure;
- valid syntax but rejected policy;
- normalization collisions;
- Unicode and encoding cases;
- unknown versions/variants;
- oversized and deeply nested;
- duplicate and reordered;
- stale version;
- cancelled at each partial step.

Assert structured categories and retained evidence, not only `is_err()`.

## Property-test design

Define the property in domain language before generator code. Ensure generators
cover valid and invalid partitions, avoid excessive rejection, record failing
seeds, and shrink to interpretable cases. Compare against an independent model
or stable specification when possible. Bound sizes explicitly so execution
cost and unexplored regions are known.

## Compile-fail decision

Use a compile-fail test when:

- privacy prevents direct trusted construction;
- ownership consumption prevents handle reuse;
- typestate prevents an illegal operation;
- capability types restrict authority;
- trait bounds intentionally exclude a class.

Keep each failing source minimal. Verify the diagnostic points to the intended
rule. Update expected output only after semantic review on the pinned toolchain.

## Double fidelity decision

For each double, compare:

| Real behavior         | Double behavior                   | Gap owner               |
| --------------------- | --------------------------------- | ----------------------- |
| latency/cancellation  | controlled delay or instant       | named integration suite |
| capacity/backpressure | bounded or unlimited              | overload suite          |
| transaction/isolation | real or simplified                | database tests          |
| duplicate/order       | configurable or perfect           | messaging fault tests   |
| unknown outcome       | representable or binary           | distributed suite       |
| schema/version        | actual codec or hand-built values | contract suite          |

If the double erases the very risk under test, replace it.

## Flaky-test procedure

1. retain the first failure signature and full reproducibility data;
2. classify shared state, time, schedule, randomness, resource, and external
   dependencies;
3. reproduce with fixed seed or controlled schedule;
4. determine whether product or harness owns the nondeterminism;
5. fix the cause;
6. use temporary quarantine only with owner and expiry;
7. remove retries that mask the resolved class.

## Stop conditions

Stop approval when:

- tests have no claim mapping;
- only positive construction is tested;
- a mock removes critical failure semantics;
- concurrency ordering depends on sleep;
- fault tests fail only before any effect;
- compiler output was bulk accepted;
- snapshot change lacks meaning analysis;
- flaky failure is resolved solely by retries;
- coverage percentage is the primary quality argument;
- production absence of incidents is described as proof.

---

## Source: `doctrines/0008-testing-and-evidence/review-standard.md`

# Review standard

Record **pass**, **fail**, **not applicable**, or an approved **waiver
reference** for every gate.

| Gate | Question                                            | Pass evidence               | Failure example                    | Severity | Remediation           |
| ---- | --------------------------------------------------- | --------------------------- | ---------------------------------- | -------- | --------------------- |
| T01  | Does each test map to a claim?                      | invariant/risk reference    | incidental method call test        | high     | name purpose          |
| T02  | Does every critical invariant have evidence?        | evidence matrix             | authority invariant untested       | critical | add layer             |
| T03  | Are evidence limits stated?                         | scope/non-proof column      | passing tests called proof         | high     | qualify               |
| T04  | Are valid constructor cases tested?                 | representative table        | no acceptance test                 | medium   | add                   |
| T05  | Are invalid constructor cases tested?               | boundary rejection          | only happy path                    | high     | add negatives         |
| T06  | Are exact bounds tested?                            | below/at/above cases        | only middle value                  | high     | add boundaries        |
| T07  | Are structured errors asserted?                     | category assertions         | only `is_err()`                    | medium   | inspect category      |
| T08  | Is normalization tested?                            | idempotence/collision cases | silent data change                 | high     | add properties        |
| T09  | Are Unicode/encoding risks represented?             | fixtures                    | ASCII-only parser tests            | high     | broaden domain        |
| T10  | Are size/resource limits tested?                    | oversized input             | decoder limit unexercised          | high     | add adversarial case  |
| T11  | Is property appropriate?                            | domain-level statement      | implementation restatement         | high     | define independently  |
| T12  | Does generator cover relevant partitions?           | distribution analysis       | invalid cases filtered out         | high     | improve generator     |
| T13  | Are failing seeds reproducible?                     | seed capture                | random CI failure irreproducible   | high     | persist seed          |
| T14  | Is shrink result interpretable?                     | minimal case                | huge opaque failure                | medium   | tune strategy         |
| T15  | Is oracle independent enough?                       | model/spec comparison       | encoder tests itself               | high     | add oracle            |
| T16  | Are prohibited APIs compile-tested?                 | UI cases                    | privacy only assumed               | high     | add compile-fail      |
| T17  | Is each compile-fail source minimal?                | one prohibition             | unrelated errors                   | high     | simplify              |
| T18  | Does diagnostic fail for intended reason?           | semantic inspection         | missing import causes pass         | critical | repair fixture        |
| T19  | Was `.stderr` change reviewed?                      | focused diff rationale      | overwrite accepted blindly         | critical | inspect               |
| T20  | Is pinned compiler used for UI evidence?            | toolchain config            | diagnostics vary unnoticed         | high     | pin                   |
| T21  | Are real codecs exercised?                          | serialization integration   | hand-built domain only             | high     | cross boundary        |
| T22  | Is real database behavior exercised where needed?   | integration setup           | in-memory map stands for isolation | critical | add DB test           |
| T23  | Are migrations tested from old fixtures?            | version fixtures            | fresh schema only                  | high     | migrate               |
| T24  | Are protocol contracts versioned?                   | compatibility tests         | current pair only                  | high     | add matrix            |
| T25  | Are unknown fields/variants tested?                 | forward cases               | decoder panics                     | high     | add                   |
| T26  | Is authentication/authorization boundary tested?    | separate outcomes           | mock principal injected            | critical | cross real adapter    |
| T27  | Do doubles preserve relevant failures?              | fidelity table              | remote never times out             | critical | improve double        |
| T28  | Are double gaps covered elsewhere?                  | suite reference             | undocumented omission              | high     | assign owner          |
| T29  | Does concurrency test use explicit synchronization? | barrier/event/model         | sleeps establish order             | critical | control schedule      |
| T30  | Is cancellation tested at partial steps?            | cancellation matrix         | only before start                  | critical | inject at awaits      |
| T31  | Is cleanup asserted after cancellation?             | resource counts/state       | task drop assumed enough           | high     | inspect postcondition |
| T32  | Are lock/channel closures tested?                   | owner-drop cases            | unwrap closure                     | high     | add                   |
| T33  | Is shutdown tested while loaded?                    | drain/deadline case         | idle-only shutdown                 | high     | add outstanding work  |
| T34  | Is model abstraction documented?                    | production/model map        | Loom model differs silently        | high     | explain gaps          |
| T35  | Are model bounds sufficient for claim?              | bound rationale             | one trivial step                   | high     | expand/narrow claim   |
| T36  | Are partial durable failures injected?              | crash-point matrix          | error only before write            | critical | inject between steps  |
| T37  | Are duplicate deliveries tested?                    | repeated identity case      | broker double once-only            | critical | add replay            |
| T38  | Are delayed acknowledgements tested?                | effect-before-loss case     | timeout only pre-dispatch          | critical | inject loss           |
| T39  | Are reorderings tested?                             | version/out-of-order cases  | global FIFO assumed                | high     | add sequences         |
| T40  | Does unknown remain unknown?                        | outcome assertion           | timeout collapsed                  | critical | preserve state        |
| T41  | Is reconciliation tested repeatedly?                | still-unknown then terminal | one query only                     | high     | model lifecycle       |
| T42  | Are retries bounded in tests?                       | virtual-time budget         | test can loop forever              | high     | cap                   |
| T43  | Do snapshots exclude nondeterministic noise?        | normalization policy        | changing timestamps                | medium   | stabilize             |
| T44  | Are snapshot changes semantically explained?        | review note                 | bulk approval                      | critical | classify diffs        |
| T45  | Are golden fixtures sourced and versioned?          | provenance                  | unexplained blob                   | medium   | document              |
| T46  | Are flaky signatures retained?                      | issue/log evidence          | rerun erases failure               | high     | capture first         |
| T47  | Is retry temporary and visible?                     | owner/expiry                | permanent CI reruns                | high     | fix cause             |
| T48  | Are tests isolated in parallel?                     | unique resources            | shared fixed port/file             | high     | allocate uniquely     |
| T49  | Are environment mutations restored safely?          | scoped guard/process        | global env races                   | high     | isolate process       |
| T50  | Are clocks controlled?                              | injected/paused clock       | wall-clock sleep                   | high     | abstract time         |
| T51  | Is randomness seeded?                               | recorded seed               | irreproducible fuzz failure        | high     | capture               |
| T52  | Is coverage supplemental?                           | invariant matrix            | percentage alone                   | high     | add semantic evidence |
| T53  | Are benchmark assertions separately tested?         | correctness suite           | benchmark only                     | high     | extract tests         |
| T54  | Does unsafe code have specialized evidence?         | Miri/sanitizer results      | ordinary tests only                | critical | run tools             |
| T55  | Are tool blind spots stated?                        | limitations                 | Miri called complete proof         | high     | qualify               |
| T56  | Does telemetry detect claimed outcomes?             | tested metrics              | silent failure not instrumented    | high     | add observability     |
| T57  | Did incidents create regressions?                   | linked test                 | fix has no reproduction            | high     | encode mechanism      |
| T58  | Is test-data sensitivity controlled?                | synthetic/redacted fixtures | production secret copied           | critical | scrub                 |
| T59  | Are cleanup failures visible?                       | teardown result             | errors ignored                     | high     | report                |
| T60  | Is total suite cost proportionate?                  | layer rationale             | redundant slow tests               | medium   | rebalance             |

Critical gaps block merge. Waivers identify the uncovered invariant, alternative
evidence, consequence, owner, expiry, and removal condition.

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0008-R001`, `RUST-DOC-0008-R002`, `RUST-DOC-0008-R003`, `RUST-DOC-0008-R004`
- `RUST-DOC-0008-R005`, `RUST-DOC-0008-R006`, `RUST-DOC-0008-R007`, `RUST-DOC-0008-R008`
- `RUST-DOC-0008-R009`, `RUST-DOC-0008-R010`, `RUST-DOC-0008-R011`, `RUST-DOC-0008-R012`
- `RUST-DOC-0008-R013`, `RUST-DOC-0008-R014`, `RUST-DOC-0008-R015`, `RUST-DOC-0008-R016`
- `RUST-DOC-0008-R017`, `RUST-DOC-0008-R018`, `RUST-DOC-0008-R019`, `RUST-DOC-0008-R020`
- `RUST-DOC-0008-R021`

---

## Source: `doctrines/0008-testing-and-evidence/anti-patterns.md`

# Anti-pattern catalogue

## Happy-path certificate

**Weak example.** One accepted value test is cited as proof a validated type is
correct.

**Why it fails.** The excluded set and boundary behavior are unobserved.

**Risk.** invalid construction and weak error contracts.

**Improved direction.** test accepted, rejected, boundary, normalization, and
conversion paths.

**When justified.** A thin delegation may test integration while citing the
complete underlying suite.

## `is_err()` everywhere

**Weak example.** Tests assert only that some error occurred.

**Why it fails.** Error categories can collapse or the wrong validation can
trigger.

**Risk.** callers lose actionable distinctions.

**Improved direction.** assert structured category and relevant safe context.

**When justified.** Exact category may be intentionally opaque at a security
boundary; assert the public contract there.

## Examples as properties

**Weak example.** Three hand-picked strings are used to claim parser robustness.

**Why it fails.** broad input partitions and interactions remain unexplored.

**Risk.** rare panics or normalization defects.

**Improved direction.** define properties and generators plus targeted edge
fixtures.

**When justified.** Exhaustive small finite domains can use tables.

## Self-confirming oracle

**Weak example.** A serializer is tested by decoding only with its paired
implementation and comparing a value both normalize identically.

**Why it fails.** shared defects can preserve round trips.

**Risk.** external incompatibility.

**Improved direction.** add specification fixtures or independent
implementation/contract evidence.

**When justified.** Round-trip remains one useful property among independent
checks.

## Compile-fail for the wrong reason

**Weak example.** A UI fixture lacks an import, so it fails before testing field
privacy.

**Why it fails.** the harness is green while the intended prohibition may have
vanished.

**Risk.** forged trusted values.

**Improved direction.** minimize source and inspect exact diagnostics.

**When justified.** Never as evidence for the intended rule.

## Overwrite compiler output

**Weak example.** All new `.stderr` files are accepted after a toolchain update
without reading them.

**Why it fails.** expected failure cause can change.

**Risk.** silent API weakening.

**Improved direction.** review each diagnostic semantically and group only
equivalent wording changes.

**When justified.** Formatting-only compiler changes can share a documented
review.

## Perfect mock network

**Weak example.** The mock either returns success or fails before execution.

**Why it fails.** it cannot express delayed success, lost response, or duplicate
execution.

**Risk.** timeout collapses into rejection.

**Improved direction.** use a controllable fake with failure points and real
integration evidence.

**When justified.** A narrow pure mapping unit test may omit network semantics.

## Sleep for ordering

**Weak example.** task A sleeps ten milliseconds so task B is expected to run.

**Why it fails.** scheduler and host timing are not controlled.

**Risk.** flakiness and false schedule evidence.

**Improved direction.** use barriers, channels, paused clocks, or model
checking.

**When justified.** Sleep may define a deadline, not establish the event.

## Retry green

**Weak example.** CI retries a flaky test until one run passes and reports
success.

**Why it fails.** the failure mechanism and frequency disappear.

**Risk.** races and production instability persist.

**Improved direction.** capture first failure, make it reproducible, fix cause,
and use temporary visible quarantine only.

**When justified.** Bounded retries may gather diagnostic samples while owned
remediation proceeds.

## Snapshot approval as review

**Weak example.** A broad snapshot update command is followed by commit because
the files now match.

**Why it fails.** current behavior becomes expected without semantic judgment.

**Risk.** overclaims, UI regressions, or diagnostic weakening.

**Improved direction.** classify and explain focused changes.

**When justified.** A deterministic mechanical formatting migration may be
reviewed as one classified transformation.

## Coverage target as quality

**Weak example.** A high line percentage is the sole release criterion.

**Why it fails.** executed lines may contain no meaningful assertions or
adverse inputs.

**Risk.** untested invariants under impressive metrics.

**Improved direction.** maintain an invariant-to-evidence matrix and use
coverage for gap discovery.

**When justified.** Coverage can enforce that new code is not wholly
unexercised.

## Production has not failed

**Weak example.** absence of incidents is cited as proof a protocol is correct.

**Why it fails.** failure may be rare, invisible, or absent from observed
workloads.

**Risk.** unsupported guarantee claims.

**Improved direction.** test detection, inject faults, and state observation
limits.

**When justified.** Production evidence can update likelihood estimates, not
establish impossibility.

---

## Source: `doctrines/0008-testing-and-evidence/glossary.md`

# Glossary

**Contract test**
: Evidence that independently deployed components preserve an agreed protocol
shape and semantics.

**Crash-point matrix**
: A list of interruption locations between durable or external steps and the
expected recoverable state at each.

**Evidence ledger**
: A mapping from claims to evidence scope, non-proofs, and residual risk.

**Fault injection**
: Deliberate creation of failures, delays, losses, crashes, or resource
exhaustion at controlled protocol points.

**Flaky test**
: A test whose result changes without an intended change to the tested contract,
indicating uncontrolled state, time, schedule, environment, or behavior.

**Model checking**
: Systematic exploration of states or schedules within an explicit model and
bounds.

**Property test**
: Repeated evaluation of a stated property over generated values, usually with
shrinking of counterexamples.

**Snapshot**
: Committed expected output compared as a whole or structured artifact.

**Test double**
: A replacement for a real collaborator, including mock, fake, emulator, or
in-memory implementation.

**UI test**
: In this repository, a compile-fail test that compares compiler diagnostics;
the term does not necessarily mean graphical interface testing.

---

## Source: `doctrines/0008-testing-and-evidence/references.md`

# References

- [The Rust Book: writing automated tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
  describes Rust's built-in test structure and organization.
- [Cargo reference: tests](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
  defines workspace test execution behavior.
- [trybuild documentation](https://docs.rs/trybuild/latest/trybuild/) documents
  stable compile-fail/UI testing and expected diagnostic files.
- [proptest documentation](https://docs.rs/proptest/latest/proptest/) documents
  generated property testing, strategies, shrinking, and persistence.
- [Loom documentation](https://docs.rs/loom/latest/loom/) describes permutation
  testing for concurrent Rust code under a model.
- [Miri](https://github.com/rust-lang/miri) documents interpreted execution for
  detecting many undefined-behavior violations.
- [Rust compiler sanitizer documentation](https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html)
  documents available sanitizers and platform constraints.
- [Criterion.rs documentation](https://bheisler.github.io/criterion.rs/book/)
  documents statistical benchmarking; RUST-DOC-0009 governs claims made from
  measurements.

The doctrine adds invariant traceability, evidence-limit statements,
compiler-diagnostic review, double-fidelity analysis, incident feedback, and
operational review gates.

---

## Source: `doctrines/0009-performance-and-measurement/README.md`

---
id: RUST-DOC-0009
slug: performance-and-measurement
title: Performance Claims Require Measurement
status: active
version: 0.1.0
normative: true
applies_to:
  - planning
  - implementation
  - review
  - audit
  - maintenance
risk_domains:
  - performance
  - capacity
  - latency
  - resource-cost
supersedes: []
superseded_by: null
---

# Performance Claims Require Measurement

## Scope

This package governs performance objectives, measurements, optimizations, and
claims about Rust software. It covers workload definition, benchmark design,
profiling, latency distributions, throughput, CPU and wall-clock time,
allocations, cache locality, contention, batching, backpressure, serialization,
system calls, database and network limits, binary size, monomorphization, and
compile time.

Rust enables low-level control and strong optimization, but compilation does not
establish speed. A result is meaningful only relative to an environment, input
distribution, concurrency level, system state, measurement method, and
correctness contract. Faster output that violates an invariant is not an
optimization.

## Out of scope

This doctrine does not select one benchmark framework, profiler, allocator, or
runtime. It does not prescribe optimization of code without an objective. It
does not treat a microbenchmark as end-to-end evidence or use unsafe code as a
default performance technique.

## Intended readers

- planners defining objectives and workloads;
- implementers profiling and changing hot paths;
- reviewers checking methodology and correctness preservation;
- auditors challenging broad or irreproducible claims;
- maintainers controlling regressions, build cost, and environment drift.

## Normative status

[`doctrine.md`](../doctrines/0009-performance-and-measurement/doctrine.md) is normative. A performance statement in code,
documentation, review, or release notes is a claim subject to these rules.
Waivers may accept an unmeasured low-risk cleanup, but cannot convert intuition
into a measured claim.

## Prerequisite foundations

Read complexity budget, guarantee honesty, invariants, and evidence under
[`../../foundations/`](../foundations/). Also apply RUST-DOC-0004 to
concurrency, RUST-DOC-0007 to unsafe optimization, and RUST-DOC-0008 to the
difference between benchmark and correctness evidence.

## Related material

- Patterns: [validated collections](../patterns/validated-collections.md),
  [opaque newtypes](../patterns/opaque-newtypes.md), typestate, and
  [hybrid state machines](../patterns/hybrid-state-machines.md) all have runtime and compile-time
  costs.
- Boundaries: serialization, database, [HTTP/RPC](../boundaries/http-and-rpc.md), messaging, and
  filesystem.
- Reviews: [pre-implementation](../reviews/pre-implementation.md), typestate, distributed
  effects, and final audit.
- Case studies: performance choices remain subordinate to each guarantee ledger.

## Reading order

Read the rules, then the rationale. Use the decision framework to design
measurement before changing code. Apply the review standard to benchmark
artifacts and claims. Use anti-patterns to detect attractive but unsupported
stories.

## Compact doctrine summary

Optimization begins with a defined objective and representative workload.
Claims name environment, inputs, system state, sample method, and uncertainty.
Profile before optimizing. Benchmarks defend against dead-code elimination,
setup contamination, unstable machines, and invalid comparisons. Report
latency distributions, not only averages. Measure allocation, copies, syscalls,
contention, and size rather than inferring them. Async concurrency is not
parallel speedup. Zero-copy claims identify exactly which copies are removed and
which lifetime or retention costs are introduced. Regression thresholds are
automated only for sufficiently stable signals. All changes preserve invariants.

## Executable evidence status

The 0.1.0 workspace contains no benchmark harness, retained measurement,
allocation profile, flamegraph, or performance-regression threshold. It
therefore makes no measured claim about example speed, latency, allocation, or
binary size. This doctrine specifies the evidence required when such a claim is
introduced; it does not convert unmeasured examples into performance evidence.

---

## Source: `doctrines/0009-performance-and-measurement/doctrine.md`

# Normative doctrine

## RUST-DOC-0009-R001 — Define objective and workload

**Statement.** Optimization MUST begin with a quantified objective and a
workload representing the input distribution, concurrency, and system boundary
that matter.

**Intent.** Prevent work on irrelevant micro-costs.

**Applicability.** Performance changes, capacity plans, and regression gates.

**Allowed exceptions.** Removing an obviously unnecessary operation may proceed
as ordinary cleanup if no performance claim is made.

**Review evidence.** Metric, target, baseline, workload, and correctness
constraints.

## RUST-DOC-0009-R002 — Scope every performance claim

**Statement.** Performance claims MUST include environment, toolchain, build
profile, input distribution, concurrency, warmup/cache state, measurement
method, and comparison baseline sufficient for reproduction.

**Intent.** Make numbers interpretable and falsifiable.

**Applicability.** Documentation, pull requests, releases, and design decisions.

**Allowed exceptions.** A local exploratory note may be labeled preliminary and
must not support a merge claim.

**Review evidence.** Reproducible command, environment manifest, raw or
summarized samples, and commit identities.

## RUST-DOC-0009-R003 — Profile before optimizing

**Statement.** Profiling SHOULD precede nontrivial optimization and MUST precede
claims about a dominant bottleneck.

**Intent.** Direct effort to measured cost centers.

**Applicability.** Latency, CPU, allocation, contention, I/O, and size work.

**Allowed exceptions.** Algorithmic complexity defects apparent from complete
input bounds may be corrected without a profile, while still measuring outcome.

**Review evidence.** Flamegraph, trace, allocation profile, system metrics, or
equivalent relevant evidence.

## RUST-DOC-0009-R004 — Preserve correctness independently

**Statement.** A performance change MUST preserve domain invariants,
error/uncertainty semantics, security properties, and boundary validation, with
correctness evidence independent of the benchmark.

**Intent.** Reject faster incorrect behavior.

**Applicability.** All optimizations.

**Allowed exceptions.** An explicit product tradeoff may change semantics only
as a separately reviewed normative or API change, not as hidden optimization.

**Review evidence.** Invariant-linked tests and guarantee-ledger diff.

## RUST-DOC-0009-R005 — Defend benchmark execution

**Statement.** Benchmark code MUST prevent dead-code elimination, constant
folding, unintended setup measurement, and unrealistic reuse from invalidating
the intended workload.

**Intent.** Ensure measured work corresponds to the claim.

**Applicability.** Microbenchmarks and component benchmarks.

**Allowed exceptions.** None; framework facilities may provide the mechanism.

**Review evidence.** Input generation, black-boxing where appropriate,
setup/measurement separation, and result consumption.

## RUST-DOC-0009-R006 — Separate wall-clock and CPU claims

**Statement.** Measurements MUST distinguish wall-clock latency, CPU time, and
aggregate CPU consumption when their interpretations differ.

**Intent.** Prevent waiting and parallel work from being described as reduced
compute cost.

**Applicability.** Async, parallel, I/O-bound, and multi-process workloads.

**Allowed exceptions.** A single-threaded CPU-bound benchmark may report one
measure with its assumption stated.

**Review evidence.** Metric definition and collection method.

## RUST-DOC-0009-R007 — Report distributions

**Statement.** User-visible or service latency claims MUST report appropriate
distributions such as p50, p95, and p99 rather than only arithmetic averages.

**Intent.** Reveal tail behavior and multimodal workloads.

**Applicability.** Requests, queues, storage, and batch completion.

**Allowed exceptions.** Deterministic fixed-cost operations may use a narrow
summary after showing low variance.

**Review evidence.** Sample count, percentile method, confidence or variability,
and outlier policy.

## RUST-DOC-0009-R008 — Document warmup and cache state

**Statement.** Measurements MUST state process warmup, JIT or runtime
initialization where applicable, filesystem/page/cache state, connection reuse,
and dataset residency relevant to the claim.

**Intent.** Prevent cold and warm behavior from being mixed invisibly.

**Applicability.** Storage, network, serialization, and repeated services.

**Allowed exceptions.** A test may deliberately mix states only if the workload
distribution matches production and is documented.

**Review evidence.** Preparation sequence and separate cold/warm results where
both matter.

## RUST-DOC-0009-R009 — Measure allocation claims

**Statement.** Claims that code allocates less, performs no allocation, or
reduces memory MUST be supported by an allocator-aware measurement and MUST
identify retained as well as peak memory where relevant.

**Intent.** Avoid inferring allocation from syntax or clone count.

**Applicability.** Buffering, parsing, collections, async boxing, and caching.

**Allowed exceptions.** A direct removal of the only allocation call may be
noted structurally, but broader runtime claims still require measurement.

**Review evidence.** Allocation count/bytes, allocator, peak/resident set, and
workload.

## RUST-DOC-0009-R010 — Scope zero-copy claims

**Statement.** A zero-copy claim MUST identify every copy avoided within the
specified path and the lifetime, pinning, retention, fragmentation, API, and
ownership costs introduced.

**Intent.** Prevent one avoided copy from becoming a broad slogan.

**Applicability.** Parsers, networking, serialization, buffers, and FFI.

**Allowed exceptions.** None for the phrase "zero-copy."

**Review evidence.** Data-flow diagram, measured copy/allocation evidence, and
non-guarantees.

## RUST-DOC-0009-R011 — Do not equate async with speedup

**Statement.** Async concurrency MUST NOT be described as parallel CPU speedup
without evidence of parallel execution and a workload that benefits.

**Intent.** Distinguish overlap of waiting from reduced compute time.

**Applicability.** Runtime migrations, fan-out, and worker design.

**Allowed exceptions.** None for the claim; async may still improve resource
efficiency or concurrent latency.

**Review evidence.** Executor configuration, CPU utilization, throughput,
latency, and contention.

## RUST-DOC-0009-R012 — Make throughput/latency tradeoffs explicit

**Statement.** Batching, buffering, pipelining, and concurrency changes MUST
report both throughput and relevant latency/queue consequences.

**Intent.** Prevent aggregate gains from hiding worse tails or freshness.

**Applicability.** Brokers, databases, serializers, and service queues.

**Allowed exceptions.** Offline throughput-only jobs may state that latency has
no objective while still bounding resource use.

**Review evidence.** Batch/concurrency sweep and distribution results.

## RUST-DOC-0009-R013 — Measure contention and backpressure

**Statement.** Concurrent performance analysis MUST include queue depth, wait
time, saturation, lock or permit contention, rejection, and downstream load
where relevant.

**Intent.** Reveal whether local throughput shifts cost elsewhere.

**Applicability.** Shared state, pools, channels, and fan-out.

**Allowed exceptions.** Pure independent parallel work may document absence of
shared contention.

**Review evidence.** Contention profile, load curve, and overload behavior.

## RUST-DOC-0009-R014 — Count boundary costs

**Statement.** Performance investigations MUST consider serialization,
allocation, copies, syscalls, context switches, database queries, network
round-trips, and external rate limits before attributing cost solely to Rust
source constructs.

**Intent.** Optimize the actual end-to-end path.

**Applicability.** Integrated and service workloads.

**Allowed exceptions.** A deliberately isolated microbenchmark may narrow scope
and state that it excludes boundary cost.

**Review evidence.** Trace or component budget.

## RUST-DOC-0009-R015 — Review clone removal architecturally

**Statement.** Avoiding `clone` MUST NOT introduce worse algorithmic complexity,
excessive borrowing, global sharing, lock contention, or retention without
measurement and ownership analysis.

**Intent.** Prevent syntax-focused optimization from degrading architecture.

**Applicability.** Buffers, collections, async tasks, and shared state.

**Allowed exceptions.** Removal of a proven redundant clone with unchanged
ownership shape may be a local cleanup.

**Review evidence.** Data ownership, allocation profile, complexity, and
contention.

## RUST-DOC-0009-R016 — Govern unsafe optimization

**Statement.** Unsafe performance changes MUST satisfy RUST-DOC-0007 and MUST
show a material measured benefit under the target workload.

**Intent.** Charge proof risk to the benefit it buys.

**Applicability.** Unchecked indexing, custom allocation, SIMD, FFI, and
lock-free code.

**Allowed exceptions.** Unsafe may be necessary for an external API even when
performance is not its justification; that case is not an optimization claim.

**Review evidence.** Safe baseline, benchmark, profile, safety proof, and
specialized tests.

## RUST-DOC-0009-R017 — Automate stable regressions

**Statement.** Regression thresholds SHOULD be automated only for metrics whose
environmental variance is measured and whose threshold includes a justified
noise budget.

**Intent.** Catch real regressions without normalizing noisy gates.

**Applicability.** CI benchmarks, binary-size checks, allocations, and compile
time.

**Allowed exceptions.** Noisy metrics may run as trend reports or on controlled
dedicated hosts.

**Review evidence.** Baseline history, variance, threshold, hardware stability,
and rerun policy.

## RUST-DOC-0009-R018 — Do not generalize microbenchmarks

**Statement.** Microbenchmark results MUST NOT be generalized to end-to-end
performance without evidence connecting the measured operation to overall
workload contribution.

**Intent.** Prevent large local ratios from masking tiny system impact.

**Applicability.** Library and application optimization claims.

**Allowed exceptions.** A microbenchmark may establish the cost of the exact
isolated primitive it measures.

**Review evidence.** Profile share, integrated benchmark, or component budget.

## RUST-DOC-0009-R019 — Account for build and binary cost

**Statement.** Abstraction choices involving generics, code generation, feature
sets, or dependencies SHOULD assess compile time, monomorphization, binary size,
incremental behavior, and diagnostic cost when material.

**Intent.** Treat developer and deployment resources as performance dimensions.

**Applicability.** Public generic APIs, macro-heavy code, and constrained
artifacts.

**Allowed exceptions.** Small local code with immaterial measured impact may
document no concern.

**Review evidence.** Build timing, artifact sections, generic instantiations, or
dependency analysis.

## RUST-DOC-0009-R020 — Retain reproducible evidence

**Statement.** Accepted performance decisions MUST retain commands, commits,
configuration, result summaries, and raw-data location or format sufficient to
repeat or challenge the result.

**Intent.** Make optimization decisions durable and auditable.

**Applicability.** Merged performance changes and release claims.

**Allowed exceptions.** Sensitive production traces may be retained in
controlled storage with a sanitized reproducible summary.

**Review evidence.** Benchmark record and provenance.

---

## Source: `doctrines/0009-performance-and-measurement/rationale.md`

# Rationale

## Performance has a workload

"Faster" is incomplete. An implementation can be faster for tiny ASCII inputs
and slower for large Unicode data; improve throughput at low concurrency and
collapse under contention; reduce average latency while worsening p99; save CPU
but increase memory retention. The objective selects the metric and the
workload establishes relevance.

A useful objective resembles: process the production-sized message distribution
at a stated concurrency with p99 latency below a target, while limiting peak
memory and preserving rejection and ordering semantics. It creates a decision
criterion rather than an aesthetic preference.

## Profile before explanation

Source inspection suggests hypotheses, not cost attribution. A clone may be
optimized away or insignificant next to a database round-trip. A parser may be
CPU-heavy only because decompression dominates. A mutex may show little
contention until a downstream service slows and the critical section expands.

CPU profiles, allocation traces, async spans, system-call traces, database query
plans, and network observations answer different questions. Choose the profiler
that can observe the suspected resource and verify that sampling overhead or
instrumentation does not change the conclusion materially.

## Benchmark discipline

Optimizing compilers can remove work whose result is unused, hoist constants,
or precompute predictable inputs. Benchmarks should generate or select inputs
outside the timed path, consume results, and prevent unrealistic knowledge
without hiding real optimization opportunities. Setup, teardown, allocation,
and cloning belong inside or outside measurement according to the claim.

Benchmark processes also share hosts with frequency scaling, thermal limits,
interrupts, background work, and virtual-machine noise. Record environment,
repeat samples, and compare like with like. A statistically significant small
difference may still be operationally irrelevant; a large difference under a
nonrepresentative input may also be irrelevant.

## Wall-clock, CPU, and throughput

Wall-clock latency includes waiting. CPU time measures compute consumed by one
or more threads. Parallel code can reduce wall time while consuming more total
CPU. Async code can improve concurrency by yielding during I/O without making
one operation's CPU work faster. Throughput can increase by batching while
individual items wait longer.

State the desired resource. A service constrained by CPU cost may reject a
change that improves latency through excessive parallelism. A latency-critical
batch may accept more CPU within capacity.

## Distributions and tails

Averages hide skew. A service with most requests at one millisecond and a small
group at one second can have an acceptable-looking mean but severe customer
impact. p50 describes central behavior; p95 and p99 expose progressively rarer
tails. Percentiles need sufficient samples and an explicit aggregation method,
especially across hosts or time windows.

Tail analysis should correlate with input size, tenant, cache state, queue wait,
retries, and downstream calls. Discarding outliers requires a methodological
reason; those observations may be the failure mode.

## Warm and cold systems

First-use costs include process initialization, page faults, DNS, connection
setup, TLS, allocator state, filesystem cache, and database plan/cache behavior.
Long-running steady-state services care about warm operation but also experience
deploy and failover cold starts. Command-line tools may be dominated by startup.

Measure the state the user experiences. If both matter, report both rather than
mixing an unspecified proportion.

## Allocation and retention

Source-level `clone` may increment a reference count, copy a small scalar, or
allocate and copy a large buffer. Conversely, eliminating a clone can extend a
borrow, retain a large backing buffer for one small slice, or introduce shared
locking. Allocation counts, allocated bytes, peak resident memory, and retained
memory answer different questions.

Arena and pool designs reduce allocation frequency but can increase peak memory
and latency spikes. Measure cleanup and long-lived fragmentation under realistic
lifetimes.

## Zero copy and lifetime cost

Data may be copied by kernel-to-user transfer, buffering, parsing, decoding,
normalization, ownership conversion, or serialization. A zero-copy parser might
avoid one application copy while requiring the entire input buffer to stay
alive. Scatter/gather I/O may avoid concatenation but complicate APIs and system
calls. DMA and kernel mechanisms have platform-specific boundaries.

Name the exact path and copy removed. Measure end-to-end effect and account for
retention, pinning, fragmentation, and caller ergonomics.

## Contention, queues, and backpressure

Increasing concurrency initially hides waiting, then saturates CPU, locks,
connections, database capacity, network, or a remote rate limit. Beyond that
point, queues increase latency and memory while throughput stays flat or falls.
A benchmark that measures only completed throughput and drops rejected work can
misrepresent service quality.

Sweep concurrency and batch size. Record queue depth, wait time, rejection,
timeouts, and downstream utilization. RUST-DOC-0004 requires bounded admission;
performance work must not remove the safety valve to inflate a benchmark.

## Boundary costs dominate often

Serialization can allocate, validate, and copy. Small database queries can incur
network round-trips and lock waits. Filesystem durability can require sync
operations. Logging can format and write synchronously. System calls and context
switches can dominate small computations.

Micro-optimizing an iterator has little effect if it represents one percent of
the profile. Batching queries or eliminating a round-trip may matter more, while
also changing consistency and error behavior that must be reviewed.

## Unsafe and correctness

Unchecked indexing can remove a branch in source yet produce no measurable gain
after compiler bounds-check elimination. Custom lock-free structures can
increase proof cost and regress under contention. SIMD may require alignment,
target detection, and fallback. Unsafe is justified only after a safe baseline,
material measured gain, and complete RUST-DOC-0007 evidence.

The benchmark itself cannot detect undefined behavior reliably and cannot prove
semantic equivalence. Correctness tests remain separate.

## Build-time performance

Heavy generics and macros can improve runtime but increase compile time,
monomorphized code, binary size, instruction-cache pressure, and diagnostics.
Feature unification can pull unused capabilities into tools. Dynamic dispatch
can reduce code size while adding an indirect call. Measure the dimension that
matters rather than applying slogans.

## Regression gates

Shared CI hosts are noisy. A strict one-percent wall-time gate may fail
randomly, training maintainers to rerun or ignore it. Stable metrics such as
binary bytes or allocation counts can support tight thresholds. Timing gates
need controlled hardware, historical variance, sufficient samples, and a
threshold above noise. Trend reports may be better on ordinary CI.

## Performance guarantee ledger

| Claim                            | Workload/environment                     | Established by                   | Does not prove           | Residual risk                   |
| -------------------------------- | ---------------------------------------- | -------------------------------- | ------------------------ | ------------------------------- |
| parser reduced allocation bytes  | named corpus, allocator, release build   | allocation profile and benchmark | lower end-to-end latency | corpus drift                    |
| batching raises throughput       | concurrency sweep, real database         | load test                        | improved p99 latency     | production query mix            |
| async version overlaps I/O       | executor trace and utilization           | integrated benchmark             | parallel CPU speedup     | runtime contention              |
| binary shrank                    | identical features/toolchain/target      | artifact measurement             | faster startup           | compression/deployment variance |
| unsafe path is materially faster | safe baseline and representative samples | profile plus benchmark           | soundness                | target and compiler changes     |

## Proportionality

Measurement has cost. A low-risk readability change need not create a laboratory
benchmark. A claim affecting capacity, infrastructure spend, user latency, or
unsafe design deserves reproducible evidence. Preserve raw results only as long
as useful and keep sanitized summaries reviewable. The desired outcome is not
maximum benchmark sophistication; it is a trustworthy decision.

---

## Source: `doctrines/0009-performance-and-measurement/decision-framework.md`

# Decision framework

## Define the objective

Record:

- user or system outcome;
- metric and unit;
- baseline commit;
- target or regression budget;
- representative input distribution;
- concurrency and batch range;
- correctness and resource constraints;
- deployment environment;
- decision deadline.

If no decision changes with the result, do not optimize yet.

## Choose observation

| Question               | Observation                                         |
| ---------------------- | --------------------------------------------------- |
| Where is CPU spent?    | sampled/instrumented CPU profile                    |
| What waits?            | async trace, span timing, syscall trace             |
| What allocates?        | allocation count/bytes and heap profile             |
| Why are tails slow?    | percentile trace correlated with inputs/queues      |
| Is a lock saturated?   | lock wait/hold profile and concurrency sweep        |
| Is storage dominant?   | query plan, round-trips, I/O and durability timing  |
| Is network dominant?   | request trace, payload size, retransmit/rate limits |
| What grows the binary? | section/symbol/generic analysis                     |
| What slows builds?     | clean/incremental timing and compiler timings       |

Collect a baseline before modifying code.

## Design the benchmark

1. Build with the intended profile and features.
2. Fix or record toolchain and target.
3. Select representative and adversarial inputs.
4. Decide whether setup and allocation belong in the measured operation.
5. make input unavailable for unintended constant folding;
6. consume output;
7. define warmup and cache state;
8. collect enough samples for expected variance;
9. record environment noise and thermal/frequency policy;
10. retain commands and summarized data.

Validate benchmark outputs against correctness tests.

## Interpret changes

Ask:

- Is the difference larger than measured noise?
- Is it large enough to matter to the objective?
- Does the profile show the changed code contributes enough?
- Did total CPU, memory, or downstream load worsen?
- Did p95/p99 change differently from the mean?
- Did input distribution or cache state shift?
- Did compiler output or features differ?
- Did correctness, failure, or backpressure behavior change?

If the benchmark improves but the integrated workload does not, narrow the
claim to the primitive or reject the complexity.

## Concurrency sweep

Measure at minimum:

```text
concurrency: 1 → nominal → saturation → overload
```

At each point capture throughput, p50/p95/p99, queue depth, wait time,
rejection, timeouts, CPU, memory, and downstream utilization. Sweep batch size
where batching changes. Choose capacity before the collapse region with
operational margin.

## Optimization choice

| Measured bottleneck    | Candidate direction                             | Correctness check            |
| ---------------------- | ----------------------------------------------- | ---------------------------- |
| algorithmic complexity | better data structure/algorithm                 | ordering, limits, worst case |
| allocation churn       | reuse, ownership change, compact representation | retention, aliasing          |
| serialization          | format/configuration/buffering                  | compatibility, validation    |
| syscall/round-trip     | batching or pipelining                          | partial failure, latency     |
| lock contention        | ownership partitioning or shorter scope         | invariant atomicity          |
| cache misses           | layout/locality change                          | representation validity      |
| monomorphization       | dispatch/API simplification                     | behavior and object safety   |
| compile time           | dependency/features/generic reduction           | runtime and diagnostics      |

## Unsafe gate

Proceed toward unsafe only if:

1. safe baseline is correct and profiled;
2. bottleneck is material;
3. safe alternatives were measured or rejected;
4. expected gain changes the objective;
5. safety invariant is reviewable;
6. Miri/sanitizer/target evidence is feasible;
7. fallback and re-audit triggers exist.

Otherwise keep the safe implementation.

## Regression strategy

Use a blocking threshold for stable allocation counts, artifact sizes, or
dedicated-host timings with measured variance. Use trend reporting for noisy
shared-host timings. Always retain correctness gates. A rerun policy must not
select only favorable samples.

## Stop conditions

Stop when:

- objective or workload is undefined;
- only debug-build or single-run numbers exist;
- profile contradicts the proposed bottleneck;
- benchmark result is unused or constant-folded;
- average hides material tail regression;
- local improvement increases downstream load;
- clone removal creates global sharing without evidence;
- zero-copy scope is unspecified;
- unsafe gain is immaterial;
- result depends on unrecorded environment differences;
- microbenchmark ratio is claimed end to end without contribution analysis.

---

## Source: `doctrines/0009-performance-and-measurement/review-standard.md`

# Review standard

Mark every gate **pass**, **fail**, **not applicable**, or an approved **waiver
reference**.

| Gate | Question                                      | Pass evidence             | Failure example                         | Severity | Remediation            |
| ---- | --------------------------------------------- | ------------------------- | --------------------------------------- | -------- | ---------------------- |
| M01  | Is objective quantified?                      | metric and target         | "make faster"                           | critical | define outcome         |
| M02  | Is workload representative?                   | input distribution        | tiny synthetic only                     | critical | sample/model reality   |
| M03  | Is concurrency specified?                     | range and nominal load    | single-thread claim generalized         | high     | sweep                  |
| M04  | Are correctness constraints fixed?            | invariant list            | errors dropped for speed                | critical | restore semantics      |
| M05  | Is baseline commit identified?                | SHA/config                | vague prior version                     | high     | record                 |
| M06  | Is toolchain recorded?                        | version/profile/target    | debug vs release comparison             | critical | rebuild comparably     |
| M07  | Are features identical?                       | feature manifest          | dependency feature changed              | high     | normalize              |
| M08  | Is hardware/OS recorded?                      | environment summary       | different machines                      | high     | control or qualify     |
| M09  | Is frequency/thermal state considered?        | policy/monitoring         | throttled run                           | high     | stabilize              |
| M10  | Is environment noise measured?                | repeated baseline         | one shared-host sample                  | high     | repeat/control         |
| M11  | Was profiling performed?                      | relevant profile          | bottleneck guessed                      | high     | profile                |
| M12  | Does profile support target?                  | cost attribution          | optimized cold code                     | critical | redirect               |
| M13  | Is profiler overhead considered?              | comparison                | tracing dominates                       | medium   | sample/qualify         |
| M14  | Is benchmark work retained?                   | result consumption        | computation optimized away              | critical | black-box/consume      |
| M15  | Are constants controlled?                     | dynamic inputs            | compiler precomputes                    | critical | vary input             |
| M16  | Is setup located correctly?                   | methodology               | input allocation accidentally timed     | high     | separate/define        |
| M17  | Is teardown excluded/included deliberately?   | scope                     | destructor cost omitted unintentionally | high     | align claim            |
| M18  | Is warmup documented?                         | preparation               | first and steady mixed                  | high     | separate               |
| M19  | Is cache state documented?                    | cold/warm method          | filesystem cache unknown                | high     | control                |
| M20  | Are connections reused as intended?           | setup trace               | handshake accidentally excluded         | high     | match workload         |
| M21  | Is sample count sufficient?                   | framework/statistics      | one timing                              | critical | collect samples        |
| M22  | Is variability reported?                      | CI/error/dispersion       | point estimate only                     | high     | report                 |
| M23  | Is practical significance assessed?           | objective delta           | tiny statistical win                    | medium   | simplify               |
| M24  | Are p50/p95/p99 present for latency?          | distribution              | average only                            | critical | measure tails          |
| M25  | Are outliers explained rather than discarded? | policy                    | slow samples deleted                    | high     | analyze                |
| M26  | Is wall-clock distinguished from CPU?         | named metrics             | parallel run called cheaper             | high     | measure both           |
| M27  | Is aggregate CPU captured for parallel work?  | process/thread CPU        | only elapsed time                       | high     | record resource        |
| M28  | Is throughput paired with latency?            | load curve                | batching throughput only                | high     | report distribution    |
| M29  | Is saturation point identified?               | sweep                     | nominal point only                      | high     | load to overload       |
| M30  | Is overload behavior preserved?               | rejection/queue data      | unbounded queue inflates throughput     | critical | restore backpressure   |
| M31  | Is queue wait included?                       | end-to-end latency        | service time only                       | high     | include ingress        |
| M32  | Is downstream load measured?                  | DB/API metrics            | local speed overloads dependency        | critical | coordinate             |
| M33  | Are lock wait and hold measured?              | contention profile        | mutex blamed by count                   | high     | instrument             |
| M34  | Are allocations counted?                      | count/bytes               | clone syntax used as proof              | high     | measure                |
| M35  | Is peak and retained memory considered?       | heap/RSS profile          | fewer allocs retain huge buffer         | high     | measure lifetimes      |
| M36  | Is allocator identified?                      | environment               | cross-allocator comparison              | medium   | record                 |
| M37  | Is copy claim scoped?                         | data-flow                 | "zero-copy" broad claim                 | critical | enumerate copies       |
| M38  | Are lifetime/retention costs assessed?        | ownership analysis        | slice pins large buffer                 | high     | compare total          |
| M39  | Are serialization costs profiled?             | component trace           | iterator optimized instead              | high     | target boundary        |
| M40  | Are syscalls/round-trips counted?             | trace                     | source CPU blamed                       | high     | measure system path    |
| M41  | Are database plans and locks considered?      | query evidence            | local benchmark excludes DB             | high     | integrate              |
| M42  | Are network limits/rate behavior included?    | load trace                | unlimited fake server                   | high     | use realistic boundary |
| M43  | Is async described accurately?                | overlap/CPU data          | async means parallel                    | critical | narrow claim           |
| M44  | Is clone removal architecturally safe?        | ownership/contention      | introduces global mutex                 | critical | redesign/measure       |
| M45  | Does algorithmic complexity improve?          | input-size curve          | lower constant, worse growth            | high     | test sizes             |
| M46  | Are worst-case inputs represented?            | adversarial corpus        | average-only parser                     | high     | add                    |
| M47  | Does unsafe satisfy doctrine 0007?            | proof and tools           | unchecked indexing for tiny gain        | critical | remove/review          |
| M48  | Is unsafe benefit material?                   | safe baseline comparison  | no measurable difference                | critical | keep safe              |
| M49  | Are platform fallbacks measured?              | target matrix             | SIMD only one CPU                       | high     | test dispatch          |
| M50  | Is binary size measured where affected?       | artifact data             | generics assumed free                   | medium   | inspect                |
| M51  | Is compile time measured where affected?      | clean/incremental data    | macro cost ignored                      | medium   | time builds            |
| M52  | Is monomorphization assessed?                 | symbol/code-size evidence | generic explosion                       | medium   | simplify               |
| M53  | Is benchmark separate from correctness tests? | suite link                | benchmark is sole check                 | critical | add tests              |
| M54  | Are fault/error paths still tested?           | negative suite            | fast path bypasses validation           | critical | restore coverage       |
| M55  | Is regression metric stable enough?           | history/variance          | noisy shared runner hard gate           | high     | trend/dedicated host   |
| M56  | Is threshold above noise and meaningful?      | rationale                 | arbitrary one percent                   | high     | calibrate              |
| M57  | Does rerun policy avoid cherry-picking?       | aggregate policy          | keep fastest rerun                      | critical | predefine method       |
| M58  | Are commands reproducible?                    | checked-in harness/docs   | manual GUI steps only                   | high     | script                 |
| M59  | Are results retained with provenance?         | record/raw format         | PR says "much faster"                   | high     | attach evidence        |
| M60  | Is claim no broader than evidence?            | guarantee ledger          | microbench generalized                  | critical | narrow                 |

Critical failures block the performance claim and any complexity justified by
it. Correctness failures block the change itself.

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0009-R001`, `RUST-DOC-0009-R002`, `RUST-DOC-0009-R003`, `RUST-DOC-0009-R004`
- `RUST-DOC-0009-R005`, `RUST-DOC-0009-R006`, `RUST-DOC-0009-R007`, `RUST-DOC-0009-R008`
- `RUST-DOC-0009-R009`, `RUST-DOC-0009-R010`, `RUST-DOC-0009-R011`, `RUST-DOC-0009-R012`
- `RUST-DOC-0009-R013`, `RUST-DOC-0009-R014`, `RUST-DOC-0009-R015`, `RUST-DOC-0009-R016`
- `RUST-DOC-0009-R017`, `RUST-DOC-0009-R018`, `RUST-DOC-0009-R019`, `RUST-DOC-0009-R020`

---

## Source: `doctrines/0009-performance-and-measurement/anti-patterns.md`

# Anti-pattern catalogue

## Rust is fast

**Weak example.** A design claims acceptable performance because it is written
in Rust.

**Why it fails.** language capability does not define algorithm, workload,
allocation, I/O, or contention.

**Risk.** untested capacity and latency.

**Improved direction.** define objective, profile, and measure representative
work.

**When justified.** Rust features can explain mechanisms after measurement, not
replace it.

## Optimize by inspection

**Weak example.** Code with clones or iterator adapters is rewritten without a
profile.

**Why it fails.** compiler optimization or dominant I/O can make the change
irrelevant.

**Risk.** complexity with no material benefit.

**Improved direction.** profile cost and retain a clear baseline.

**When justified.** A clear algorithmic complexity defect may be corrected and
then measured.

## One stopwatch run

**Weak example.** A command is timed once before and after on a busy laptop.

**Why it fails.** noise, cache, frequency, and background work dominate.

**Risk.** random variation becomes architecture.

**Improved direction.** use repeated controlled measurements and report
variability.

**When justified.** A single run can orient exploration but cannot support a
claim.

## Debug-versus-release comparison

**Weak example.** old code runs in debug and new code in release.

**Why it fails.** configuration, not implementation, explains the result.

**Risk.** invalid decision.

**Improved direction.** compare identical profiles, features, toolchains, and
targets.

**When justified.** Comparing build profiles is valid when profile choice is the
actual subject.

## Average latency only

**Weak example.** Mean request time improves while tail samples are omitted.

**Why it fails.** queueing or rare inputs can harm users severely.

**Risk.** hidden p99 regression.

**Improved direction.** report distributions correlated with workload.

**When justified.** Near-deterministic operations may summarize narrowly after
variance evidence.

## Throughput at any cost

**Weak example.** Batch size increases until maximum throughput, ignoring item
wait and memory.

**Why it fails.** queueing shifts cost into latency and resource retention.

**Risk.** deadline misses and overload collapse.

**Improved direction.** sweep batch/concurrency and record latency, queue,
memory, and rejection.

**When justified.** Offline throughput-only jobs can prioritize aggregate
completion while bounding resources.

## Async means parallel

**Weak example.** An async rewrite is expected to speed CPU-heavy work.

**Why it fails.** cooperative concurrency may remain on one worker and adds
scheduling overhead.

**Risk.** worse latency and executor starvation.

**Improved direction.** isolate CPU work, measure parallel execution, and bound
concurrency.

**When justified.** Async can improve overlap of waiting operations.

## Clone-count optimization

**Weak example.** Removing `.clone()` introduces shared ownership and locking
without measuring the clone.

**Why it fails.** clone cost varies, and longer sharing can be worse.

**Risk.** contention, retention, and complex lifetimes.

**Improved direction.** measure allocations/bytes and compare ownership
architectures.

**When justified.** A proven large copy on the hot path may deserve redesign.

## Zero-copy slogan

**Weak example.** A borrowed parser is marketed as zero-copy though
normalization and output serialization still copy.

**Why it fails.** scope and retained buffers are hidden.

**Risk.** misleading API and higher peak memory.

**Improved direction.** enumerate avoided copies and lifetime costs.

**When justified.** Use the term only for a precisely defined path with
evidence.

## Microbenchmark victory

**Weak example.** A primitive becomes twice as fast but occupied 0.2 percent of
request time.

**Why it fails.** end-to-end impact is below relevance.

**Risk.** maintenance cost without user benefit.

**Improved direction.** connect profile share to integrated measurement.

**When justified.** Library users may value the exact primitive claim; keep it
narrow.

## Unsafe for theoretical speed

**Weak example.** Checked indexing is replaced by raw pointers without showing
a measurable bottleneck.

**Why it fails.** compiler may remove checks; proof risk remains.

**Risk.** undefined behavior for no gain.

**Improved direction.** measure safe baseline, try safe structure changes, then
apply RUST-DOC-0007 if material.

**When justified.** Material target-workload gain plus complete safety evidence.

## Noisy hard gate

**Weak example.** Shared CI fails on a tiny timing regression, so maintainers
rerun until green.

**Why it fails.** gate measures host noise and selects favorable samples.

**Risk.** distrust and missed real regressions.

**Improved direction.** calibrate variance, use a dedicated host, raise
threshold, or report trends.

**When justified.** Stable metrics with justified thresholds make good gates.

---

## Source: `doctrines/0009-performance-and-measurement/glossary.md`

# Glossary

**Allocation profile**
: Measurement of allocation count, bytes, lifetime, and call sites under a
defined workload.

**Benchmark workload**
: The input distribution, concurrency, state, and boundary operations exercised
by a measurement.

**Cold state**
: Execution before relevant process, connection, page, filesystem, data, or
instruction caches are populated.

**Flamegraph**
: Aggregated sampled stack visualization showing where observed time or samples
accumulated.

**Latency percentile**
: Value below which a stated proportion of observed latencies falls, calculated
with a specified aggregation method.

**Monomorphization**
: Generation of concrete code instances for generic uses, affecting
optimization, compile time, and binary size.

**Practical significance**
: Whether a measured difference is large enough to affect the stated objective,
distinct from statistical detectability.

**Saturation**
: Load point where a limiting resource is fully utilized and additional demand
primarily increases waiting, rejection, or collapse.

**Steady state**
: Measurement period after intended initialization and warmup conditions have
stabilized.

**Zero-copy**
: A scoped claim that specified data copies do not occur along a defined path;
it is not a universal property of a system.

---

## Source: `doctrines/0009-performance-and-measurement/references.md`

# References

- [Criterion.rs book](https://bheisler.github.io/criterion.rs/book/) documents
  statistical benchmarking, measurement, comparison, and common methodology.
- [Rust `std::hint::black_box`](https://doc.rust-lang.org/std/hint/fn.black_box.html)
  documents an optimization barrier useful in benchmarks and its limitations.
- [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
  define optimization, debug information, code generation, and related build
  settings that must be recorded.
- [rustc performance data](https://perf.rust-lang.org/) and the
  [rustc-perf repository](https://github.com/rust-lang/rustc-perf) demonstrate
  controlled compiler performance tracking.
- [Linux `perf` documentation](https://perf.wiki.kernel.org/index.php/Main_Page)
  describes system profiling facilities commonly used for CPU and hardware
  events.
- [Brendan Gregg's FlameGraph repository](https://github.com/brendangregg/FlameGraph)
  documents the original stack-collapse and flamegraph tooling.
- [Tokio runtime metrics documentation](https://docs.rs/tokio/latest/tokio/runtime/struct.RuntimeMetrics.html)
  provides runtime-specific observations for async scheduling and queues.

Third-party tools establish their own measurement mechanics. This doctrine adds
workload, provenance, correctness, scope, regression, and guarantee-honesty
requirements.

---

## Source: `doctrines/0010-staged-protocols/README.md`

---
id: RUST-DOC-0010
slug: staged-protocols
title: Staged Protocols and Successor Capabilities
status: active
version: 0.2.0
normative: true
applies_to:
  - planning
  - implementation
  - review
  - audit
risk_domains:
  - protocol-design
  - state-machines
  - api-design
  - persistence
supersedes: []
superseded_by: null
---

# Staged Protocols and Successor Capabilities

## Scope

A staged protocol is an in-process sequence in which each stage establishes a fact that later
stages depend on: canonicalize, then check, then authorize, then prepare, then hand off. This
doctrine governs how such a protocol is represented so that its ordering is enforced rather than
remembered.

Its distinctive concern is the protocol edge. A stage capability names its legal successor as an
associated type bounded by the capability that successor must satisfy. The edge therefore lives
in the contract, and a stage that stops leading anywhere legal fails to compile. The doctrine
also governs branch and recovery edges, per-stage failure identity, stage granularity, effect
disclosure, the boundary at which the protocol may be erased, and the point at which a local
transition stops being evidence of a durable one.

## Out of scope

It does not govern value validation or newtype construction, which belong to RUST-DOC-0001. It
does not design an error taxonomy, which belongs to RUST-DOC-0002. It does not define custody,
capability issuance, or revocation, which belong to RUST-DOC-0003. It does not define
cancellation mechanics, which belong to RUST-DOC-0004. It does not govern durable decoding,
migration, or transactions, which belong to RUST-DOC-0005, nor distributed ambiguity and
reconciliation, which belong to RUST-DOC-0006. It states no soundness obligation, which belongs
to RUST-DOC-0007, defines no evidence class, which belongs to RUST-DOC-0008, and makes no cost
claim, which belongs to RUST-DOC-0009.

It does not claim that every ordered sequence deserves a stage type, and it does not make a
typed protocol a substitute for a durable workflow engine.

## Intended readers

Planners inventory stages, edges, evidence, and effects before types exist. Implementers build
the capability traits, the successor bounds, and the topology assertion. Reviewers test whether
the documented graph is the compiled graph and whether each stage claims only what its
construction establishes. Auditors search for conversion bypasses, forged stage evidence, and
local transitions presented as durable ones. Maintainers keep stage identity stable across
versions.

## Normative status

`doctrine.md` is normative and carries the stable rule identifiers. This package is version
0.2.0 with status active. Rationale, decision framework, anti-patterns, glossary, and references
are informative and cannot create an obligation that `doctrine.md` does not state.

Rules `RUST-DOC-0010-R012`, `RUST-DOC-0010-R016`, and `RUST-DOC-0010-R019` permit a waiver on
the terms recorded in the normative waiver section. The rules governing successor bounds,
construction bypass, durable claims, the guarantee ledger, terminology, and the authority
partition do not.

`RUST-DOC-0010-R022` was restated in repository version 0.4.0 under RFC-0003. It keeps its
identifier and its non-waivable status, and it now states an authority partition rather than a
blanket governance precedence. RUST-DOC-0011 governs that partition generally.

## Prerequisite foundations

Read [normative language](../foundations/normative-language.md) for requirement levels and
waiver structure, [invariants](../foundations/invariants.md) for classifying the facts stages
prove, [evidence](../foundations/evidence.md) for what each evidence class establishes,
[guarantee honesty](../foundations/guarantee-honesty.md) for the ledger discipline used
throughout, and [complexity budget](../foundations/complexity-budget.md) for the granularity
assessment required by `RUST-DOC-0010-R012`.

## Related material

Patterns: [successor capabilities](../patterns/successor-capabilities.md) is the mechanism
this doctrine governs; [typestate](../patterns/typestate.md) and
[consuming transitions](../patterns/consuming-transitions.md) are its foundation;
[sum types](../patterns/sum-types.md) carry its branches;
[hybrid state machines](../patterns/hybrid-state-machines.md) carry the durable half that
`RUST-DOC-0010-R015` requires; and
[executable narrative](../patterns/executable-narrative.md) is the general form of the
placement question `RUST-DOC-0010-R022` answers for stages.

Doctrines: [RUST-DOC-0011](../doctrines/0011-executable-narrative/) owns the authority partition,
prohibits a competing manually maintained copy of an enforced claim, and governs when a decision
record is justified. `RUST-DOC-0010-R022` is its application to staged protocols.

Boundaries: [HTTP and RPC](../boundaries/http-and-rpc.md) for the untrusted input that enters
the first stage, [database decoding](../boundaries/database-decoding.md) for restoration, and
[messaging](../boundaries/messaging.md) for published effects.

Reviews: [typestate review](../reviews/typestate-review.md) covers proportionality and adds a
staged-protocol gate group. Case studies:
[registration onboarding](../case-studies/registration-onboarding/) is the worked protocol;
[payment lifecycle](../case-studies/payment-lifecycle/) and
[authenticated session](../case-studies/authenticated-session/) show the durable and authority
halves this doctrine defers.

Executable evidence lives in [`examples/staged-protocol`](../examples/staged-protocol/src/lib.rs)
with compiler-rejection cases under [`examples/compile-fail/ui/`](../examples/compile-fail/ui/).

## Reading order

Start with this file for scope, then `doctrine.md` for the obligations. Read `rationale.md` for
the failure modes and the guarantee ledger, then `decision-framework.md` before committing to a
stage graph. Use `review-standard.md` during review, `anti-patterns.md` when a design feels
close to one of the known failures, `glossary.md` for terms whose local meaning is narrower than
ordinary usage, and `references.md` for provenance.

## Compact doctrine summary

Inventory the protocol before typing it. Name each stage for the fact it proves. Put the legal
successor in the contract as an associated type bounded by the next capability, and never widen
that bound to make an implementation compile. Consume the stage on transition, carry forward
exactly the evidence successors need, and keep each failure identifiable by stage. Model
branches as named alternatives over distinct successors, and name every retry and recovery edge.
Allow no conversion, derive, or public constructor that produces a later stage without its
transition, and restrict and inventory the trusted paths that remain. Disclose durable and
external effects per stage. Erase the protocol only at a named boundary.

The central non-guarantee: reaching a later stage proves that the in-process protocol ran in
order, and nothing more. A move consumes a local value; stored facts are read, copied, and
replayed, so no local move consumes them. Durable advancement re-checks identity, stored state,
and a concurrency token, and persisted lifecycle stays a runtime model.

## Package completion check

- metadata agrees with `manifest/doctrines.yaml` and its JSON Schema;
- rule IDs use `RUST-DOC-0010-RNNN` and every one appears in `review-standard.md`;
- all eight files carry domain-specific substance;
- references and source notes separate external facts from repository governance, and record
  which vocabulary is local;
- the example crate, its topology assertion, and the compiler-rejection cases are linked;
- generated bundles reproduce after the manifest update.

---

## Source: `doctrines/0010-staged-protocols/doctrine.md`

# Normative doctrine

## RUST-DOC-0010-R001 — Inventory the protocol before typing it

**Statement.** A staged protocol MUST have a written inventory of stages, edges, the evidence
each transition establishes, its failure classes, and its external effects before stage types
or capability traits are introduced.

**Intent.** Prevent a type graph from being derived mechanically from existing functions rather
than from the proof boundaries the domain actually has.

**Applicability.** Multi-stage command, request, submission, handshake, and workflow protocols
whose stage order carries consequence.

**Allowed exceptions.** A single-transition operation MAY record the inventory inline with its
design note.

**Review evidence.** Stage and edge inventory, evidence-per-transition table, and the design
note that preceded the types.

## RUST-DOC-0010-R002 — Name each stage by the fact it proves

**Statement.** A stage type MUST be named for the fact its construction establishes, and MUST
NOT be named for its position, its processing step, or a version counter.

**Intent.** Keep the stage graph readable as a sequence of proofs rather than an ordering of
implementation steps.

**Applicability.** Every named stage type and type-level state marker in a staged protocol.

**Allowed exceptions.** None. A stage whose proven fact cannot be named is evidence that the
boundary is not a real one.

**Review evidence.** Stage names, their documented guarantees, and the guarantee ledger.

## RUST-DOC-0010-R003 — Expose the successor capability in the stage contract

**Statement.** A stage capability whose protocol has a legal successor MUST expose that
successor as an associated type bounded by the capability the successor is required to satisfy,
rather than returning an unconstrained generic, an erased type, or a value whose successor
relationship exists only in prose.

**Intent.** Make the protocol edge a checked part of the contract, so a stage that stops leading
anywhere legal fails to compile instead of failing in review.

**Applicability.** Capability traits for staged protocols with more than one transition.

**Allowed exceptions.** A terminal stage MUST NOT name a successor. A protocol with exactly one
transition MAY return a concrete successor type directly when no second implementation is
anticipated.

**Review evidence.** Trait definitions, associated-type bounds, and the topology assertion
required by RUST-DOC-0010-R019.

## RUST-DOC-0010-R004 — Bound the successor by capability actually established

**Statement.** A successor bound MUST name only capabilities the successor value genuinely
establishes, and MUST NOT be widened, relaxed, or removed in order to make an implementation
compile.

**Intent.** Prevent the protocol contract from being edited to match a convenient
implementation, which converts a compile-time guarantee into decoration.

**Applicability.** Every associated successor type and its bounds.

**Allowed exceptions.** None. A bound that cannot be satisfied indicates the stage graph or the
implementation is wrong, not the bound.

**Review evidence.** Bound change history, the reason each bound exists, and the review record
for any relaxation.

## RUST-DOC-0010-R005 — Consume the stage on transition

**Statement.** A stage transition MUST consume the stage value when reuse of the prior stage
would be invalid, and MUST NOT rely on an internal flag to mark the stage as advanced. A stage
whose protocol claims single progression MUST NOT be duplicable, so it MUST NOT implement or
derive `Clone` or `Copy` unless duplicate progression is deliberately permitted and documented.

**Intent.** Make the successor value the evidence that the transition ran, and make reuse of the
superseded stage a compiler error. Consumption alone is insufficient: a caller holding a
duplicable stage can copy it first and advance every copy, which satisfies the letter of a
consuming signature while defeating its purpose.

**Applicability.** Transitions between stages of a locally owned protocol, and the trait
implementations of every stage type and branch wrapper. RUST-DOC-0003 governs custody and
RUST-DOC-0001 governs legal transitions and the clone audit generally; this rule adds the
stage-to-stage obligation.

**Allowed exceptions.** A read-only inspection that establishes no new fact MAY borrow. A
failure proven to occur before any part of the transition MAY return the prior stage with its
error. A terminal stage with no successor MAY be duplicable, since duplicating it advances no
protocol.

**Review evidence.** Method receivers, recovery shapes, the derive and trait-implementation
audit for every stage type, and compile-fail cases for both consumed-stage reuse and stage
duplication.

## RUST-DOC-0010-R006 — Carry forward exactly the evidence successors need

**Statement.** A stage MUST carry the evidence its successors require, and MUST NOT retain a
superseded untrusted representation unless a named audit, diagnostic, or reconciliation
obligation requires it and the retained value is distinguishable from the canonical one.

**Intent.** Keep a later stage from re-deriving a fact, and keep a raw value from being mistaken
for a checked one after the stage that checked it.

**Applicability.** Stage payloads and the values transitions move between them.

**Allowed exceptions.** Audit, reconciliation, and error-reporting obligations MAY retain the
original input when it is separately named.

**Review evidence.** Stage fields, the field-provenance mapping, and tests that canonical values
survive every transition.

## RUST-DOC-0010-R007 — Keep stage failure distinguishable

**Statement.** Each **fallible** transition MUST expose a failure type that identifies the stage
that failed, and a protocol MUST NOT erase its stage failures into one opaque type before the
protocol completes. A transition that cannot fail MUST NOT declare a failure type it never
constructs.

**Intent.** Preserve which proof was not established, which is the information a caller needs to
choose between retry, revision, and abandonment, without spending that machinery on a state the
transition cannot reach.

**Applicability.** Failure types of stage transitions. RUST-DOC-0002 governs error taxonomy
design; this rule adds the stage-identity obligation inside a protocol. The second sentence
applies to any transition whose body has no failure path.

**Allowed exceptions.** A boundary adapter MAY map stage failures into one transport or
presentation error after the protocol completes. A transition that only rearranges evidence
already established, performs no I/O, and enforces no further condition MAY be infallible, as
RUST-DOC-0001-R013 permits for pure in-process operations; its signature then returns the
successor directly rather than a `Result`.

**Review evidence.** Per-stage failure types, the boundary mapping, tests asserting stage
identity is preserved, and, for each transition declared fallible, a test or code path that
constructs its failure.

## RUST-DOC-0010-R008 — Model material branches as named successor alternatives

**Statement.** A transition with materially different outcomes MUST return a named sum type over
distinct successor stages, and MUST NOT return one successor carrying optional fields that stand
in for a state that was never established.

**Intent.** Prevent a branch from degrading into a partially populated value that every later
stage must re-inspect.

**Applicability.** Approval, availability, eligibility, verification, and routing transitions.

**Allowed exceptions.** An outcome that changes no successor capability and no later obligation
MAY be represented as data on one successor.

**Review evidence.** Branch enum definitions, successor bounds per variant, and a test per
branch.

## RUST-DOC-0010-R009 — Name retry, revision, and recovery edges

**Statement.** A protocol that permits retry, revision, correction, or resumption MUST represent
each such path as a named stage and a named edge, and MUST NOT leave it implicit in caller
control flow.

**Intent.** Keep the recovery half of a protocol as visible and as reviewable as its success
path.

**Applicability.** Protocols with revisable input, contended identity, recoverable rejection, or
resumable interruption.

**Allowed exceptions.** A protocol whose only recovery is to restart from the initial stage MAY
state that explicitly instead of adding a stage.

**Review evidence.** Recovery stage types, the edges that reach them, and tests exercising each
recovery path.

## RUST-DOC-0010-R010 — Prohibit conversion paths that skip stages

**Statement.** A protocol MUST NOT expose a `From`, `Into`, `Default`, public constructor,
public field, or derived decoding path that constructs a later stage without performing the
intervening transitions.

**Intent.** Close the bypass that makes an otherwise sound stage graph decorative, since a
conversion that produces a later stage asserts every proof that stage represents.

**Applicability.** Trait implementations, constructors, field visibility, and derived
deserialization on stage types and stage evidence.

**Allowed exceptions.** A restricted trusted-construction path MAY exist under
RUST-DOC-0010-R011.

**Review evidence.** Trait implementation inventory, field visibility audit, derive audit, and
the evidence-forgery compile-fail case.

## RUST-DOC-0010-R011 — Restrict and inventory trusted stage construction

**Statement.** Any path that constructs a stage or its evidence without running the
corresponding transition MUST be visibility-restricted to a named owner, MUST be listed in the
guarantee ledger, and MUST state the obligation its caller assumes.

**Intent.** Keep necessary construction paths for testing, migration, and checked restoration
from becoming ambient protocol bypasses.

**Applicability.** Test builders, migration adapters, restoration services, and privileged
factories.

**Allowed exceptions.** None to omit the inventory. The path itself is permitted only with a
recorded owner and obligation.

**Review evidence.** Visibility, the escape-hatch inventory, and the caller obligation recorded
beside each path.

## RUST-DOC-0010-R012 — Keep stage granularity proportionate

**Statement.** A stage MUST correspond to a proof boundary rather than an implementation helper,
and the stage count SHOULD be justified against the complexity budget when the protocol exceeds
the size a reader can hold in one signature chain.

**Intent.** Prevent both directions of failure: one stage hiding several unrelated
responsibilities, and a stage per helper function.

**Applicability.** Protocol design and any change that adds or merges a stage.

**Allowed exceptions.** A regulated process MAY require a stage per externally mandated
checkpoint even when the engineering boundary is weaker.

**Review evidence.** Stage count, the proof each stage adds, the complexity-budget assessment,
and the rejected alternative granularity.

## RUST-DOC-0010-R013 — Disclose durable and external effects per stage

**Statement.** A transition MUST disclose the durable writes, external calls, and messages it
performs, and a transition named for a check, validation, or preparation MUST NOT perform a
durable write or publish a message.

**Intent.** Keep the collapsed call chain an accurate summary of what the protocol does, not
only of what it proves.

**Applicability.** Every transition in a protocol that touches storage, a network, a broker, or
a filesystem.

**Allowed exceptions.** A domain that genuinely defines one atomic operation MAY combine effects
under a name that says so.

**Review evidence.** Per-stage effect inventory, the transition names, and tests asserting that
effect-free stages perform no effect.

## RUST-DOC-0010-R014 — Do not present a local transition as a durable one

**Statement.** A consuming in-process transition MUST NOT be presented as evidence that a
durable or remote state change occurred, and a transition that advances authoritative state MUST
re-check the entity identity together with its stored state and a version, fence, or equivalent
concurrency token at the authoritative store.

**Intent.** Prevent the strongest available local guarantee from being read as a distributed
one. A move consumes a local value; stored facts are read, copied, and replayed, so no local
move can consume them.

**Applicability.** Protocols whose stages correspond to persisted lifecycle states, and any
mapping of a typed protocol onto database procedures or stored state.

**Allowed exceptions.** None for the claim. A protocol that never advances durable state states
that limit instead.

**Review evidence.** The authoritative-transition query or procedure, its concurrency token, the
guarantee ledger row separating local from durable proof, and competing-writer evidence.

## RUST-DOC-0010-R015 — Keep persisted or multi-actor lifecycle in a runtime model

**Statement.** Where protocol state is persisted, inspected heterogeneously, or advanced by more
than one actor, the durable model MUST be a runtime representation, and the typed stage protocol
MUST be scoped to one in-process pass that is issued by checked construction.

**Intent.** Keep a mechanism that is sound for a local sequence from being extended to a durable
lifecycle it cannot govern.

**Applicability.** Registration, onboarding, payment, approval, fulfillment, and any workflow
with durable status and several participants.

**Allowed exceptions.** A protocol that runs entirely within one process and stores nothing MAY
omit the runtime model.

**Review evidence.** The persisted representation, the restoration path that issues a typed
stage, and the conversion contract between the two.

## RUST-DOC-0010-R016 — State the async stage contract

**Statement.** An asynchronous transition MUST state its cancellation behavior, whether retry is
safe, the identity under which a retry is deduplicated, and whether the successor proof exists
only after a durable acknowledgment.

**Intent.** Keep an interrupted transition from silently producing a successor whose proof was
never completed.

**Applicability.** Transitions that await I/O, cross a process boundary, or can be cancelled.
RUST-DOC-0004 governs cancellation mechanics; this rule requires the contract per stage.

**Allowed exceptions.** A transition that performs no external effect and holds no resource MAY
state that cancellation is inconsequential.

**Review evidence.** Per-stage cancellation table, idempotency identity, retry policy, and fault
tests at each interruption point.

## RUST-DOC-0010-R017 — Erase the protocol only at a named boundary

**Statement.** Type erasure of protocol state into trait objects, maps, dynamic contexts, or
serialized documents MUST occur at a named orchestration or persistence boundary, and MUST NOT
occur between stages.

**Intent.** Keep the stage graph checkable for its whole length, since an erased intermediate
value ends static enforcement for every stage after it.

**Applicability.** Orchestration layers, dynamic strategy selection, and persistence adapters.

**Allowed exceptions.** Runtime selection among protocol implementations MAY be dynamic while
each selected branch continues to advance through typed stages.

**Review evidence.** The named boundary, what is erased there, and the reason earlier erasure is
unnecessary.

## RUST-DOC-0010-R018 — Prove the prohibited orderings

**Statement.** Illegal stage orderings, reuse of a consumed stage, and construction of stage
evidence outside its transition MUST have compile-fail evidence when the protocol claims those
programs are unrepresentable.

**Intent.** Keep a claimed impossibility from silently becoming possible during refactoring.

**Applicability.** Every negative guarantee a staged protocol states.

**Allowed exceptions.** A prohibition enforced only at runtime MUST be stated as a runtime check
rather than given compile-fail evidence it does not have.

**Review evidence.** Compile-fail cases, their reviewed diagnostics, and confirmation that each
rejection occurs at the intended boundary.

## RUST-DOC-0010-R019 — Assert the stage graph executably

**Statement.** The stage and successor graph a protocol documents MUST be asserted executably,
so that a redirected associated type, a widened bound, or a removed implementation is detected
by the build rather than by reading. At least one assertion per capability MUST derive the
successor's required capability from the stage capability alone, and MUST NOT restate that
requirement as its own bound.

**Intent.** Keep the documented topology and the compiled topology from diverging, which is the
failure that prose review is least able to catch.

**Applicability.** Protocols with more than two stages or more than one branch.

**Allowed exceptions.** A protocol whose complete graph is visible in one function signature MAY
rely on that signature.

**Review evidence.** The contract assertions, the edge assertions, their coverage of every
documented edge, and an observed compiler failure when a successor bound is deleted from a
capability. An assertion whose own bounds restate the trait's obligation is not evidence for
this rule.

## RUST-DOC-0010-R020 — Record a guarantee ledger row per stage

**Statement.** Each stage MUST have a guarantee ledger row stating the claim it establishes, the
transition that establishes it, how its construction is protected, how boundary decoding
preserves it, its escape hatches, what it does not prove, and the residual runtime risk.

**Intent.** Keep the protocol's honesty auditable at the granularity at which its claims are
made.

**Applicability.** Every stage type and every piece of stage evidence.

**Allowed exceptions.** None.

**Review evidence.** The completed ledger and its agreement with the stage definitions.

## RUST-DOC-0010-R021 — Keep protocol terminology honest

**Statement.** Documentation for a staged protocol MUST NOT present project vocabulary as
standardized external terminology, and MUST identify the established family a mechanism belongs
to when it uses a local name for it.

**Intent.** Keep a useful local vocabulary from being cited as external authority it does not
have.

**Applicability.** Doctrine text, design notes, API documentation, and agent instructions that
name a protocol mechanism.

**Allowed exceptions.** Terms defined by a cited specification or published literature MAY be
used as standard when the citation is given.

**Review evidence.** Terminology definitions, their family attribution, and the source notes
recording which vocabulary is local.

## RUST-DOC-0010-R022 — Partition protocol authority explicitly

**Statement.** Each claim a staged protocol makes MUST be classified as an in-process claim the
executable protocol mechanically enforces, a durable or remote claim an external system owns, or
a rationale, non-guarantee, waiver, or change-authority claim its governing records own. The
executable protocol MUST be treated as authoritative for the ordering, successor constraints,
construction restrictions, and negative capabilities it mechanically enforces. An artifact
governing one of these classes MUST NOT be maintained as a second, independently edited source
for another class.

**Intent.** Replace a precedence contest with a partition. The accurate observation that code
enforces ordering does not make code the source of rationale, accepted risk, or change authority;
the accurate observation that doctrine governs change does not make doctrine a second description
of what the program currently permits. `RUST-DOC-0010-R018` and `RUST-DOC-0010-R019` exist
because prose cannot detect a widened bound or a redirected successor, and a rule subordinating
the executable protocol to prose would contradict them.

**Applicability.** Design notes, doctrine text, decision records, review records, and agent
instructions that state which artifact settles a question about a staged protocol. RUST-DOC-0011
governs the partition generally, including the decision-record obligations; this rule applies it
to stages, edges, and stage evidence.

**Allowed exceptions.** A generated or mechanically checked view of the executable protocol MAY
restate its topology, because such a view cannot drift from the artifact it is derived from.

**Review evidence.** The claim classification, the executable artifact cited for each in-process
claim, the external check cited for each durable claim, and the governing record cited for each
rationale, non-guarantee, waiver, or change-authority claim.

## Guarantee and non-guarantee requirements

A staged protocol states, for each stage and each piece of stage evidence: the claim its
construction establishes under RUST-DOC-0010-R002; how construction is protected under
RUST-DOC-0010-R010 and RUST-DOC-0010-R011; how decoding and restoration preserve or re-establish
it under RUST-DOC-0010-R015; its escape hatches under RUST-DOC-0010-R011; the external facts
that remain mutable under RUST-DOC-0010-R014; the failures that remain runtime failures under
RUST-DOC-0010-R007; the outcomes that can remain indeterminate under RUST-DOC-0010-R016; and the
executable evidence supporting the claim under RUST-DOC-0010-R018 and RUST-DOC-0010-R019.

## Boundary requirements

Untrusted input enters at the initial stage and is canonicalized under RUST-DOC-0010-R006 before
any stage claims a checked value. Persistence and wire boundaries follow RUST-DOC-0010-R015 and
RUST-DOC-0010-R017: durable state is a runtime model, erasure is named, and a typed stage is
issued only by checked construction. Durable advancement follows RUST-DOC-0010-R014 and re-checks
identity, stored state, and a concurrency token. Sensitive values carried as stage evidence
remain subject to RUST-DOC-0003 secret handling, and failure mapping at the outer boundary
follows RUST-DOC-0010-R007.

## Waiver requirements

RUST-DOC-0010-R012, RUST-DOC-0010-R016, and RUST-DOC-0010-R019 MAY be waived for a protocol
whose scope, lifetime, or effect makes the obligation disproportionate. A waiver records the
affected rule and protocol, the owner accepting the risk, the consequence, the compensating
control, an expiry or reconsideration trigger, and the removal condition.

RUST-DOC-0010-R003, RUST-DOC-0010-R004, RUST-DOC-0010-R010, RUST-DOC-0010-R011,
RUST-DOC-0010-R014, RUST-DOC-0010-R020, RUST-DOC-0010-R021, and RUST-DOC-0010-R022 MUST NOT be
waived. A waiver cannot make a bypassed protocol sound, cannot convert a local move into a
durable transition, and cannot make an inaccurate external claim true.

---

## Source: `doctrines/0010-staged-protocols/rationale.md`

# Rationale

## Failure modes

**The successor that quietly stopped leading anywhere.** A protocol declares four stages. During
a refactor the second stage's return type is changed from the third stage to a general-purpose
context value, because one caller needed to branch. Every call site still compiles; the chain
still reads like the business process; the ordering guarantee is gone. Nothing in a
documentation-only design detects this. `RUST-DOC-0010-R003` puts the successor in the contract
and `RUST-DOC-0010-R019` makes its removal a build failure.

**The bound widened to make the build pass.** An implementation cannot satisfy
`type Next: AcceptPolicy`, so the bound is relaxed to `type Next: Sized`. The edit is one line,
looks like a generics fix, and converts a compile-time protocol into a naming convention.
`RUST-DOC-0010-R004` makes the bound the fixed point and the implementation the thing that must
change.

**The conversion that skipped four stages.** A `From<Submission> for ApprovedRegistration`
implementation is added for a test fixture and later used in production code because it is
convenient. The type name still asserts approval; no approval occurred. This is the bypass that
makes an otherwise sound stage graph decorative, and it is why `RUST-DOC-0010-R010` prohibits
the conversion path outright while `RUST-DOC-0010-R011` requires the remaining trusted paths to
be restricted, owned, and inventoried.

**The branch that became optional fields.** An availability check returns one successor carrying
`Option<ExistingAccount>`. Every later stage re-inspects it, one of them forgets, and a
registration completes against a taken identity. `RUST-DOC-0010-R008` requires distinct
successor types so the conflicting path cannot be reached with the available path's evidence.
A third outcome belongs in the failure type rather than in a third variant of the branch, and
`RUST-DOC-0010-R007` keeps an undetermined check distinguishable from both branches.

**The check stage that wrote a row.** A transition named `validate` acquires an identifier and
inserts a reservation, because the identifier was needed downstream. The collapsed chain still
reads as validation. A cancelled request now leaves durable state. `RUST-DOC-0010-R013` makes
the disclosure an obligation and forbids the naming mismatch.

**The move that was read as a commit.** The strongest local guarantee available in Rust is that
a consumed value cannot be used again. A design maps stages onto persisted lifecycle states and
concludes that because the Rust value was consumed, the durable advance happened once. Stored
facts do not work that way: they are read, copied into a value, and can be read again by another
worker, so no local move consumes them. Two workers can each hold a consumed local handle for
the same row. `RUST-DOC-0010-R014` separates the two claims and requires identity, stored state,
and a concurrency token to be re-checked where durable state advances.
`RUST-DOC-0010-R015` keeps the durable model at runtime.

**The consuming transition defeated by a derive.** Every transition takes `self`, the
compile-fail case for reuse-after-move passes, and the design is described as permitting single
progression. Then a stage derives `Clone`, because some test wanted a copy. A caller now clones
the stage and advances both copies; the consuming signatures are all still there and all still
satisfied. This defect was live in this package's own example until review caught it, and the
committed compiler diagnostic for the reuse case even suggested `.clone()` as the workaround.
`RUST-DOC-0010-R005` therefore makes non-duplicability part of the obligation rather than a
consequence of it, and `RUST-DOC-0010-R018` requires a compile-fail case for duplication
separately from one for reuse.

**The topology assertion that asserted itself.** A helper is written to pin the stage graph:

```rust
fn assert_edge<S, N>() where S: Canonicalize<Next = N>, N: CheckIdentity {}
```

It compiles, it names every edge, and it detects nothing. The `N: CheckIdentity` bound in the
helper supplies exactly the constraint the trait is supposed to declare, so deleting
`type Next: CheckIdentity` from `Canonicalize` leaves the assertion green. This too was live in
this package until review compiled the library with the bound removed and observed a passing
suite. A contract assertion has to derive the successor capability from the trait alone:

```rust
fn assert_contract<S: Canonicalize>() {
    fn requires<T: CheckIdentity>() {}
    requires::<S::Next>();
}
```

Nothing here supplies the bound, so it fails the moment the trait stops declaring it. Edge
assertions remain useful for pinning the concrete successor; they are not a substitute.
`RUST-DOC-0010-R019` requires the contract form.

**The protocol erased in the middle.** An orchestration layer converts stage three into a
dynamic map so a plugin can inspect it, then converts back. Static enforcement ends at that
point for every later stage. `RUST-DOC-0010-R017` confines erasure to a named boundary while
still permitting dynamic strategy selection, so long as each selected branch continues through
typed stages.

**The vocabulary that borrowed authority.** A design note names a local mechanism, and a later
reader cites the name as established practice, treating a project convention as external
consensus. `RUST-DOC-0010-R021` requires the family attribution to travel with the local name.

**The authority nobody partitioned.** A protocol enforces ordering, and the argument then runs in
one of two directions, both wrong. In one, the accurate observation that code enforces ordering
grows into the claim that code is the whole contract, so review evidence, guarantee ledgers, and
the decision process become optional; code enforces what it enforces, and records neither why the
ordering was chosen, nor what the stages deliberately do not prove, nor who accepted the residual
risk. In the other, the accurate observation that doctrine governs change grows into a precedence
of prose over the compiled graph, which would contradict `RUST-DOC-0010-R018` and
`RUST-DOC-0010-R019`, both of which exist because prose cannot detect a widened bound.
`RUST-DOC-0010-R022` partitions the claims instead of ranking the artifacts, and RUST-DOC-0011
governs that partition generally. The earlier text of this rule asserted the second direction; the
restatement is recorded in RFC-0003.

## Why weaker alternatives fail

**Prose ordering.** A design document stating "authorize before capture" is readable and cheap.
It is also unenforced, and it goes stale silently: the document and the code diverge without any
signal. It remains the right choice when the sequence is advisory or when the states are
externally determined.

**A concrete successor return type.** Returning `Authorized` directly from `authenticate` is
genuine typestate and satisfies most of this doctrine. What it cannot express is one capability
with several implementations producing different successor evidence. A password login and an
invitation-based signup both need to reach the authorization stage while carrying different
proofs. Without an associated successor type, that requires either one widened successor
carrying both proofs as options, which reintroduces the optional-field failure, or a duplicated
protocol. This is the specific gap `RUST-DOC-0010-R003` fills, and it is why the doctrine exists
separately from `patterns/typestate.md`.

**A runtime state machine.** An enum with a `state` field and a `transition` method handles
dynamic, persisted, heterogeneous, and externally-determined state well, and it is the correct
choice for durable lifecycle. What it does not do is remove illegal calls from the API surface;
each method re-checks and each caller must handle a rejection that a typed protocol would have
made unrepresentable. `RUST-DOC-0010-R015` is the explicit instruction to use both: runtime for
the durable half, typed stages for the in-process pass.

**A middleware chain.** Ordering by registration position is flexible and composes well. It
proves nothing about what a downstream handler receives, and reordering two entries is a silent
behavioral change. It remains appropriate when the stages are genuinely independent and share no
evidence.

**Compile-fail tests alone.** Negative tests prove the specific programs written remain
rejected. They do not prove the graph is intact, because a redirected associated type can leave
every existing negative test passing while the edge it protected no longer exists. That
asymmetry is why `RUST-DOC-0010-R018` and `RUST-DOC-0010-R019` are separate obligations.

## Interaction with external reality

A stage type is local evidence with a timestamp. An availability observation records that no
conflicting account was visible to one reader at one moment; another writer can take the
identity immediately afterward. A consent proof records that an offered version matched the
version in force when the check ran; the policy can change before the record is written. A
prepared value records that the in-process protocol ran in order; it records nothing about
whether a durable write followed.

Asynchronous transitions add interruption. A transition cancelled after a remote effect was
accepted but before the successor was constructed leaves the external world advanced and the
local protocol not advanced. `RUST-DOC-0010-R016` requires that possibility to be stated per
stage rather than discovered, and where it matters the honest representation is an additional
stage for the interval whose outcome is unknown, which is the territory `RUST-DOC-0006` governs.

## Costs and overapplication

Capability traits with associated successor types make signatures longer and diagnostics
harder than a concrete return type; a mismatch is reported as an unsatisfied bound rather than a
plain type error. Generic stage types spread through helper functions, test harnesses, and mock
implementations. Each additional stage adds a type, a failure type, a ledger row, and a
topology assertion. Monomorphization grows with the product of stages and implementations.

The mechanism earns none of that when a protocol has two stages, when the sequence is advisory,
when states are chosen at runtime by external systems, when callers must hold heterogeneous
stages in one collection, or when the whole graph already fits in one function signature. A
three-line pipeline of ordinary functions is a better answer than seven traits, and
`RUST-DOC-0010-R012` exists to make that comparison mandatory rather than optional.

## Guarantee ledger

| Claim                                                   | Established by                                               | Protected construction                                                              | Boundary preservation                                      | Escape hatches                                   | Does not prove                                                                   | Residual runtime risk                                        |
| ------------------------------------------------------- | ------------------------------------------------------------ | ----------------------------------------------------------------------------------- | ---------------------------------------------------------- | ------------------------------------------------ | -------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| The in-process protocol ran in the documented order     | consuming transitions plus bounded associated successors     | private stage fields, no public stage constructor, no `Clone` on nonterminal stages | untrusted input canonicalized at the first stage           | restricted trusted construction under R011       | that any durable or remote effect occurred                                       | a stage reached through an unreviewed trusted path           |
| A stage's legal successor satisfies the next capability | associated-type bound checked by the compiler                | bound may not be widened under R004                                                 | restoration issues a typed stage only through checked code | none                                             | that the successor's evidence is externally current                              | a bound relaxed in a refactor without the topology assertion |
| Canonical values were established once                  | the canonicalization stage and its value constructors        | private newtype representations                                                     | raw input is dropped or separately named under R006        | audit retention named beside the canonical value | that the canonical form matches an external system's normalization               | divergent normalization policy between services              |
| A check observed no conflicting identity                | the identity-check transition against a directory read       | observation constructible only by that transition                                   | the read is a boundary observation, not a durable claim    | none                                             | that the identity is still free at write time                                    | a competing writer between observation and durable write     |
| Consent evidence corresponds to a checked version       | the policy transition comparing offered and required version | private evidence field, no public literal                                           | offered consent arrives as untrusted input                 | none                                             | that the policy version is still in force when the record is stored              | policy change between the check and the durable write        |
| The failing stage is identifiable                       | per-stage failure types under R007                           | failure types are not unified inside the protocol                                   | mapped to a transport error only at the outer boundary     | boundary adapter mapping                         | that the failure is recoverable, or that a retry is safe                         | a stage failure erased early by an over-eager adapter        |
| Durable state advanced exactly once                     | identity, stored state, and concurrency token re-checked     | the authoritative query or procedure                                                | the durable model is a runtime representation under R015   | administrative repair paths                      | that the local protocol observed the advance, or that no duplicate was attempted | lost update, stale read, or an unfenced competing writer     |
| The documented stage graph is the compiled graph        | the executable topology assertion under R019                 | assertion covers every documented edge                                              | assertion runs in the ordinary test suite                  | waiver under the normative waiver section        | that the graph is the right graph for the domain                                 | an edge added to the code and omitted from the assertion     |

## Evidence limits

Compiler rejection proves that the specific programs written are rejected at the pinned
diagnostic boundary, and nothing about programs nobody wrote. A topology assertion proves the
edges it names still typecheck; it does not prove the graph matches the business process, which
remains a review judgment under `RUST-DOC-0010-R001`. Unit tests over an in-memory collaborator
prove the transitions behave as written on the inputs supplied, and prove nothing about a real
directory, database, or broker.

No evidence in this repository establishes the durable half. The example crate deliberately
stops at a persistable value, so `RUST-DOC-0010-R014` and `RUST-DOC-0010-R015` are supported by
argument and review gates rather than by an executed database test. A consuming system supplies
its own competing-writer and fault evidence; this package does not claim it.

---

## Source: `doctrines/0010-staged-protocols/decision-framework.md`

# Decision framework

## Inputs

Bring the stage and edge inventory, the evidence each transition establishes, the failure
classes per transition, the external-effect inventory, the ownership map for the values being
advanced, the persistence model where any stage state is durable, the complexity budget, and the
evidence plan. A protocol cannot be assessed from its happy path alone.

## Questions

1. What consequential ordering is being protected, and what does the system do today when the
   order is violated?
2. Does each proposed stage establish a fact a later stage depends on, or is it a processing
   step that was convenient to name?
3. Is the sequence controlled by one owner within one process, or advanced by several actors
   against durable state?
4. Will one capability have several implementations producing different successor evidence?
5. Which transitions branch materially, which permit revision or retry, and which can end the
   protocol?
6. Which transitions perform durable writes, external calls, or message publication?
7. Where does untrusted input enter, and where must the protocol be erased for storage or
   dispatch?
8. What evidence would show the design is wrong, and what would show the graph has drifted?

## Decision table

| Situation                                                        | Preferred mechanism                                                 | Conditions                                                          | Stop condition                                                          |
| ---------------------------------------------------------------- | ------------------------------------------------------------------- | ------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| Advisory ordering with no shared evidence                        | ordinary functions in sequence                                      | violation is inconvenient rather than consequential                 | when a violation becomes a security, financial, or integrity fault      |
| Two-stage local sequence, one implementation                     | consuming transition returning a concrete successor                 | the successor never varies                                          | when a second implementation needs different successor evidence         |
| Multi-stage local sequence, one implementation per stage         | typestate with consuming transitions                                | the graph fits in signatures a reader can follow                    | when the successor relationship must be abstracted over implementations |
| Multi-stage sequence, several implementations per capability     | capability traits with bounded associated successor types           | successors differ in evidence but agree on the next capability      | when stages must be stored heterogeneously or inspected dynamically     |
| Materially different outcomes from one transition                | named sum type over distinct successor stages                       | each outcome changes the successor capability or a later obligation | when the outcome changes nothing downstream and is ordinary data        |
| Outcome that is neither success nor a modeled branch             | stage-identifying failure type                                      | availability, eligibility, or authority could not be determined     | when the third case is common enough to deserve its own stage           |
| Durable lifecycle advanced by several actors                     | runtime state model plus a typed pass issued by checked restoration | storage is authoritative and the typed protocol covers one pass     | when the typed protocol starts being treated as the durable record      |
| Runtime choice among protocol implementations                    | enum or dispatch at the selection point only                        | each branch continues through typed stages afterwards               | when the whole protocol is erased to accommodate one choice             |
| Stage state must be persisted, listed, or inspected across kinds | runtime enum with explicit operations                               | callers hold heterogeneous states together                          | when static enforcement is being simulated with runtime checks anyway   |

## Decision tree

```text
Is the ordering consequential when violated?
├─ no  → ordinary functions; record the sequence in the design note and stop
└─ yes → Does each stage establish a fact a later stage consumes?
   ├─ no  → merge the steps until they do; re-enter this tree
   └─ yes → Is the sequence advanced by one owner within one process?
      ├─ no  → runtime state model is authoritative (RUST-DOC-0005, RUST-DOC-0006)
      │        └─ is there also a local pass worth enforcing?
      │           ├─ no  → stop; runtime model only
      │           └─ yes → typed stages for the pass, issued by checked restoration
      └─ yes → How many transitions?
         ├─ one   → consuming transition with a concrete successor; stop
         ├─ two   → typestate with concrete successors unless a second
         │          implementation is already known
         └─ three or more → Will one capability have several implementations
                            producing different successor evidence?
            ├─ no  → typestate with concrete successors; revisit if that changes
            └─ yes → capability traits with bounded associated successors
                     ├─ add named sum types for material branches
                     ├─ add named stages for retry, revision, and recovery
                     ├─ add the topology assertion
                     └─ is the stage count still justifiable against the budget?
                        ├─ no  → merge to proof boundaries and re-enter
                        └─ yes → proceed to the evidence plan
```

The tree has two deliberate exits into simpler designs. A protocol that cannot answer the second
question is not a protocol, and a protocol whose stage count fails the budget check is expressing
implementation structure rather than proof structure.

## Complexity check

Count the stages, the capabilities, the implementations per capability, and the resulting
monomorphized combinations. Read one full transition signature aloud; if its bounds cannot be
followed, callers and mock authors will not follow them either. Check how far generic stage
parameters travel into helper functions, test harnesses, and public API boundaries, and whether
they can be stopped at an internal boundary.

Compare against the runtime alternative honestly: the same protocol as an enum with explicit
operations, and the same protocol as ordinary sequenced functions. Record what each alternative
would fail to prevent. If the answer is "nothing consequential," the simpler design wins.

Then check diagnostics. An unsatisfied successor bound is a worse first-encounter error message
than a plain type mismatch. If the protocol will be used mainly by people who did not write it,
that cost is real and belongs in the assessment.

## Evidence selection

| Decision                                 | Evidence class                                                        |
| ---------------------------------------- | --------------------------------------------------------------------- |
| Stage graph matches the documented graph | executable topology assertion over every edge                         |
| Illegal ordering is unrepresentable      | compile-fail case per claimed impossibility                           |
| Consumed stages cannot be reused         | compile-fail case on reuse after a consuming transition               |
| Stage evidence cannot be forged          | compile-fail case on literal construction of private evidence         |
| Each transition builds correct evidence  | unit test per transition, positive and negative                       |
| Branches produce the right successor     | unit test per branch variant                                          |
| Recovery edges re-enter correctly        | unit test per recovery path, including the terminal one               |
| Canonical values survive transitions     | unit test comparing first-stage input with terminal-stage output      |
| Effect-free stages perform no effect     | collaborator observation or fault injection asserting no write        |
| Cancellation behavior is as stated       | fault test interrupting each async transition                         |
| Durable advancement is exactly once      | competing-writer test against the real store, in the consuming system |
| Restoration issues a valid typed stage   | integration test over stored state, in the consuming system           |

The last two rows are deliberately assigned to the consuming system. This repository ships no
database or broker, so the doctrine states those obligations and the review gates check them,
but the executable evidence for them is not claimed here.

---

## Source: `doctrines/0010-staged-protocols/review-standard.md`

# Review standard

Mark every gate **pass**, **fail**, **not applicable**, or with an approved **waiver
reference**. Blank status is not approval.

## Protocol discovery and stage identity

| Gate | Question                                               | Pass evidence      | Failure example                          | Severity | Remediation               |
| ---- | ------------------------------------------------------ | ------------------ | ---------------------------------------- | -------- | ------------------------- |
| S01  | Does a stage and edge inventory exist?                 | inventory document | types derived from existing functions    | high     | write the inventory first |
| S02  | Does each transition name the evidence it establishes? | evidence table     | transition described only as a step      | high     | name the proof            |
| S03  | Are failure classes listed per transition?             | failure inventory  | one shared failure list                  | high     | separate by stage         |
| S04  | Are external effects listed per transition?            | effect inventory   | effects discovered during implementation | high     | complete the inventory    |
| S05  | Is each stage named for a proven fact?                 | names and claims   | a stage named for its position           | high     | rename to the proof       |
| S06  | Can every stage name be tied to a ledger claim?        | guarantee ledger   | a stage with no stated claim             | critical | state or delete the stage |
| S07  | Is any stage a renamed processing step?                | boundary rationale | a stage per helper function              | medium   | merge into a proof        |

## Successor capability and bounds

| Gate | Question                                                 | Pass evidence         | Failure example                      | Severity | Remediation                 |
| ---- | -------------------------------------------------------- | --------------------- | ------------------------------------ | -------- | --------------------------- |
| S08  | Does each nonterminal capability name a successor type?  | trait definitions     | successor stated only in prose       | critical | add the associated type     |
| S09  | Is the successor bounded by the next capability?         | associated-type bound | unconstrained generic successor      | critical | add the bound               |
| S10  | Is the successor relationship free of type erasure?      | signatures            | successor returned as a trait object | critical | keep the concrete relation  |
| S11  | Do terminal stages avoid naming a successor?             | trait definitions     | terminal stage points at itself      | medium   | mark the stage terminal     |
| S12  | Does each bound reflect capability actually established? | evidence mapping      | bound widened to compile             | critical | fix stage or implementation |
| S13  | Was any bound relaxed, and was the relaxation reviewed?  | change record         | silent bound removal in a refactor   | critical | restore or record           |
| S14  | Can two implementations produce different successors?    | implementation list   | successor hardcoded where it varies  | medium   | abstract the successor      |

## Transition, evidence, and failure

| Gate | Question                                                   | Pass evidence       | Failure example                             | Severity | Remediation                |
| ---- | ---------------------------------------------------------- | ------------------- | ------------------------------------------- | -------- | -------------------------- |
| S15  | Does each transition consume its stage where reuse is bad? | method receivers    | transition advances an internal flag        | critical | consume the stage          |
| S16  | Is a borrowing transition justified?                       | read-only rationale | borrowing chosen for caller convenience     | high     | consume or justify         |
| S17  | Is the prior stage returned only on proven non-transition? | recovery shape      | prior stage restored after a partial effect | critical | return an explicit outcome |
| S18  | Does each stage carry what its successors need?            | field mapping       | later stage re-derives a checked fact       | medium   | move the evidence forward  |
| S19  | Are superseded raw representations removed?                | field audit         | raw input kept beside canonical value       | high     | drop or name separately    |
| S20  | Is retained original input separately named?               | field names         | one field holds raw or canonical            | high     | split the fields           |
| S21  | Does each failure identify its stage?                      | failure types       | one opaque protocol error                   | high     | separate by stage          |
| S22  | Is failure erasure deferred to the boundary?               | mapping location    | stages erase failure immediately            | high     | map at the boundary        |
| S59  | Is every nonterminal stage non-duplicable?                 | derive audit        | stage derives `Clone`, so copies advance    | critical | remove the derive          |
| S60  | Does each declared failure type have a constructing path?  | test or code path   | infallible transition returns `Result`      | high     | make the signature honest  |

## Branches, recovery, and granularity

| Gate | Question                                                | Pass evidence      | Failure example                            | Severity | Remediation               |
| ---- | ------------------------------------------------------- | ------------------ | ------------------------------------------ | -------- | ------------------------- |
| S23  | Is each material branch a named sum over successors?    | branch enum        | one successor with optional fields         | critical | model the branch          |
| S24  | Does each branch variant carry its own successor bound? | variant bounds     | both branches share one capability         | high     | bound per variant         |
| S25  | Is an undetermined outcome distinct from both branches? | failure or outcome | undetermined treated as rejection          | critical | represent the third case  |
| S26  | Is each retry or revision path a named edge?            | recovery stage     | retry left to caller control flow          | high     | name the edge             |
| S27  | Does a revision edge re-enter at the correct stage?     | successor bound    | revision skips canonicalization            | critical | bound the re-entry        |
| S28  | Is a terminal recovery stage genuinely terminal?        | stage definition   | abandoned stage still exposes transitions  | medium   | remove the operations     |
| S29  | Is the stage count justified against complexity?        | budget assessment  | twenty stages for one request              | medium   | merge to proof boundaries |
| S30  | Does any stage hide unrelated responsibilities?         | effect inventory   | one stage validates, writes, and publishes | high     | split the stage           |

## Construction, bypass, and erasure

| Gate | Question                                                  | Pass evidence       | Failure example                        | Severity | Remediation             |
| ---- | --------------------------------------------------------- | ------------------- | -------------------------------------- | -------- | ----------------------- |
| S31  | Are stage fields private?                                 | visibility audit    | public field on a later stage          | critical | restrict visibility     |
| S32  | Is there a conversion that produces a later stage?        | implementation list | a conversion into an approved stage    | critical | delete the conversion   |
| S33  | Does any derive construct a stage without its transition? | derive audit        | derived decoding of stage evidence     | critical | route through the stage |
| S34  | Are trusted construction paths visibility-restricted?     | visibility          | public test builder in the shipped API | critical | restrict the path       |
| S35  | Is every trusted path in the escape-hatch inventory?      | ledger              | an undocumented factory                | critical | inventory or remove     |
| S36  | Does each trusted path state its caller obligation?       | obligation record   | path documented only as convenience    | high     | state the obligation    |
| S37  | Does erasure occur only at a named boundary?              | boundary record     | a map passed between stages            | critical | keep the types          |
| S38  | Does dynamic selection preserve typed progression?        | dispatch design     | whole protocol erased for one choice   | high     | erase only the choice   |

## Effects, durability, and asynchrony

| Gate | Question                                                    | Pass evidence        | Failure example                           | Severity | Remediation                 |
| ---- | ----------------------------------------------------------- | -------------------- | ----------------------------------------- | -------- | --------------------------- |
| S39  | Does each transition disclose its durable effects?          | effect inventory     | a check stage writes a row                | critical | disclose or move the effect |
| S40  | Do check and preparation stages perform no durable write?   | code trace and tests | validation publishes a message            | critical | separate the stages         |
| S41  | Is a local transition kept distinct from a durable one?     | ledger rows          | consumed handle presented as commit proof | critical | narrow the claim            |
| S42  | Does authoritative advancement re-check identity and state? | query or procedure   | update by identity alone                  | critical | add the state predicate     |
| S43  | Does it carry a version, fence, or equivalent token?        | concurrency token    | blind overwrite of durable state          | critical | add concurrency control     |
| S44  | Is persisted lifecycle modeled at runtime?                  | storage model        | stage marker persisted as protocol truth  | critical | persist a runtime state     |
| S45  | Does restoration issue a typed stage through checked code?  | restoration service  | stored tag deserialized into a stage      | critical | validate before issuing     |
| S46  | Is each async transition's cancellation behavior stated?    | cancellation table   | interruption behavior unexamined          | high     | state per stage             |
| S47  | Is retry safety and its identity stated?                    | idempotency identity | retry without a deduplication identity    | critical | define the identity         |
| S48  | Is a durable acknowledgment required before the successor?  | ordering evidence    | successor built before acknowledgment     | critical | reorder or split the stage  |

## Evidence, honesty, and governance

| Gate | Question                                                    | Pass evidence            | Failure example                       | Severity | Remediation              |
| ---- | ----------------------------------------------------------- | ------------------------ | ------------------------------------- | -------- | ------------------------ |
| S49  | Does each claimed impossibility have compile-fail evidence? | compile-fail cases       | claim stated only in prose            | high     | add the case             |
| S50  | Was each diagnostic inspected for its semantic cause?       | reviewed diagnostic      | fixture accepted mechanically         | high     | inspect and re-record    |
| S51  | Do the cases reject at the intended boundary?               | diagnostic analysis      | case fails for an unrelated reason    | high     | rewrite the case         |
| S52  | Is the documented stage graph asserted executably?          | topology assertion       | graph checked only by reading         | high     | add the assertion        |
| S53  | Does the assertion cover every documented edge?             | coverage comparison      | branch edges unasserted               | medium   | extend the assertion     |
| S54  | Does the assertion fail when an edge changes?               | deliberate break         | assertion passes after a redirect     | high     | strengthen the assertion |
| S55  | Does every stage have a guarantee ledger row?               | completed ledger         | evidence absent from the ledger       | critical | complete the ledger      |
| S56  | Does each row state what the stage does not prove?          | ledger column            | stage claims durable completion       | critical | narrow the claim         |
| S57  | Is local vocabulary distinguished from standard terms?      | terminology definitions  | a local coinage cited as established  | medium   | attribute the family     |
| S58  | Is each protocol claim assigned to exactly one authority?   | claim classification     | one artifact cited for every class    | critical | partition the claims     |
| S61  | Is the enforced artifact cited for in-process ordering?     | trait bounds cited       | doctrine quoted for what code permits | high     | cite the mechanism       |
| S62  | Does a hand-maintained prose copy of the graph exist?       | representation inventory | stage table beside the traits         | high     | generate or delete       |

## Outcome

Critical failures block merge. A valid waiver identifies the affected rule and protocol, the
owner accepting the risk, the consequence, the compensating control and its evidence, an expiry
or reconsideration trigger, and the removal condition. A waiver cannot make a bypassed protocol
sound, cannot convert a local move into a durable transition, and cannot make an inaccurate
external claim true. Remediation is verified by re-running the gate against the changed
artifact, not by asserting that the change was made.

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0010-R001`, `RUST-DOC-0010-R002`, `RUST-DOC-0010-R003`, `RUST-DOC-0010-R004`
- `RUST-DOC-0010-R005`, `RUST-DOC-0010-R006`, `RUST-DOC-0010-R007`, `RUST-DOC-0010-R008`
- `RUST-DOC-0010-R009`, `RUST-DOC-0010-R010`, `RUST-DOC-0010-R011`, `RUST-DOC-0010-R012`
- `RUST-DOC-0010-R013`, `RUST-DOC-0010-R014`, `RUST-DOC-0010-R015`, `RUST-DOC-0010-R016`
- `RUST-DOC-0010-R017`, `RUST-DOC-0010-R018`, `RUST-DOC-0010-R019`, `RUST-DOC-0010-R020`
- `RUST-DOC-0010-R021`, `RUST-DOC-0010-R022`

Gate groups map to rules as follows. S01 to S07 cover `RUST-DOC-0010-R001` and
`RUST-DOC-0010-R002`. S08 to S14 cover `RUST-DOC-0010-R003` and `RUST-DOC-0010-R004`. S15 to S22
cover `RUST-DOC-0010-R005`, `RUST-DOC-0010-R006`, and `RUST-DOC-0010-R007`. S23 to S30 cover
`RUST-DOC-0010-R008`, `RUST-DOC-0010-R009`, and `RUST-DOC-0010-R012`. S31 to S38 cover
`RUST-DOC-0010-R010`, `RUST-DOC-0010-R011`, and `RUST-DOC-0010-R017`. S39 to S48 cover
`RUST-DOC-0010-R013`, `RUST-DOC-0010-R014`, `RUST-DOC-0010-R015`, and `RUST-DOC-0010-R016`. S49
to S58 and S61 to S62 cover `RUST-DOC-0010-R018`, `RUST-DOC-0010-R019`, `RUST-DOC-0010-R020`,
`RUST-DOC-0010-R021`, and `RUST-DOC-0010-R022`. S59 and S60 sit with the transition group they
extend, covering `RUST-DOC-0010-R005` and `RUST-DOC-0010-R007`.

S58, S61, and S62 apply the authority partition to a staged protocol.
[The executable narrative review](../reviews/executable-narrative-review.md) is the general
procedure for the same question, and RUST-DOC-0011 carries the rules it operationalizes.

---

## Source: `doctrines/0010-staged-protocols/anti-patterns.md`

# Anti-pattern catalogue

## Chain without state change

**Weak example.**

```rust
impl Registration {
    fn canonicalize(self) -> Result<Self, Error> { /* ... */ }
    fn check_identity(self) -> Result<Self, Error> { /* ... */ }
    fn accept_policy(self) -> Result<Self, Error> { /* ... */ }
}
```

**Why it fails.** The chain reads like a protocol and enforces nothing. Every method is available
at every point, so `accept_policy` can run first and the compiler is content. The fluency is
real; the ordering guarantee is imaginary.

**Risk.** A reviewer reads the call site, sees the business sequence, and approves an API that
permits every other sequence.

**Improved direction.** Give each stage a distinct type whose construction proves the preceding
transition ran, and expose the successor through the stage contract.

**When justified.** When the operations genuinely are reorderable, in which case the sequence
should not be presented as a protocol at all.

## Successor named only in prose

**Weak example.**

```rust
pub trait CheckIdentity {
    /// Returns a value that can then accept policy.
    fn check_identity(self) -> Result<Box<dyn Any>, Error>;
}
```

**Why it fails.** The successor relationship exists in a doc comment. Nothing checks that the
returned value can accept policy, and the first refactor that returns something else compiles
cleanly.

**Risk.** The protocol graph degrades silently, and the failure surfaces at a call site far from
the change.

**Improved direction.** Name the successor as an associated type bounded by the next capability,
and assert the resulting graph executably.

**When justified.** Never for a stage with a legal successor. A terminal stage names no successor
because it has none.

## Bound widened to satisfy an implementation

**Weak example.**

```rust
pub trait Canonicalize {
    type Next; // was: type Next: CheckIdentity
}
```

**Why it fails.** The bound was the entire protocol guarantee. Removing it is a one-line edit
that looks like a generics simplification and reads as noise in a diff.

**Risk.** A stage graph that still has all its types, all its names, and none of its edges.

**Improved direction.** Treat the bound as fixed and change the implementation or the stage
design. Where a bound genuinely must move, record the reason and re-check the topology
assertion.

**When justified.** Only when the protocol itself was wrong, and then the change is a doctrine
decision rather than a refactor.

## Conversion that manufactures a later stage

**Weak example.**

```rust
impl From<RawSubmission> for AcceptedRegistration {
    fn from(raw: RawSubmission) -> Self { /* ... */ }
}
```

**Why it fails.** `AcceptedRegistration` asserts that canonicalization, an availability check,
and a policy check all succeeded. The conversion asserts all three without performing any. The
same failure arrives through a public constructor, a public field, or a derived decoder.

**Risk.** Every guarantee downstream of the stage becomes false while the type names continue to
claim it.

**Improved direction.** Remove the conversion. Where a trusted path is genuinely required, make
it visibility-restricted, give it an owner, state the obligation its caller assumes, and list it
in the guarantee ledger.

**When justified.** Never as an ambient conversion. A restricted, inventoried, owned
construction path for checked restoration or migration is a different mechanism with different
obligations.

## Branch flattened into optional fields

**Weak example.**

```rust
pub struct CheckedRegistration {
    conflict: Option<AccountId>,
    uniqueness: Option<UniquenessObservation>,
}
```

**Why it fails.** The type admits all four combinations, two of which are meaningless. Every
later stage must re-inspect and re-decide, and the first stage that forgets proceeds on a
conflicting identity.

**Risk.** A registration completes against an identity another account already holds.

**Improved direction.** Return a named sum type over distinct successor stages, each bounded by
the capability that outcome legitimately leads to.

**When justified.** When the outcome changes no successor capability and no later obligation, in
which case it is ordinary data rather than a branch.

## Undetermined outcome folded into a branch

**Weak example.**

```rust
match directory.lookup(&address) {
    Ok(Some(holder)) => conflicting(holder),
    _ => available(),
}
```

**Why it fails.** A directory that could not be reached is treated as proof that the identity is
free. The most dangerous outcome is silently mapped onto the most permissive one.

**Risk.** A protocol advances on evidence that was never obtained.

**Improved direction.** Keep the undetermined case in the stage-identifying failure type, and
carry enough identity for an operator to look the attempt up.

**When justified.** Never. If the undetermined case is common, it deserves its own stage rather
than a quieter default.

## Check stage with a durable effect

**Weak example.**

```rust
fn validate(self) -> Result<Validated, Error> {
    self.repository.insert_reservation(&self.id)?;
    // ...
}
```

**Why it fails.** The name says the stage establishes a fact about the input; the body changes
the world. A cancelled or failed request now leaves durable state that nothing in the collapsed
chain suggests exists.

**Risk.** Orphaned reservations, duplicate side effects on retry, and a cleanup path nobody
wrote because nobody knew it was needed.

**Improved direction.** Split into a stage that checks and a stage that writes, and disclose the
effect on the stage that performs it.

**When justified.** When the domain genuinely defines one atomic operation, in which case the
stage name says so.

## Local move presented as durable proof

**Weak example.**

```rust
// The handle was consumed, so the row advanced exactly once.
let receipt = pending.mark_paid()?;
```

**Why it fails.** Consuming a Rust value proves the caller cannot use that value again. It
proves nothing about a stored row, because a stored fact can be read again into a second value
by a second worker. Two workers can each hold a consumed handle for the same row.

**Risk.** Lost updates, duplicate durable transitions, and a concurrency bug whose type
signatures look impeccable.

**Improved direction.** Re-check identity, stored state, and a version or fencing token in the
authoritative statement, and keep the durable model at runtime. State the local and durable
claims as separate ledger rows.

**When justified.** Never for the claim. The consuming transition remains correct and useful as
a statement about local handle lifecycle.

## Protocol erased between stages

**Weak example.**

```rust
let context: HashMap<String, Value> = stage_three.into_context();
let stage_four = StageFour::from_context(&context)?;
```

**Why it fails.** Static enforcement ends at the map. Every stage after it is checked by
convention, and the round trip re-admits exactly the states the protocol was built to exclude.

**Risk.** The remaining stages carry the appearance of enforcement without the substance.

**Improved direction.** Keep the types through the protocol and erase once, at a named
orchestration or persistence boundary, after the stages that matter have run.

**When justified.** At the named boundary itself, and for runtime selection among
implementations where each selected branch continues through typed stages.

## Stage per helper function

**Weak example.**

```rust
raw.trim()?.lowercase()?.split()?.reassemble()?.validate()?.normalize()?
```

**Why it fails.** None of these establish a fact a later stage depends on; they are steps in one
transformation. The reader now navigates six types to follow one canonicalization.

**Risk.** The protocol becomes unreadable, and the genuine proof boundaries are lost among the
mechanical ones.

**Improved direction.** Merge until each stage corresponds to a proof, and assess the resulting
count against the complexity budget.

**When justified.** When an external mandate requires a checkpoint per step even though the
engineering boundary is weaker.

## Local vocabulary cited as external authority

**Weak example.** A design note introduces a coined name for a mechanism, and a later document
cites the name as established practice without attribution.

**Why it fails.** The name is a useful local shorthand. Treating it as external consensus
borrows authority the mechanism has not earned and makes the claim hard for a reader to check.

**Risk.** Review deference to a term rather than to the argument behind it.

**Improved direction.** Keep the local name, and state the established family it refines and the
citation for that family.

**When justified.** When the term is genuinely defined by a cited specification or published
literature, in which case the citation travels with it.

## Code offered as its own governance

**Weak example.** A protocol enforces ordering, and the design note concludes that documentation
of the ordering is therefore unnecessary and no rationale, ledger, or review evidence is required.

**Why it fails.** The code records what is enforced. It does not record why this ordering was
chosen over alternatives, what the stages deliberately do not prove, which residual risks were
accepted, or who accepted them. A future maintainer reading only the code can reconstruct the
mechanism and none of the reasoning.

**Risk.** A guarantee is weakened in a refactor because the reason it existed was never written
down.

**Improved direction.** Let the code be authoritative for the in-process ordering it enforces, and
keep the rationale, the guarantee ledger, and the review evidence beside it as the authority for
what the code does not carry.

**When justified.** Never as stated. The underlying observation, that an enforceable obligation
belongs in the mechanism that enforces it, is correct and is what `RUST-DOC-0010-R022` and
RUST-DOC-0011 already require.

## Doctrine offered as the account of current behavior

**Weak example.** A reviewer answers "can this transition run before that one" by quoting the
doctrine package, and a design note describes the stage graph in prose beside the traits that
enforce it.

**Why it fails.** This is the same error in the opposite direction. Doctrine states obligations;
what a protocol currently permits is decided by the bounds, and the two can diverge in either
direction without either artifact announcing it. The prose stage graph is additionally a second
editable source, so a refactor updates one of them.

**Risk.** A review that certifies the obligation and misses the violation, and a reader who plans
against a graph the compiler abandoned two releases ago.

**Improved direction.** Cite the trait bounds and the topology assertion for what the protocol
permits, cite doctrine for whether that is what it ought to permit, and either generate the prose
graph or delete it.

**When justified.** Never as a substitute. Doctrine remains the authority for the obligation, the
review process, and who may change the contract, which is the partition `RUST-DOC-0010-R022`
states.

---

## Source: `doctrines/0010-staged-protocols/glossary.md`

# Glossary

Terms whose meaning here is narrower than ordinary Rust or architecture usage. Shared vocabulary
lives in the foundations.

**Staged protocol**
: An in-process sequence in which each stage establishes a fact later stages depend on. Narrower
than "workflow": a workflow may be durable, distributed, and multi-actor, while a staged
protocol is one owner's pass through a sequence within one process.

**Stage**
: A distinct type whose construction proves that its preceding transition completed. Narrower
than "state": a stage carries the evidence of the transition that produced it, and a state may
be a mere marker.

**Transition**
: The operation that consumes one stage and produces its successor. A transition is a protocol
edge, not any method that happens to return a new value.

**Capability**
: A trait describing the operation legal at a stage, together with the successor that operation
produces. Distinguish from a capability in the authority sense governed by RUST-DOC-0003, where
possession conveys permission; a stage capability conveys position in a protocol, not authority.

**Successor capability**
: The bound placed on a capability's associated output type, naming what the next stage is
required to satisfy. This is the protocol edge in checkable form.

**Protocol edge**
: One legal transition from one stage to one successor. Edges include branch alternatives and
recovery paths, not only the success path.

**Stage evidence**
: A value constructible only by a specific transition, whose possession is proof that the
transition ran. Narrower than "data carried by a stage": ordinary payload is not evidence unless
its construction is protected.

**Protocol topology**
: The complete graph of stages and edges a protocol documents. The topology is asserted
executably so that documentation and compilation cannot diverge.

**Topology assertion**
: An executable check that each documented edge still typechecks. It proves the edges named
still exist; it does not prove the graph is right for the domain.

**Collapsed view**
: The call site read as a sequence of transitions. It summarizes the protocol and is not itself
evidence of anything.

**Expanded view**
: The stages, evidence, failures, branches, and effects the collapsed view abbreviates.

**Terminal stage**
: A stage with no legal successor, including a recovery stage that ends an attempt. A terminal
stage names no successor capability.

**Undetermined outcome**
: A transition result in which the fact could not be observed. It is distinct from both a
success branch and a modeled rejection, and it belongs in the stage-identifying failure type.

**Durable advancement**
: A change to authoritative stored state. Distinct from a local transition: a local transition
consumes a value, while durable advancement requires re-checking identity, stored state, and a
concurrency token. No local move consumes a stored fact.

**Trusted construction path**
: A restricted way to build a stage or its evidence without running the transition, used for
checked restoration, migration, or testing. It carries an owner, a stated caller obligation, and
a guarantee-ledger entry.

**Chainable Telescopic Typestate Traits (CT³)**
: Local project vocabulary for the mechanism this doctrine governs, recorded here so the term is
recognizable in older internal documents. It is not standardized external terminology. The
established families it refines are typestate-oriented programming, behavioral types, and object
protocols; the specific mechanism is a consuming transition with an associated successor type
bounded by the next capability. Each word carries part of the mechanism. _Typestate_: the current
type proves the current state. _Trait_: the stage exposes the behavioral capability legal at that
state. _Chainable_: the legal happy path composes into a readable sequence, the collapsed view.
_Telescopic_: a chain gives order, `A → B → C`, while a telescope gives containment — A holds the
controlled opening into B, and B holds the controlled opening into C. The associated successor
type is that opening, so a stage carries both proof of completed history and permission for a
constrained future. Prefer the descriptive terms above in new material, per
`RUST-DOC-0010-R021`.

**Authority partition**
: The assignment of each protocol claim to exactly one authority: the executable protocol for what
it mechanically enforces, an external system for a durable or remote fact, and a governing record
for rationale, non-guarantees, waivers, and change authority. Narrower than "precedence": the
partition does not rank artifacts, it assigns claims. `RUST-DOC-0010-R022` states it for staged
protocols and RUST-DOC-0011 governs it generally.

## Glossary review

- every normative term is defined in the foundations or here;
- no definition implies a stronger guarantee than construction establishes;
- local vocabulary is marked as local and attributed to its established family;
- observations record their scope and the moment they were taken;
- abbreviations expand on first use;
- links point to the authoritative rule or foundation.

---

## Source: `doctrines/0010-staged-protocols/references.md`

# References

Primary sources for the language mechanics this doctrine relies on, and the literature for the
families it refines. Each entry states what the source establishes; repository governance
supplies the obligations.

## Language mechanics

- [The Rust Reference: associated items](https://doc.rust-lang.org/reference/items/associated-items.html)
  — establishes associated types and the trait bounds that may be placed on them, which is the
  mechanism `RUST-DOC-0010-R003` requires for exposing a successor capability.
- [The Rust Reference: traits](https://doc.rust-lang.org/reference/items/traits.html) — establishes
  supertraits, `Self: Sized` requirements, and the conditions under which a trait is
  dyn-compatible, which bound what a stage capability can express.
- [The Rust Book: what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
  — establishes move semantics, the basis of the consuming transition in `RUST-DOC-0010-R005`
  and the exact limit of the claim in `RUST-DOC-0010-R014`.
- [The Rust Reference: visibility and privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html)
  — establishes the field and constructor privacy that makes stage evidence unforgeable under
  `RUST-DOC-0010-R010` and `RUST-DOC-0010-R011`.
- [The Rust Reference: generic parameters](https://doc.rust-lang.org/reference/items/generics.html)
  — establishes monomorphization, relevant to the cost assessment in `RUST-DOC-0010-R012`.
- [`trybuild`](https://docs.rs/trybuild) — the harness used for the compiler-rejection evidence
  required by `RUST-DOC-0010-R018`. Version scope: 1.0.118, checked 2026-08-04.

## Families this doctrine refines

- Strom and Yemini, "Typestate: A Programming Language Concept for Enhancing Software
  Reliability," _IEEE Transactions on Software Engineering_ SE-12(1), 1986 — establishes
  typestate as a compile-time discipline in which an object's legal operations depend on its
  state. It is the origin of the family; it does not define the trait-based successor mechanism.
- Aldrich, Sunshine, Saini, and Sparks, "Typestate-Oriented Programming," _OOPSLA Onward!_ 2009
  — establishes state as a first-class unit with state-specific members, and the framing of
  valid method-call sequences as an object protocol.
- Honda, Vasconcelos, and Kubo, "Language Primitives and Type Discipline for Structured
  Communication-Based Programming," _ESOP_ 1998 — establishes session types for communication
  protocols with dual participants. A close relative; the distinction is recorded in the source
  notes, and this doctrine governs an object's internal protocol rather than communication
  duality.

## Durability and concurrency

- [PostgreSQL: transaction isolation](https://www.postgresql.org/docs/current/transaction-iso.html)
  — establishes that concurrent readers observe snapshots and that serialization failures must be
  retried, which is why an observation taken during one stage does not remain true at write time.
- [PostgreSQL: MVCC introduction](https://www.postgresql.org/docs/current/mvcc-intro.html) —
  establishes that readers obtain a copy of committed state rather than exclusive custody of it.
  This is the mechanical basis of `RUST-DOC-0010-R014`: a Rust move consumes a local value, and
  a row read into that value can be read again by another worker.
- Gray and Cheriton, "Leases: An Efficient Fault-Tolerant Mechanism for Distributed File Cache
  Consistency," _SOSP_ 1989 — establishes time-bounded distributed authority, relevant to the
  fencing token required where durable advancement is contended. See also RFC-0001, which
  records the repository's accepted position on time-based authority.

## Attribution discipline

Quotations are short and used only where wording matters; sources are otherwise summarized and
linked. No external media, transcript, or specification text is mirrored here. Source notes
under [`sources/0010-staged-protocols/`](../sources/0010-staged-protocols/source-notes.md)
classify which ideas this package accepts, refines, rejects, and adds, and identify which
vocabulary is local to this repository rather than external.

References are informative. A normative obligation exists only where `doctrine.md` states it
with a rule identifier. Changing facts, including tool versions and product behavior, carry the
version or date checked and are rechecked when the package is maintained.

---

## Source: `doctrines/0011-executable-narrative/README.md`

---
id: RUST-DOC-0011
slug: executable-narrative
title: Executable Narrative and Minimal Decision Records
status: active
version: 0.1.0
normative: true
applies_to:
  - planning
  - implementation
  - review
  - audit
  - maintenance
risk_domains:
  - architecture-governance
  - documentation-drift
  - decision-records
  - agent-context
supersedes: []
superseded_by: null
---

# Executable Narrative and Minimal Decision Records

## Scope

An architectural obligation that a mechanism can enforce belongs in that mechanism. This doctrine
governs where an obligation lives, which artifact settles which class of claim, how a derived
view is kept from drifting, and when a manually maintained decision record earns its permanent
cost.

Its distinctive concern is the second copy. A type, a schema, a manifest, or a test changes when
the system changes and fails when it is contradicted. A hand-maintained description of the same
obligation changes only when someone remembers, survives the constraint that produced it, and is
still discoverable when a reader treats it as current authority. The doctrine names the artifact
that is authoritative for each class of claim, prohibits a competing editable copy, prefers
generation to synchronization, and makes a decision record the exception rather than the default
artifact.

## Out of scope

It does not decide which invariants a domain has, which belongs to RUST-DOC-0001, nor how errors
are modeled, which belongs to RUST-DOC-0002. It does not define custody or authority, which
belong to RUST-DOC-0003, or cancellation, which belongs to RUST-DOC-0004. It does not govern
durable decoding, migration, or transactions, which belong to RUST-DOC-0005, nor distributed
ambiguity, which belongs to RUST-DOC-0006. It states no soundness obligation, which belongs to
RUST-DOC-0007. It does not define evidence classes or their strength, which belong to
RUST-DOC-0008, and it makes no cost claim, which belongs to RUST-DOC-0009. It does not design a
staged protocol, which belongs to RUST-DOC-0010; it supplies the authority partition that
doctrine's `RUST-DOC-0010-R022` applies to stages.

It does not claim that documentation is worthless, that code explains an external constraint, or
that a system with no decision records has no unrecorded constraints.

## Intended readers

Planners assess whether an obligation can be enforced before deciding how to describe it, and
propose a decision record only after that assessment fails. Implementers encode the obligation,
add the negative evidence, generate the derived views, and link the rare record to the artifacts
that remain authoritative. Reviewers reject an unnecessary record, find the competing copy, and
test whether an unenforced part of a claim was stated or assumed. Auditors enumerate the active
records, find those whose reason has ended, and find historical records being cited as current
authority. Maintainers archive what expired, revalidate what survived, and keep generated context
free of architecture archaeology.

## Normative status

`doctrine.md` is normative and carries the stable rule identifiers. This package is version
0.1.0 with status active. Rationale, decision framework, anti-patterns, glossary, and references
are informative and cannot create an obligation that `doctrine.md` does not state.

Rules `RUST-DOC-0011-R002`, `RUST-DOC-0011-R005`, `RUST-DOC-0011-R015`, `RUST-DOC-0011-R016`,
and `RUST-DOC-0011-R017` permit a waiver on the terms recorded in the normative waiver section.
The rules governing claim classification, operational authority, competing copies, record
necessity and lifecycle, rationale honesty, external claims, agent hydration, and exception terms
do not.

## Prerequisite foundations

Read [normative language](../foundations/normative-language.md) for requirement levels and
waiver structure, [evidence](../foundations/evidence.md) for what each artifact class
establishes, [guarantee honesty](../foundations/guarantee-honesty.md) for the discipline that
separates a claim from its limits,
[complexity budget](../foundations/complexity-budget.md) for the assessment
`RUST-DOC-0011-R002` requires before an obligation is left prose-carried, and
[invariants](../foundations/invariants.md) for classifying the obligations being placed.

## Related material

Patterns: [executable narrative](../patterns/executable-narrative.md) is the mechanism this
doctrine governs; [successor capabilities](../patterns/successor-capabilities.md),
[typestate](../patterns/typestate.md), and
[opaque newtypes](../patterns/opaque-newtypes.md) are three of the mechanisms an obligation
can move into.

Boundaries: [database decoding](../boundaries/database-decoding.md) and
[Serde](../boundaries/serde.md) carry obligations across persistence and wire boundaries, and
[configuration](../boundaries/configuration.md) carries operational policy.

Reviews: [executable narrative review](../reviews/executable-narrative-review.md) is the
procedure for this doctrine, and
[final correctness audit](../reviews/final-correctness-audit.md) aggregates it. Case studies:
[registration onboarding](../case-studies/registration-onboarding/) shows an obligation
carried by types rather than by a record, and
[payment lifecycle](../case-studies/payment-lifecycle/) shows the durable and external claims
this doctrine keeps outside the executable authority.

Decision records, their template, and the worked examples live under
[`decisions/`](../decisions/README.md); the active set is enumerated in
[`manifest/decision-records.yaml`](../manifest/decision-records.yaml) and validated by
`doctrine-lint`.

## Reading order

Start with this file for scope, then `doctrine.md` for the obligations and the authority
partition. Read `rationale.md` for the failure modes this doctrine answers, then
`decision-framework.md` before writing either a mechanism or a record. Use `review-standard.md`
during review, `anti-patterns.md` when a proposed document feels close to a known failure,
`glossary.md` for terms whose local meaning is narrower than ordinary usage, and `references.md`
for provenance.

## Compact doctrine summary

Classify a claim before citing an authority for it. Put an enforceable obligation in the
mechanism that enforces it, and treat that mechanism as authoritative for what it enforces. State
the part it does not enforce rather than letting the enforced part imply it. Keep no second
manually maintained copy of an enforced claim; generate a derived view, declare its source, and
check it for drift. Name the external system authoritative for every durable or remote fact.

Write a decision record only for the residue that no artifact can carry, and then only with an
owner, a revalidation trigger, an obsolescence condition, and links to the artifacts that remain
authoritative for current behavior. Retire a record when its reason ends, and confirm a record
still applies before citing it against a change. Record rationale that cannot be recovered, and
record an absent rationale as unknown rather than inferring one.

The central non-guarantee: moving an obligation into a mechanism proves the obligation is now
enforced, not that it is the right obligation. A generated view is current, not correct. An empty
decision-record set is evidence about the record set, not about the constraints a system is
under.

## Package completion check

- metadata agrees with `manifest/doctrines.yaml` and its JSON Schema;
- rule IDs use `RUST-DOC-0011-RNNN` and every one appears in `review-standard.md`;
- all eight files carry domain-specific substance;
- references and source notes separate external facts from repository governance, and record the
  originating claim accurately;
- the decision-record registry, its schema, and the linter checks are linked;
- generated bundles reproduce after the manifest update.

---

## Source: `doctrines/0011-executable-narrative/doctrine.md`

# Normative doctrine

## RUST-DOC-0011-R001 — Classify a claim before assigning its authority

**Statement.** An architectural claim MUST be classified, before any artifact is cited as its
authority, as an in-process claim that executable structures enforce, a durable or remote claim
an external system owns, rationale or historical context, a stated non-guarantee or accepted
residual risk, or a governance claim about who may change a contract. One artifact MUST NOT be
cited as the authority for every class.

**Intent.** Replace precedence arguments between code and documents with a partition, so that a
question about what a program currently permits and a question about who accepted a residual risk
are answered by different artifacts rather than by whichever artifact is nearer to hand.

**Applicability.** Design notes, doctrine text, decision records, review records, and agent
instructions that state what settles a question about a system's architecture.

**Allowed exceptions.** None. A claim whose class cannot be named is evidence that the claim is
not yet stated precisely enough to review.

**Review evidence.** The claim classification and the single artifact cited as authority for each
classified claim.

## RUST-DOC-0011-R002 — Represent an enforceable obligation in the mechanism that enforces it

**Statement.** An ordering, invariant, construction restriction, capability boundary, transition
restriction, or negative guarantee that an available mechanism can enforce mechanically MUST be
represented in that mechanism, and MUST NOT be carried by prose alone.

**Intent.** Keep an obligation that a type, schema, constraint, manifest, or test could enforce
from surviving only as a description that nothing contradicts when it is violated.

**Applicability.** Architectural obligations in systems governed by this corpus, where a
mechanism is available in the language, the schema, the build, or the deployment configuration.
RUST-DOC-0001 governs which invariants are representable; this rule governs whether a
representable obligation was in fact represented.

**Allowed exceptions.** An obligation whose enforcement cost exceeds the assessment required by
`foundations/complexity-budget.md` MAY remain prose-carried when the assessment, its owner, and
the residual risk are recorded on the terms of RUST-DOC-0011-R020.

**Review evidence.** The enforcing artifact, or the recorded assessment showing that no available
mechanism enforces the obligation proportionately.

## RUST-DOC-0011-R003 — Treat the enforcing artifact as the operational authority

**Statement.** Where an executable or machine-checked artifact completely enforces a claim, that
artifact MUST be treated as authoritative for the claim's current operational truth, and any
prose description of the same claim MUST be treated as informative.

**Intent.** Name the artifact a reader, reviewer, or agent should consult for what the system
currently does, so that a stale description cannot be cited against a mechanism that is running.

**Applicability.** Claims about legal ordering, available operations, construction restrictions,
permitted conversions, schema constraints, canonical encodings, visibility boundaries, and
negative guarantees.

**Allowed exceptions.** Where an artifact enforces only part of a claim, prose remains
authoritative for the unenforced part, which MUST be stated separately rather than left implied
by the enforced part.

**Review evidence.** The artifact cited for the claim, and the statement of any part of the claim
it does not enforce.

## RUST-DOC-0011-R004 — Keep no competing manually maintained copy of an enforced claim

**Statement.** A manually maintained artifact MUST NOT restate an enforced topology, invariant,
interface, or constraint as an independently editable normative source. A human-readable view of
an enforced claim MAY exist only when it is generated from the enforcing artifact, mechanically
checked against it, explicitly marked informative, or confined to rationale and non-guarantees.

**Intent.** Remove the second source that drifts. Two editable descriptions of one obligation
produce two obligations, one of which is wrong and neither of which announces which.

**Applicability.** Protocol tables, state diagrams, interface listings, dependency descriptions,
schema documentation, and architecture overviews that describe an enforced claim.

**Allowed exceptions.** A dated, informative illustration that is not cited as authority MAY be
hand-written. An excerpt quoted for explanation MAY appear in rationale when the enforcing
artifact is named at the point of quotation.

**Review evidence.** The generation command or drift check for each derived view, or the
informative marking and the authority it points to.

## RUST-DOC-0011-R005 — Generate a derived view and declare its source

**Statement.** A derived view of a machine-readable source SHOULD be generated from that source
and checked for drift rather than synchronized by hand, and a generated artifact MUST declare the
source it was generated from and MUST NOT be edited in place.

**Intent.** Convert a recurring synchronization obligation into a build step, so that a view is
current because it was produced rather than because someone remembered.

**Applicability.** Diagrams, tables, interface listings, dependency graphs, distribution bundles,
and agent context packs derived from code, schemas, or manifests.

**Allowed exceptions.** A view whose generator would itself require a hand-maintained input
describing the same claim MUST NOT be generated, because that input is the competing copy
RUST-DOC-0011-R004 prohibits. Such a view stays informative, or the claim is derived from the
enforcing artifact directly.

**Review evidence.** The generator, its declared source, the drift check, and the reason for any
view left hand-written.

## RUST-DOC-0011-R006 — Create a decision record only for what cannot live elsewhere

**Statement.** A decision record MUST NOT be created when the decision can be represented,
enforced, generated, tested, or recovered from executable and machine-readable artifacts. A
record MAY be created only for the part that cannot be: an external mandate, an irreversible or
externally expensive commitment, a rejected alternative whose rejection depends on evidence the
implementation does not carry, a decision no single system owns, an accepted residual risk or
waiver, or a compatibility obligation created by previously shipped behavior.

**Intent.** Make the record the exception. A record duplicates the system, drifts independently
of it, and outlives the constraint that produced it, so its permanent cost is only justified by a
fact the system genuinely cannot carry.

**Applicability.** Every proposal to create an architecture decision record, design note, or
equivalent durable rationale artifact.

**Allowed exceptions.** None. That a decision is large, was debated, is hard to understand, or
may be forgotten is not a fact the executable artifacts cannot carry; those are arguments for
better names, types, tests, generated views, and examples.

**Review evidence.** The executability assessment, the artifact each recoverable part of the
decision now lives in, and the justification required by RUST-DOC-0011-R007 for whatever remains.

## RUST-DOC-0011-R007 — State the last-resort justification in the record

**Statement.** An active decision record MUST state which fact cannot be represented executably
and why, why a generated view is insufficient, the future decision the record protects, a named
owner, a revalidation trigger, an obsolescence condition, and the executable artifacts that
remain authoritative for current behavior.

**Intent.** Make an active record auditable and removable. A record without an owner and an end
condition cannot be retired, and a record that does not name the current authority invites a
reader to treat it as one.

**Applicability.** Every decision record in the active set, and every record proposed for it.

**Allowed exceptions.** None. A record whose justification cannot be stated in these terms fails
RUST-DOC-0011-R006 and is not created.

**Review evidence.** The record's own metadata and the registry entry that makes it discoverable.

## RUST-DOC-0011-R008 — Keep a decision record narrow

**Statement.** A decision record MUST answer one decision question, MUST state what it does not
govern, and MUST NOT be used as a general description of a system's architecture or as a home for
decisions adjacent to the one it records.

**Intent.** Keep the record's scope small enough that its obsolescence condition can be evaluated.
A record covering several decisions expires in parts, so it never expires at all.

**Applicability.** Every active decision record.

**Allowed exceptions.** None. Several related decisions are several records, each with its own
owner and expiry, or one record and an executable representation of the rest.

**Review evidence.** The record's stated question, its stated exclusions, and the review record
confirming no adjacent decision was folded in.

## RUST-DOC-0011-R009 — Expire a record whose reason has ended

**Statement.** A decision record whose external constraint, commitment, or accepted risk no
longer applies MUST be marked expired or superseded and removed from active discovery, and MUST
NOT remain in the active set because no one revisited it.

**Intent.** A record's danger begins when its reason ends and its text does not. Survival by
inattention is the mechanism by which a correct record becomes a false one.

**Applicability.** Every active decision record, at each of its revalidation triggers and at any
review that observes its obsolescence condition satisfied.

**Allowed exceptions.** A record retained for a stated compatibility or audit obligation MAY
remain discoverable when it is marked as archival and is excluded from the active set.

**Review evidence.** The registry status, the archival marking, and the trigger or condition that
was observed.

## RUST-DOC-0011-R010 — Confirm applicability before citing a record as a constraint

**Statement.** A decision record MUST NOT be cited to block or restrict a change unless its
governing constraint is confirmed still applicable, its revalidation condition is satisfied, and
the current implementation still depends on it.

**Intent.** Remove the veto a historical choice otherwise acquires. A record states what was
decided under conditions that held at the time; discoverability is not authority, and age is not
consent.

**Applicability.** Review comments, planning documents, and agent reasoning that cite a decision
record as a reason a change cannot proceed.

**Allowed exceptions.** None. A citation whose applicability cannot be confirmed is recorded as
an open question rather than as a constraint.

**Review evidence.** The confirmation of current applicability, its date, and the person or role
that made it.

## RUST-DOC-0011-R011 — Retire an implemented proposal from operational authority

**Statement.** A proposal governs review and acceptance before implementation. After
implementation the accepted proposal MUST be treated as decision history, and MUST NOT be
maintained or cited as a current specification of behavior that canonical doctrine and executable
artifacts now carry.

**Intent.** Keep an accepted RFC from becoming a competing specification that future maintainers
must reconcile against current behavior.

**Applicability.** Accepted RFCs and equivalent proposal documents after their implementation has
landed. This rule does not weaken the RFC obligations stated in `AGENTS.md` and
`rfcs/README.md`, which govern the change process rather than the resulting contract.

**Allowed exceptions.** A proposal MAY remain cited for its decision, its date, its owners, its
accepted conditions, and its recorded alternatives, which are rationale rather than
specification.

**Review evidence.** The canonical doctrine and executable artifacts the proposal points to, and
the absence of a normative obligation stated only in the proposal.

## RUST-DOC-0011-R012 — Record only rationale that cannot be recovered

**Statement.** Rationale MUST be recorded when it cannot be reconstructed safely from executable
artifacts and remains material to a future decision, and MUST NOT restate the operational
topology, interface, or invariant set as an independent contract.

**Intent.** Confine prose to what only prose carries: the constraint that shaped a design, the
alternative that was rejected and why the rejection still holds, and the risk somebody accepted.

**Applicability.** Rationale sections, design notes, decision records, and source-provenance
files.

**Allowed exceptions.** Rationale MAY quote or reference an enforced artifact for explanation
when the artifact is named as the authority at the point of reference.

**Review evidence.** The rationale text, the artifact it points to, and the statement of why the
recorded reason is not recoverable from that artifact.

## RUST-DOC-0011-R013 — Do not invent rationale for an existing constraint

**Statement.** Where the governing rationale for an enforced constraint is absent, a reviewer,
author, or agent MUST record it as unknown, and MUST NOT infer a reason from the implementation
and present that inference as the governing rationale.

**Intent.** An inferred reason presented as governing is a fabricated authority. It is
indistinguishable from a recorded one at the point of use, and it will be cited to block or
justify a change that the absent original reason may not have supported.

**Applicability.** Review records, documentation of existing systems, migration analyses, and
agent-generated summaries of code whose history is unavailable.

**Allowed exceptions.** An inference MAY be recorded when it is labelled as an inference, names
its evidence, and states that the governing rationale is unknown.

**Review evidence.** The unknown-rationale record, or the labelled inference with its evidence.

## RUST-DOC-0011-R014 — Keep an external claim outside the executable authority

**Statement.** A local executable guarantee MUST NOT be presented as evidence of a current
durable, remote, or externally governed fact, and each such fact MUST name the external system
that is authoritative for it.

**Intent.** State the partition's external leg. A type proves what its construction established
inside one process; committed state, remote acknowledgment, provider status, policy currency,
lock ownership, and settlement are facts other systems own.

**Applicability.** Claims about persisted state, remote effects, external identity, current
policy, distributed locks, fencing tokens, delivery, and settlement. RUST-DOC-0006 governs
ambiguity and reconciliation and RUST-DOC-0010-R014 governs durable advancement in a staged
protocol; this rule adds the obligation to name the authoritative external system for each claim.

**Allowed exceptions.** None. An external fact with no named authority is an unowned claim.

**Review evidence.** The claim, the named external authority, and the check that consults it.

## RUST-DOC-0011-R015 — Make a compatibility or migration promise executable

**Statement.** A compatibility promise, migration obligation, or negative guarantee SHOULD be
carried by a test, schema check, compile-fail fixture, or migration code, and where it is carried
by prose alone the artifact stating it MUST record that no mechanism enforces it.

**Intent.** Keep a promise from being read as a guarantee. A published compatibility statement
with no check behind it is a claim about intent, and a reader is entitled to know which it is.

**Applicability.** Published interfaces, wire formats, schemas, persisted representations, and
documented negative guarantees.

**Allowed exceptions.** A promise whose enforcement requires a system unavailable to the
repository stating it MAY remain prose-carried when the gap is recorded on the terms of
RUST-DOC-0011-R020.

**Review evidence.** The enforcing test, check, fixture, or migration, or the recorded statement
that the promise is unenforced.

## RUST-DOC-0011-R016 — Keep the enforced structure readable as its domain story

**Statement.** An executable structure relied on as the authority for an architectural claim MUST
use domain names, MUST name its states for the facts they establish, MUST disclose its effects,
MUST keep capabilities narrow, and MUST delay type erasure, so that the enforced obligation is
legible without a parallel prose description.

**Intent.** An authority nobody can read produces the duplicate this doctrine exists to remove.
Legibility is not decoration here; it is the condition under which the executable artifact can
actually serve as the shared account of what the system does.

**Applicability.** Types, traits, schemas, manifests, and configuration relied on as the
authority for an architectural claim. RUST-DOC-0010-R002 governs stage naming within a staged
protocol; this rule generalizes the obligation to any artifact carrying architectural authority.

**Allowed exceptions.** An internal artifact with a documented, narrow audience MAY use local
abbreviations when they are defined at the artifact's entry point.

**Review evidence.** Names, state definitions, effect disclosure, capability scope, and the
location of any erasure boundary.

## RUST-DOC-0011-R017 — Count and reduce the maintained representations of a claim

**Statement.** A design review MUST identify every maintained representation of an architectural
claim, and MUST remove those that are neither authoritative, generated, mechanically checked, nor
required for irrecoverable rationale.

**Intent.** Make duplication a reviewable quantity rather than a matter of taste. The count is the
number of places a future change has to be made correctly, and it is the honest measure of the
cost of an architectural decision.

**Applicability.** Design reviews, doctrine changes, and any change that adds a description of an
existing obligation.

**Allowed exceptions.** A representation retained for a stated audience obligation MAY remain
when it is generated, mechanically checked, or marked informative and owned.

**Review evidence.** The representation inventory for the claim, and the disposition recorded for
each entry.

## RUST-DOC-0011-R018 — Hydrate agents from current authority

**Statement.** Generated agent context MUST be built from current canonical and executable
authority, and MUST NOT include expired, superseded, or archived decision records by default.

**Intent.** Keep obsolete decisions out of the context an agent reasons from. An agent cannot
apply RUST-DOC-0011-R010 to a record it was handed as background rather than as a citation.

**Applicability.** Agent manifests, generated hydration packs, and any automated assembly of
context for planning, implementation, review, audit, or maintenance.

**Allowed exceptions.** An archived record MAY be included for a task whose scope is that record,
when the inclusion is explicit and the archival status travels with it.

**Review evidence.** The agent manifest, the generated pack contents, and the drift check.

## RUST-DOC-0011-R019 — Govern a change without duplicating what it changes

**Statement.** A normative change remains subject to the RFC, review, versioning, and migration
obligations of this corpus, and a governance artifact MUST NOT thereby become a second
operational specification of the contract it governs.

**Intent.** Preserve the change process without letting it accumulate a parallel description of
the system. Governance decides who may change a contract and on what evidence; it does not
restate the contract.

**Applicability.** RFCs, manifests, review records, waivers, and release notes.

**Allowed exceptions.** A governance artifact MAY state the contract as it stands at the moment
of decision, as the record of what was decided, when it is dated and is not maintained afterwards.

**Review evidence.** The governance artifact, the canonical contract it governs, and the absence
of a maintained restatement.

## RUST-DOC-0011-R020 — Record the terms of a prose-only obligation

**Statement.** An exception that leaves an obligation carried by prose alone, or that keeps a
decision record in the active set, MUST name the owner, the consequence if the obligation is not
met, the compensating control, the reconsideration trigger, and the removal condition.

**Intent.** Give every unenforced obligation an end condition and somebody who owns it, so that
the exception is a decision with a lifetime rather than an omission with a description.

**Applicability.** Every exception claimed under RUST-DOC-0011-R002, RUST-DOC-0011-R005,
RUST-DOC-0011-R009, RUST-DOC-0011-R015, and RUST-DOC-0011-R017.

**Allowed exceptions.** None.

**Review evidence.** The recorded exception with all five terms, and the review that confirmed the
trigger has not fired.

## Authority partition

Every claim this doctrine governs belongs to exactly one of five classes, and each class has one
kind of authority.

Executable and machine-checked artifacts are authoritative for the in-process operational truths
they enforce: legal ordering, available operations, successor relationships, construction
restrictions, permitted conversions and casts, schema constraints, canonical encodings,
visibility boundaries, runtime transition predicates, generated interface surfaces, and negative
guarantees demonstrated by rejection. For these claims, prose is informative under
RUST-DOC-0011-R003.

External and durable systems are authoritative for facts outside local execution: committed
state, remote acknowledgment, broker acceptance, provider identity status, current policy, the
current time, distributed lock ownership, fencing-token validity, delivery, and settlement.
RUST-DOC-0011-R014 keeps a local guarantee from standing in for any of them.

Rationale artifacts are authoritative only for what cannot be recovered from the artifacts: the
external constraint that shaped a design, a rejected alternative whose rejection remains
material, an irreversible commitment, a regulatory interpretation, a contractual obligation, an
accepted trade-off, and migration history that affects compatibility. RUST-DOC-0011-R012 keeps
them from restating the topology.

Non-guarantee and residual-risk statements are authoritative for what a design deliberately does
not prove and who accepted the remainder, on the terms `foundations/guarantee-honesty.md` states.

Governance artifacts are authoritative for who may change a normative contract, the required
review, waiver ownership, versioning policy, migration obligations, release gates, and legal or
regulatory approval. RUST-DOC-0011-R019 keeps them from becoming a second specification.

## Decision-record requirements

A decision record is created only for the residue identified by RUST-DOC-0011-R006, carries the
justification required by RUST-DOC-0011-R007, answers one question under RUST-DOC-0011-R008, and
ends under RUST-DOC-0011-R009. The active set is enumerated in a machine-readable registry so
that it can be audited, and so that RUST-DOC-0011-R018 can exclude what is no longer current.

A record is not a substitute for an RFC. An RFC proposes a change to a normative contract and is
governed by RUST-DOC-0011-R011 and by `rfcs/README.md`; a decision record captures a fact that
outlives the change and that no artifact carries.

## Guarantee and non-guarantee requirements

This doctrine states, for each claim it governs: the class the claim belongs to under
RUST-DOC-0011-R001; the artifact authoritative for it under RUST-DOC-0011-R003 or
RUST-DOC-0011-R014; the part of the claim no artifact enforces, stated separately under
RUST-DOC-0011-R003 and RUST-DOC-0011-R015; the maintained representations that remain and why,
under RUST-DOC-0011-R017; and the owner, trigger, and removal condition of every exception, under
RUST-DOC-0011-R020.

What this doctrine does not establish: that an obligation moved into a mechanism is thereby
correct; that a generated view is correct because it is current; that a record with a stated
justification has a good one; or that a system with no decision records has no unrecorded
constraints. Absence of a record is evidence about the record set, not about the constraints.

## Boundary requirements

Where an obligation crosses a boundary, the enforcing mechanism changes and the authority moves
with it. A wire contract is enforced by its canonical encoder, decoder, schema, and compatibility
suite under `boundaries/serde.md` and `boundaries/http-and-rpc.md`. A persistence invariant is
enforced by schema constraints, checked decoding, and transaction predicates under
RUST-DOC-0005 and `boundaries/database-decoding.md`. An operational policy is enforced by
deployable configuration and machine-checked manifests under `boundaries/configuration.md`. A
claim that crosses into another system's ownership becomes an external claim governed by
RUST-DOC-0011-R014.

## Waiver requirements

RUST-DOC-0011-R002, RUST-DOC-0011-R005, RUST-DOC-0011-R015, RUST-DOC-0011-R016, and
RUST-DOC-0011-R017 MAY be waived for an obligation whose enforcement or review cost is
disproportionate to its consequence. A waiver records the affected rule and claim, the owner
accepting the risk, the consequence, the compensating control, an expiry or reconsideration
trigger, and the removal condition, which are the same terms RUST-DOC-0011-R020 requires.

RUST-DOC-0011-R001, RUST-DOC-0011-R003, RUST-DOC-0011-R004, RUST-DOC-0011-R006,
RUST-DOC-0011-R007, RUST-DOC-0011-R008, RUST-DOC-0011-R009, RUST-DOC-0011-R010,
RUST-DOC-0011-R011, RUST-DOC-0011-R012, RUST-DOC-0011-R013, RUST-DOC-0011-R014,
RUST-DOC-0011-R018, RUST-DOC-0011-R019, and RUST-DOC-0011-R020 MUST NOT be waived. A waiver
cannot make an obsolete record current, cannot make an inferred rationale a governing one, cannot
make a local guarantee external evidence, and cannot authorize a second maintained source for a
claim an artifact already enforces.

---

## Source: `doctrines/0011-executable-narrative/rationale.md`

# Rationale

## Failure modes

**The obligation that only prose held.** A design states that authentication precedes
authorization. Nothing enforces it. A refactor reorders two calls, every test passes because
every test exercises the ordinary path, and the document still says the right thing while the
program does the wrong one. The document's correctness is what makes this hard to find: a
reviewer reads the sentence, believes the system, and never opens the call site.
`RUST-DOC-0011-R002` moves an enforceable obligation into the mechanism that enforces it.

**The second copy that drifted.** A protocol's stage graph is enforced by types and also
described by a table in an architecture document. One of them is updated. Nothing announces which
is now wrong, and a reader who consults the table forms a false model of the running system with
no signal that anything is amiss. Two editable descriptions of one obligation are two obligations.
`RUST-DOC-0011-R004` prohibits the second, and `RUST-DOC-0011-R005` prefers generation so the
view cannot drift at all.

**The record that outlived its reason.** A record states that a component was chosen over an
alternative because of a constraint that held at the time. The constraint disappears; the record
does not. It stays discoverable, it stays plausible, and a reviewer or an agent cites it against
a change that is now clearly better. The improvement then requires arguing against a document
rather than against a constraint, and the person best placed to know the constraint is gone has
no standing that the document lacks. `RUST-DOC-0011-R009` requires an end condition and
`RUST-DOC-0011-R010` requires applicability to be confirmed before the record is cited.

**The record that was written because the decision felt large.** A team debates a choice at
length, the discussion is valuable, and a record is written to preserve it. The record describes
the module layout, the interfaces, and the ordering, all of which the code carries and enforces.
Nothing in it is irrecoverable. What it produces is a second source that a future maintainer must
reconcile against current behavior, and a permanent artifact whose only stated justification is
that the decision mattered. `RUST-DOC-0011-R006` requires the residue to be named, and no rule in
this package permits a record on the grounds that a decision was important, debated, complex, or
memorable. Those are arguments for better names, types, tests, and examples.

**The rationale that was inferred.** A constraint exists in code with no recorded reason. A
reviewer reconstructs a plausible motive from the surrounding implementation and writes it down.
The inference is now indistinguishable from a recorded decision at the point of use, and it will
be cited to defend a constraint whose actual reason may have been different or may have expired.
`RUST-DOC-0011-R013` requires the unknown to stay unknown, or the inference to be labelled as
one.

**The local guarantee read as an external fact.** A type proves that a local sequence ran. A
reader concludes that a row was written, a message was delivered, or a provider agreed. The
inference is natural because the local guarantee is the strongest one visibly available, and it
is wrong because the external system was never consulted. `RUST-DOC-0011-R014` requires each
external fact to name the system authoritative for it, and this doctrine's partition keeps the
two classes apart by construction rather than by care.

**The precedence rule that answered the wrong claim.** This corpus recorded a source claim as
"code is a sufficient contract", rejected it, and encoded the rejection as a blanket precedence
of governance over the executable protocol. The source had not made that claim. What resulted was
a rule stating that the executable protocol does not settle what the system is obliged to do,
sitting in a package whose `RUST-DOC-0010-R018` and `RUST-DOC-0010-R019` exist precisely because
prose cannot detect a widened bound or a redirected successor. The package argued the partition
and then denied it. The defect was live in this corpus until the restatement recorded in RFC-0003.

**The index nobody regenerated.** `rfcs/accepted/README.md` listed RFC-0001 and omitted RFC-0002
within a single release, because the index of accepted proposals is maintained by hand beside a
directory that already contains the answer. The cost was small and the mechanism is the general
one: a hand-maintained view of a machine-readable fact is wrong as soon as attention lapses.
`RUST-DOC-0011-R005` prefers generation, and `RUST-DOC-0011-R017` makes the count of maintained
representations something a review states rather than estimates.

## Why weaker alternatives fail

**"Documentation is important, and so is code."** True and useless. It resolves no question,
because every disputed claim has two artifacts describing it and this position tells a reviewer
nothing about which to trust. The partition is what makes the sentiment actionable: for an
enforced claim the artifact wins, for an external fact the external system wins, and for a
rejected alternative only the record has anything to say.

**Blanket precedence in either direction.** "Code is the contract" cannot express an external
mandate, an accepted residual risk, or who may change an interface. "Documentation governs" turns
a stale sentence into an authority over a running mechanism. Both are attempts to answer a
question about five classes of claim with one rule.

**Diligence.** Requiring people to keep two representations synchronized assigns an obligation
that has no failure signal. Nothing breaks when the copy goes stale, so nothing surfaces it, and
the discovery happens later at a reader who trusted the wrong one.
`RUST-DOC-0011-R005` removes the obligation rather than assigning it.

**A record for every architectural change.** This is the practice the doctrine restricts, and its
cost is not the writing. It is the accumulating set of plausible, discoverable, unowned documents
that a future reader cannot distinguish from current constraints, and that an agent will hydrate
as context. The set grows monotonically unless expiry is built in, which is why
`RUST-DOC-0011-R007` and `RUST-DOC-0011-R009` are separate obligations from
`RUST-DOC-0011-R006`.

**Prohibiting records entirely.** An external mandate, an irreversible commitment, and an
accepted residual risk are real facts that no artifact carries. Prohibiting the record does not
remove the fact; it relocates it to a commit message or an issue thread, where it has no owner,
no expiry, and no place in review.

## Interaction with external reality

The partition's external leg is the one most often crossed by accident, because the local
guarantee is visible in the editor and the external one is not. Committed state, remote
acknowledgment, provider identity status, current policy, the current time, lock ownership,
fencing-token validity, delivery, and settlement are each owned by a system that has to be asked.
A design that names the owner for each of them can be reviewed; a design that does not has an
unowned claim, and the review has nothing to check it against.

Rationale is also subject to external change. A regulatory interpretation, a contractual
obligation, and a vendor restriction can all lapse without anything in the repository moving,
which is why `RUST-DOC-0011-R009` attaches an obsolescence condition to the record rather than to
a review calendar.

## Costs and overapplication

Classifying a claim before citing an authority for it is a real step, and on a small change it
can cost more than the confusion it prevents. Counting maintained representations is a review
obligation that adds time to design review. Generation adds a build step, a generator to
maintain, and a drift check that fails on unrelated changes until it is understood.

The doctrine is overapplied when it becomes an argument for deleting rationale. The rules that
prohibit a competing copy are about copies of _enforced_ claims; the constraint that shaped a
design, the alternative that was rejected, and the risk somebody accepted are not enforced by
anything and have no other home. `RUST-DOC-0011-R012` and `RUST-DOC-0011-R013` are the guards,
and a reviewer who cites this doctrine to remove an irrecoverable reason has inverted it.

It is also overapplied when generation is demanded for a view whose generator would need a
hand-maintained input describing the same claim. That input is the competing copy wearing the
word "generated", and `RUST-DOC-0011-R005` names the case explicitly so it does not have to be
rediscovered.

## Guarantee ledger

| Claim                                                          | Established by                                                    | Protected construction                                     | Boundary preservation                                     | Escape hatches                                    | Does not prove                                                    | Residual runtime risk                                        |
| -------------------------------------------------------------- | ----------------------------------------------------------------- | ---------------------------------------------------------- | --------------------------------------------------------- | ------------------------------------------------- | ----------------------------------------------------------------- | ------------------------------------------------------------ |
| An enforced claim has exactly one operational authority        | the classification under R001 and the citation under R003         | review gates E01 to E05                                    | the unenforced part is stated separately under R003       | recorded assessment under R002 and R020           | that the enforced obligation is the right obligation              | a claim nobody classified, so nobody noticed was unenforced  |
| No competing manually maintained copy of an enforced claim     | the representation inventory under R017 and the ban under R004    | review gates E15 to E21                                    | derived views are generated and drift-checked under R005  | informative marking with a named owner            | that the remaining authority is current with the domain           | a copy in a system outside this repository's review scope    |
| A generated view cannot silently diverge from its source       | generation plus drift detection under R005                        | the generator declares its source and output is not edited | `dist/` is regenerated, never hand-corrected              | none                                              | that the generated content is correct, only that it is current    | a canonical source that is itself a stale second copy        |
| Every active decision record is owned and has an end condition | `doctrine-lint` reading each record's own front matter under R007 | the registry lists membership only, duplicating no field   | archived records are excluded from agent packs under R018 | none                                              | that the record's stated justification is a good one              | a record whose justification is stated, auditable, and wrong |
| An obsolete record is not current authority                    | status change under R009 and confirmation under R010              | archival marking travels with the record                   | generated packs exclude archived records                  | explicit inclusion for a record-scoped task       | that a reader will not open the archive and cite it anyway        | a record cited outside review, where no gate applies         |
| An external fact names its authoritative system                | the external-authority requirement under R014                     | review gates E06 and E07                                   | the check that consults the external system is named      | none                                              | that the external system was consulted at the moment of the claim | staleness between the consultation and the use               |
| Recorded rationale is genuinely irrecoverable                  | the recoverability check under R012                               | review gate E36                                            | rationale references the enforcing artifact by name       | quotation for explanation with the artifact named | that the recorded reason is the reason that actually governed     | a recorded reason that was itself an unlabelled inference    |
| An absent rationale stays absent rather than invented          | the unknown record required by R013                               | review gates E37 and E38                                   | inferences are labelled with their evidence               | labelled inference                                | that the governing reason can be recovered later                  | an inference labelled once and cited later without its label |

## Evidence limits

Most of this doctrine is judgment. The linter opens each registered record and validates, from
that record's own front matter, that it has a well-formed unique identifier, a title, an owner, a
scope, and for an active record a revalidation trigger, an obsolescence condition, and executable
authorities that resolve to real files. It cannot decide whether the record should exist, whether
the stated justification is honest, or whether an obligation carried by prose could have been
carried by a type. Those are
`RUST-DOC-0011-R002`, `RUST-DOC-0011-R003`, `RUST-DOC-0011-R006`, `RUST-DOC-0011-R012`,
`RUST-DOC-0011-R013`, `RUST-DOC-0011-R016`, and `RUST-DOC-0011-R017`, and they are supported by
review gates rather than by executed evidence.

Deterministic bundle generation with drift detection is real evidence for the mechanism
`RUST-DOC-0011-R005` prefers, in this repository, for these bundles. It is not evidence that any
view elsewhere is generated, and it is not evidence that the canonical sources feeding it are
themselves free of duplication.

The registry ships with an empty active set. That is the state this doctrine predicts for a
repository whose obligations are carried by doctrine, manifests, schemas, and a linter, and the
linter validates the registry as such. An empty set is evidence about the record set and not
about the constraints this repository is under; an unrecorded constraint leaves no trace in a
registry designed to hold only what somebody chose to record.

---

## Source: `doctrines/0011-executable-narrative/decision-framework.md`

# Decision framework

## Inputs

- the claim, stated precisely enough that its truth could be checked;
- the mechanisms available in the language, schema, build, and deployment configuration;
- the systems that own any durable or remote fact the claim depends on;
- the existing artifacts that already describe the claim, and who maintains each;
- the complexity budget assessment from `foundations/complexity-budget.md`;
- the audience that has to act on the claim, and what they consult today.

## Questions

1. Which class does the claim belong to: enforced local truth, external or durable fact,
   rationale, non-guarantee or accepted risk, or change authority?
2. Which available mechanism could enforce it, and what would that cost?
3. If a mechanism enforces part of it, which part is left unenforced?
4. Which artifacts already describe this claim, and how many of them are maintained by hand?
5. Can a described view be generated from the artifact that enforces the claim?
6. Would a generator need a hand-maintained input that describes the same claim?
7. If a record is proposed, which exact fact in it cannot be represented, enforced, generated, or
   recovered?
8. What event makes that fact stop mattering, and who notices?
9. Which artifact stays authoritative for current behavior after the record exists?
10. Is the proposal actually a change proposal, and therefore an RFC rather than a record?

## Decision table

| Situation                                                          | Placement                                                | Rules                                      |
| ------------------------------------------------------------------ | -------------------------------------------------------- | ------------------------------------------ |
| Ordering, invariant, or construction restriction, mechanism exists | the mechanism; prose is informative                      | `RUST-DOC-0011-R002`, `RUST-DOC-0011-R003` |
| Negative guarantee that can be demonstrated by rejection           | compile-fail fixture or rejected-case test               | `RUST-DOC-0011-R002`, `RUST-DOC-0011-R015` |
| Enforceable, but enforcement cost exceeds the budget               | prose, with the assessment and the five exception terms  | `RUST-DOC-0011-R002`, `RUST-DOC-0011-R020` |
| Human-readable view of an enforced claim                           | generated from the enforcing artifact, drift-checked     | `RUST-DOC-0011-R004`, `RUST-DOC-0011-R005` |
| View whose generator needs a hand-maintained description           | leave informative and owned; do not call it generated    | `RUST-DOC-0011-R005`                       |
| Durable, remote, or externally governed fact                       | the external system, named as the authority              | `RUST-DOC-0011-R014`                       |
| Rejected alternative whose rejection still governs                 | rationale, with the evidence the code does not carry     | `RUST-DOC-0011-R012`                       |
| Reason for an existing constraint is unavailable                   | record it as unknown, or label the inference             | `RUST-DOC-0011-R013`                       |
| External mandate, irreversible commitment, or accepted risk        | a decision record, with owner, triggers, and authorities | `RUST-DOC-0011-R006`, `RUST-DOC-0011-R007` |
| Proposal to change a normative contract                            | an RFC; retire it from authority once implemented        | `RUST-DOC-0011-R011`, `RUST-DOC-0011-R019` |
| Onboarding difficulty                                              | names, types, tests, generated views, examples           | `RUST-DOC-0011-R006`, `RUST-DOC-0011-R016` |

## Decision tree

```text
Is the claim about a durable, remote, or externally governed fact?
  yes -> name the external authority and the check that consults it. R014. Stop.
  no  -> continue

Can an available mechanism enforce the claim, wholly or partly?
  no  -> is the reason cost, or is the fact simply not enforceable by anything?
           cost         -> prose, plus the budget assessment and the five terms. R002, R020.
           unenforceable -> continue to the record test.
  yes -> represent it in that mechanism. R002.
         Does the mechanism enforce all of it?
           yes -> the mechanism is the authority; any prose is informative. R003.
           no  -> state the unenforced part separately, and label it unenforced. R003, R015.

Does another maintained artifact also describe this claim?
  yes -> can it be generated from the enforcing artifact?
           yes -> generate it, declare the source, add the drift check. R005.
           no  -> would the generator need a hand-maintained description of the claim?
                    yes -> keep it informative and owned; do not call it generated. R005.
                    no  -> delete it, or confine it to rationale and non-guarantees. R004.
  no  -> continue

Record test. Which exact fact cannot be represented, enforced, generated, or recovered?
  none named        -> write no record. R006. Stop.
  the decision is a proposal to change a contract -> file an RFC instead. R011.
  a fact is named   -> is it an external mandate, an irreversible or externally expensive
                       commitment, a rejected alternative whose rejection depends on evidence the
                       implementation does not carry, a decision no single system owns, an
                       accepted residual risk, or a compatibility obligation from shipped
                       behavior?
                         no  -> write no record. R006. Stop.
                         yes -> write one narrow record. R008.
                                State the last-resort justification, the owner, the revalidation
                                trigger, the obsolescence condition, and the executable
                                authorities that govern current behavior. R007.
                                Register it in the active set so it can be audited and expired.
                                R009, R018.
```

## Complexity check

Count the representations of the claim after the decision, and compare with the count before it.
An acceptable outcome reduces the count or holds it at one authoritative representation plus
whatever is generated. An outcome that adds a maintained representation needs a reason stated in
the review record under `RUST-DOC-0011-R017`.

Moving an obligation into a mechanism has costs of its own. A type that enforces an ordering
lengthens signatures and worsens first-encounter diagnostics; a schema constraint moves a failure
from a readable message to a driver error; a generated view adds a generator and a drift check
that fails on unrelated changes until it is understood. Where the enforcement cost exceeds the
consequence of the obligation being violated, `RUST-DOC-0011-R002` permits the prose form with
the assessment recorded, and `RUST-DOC-0011-R020` requires the exception to name an owner and an
end condition. That is the honest exit, and it is preferable to a mechanism nobody can read,
which `RUST-DOC-0011-R016` treats as its own failure.

## Evidence selection

| Claim class                                  | Evidence that fits                                               | Evidence that does not                         |
| -------------------------------------------- | ---------------------------------------------------------------- | ---------------------------------------------- |
| Legal ordering or transition restriction     | types, compile-fail fixtures, contract assertions                | a document stating the order                   |
| Construction restriction                     | private representation, checked constructor, visibility audit    | a naming convention                            |
| Permitted conversion or cast                 | schema base types, explicit conversion functions, rejected cases | a comment naming the intended type             |
| Persistence invariant                        | schema constraint, checked decoding, transaction predicate       | an application-layer assertion alone           |
| Wire compatibility                           | canonical encoder and decoder, schema, compatibility suite       | a version note in a changelog                  |
| Negative guarantee                           | compile-fail fixture, rejected input case                        | prose asserting impossibility                  |
| Derived human-readable view                  | generator, declared source, drift check                          | a hand-updated diagram                         |
| Durable or remote fact                       | the external system's own check, with its identity and token     | a local type reached by a consuming transition |
| External mandate or accepted risk            | a registered decision record with owner and end condition        | a commit message or an issue thread            |
| Reason a rejected alternative stays rejected | rationale naming the evidence, dated                             | an inference from the current implementation   |

Choose the narrowest evidence class that matches the claim. A green suite, a passing build, or a
generated bundle is never itself evidence that an obligation is enforced; each proves only what
it exercised.

---

## Source: `doctrines/0011-executable-narrative/review-standard.md`

# Review standard

Mark every gate **pass**, **fail**, **not applicable**, or with an approved **waiver
reference**. Blank status is not approval. There is no score: a total would let a strong result
in a cheap category offset a critical failure in an expensive one.

## Claim classification and authority

| Gate | Question                                                      | Pass evidence         | Failure example                          | Severity | Remediation             |
| ---- | ------------------------------------------------------------- | --------------------- | ---------------------------------------- | -------- | ----------------------- |
| E01  | Is each architectural claim classified before it is cited?    | claim classification  | claim reviewed with no class named       | high     | classify first          |
| E02  | Does each classified claim name one authority?                | authority mapping     | two artifacts cited for one claim        | high     | pick the authority      |
| E03  | Is any one artifact cited as authority for every class?       | authority mapping     | doctrine cited for current behavior      | critical | partition the claims    |
| E04  | Is the enforcing artifact cited for an enforced claim?        | source or schema path | prose cited for legal ordering           | critical | cite the mechanism      |
| E05  | Is the unenforced part of a claim stated separately?          | scope statement       | partial enforcement read as complete     | critical | state the remainder     |
| E06  | Does each external fact name its authoritative system?        | external check        | remote status inferred from a local type | critical | name the external owner |
| E07  | Is a local guarantee presented as durable or remote evidence? | ledger rows           | consumed handle read as commit proof     | critical | narrow the claim        |

## Executable representation

| Gate | Question                                                          | Pass evidence         | Failure example                                 | Severity | Remediation             |
| ---- | ----------------------------------------------------------------- | --------------------- | ----------------------------------------------- | -------- | ----------------------- |
| E08  | Could this obligation be a type, constructor, or visibility rule? | executability test    | ordering enforced by convention                 | high     | move it into the type   |
| E09  | Could it be a schema constraint, cast rule, or procedure?         | schema or migration   | identifier species mixed without a cast         | high     | constrain in the schema |
| E10  | Could it be a test, fixture, or manifest entry?                   | test or manifest      | negative guarantee asserted in prose            | high     | add the check           |
| E11  | Is a prose-only obligation recorded with its assessment?          | complexity assessment | prose obligation with no reason recorded        | high     | record or enforce       |
| E12  | Does a compatibility promise have a mechanism behind it?          | test or schema check  | published promise with no check                 | high     | enforce or label        |
| E13  | Is an unenforced promise labelled as unenforced?                  | explicit statement    | intent stated as a guarantee                    | critical | label the claim         |
| E14  | Is the authoritative structure legible in domain terms?           | names and states      | positional names, hidden effects, early erasure | medium   | rename and disclose     |

## Duplication and generated views

| Gate | Question                                                    | Pass evidence            | Failure example                          | Severity | Remediation                |
| ---- | ----------------------------------------------------------- | ------------------------ | ---------------------------------------- | -------- | -------------------------- |
| E15  | Does a manually maintained copy of an enforced claim exist? | representation inventory | hand-written stage table beside the code | high     | generate or delete         |
| E16  | Is each derived view generated or drift-checked?            | generator and check      | diagram updated by hand after a change   | high     | generate the view          |
| E17  | Does each generated artifact declare its source?            | banner or header         | generated file with no provenance        | medium   | declare the source         |
| E18  | Was any generated artifact edited in place?                 | drift check              | manual fix applied to generated output   | high     | fix the source             |
| E19  | Is a hand-written view marked informative and owned?        | marking and owner        | informal diagram cited as authority      | medium   | mark or remove             |
| E20  | Would the generator need a hand-maintained input?           | generator input          | edge list retyped to feed a generator    | high     | derive or stay informative |
| E21  | Is the representation count for the claim recorded?         | inventory disposition    | duplication assessed by impression       | medium   | count the representations  |

## Decision-record necessity

| Gate | Question                                                    | Pass evidence       | Failure example                      | Severity | Remediation                |
| ---- | ----------------------------------------------------------- | ------------------- | ------------------------------------ | -------- | -------------------------- |
| E22  | Which exact fact cannot live in an executable artifact?     | named fact          | record justified as "important"      | critical | name the fact or drop it   |
| E23  | Why is a generated view insufficient for it?                | stated reason       | generation never considered          | high     | assess generation          |
| E24  | Which future decision does the record protect?              | stated risk         | record protects nothing identifiable | high     | state the risk or drop it  |
| E25  | Is the record a restatement of a decision the code carries? | comparison          | record describes the module layout   | critical | delete the record          |
| E26  | Is the record actually a proposal, so an RFC instead?       | governance route    | change proposal filed as a record    | medium   | route to the RFC process   |
| E27  | Is the record onboarding prose in decision form?            | audience check      | record explains how the system works | high     | improve names and examples |
| E28  | Does the record answer exactly one decision question?       | stated question     | one record covering four decisions   | high     | split the record           |
| E29  | Does the record state what it does not govern?              | exclusion statement | scope left to the reader             | high     | state the exclusions       |

## Record lifecycle and historical veto

| Gate | Question                                                           | Pass evidence       | Failure example                            | Severity | Remediation            |
| ---- | ------------------------------------------------------------------ | ------------------- | ------------------------------------------ | -------- | ---------------------- |
| E30  | Does each active record name an owner?                             | registry entry      | record with no accountable role            | critical | assign an owner        |
| E31  | Does it name a revalidation trigger and an obsolescence condition? | registry entry      | record active with no end condition        | critical | state both             |
| E32  | Does it link the executable authorities for current behavior?      | linked paths        | record silent on what governs behavior now | high     | link the authorities   |
| E33  | Has a record whose reason ended been expired or archived?          | status change       | obsolete record still active               | critical | expire the record      |
| E34  | Was a record cited as a constraint without confirming it?          | confirmation record | old record cited to block a change         | critical | confirm or withdraw    |
| E35  | Is an implemented proposal still cited as a specification?         | citation audit      | accepted RFC treated as current contract   | high     | cite doctrine and code |

## Rationale honesty, agents, and governance

| Gate | Question                                                      | Pass evidence        | Failure example                            | Severity | Remediation            |
| ---- | ------------------------------------------------------------- | -------------------- | ------------------------------------------ | -------- | ---------------------- |
| E36  | Is recorded rationale genuinely irrecoverable from artifacts? | recoverability check | rationale restates the type signatures     | medium   | trim to what is unique |
| E37  | Is an absent rationale recorded as unknown?                   | unknown record       | a reason inferred from the implementation  | critical | record unknown         |
| E38  | Is any inference labelled as an inference with its evidence?  | labelled inference   | inference presented as governing rationale | critical | label or remove        |
| E39  | Do generated agent packs exclude archived records?            | pack contents        | expired record hydrated as context         | high     | exclude from the packs |
| E40  | Does every exception carry all five recorded terms?           | exception record     | exception with no removal condition        | high     | complete the terms     |

## Outcome

Critical failures block merge. A valid waiver identifies the affected rule and claim, the owner
accepting the risk, the consequence, the compensating control and its evidence, an expiry or
reconsideration trigger, and the removal condition. A waiver cannot make an obsolete record
current, cannot make an inferred rationale a governing one, cannot make a local guarantee
external evidence, and cannot authorize a second maintained source for a claim an artifact
already enforces. Remediation is verified by re-running the gate against the changed artifact,
not by asserting that the change was made.

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0011-R001`, `RUST-DOC-0011-R002`, `RUST-DOC-0011-R003`, `RUST-DOC-0011-R004`
- `RUST-DOC-0011-R005`, `RUST-DOC-0011-R006`, `RUST-DOC-0011-R007`, `RUST-DOC-0011-R008`
- `RUST-DOC-0011-R009`, `RUST-DOC-0011-R010`, `RUST-DOC-0011-R011`, `RUST-DOC-0011-R012`
- `RUST-DOC-0011-R013`, `RUST-DOC-0011-R014`, `RUST-DOC-0011-R015`, `RUST-DOC-0011-R016`
- `RUST-DOC-0011-R017`, `RUST-DOC-0011-R018`, `RUST-DOC-0011-R019`, `RUST-DOC-0011-R020`

Gate groups map to rules as follows. E01 to E07 cover `RUST-DOC-0011-R001`,
`RUST-DOC-0011-R003`, and `RUST-DOC-0011-R014`. E08 to E14 cover `RUST-DOC-0011-R002`,
`RUST-DOC-0011-R015`, and `RUST-DOC-0011-R016`. E15 to E21 cover `RUST-DOC-0011-R004`,
`RUST-DOC-0011-R005`, and `RUST-DOC-0011-R017`. E22 to E29 cover `RUST-DOC-0011-R006`,
`RUST-DOC-0011-R007`, and `RUST-DOC-0011-R008`. E30 to E35 cover `RUST-DOC-0011-R009`,
`RUST-DOC-0011-R010`, and `RUST-DOC-0011-R011`. E36 to E40 cover `RUST-DOC-0011-R012`,
`RUST-DOC-0011-R013`, `RUST-DOC-0011-R018`, `RUST-DOC-0011-R019`, and `RUST-DOC-0011-R020`.

---

## Source: `doctrines/0011-executable-narrative/anti-patterns.md`

# Anti-pattern catalogue

## Ordering held only by a document

**Weak example.** A design document states that authentication precedes authorization, and the
call sites depend on developers reading it.

**Why it fails.** Nothing contradicts the document when the code stops matching it. The sequence
is enforceable by types, so the obligation had a mechanism available and was left in the one
place that cannot fail.

**Risk.** A reordering passes review because the reviewer read the sentence and believed the
system.

**Improved direction.** Make the successor of authentication something only authorization
accepts, and let the document explain why the order exists rather than assert that it holds.

**When justified.** When the ordering is advisory, or when the stages are chosen externally at
runtime, in which case the runtime model owns it and the document says so.

## Architecture document that restates the enforced graph

**Weak example.** A protocol's stages are enforced by types and also tabulated in an architecture
overview, kept current by whoever remembers.

**Why it fails.** Two editable descriptions of one obligation are two obligations. When they
diverge, neither announces it, and the reader who consults the table forms a false model with no
signal.

**Risk.** A design decision made against a stale table, then defended by citing it.

**Improved direction.** Generate the view from the enforcing artifact and check it for drift, or
mark it informative, name its owner, and point it at the authority.

**When justified.** Never as an independently maintained normative source. A generated view, a
drift-checked view, or a dated informative illustration is the acceptable form.

## Generator fed by a hand-maintained description

**Weak example.** A protocol diagram is generated from an edge list that a developer types in and
updates by hand after changing the traits.

**Why it fails.** The edge list is the competing copy the doctrine prohibits, and calling the
output generated conceals that. The generator proves the diagram matches the list, not that the
list matches the code.

**Risk.** Confidence in a view whose only guarantee is internal consistency with a second
manually maintained artifact.

**Improved direction.** Derive the view from the enforcing artifact, or leave it informative and
owned and stop describing it as generated.

**When justified.** When the input is itself the authority, as a manifest is, and the code is
validated against it rather than the other way round.

## Decision record for a decision the code carries

**Weak example.** A record titled "we will validate input at the boundary" describes the module
layout, the interfaces, and the ordering, all of which the types enforce.

**Why it fails.** Nothing in the record is irrecoverable. It creates a second source, drifts
independently, and gives a future maintainer something to reconcile against current behavior.

**Risk.** A record cited years later as the reason a boundary cannot move, when the code has long
since moved it.

**Improved direction.** Delete the record. If the design is hard to follow, improve the names, the
types, the tests, and the worked examples.

**When justified.** Never for the recoverable part. If some fact in the same decision is genuinely
external, that fact becomes a narrow record and the rest does not.

## Record justified by the importance of the decision

**Weak example.** "This was a major architectural decision, so it deserves a record."

**Why it fails.** Importance is not a property that makes a fact irrecoverable. The justification
names no fact that cannot live elsewhere, so it cannot be evaluated, and no future reader can
tell when the record stops applying.

**Risk.** A record set that grows monotonically, each entry plausible and none removable.

**Improved direction.** Name the exact fact that no artifact can carry. If naming it is not
possible, that is the answer.

**When justified.** Never. The valid categories are external mandate, irreversible or externally
expensive commitment, a rejected alternative whose rejection depends on evidence the code does
not carry, a decision no single system owns, accepted residual risk, and a compatibility
obligation from shipped behavior.

## Active record with no owner and no end

**Weak example.** A record states a constraint, names no accountable role, and has no condition
under which it stops applying.

**Why it fails.** Nobody can retire it, so it survives every review that does not specifically
attack it. Its authority grows with age because nothing about it changes while everything around
it does.

**Risk.** An obsolete constraint enforced by deference.

**Improved direction.** Give every active record an owner, a revalidation trigger, an obsolescence
condition, and links to the artifacts that are authoritative for current behavior.

**When justified.** Never for an active record. A record with no owner belongs in the archive,
marked as not current authority.

## Historical record used to block an improvement

**Weak example.** A reviewer rejects a change by citing a record from three years ago, without
checking whether its constraint still holds.

**Why it fails.** The record states what was decided under conditions that held then.
Discoverability is not authority, and the person proposing the improvement is asked to argue
against a document that may describe nothing current.

**Risk.** Improvement blocked by an expired constraint, and the reviewer's citation treated as
governance because it is written down.

**Improved direction.** Confirm the constraint is still applicable, the revalidation condition is
satisfied, and the implementation still depends on it. If confirmation is not available, record
the citation as an open question rather than as a constraint.

**When justified.** When the constraint is confirmed current, in which case the confirmation and
its date belong in the review record.

## Accepted proposal kept as a specification

**Weak example.** An accepted RFC continues to be cited for what the system does, after doctrine
and code have carried the contract for several releases.

**Why it fails.** The proposal described an intended change at a point in time. Maintaining it as
a description of current behavior creates the reconciliation obligation the doctrine exists to
remove.

**Risk.** A maintainer implementing to the proposal rather than to the contract.

**Improved direction.** Cite doctrine and the executable artifacts for current behavior, and cite
the proposal for its decision, date, owners, conditions, and rejected alternatives.

**When justified.** For the decision history itself, which is rationale and has no other home.

## Rationale inferred from the implementation

**Weak example.** A reviewer cannot find why a constraint exists, reconstructs a plausible motive
from the surrounding code, and writes it into the documentation as the reason.

**Why it fails.** The inference is indistinguishable from a recorded decision at the point of use.
It will be cited to defend the constraint, and the actual reason may have been different or may
have expired years ago.

**Risk.** A fabricated authority, created in good faith, that outlives every person who could
correct it.

**Improved direction.** Record the rationale as unknown. If the inference is useful, label it as
an inference, name its evidence, and state that the governing reason is unavailable.

**When justified.** Never unlabelled. A labelled inference with its evidence is honest and useful.

## Local guarantee presented as an external fact

**Weak example.** A design states that a record was persisted because a local transition consumed
its handle and produced a persistable value.

**Why it fails.** The local guarantee describes one process. The durable fact is owned by a store
that was never consulted, and no local mechanism can establish it.

**Risk.** A system that reports success for an effect that did not occur, with a type signature
that appears to justify the report.

**Improved direction.** Name the external system authoritative for the fact and the check that
consults it, and keep the local claim scoped to the sequence it actually proves.

**When justified.** Never. The two claims are different classes and stay separate.

## Doctrine cited for what the program currently permits

**Weak example.** A reviewer answers "can this call happen before that one" by quoting a doctrine
package rather than by reading the trait bounds that decide it.

**Why it fails.** Doctrine states obligations. What a program currently permits is decided by the
program, and a doctrine sentence can be correct about the obligation while the code has drifted
from it, or stale while the code is right.

**Risk.** A review that certifies the obligation and misses the violation.

**Improved direction.** Cite the enforcing artifact for what the program permits, and cite
doctrine for whether that is what it ought to permit. Both answers are needed; they are answers
to different questions.

**When justified.** Never as a substitute. Doctrine remains the authority for the obligation, for
the review process, and for who may change the contract.

## Deleting rationale in the name of executable architecture

**Weak example.** A cleanup removes a design note explaining why an alternative was rejected,
citing this doctrine's preference for executable artifacts.

**Why it fails.** The prohibition covers manually maintained copies of _enforced_ claims. A
rejected alternative, an external constraint, and an accepted risk are enforced by nothing and
have no other home, so removing them destroys the only record of them.

**Risk.** A future team re-adopting a rejected alternative because the reason for rejecting it was
deleted as duplication.

**Improved direction.** Trim the parts that restate the enforced topology, keep the parts that
cannot be recovered, and name the enforcing artifact where the two meet.

**When justified.** Never for irrecoverable rationale. Removing a restatement of the enforced
graph from the same document is the correct edit.

---

## Source: `doctrines/0011-executable-narrative/glossary.md`

# Glossary

Terms whose meaning here is narrower than ordinary architecture usage. Shared vocabulary lives in
the foundations.

**Executable narrative**
: An architectural obligation carried by the mechanism that enforces it, in a form a reader can
follow as the domain's own account of what the system does. Narrower than "self-documenting
code": the obligation is enforced, not merely described, and legibility is a requirement of the
enforcing artifact rather than a hoped-for property of it.

**Authority partition**
: The assignment of each class of architectural claim to exactly one kind of authority. Narrower
than "separation of concerns": the partition is about which artifact settles a disputed claim, not
about how responsibilities are divided.

**Operational authority**
: The artifact that decides what a system currently does, for the claims it mechanically
enforces. Distinct from governance authority, which decides who may change the contract, and from
external authority, which owns durable and remote facts.

**Enforced claim**
: A claim a mechanism rejects violations of. A claim that a mechanism merely describes is not
enforced, however precisely it is written.

**Maintained representation**
: A description of a claim that a person has to update when the claim changes. A generated view
is not a maintained representation; a hand-updated table is, whether or not anyone updates it.

**Derived view**
: A human-readable projection of a machine-readable source, produced by a generator and checked
for drift. A view produced from a hand-written description of the same claim is not derived; the
description is a maintained representation with a generator attached.

**Competing copy**
: A manually maintained artifact that restates an enforced claim as an independently editable
normative source. It is the object of `RUST-DOC-0011-R004`, and it is defined by editability
rather than by format.

**Irrecoverable rationale**
: A reason that cannot be reconstructed safely from the artifacts: an external constraint, a
rejected alternative and the evidence for rejecting it, an irreversible commitment, an accepted
risk, and who accepted it. Narrower than "why the code is like this", most of which the code and
its tests answer.

**Decision record**
: A durable artifact recording irrecoverable rationale for one decision, with an owner, a
revalidation trigger, an obsolescence condition, and links to the artifacts authoritative for
current behavior. Narrower than the general architecture-decision-record practice this doctrine
restricts: here a record is the residue left after the executability test, not the default
artifact of a decision.

**Active set**
: The decision records currently claimed as authority, enumerated in a machine-readable registry.
Membership is what makes a record citable; a file in the archive is not in the active set.

**Archival record**
: A record retained for a stated compatibility or audit obligation, marked as not current
operational authority and excluded from generated agent context.

**Revalidation trigger**
: A named event on whose occurrence a record's constraint is re-examined. A calendar date is one
form; a dependency major version, a regulatory renewal, an architecture-boundary change, or a
migration completion are others.

**Obsolescence condition**
: The stated condition under which a record stops applying and is expired or archived. Distinct
from a revalidation trigger: the trigger prompts a check, the condition ends the record.

**Historical veto**
: The authority a discoverable but unconfirmed record acquires when it is cited against a change.
`RUST-DOC-0011-R010` removes it by requiring current applicability to be confirmed first.

**Executability test**
: The assessment required before a record is created, asking which part of the decision can be
represented, enforced, generated, or recovered from the artifacts, and which part cannot.

**Representation count**
: The number of maintained representations of one claim. It is the number of places a future
change has to be made correctly, and `RUST-DOC-0011-R017` makes it a stated review quantity.

**Labelled inference**
: A reconstructed reason recorded as an inference, with its evidence, alongside a statement that
the governing rationale is unknown. The only permitted form of an unrecorded reason under
`RUST-DOC-0011-R013`.

## Glossary review

- every normative term is defined in the foundations or here;
- no definition implies a stronger guarantee than the artifact establishes;
- the difference between enforced and described is preserved in every entry;
- local vocabulary is marked as local and attributed to its established family;
- abbreviations expand on first use;
- links point to the authoritative rule or foundation.

---

## Source: `doctrines/0011-executable-narrative/references.md`

# References

References identify where a mechanical fact or an established practice comes from. They do not
transfer authority to this doctrine's obligations, which are repository governance.

## Practices this doctrine restricts

The architecture decision record was introduced by Michael Nygard as a lightweight, numbered file
capturing the context, decision, and consequences of one architecturally significant choice, and
was later collected and extended by Joel Parker Henderson and others. The form is cited here for
what it is, not as an endorsement of the restriction this doctrine places on it. Nygard's original
proposal was a reaction to unread architecture documents and already argued for small, dated,
single-decision files; the additional obligations here, that a record must name the fact no
artifact can carry, must carry an owner and an end condition, and must not be cited without
confirming applicability, are this repository's.

Documentation-generation practice, in which a human-readable view is produced from the artifact
it describes rather than maintained beside it, is long established across tooling ecosystems.
Rust's own `rustdoc` and the doc-test mechanism are the instance nearest to hand: the
documentation is derived from the item it documents, and the examples in it are compiled and run,
so a divergence fails the build rather than accumulating. That mechanism is cited as an existence
proof for `RUST-DOC-0011-R005`, not as a claim that every derived view can be produced so cheaply.

## Mechanisms an obligation can move into

Rust language mechanics for visibility, module privacy, traits and their bounds, associated
items, and move semantics are cited from the Rust Reference and the Rust Book, maintained by the
Rust project under their published terms, for the pinned toolchain 1.97.1 and the minimum
supported version 1.85.0, checked 2026-08-04. These supply the construction restrictions,
capability boundaries, and ordering constraints that `RUST-DOC-0011-R002` prefers over prose.

PostgreSQL documentation is cited for base types, domains, constraints, and explicit casts, which
carry a nominal distinction between identifier species into the schema so that comparing two of
them requires a stated conversion. The claim borrowed here is only the mechanical one; persistence
obligations are governed by RUST-DOC-0005.

JSON Schema Draft 2020-12 is cited for the machine-readable validation of the decision-record
registry, consistent with the doctrine and agent-pack manifests already validated in this
repository.

## Related repository material

`foundations/guarantee-honesty.md` supplies the discipline that separates a claim from its
limits, which is what `RUST-DOC-0011-R003` relies on when it requires the unenforced part of a
claim to be stated separately. `foundations/evidence.md` supplies the evidence classes the
decision framework selects between. `foundations/complexity-budget.md` supplies the assessment
`RUST-DOC-0011-R002` requires before an obligation is left prose-carried.

RUST-DOC-0010 applies this doctrine's partition to staged protocols in `RUST-DOC-0010-R022`, and
its `RUST-DOC-0010-R018` and `RUST-DOC-0010-R019` are worked instances of an obligation moved into
an executable artifact because prose could not detect its violation.

## Research limit

This package records the sources actually used. It does not claim exhaustive coverage of
architecture-documentation literature, knowledge-management research, technical-debt studies, or
the empirical work on documentation decay. The absence of a source is not a judgment about it.

No quotation long enough to require separate license analysis is reproduced. No external media,
transcript, or specification text is mirrored. Repository licensing applies to the original
doctrine prose here and makes no claim over the cited works.

---

## Source: `patterns/README.md`

# Representation patterns

Patterns are reusable design mechanisms, not universal prescriptions. Each
pattern begins with a problem and forces, compares a weak and improved
representation, and states both guarantees gained and guarantees not gained.
Boundary, persistence, testing, and complexity sections prevent a local type
shape from being mistaken for a complete system proof.

| Pattern                                             | Primary fit                                                               | Common overapplication                               |
| --------------------------------------------------- | ------------------------------------------------------------------------- | ---------------------------------------------------- |
| [Sum types](../patterns/sum-types.md)                           | mutually exclusive runtime states                                         | variant explosion for independent dimensions         |
| [Opaque newtypes](../patterns/opaque-newtypes.md)               | one value with a stable local invariant                                   | names stronger than construction evidence            |
| [Smart constructors](../patterns/smart-constructors.md)         | checked establishment and normalization                                   | incomplete checks split across callers               |
| [Typestate](../patterns/typestate.md)                           | small, locally controlled protocol sequence                               | persisted or externally determined state             |
| [Capability types](../patterns/capability-types.md)             | possession represents authority                                           | cloneable handles with undefined revocation          |
| [Consuming transitions](../patterns/consuming-transitions.md)   | prevent reuse of prior lifecycle state                                    | losing recovery evidence on fallible transition      |
| [Validated collections](../patterns/validated-collections.md)   | non-empty, bounded, sorted, or unique sets                                | mutation paths that invalidate the wrapper           |
| [Hybrid state machines](../patterns/hybrid-state-machines.md)   | local typed workflow plus dynamic persistence                             | duplicated state without conversion contract         |
| [Explicit uncertainty](../patterns/explicit-uncertainty.md)     | external effect may have indeterminate outcome                            | treating unknown as generic error                    |
| [Successor capabilities](../patterns/successor-capabilities.md) | one capability, several implementations with differing successor evidence | bounds widened until the protocol edge is decorative |
| [Executable narrative](../patterns/executable-narrative.md)     | placing an architectural obligation in the mechanism that enforces it     | deleting rationale that no artifact could carry      |

## Selection rule

Choose the simplest mechanism that directly protects the consequential
invariant:

- mutually exclusive states: sum type;
- refined scalar or identifier: opaque newtype and smart constructor;
- collection invariant: validated wrapper;
- locally controlled sequence with few states: typestate or consuming
  transition;
- multi-stage sequence whose capabilities have several implementations with
  differing successor evidence: successor capabilities;
- authority: capability;
- dynamic, heterogeneous, persisted, or externally observed state: runtime
  enum/state machine;
- external effect: `Result` plus explicit unknown/reconciliation state where
  execution can be ambiguous.

Executable narrative is not an alternative to the mechanisms above; it is the
question asked before choosing one. Which artifact should carry this obligation,
which is authoritative for it, and is any other maintained copy of it needed?

Patterns can combine. A payment workflow may use an opaque operation ID, a
capability for capture authority, consuming local transitions, a persisted
runtime status, and explicit unknown capture outcome. Each layer must name its
own evidence and avoid claiming the others' guarantees.

## Pattern review

Reviewers should ask:

1. Which invariant is protected?
2. Can every construction and mutation path preserve it?
3. What boundary re-establishes the evidence?
4. How is the value persisted and evolved?
5. Which external facts remain mutable?
6. Does the mechanism improve diagnostics?
7. Is the type/API complexity proportional to misuse frequency and impact?
8. Which executable evidence demonstrates admitted and prohibited behavior?

Executable examples live under [`../examples/`](../examples/). They illustrate
mechanics and limitations; they are not a substitute for a domain's own
invariant inventory.

---

## Source: `patterns/sum-types.md`

# Sum types

## 1. Problem

A record uses booleans and optional fields to describe mutually exclusive
states. Many combinations have no domain meaning: `paid = true` with no receipt,
`failed = true` while `paid = true`, or a failure reason attached to a pending
invoice. Every operation must rediscover which combinations are legal.

## 2. Forces

The state must be inspected at runtime, stored, serialized, logged, matched by
heterogeneous consumers, and evolved over releases. Each state carries different
data. Exhaustive handling is valuable, but public API compatibility and unknown
future persisted values matter. Independent dimensions should not be forced
into one enormous cross-product.

## 3. Weak representation

```rust
struct Invoice {
    paid: bool,
    failed: bool,
    receipt: Option<String>,
    failure: Option<String>,
}
```

The representation admits contradictory values and permits code to forget the
relationship among fields. A validator can reject combinations, but nothing
prevents later mutation from breaking them.

## 4. Improved representation

```rust
enum InvoiceState {
    Pending,
    Paid { receipt: ReceiptId },
    Failed { reason: FailureReason },
}
```

Associated data appears only where meaningful. Transitions replace the complete
state or occur through methods that enforce legal edges. Use separate enums for
independent dimensions rather than multiplying variants.

## 5. Exact guarantee gained

A value constructed through safe Rust is exactly one declared variant, and its
associated fields have the types required by that variant. Matching can be
exhaustive within the current crate/API contract. The representation rules out
the particular contradictory field combinations removed by the design.

## 6. Guarantees not gained

The enum does not prove the transition history was authorized, that associated
IDs exist, that a persisted row is current, or that external reality agrees.
Public construction may still permit an unauthorized `Paid` value. Variant data
can contain weaker invariants. Exhaustive matching today does not make a durable
protocol closed to future values.

## 7. Boundary considerations

Decode external discriminators into a raw representation first when unknown
values or invalid associated fields are possible. Validate each payload through
its constructors. Decide deliberately whether unknown fields/variants are
rejected, retained, or mapped to an explicit `Unknown` form. Authentication and
authorization occur separately from structural decoding.

## 8. Persistence considerations

Choose stable tags independent of incidental Rust spelling. Document rename,
addition, downgrade, and unknown-value policy. Database columns may use a tag
plus associated values, a JSON envelope, or normalized state tables; each needs
a constraint or fallible conversion that rejects invalid combinations. A
runtime enum is generally more suitable than typestate for heterogeneous stored
states.

## 9. Testing evidence

Test construction and behavior for every variant, legal transition edges, and
rejection of invalid raw combinations. Compile exhaustiveness helps when adding
variants inside a controlled codebase. Boundary fixtures must include unknown
tags, missing associated data, surplus fields, old versions, and invalid nested
newtypes. Migration tests preserve stable tags.

## 10. Costs

Variant additions can break exhaustive downstream matches. Large enums can
couple unrelated lifecycle dimensions. Serialization shape becomes a protocol.
Code may need adapters for storage and UI state. Associated payloads can
increase enum size, though boxing without measurement may create worse cost.

## 11. When not to use it

Do not use one enum for independent facts that can combine legitimately. Do not
generate a variant for every permutation when a product of smaller validated
types is clearer. Do not use a closed enum for open third-party identifiers
unless unknown values remain representable. Plain booleans remain appropriate
for genuinely independent binary properties.

## 12. Related doctrines

RUST-DOC-0001 requires sum types for mutually exclusive state.
RUST-DOC-0002 governs errors represented as enums. RUST-DOC-0005 governs stable
persistence and unknown values. RUST-DOC-0006 governs outcome enums whose
unknown state reflects distributed ambiguity.

## 13. Executable example

See [`../examples/domain-modeling/src/lib.rs`](../examples/domain-modeling/src/lib.rs)
for `InvoiceState`, and the invoice and payment case studies for boundary and
transition qualifications.

## 14. Worked application

Consider a support ticket with `Open`, `WaitingForCustomer { requested_at }`,
and `Resolved { resolution, resolved_at }`. A sum type prevents a resolved
ticket from carrying an active waiting request and makes the timestamp
requirements visible. Priority and confidentiality remain separate fields
because they can combine with every lifecycle state. Resolution authority
remains a transition service concern; making the variant public would let any
caller fabricate it.

When this ticket is stored, a raw row might contain `status`, `requested_at`,
`resolution`, and `resolved_at`. Fallible conversion matches the truth table and
constructs the enum. An unknown status from a newer writer is retained or
rejected according to compatibility policy. That conversion, not SQL text
decoding alone, establishes the associated-data invariant.

## 15. Review prompts

- Are the represented conditions truly mutually exclusive?
- Does each variant carry only data meaningful in that state?
- Are independent dimensions kept independent?
- Can public construction forge a transition that requires authority?
- Is external tagging stable across rename and rolling deployment?
- What happens when a newer variant reaches an older reader?
- Does every transition preserve required history and structured failure?
- Does the guarantee ledger distinguish enum shape from external truth?

---

## Source: `patterns/opaque-newtypes.md`

# Opaque newtypes

## 1. Problem

Primitive values such as `String` and `u64` carry too little evidence.
Unvalidated email input, a syntactically accepted address, and a
ownership-verified address may all share the same representation. Raw integers
can be mixed across units or admit zero where policy requires a positive value.

## 2. Forces

The invariant is stable and local to one value. Construction must be
centralized without making ordinary reads awkward. Values cross Serde,
database, logging, and API boundaries. Conversion and borrowing should remain
ergonomic. Type names must not imply evidence unavailable to constructors.
Occasional privileged imports may need explicit escape paths.

## 3. Weak representation

```rust
fn send(to: String, cents: u64) { /* ... */ }
```

Every caller can supply empty, malformed, zero, wrong-unit, or unchecked data.
Validation becomes scattered, may drift, and is easy to omit after decoding.

## 4. Improved representation

```rust
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn parse(input: String) -> Result<Self, EmailSyntaxError> {
        validate_syntax(&input)?;
        Ok(Self(input))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

The field is private. Construction runs the complete documented invariant.
Conversions are fallible; accessors expose only operations that cannot violate
the invariant.

## 5. Exact guarantee gained

Every instance obtainable through the public safe API has passed the specified
constructor policy, assuming no privileged escape path violated its obligation.
Different newtypes prevent accidental parameter mixing even when their bytes
match. Private representation centralizes invariant evolution and formatting
policy.

## 6. Guarantees not gained

An `EmailAddress` does not prove ownership, deliverability, or future validity.
A `NonZeroU64` proves nonzero, not correct currency, sufficient funds, or tax
policy. A locally validated identifier does not prove the referenced entity
exists. Newtype names must match the actual evidence level.

## 7. Boundary considerations

Deserialize through `try_from` or a manual implementation that calls the
constructor. Request DTOs and CLI values remain raw until validation. Protect
allocation and length before complex parsing. Debug and display output should
avoid accidental secret exposure. A verifier should produce a separate stronger
type rather than mutate the meaning of the weaker one invisibly.

## 8. Persistence considerations

Decode a physical row value then use `TryFrom`; do not let an ORM assign private
bytes unchecked. Schema constraints can reinforce stable predicates. Historical
invalid values become conversion failures or quarantine records. Persisted
normalization rules need versioning because later policy changes can alter
equality and uniqueness.

## 9. Testing evidence

Test accepted values, rejection partitions, exact bounds, normalization
idempotence, formatting/redaction, conversions, and boundary decoding.
Compile-fail tests can prove direct field construction is unavailable outside
the crate. Property tests can cover round trips and generator-defined valid
sets. Database fixtures must include invalid historical values.

## 10. Costs

Newtypes add conversion and API surface, can produce wrapper proliferation, and
may complicate generic code or serialization. Excessive micro-types obscure
data flow. Changing the invariant can become a migration. Private fields may
require deliberate borrowing and formatting implementations.

## 11. When not to use it

Do not wrap values without a meaningful invariant, unit, identity, secrecy, or
misuse distinction. Do not use a newtype to represent dynamic cross-entity
policy that requires external state. Do not name a value `Verified` when any
public parser can construct it. A type alias or plain primitive is appropriate
when interchangeability is intentional.

## 12. Related doctrines

RUST-DOC-0001 governs trusted construction and evidence-accurate naming.
RUST-DOC-0003 applies to secret and authority-bearing wrappers.
RUST-DOC-0005 governs checked decoding. RUST-DOC-0008 requires negative and
boundary evidence.

## 13. Executable example

See [`../examples/validated-newtypes/src/lib.rs`](../examples/validated-newtypes/src/lib.rs)
and [`../examples/domain-modeling/src/lib.rs`](../examples/domain-modeling/src/lib.rs).

## 14. Worked application

`PositiveMoney` can contain a `NonZeroU64` minor-unit amount and a `Currency`.
This proves a nonzero amount in one named currency. Checked addition first
requires currency equality and then handles integer overflow. The type still
does not choose an FX rate, tax regime, decimal scale migration, allocation
rule, or rounding policy. Those facts belong to services and explicit policies.

For an API request, the DTO may contain an integer and currency code. Parse the
currency, reject zero, then construct the newtype. For a database row, perform
the same checked conversion. Exposing `as_minor_units` is safe because reading
does not invalidate the value; exposing `&mut u64` would destroy the invariant.

## 15. Review prompts

- Is the private field inaccessible from every unprivileged module?
- Does every constructor establish every guarantee named by the type?
- Are parsing, normalization, policy acceptance, and external verification
  distinguished?
- Do Serde and database paths call the checked constructor?
- Can formatting leak sensitive representation?
- Are mutation and conversion paths invariant-preserving?
- Is an escape hatch visible, scoped, and audited?
- Would a type alias communicate the intended distinction just as well?

---

## Source: `patterns/smart-constructors.md`

# Smart constructors

## 1. Problem

A type documents an invariant but ordinary constructors, derived decoding, or
mutation permit instances that do not satisfy it. Checks appear near some
callers and are omitted near others. Parsing, normalization, and business policy
are combined without clear evidence levels.

## 2. Forces

Construction may begin from strings, bytes, database columns, or already parsed
values. Callers need actionable errors. Normalization may change representation.
Some checks are pure and stable; others require policy or external observation.
Rust conventions include `new`, `parse`, and `TryFrom`, but naming must reveal
fallibility and evidence.

## 3. Weak representation

```rust
pub struct BoundedName(pub String);

fn make_name(raw: String) -> BoundedName {
    BoundedName(raw)
}
```

Call sites may remember to check length, but public construction does not
protect the documented bound. A later Serde derive can bypass whatever checks
exist.

## 4. Improved representation

```rust
pub struct BoundedName(String);

impl TryFrom<String> for BoundedName {
    type Error = NameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let normalized = value.trim().to_owned();
        validate_name(&normalized)?;
        Ok(Self(normalized))
    }
}
```

Use `parse` when interpreting textual syntax, fallible `new` or `try_from` for
validated values, and a separate domain service when external policy or
cross-entity facts are required.

## 5. Exact guarantee gained

Every value created through the protected construction path has passed the
complete invariant implemented there, after the documented normalization.
Callers receive a stable structured error contract. Centralization makes review
and invariant changes discoverable.

## 6. Guarantees not gained

Constructor correctness is not proven merely by centralization. A pure
constructor cannot establish mailbox ownership, database uniqueness under
concurrency, authorization, or mutable external facts. Normalization may have
policy and collision consequences. A constructor does not protect later
mutation unless mutation APIs also preserve the invariant.

## 7. Boundary considerations

Boundary adapters should decode physical structure and delegate to the smart
constructor. Keep raw DTOs distinct when parsing and policy validation have
different error mappings. Apply size limits before allocating or normalizing
untrusted input. Do not expose internal validation diagnostics that leak
secrets or security distinctions beyond policy.

## 8. Persistence considerations

Every database reader calls the same constructor or an equivalent complete
validation function. Store normalized or original forms according to explicit
requirements; if both matter, represent both. Schema constraints reinforce
stable invariants. Policy changes require migration or a version-aware
constructor rather than silent reinterpretation.

## 9. Testing evidence

Create a boundary table: below, at, and above bounds; empty; malformed; valid;
normalization collision; and representative Unicode. Assert structured errors.
Property-test normalization idempotence and accepted-value round trips.
Integration-test Serde and database paths. Search for all construction sites and
compile-test private representation when consequential.

## 10. Costs

Fallible construction propagates errors and creates conversion code.
Normalization can allocate. A large constructor that checks unrelated external
policy becomes difficult to reuse and test. Too many synonymous constructors
can form bypasses or unclear evidence levels. Public error enums add API
compatibility obligations.

## 11. When not to use it

Do not hide fallible behavior behind an infallible-looking function. Do not put
database or network calls into a scalar constructor. Do not use one constructor
to produce a type named for stronger verification than it performs. Plain
construction is sufficient for unconstrained internal data.

## 12. Related doctrines

RUST-DOC-0001 defines complete protected construction. RUST-DOC-0002 governs
constructor errors. RUST-DOC-0005 governs persistence conversions.
RUST-DOC-0008 governs negative and property evidence.

## 13. Executable example

See [`../examples/validated-newtypes/src/lib.rs`](../examples/validated-newtypes/src/lib.rs)
and [`../examples/boundary-validation/src/lib.rs`](../examples/boundary-validation/src/lib.rs).

## 14. Worked application

An email syntax constructor can trim surrounding transport whitespace if policy
allows, cap total length, require one nonempty local and domain part, reject
control characters, and retain the normalized string. Its name should remain
`EmailAddress` or `SyntacticallyValidEmail`, not `VerifiedEmailAddress`.
Ownership verification consumes separate verifier evidence and constructs the
stronger type through a restricted path.

Constructor errors might distinguish empty input, excessive length, missing
separator, invalid local part, and invalid domain. An HTTP adapter can map these
to a public validation response while preserving the internal category. A
database import uses the identical parser and quarantines failures. No caller
reimplements `contains('@')`.

## 15. Review prompts

- Does function naming expose fallibility?
- Is validation complete or followed by undocumented caller checks?
- Is normalization performed before every dependent predicate?
- Can two inputs normalize to one identity, and is collision policy explicit?
- Are errors structured for callers without leaking secrets?
- Does `TryFrom` delegate to one canonical validation path?
- Are external facts kept out of pure constructors?
- Can any derived decoder or convenience conversion bypass construction?

---

## Source: `patterns/typestate.md`

# Typestate

## 1. Problem

A locally controlled handle supports operations only after certain transitions.
Calling `send` before `connect`, capturing before authorization, or reusing a
closed transaction is programmer misuse. Runtime checks repeat in every method
and discover the problem only during execution.

## 2. Forces

The protocol has few stable states and a mostly static API. One owner controls
the handle and transitions. State-specific methods improve callers. Transitions
may perform fallible I/O. The state may later need persistence, heterogeneous
collections, dynamic dispatch, or runtime inspection. Generic diagnostics,
compile time, and code size matter.

## 3. Weak representation

```rust
struct Connection {
    open: bool,
}

impl Connection {
    fn send(&mut self, bytes: &[u8]) -> Result<(), SendError> {
        if !self.open { /* runtime rejection */ }
        // ...
    }
}
```

Every method carries a branch, and the public API advertises operations that are
illegal for the current state.

## 4. Improved representation

```rust
struct Closed;
struct Open;

struct Connection<State> {
    transport: Transport,
    state: PhantomData<State>,
}

impl Connection<Closed> {
    fn connect(self) -> Result<Connection<Open>, ConnectError> { /* ... */ }
}

impl Connection<Open> {
    fn send(&mut self, bytes: &[u8]) -> Result<Receipt, SendError> { /* ... */ }
}
```

Marker zero-sized types carry compile-time local protocol evidence. Consuming
transitions prevent reuse of the prior handle.

## 5. Exact guarantee gained

Safe code holding `Connection<Closed>` cannot call methods implemented only for
`Connection<Open>`. A consuming transition can ensure the previous handle is no
longer usable after success or after a deliberately consuming attempt. The
state parameter records that the local transition returned successfully.

## 6. Guarantees not gained

`Connection<Open>` does not prove the remote peer remains reachable. `Authorized`
does not prove a later capture succeeds. Typestate does not establish external
effects, persisted status, or authorization unless construction actually
obtained that evidence. Fallible operations remain `Result`; timeouts may create
unknown outcomes.

## 7. Boundary considerations

External input determines runtime facts and must first decode into a runtime
representation. Do not deserialize a marker state directly from an untrusted
tag. A checked restoration service may inspect a persisted status, validate
resources and authority, then issue a local handle whose proof is accurately
scoped.

## 8. Persistence considerations

Generic state types are awkward for heterogeneous rows and evolving storage.
Persist a stable runtime enum and associated evidence. Rehydrate typestate only
through checked code. A hybrid state machine often uses typestate for one
in-process operation and a runtime enum for durable lifecycle. Never treat
serialized marker spelling as proof.

## 9. Testing evidence

Unit-test successful and failed transitions, resource cleanup, and state
payloads. Compile-fail tests prove illegal state-specific calls and consumed
handle reuse. Integration-test external failure after a locally successful
transition. Test decoding/restoration separately. Measure generic
monomorphization if the state/API set is large.

## 10. Costs

State parameters spread through signatures, trait bounds, mocks, and error
types. Fallible consuming transitions need careful recovery ergonomics. Many
orthogonal states create a type cross-product. Dynamic dispatch and containers
may require erasure. Monomorphization increases code size. Compiler diagnostics
can become harder than a clear runtime enum.

## 11. When not to use it

Do not use typestate for externally determined, frequently inspected, persisted,
large, plugin-defined, or highly dynamic states. Do not use it when callers
must switch over heterogeneous states at runtime. Do not encode every boolean
property as a marker. Prefer a runtime enum or plain validation when complexity
exceeds prevented misuse.

## 12. Related doctrines

RUST-DOC-0001 limits typestate to proportionate locally controlled sequencing.
RUST-DOC-0003 covers ownership-consuming authority. RUST-DOC-0004 governs async
fallibility and cancellation. RUST-DOC-0005 governs persistence.

## 13. Executable example

See [`../examples/typestate/src/lib.rs`](../examples/typestate/src/lib.rs) and
the compile-fail cases under [`../examples/compile-fail/ui/`](../examples/compile-fail/ui/).

## 14. Worked application

A `Connection<Closed>` owns local transport configuration. `connect(self)`
attempts I/O and returns `Connection<Open>` only after the local connect
protocol reports success. `send(&mut self)` remains fallible because the peer
can disappear immediately. `close(self)` may also fail; its error must state
whether the caller receives recoverable local ownership or only an uncertain
cleanup result.

This API is appropriate for one owned connection. It is less suitable for a
dashboard holding thousands of persisted connection statuses or a plugin API
whose states are discovered dynamically. There, a runtime enum with explicit
operations usually yields clearer storage, dispatch, and diagnostics.

## 15. Review prompts

- Is state locally controlled rather than merely observed externally?
- Is the graph small enough to understand in signatures?
- Does each fallible consuming transition preserve recovery evidence honestly?
- Can marker construction be forged?
- Are remote liveness and success claims explicitly excluded?
- Must instances be persisted, erased, or stored heterogeneously?
- Would a runtime enum produce simpler caller behavior?
- Has monomorphization and diagnostic complexity been assessed?

---

## Source: `patterns/capability-types.md`

# Capability types

## 1. Problem

An operation requires authority, but APIs accept ambient identifiers, booleans,
or a broadly privileged service handle. Any caller that can name a resource can
attempt the operation. Authorization is repeatedly checked, easy to omit, and
hard to trace through helper layers.

## 2. Forces

Authority may be scoped to action, resource, tenant, time, or workflow. It must
be hard to forge and easy to pass to authorized code. Cloning, transfer,
revocation, expiry, serialization, logging, and task movement affect meaning.
External policy can change after issuance. Some operations require one-time or
exclusive authority.

## 3. Weak representation

```rust
fn capture(payment_id: PaymentId, authorized: bool) -> Result<Receipt, Error> {
    if !authorized { /* reject */ }
    // ...
}
```

The flag is forgeable, carries no scope or provenance, and can separate from the
authorization decision. A generic client with all methods grants excess
authority to every caller.

## 4. Improved representation

```rust
pub struct CapturePermit {
    payment: PaymentId,
    grant: GrantId,
    expires_at: Instant,
}

impl PaymentAuthorizer {
    pub fn authorize_capture(
        &self,
        principal: &AuthenticatedPrincipal,
        payment: PaymentId,
    ) -> Result<CapturePermit, AuthorizationError> {
        // Check policy, then construct private fields.
    }
}
```

The capture API requires possession of `CapturePermit`. Constructors remain in
the authorization component. Methods expose only permitted operations.

## 5. Exact guarantee gained

Safe callers outside privileged construction cannot forge the capability's
private representation. An API requiring the capability cannot be called
accidentally with only an identifier. Non-`Clone` ownership can express
single-custody transfer, and consuming methods can prevent local reuse.

## 6. Guarantees not gained

Possession does not prove external policy remains current, a session has not
been revoked, clocks agree, or the downstream effect will succeed. A cloneable
capability is not exclusive. A serialized token can be copied unless the
receiver enforces use and revocation. Rust privacy does not constrain privileged
database or raw network access outside the process.

## 7. Boundary considerations

Authentication establishes principal evidence; authorization constructs a
scoped capability. Do not deserialize an authority-bearing Rust type directly
from an untrusted request. Verify signatures, audience, issuer, expiry, replay,
and revocation before construction. Redact secrets and avoid derived formatting
that exposes bearer material. Map authorization failure without leaking
sensitive policy.

## 8. Persistence considerations

Persist an authorization grant or token record only when recovery needs it.
Store stable identity, scope, issuance evidence, expiry, version, and revocation
state; minimize secrets. Rehydrate through the authorizer rather than raw row
decoding. Revocation requires a runtime observation or an effect resource that
checks current grant state.

## 9. Testing evidence

Compile-fail test private construction and disallowed methods. Unit-test scope,
resource mismatch, expiry boundaries, consuming use, and non-clone behavior.
Integration-test authentication-to-authorization conversion and revoked/stale
grants. Audit every privileged constructor and serialization path. Concurrency
tests cover duplicate use when one-time semantics matter.

## 10. Costs

Capabilities add types, issuance services, dependency threading, and explicit
revocation checks. Fine-grained types can multiply. Non-clone authority can be
ergonomically harder across async tasks. Bearer capabilities can increase
security risk if logged or leaked. Distributed enforcement needs storage or a
remote authority, not only a Rust wrapper.

## 11. When not to use it

Do not create capability types for ordinary data access with no authority
distinction. Do not wrap a forgeable boolean and call it authority. Do not use a
long-lived capability when policy must be checked on every action. A normal
authorization service call may be clearer for dynamic, cross-entity decisions.

## 12. Related doctrines

RUST-DOC-0003 defines ownership as authority and lifecycle. RUST-DOC-0001
requires protected construction and honest evidence names. RUST-DOC-0004 covers
transfer across tasks. RUST-DOC-0006 covers leases, stale authority, and
fencing.

## 13. Executable example

The authenticated-session case study demonstrates principal-to-capability
conversion. Payment types in
[`../examples/compile-fail/src/lib.rs`](../examples/compile-fail/src/lib.rs)
demonstrate capture authority in a local protocol.

## 14. Worked application

A shutdown coordinator can issue one `ShutdownPermit` to the component allowed
to stop admission and join workers. The permit is non-`Clone`, carries the
service generation, and is consumed by `begin_shutdown`. This prevents ordinary
request code from invoking the transition and makes transfer across a
supervisor task explicit. It does not guarantee every task cooperates or every
external resource closes.

For revocable payment authority, the capability can carry a grant identity
rather than a bearer secret. The capture boundary checks current grant status
and payment version before acting. The Rust value improves local least
privilege; the runtime check handles revocation and stale policy.

## 15. Review prompts

- Who alone can construct the capability?
- Which action, resource, tenant, and lifetime does it cover?
- Is cloning semantically safe?
- Can serialization turn it into an uncontrolled bearer value?
- How are transfer, consumption, expiry, and revocation enforced?
- Can logs or debug output leak authority?
- Does downstream enforcement check the capability's identity?
- Which external policy can change after issuance?

---

## Source: `patterns/consuming-transitions.md`

# Consuming transitions

## 1. Problem

After commit, close, capture, submit, or token redemption, the prior handle must
not be reused. A mutable method can update an internal flag, but aliases or
missed checks retain an apparently usable value. The compiler cannot help if the
API leaves the old state alive.

## 2. Forces

One owner controls the lifecycle. The transition may be infallible, fallible
before any change, partially executed, or externally ambiguous. Callers may need
the original handle back after a recoverable failure. Resource cleanup and
destructors matter. Persistent state remains a separate runtime concern.

## 3. Weak representation

```rust
impl Transaction {
    pub fn commit(&mut self) -> Result<(), CommitError> {
        self.committed = true;
        // ...
    }
}
```

The same handle remains callable. Every later method must inspect a flag.
Aliases through shared containers make lifecycle authority unclear.

## 4. Improved representation

```rust
impl ActiveTransaction {
    pub fn commit(self) -> Result<CommitReceipt, CommitFailure> {
        // self cannot be used by the caller after the attempt
    }
}
```

For a failure known to occur before the transition, return
`Result<Committed, (ActiveTransaction, CommitError)>`. For ambiguous external
execution, consume the handle and return an explicit unknown/reconciliation
state rather than restoring authority dishonestly.

## 5. Exact guarantee gained

After passing an owned value to a consuming method, safe caller code cannot use
that same value again. A returned successor type can expose only successor
operations. Returning the original handle only on proven pre-transition failure
can preserve ergonomic retry without weakening lifecycle evidence.

## 6. Guarantees not gained

Consumption does not prove an external commit, close, or capture occurred. Drop
does not guarantee remote rollback. Another separately constructed handle may
still act on the same resource. Persistence may contain stale status. Unsafe or
privileged constructors can bypass local lifecycle if misused.

## 7. Boundary considerations

External responses determine which successor evidence can be constructed.
Authenticate response source and preserve structured errors. If response loss
makes execution unknown, return an unknown outcome with stable operation
identity. Cancellation of an async consuming transition needs a guard or owner
that accounts for the consumed resource.

## 8. Persistence considerations

Store runtime lifecycle state and optimistic version independently. A local
consuming handle can own a transaction connection, while the database remains
the authority for durable status. Rehydration checks current state and issues a
new handle; it cannot deserialize the old moved value. Commit ambiguity needs
database-specific reconciliation.

## 9. Testing evidence

Compile-fail tests demonstrate reuse after move is rejected. Unit tests cover
every transition result, returned recovery handle, and destructor behavior.
Fault tests cancel or disconnect at each partial step. Concurrent tests ensure
one operation identity cannot be claimed twice where required. Boundary tests
preserve unknown rather than returning the prior state.

## 10. Costs

Ownership-consuming APIs can complicate error handling and chaining. Returning a
handle with error creates larger types. Async transitions may need background
completion or reconciliation. Generic successor states can spread through API
signatures. Callers sometimes need runtime collections of mixed states, where a
single runtime enum is simpler.

## 11. When not to use it

Do not consume immutable value objects merely for style. Do not return a prior
handle after an error if the transition may already have happened. Do not force
consumption where shared observation is the real model. A mutable runtime state
machine is appropriate when many actors inspect or coordinate one persisted
entity.

## 12. Related doctrines

RUST-DOC-0001 governs legal transitions and external fallibility.
RUST-DOC-0003 governs custody and RAII. RUST-DOC-0005 covers transaction
lifecycle and commit ambiguity. RUST-DOC-0006 covers unknown outcomes.

## 13. Executable example

See transaction and connection flows in
[`../examples/typestate/src/lib.rs`](../examples/typestate/src/lib.rs) and the
reuse compile-fail case under
[`../examples/compile-fail/ui/`](../examples/compile-fail/ui/).

## 14. Worked application

A single-use invitation token can be redeemed by passing ownership to
`redeem(self)`. Validation failure before contacting storage can return
`(self, ValidationError)` if correcting local context and retrying is safe.
After an atomic storage claim is attempted, returning the original token would
misrepresent its status; the result should be redeemed, rejected as already
used, or unknown with a lookup identity.

Similarly, a database `commit(self)` blocks local reuse but cannot declare the
remote outcome after connection loss. Consumption protects the caller's handle
lifecycle. A transaction operation ID and database observation protect the
distributed outcome.

## 15. Review prompts

- Is reuse actually invalid and consequential?
- At which point does the old authority cease to be truthful?
- Can a failure safely return the original value?
- Does cancellation consume, recover, or reconcile the resource?
- Are destructor side effects limited to what RAII can guarantee?
- Can a second handle target the same external resource?
- Is persistent versioning needed in addition to local ownership?
- Would a clear runtime guard be more ergonomic at equal safety?

---

## Source: `patterns/validated-collections.md`

# Validated collections

## 1. Problem

Ordinary collections admit states the domain excludes: empty recipient lists,
too many batch items, duplicate identifiers, unsorted ranges, mismatched
currencies, or totals beyond policy. Validating once is ineffective if public
mutation can later violate the predicate.

## 2. Forces

The invariant may concern length, uniqueness, ordering, per-element validity, or
a relationship among elements. Construction can come from iterators and
deserialization. Mutations need validation. Consumers want slices and
iteration. Large inputs need bounded allocation and efficient validation.
Persistence and schema representations may not enforce the same property.

## 3. Weak representation

```rust
type Recipients = Vec<EmailAddress>;

fn send_all(recipients: Recipients) {
    assert!(!recipients.is_empty());
}
```

The alias supplies no construction boundary. Checks repeat at use sites. Any
caller can pass empty or oversized data, and later mutation can introduce
duplicates.

## 4. Improved representation

```rust
pub struct NonEmptyRecipients(Vec<EmailAddress>);

impl TryFrom<Vec<EmailAddress>> for NonEmptyRecipients {
    type Error = RecipientSetError;

    fn try_from(items: Vec<EmailAddress>) -> Result<Self, Self::Error> {
        if items.is_empty() { return Err(RecipientSetError::Empty); }
        if items.len() > 100 { return Err(RecipientSetError::TooMany); }
        validate_unique(&items)?;
        Ok(Self(items))
    }
}
```

Expose read-only slice/iterator access and only mutation methods that preserve
the complete invariant.

## 5. Exact guarantee gained

Every safely constructed wrapper satisfies the documented collection predicate
at construction. If all mutation paths are private and checked, the predicate
continues to hold. Algorithms can rely on non-empty, bounded, sorted, or unique
properties without repeating checks.

## 6. Guarantees not gained

The wrapper does not prove cross-entity existence, current permissions, external
capacity, or relationships omitted from its constructor. `NonEmpty` does not
mean maximum size is safe. Uniqueness depends on the chosen equality and
normalization. Sorted order does not prove intervals do not overlap unless
checked separately.

## 7. Boundary considerations

Apply an input element and total length limit before or during collection.
Validate each element through its own boundary constructor, then validate the
aggregate. Stream when an untrusted declared length could allocate excessive
memory. Preserve errors that identify category and safe index without logging
sensitive contents.

## 8. Persistence considerations

Database child rows may require a transaction to validate the aggregate and
prevent concurrent changes. Unique indexes can reinforce uniqueness; ordering
usually needs an explicit key. Loading a subset must not construct a wrapper
named for the complete set. Version large persisted aggregates or validate
under locking/optimistic concurrency.

## 9. Testing evidence

Test empty, minimum, maximum, over-maximum, duplicates, normalization collisions,
sorted/unsorted, and invalid elements. Property-test every public mutation to
ensure the invariant remains. Test `collect`/iterator failure and short-circuit
behavior. Boundary tests include deceptive length declarations and historical
invalid rows.

## 10. Costs

Construction may require allocation, sorting, hashing, or a full scan.
Incremental mutation can be complex. Hiding the underlying vector removes some
standard APIs. Large aggregate validation may need transactional locking.
Overly broad aggregate invariants can make harmless local edits expensive.

## 11. When not to use it

Do not wrap a collection with no consequential aggregate invariant. Do not
claim completeness when pagination returns a subset. Do not enforce a
cross-entity fact using a pure collection wrapper. Plain slices and iterators
are appropriate for transient processing where the caller already owns the
invariant.

## 12. Related doctrines

RUST-DOC-0001 covers invalid empty and bounded states. RUST-DOC-0005 governs
aggregate persistence and concurrent validation. RUST-DOC-0008 recommends
property tests for mutation-preserved invariants. RUST-DOC-0009 governs
validation cost claims.

## 13. Executable example

The bounded-name implementation in
[`../examples/validated-newtypes/src/lib.rs`](../examples/validated-newtypes/src/lib.rs)
demonstrates the scalar analogue. Case studies apply bounded and deduplicated
sets at message and invoice boundaries.

## 14. Worked application

An `Allocation` wrapper can require at least one line, unique account IDs, one
currency, and basis-point shares totaling exactly 10,000. Construction validates
elements first, then aggregate uniqueness and total using checked arithmetic.
Read-only iteration is safe. `push` is absent because arbitrary insertion could
break uniqueness and totals; a domain method replaces the complete allocation
after revalidation.

Persistence may store one row per line. Loading through pagination cannot issue
`Allocation`, because a page is not the complete set. The repository reads the
versioned aggregate in a transaction or snapshot, validates all rows, and only
then constructs the wrapper.

## 15. Review prompts

- Is the invariant about each element, the aggregate, or both?
- Are every constructor and iterator-collection path fallible?
- Can `DerefMut`, mutable slices, or retained aliases break the predicate?
- Are length and allocation bounded before accepting input?
- Does uniqueness use normalized domain equality?
- Can partial database reads masquerade as a complete collection?
- How are concurrent child-row updates coordinated?
- Would validation at one operation site be simpler than a persistent wrapper?

---

## Source: `patterns/hybrid-state-machines.md`

# Hybrid state machines

## 1. Problem

A workflow benefits from compile-time local sequencing, but its durable and
external state must be loaded, listed, inspected, and changed by multiple
processes. Pure typestate is awkward for persistence; a runtime enum alone
allows local operation misuse. Treating either mechanism as universally
superior loses useful guarantees.

## 2. Forces

The local operation has a small controlled sequence. Durable lifecycle has more
states, concurrent actors, history, unknown outcomes, and schema evolution.
Workers recover after process loss. APIs and UIs inspect heterogeneous states.
External effects are fallible and may be ambiguous. Conversion between local
evidence and stored status must be protected.

## 3. Weak representation

One weak design serializes typestate marker names and reconstructs them without
validation. Another exposes one mutable `PaymentStatus` enum to every local
method, so `capture` can be attempted from `Draft` and checked repeatedly.
Both confuse in-process protocol evidence with durable authority.

## 4. Improved representation

Use a stable runtime enum:

```rust
enum PaymentStatus {
    Draft,
    Validated,
    Authorized { authorization: AuthorizationId },
    CaptureUnknown { operation: OperationId },
    Captured { receipt: CaptureReceipt },
}
```

A repository atomically claims an eligible version and issues a local
`AuthorizedPayment` handle. Its consuming `capture` returns a runtime outcome
that is persisted through a checked transition. Rehydration always starts from
the persisted enum and current authority checks.

## 5. Exact guarantee gained

The local handle prevents state-inappropriate method calls during one owned
operation. The runtime enum represents heterogeneous persisted and externally
observed states explicitly. A checked conversion boundary ensures typestate is
issued only after current durable state and version satisfy the claim.

## 6. Guarantees not gained

Local typestate does not lock the database or prevent another worker unless the
claim protocol does. Persisted status does not make external reality current.
Conversion does not guarantee a later effect. Unknown outcomes remain possible.
Two representations can diverge if updates are not transactionally coordinated.

## 7. Boundary considerations

HTTP and message inputs request transitions; they do not choose trusted
successor states. Authenticate, authorize, load current status, check version,
then construct the local handle. External responses produce confirmed or
unknown outcomes. UIs receive runtime state and may not use frontend types as
backend authorization.

## 8. Persistence considerations

Persist stable state tags, version, evidence IDs, and reconciliation data.
Claim work through optimistic update, row lock, lease/fencing, or durable queue.
Write successor status atomically with local durable effects and outbox intent.
Invalid historical combinations fail row conversion. Migration covers every
variant and old reader.

## 9. Testing evidence

Compile-fail test illegal local transitions. Unit-test conversion from each
eligible and ineligible runtime state. Integration-test concurrent claims,
version conflicts, process restart, and persisted unknown outcomes. Fault-test
external execution around persistence. Property-test state graph edges and
invariant preservation.

## 10. Costs

Two state representations require mapping code and a clear source of truth.
Typestate generics add signatures; runtime enums add checked conversions.
Recovery code must handle expired local authority. Incorrect duplication can
create two transition graphs that drift. Documentation and tests must connect
every local successor to durable transition.

## 11. When not to use it

Do not use a hybrid when the workflow is entirely local and ephemeral; typestate
or a consuming handle may suffice. Do not add typestate when all operations are
dynamic service commands and runtime state already provides clear errors. Do
not duplicate state merely to demonstrate Rust types.

## 12. Related doctrines

RUST-DOC-0001 defines proportional mechanism choice. RUST-DOC-0004 governs work
claims and cancellation. RUST-DOC-0005 governs durable state and optimistic
concurrency. RUST-DOC-0006 governs unknown external outcomes.

## 13. Executable example

The typestate mechanics are in
[`../examples/typestate/src/lib.rs`](../examples/typestate/src/lib.rs). The
payment-lifecycle and database-transaction case studies demonstrate the hybrid
mapping and residual uncertainty.

## 14. Worked application

A payment worker loads `PaymentStatus::Authorized`, atomically changes an
optimistic version to a claimed state, and receives an
`AuthorizedCaptureWork` handle containing the operation ID. Its consuming
capture method can return confirmed receipt, confirmed rejection, or unknown.
The repository persists that runtime outcome and outbox intent. Process loss
does not require serializing the marker type; another worker begins from durable
status and claim rules.

The checked conversion is the critical bridge. If code can construct
`AuthorizedCaptureWork` from any row or request tag, the local typestate becomes
forged. If code changes only the local handle without persisting outcome, the
runtime source of truth diverges.

## 15. Review prompts

- Which representation is authoritative at each lifecycle phase?
- What exact evidence issues the local typed handle?
- Does a durable claim prevent concurrent workers?
- Is every local successor mapped to a runtime transition?
- Can process loss leave a recoverable claimed state?
- Are unknown outcomes persisted before retry?
- Do migrations cover old and new runtime variants?
- Does the hybrid remove enough local misuse to justify two models?

---

## Source: `patterns/explicit-uncertainty.md`

# Explicit uncertainty

## 1. Problem

An external operation can execute while its acknowledgement is lost. A timeout
or connection error does not reveal whether the effect occurred. Mapping the
observation to success or failure invents certainty and makes later retry
unsafe.

## 2. Forces

Callers need actionable outcomes. Confirmed success and rejection carry
different evidence. Unknown states need stable identity, persistence,
reconciliation, age, and ownership. Idempotency can make replay safer within a
defined boundary but does not automatically cover every effect. User interfaces
must prevent harmful repeat actions while still showing progress.

## 3. Weak representation

```rust
fn capture(...) -> Result<Receipt, CaptureError>;
```

If `CaptureError::Timeout` is handled as rejection, callers may submit a new
capture. If it is handled as success, records may claim money moved when it did
not. A string error loses reconciliation identity.

## 4. Improved representation

```rust
pub enum CaptureOutcome {
    Confirmed(CaptureReceipt),
    Rejected(CaptureRejection),
    Unknown {
        operation_id: OperationId,
        reconciliation: ReconciliationToken,
    },
}
```

A separate outer error may represent failure proven before dispatch. Unknown is
a durable lifecycle state, not a transient logging category.

## 5. Exact guarantee gained

The type prevents exhaustive callers from treating unknown as either confirmed
terminal result without an explicit branch. The unknown variant guarantees
availability of the fields its constructor requires, such as operation identity
and reconciliation token. State transitions can restrict resolution to new
evidence.

## 6. Guarantees not gained

The type does not determine the external result, make reconciliation succeed,
or make retry safe. A token may reference stale or incomplete data. Confirmed
success proves only the external boundary and time represented by its evidence;
later reversal or settlement may remain possible.

## 7. Boundary considerations

Classify where failure occurred. Only protocol evidence can establish
pre-dispatch non-execution. Authenticate provider responses and callbacks.
Bind idempotency keys to request fingerprints. Protect reconciliation records
from secret leakage. API error mapping must preserve unknown rather than
collapsing it into generic service failure.

## 8. Persistence considerations

Persist operation ID, external key, target, request fingerprint, attempt
history, current evidence, age, next observation, and optimistic version.
Index pending reconciliation. Retain idempotency and deduplication evidence
longer than retry/replay horizons. Operator overrides are audited decisions, not
retroactive proof.

## 9. Testing evidence

Fault-test loss before dispatch, after dispatch, after remote execution, and
after acknowledgement. Assert that only the ambiguous cases become unknown.
Test repeated reconciliation: still unknown, then confirmed or rejected.
Test concurrent reconcilers, idempotent attempts, retention expiry, and UI/API
behavior that prevents blind repeat.

## 10. Costs

Unknown adds states, storage, worker ownership, user messaging, monitoring, and
support procedures. Reconciliation can consume external quota and remain
unavailable. Stable identifiers and retention increase data-management burden.
Every downstream match must handle the additional state.

## 11. When not to use it

Do not add unknown to pure local validation or failures proven before an effect
boundary. Do not use it as a vague replacement for structured errors. A
best-effort telemetry action may accept permanent loss without reconciliation,
provided the contract states that. Conversely, do not omit unknown merely to
simplify a consequential API.

## 12. Related doctrines

RUST-DOC-0002 preserves actionable error categories. RUST-DOC-0005 governs
durable operation records. RUST-DOC-0006 supplies the complete retry,
idempotency, and reconciliation rules. RUST-DOC-0008 governs fault evidence.

## 13. Executable example

See [`../examples/distributed-outcomes/src/lib.rs`](../examples/distributed-outcomes/src/lib.rs)
and the payment, message-delivery, and database-transaction case studies.

## 14. Worked application

An email provider may accept a send request and lose the acknowledgement. The
application stores `DeliveryUnknown { operation_id, provider_key, first_attempt
}` rather than `Failed`. A reconciler queries provider status or consumes a
provider event. If the provider cannot supply final evidence, policy may keep
the state unknown and warn the user that a repeat could duplicate delivery.

The same pattern applies to database commit ambiguity, but reconciliation
sources differ. A transaction ID, durable business key, or follow-up read may
establish committed state. The generic unknown shape should not hide
domain-specific evidence or safe action.

## 15. Review prompts

- At what exact point can execution become ambiguous?
- Which response proves rejection, and is its source authenticated?
- Does unknown retain stable logical and external identity?
- Can reconciliation itself return stale or unknown evidence?
- Which retry is safe before and after observation?
- How are concurrent reconcilers controlled?
- What do API and UI callers do while pending?
- Is retention long enough for delayed acknowledgements and replay?
- Are terminal operator decisions recorded as policy rather than fabricated
  proof?

---

## Source: `patterns/successor-capabilities.md`

# Successor capabilities

## 1. Problem

A protocol has several stages, and more than one way to enter it. A self-service signup and an
invitation-based signup both reach the same availability check, but each carries different
evidence: a challenge identifier in one case, an invitation and its issuing account in the other.

Plain typestate returns one concrete successor. Serving both entry paths then requires either one
successor type carrying every possible proof as an option, which reintroduces the contradictory
combinations the stages were built to remove, or a duplicated protocol per entry path. Neither
expresses the actual contract: whatever the entry stage produces, it must be something the
availability check accepts.

## 2. Forces

The protocol has few stable stages, controlled by one owner within one process. A capability may
have several implementations whose successors differ in evidence but agree on what comes next.
Transitions may perform fallible work. The stage graph will be refactored by people who did not
design it. Diagnostics, monomorphization, and the reach of generic parameters into helper and
test code all matter.

## 3. Weak representation

```rust
pub trait Canonicalize {
    /// Returns a value that can then be identity-checked.
    fn canonicalize(self) -> Result<CanonicalRegistration, CanonicalizeError>;
}
```

The successor is hardcoded, so a second implementation cannot carry different evidence. Worse,
the sentence that makes this a protocol lives in a doc comment: nothing checks that
`CanonicalRegistration` can in fact be identity-checked, and nothing notices when it stops being
able to.

## 4. Improved representation

```rust
pub trait Canonicalize: Sized {
    type Next: CheckIdentity;
    type Error;

    fn canonicalize(self) -> Result<Self::Next, Self::Error>;
}

impl Canonicalize for SelfServiceSubmission {
    type Next = CanonicalRegistration<SelfServiceOrigin>;
    type Error = CanonicalizeError;
    // ...
}

impl Canonicalize for InvitedSubmission {
    type Next = CanonicalRegistration<InvitedOrigin>;
    type Error = CanonicalizeError;
    // ...
}
```

The successor is now part of the contract and bounded by the capability it must satisfy. Two
implementations produce different successors carrying different origin evidence, and both are
statically required to lead into the identity check.

**Local name.** This repository's local name for the chainable trait-oriented form is _Chainable
Telescopic Typestate Traits_, abbreviated CT³. A chain gives order, `A → B → C`. A telescope gives
containment: A holds the controlled opening into B, and B holds the controlled opening into C. The
associated successor type is that opening, which is why the present capability carries both proof
of a completed history and permission for a constrained future. The term is local vocabulary and
not standardized terminology; the established families it refines are typestate-oriented
programming, behavioral types, and object protocols, and `RUST-DOC-0010-R021` requires that
attribution to travel with the name. Prefer the descriptive terms in new material, and use the
abbreviation only where a reader arriving from an older internal document needs the bridge.

A branching stage names one successor per outcome:

```rust
pub trait CheckIdentity: Sized {
    type Available: AcceptPolicy;
    type Conflicting: ResolveConflict;
    type Error;

    fn check_identity(
        self,
        directory: &IdentityDirectory,
    ) -> Result<IdentityOutcome<Self::Available, Self::Conflicting>, Self::Error>;
}
```

## 5. Exact guarantee gained

Safe code cannot advance a stage through a capability that stage does not implement, cannot reuse
a stage after a consuming transition, and cannot construct a later stage's evidence when its
fields are private and no public constructor exists. The associated bound additionally guarantees
that every implementation's successor satisfies the next capability, which a hardcoded return
type cannot state and a doc comment cannot enforce.

The bound is the protocol edge in checkable form. A refactor that redirects `type Next` elsewhere
does not merely change a type: it either fails to satisfy the bound, or it silently changes the
graph, which is why the pattern is completed by an executable topology assertion.

## 6. Guarantees not gained

Reaching a later stage proves the in-process protocol ran in order and nothing else. An
availability observation proves no conflict was visible to one reader at one moment, not that the
identity is still free when a row is written. A consent proof records that an offered version
matched the version in force during the check, not that the policy is unchanged at write time.

Most importantly, a consuming transition does not consume a durable fact. A move ends the
caller's use of a local value; a stored row is read into a value and can be read again by another
worker, so two workers can each hold a consumed handle for the same row. Durable advancement
needs identity, stored state, and a version or fencing token re-checked at the authoritative
store.

## 7. Boundary considerations

Untrusted input enters at the first stage and is canonicalized before any stage claims a checked
value. Do not derive a decoder that produces a later stage: a decoded value asserts every proof
that stage represents, and the decoder performed none of them. Where restoration is genuinely
needed, a checked service inspects stored state and issues a typed stage whose claim is scoped to
what it actually verified.

Erase the protocol at one named boundary. A round trip through a map or a dynamic context in the
middle of the graph ends static enforcement for every stage after it, while leaving the
appearance of a typed protocol intact.

## 8. Persistence considerations

Persist a runtime representation, not the stage type. Stage markers are compile-time artifacts;
their spelling in a column is not protocol evidence, and heterogeneous rows do not fit generic
stage types comfortably. A hybrid design is the norm: a runtime enum owns the durable lifecycle,
and the typed protocol covers one in-process pass issued through checked construction. See
[hybrid state machines](../patterns/hybrid-state-machines.md).

## 9. Testing evidence

Unit-test each transition on success and failure, each branch variant, each recovery edge, and
the survival of canonical values from first stage to last. Add compile-fail cases for the
orderings the protocol claims are unrepresentable: skipping a stage, reusing a consumed stage,
and constructing stage evidence from a literal.

Then assert the topology executably, because compile-fail cases alone do not cover it. A
redirected associated type can leave every existing negative test passing while the edge it
protected no longer exists.

Two assertions are needed, and the difference between them is easy to get wrong. A **contract**
assertion knows only the stage capability and demands that its associated successor satisfies the
next one. Nothing in the helper supplies that bound, so it compiles only while the trait declares
it:

```rust
fn assert_canonicalize_contract<S: Canonicalize>() {
    fn requires_check_identity<T: CheckIdentity>() {}
    requires_check_identity::<S::Next>();
}
```

An **edge** assertion additionally pins the concrete successor:

```rust
fn assert_canonicalize_edge<S, N>()
where
    S: Canonicalize<Next = N>,
    N: CheckIdentity,
{
}
```

The edge form cannot replace the contract form. Its own `N: CheckIdentity` bound silently
supplies whatever the trait lost, so deleting `type Next: CheckIdentity` from `Canonicalize`
leaves a suite of edge assertions entirely green. Write both: contract assertions for the trait
obligations, edge assertions for the concrete graph.

## 10. Costs

Bounds make signatures longer and first-encounter diagnostics worse: a mismatch is reported as an
unsatisfied bound rather than a plain type error. Generic stage parameters travel into helper
functions, mocks, and sometimes public APIs. Each stage adds a type, a failure type, a ledger
row, and an assertion. Monomorphization grows with stages multiplied by implementations.

## 11. When not to use it

Do not use it when one capability will only ever have one implementation, where a concrete
successor return is simpler and equally safe. Do not use it for advisory ordering, for state
determined externally or chosen at runtime, for durable multi-actor lifecycle, or where callers
must hold heterogeneous stages in one collection. Do not add a stage for a transformation that
establishes no fact a later stage consumes. A short pipeline of ordinary functions is often the
better answer.

## 12. Related doctrines

RUST-DOC-0010 governs this mechanism, its bounds, branches, effect disclosure, erasure boundary,
and the limit at which a local transition stops being durable evidence. RUST-DOC-0001 governs
legal transitions and unrepresentable states generally. RUST-DOC-0002 governs the error taxonomy
the stage-specific failures map into. RUST-DOC-0003 governs custody of the values being advanced.
RUST-DOC-0004 governs cancellation of async transitions. RUST-DOC-0005 and RUST-DOC-0006 govern
the durable and ambiguous halves this pattern defers. RUST-DOC-0011 governs which artifact is
authoritative for each claim the protocol makes, and prohibits maintaining a prose copy of the
graph beside the traits that enforce it; see
[executable narrative](../patterns/executable-narrative.md).

## 13. Executable example

See [`../examples/staged-protocol/src/lib.rs`](../examples/staged-protocol/src/lib.rs) for the
capability traits, two entry implementations with different successors, the branch and recovery
edges, and the topology assertion. Compiler-rejection cases are under
[`../examples/compile-fail/ui/`](../examples/compile-fail/ui/).

## 14. Worked application

A registration protocol canonicalizes a submission, checks identity availability, records policy
consent, and prepares a persistable value. Two entry stages produce canonical registrations
carrying different origin evidence; both satisfy the identity check.

The check branches. Availability leads to the policy stage; conflict leads to a resolution stage
whose revision edge is bounded back to the first capability, so a revised submission re-enters at
canonicalization rather than skipping ahead. A directory that cannot be reached is neither
branch: it is a stage-identifying failure carrying the address, so an operator can look the
attempt up.

Origin evidence is erased to a runtime discriminant exactly once, at the persistence boundary,
and the protocol stops at a persistable value. It does not claim the row was written. That
remains a durable operation which re-checks identity and state under its own concurrency control.

## 15. Review prompts

- Does each nonterminal capability name its successor as a bounded associated type?
- Does any bound name a capability the successor does not actually establish?
- Was a bound widened or removed to make an implementation compile?
- Will more than one implementation produce a different successor, or is a concrete return simpler?
- Is each material branch a named sum over distinct successors?
- Does a revision edge re-enter at the correct stage?
- Is an undetermined outcome distinguishable from both branches?
- Can any conversion, derive, or public constructor produce a later stage?
- Does any nonterminal stage derive `Clone` or `Copy`, allowing a copy to advance separately?
- Does the documented graph have a contract assertion, not only edge assertions?
- Does deleting a successor bound actually break the build?
- Is any local transition being presented as durable evidence?

---

## Source: `patterns/executable-narrative.md`

# Executable narrative

## 1. Problem

An architectural obligation has to be somewhere. A team decides that authentication precedes
authorization, that a transaction identifier is never compared with a wallet identifier, or that a
capture cannot be attempted twice. Each of these is a sentence someone can write down, and each is
also something a mechanism could reject.

Written down, the sentence is cheap and immediately readable, and nothing contradicts it when the
code stops matching. Enforced, it changes when the system changes and fails loudly when it is
violated, but it is only useful if a reader can still follow it. The problem is not choosing
between the two. It is deciding which artifact settles which claim, and then not maintaining a
second editable copy of the answer.

## 2. Forces

Obligations differ in whether a mechanism exists for them. Some are enforceable by types, some
only by a schema, some only by a deployment check, and some by nothing available. Readers differ
too: an implementer wants the mechanism, a reviewer wants the reason, an auditor wants the owner.
Records outlive the constraints that produced them, and discoverability turns into authority
without anybody deciding that it should. Generation removes a synchronization obligation but adds
a generator, and a generator fed by a hand-written description has removed nothing. Agents hydrate
whatever context is offered and cannot distinguish a current constraint from an expired one unless
the artifacts do.

## 3. Weak representation

A design document, maintained by hand, states the obligation and describes the enforced structure
beside it:

```text
## Registration protocol

Stages run in this order:

  1. Canonicalize    -> produces CanonicalRegistration
  2. Check identity  -> produces AvailableRegistration or ConflictingRegistration
  3. Accept policy   -> produces ConsentedRegistration
  4. Prepare         -> produces PersistableRegistration

Authentication must precede authorization. Transaction identifiers must never be
compared with wallet identifiers.
```

Three separate failures live in that block. The stage list duplicates a graph the compiler already
enforces, so the two can diverge with no signal. The ordering sentence is enforceable and is not
enforced, so it is a description of an intention. The identifier sentence is enforceable in the
type system and in the schema, and is enforced in neither.

## 4. Improved representation

Move each obligation into the mechanism that rejects violations of it, and keep the document for
what only it can carry.

The ordering becomes the successor bound, so a stage that stops leading anywhere legal fails to
compile:

```rust
pub trait Authenticate: Sized {
    type Next: Authorize;
    type Error;

    fn authenticate(self) -> Result<Self::Next, Self::Error>;
}
```

The negative guarantee becomes a rejected program rather than a claim of impossibility:

```rust
// examples/compile-fail/ui/skip_protocol_stage.rs
// Reaching authorization without authenticating is a compiler error, and the
// committed diagnostic is the evidence that it still is.
```

The identifier distinction becomes nominal in both type systems, so mixing two species requires a
conversion somebody wrote on purpose:

```rust
pub struct TransactionId(Uuid); // private field, checked constructor
pub struct WalletId(Uuid);
```

```sql
-- The same distinction, carried into the schema. A comparison across species
-- needs an explicit cast, so it is visible in review rather than silent.
CREATE DOMAIN transaction_id AS uuid;
CREATE DOMAIN wallet_id      AS uuid;
```

What the document keeps is the part no mechanism carries: why this ordering was chosen, what the
stages deliberately do not prove, which residual risks were accepted, and who accepted them.

## 5. Exact guarantee gained

For each obligation moved into a mechanism, a violation now fails: at compile time for a bound or
a visibility restriction, at test time for a rejected program or input, at deploy time for a
schema constraint or a machine-checked manifest. The claim and its enforcement change together
because they are the same artifact, so the class of defect where a correct document describes an
incorrect system is removed for that obligation.

For each derived view that is generated and drift-checked, divergence from its source fails the
build. That is a guarantee about currency, and it holds without anybody remembering.

## 6. Guarantees not gained

An enforced obligation is not thereby a correct obligation. The compiler agrees that the graph is
the graph; it has no opinion about whether that graph matches the business process, which stays a
review judgment.

A generated view is current, not correct. It faithfully reflects a source that may itself be
wrong, and generating it removes drift without removing error.

An empty decision-record set is evidence about the record set. A constraint nobody recorded leaves
no trace in a registry that holds only what somebody chose to record, so absence of records is not
absence of constraints.

Legibility is not proved by anything mechanical. `RUST-DOC-0011-R016` states the obligation, and
review is the only check on it.

## 7. Boundary considerations

The enforcing mechanism changes at each boundary, and the authority moves with it. Inside the
process, types and visibility carry the obligation. At the wire, the canonical encoder, the
decoder, the schema, and the compatibility suite carry it. At the database, schema constraints,
checked decoding, and transaction predicates carry it. At deployment, machine-checked
configuration carries it.

A claim that crosses into another system's ownership stops being an in-process claim at that
point. Committed state, remote acknowledgment, provider status, current policy, lock ownership,
and settlement are owned by systems that have to be asked, and no local artifact can be cited for
them.

## 8. Persistence considerations

Persist the fact, not the enforcement artifact. A stage marker, a type name, and a trait bound are
compile-time artifacts whose spelling in a column establishes nothing. The durable representation
is a runtime model, and the obligation at the persistence boundary is carried by constraints,
checked decoding, and predicates.

The schema is a mechanism in its own right, and often the strongest one available for an
obligation that several services share. A nominal distinction expressed only in one service's
types is enforced for that service; expressed as a base type or domain in the schema, it is
enforced for every writer.

## 9. Testing evidence

Test what the mechanism rejects, not only what it accepts. A compile-fail fixture for a claimed
impossibility, a rejected-input case for a checked constructor, and a rejected-cast case for a
schema distinction are each the evidence that the negative guarantee is real. A green suite of
positive tests demonstrates that the ordinary path works and says nothing about the obligation.

Add a drift check for each generated view. Generation without a check is a convention; generation
with a check is a guarantee that the view matches its source at build time.

An assertion or gate that has never been observed failing is not evidence. Delete the thing it
protects, confirm the build breaks, and restore it. That step is what distinguishes a real check
from a decoration, and it is cheap.

## 10. Costs

Enforcement costs are real and specific. A bound lengthens signatures and reports a mismatch as an
unsatisfied bound rather than as a plain type error. A schema constraint moves a failure from a
readable domain message into a driver error. A generated view adds a generator to maintain and a
drift check that fails on unrelated changes until its cause is understood. Classification adds a
step to every review, and counting representations adds another.

The offsetting saving is that the common outcome of the executability test is that no record is
written at all, and every record not written is a permanent artifact nobody has to revalidate,
expire, or reconcile.

## 11. When not to use it

Do not force enforcement where the cost exceeds the consequence. An advisory ordering, a
convention with no failure mode, and a preference that has never been violated are all reasonable
to leave in prose with the assessment recorded.

Do not generate a view whose generator would need a hand-maintained description of the same
claim. That input is the competing copy under another name, and an informative hand-written
diagram with a named owner is the more honest artifact.

Do not use this pattern as an argument for deleting rationale. The prohibition covers copies of
enforced claims. A rejected alternative, an external constraint, and an accepted risk have no
other home, and removing them destroys the only record of them.

Do not read it as a prohibition on decision records. The residue is real, and pushing it into a
commit message or an issue thread relocates the fact to somewhere with no owner and no expiry.

## 12. Related doctrines

RUST-DOC-0011 governs this pattern, the authority partition, the prohibition on competing copies,
and the decision-record policy. RUST-DOC-0010 applies the partition to staged protocols in
`RUST-DOC-0010-R022`, and its compile-fail and topology-assertion rules are worked instances of an
obligation moved into an artifact because prose could not detect its violation. RUST-DOC-0001
governs which invariants are representable at all. RUST-DOC-0005 governs the persistence
mechanisms an obligation moves into at the database boundary. RUST-DOC-0006 governs the ambiguity
that remains once a claim becomes external. RUST-DOC-0008 governs which evidence class supports
which claim.

## 13. Executable example

This repository is its own worked instance of the generated-view half.
`tools/bundle-agent-context` builds every [generated distribution](README.md) from the canonical
sources named in [`../manifest/doctrines.yaml`](../manifest/doctrines.yaml) and
[`../manifest/agents.yaml`](../manifest/agents.yaml), stamps each output with a banner naming the
canonical roots, and its `check` mode fails on drift. No file under `dist/` is edited by hand, and
the drift check is part of the ordinary validation set.

`tools/doctrine-lint` is the enforcement half. It validates the decision-record registry at
[`../manifest/decision-records.yaml`](../manifest/decision-records.yaml) against its schema and
against the obligations `RUST-DOC-0011-R007` states, so a record without an owner, a revalidation
trigger, an obsolescence condition, or resolvable executable authorities fails the build rather
than a review.

No protocol-graph generator is shipped. A generator that derived the stage graph from the trait
definitions would be a further instance of this pattern; one fed by a hand-written edge list would
be the competing copy this pattern prohibits, wearing the word "generated". The graph obligation
is carried instead by the contract and edge assertions in
[`../examples/staged-protocol/src/lib.rs`](../examples/staged-protocol/src/lib.rs), which fail the
build when an edge changes.

## 14. Worked application

**A decision that needs no record.** A team decides that authentication precedes authorization.
The executability test finds a mechanism for every part of it: the ordering becomes a successor
bound, the impossibility of skipping becomes a committed compile-fail diagnostic, and the graph
becomes a contract assertion that fails if the bound is deleted. Nothing remains that an artifact
cannot carry, so no record is written. Someone who later asks why the order is what it is finds a
short comment and a doctrine cross-reference; someone who asks whether the order holds reads the
bound.

**A decision that needs a narrow record.** A system is required to keep a class of data inside one
jurisdiction because of an obligation a regulator and a customer contract impose. Deployment
regions, storage endpoints, and replica placement are enforceable, and they become
policy-as-code with a machine-checked manifest. What no artifact carries is the interpretation of
the obligation, the accountable owner, and the condition under which it lapses.

That residue is one narrow record. It states the single question it answers, states that it does
not govern the deployment topology, links the policy-as-code that is authoritative for current
behavior, names the owner, and names a revalidation trigger and an obsolescence condition. It is
registered in the active set so it can be audited and expired. The worked form is in
[`../decisions/examples/justified-data-residency.md`](../decisions/examples/justified-data-residency.md).

**A record that should not exist.** A proposed record titled "Authentication must happen before
authorization" restates an obligation the successor bound enforces. It names no fact an artifact
cannot carry, so it fails the test at the first question and is not written. The rejected form,
with the reasoning, is in
[`../decisions/examples/rejected-authentication-order.md`](../decisions/examples/rejected-authentication-order.md).

## 15. Review prompts

- Which class does this claim belong to, and which single artifact is authoritative for it?
- Is there a mechanism that could reject a violation, and was it used?
- If the mechanism enforces part of the claim, is the remainder stated separately?
- How many maintained representations of this claim exist, and which are neither authoritative,
  generated, nor irrecoverable rationale?
- Is any derived view synchronized by hand, and could it be generated instead?
- Would the generator need a hand-maintained description of the same claim?
- Does every generated artifact declare its source, and does a drift check cover it?
- Which fact in this proposed record cannot be represented, enforced, generated, or recovered?
- Who owns the record, what event revalidates it, and what condition ends it?
- Is this record actually a change proposal, and therefore an RFC?
- Was any record cited against a change without confirming its constraint still applies?
- Is any recorded reason an inference from the implementation rather than a governing rationale?
- Does any local guarantee stand in for a durable, remote, or externally governed fact?

---

## Source: `boundaries/README.md`

# Trust-boundary guides

Boundary guides operationalize the shared pipeline:

```text
untrusted representation
    ↓ parse
structural representation
    ↓ validate
trusted domain representation
    ↓ execute
external side effect
    ↓ observe / reconcile
new trusted evidence or explicit uncertainty
```

Validation is relocated and centralized; it is not eliminated. A wire decoder
can establish valid JSON structure while domain construction rejects a zero
amount. A database driver can establish a valid SQL integer while a money type
rejects the value or currency combination. A successful local filesystem write
does not by itself prove durable storage after power loss. Every guide separates
these evidence levels.

## Guides

| Boundary                                  | Primary concerns                                                       |
| ----------------------------------------- | ---------------------------------------------------------------------- |
| [Serde](../boundaries/serde.md)                         | checked deserialization, versioning, allocation limits                 |
| [Database decoding](../boundaries/database-decoding.md) | raw rows, domain conversion, migration, concurrency                    |
| [HTTP and RPC](../boundaries/http-and-rpc.md)           | DTOs, authentication/authorization, idempotency, error mapping         |
| [Messaging](../boundaries/messaging.md)                 | duplicates, ordering, acknowledgement, replay, schema evolution        |
| [Configuration](../boundaries/configuration.md)         | startup validation, secrets, defaults, reload                          |
| [Filesystem](../boundaries/filesystem.md)               | path trust, symlinks, TOCTOU, atomic replacement, durability           |
| [FFI](../boundaries/ffi.md)                             | ABI, representation, ownership, unwind, allocator and error boundaries |

## Required boundary record

For each implementation record:

1. untrusted bytes or values;
2. parser and physical limits;
3. structural DTO/row/foreign type;
4. domain validations and constructors;
5. alternate and privileged bypass paths;
6. structured failure mapping;
7. unknown/version evolution policy;
8. sensitive-data handling;
9. positive, negative, and integration evidence;
10. external or temporal facts the boundary cannot establish.

## Layering rule

Do not expose raw transport, row, or foreign representations as trusted domain
types merely to reduce conversion code. Do not duplicate domain policy in every
adapter; adapters call the canonical constructor and map its structured errors.
Conversely, keep protocol-only concerns such as body size, field presence,
schema version, and ABI layout at the boundary.

Authentication and authorization are separate evidence transitions.
Deserializing a principal identifier does not authenticate it. Authentication
does not grant every operation. An authorized capability remains time- and
scope-limited when policy can change.

## Review use

Apply [`../reviews/boundary-review.md`](../reviews/boundary-review.md) and the
relevant doctrine package. Trace all generated derives, ORM mappings, raw SQL,
administrative imports, cache paths, feature-gated implementations, and test
helpers. A single unchecked construction path weakens the type's claim
repository-wide.

---

## Source: `boundaries/serde.md`

# Serde boundary guide

## 1. What is untrusted?

Every byte sequence or data model supplied by a request, message, file,
environment-derived document, cache, or old persisted blob is untrusted. A
successful Serde decode establishes compatibility with the selected Serde data
model and deserializer behavior, not current domain validity. Declared lengths,
nested structures, map keys, duplicate fields, enum tags, and textual encodings
can be hostile or historically incompatible.

## 2. What parsing occurs?

The format parser establishes syntax and physical Rust representations:
integers, strings, sequences, maps, and a raw version envelope. Enforce
transport and decompression limits before Serde where possible. Configure
recursion, input length, collection length, and numeric behavior through the
format implementation. A format parser must not allocate based solely on an
untrusted length without a reviewed bound.

Parsing and normalization are distinct. Preserve raw text long enough to apply
the intended Unicode, case, whitespace, and canonicalization policy.

## 3. What validation occurs?

Validate required combinations, numeric bounds, identifiers, collection
invariants, stable version, and cross-field state after structural decoding.
Use a raw DTO when multiple fields jointly establish a domain state. Scalar
newtypes use their canonical parser or `TryFrom`.

```rust
#[derive(Deserialize)]
#[serde(try_from = "RawEmail")]
pub struct EmailAddress(String);
```

The `TryFrom<RawEmail>` implementation must call the complete constructor, not
reproduce a weaker subset.

## 4. How is a trusted type constructed?

Preferred routes are `#[serde(try_from = "...")]`, manual `Deserialize` that
delegates to a checked conversion, or a remote derive applied to a raw adapter.
`into` is suitable for trusted-to-wire serialization; `try_from` is appropriate
when wire-to-domain conversion can fail. Manual visitors are justified for
streaming, allocation control, compatibility, or precise diagnostics, but still
finish at protected domain construction.

Construction errors map into the deserializer's error type while retaining a
stable safe category in application code.

## 5. How can construction be bypassed?

Risk paths include deriving `Deserialize` directly on private-field newtypes,
adding a default that invents evidence, exposing an unchecked `From<String>`,
using `serde_json::Value` followed by field assignment, test-only constructors
enabled in production features, and custom visitors that write representation
without validation. An unsafe layout shortcut is never an acceptable decoder.

Search every `Deserialize`, `from_value`, raw cache decode, and compatibility
adapter. Privacy alone does not protect against code inside the defining module.

## 6. How is failure represented?

Separate syntax failure, unsupported version, structural mismatch, domain
validation, size/resource rejection, and security policy when callers act
differently. Public error responses can combine sensitive distinctions while
internal error chains retain safe context. Never panic on malformed external
data. Do not replace an unknown future variant with an arbitrary current
variant.

Batch decoding should report whether processing is atomic, partial, or
quarantined, with bounded diagnostics.

## 7. How are unknown or future values handled?

Choose deliberately among reject, ignore, retain, and explicit unknown. `deny_unknown_fields`
is a compatibility and security policy, not a universal default: it detects
misspellings and surplus fields but prevents additive forward compatibility.
Ignoring unknown fields can hide client mistakes. Retaining them can support
round-trip proxies but expands memory and sensitive-data exposure.

Version durable envelopes. Test old-reader/new-writer and
new-reader/old-writer combinations. Use stable external enum tags rather than
incidental variant spelling.

## 8. How is sensitive data protected?

Avoid derived `Debug` and broad serialization on secret-bearing domain types.
Use dedicated response DTOs so internal secrets cannot be emitted accidentally.
Limit error excerpts; never log complete tokens, credentials, private keys, or
regulated payloads. Unknown fields may themselves be sensitive and require the
same retention policy as known fields. Zeroization claims remain limited by
copies made by parsers and allocators.

## 9. How is evidence tested?

Test valid fixtures, malformed syntax, missing and duplicate fields, unknown
fields/variants, old versions, bounds, excessive nesting, large declared
lengths, normalization collisions, and invalid nested newtypes. Property tests
can exercise round-trip and parser non-panic properties with bounded generators.
Fuzz the raw decoder when input exposure and parser risk justify it.

Compile-time privacy evidence must be paired with integration tests proving
Serde uses the checked constructor. Review snapshot changes semantically.

## 10. What remains uncertain?

Deserialization does not prove authentication, authorization, referenced entity
existence, mailbox ownership, remote state, current policy, or later external
success. A version accepted today can become obsolete. An authenticated message
can still carry invalid business intent. Record these non-guarantees in the
boundary ledger and establish stronger evidence through the appropriate domain
service.

## Decision table

| Situation                    | Preferred approach                                    |
| ---------------------------- | ----------------------------------------------------- |
| scalar with local invariant  | raw scalar plus `try_from`                            |
| cross-field state            | raw DTO then domain conversion                        |
| untrusted large sequence     | streaming visitor with explicit cap                   |
| long-lived format            | versioned envelope and compatibility fixtures         |
| proxy retaining unknown data | bounded extension map with sensitivity policy         |
| secret-bearing output        | dedicated response DTO, no broad domain serialization |

## Review prompts

- Does every trusted target use a fallible checked conversion?
- Which byte, nesting, sequence, and decompression limits apply before allocation?
- Is unknown-field policy compatible with both security and rolling deployment?
- Can defaults or aliases create evidence not present in the input?
- Do serialization and debug implementations expose sensitive domain fields?
- Which external facts remain outside deserialization evidence?

---

## Source: `boundaries/database-decoding.md`

# Database decoding boundary guide

## 1. What is untrusted?

Rows are untrusted domain representations even when the database is controlled.
They may come from old application versions, alternate writers, administrative
repair, incomplete migration, import, replication, backup restoration, relaxed
constraints, or corruption. The driver can prove only that a column decoded
according to its SQL type and protocol. It does not prove current value,
cross-column, or lifecycle invariants.

## 2. What parsing occurs?

The driver parses wire values into a raw row model using physical types such as
integer, text, timestamp, nullable column, or byte buffer. Checked SQL mappings
should avoid lossy casts and surprising numeric truncation. Queries name columns
explicitly when schema order is not a stable contract. Apply row, blob, and
batch limits; paginate large results.

Keep database nullability in the raw model rather than weakening the domain
type.

## 3. What validation occurs?

`TryFrom<RawRow>` validates scalar newtypes, discriminator/associated-field
truth tables, collection bounds, timestamps, versions, and local relationships.
Cross-row or cross-entity invariants require a transaction with suitable
constraints, locks, versions, or isolation. Schema checks, uniqueness, and
foreign keys reinforce stable rules but do not replace the domain conversion.

Invalid historical rows receive a distinct integrity error, not a guessed
default.

## 4. How is a trusted type constructed?

Repository code decodes a raw row and calls protected smart constructors. A
custom driver decoding trait may delegate to the same complete conversion if it
can return structured failure. Otherwise, keep the raw/domain split explicit.

```rust
impl TryFrom<InvoiceRow> for Invoice {
    type Error = InvoiceRowError;

    fn try_from(row: InvoiceRow) -> Result<Self, Self::Error> {
        // Parse newtypes, then match a checked state truth table.
    }
}
```

Successful conversion establishes current local domain invariants for the
observed row version.

## 5. How can construction be bypassed?

Bypasses include ORM derives that assign private fields, unchecked `From` impls,
raw SQL helpers returning domain types, `unwrap_or_default` on invalid columns,
administrative imports using privileged constructors, partial projections named
as complete aggregates, and tests that expose constructors through enabled
features. Unsafe representation casts are forbidden.

Audit every read query, cache population, event replay, migration, and restore
path. A schema constraint is not evidence that every historical row satisfied
it before constraint validation.

## 6. How is failure represented?

Distinguish driver/protocol failure, missing row, cardinality mismatch, physical
type failure, domain-integrity failure, unsupported version, optimistic
conflict, constraint conflict, transient availability, and ambiguous commit
where relevant. Preserve source chains and safe row identity. Invalid data may
fail the request, enter quarantine, or appear through an explicit degraded
administrative type.

Do not silently skip invalid rows in business totals or batches.

## 7. How are unknown or future values handled?

Persist stable enum tags and versions. Readers choose reject, retain raw,
explicit unknown, or migrate. Rolling deployments require a compatibility
matrix. Schema migrations state preconditions, invariant transformation, full
postcondition checks, and forward-repair/rollback semantics. New columns may be
nullable during expansion without making the domain field optional.

An older reader must not reinterpret a new state as a semantically similar old
one without an explicit compatibility rule.

## 8. How is sensitive data protected?

Use parameterized queries. Limit database error and row logging. Redact secrets,
tokens, personal data, and encrypted values; encryption does not make logging
ciphertext harmless. Repository errors carry identifiers needed for repair, not
entire rows. Administrative quarantine and migration tools require least
privilege and an audit trail.

Connection strings and migration credentials never enter fixtures or committed
configuration.

## 9. How is evidence tested?

Unit-test raw-row conversions for every invalid combination. Integration-test
against the actual database and driver for constraints, transactions, enum
encodings, nullability, versions, and error mapping. Run concurrent
read-modify-write tests for optimistic conflicts and relevant isolation
anomalies. Migrate old fixtures and verify the complete postcondition.

Test invalid historical data quarantine, backup restoration, commit connection
loss where the driver can simulate it, and outbox atomicity.

## 10. What remains uncertain?

A valid decoded entity reflects an observation at a time and version. It does
not prove no concurrent writer changed it, a replica is current, a transaction
will commit, or an external side effect occurred. Database durability and
isolation depend on product, configuration, topology, and failure mode.
Ambiguous commit and persistence-after-external-effect require reconciliation.

## Decision table

| Invariant                  | Boundary treatment                                         |
| -------------------------- | ---------------------------------------------------------- |
| scalar range/format        | row field to checked newtype                               |
| mutually exclusive columns | raw row truth table to enum                                |
| uniqueness                 | constructor plus database constraint                       |
| aggregate version          | optimistic update predicate                                |
| cross-row balance          | transaction plus appropriate isolation/locking             |
| historical invalid data    | quarantine or audited migration                            |
| state plus publication     | transactional outbox, not fictional cross-system atomicity |

## Review prompts

- Does every query return a raw row or a fully checked domain conversion?
- Can projections or joins create an incomplete value under a complete type name?
- Which database constraints arbitrate concurrency rather than merely duplicate checks?
- Are migration preconditions and postconditions evaluated over the full affected set?
- How do callers distinguish conflict, integrity failure, and ambiguous commit?
- Which product configuration underlies durability and isolation claims?

---

## Source: `boundaries/http-and-rpc.md`

# HTTP and RPC boundary guide

## 1. What is untrusted?

Method, path, query, headers, body, peer metadata, forwarded identity, cookies,
tokens, deadlines, and idempotency keys are untrusted until their responsible
layer validates them. TLS termination or authenticated transport does not make
the payload domain-valid. Proxy-added headers require a configured trusted-proxy
boundary. Client-supplied correlation IDs and resource names can be malformed,
oversized, or misleading.

## 2. What parsing occurs?

Enforce method, route, header count/size, body bytes, decompression ratio,
content type, nesting, field count, and deadline limits before or during decode.
Parse into request DTOs whose shape follows the protocol, not directly into
trusted aggregates. Parse typed durations and sizes with explicit units. Reject
ambiguous duplicate security headers according to the server framework's
documented behavior.

Protocol parsing establishes structural representation only.

## 3. What validation occurs?

Validate DTO field formats, ranges, cross-field combinations, version,
idempotency-key syntax, and resource identifiers. Authenticate the credential
to obtain a principal. Authorize that principal for the specific action and
resource, producing a scoped capability or explicit decision. Validate
cross-entity business rules transactionally where needed.

Authentication establishes identity evidence, not universal authorization. Under
[RUST-DOC-0003-R004](../doctrines/0003-ownership-and-capabilities/doctrine.md#rust-doc-0003-r004--restrict-capability-issuance-and-surface),
an authorization decision or capability grants only its documented action and resource scope.
Frontend state or client-provided role data is never authority.

## 4. How is a trusted type constructed?

Adapters convert request DTO fields through smart constructors and then call a
domain command/service. The domain layer receives trusted values and
authenticated/authorized evidence rather than framework extractors.

For mutation, create a stable operation ID and bind the idempotency key to
principal, endpoint, target, and request fingerprint before execution. Response
DTOs are constructed deliberately from output evidence; broad domain
serialization can expose internal or secret fields.

## 5. How can construction be bypassed?

Common bypasses include deriving request decoding directly on trusted newtypes,
accepting user-supplied principal structures, checking authorization only in UI
or middleware paths that some routes skip, calling domain methods from an
alternate endpoint without capability, using unchecked `From<String>`, and
letting internal administrative routes share public constructors without audit.

Generated RPC bindings still require a domain conversion layer. Internal
network placement is not a constructor.

## 6. How is failure represented?

Preserve parse, validation, authentication, authorization, not-found, conflict,
rate limit, timeout, cancellation, confirmed rejection, unknown outcome, and
internal failure when clients act differently. Map to HTTP/RPC codes
consistently without leaking secrets or existence information contrary to
policy. Include machine-actionable error codes and safe correlation identity.

A timeout after possible dispatch becomes an explicit unknown operation state,
not an automatic rejection.

## 7. How are unknown or future values handled?

Version public contracts deliberately. Additive fields may be ignored or
retained according to protocol; unknown enum values need reject or explicit
unknown behavior. Support rolling clients with compatibility tests. Deprecation
needs observable usage and a removal policy. Preserve unknown provider outcomes
separately from unknown schema values.

Idempotency response replay must define behavior when server response schemas
evolve during key retention.

## 8. How is sensitive data protected?

Never log authorization headers, cookies, API keys, private request bodies, or
secret response fields. Limit validation excerpts and tracing attributes.
Authenticate correlation and forwarded-client metadata only from trusted
infrastructure. Response errors do not expose stack traces or internal source
messages. Apply cache-control and browser security policy according to endpoint
sensitivity.

Credential parsing minimizes copies; zeroization claims must account for
framework buffers.

## 9. How is evidence tested?

Test method/content type, malformed body, size/decompression limits, missing and
duplicate fields, unknown versions, constructor rejection, authentication
failure, resource-scoped authorization, sensitive error mapping, and rate
limits. Contract tests protect wire schemas and codes. Integration tests use the
real router, middleware order, codec, and identity verifier.

For mutable effects, test concurrent same-key requests, payload conflict,
response replay, timeout after dispatch, safe retry, reconciliation, and key
expiry.

## 10. What remains uncertain?

An authenticated request proves only the verifier's evidence at that time.
Authorization can become stale. A successful server response proves the server
boundary's stated result, not necessarily downstream settlement or durable
client receipt. Client disconnect does not prove cancellation. Proxies and
retries can duplicate requests. External observations can change immediately
after return.

## Decision table

| Concern           | Boundary mechanism                       |
| ----------------- | ---------------------------------------- |
| request shape     | bounded DTO decode                       |
| value invariant   | smart constructor                        |
| identity          | credential verification                  |
| authority         | action/resource authorization capability |
| concurrent update | version/conflict protocol                |
| repeat mutation   | scoped idempotency contract              |
| ambiguous effect  | operation ID plus unknown/reconciliation |
| public failure    | stable code and redacted message         |

## Review prompts

- Are route, middleware, and generated-RPC alternate paths all protected?
- Do request limits apply before decompression and allocation?
- Is authentication evidence distinct from action/resource authorization?
- Does one logical retry reuse operation and idempotency identity?
- Can client cancellation occur after external dispatch?
- Are correlation and error diagnostics useful without exposing credentials or
  internal topology?
- Does version evolution preserve old-client behavior intentionally?
- What response evidence can later become stale or be reversed?

---

## Source: `boundaries/messaging.md`

# Messaging boundary guide

## 1. What is untrusted?

Message bytes, headers, event identity, producer identity, schema tag,
timestamp, partition key, ordering metadata, delivery count, and broker
attributes are untrusted until validated against the broker and application
contract. A message may be duplicated, delayed, reordered, replayed, truncated,
poisoned, produced by an old writer, or deliberately hostile. Broker
acknowledgement is not domain authorization.

At-least-once delivery makes repeated valid messages normal, not exceptional.

## 2. What parsing occurs?

Apply broker frame and consumer batch limits before payload decoding. Verify
compression and decompression bounds. Parse a versioned envelope containing
stable event/message ID, source, type, schema version, correlation/causation,
timestamp, partition or aggregate key, and payload. Then parse the payload into
a raw DTO.

Do not allocate according to an untrusted collection length or retain unlimited
unknown headers. Preserve raw evidence needed for quarantine within sensitivity
limits.

## 3. What validation occurs?

Validate envelope version, producer/source authorization, identifier syntax,
payload shape, domain values, aggregate version, and allowed command/event type.
For commands, authenticate the producer and authorize the requested operation.
For events, distinguish a statement from authority to mutate local state.

Validate ordering or predecessor rules at the aggregate boundary. Duplicate
identity is handled before non-idempotent effects. Cross-entity changes use the
required local transaction.

## 4. How is a trusted type constructed?

Decode envelope and raw payload, then call domain constructors. Construct a
trusted command only after source and authority evidence is attached. Claim a
stable message ID in an inbox when duplicate suppression protects a durable
local effect. Atomically record inbox identity, local mutation, and durable
outbox intent where one database can coordinate them.

External effects receive a separate stable operation ID and explicit unknown
outcome; inbox presence alone does not prove the external effect.

## 5. How can construction be bypassed?

Bypasses include deriving domain deserialization, trusting a topic name as
authorization, direct administrative replay into business handlers, using
payload business IDs as delivery identity without scope, in-memory-only dedup,
calling handlers outside the inbox transaction, and converting unknown schemas
to a default variant.

Dead-letter and replay tools are privileged writers. They must preserve original
identity and pass the same validation unless an audited repair establishes new
evidence.

## 6. How is failure represented?

Distinguish temporary broker/consumer failure, malformed envelope, unsupported
version, validation rejection, unauthorized source, duplicate already
completed, duplicate still in progress, ordering gap, stale version, domain
conflict, poison message, external unknown outcome, and internal defect.

Retry only categories with safe semantics. Poison messages enter a bounded
quarantine or dead-letter process with redacted diagnostics, owner, and replay
policy. Acknowledgement errors preserve ambiguity.

## 7. How are unknown or future values handled?

Use versioned envelopes and stable external tags. Define forward/backward
compatibility for rolling producers and consumers. An unknown event type may be
ignored only if omission is safe and observable; commands should usually reject
unsupported meaning. Retaining raw unknown payloads supports later replay but
requires size and sensitivity controls.

Schema registry compatibility is supporting evidence, not proof every semantic
change is compatible. Event meaning changes require new versions or types.

## 8. How is sensitive data protected?

Minimize payload data and avoid credentials in messages. Encrypt where threat
model requires, while preserving broker metadata sensitivity. Logs contain
message IDs, categories, and safe fingerprints rather than bodies. Dead-letter
queues need access control, retention, deletion, and encryption matching the
original data. Correlation IDs can become personal tracking data and require
policy.

Signatures and encryption do not validate domain policy or prevent authorized
producers from sending invalid values.

## 9. How is evidence tested?

Contract-test envelope and every supported schema version. Integration-test the
real codec and broker where feasible. Inject duplicates before, during, and
after local commit; lost acknowledgements; out-of-order and missing versions;
consumer restart; poison payload; dead-letter replay; and retention expiry.

Assert inbox atomicity, outbox publication retry, bounded backpressure, channel
closure, and graceful shutdown. External-effect tests preserve unknown status
after acknowledgement loss.

## 10. What remains uncertain?

Broker acceptance does not prove consumer processing. Consumer acknowledgement
does not prove an uncoordinated external side effect. Partition order is not
global order. Event time may differ from processing time and clocks may skew.
Deduplication stops working after its retained identity expires. Replayed
historical facts may no longer authorize current actions.

## Delivery decision table

| Scenario                               | Required behavior                               |
| -------------------------------------- | ----------------------------------------------- |
| repeated message ID, completed locally | return/reuse recorded outcome                   |
| repeated ID, different fingerprint     | reject as conflict/security event               |
| gap in required aggregate sequence     | wait, fetch, or reconcile; do not guess         |
| invalid payload                        | quarantine/dead-letter with bounded diagnostics |
| effect completed, ack lost             | deduplicate on redelivery                       |
| external effect response lost          | persist unknown and reconcile                   |
| newer unsupported schema               | reject or retain per compatibility policy       |

## Review prompts

- Is stable message identity separate from aggregate/business identity?
- Does inbox claim share an atomic boundary with the protected local effect?
- Are acknowledgement crash points and replay retention explicitly tested?
- Which order exists per producer, aggregate, or partition — and nowhere else?
- Can a privileged replay tool alter identity or bypass current validation?
- Are poison records bounded, access-controlled, and owned through resolution?

---

## Source: `boundaries/configuration.md`

# Configuration boundary guide

## 1. What is untrusted?

Environment variables, files, command-line arguments, mounted secrets, remote
configuration, service discovery, and default selection are untrusted
representations. Deployment control reduces hostile input likelihood but does
not prevent typo, unit mismatch, partial rollout, stale secret, invalid
combination, excessive value, or accidental disclosure.

Configuration from several sources also has an ordering and provenance problem:
operators must know which source won.

## 2. What parsing occurs?

Collect raw strings/bytes into a source-aware raw configuration. Parse explicit
booleans, integers, addresses, paths, enums, durations, byte sizes, lists, and
version. Units belong in names or syntax: `30s` is safer than an unexplained
`30`; bytes and item counts are distinct. Bound file size, line count, list
length, and remote response size before allocation.

Reject invalid encoding deliberately. Do not apply lossy parsing to identifiers
or secrets.

## 3. What validation occurs?

Validate per-field ranges and formats, then cross-field combinations: TLS
certificate with key, min not greater than max, retry budget within request
deadline, queue capacity compatible with workers, mutually exclusive modes,
required secret for enabled integration, and nonzero resource bounds.

Startup validation should complete before accepting work. Checks requiring
network reachability are readiness observations, not permanent configuration
validity.

## 4. How is a trusted type constructed?

Use a `RawConfig` that preserves optional sources, then a fallible conversion to
an immutable or carefully reloadable `ValidatedConfig` composed of typed
durations, sizes, endpoints, paths, and secret wrappers. Defaults are applied in
one documented layer before final cross-field validation. Report provenance for
the effective non-secret values.

Subcomponents receive narrow configuration values rather than the entire
configuration object, limiting accidental authority and secret access.

## 5. How can construction be bypassed?

Bypasses include reading environment variables ad hoc inside business code,
calling `unwrap_or` with hidden defaults, exposing mutable public configuration
fields, deserializing directly to trusted types, letting test defaults ship to
production, building URLs by string concatenation, and reloading individual
fields without validating the complete new snapshot.

Library code should accept typed configuration from its owner instead of
silently reading process-global state.

## 6. How is failure represented?

Distinguish missing required value, malformed syntax, unsupported version,
out-of-range, inconsistent combination, inaccessible file, permission failure,
secret-provider failure, and reload rejection. Aggregate independent startup
errors where safe so operators can fix one deployment, but redact secret
values. Exit nonzero before admission when required configuration is invalid.

Reload failure normally preserves the previous valid snapshot and exposes a
metric/event; it must not install a partial mix.

## 7. How are unknown or future values handled?

Version durable configuration files and remote schemas where long-lived.
Unknown fields may detect misspelling through strict rejection or support
rolling deployment through deliberate ignore; choose per configuration
contract. Unknown enum values should reject unless an explicit disabled or
opaque behavior is safe. Deprecations need warnings without secret values and a
defined removal version.

Defaults can change behavior across releases, so compatibility-sensitive
defaults require versioning or explicit operator selection.

## 8. How is sensitive data protected?

Secret-bearing types avoid `Debug`, `Display`, cloning, serialization, and
metrics labels. Logs show source and presence, never value. Environment
variables can leak through process inspection and crash reports; use an
appropriate secret provider for the threat model. Files need ownership and
permission checks. Remote secret retrieval needs authenticated transport,
rotation, cache, expiry, and failure policy.

Zeroization claims must account for parser strings, process environment, and
provider SDK copies.

## 9. How is evidence tested?

Test each source, precedence, explicit/default values, unit parsing, exact
bounds, unknown fields, invalid combinations, redaction, missing secret,
unreadable file, and remote failure. Snapshot the effective non-sensitive
configuration schema, not live secret values. Integration-test the real startup
path and confirm no work is accepted after invalid configuration.

Reload tests cover atomic snapshot replacement, concurrent readers, rejected
updates, removal, secret rotation, and shutdown.

## 10. What remains uncertain?

Successful parsing does not prove endpoints are reachable, credentials remain
valid, certificates remain unrevoked, files stay unchanged, or selected
capacities suit live load. Reloaded authorization configuration can become stale
immediately. OS resource limits and topology may differ from declared values.
Readiness checks are observations at a time.

## Reload contract

| Phase    | Required behavior                                         |
| -------- | --------------------------------------------------------- |
| acquire  | bounded read with source/version                          |
| parse    | raw snapshot, no mutation of active state                 |
| validate | all per-field and cross-field rules                       |
| prepare  | create dependent resources without exposing partial state |
| swap     | atomic publication of complete valid snapshot             |
| retire   | bounded cleanup of old resources                          |
| fail     | retain prior snapshot and report redacted diagnostics     |

## Review prompts

- Is every unit explicit and parsed once?
- Are defaults documented as policy rather than hidden in call sites?
- Does startup finish complete validation before admission?
- Does reload validate one complete snapshot and retain the previous one on
  failure?
- Can any subcomponent read environment or files behind the configuration owner?
- Are secrets absent from debug, error, metric, and effective-config output?
- Which reachability or capacity assumptions require ongoing observation?
- Are precedence and provenance visible to operators without revealing values?

---

## Source: `boundaries/filesystem.md`

# Filesystem boundary guide

## 1. What is untrusted?

Paths, directory entries, symlinks, metadata, file contents, mount behavior,
permissions, ownership, environment-derived base directories, archive members,
and concurrent filesystem changes are untrusted. A path string accepted by a
parser does not prove it names the intended object. Attackers or other processes
may replace components between checks and use.

Unicode and platform path representations are not universally valid UTF-8.

## 2. What parsing occurs?

Parse user-visible path syntax without forcing `Path`/`OsStr` through lossy
Unicode conversion. Join only after selecting a trusted base. Reject absolute
paths, parent traversal, device prefixes, or alternate separators according to
the target policy. Bound filename, path depth, directory entry count, file size,
archive expansion, and total extraction size.

File content parsing occurs only after resource limits and the appropriate
format/version checks.

## 3. What validation occurs?

Validate containment, allowed object type, ownership, permissions, link policy,
extension only where it is meaningful, and content invariants. Prefer
descriptor-relative operations or platform facilities that constrain traversal
when security depends on staying beneath a directory. Metadata checks and
operation must be coupled to avoid time-of-check/time-of-use gaps.

Do not rely on string prefix comparison for path containment.

## 4. How is a trusted type constructed?

A trusted path or opened-file capability is constructed by a filesystem service
after resolving through the approved policy and opening the actual object with
appropriate flags. Passing an already opened handle often preserves object
identity better than returning a checked path string. Content then decodes into
a raw representation and domain constructors.

For output, create a temporary file in the destination directory with explicit
permissions, write and flush as required, then atomically replace according to
platform semantics.

## 5. How can construction be bypassed?

Bypasses include concatenating strings, accepting absolute archive entries,
canonicalizing then reopening by path, following symlinks after a separate
check, using predictable temporary names, sharing a validated path while
another process can replace it, trusting extension as content type, and direct
filesystem calls outside the service.

Privileged repair tools and tests must follow or explicitly audit the same path
policy.

## 6. How is failure represented?

Distinguish invalid path policy, not found, wrong object type, symlink rejected,
permission denied, concurrent replacement, already exists, quota/no space,
partial read/write, content validation, lock contention, and durability
failure. Preserve OS source errors internally without exposing sensitive paths
or mount topology publicly.

After partial output failure, report whether the original, temporary, or
replacement file is present when that fact can be observed.

## 7. How are unknown or future values handled?

Version durable file formats. Reject, migrate, or preserve unknown versions.
Unknown directory entries should not be processed merely because they match a
broad glob. Archive formats and metadata require explicit supported subsets.
Platform-specific path forms, case sensitivity, and rename semantics belong to
the compatibility matrix.

Readers should tolerate safe additive fields only according to the file format,
not by ignoring every parse error.

## 8. How is sensitive data protected?

Create secrets with restrictive permissions from the first open, not by
tightening them after writing. Avoid exposing values in filenames, temporary
paths, logs, process arguments, or error messages. Ensure temporary and backup
files follow retention and deletion policy. Directory permissions, umask,
hardlinks, backups, snapshots, and crash dumps affect confidentiality.

Secure deletion is not generally guaranteed by ordinary file removal,
copy-on-write filesystems, or storage hardware.

## 9. How is evidence tested?

Test traversal, absolute paths, symlink chains, race-capable replacement where
the platform permits, wrong object types, permissions, Unicode/non-UTF-8 names,
partial writes, disk-full injection, temp cleanup, concurrent writers, lock
behavior, format versions, and oversized/archive-bomb inputs. Run on every
supported OS/filesystem family for semantic claims.

Crash tests verify atomic replacement and the configured durability sequence,
while documenting what the platform does not guarantee.

## 10. What remains uncertain?

An opened file can later be modified by another actor unless locking or access
control prevents it. Successful `write` may cover only a userspace/kernel
buffer; durable persistence depends on flush operations, directory metadata,
filesystem, mount, device, and power-failure behavior. File locks vary by
platform and cooperation. Network filesystems add distributed caching and
failure ambiguity.

## Safe replacement sequence

```text
trusted destination directory handle
    ↓ create unique temporary file with final permissions
write bounded complete content
    ↓ flush file according to durability requirement
atomic rename/replace within same filesystem
    ↓ flush directory metadata where required
report exact achieved guarantee and cleanup residuals
```

Atomic visibility and power-loss durability are different claims.

## Review prompts

- Does path containment rely on opened directory capabilities rather than
  string prefixes?
- Can a symlink, hardlink, mount, or concurrent rename change object identity
  between validation and use?
- Are temporary files unique, same-filesystem, permissioned at creation, and
  cleaned on every error?
- Are write completion, atomic replacement, file flush, and directory flush
  described as distinct guarantees?
- Do locks rely on cooperation, and what happens after process failure?
- Are non-UTF-8 and platform-specific paths preserved without lossy identity?
- Can archive expansion exceed per-file or total limits?
- Which files, backups, snapshots, and logs may retain sensitive content?

---

## Source: `boundaries/ffi.md`

# FFI boundary guide

## 1. What is untrusted?

Foreign pointers, lengths, return codes, discriminants, callbacks, user-data
contexts, thread origin, string bytes, handles, and allocation provenance are
untrusted until established by the foreign contract and checked locally.
Generated bindings reduce transcription error but do not prove the library
implementation obeys its header or that Rust usage satisfies every lifetime and
thread rule.

FFI is both a trust boundary and an unsafe proof boundary.

## 2. What parsing occurs?

Raw declarations parse ABI-level values: integer widths, pointers, structs,
unions, and opaque handles. Convert integer lengths with checked arithmetic and
correct byte/element units. Validate nullability before access. Keep arbitrary
string bytes in a representation compatible with the specified encoding until
validated. Return-code and out-parameter combinations form a raw response type.

Never construct a typed reference or enum before Rust validity is established.

## 3. What validation occurs?

Validate ABI and target, representation layout, pointer alignment and bounds,
initialization, discriminants, string encoding, handle state, ownership,
callback lifetime, thread affinity, and error combinations. Business values
then pass normal domain constructors. Cross-call lifecycle rules belong to a
safe wrapper or explicit unsafe caller contract.

If a foreign library can mutate a buffer asynchronously, validation must occur
after exclusive completion evidence or the buffer cannot be exposed as an
immutable Rust slice.

## 4. How is a trusted type constructed?

Keep raw bindings in a private low-level module. A wrapper validates outputs,
copies or takes ownership according to the contract, and constructs safe domain
types. Opaque handles live in private fields with a matching destructor.
Borrowed outputs receive the shortest lifetime tied to the owning handle.

A safe public API is allowed only when all safety obligations can be enforced
for every safe caller. Otherwise expose a narrowly scoped `unsafe fn` with a
complete `# Safety` section.

## 5. How can construction be bypassed?

Bypasses include public raw bindings, transmuting foreign integers to enums,
assuming `repr(Rust)` layout, building slices from unchecked length, forging
lifetimes, implementing `Send`/`Sync` without upstream guarantees, freeing with
the wrong allocator, retaining stack callback context, and allowing panics to
cross an incompatible ABI.

Feature flags and alternate targets can compile different bindings, so audit the
complete supported matrix.

## 6. How is failure represented?

Map foreign status codes to structured Rust errors while preserving unknown
codes and source context. Distinguish invalid foreign output, library rejection,
resource exhaustion, unsupported feature, callback panic containment, and
indeterminate external effect where applicable. Do not call `last_error` unless
the API contract says it is meaningful for the specific failure.

Destructors cannot report ordinary errors through `Drop`; provide explicit
close when cleanup failure matters and make drop behavior conservative.

## 7. How are unknown or future values handled?

Represent unknown integer codes as raw/unknown variants rather than transmute.
Use size/version fields in extensible structs according to the C API contract.
Bindings and wrappers track library and ABI versions. New optional fields remain
zeroed or initialized exactly as required. Dynamic symbol absence becomes a
structured unsupported-feature result.

Do not assume an undocumented reserved value remains unused.

## 8. How is sensitive data protected?

Minimize copies of secrets into foreign memory, but state zeroization limits
across allocators and library internals. Avoid debug output of buffers and
handles. Define ownership and cleanup for credentials on every error path.
Foreign crash dumps and logging can expose values outside Rust controls.

Callbacks must not capture broader authority or secrets than required. Pointer
addresses can be sensitive and should not appear in routine public diagnostics.

## 9. How is evidence tested?

Compare bindings to authoritative headers; use generated/layout assertions and
C-side integration tests. Test null, zero length, maximum length, unknown codes,
invalid encoding, partial output, allocation failure where injectable,
callback reentrancy, callback on unexpected threads, panic containment, and
double-close prevention. Run Miri on pure-Rust wrapper paths where supported and
sanitizers/Valgrind-like tools on real foreign integration.

Test every supported target ABI. Record tool blind spots and foreign-library
versions.

## 10. What remains uncertain?

A sound wrapper cannot prove the foreign implementation lacks memory errors,
races, or undocumented behavior. A successful call may not prove an external
device or service remains available. Resource release can fail internally even
when the ABI returns no status. Library upgrades, compile flags, allocator
changes, and target ABIs can invalidate premises and trigger re-audit.

## FFI contract table

| Dimension   | Required record                                        |
| ----------- | ------------------------------------------------------ |
| ABI/layout  | calling convention, `repr`, widths, alignment, targets |
| pointer     | nullability, bounds, mutability, provenance            |
| ownership   | borrow/take/give, lifetime, matching release           |
| strings     | encoding, length, terminator, interior null            |
| errors      | status/out-param rules, unknown code                   |
| callbacks   | retention, thread, reentrancy, unregister              |
| unwind      | catch, abort, or supported unwind ABI                  |
| concurrency | affinity, synchronization, `Send`/`Sync` proof         |
| allocation  | allocator of origin and cleanup                        |

---

## Source: `reviews/README.md`

# Operational reviews

Review procedures turn doctrine into repeatable decision gates. They are not
surveys and do not replace reading the applicable doctrine packages. Each item
must be recorded as:

- **pass** — cited evidence satisfies the question;
- **fail** — evidence is missing or contradicts the requirement;
- **not applicable** — scope explanation shows the risk is absent;
- **waiver reference** — approved waiver names scope, owner, consequence,
  compensating control, expiry, and removal condition.

Blank status is not approval. A critical doctrine violation cannot be converted
into soundness or external certainty through a waiver; a waiver records accepted
residual risk only where governance permits it.

## Procedures

| Procedure                                                     | Use                                                          |
| ------------------------------------------------------------- | ------------------------------------------------------------ |
| [Pre-implementation](../reviews/pre-implementation.md)                   | before representation and API commitments                    |
| [Domain model review](../reviews/domain-model-review.md)                 | values, states, construction, transition, authority          |
| [Boundary review](../reviews/boundary-review.md)                         | DTO, Serde, database, protocol, size, version, secrecy       |
| [Typestate review](../reviews/typestate-review.md)                       | proportional use of type-level sequencing                    |
| [Distributed-effects review](../reviews/distributed-effects-review.md)   | timeout, retry, duplicate, reconciliation, ordering          |
| [Executable narrative review](../reviews/executable-narrative-review.md) | where an obligation lives, and whether a record is justified |
| [Final correctness audit](../reviews/final-correctness-audit.md)         | release/merge guarantee ledger and aggregate gates           |

## Evidence rule

Reviewers cite concrete artifacts: invariant IDs, state graphs, source paths,
constructor visibility, queries, protocol documents, tests, fault matrices,
measurements, and generated-bundle checks. "Idiomatic Rust," compilation, or a
green suite alone is not enough. Evidence must match the claim and identify its
limits.

## Procedure format policy

Compact tables are the default for dense operational checklists because they
make gate, evidence, failure, severity, and remediation fields easy to scan.
Expanded gate sections remain valid when a procedure needs a fuller argument
per question. The two forms carry the same disposition and traceability
requirements.

The current corpus deliberately uses the expanded form only for the
foundational RUST-DOC-0001 package review; RUST-DOC-0002 through
RUST-DOC-0009 use tables. Do not normalize that exception mechanically if doing
so would discard its evidence and remediation detail. New divergence must
explain what additional review value the expanded form supplies.

## Severity and disposition

Use **critical** when failure can forge trusted evidence, bypass authority,
cause undefined behavior, repeat a consequential effect, lose durable state, or
make an externally false guarantee. Use **high** for likely correctness,
recovery, compatibility, or operational failures. Use **medium** for material
maintainability, diagnostic, or evidence weakness. Every fail has remediation
or a governance decision.

The final audit references completed focused reviews rather than copying their
answers. Generated agent packs may embed selected gates, but these canonical
files remain authoritative.

---

## Source: `reviews/pre-implementation.md`

# Pre-implementation review

## Record

Record feature/change identifier, planner, reviewer, date, affected doctrine
IDs, and status for every gate. Status is **pass**, **fail**, **not applicable**,
or **waiver reference**. Complete this review before public type, persistence,
or protocol choices become expensive to reverse.

## Domain and invariant inventory

| ID     | Question                                                                                                                                         | Pass evidence                       |
| ------ | ------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------- |
| PRE-01 | Is the domain objective stated without prescribing a Rust mechanism?                                                                             | outcome and scope                   |
| PRE-02 | Is a shared vocabulary defined for values, actors, states, and effects?                                                                          | vocabulary artifact                 |
| PRE-03 | Are ambiguous terms split by evidence level?                                                                                                     | definitions such as parsed/verified |
| PRE-04 | Are non-goals and excluded systems explicit?                                                                                                     | bounded scope                       |
| PRE-05 | Does every consequential invariant have a stable ID?                                                                                             | invariant inventory                 |
| PRE-06 | Is each invariant statement testable or reviewable?                                                                                              | precise predicate                   |
| PRE-07 | Is each invariant classified as value, state, transition, authority, lifecycle, boundary, cross-entity, temporal, environmental, or distributed? | classification field                |
| PRE-08 | Is the invariant owner named?                                                                                                                    | component or role                   |
| PRE-09 | Is the enforcement mechanism proposed without claiming more than it proves?                                                                      | mechanism column                    |
| PRE-10 | Is the trust boundary that establishes evidence named?                                                                                           | boundary column                     |
| PRE-11 | Is failure consequence recorded?                                                                                                                 | consequence/severity                |
| PRE-12 | Is residual uncertainty recorded?                                                                                                                | uncertainty column                  |
| PRE-13 | Are preconditions distinguished from invariants?                                                                                                 | separate entries                    |
| PRE-14 | Are assumptions and observations distinguished from guarantees?                                                                                  | assumption ledger                   |
| PRE-15 | Are cross-entity rules excluded from pure scalar constructors?                                                                                   | enforcement placement               |
| PRE-16 | Are external mutable facts identified as runtime evidence?                                                                                       | observation policy                  |

## State and authority

| ID     | Question                                                                        | Pass evidence            |
| ------ | ------------------------------------------------------------------------------- | ------------------------ |
| PRE-17 | Is a state graph provided for each meaningful lifecycle?                        | nodes and legal edges    |
| PRE-18 | Does each state list required associated evidence?                              | state payload table      |
| PRE-19 | Are mutually exclusive and independent dimensions distinguished?                | representation rationale |
| PRE-20 | Does every transition identify actor and authority?                             | transition table         |
| PRE-21 | Does every transition identify precondition and postcondition?                  | edge contract            |
| PRE-22 | Are failure and cancellation edges present?                                     | complete graph           |
| PRE-23 | Are unknown or reconciliation states included where execution can be ambiguous? | explicit nodes           |
| PRE-24 | Is an authority map provided for privileged actions?                            | principal/capability map |
| PRE-25 | Are capability construction, transfer, clone, expiry, and revocation defined?   | authority lifecycle      |
| PRE-26 | Are secret-bearing values and permitted readers identified?                     | data/authority map       |

## Trust boundaries and external effects

| ID     | Question                                                                  | Pass evidence        |
| ------ | ------------------------------------------------------------------------- | -------------------- |
| PRE-27 | Is every ingress and egress boundary inventoried?                         | boundary map         |
| PRE-28 | Does each ingress show raw, structural, and trusted representations?      | conversion pipeline  |
| PRE-29 | Are alternate writers and privileged bypass paths listed?                 | bypass inventory     |
| PRE-30 | Are parsing, validation, authentication, and authorization separated?     | layered design       |
| PRE-31 | Are size, nesting, allocation, and concurrency limits proposed?           | resource table       |
| PRE-32 | Is version/unknown-value policy stated?                                   | compatibility matrix |
| PRE-33 | Is every external side effect inventoried?                                | effect list          |
| PRE-34 | Does each effect identify the point after which execution can be unknown? | protocol timeline    |
| PRE-35 | Are idempotency and retry classifications stated per failure point?       | failure matrix       |
| PRE-36 | Is reconciliation evidence and owner identified?                          | reconciliation plan  |
| PRE-37 | Are compensation actions treated as new fallible effects?                 | saga contract        |
| PRE-38 | Are ordering claims scoped by key, producer, partition, and failover?     | ordering contract    |

## Persistence, complexity, and evidence

| ID     | Question                                                                                                                                                   | Pass evidence         |
| ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------- |
| PRE-39 | Is the persistence representation distinct where its contract differs?                                                                                     | row/domain comparison |
| PRE-40 | Are transaction boundaries aligned with cross-entity invariants?                                                                                           | transaction map       |
| PRE-41 | Is optimistic concurrency or another lost-update strategy selected?                                                                                        | conflict protocol     |
| PRE-42 | Are migration and old-value compatibility needs identified?                                                                                                | version plan          |
| PRE-43 | Is persistence plus messaging coordinated durably where loss matters?                                                                                      | outbox/inbox decision |
| PRE-44 | Is the concurrency ownership and synchronization model stated?                                                                                             | task/state ownership  |
| PRE-45 | Are queue, pool, and retry capacities bounded?                                                                                                             | capacity budget       |
| PRE-46 | Is cancellation cleanup defined for partial operations?                                                                                                    | cancellation table    |
| PRE-47 | Is the simplest sufficient representation selected from enum, newtype, runtime validation, typestate, capability, or plain code?                           | decision record       |
| PRE-48 | Does the complexity budget cover diagnostics, compile time, code size, migration, and team operation?                                                      | budget assessment     |
| PRE-49 | Is unsafe code absent or separately justified under RUST-DOC-0007?                                                                                         | unsafe decision       |
| PRE-50 | Does each invariant map to planned compiler, unit, property, compile-fail, integration, fault, model, or operational evidence?                             | evidence matrix       |
| PRE-51 | Are negative and prohibited paths included?                                                                                                                | rejection plan        |
| PRE-52 | Are real boundaries exercised where consequential?                                                                                                         | integration plan      |
| PRE-53 | Are evidence limitations stated?                                                                                                                           | non-proof column      |
| PRE-54 | Does the initial guarantee ledger state claim, establishment, protected construction, boundary preservation, escape hatches, non-proofs, and runtime risk? | ledger                |

## Exit criteria

Implementation may start when every critical gate passes or has an approved
governance disposition, and the invariant inventory, boundary map, state graph,
effect inventory, authority map, persistence model, complexity budget, evidence
plan, and initial guarantee ledger are reviewable. New discoveries update these
artifacts rather than being buried only in code.

---

## Source: `reviews/domain-model-review.md`

# Domain model review

## Record

Apply to every trusted value, aggregate, state machine, and authority-bearing
handle. Record **pass**, **fail**, **not applicable**, or **waiver reference**,
with source paths and doctrine rule IDs.

## Values and names

| ID     | Question                                                                                                              | Pass evidence               |
| ------ | --------------------------------------------------------------------------------------------------------------------- | --------------------------- |
| DMR-01 | Does each trusted type name only evidence its constructors establish?                                                 | constructor/name comparison |
| DMR-02 | Are raw, parsed, policy-accepted, verified, authorized, and reconciled values distinct where operationally different? | evidence ladder             |
| DMR-03 | Are primitive aliases replaced where unit or invariant mixing is consequential?                                       | opaque type                 |
| DMR-04 | Are zero and empty states evaluated explicitly?                                                                       | boundary table              |
| DMR-05 | Does integer money include explicit currency?                                                                         | money representation        |
| DMR-06 | Does arithmetic reject currency mismatch and overflow?                                                                | checked operations          |
| DMR-07 | Are tax, FX, scale, allocation, and rounding outside the scalar guarantee?                                            | non-guarantees              |
| DMR-08 | Does email syntax avoid ownership/deliverability claims?                                                              | evidence-accurate types     |
| DMR-09 | Are identifiers nonempty/bounded/normalized according to one policy?                                                  | constructor                 |
| DMR-10 | Are secrets deliberately non-formatting and minimally cloneable?                                                      | trait/API audit             |

## Construction and mutation

| ID     | Question                                                                 | Pass evidence                 |
| ------ | ------------------------------------------------------------------------ | ----------------------------- |
| DMR-11 | Are trusted representation fields private?                               | visibility inspection         |
| DMR-12 | Does every public constructor enforce the complete documented invariant? | constructor trace             |
| DMR-13 | Is fallible construction visibly fallible?                               | `Result`/`TryFrom` API        |
| DMR-14 | Is normalization centralized and ordered before dependent checks?        | construction pipeline         |
| DMR-15 | Are errors structured and actionable?                                    | error enum/categories         |
| DMR-16 | Do mutation methods preserve the complete invariant?                     | mutation proof/tests          |
| DMR-17 | Are mutable representation escapes absent?                               | no `DerefMut`/raw field       |
| DMR-18 | Do conversion impls preserve evidence direction?                         | `From` versus `TryFrom` audit |
| DMR-19 | Are unchecked constructors private, narrow, and obligation-documented?   | escape-hatch inventory        |
| DMR-20 | Are unsafe constructors reviewed under doctrine 0007?                    | safety proof                  |

## States and transitions

| ID     | Question                                                                   | Pass evidence           |
| ------ | -------------------------------------------------------------------------- | ----------------------- |
| DMR-21 | Are mutually exclusive states represented by a sum type?                   | state shape             |
| DMR-22 | Are contradictory boolean/optional combinations absent?                    | truth-table review      |
| DMR-23 | Does associated data live only in meaningful variants?                     | enum payloads           |
| DMR-24 | Are independent dimensions kept separate?                                  | state decomposition     |
| DMR-25 | Is variant evolution and unknown persistence planned?                      | encoding/version policy |
| DMR-26 | Are legal transition edges explicit?                                       | state graph             |
| DMR-27 | Are illegal transitions structurally blocked or explicitly rejected?       | API/runtime checks      |
| DMR-28 | Do consuming transitions prevent invalid prior-state reuse where useful?   | ownership API           |
| DMR-29 | Do fallible transitions preserve or consume prior authority honestly?      | error shape             |
| DMR-30 | Do async transitions handle cancellation and partial effects?              | cancellation matrix     |
| DMR-31 | Does local typestate avoid remote-liveness claims?                         | guarantee ledger        |
| DMR-32 | Are persisted/dynamic states represented at runtime?                       | hybrid/runtime model    |
| DMR-33 | Are unknown distributed outcomes explicit?                                 | outcome enum            |
| DMR-34 | Does reconciliation require new evidence rather than arbitrary assignment? | transition service      |

## Authority, aggregates, and external rules

| ID     | Question                                                          | Pass evidence         |
| ------ | ----------------------------------------------------------------- | --------------------- |
| DMR-35 | Are privileged constructors restricted to the authority owner?    | module visibility     |
| DMR-36 | Does each capability expose least-privilege operations?           | API surface           |
| DMR-37 | Is capability cloning justified?                                  | clone/transfer policy |
| DMR-38 | Are expiry and revocation runtime semantics explicit?             | authority lifecycle   |
| DMR-39 | Are single-use tokens consumed or transactionally claimed?        | use protocol          |
| DMR-40 | Are collection invariants protected after mutation?               | wrapper API           |
| DMR-41 | Are completeness claims absent from paginated subsets?            | type naming           |
| DMR-42 | Are cross-entity rules enforced in a domain service/transaction?  | service boundary      |
| DMR-43 | Are environmental assumptions represented as checks/observations? | runtime validation    |
| DMR-44 | Do external effects remain fallible?                              | `Result`/outcome API  |
| DMR-45 | Does timeout preserve unknown execution where needed?             | outcome handling      |
| DMR-46 | Are public escape hatches enumerated in the guarantee ledger?     | ledger                |
| DMR-47 | Does each type list what it proves?                               | documentation         |
| DMR-48 | Does each type list what it does not prove?                       | documentation         |
| DMR-49 | Does executable evidence cover acceptance and rejection?          | tests                 |
| DMR-50 | Is type-system complexity proportional to misuse impact?          | complexity decision   |

## Exit criteria

Approval requires protected construction, evidence-accurate names, complete
state truth tables, legal transition handling, honest external fallibility, and
an updated guarantee ledger. Idiomatic syntax is not sufficient.

---

## Source: `reviews/boundary-review.md`

# Boundary review

## Record

Apply separately to each HTTP/RPC, message, database, Serde, configuration,
filesystem, and FFI ingress/egress. Record **pass**, **fail**, **not
applicable**, or **waiver reference**.

## Inventory and layering

| ID    | Question                                                              | Pass evidence        |
| ----- | --------------------------------------------------------------------- | -------------------- |
| BR-01 | Is the boundary owner and threat/error model named?                   | boundary record      |
| BR-02 | Are all raw bytes, metadata, and alternate sources inventoried?       | input list           |
| BR-03 | Is transport/physical parsing separate from domain validation?        | layered conversion   |
| BR-04 | Is a raw DTO/row/foreign type used where contracts differ?            | representation map   |
| BR-05 | Are trusted types constructed only after complete validation?         | call trace           |
| BR-06 | Are authentication and authorization separate transitions?            | evidence path        |
| BR-07 | Are cross-entity checks placed transactionally or in domain services? | enforcement location |
| BR-08 | Are egress DTOs deliberate rather than broad domain serialization?    | output types         |
| BR-09 | Are privileged administrative paths included in the inventory?        | bypass list          |
| BR-10 | Are cache, replay, restore, and migration paths included?             | complete map         |

## Parsing and resource limits

| ID    | Question                                                           | Pass evidence        |
| ----- | ------------------------------------------------------------------ | -------------------- |
| BR-11 | Is maximum raw input size enforced before large allocation?        | limit/config test    |
| BR-12 | Are decompression ratios and expanded size bounded?                | decompression policy |
| BR-13 | Are nesting, field, element, and batch counts bounded?             | parser limits        |
| BR-14 | Are numeric conversions checked for range and units?               | conversion code      |
| BR-15 | Are text encoding and Unicode policies explicit?                   | parser policy        |
| BR-16 | Are duplicate fields/headers/keys handled deliberately?            | fixtures             |
| BR-17 | Are paths protected from traversal and lossy conversion?           | path policy          |
| BR-18 | Are pointer null, length, alignment, and ownership checked at FFI? | FFI contract         |
| BR-19 | Are time and size values represented with typed units?             | DTO/domain types     |
| BR-20 | Does malformed input return structured failure without panic?      | negative tests       |

## Construction and bypasses

| ID    | Question                                                                | Pass evidence       |
| ----- | ----------------------------------------------------------------------- | ------------------- |
| BR-21 | Does Serde delegate to checked `TryFrom` or manual validation?          | implementation      |
| BR-22 | Does every database read validate trusted newtypes?                     | row conversions     |
| BR-23 | Are derived decoders prevented from assigning trusted fields unchecked? | derive audit        |
| BR-24 | Are unchecked `From` conversions absent for fallible evidence?          | impl search         |
| BR-25 | Are defaults prevented from inventing historical or verified facts?     | default audit       |
| BR-26 | Are partial projections named as partial types?                         | query/type review   |
| BR-27 | Can test-only or feature-gated constructors ship?                       | feature matrix      |
| BR-28 | Are unsafe layout/construction shortcuts absent or fully audited?       | unsafe inventory    |
| BR-29 | Are UI/client claims excluded from backend authority?                   | authorization trace |
| BR-30 | Does every bypass have a scoped, reviewed obligation?                   | escape-hatch ledger |

## Evolution, errors, and secrecy

| ID    | Question                                                                                                   | Pass evidence                  |
| ----- | ---------------------------------------------------------------------------------------------------------- | ------------------------------ |
| BR-31 | Is long-lived representation versioned?                                                                    | envelope/schema version        |
| BR-32 | Is unknown-field policy deliberate?                                                                        | reject/ignore/retain rationale |
| BR-33 | Is unknown enum/version behavior explicit?                                                                 | compatibility tests            |
| BR-34 | Are stable external tags independent of source rename?                                                     | encoding table                 |
| BR-35 | Is rolling old/new compatibility tested?                                                                   | version matrix                 |
| BR-36 | Are syntax, validation, authority, conflict, availability, and unknown outcomes distinguishable as needed? | error model                    |
| BR-37 | Are source errors retained internally?                                                                     | error chain                    |
| BR-38 | Are public diagnostics redacted and stable?                                                                | error mapping tests            |
| BR-39 | Are secrets absent from logs, debug, metrics, and snapshots?                                               | redaction audit                |
| BR-40 | Are quarantine/dead-letter records access-controlled and retained safely?                                  | operations policy              |
| BR-41 | Are correlation IDs bounded and sensitivity-classified?                                                    | telemetry schema               |
| BR-42 | Are credentials minimized across parsing copies?                                                           | secret data flow               |

## Evidence and non-guarantees

| ID    | Question                                                             | Pass evidence          |
| ----- | -------------------------------------------------------------------- | ---------------------- |
| BR-43 | Do tests cover valid and invalid boundary values?                    | fixtures               |
| BR-44 | Do tests cover oversized and resource-hostile input?                 | adversarial cases      |
| BR-45 | Do tests cross the real codec/driver/router/ABI where consequential? | integration suite      |
| BR-46 | Do tests cover old and future/unknown values?                        | compatibility fixtures |
| BR-47 | Do fault tests cover partial effects and acknowledgement loss?       | fault matrix           |
| BR-48 | Are invalid historical records rejected or quarantined?              | database evidence      |
| BR-49 | Is fuzz/property evidence used where input space warrants it?        | test record            |
| BR-50 | Does the boundary ledger state what parsing proves?                  | guarantee entry        |
| BR-51 | Does it state mutable external facts not proved?                     | non-guarantees         |
| BR-52 | Are observation time, freshness, and reconciliation recorded?        | evidence lifecycle     |

## Exit criteria

Approval requires complete construction-path coverage, bounded resource use,
intentional evolution, redacted failures, real-boundary evidence proportional
to risk, and explicit non-guarantees.

---

## Source: `reviews/typestate-review.md`

# Typestate review

## Record

Use whenever a state parameter, marker type, consuming state-specific handle, or
type-level transition is proposed. Record **pass**, **fail**, **not applicable**,
or **waiver reference**. The review must compare a runtime enum and plain
runtime validation rather than presuming typestate wins.

## Fit and scope

| ID     | Question                                                                    | Pass evidence            |
| ------ | --------------------------------------------------------------------------- | ------------------------ |
| TSR-01 | Is the protected problem operation sequencing rather than value validation? | invariant classification |
| TSR-02 | Is the sequence locally controlled by one handle owner?                     | ownership map            |
| TSR-03 | Is state not primarily externally determined?                               | boundary analysis        |
| TSR-04 | Is the state graph small and stable enough for static APIs?                 | node/edge count          |
| TSR-05 | Are illegal calls consequential and frequent enough to justify types?       | risk assessment          |
| TSR-06 | Can marker construction be restricted?                                      | visibility               |
| TSR-07 | Does each marker represent evidence actually established?                   | guarantee mapping        |
| TSR-08 | Are independent dimensions kept out of a type cross-product?                | decomposition            |
| TSR-09 | Is the workflow unsuitable for a simpler consuming non-generic handle?      | alternative comparison   |
| TSR-10 | Is a runtime enum explicitly evaluated?                                     | decision record          |

## Transition design

| ID     | Question                                                                    | Pass evidence              |
| ------ | --------------------------------------------------------------------------- | -------------------------- |
| TSR-11 | Does each legal edge have one clear method?                                 | state API graph            |
| TSR-12 | Are illegal state-specific methods absent from the type?                    | impl inspection            |
| TSR-13 | Do transitions consume the prior state when reuse is invalid?               | signatures                 |
| TSR-14 | Are infallible transitions truly local and infallible?                      | operation analysis         |
| TSR-15 | Do fallible transitions return structured errors?                           | error types                |
| TSR-16 | Is the prior handle returned only when non-transition is proven?            | error/recovery shape       |
| TSR-17 | Is external ambiguity represented instead of restoring old state?           | unknown outcome            |
| TSR-18 | Are transition payloads carried in the successor type?                      | successor fields           |
| TSR-19 | Are authorization and capability requirements explicit?                     | method arguments           |
| TSR-20 | Can parallel or duplicate transition attempts occur through another handle? | resource identity analysis |

## Async and external reality

| ID     | Question                                                          | Pass evidence      |
| ------ | ----------------------------------------------------------------- | ------------------ |
| TSR-21 | Is every `.await` in a transition cancellation-reviewed?          | cancellation table |
| TSR-22 | Does cancellation release or reconcile consumed resources?        | cleanup evidence   |
| TSR-23 | Are blocking operations isolated appropriately?                   | async design       |
| TSR-24 | Does an open/connected marker mean only local transition success? | documentation      |
| TSR-25 | Do send/capture/commit operations remain fallible?                | method results     |
| TSR-26 | Can timeout produce explicit unknown outcome?                     | outcome type       |
| TSR-27 | Are external facts re-observed when needed?                       | validation policy  |
| TSR-28 | Does typestate avoid claiming current remote liveness?            | guarantee ledger   |
| TSR-29 | Does a lease/capability marker account for expiry/revocation?     | runtime check      |
| TSR-30 | Is compensation modeled as a later fallible transition?           | state graph        |

## Persistence and ergonomics

| ID     | Question                                                                | Pass evidence           |
| ------ | ----------------------------------------------------------------------- | ----------------------- |
| TSR-31 | Must states be stored heterogeneously or inspected dynamically?         | usage inventory         |
| TSR-32 | Is a stable runtime persisted enum defined where needed?                | storage model           |
| TSR-33 | Does rehydration validate before issuing a typed handle?                | restoration service     |
| TSR-34 | Is marker spelling excluded from durable protocol evidence?             | encoding policy         |
| TSR-35 | Are optimistic version or claim semantics present for multiple workers? | persistence concurrency |
| TSR-36 | Can trait objects or plugin interfaces use the design clearly?          | dispatch design         |
| TSR-37 | Are mocks and test harnesses comprehensible?                            | test API review         |
| TSR-38 | Are compiler diagnostics useful at misuse sites?                        | compile-fail output     |
| TSR-39 | Is generic propagation limited at public boundaries?                    | API signatures          |
| TSR-40 | Are transition errors smaller/clearer than equivalent runtime checks?   | caller comparison       |
| TSR-41 | Has monomorphization/code-size impact been considered?                  | size analysis           |
| TSR-42 | Has compile-time and IDE diagnostic impact been considered?             | complexity budget       |

## Evidence and decision

| ID     | Question                                                      | Pass evidence         |
| ------ | ------------------------------------------------------------- | --------------------- |
| TSR-43 | Do compile-fail tests prove important prohibited calls?       | UI tests              |
| TSR-44 | Do they fail for the intended reason?                         | diagnostic inspection |
| TSR-45 | Are every successful and failed transition tested?            | unit suite            |
| TSR-46 | Are cancellation and external failure tested?                 | fault suite           |
| TSR-47 | Is persisted/runtime conversion tested?                       | integration suite     |
| TSR-48 | Are unknown outcomes and reconciliation tested?               | distributed tests     |
| TSR-49 | Does documentation state exact guarantees and non-guarantees? | ledger/docs           |
| TSR-50 | Does benefit exceed type/API complexity?                      | signed decision       |

## Staged protocols and successor capabilities

Apply this group when a capability trait exposes its legal successor as an associated type.
RUST-DOC-0010 governs these questions and its review standard carries the complete gate set.

| ID     | Question                                                              | Pass evidence          |
| ------ | --------------------------------------------------------------------- | ---------------------- |
| TSR-51 | Does each nonterminal capability name a bounded associated successor? | trait definitions      |
| TSR-52 | Does any bound name capability the successor does not establish?      | evidence mapping       |
| TSR-53 | Was a bound widened or removed to make an implementation compile?     | change record          |
| TSR-54 | Would a concrete successor return be simpler and equally safe?        | alternative comparison |
| TSR-55 | Is each material branch a named sum over distinct successors?         | branch enum            |
| TSR-56 | Is an undetermined outcome distinct from both branches?               | failure type           |
| TSR-57 | Does a revision or retry edge re-enter at the correct stage?          | successor bound        |
| TSR-58 | Can a conversion, derive, or constructor produce a later stage?       | implementation audit   |
| TSR-59 | Is the documented stage graph asserted executably?                    | topology assertion     |
| TSR-60 | Is a local transition being presented as durable evidence?            | guarantee ledger       |

## Exit criteria

Approve typestate only when local sequencing is the real risk, construction is
protected, external effects remain fallible, persistence has a runtime model,
diagnostics are acceptable, and the complexity comparison favors it. Otherwise
select a runtime enum, consuming transition, capability, or ordinary validation.

---

## Source: `reviews/distributed-effects-review.md`

# Distributed-effects review

## Record

Apply to every network, broker, database-commit, payment, email, provisioning,
or other externally executed effect. Record **pass**, **fail**, **not
applicable**, or **waiver reference**.

## Effect and identity

| ID     | Question                                                   | Pass evidence         |
| ------ | ---------------------------------------------------------- | --------------------- |
| DER-01 | Is each external effect listed separately?                 | effect inventory      |
| DER-02 | Is one logical operation distinct from transport attempts? | identity model        |
| DER-03 | Is operation identity generated before first dispatch?     | lifecycle trace       |
| DER-04 | Do retries reuse the logical identity?                     | attempt tests         |
| DER-05 | Is the target/resource included in identity scope?         | key contract          |
| DER-06 | Is request intent fingerprinted canonically?               | fingerprint design    |
| DER-07 | Is identity collision risk proportionate?                  | generator analysis    |
| DER-08 | Is same identity with different payload rejected?          | conflict behavior     |
| DER-09 | Are concurrent same-identity attempts coordinated?         | atomic claim          |
| DER-10 | Is identity retained for the full replay horizon?          | retention calculation |

## Timeout, outcome, and retry

| ID     | Question                                                                         | Pass evidence         |
| ------ | -------------------------------------------------------------------------------- | --------------------- |
| DER-11 | Is the point after which execution may have occurred identified?                 | protocol timeline     |
| DER-12 | Does timeout avoid implying non-execution?                                       | outcome mapping       |
| DER-13 | Is local pre-dispatch failure supported by actual protocol evidence?             | transport contract    |
| DER-14 | Are confirmed success and confirmed rejection authenticated?                     | response verification |
| DER-15 | Are confirmed, rejected, local-failure, and unknown outcomes distinct as needed? | outcome type          |
| DER-16 | Does unknown carry reconciliation evidence?                                      | stored token          |
| DER-17 | Is retry classified at every failure point?                                      | decision matrix       |
| DER-18 | Are unsafe retries prohibited?                                                   | retry policy          |
| DER-19 | Does reconcile-before-retry exist for ambiguity?                                 | transition path       |
| DER-20 | Is one end-to-end deadline propagated?                                           | deadline budget       |
| DER-21 | Is maximum retry multiplication across layers calculated?                        | attempt equation      |
| DER-22 | Are backoff, jitter, and server guidance applied?                                | policy                |
| DER-23 | Are retry concurrency and queues bounded?                                        | capacity              |
| DER-24 | Are overload and rate-limit responses preserved?                                 | error/retry handling  |

## Delivery, order, and coordination

| ID     | Question                                                    | Pass evidence         |
| ------ | ----------------------------------------------------------- | --------------------- |
| DER-25 | Are duplicates expected for at-least-once delivery?         | consumer contract     |
| DER-26 | Is deduplication durable when protecting durable effects?   | inbox/store           |
| DER-27 | Is dedup claim atomic with the local effect?                | transaction           |
| DER-28 | Is dedup retention sufficient and expiry behavior explicit? | retention/replay plan |
| DER-29 | Is acknowledgement order documented?                        | crash-point matrix    |
| DER-30 | Is acknowledgement loss handled?                            | redelivery test       |
| DER-31 | Are poison messages isolated without a hot retry loop?      | dead-letter policy    |
| DER-32 | Is administrative replay identity-preserving and audited?   | replay runbook        |
| DER-33 | Is ordering scoped to key/partition/producer/consumer?      | ordering contract     |
| DER-34 | Are gaps and out-of-order versions handled?                 | state/version policy  |
| DER-35 | Are failover and retry effects on order stated?             | scenario tests        |
| DER-36 | Is every exactly-once claim boundary-specific?              | guarantee ledger      |
| DER-37 | Are external effects outside the claimed transaction named? | boundary diagram      |
| DER-38 | Is persistence plus publication coordinated durably?        | outbox/event log      |

## Reconciliation, compensation, and authority

| ID     | Question                                                  | Pass evidence             |
| ------ | --------------------------------------------------------- | ------------------------- |
| DER-39 | Is every unknown state durable when process loss matters? | persistence model         |
| DER-40 | Is a reconciliation owner named?                          | service/runbook ownership |
| DER-41 | Is the observation source authoritative?                  | provider contract         |
| DER-42 | Are observation freshness and finality defined?           | timestamp/version/window  |
| DER-43 | Can reconciliation remain unknown?                        | repeated state path       |
| DER-44 | Are reconciliation attempts bounded and observable?       | age/attempt metrics       |
| DER-45 | Is terminal human escalation defined?                     | operations procedure      |
| DER-46 | Are operator overrides audited as decisions, not proof?   | audit event               |
| DER-47 | Is compensation modeled as a new effect?                  | saga states               |
| DER-48 | Does compensation have idempotency and unknown handling?  | effect contract           |
| DER-49 | Are concurrent coordinators claimed atomically?           | lease/CAS                 |
| DER-50 | Are stale lease owners fenced at the effect resource?     | fencing token             |
| DER-51 | Are clock and process-pause assumptions documented?       | lease analysis            |
| DER-52 | Can users safely act while state is unknown?              | API/UI behavior           |

## Audit, secrecy, and evidence

| ID     | Question                                                             | Pass evidence        |
| ------ | -------------------------------------------------------------------- | -------------------- |
| DER-53 | Does audit preserve operation, attempt, parent, trigger, and target? | event schema         |
| DER-54 | Are outcome observations and decisions reconstructible?              | incident query       |
| DER-55 | Are credentials and unnecessary personal data excluded?              | field classification |
| DER-56 | Is correlation retained without uncontrolled tracking?               | privacy policy       |
| DER-57 | Do tests inject loss before and after dispatch?                      | fault suite          |
| DER-58 | Do tests inject duplicate, delay, reordering, and crash?             | scenario matrix      |
| DER-59 | Do tests cover concurrent identity and reconciler claims?            | concurrency suite    |
| DER-60 | Does the ledger state residual unknowns and non-guarantees?          | completed ledger     |

## Exit criteria

Approval requires stable identity, exact outcome semantics, bounded safe retry,
durable reconciliation, duplicate/order handling, honest transaction scope,
auditable compensation, sensitive-data minimization, and failure-point evidence.

---

## Source: `reviews/executable-narrative-review.md`

# Executable narrative review

## Record

Use whenever a change adds a description of an architectural obligation, proposes a decision
record, adds or edits a derived view, or cites an existing record as a reason a change cannot
proceed. Record **pass**, **fail**, **not applicable**, or **waiver reference**. There is no
score: a total would let a strong result in a cheap category offset a critical failure in an
expensive one.

The review answers a question that precedes every gate below: which claim is under review, and
which single artifact is authoritative for it. A review that cannot state the claim precisely has
nothing to check.

## Source-of-truth inventory

| ID     | Question                                                              | Pass evidence            |
| ------ | --------------------------------------------------------------------- | ------------------------ |
| ENR-01 | Is the claim stated precisely enough that its truth could be checked? | claim statement          |
| ENR-02 | Which class does the claim belong to?                                 | classification           |
| ENR-03 | Which single artifact is authoritative for it?                        | authority mapping        |
| ENR-04 | Which other artifacts describe the same claim?                        | representation inventory |
| ENR-05 | Which of those are maintained by hand?                                | maintenance owner list   |
| ENR-06 | Can any of them be generated, or deleted outright?                    | disposition per entry    |
| ENR-07 | Is the representation count after this change recorded?               | review record            |

## Executability test

| ID     | Question                                                           | Pass evidence           |
| ------ | ------------------------------------------------------------------ | ----------------------- |
| ENR-08 | Can the claim become a type, a bound, or a visibility restriction? | signature or module     |
| ENR-09 | Can it become a checked constructor or a private representation?   | constructor audit       |
| ENR-10 | Can it become a schema constraint, a domain, or a cast rule?       | schema or migration     |
| ENR-11 | Can it become a test, a fixture, or a rejected-input case?         | test path               |
| ENR-12 | Can it become a manifest entry or machine-checked configuration?   | manifest or policy file |
| ENR-13 | Can the human-readable view of it be generated and drift-checked?  | generator and check     |
| ENR-14 | Can it become an executable topology or contract assertion?        | assertion path          |
| ENR-15 | If a mechanism enforces only part of it, is the rest stated?       | scope statement         |
| ENR-16 | If it stays prose, is the budget assessment recorded?              | complexity assessment   |

## Decision-record necessity test

| ID     | Question                                                        | Pass evidence          |
| ------ | --------------------------------------------------------------- | ---------------------- |
| ENR-17 | Which exact fact cannot be executable, generated, or recovered? | named fact             |
| ENR-18 | Why is that fact material to a future decision?                 | stated risk            |
| ENR-19 | Which future mistake does the record prevent?                   | failure scenario       |
| ENR-20 | Could a short comment, a manifest field, or an example suffice? | alternative comparison |
| ENR-21 | Is this a proposal to change a contract, and therefore an RFC?  | governance route       |
| ENR-22 | Is this onboarding prose in decision form?                      | audience check         |
| ENR-23 | Does the record answer one question and state its exclusions?   | scope statement        |
| ENR-24 | Does it link the artifacts authoritative for current behavior?  | linked paths           |

## Improvement-friction test

| ID     | Question                                                              | Pass evidence          |
| ------ | --------------------------------------------------------------------- | ---------------------- |
| ENR-25 | Does this artifact make a future improvement need permission from it? | dependency reading     |
| ENR-26 | Could a future reader or agent mistake it for permanent authority?    | status marking         |
| ENR-27 | Does it preserve a constraint that may disappear?                     | obsolescence condition |
| ENR-28 | Who revalidates it, and on what trigger?                              | owner and trigger      |
| ENR-29 | Is active discovery limited to currently valid records?               | registry contents      |
| ENR-30 | Was a record cited against a change without confirming it applies?    | confirmation record    |
| ENR-31 | Is an implemented proposal still cited as a current specification?    | citation audit         |

## Durable-truth test

| ID     | Question                                                           | Pass evidence          |
| ------ | ------------------------------------------------------------------ | ---------------------- |
| ENR-32 | Is a local guarantee being read as durable or remote evidence?     | ledger rows            |
| ENR-33 | Does each external fact name the system authoritative for it?      | external authority map |
| ENR-34 | Is the check that consults that system named?                      | query or call site     |
| ENR-35 | Are concurrency, fencing, and identity explicit where state moves? | token and predicate    |
| ENR-36 | Is a wire or database scalar type being read as lifecycle state?   | schema and model       |

## Narrative test

| ID     | Question                                                         | Pass evidence     |
| ------ | ---------------------------------------------------------------- | ----------------- |
| ENR-37 | Do the enforcing artifacts read as the domain's own account?     | names and states  |
| ENR-38 | Are states named for the facts they establish?                   | state definitions |
| ENR-39 | Are effects disclosed where they occur?                          | effect inventory  |
| ENR-40 | Are branches explicit rather than implied by optional fields?    | branch types      |
| ENR-41 | Is type erasure delayed to a named boundary?                     | erasure boundary  |
| ENR-42 | Does generated documentation agree with the enforcing artifacts? | drift check       |

## Rationale honesty

| ID     | Question                                                                                | Pass evidence        |
| ------ | --------------------------------------------------------------------------------------- | -------------------- |
| ENR-43 | Is recorded rationale genuinely irrecoverable from the artifacts?                       | recoverability check |
| ENR-44 | Where a reason is unavailable, is it recorded as unknown?                               | unknown record       |
| ENR-45 | Is any inference labelled as an inference, with its evidence?                           | labelled inference   |
| ENR-46 | Does every exception carry owner, consequence, control, trigger, and removal condition? | exception record     |

## Severity guidance

Treat as **critical**: one artifact cited as authority for every class; a local guarantee
presented as an external fact; an inferred rationale presented as governing; an obsolete record
still in the active set; a record cited against a change without confirming applicability; an
unenforced part of a claim left implied by the enforced part.

Treat as **high**: an enforceable obligation left in prose with no recorded assessment; a
manually maintained copy of an enforced claim; a derived view synchronized by hand; a record whose
irrecoverable fact is not named; an implemented proposal cited as a current specification; an
archived record hydrated into agent context.

Treat as **medium**: a hand-written view that is unmarked or unowned; a generated artifact with no
declared source; a representation count assessed by impression rather than stated; rationale that
restates the enforced structure without contradicting it.

## Outcome

Critical failures block merge. A valid waiver identifies the affected rule and claim, the owner
accepting the risk, the consequence, the compensating control and its evidence, an expiry or
reconsideration trigger, and the removal condition. A waiver cannot make an obsolete record
current, cannot make an inferred rationale a governing one, cannot make a local guarantee
external evidence, and cannot authorize a second maintained source for a claim an artifact
already enforces.

The most common correct outcome of this review is that no artifact is added: the obligation moves
into a mechanism, the derived view is generated, and the proposed record is not written. Record
that outcome explicitly, because a review that produces no document is easily mistaken for a
review that did not happen.

Rules exercised: `RUST-DOC-0011-R001` through `RUST-DOC-0011-R020`, with
`RUST-DOC-0010-R022` where the claim concerns a staged protocol.

---

## Source: `reviews/final-correctness-audit.md`

# Final correctness audit

## Record

Run before merge or release for material changes. Record change/release,
commit, auditor, date, applicable doctrines, focused-review references, and
**pass**, **fail**, **not applicable**, or **waiver reference** for every gate.
This audit checks evidence; it does not infer completion from CI color.

## Repository and scope integrity

| ID     | Question                                                                                  | Pass evidence                       |
| ------ | ----------------------------------------------------------------------------------------- | ----------------------------------- |
| FCA-01 | Does the diff match the approved scope?                                                   | complete diff review                |
| FCA-02 | Are unrelated user changes preserved?                                                     | status/diff provenance              |
| FCA-03 | Are all new files intentional and reviewable?                                             | full file inventory                 |
| FCA-04 | Are archives, encoded payloads, generated source commits, and transient artifacts absent? | inventory/scan                      |
| FCA-05 | Are secrets, credentials, personal paths, and internal identifiers absent?                | positive-controlled secret/PII scan |
| FCA-06 | Are canonical and generated paths separated?                                              | architecture check                  |
| FCA-07 | Are generated files derived only by the declared tool?                                    | clean regeneration                  |
| FCA-08 | Are dependency additions justified and licensed?                                          | dependency review                   |
| FCA-09 | Is MSRV/toolchain policy preserved?                                                       | toolchain matrix                    |
| FCA-10 | Is repository version/change log accurate?                                                | metadata comparison                 |

## Invariants, construction, and authority

| ID     | Question                                                                   | Pass evidence                 |
| ------ | -------------------------------------------------------------------------- | ----------------------------- |
| FCA-11 | Is the invariant inventory current?                                        | reviewed artifact             |
| FCA-12 | Does every changed trusted type have exact proof and non-proof statements? | documentation/ledger          |
| FCA-13 | Are trusted fields and constructors protected?                             | visibility/construction audit |
| FCA-14 | Do all decoders preserve construction evidence?                            | Serde/DB/boundary trace       |
| FCA-15 | Are contradictory states structurally absent or explicitly rejected?       | state truth table             |
| FCA-16 | Are legal transitions and authority explicit?                              | state/authority graph         |
| FCA-17 | Are capability cloning, transfer, expiry, and revocation honest?           | lifecycle contract            |
| FCA-18 | Are secret types protected from formatting and serialization?              | trait audit                   |
| FCA-19 | Are cross-entity invariants enforced transactionally/runtime?              | service/query evidence        |
| FCA-20 | Are escape hatches enumerated, scoped, and reviewed?                       | ledger                        |

## Boundaries, persistence, and evolution

| ID     | Question                                                   | Pass evidence         |
| ------ | ---------------------------------------------------------- | --------------------- |
| FCA-21 | Is every ingress represented raw → structural → trusted?   | boundary map          |
| FCA-22 | Are resource limits enforced before expensive processing?  | limits/tests          |
| FCA-23 | Are authentication and authorization distinct?             | request flow          |
| FCA-24 | Are unknown fields/versions/variants handled deliberately? | compatibility policy  |
| FCA-25 | Are durable formats and enum tags stable/versioned?        | schema/encoding       |
| FCA-26 | Do migrations state and verify invariant transformations?  | migration evidence    |
| FCA-27 | Are invalid historical values rejected or quarantined?     | tests/operations      |
| FCA-28 | Are lost updates and conflicts explicit?                   | version/lock protocol |
| FCA-29 | Are transaction isolation claims mechanism-specific?       | database analysis     |
| FCA-30 | Are public errors structured and redacted?                 | error tests           |

## Concurrency, effects, and uncertainty

| ID     | Question                                                                 | Pass evidence           |
| ------ | ------------------------------------------------------------------------ | ----------------------- |
| FCA-31 | Is shared mutable state ownership explicit?                              | ownership map           |
| FCA-32 | Are locks scoped and ordered?                                            | lock graph              |
| FCA-33 | Is async blocking work isolated and bounded?                             | pool/capacity design    |
| FCA-34 | Are cancellation points and cleanup reviewed?                            | cancellation matrix     |
| FCA-35 | Are tasks supervised and shutdown bounded?                               | task tree/tests         |
| FCA-36 | Are queues and concurrency bounded with backpressure?                    | capacity/overload tests |
| FCA-37 | Does every external effect remain fallible?                              | APIs                    |
| FCA-38 | Does timeout preserve unknown execution?                                 | outcome states          |
| FCA-39 | Are idempotency scope, binding, retention, and replay defined?           | key contract            |
| FCA-40 | Are duplicates and acknowledgement loss expected?                        | consumer evidence       |
| FCA-41 | Are ordering and exactly-once claims scoped?                             | guarantee ledger        |
| FCA-42 | Is persistence plus side effect coordinated without fictional atomicity? | outbox/reconciliation   |
| FCA-43 | Are compensations fallible new effects?                                  | saga model              |
| FCA-44 | Are unknown outcomes durable, owned, and reconcilable?                   | operations plan         |

## Unsafe, evidence, and performance

| ID     | Question                                                                                    | Pass evidence          |
| ------ | ------------------------------------------------------------------------------------------- | ---------------------- |
| FCA-45 | Is unsafe code absent or fully reviewed under doctrine 0007?                                | unsafe inventory/proof |
| FCA-46 | Does each unsafe block state complete safety premises?                                      | local comments         |
| FCA-47 | Are FFI ABI, ownership, unwind, and threading explicit?                                     | boundary contract      |
| FCA-48 | Are unsafe dependencies proportionally reviewed?                                            | dependency audit       |
| FCA-49 | Do tests trace to invariants and failure risks?                                             | evidence matrix        |
| FCA-50 | Are positive, negative, and prohibited programs covered?                                    | test suite             |
| FCA-51 | Are real boundaries exercised where consequential?                                          | integration evidence   |
| FCA-52 | Are cancellation, duplicate, reordering, and partial failures injected?                     | fault matrix           |
| FCA-53 | Were compile-fail diagnostics inspected semantically?                                       | reviewed stderr diff   |
| FCA-54 | Are snapshots reviewed rather than bulk accepted?                                           | focused rationale      |
| FCA-55 | Is flakiness resolved rather than retried away?                                             | failure records        |
| FCA-56 | Are model/Miri/sanitizer limits stated?                                                     | evidence limits        |
| FCA-57 | Are performance claims workload- and environment-scoped?                                    | benchmark record       |
| FCA-58 | Does profiling support optimization?                                                        | profile                |
| FCA-59 | Are latency distributions, allocation, contention, and boundary costs measured as relevant? | results                |
| FCA-60 | Is correctness evidence independent from benchmarks?                                        | suite linkage          |

## Governance and reproducibility

| ID     | Question                                                            | Pass evidence                |
| ------ | ------------------------------------------------------------------- | ---------------------------- |
| FCA-61 | Are normative changes identified rather than called wording edits?  | doctrine diff classification |
| FCA-62 | Does every required normative change have an accepted RFC?          | RFC link                     |
| FCA-63 | Are doctrine IDs and versions preserved or changed by policy?       | manifest comparison          |
| FCA-64 | Are source notes and attribution current?                           | provenance review            |
| FCA-65 | Do manifests and JSON Schemas agree?                                | lint/schema result           |
| FCA-66 | Does doctrine lint pass on the complete tree?                       | exact command/result         |
| FCA-67 | Does deterministic bundle generation produce no diff?               | generate/check result        |
| FCA-68 | Do format, Clippy, tests, compile-fail, and dependency policy pass? | exact commands/results       |
| FCA-69 | Do Markdown links pass with only narrow documented exclusions?      | link-check result            |
| FCA-70 | Is the working tree clean after regeneration and validation?        | `git status --short`         |

## Required guarantee ledger

Every major domain or case-study claim uses:

| Claim       | Established by                                 | Protected construction      | Boundary preservation         | Escape hatches   | Does not prove | Residual runtime risk |
| ----------- | ---------------------------------------------- | --------------------------- | ----------------------------- | ---------------- | -------------- | --------------------- |
| exact claim | constructor, transition, protocol, or evidence | privacy/authority mechanism | decoding and persistence path | privileged paths | excluded facts | failure/uncertainty   |

The auditor rejects rows whose claim is broader than establishment evidence.
External mutable facts state observation time and reconciliation. Passing tests
appear under evidence, never as universal proof.

## Exit criteria

Release or merge approval requires every critical item to pass, all focused
reviews to be referenced, the guarantee ledger to be complete, generation and
validation to reproduce cleanly, and residual limitations to be written in the
change record. CI confirms locally discovered results; it does not replace this
audit.

---

## Source: `agents/shared.md`

# Shared agent obligations

## Mission

Produce Rust systems whose important guarantees are discoverable, accurately
named, protected at construction and transition, preserved at boundaries, and
supported by proportionate evidence. Compilation and test success are evidence
layers, not the definition of correctness. Follow repository `AGENTS.md` and
read applicable canonical doctrine before changing code or doctrine.

## Required reasoning order

1. State domain vocabulary and desired outcome.
2. Inventory invariants using
   [`../foundations/invariants.md`](../foundations/invariants.md).
3. Classify values, states, transitions, authority, boundaries, cross-entity
   rules, temporal assumptions, and distributed facts.
4. Map every ingress, durable representation, external effect, and observation.
5. Select the simplest mechanism that directly protects the consequential
   invariant.
6. Protect construction and mutation.
7. Keep external effects and cleanup fallible.
8. Represent indeterminate execution explicitly.
9. Map claims to executable and operational evidence.
10. Complete a guarantee ledger and relevant review.

Do not begin with typestate, an error crate, `Arc<Mutex<_>>`, or an ORM model.
Begin with the invariant and trust boundary.

## Representation obligations

Use an enum for mutually exclusive runtime state. Use an opaque newtype for a
stable local value invariant. Use a validated wrapper for aggregate collection
rules. Consider a consuming transition or typestate only for a small,
locally controlled sequence. Use a capability when possession should represent
authority. Use a runtime service or transaction for cross-entity facts. Use
ordinary code when added type structure removes little consequential risk.

RUST-DOC-0001 is central. Apply evidence-accurate names: `EmailAddress` cannot
mean mailbox ownership unless verification evidence is required; `Open` cannot
mean future remote liveness. `NonZeroU64` cannot mean complete money policy.

## Boundary obligations

Model:

```text
raw input → structural value → validated domain value
          → effect attempt → observed/reconciled outcome
```

Validation is centralized, not eliminated. Audit Serde, database, file,
message, HTTP/RPC, configuration, and FFI paths for bypass. A trusted domain
type must not expose a public construction path weaker than its claim.
Authentication and authorization are separate. Persistence is historical
evidence and must be decoded against current invariants.

Apply limits before large allocations. Preserve version and unknown-value
policy. Avoid logs and diagnostics that expose credentials, secrets, or
sensitive domain values.

## Failure and uncertainty obligations

Keep expected external failure out of panics. Preserve structured categories
when callers act differently: rejection, validation, conflict, cancellation,
timeout, unavailable, and unknown execution. Do not retry by transport class
alone. A timeout after possible dispatch requires an explicit unknown state,
stable operation identity, and reconciliation plan when the effect matters.

Idempotency is a receiver protocol, not a header name. Define scope, payload
binding, concurrent attempt behavior, response replay, retention, and expiry.
Compensation is a new fallible action, not rollback.

## Evidence obligations

For each material claim identify:

- enforcement mechanism;
- construction protection;
- boundary preservation;
- escape hatches;
- positive evidence;
- negative/prohibited evidence;
- non-guarantees;
- residual runtime risk.

Use unit tests for local behavior, property tests for generative invariants,
compile-fail tests for important prohibited programs, real integration tests
for boundary behavior, fault injection for partial/distributed failures, and
model or unsafe-specific tools where warranted. Inspect compile-fail diagnostics
before updating committed expected output. Treat flaky tests as system evidence.

## Forbidden claims

Never claim:

- compilation proves domain correctness;
- passing tests prove universal correctness;
- integer money removes all rounding policy;
- parsed email proves ownership or deliverability;
- a connected typestate guarantees next network success;
- a database transaction includes unrelated external effects;
- timeout proves non-execution;
- an outbox makes end-to-end delivery exactly once;
- a lease prevents stale owners without effect-level fencing;
- async automatically makes CPU work faster;
- unsafe is sound because Miri passed.

## Canonical and generated sources

Never edit a generated file manually: everything under `dist/`, the accepted-RFC
index `rfcs/accepted/README.md`, and the doctrine coverage map
`doctrines/map.md`. Each carries a banner naming its sources. Change canonical
material, update manifests where selection changes, regenerate, and check
deterministic output. Generated text must retain its banner and source
provenance. A bundle mismatch is a failed repository state.

A pack carries the doctrine its role routinely applies. A doctrine absent from
this pack is not thereby out of force: read the applicable canonical doctrine
from `doctrines/` when the work turns on it.

## Escalation

Escalate when intent materially changes representation, authorization,
persistence, external-effect semantics, public compatibility, unsafe proof,
licensing, or normative doctrine. Before escalating, read relevant sources and
present the exact unresolved decision, consequences, evidence, and recommended
option. Do not guess through irreversible or security-sensitive ambiguity.

Normative weakening, a new escape hatch, supersession, or new normative rule
requires RFC governance. A wording edit that changes meaning is normative even
if its diff is small.

## Completion

Completion means canonical files and code are consistent; the guarantee ledger
is honest; required tests and focused reviews pass; generated output reproduces;
format, Clippy, tests, lint, schemas, dependency policy, and links pass; and the
working tree contains no accidental artifact or secret. Report failed or
unperformed checks exactly.
