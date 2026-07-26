<!--
GENERATED FILE. DO NOT EDIT DIRECTLY.
Canonical sources live under /foundations, /doctrines, /patterns,
 /boundaries, /reviews, and /agents.
-->

# Maintainer agent doctrine pack

Evolve doctrine versions and generated artifacts without eroding guarantees or provenance.

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

Never edit `dist/` manually. Change canonical material, update manifests where
selection changes, regenerate, and check deterministic output. Generated text
must retain its banner and source provenance. A bundle mismatch is a failed
repository state.

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

---

## Source: `agents/maintainer.md`

# Maintainer overlay

## Purpose

Evolve doctrine, examples, tooling, manifests, dependencies, and generated
artifacts without eroding normative meaning, provenance, reproducibility, or
MSRV compatibility. Preserve stable doctrine and rule identity so agents and
human reviews can cite durable contracts.

## Change classification

Classify before editing:

- **wording correction** — grammar or clarity with identical normative meaning;
- **non-normative clarification** — rationale/example improvement without new
  obligation;
- **patch doctrine change** — clarification with no normative meaning change;
- **minor doctrine change** — additive normative rule or substantial compatible
  expansion;
- **major doctrine change** — incompatible meaning, removal, or contract
  change;
- **supersession/deprecation** — governed lifecycle transition.

A small diff can be normative. A large generated diff may contain no normative
change. Classification follows meaning, not line count.

## RFC gate

Require an RFC for a new doctrine, normative rule, normative weakening, new
escape hatch, supersession, change to normative term meaning, significant pack
restructuring, license change, or MSRV policy change. The RFC identifies affected
rule IDs, migration, compatibility, complexity, security, evidence, and source
provenance.

Wording corrections and non-normative clarification can proceed directly only
after reviewers confirm meaning is unchanged.

## Doctrine package maintenance

Preserve the eight-file package contract. Keep README metadata and
`manifest/doctrines.yaml` synchronized: ID, slug, title, status, version, path,
applicability, risks, foundations, relations, and supersession. New rules use
the doctrine's stable prefix and never reuse removed IDs for different meaning.
Deprecated rules retain traceable history.

Update rationale, decision framework, review standard, anti-patterns, glossary,
references, source notes, related patterns/boundaries/case studies, agent
selection, examples, and CHANGELOG according to impact. Do not edit only the
normative paragraph while leaving operational material contradictory.

## Provenance

Source notes are non-normative and distinguish accepted, refined, rejected, and
added ideas. Prefer primary Rust, protocol, database, standards, and foundational
sources. Verify changing facts such as stable toolchain, action versions, and
dependency MSRV. Use short quotations only when necessary; summarize and link.
Never mirror external media or transcripts.

## Generated bundles

Never edit `/dist` directly. After canonical or manifest changes:

```text
cargo run -p bundle-agent-context -- generate
cargo run -p bundle-agent-context -- check
```

Inspect ordering, banners, source-path headings, role relevance, and compact
operational completeness. Generation must be stable across repeated runs and
independent of map iteration. A role pack should contain shared obligations,
selected canonical rules, its workflow, and applicable reviews without manually
duplicating the corpus.

## Examples and compiler evidence

Keep examples compatible with pinned MSRV and stable toolchain. Every example
has meaningful tests and honest comments. Run format, Clippy with warnings
denied, all features, and compile-fail tests. When compiler diagnostics change,
run the UI suite in overwrite/update mode only to collect candidate output;
inspect every `.stderr` to confirm the intended prohibited program still fails,
then commit.

Do not loosen privacy, trait bounds, or ownership merely to stabilize a
diagnostic. Adjust minimal fixtures if compiler wording changes without
semantic impact.

## Dependencies and tools

Add dependencies only for clear capability. Verify current release, MSRV,
license, source, advisories, feature surface, and duplicate risk. Keep
`Cargo.lock` committed. Update `deny.toml` narrowly when policy changes, never to
silence an unexplained result. Tooling CLIs must perform real validation and
carry unit tests; a success printer is not acceptable.

Toolchain/MSRV changes require compatibility evidence and governance. Edition
and resolver policy remain explicit.

## Schema and manifest maintenance

Validate both YAML manifests against Draft 2020-12 schemas. Ensure related paths
exist, IDs match folders, package files exist, statuses are allowed, and
supersession is coherent. Agent outputs are unique and under `dist/agents`.
Schema changes include fixtures or tool tests proving current documents remain
valid and invalid shapes fail.

## CI and repository hygiene

Workflows remain read-only, least privilege, and never commit or rewrite source.
Pin supported action majors or immutable SHAs according to policy. CI confirms
the complete local suite. Maintain narrow link exclusions, no broad warning
suppression, no hidden credentials, no repository payload indirection, and no
tracked build output.

Before release or push, run the documented commands, regenerate, inspect
`git diff --check`, full file inventory, forbidden-marker scan, secret/PII scan,
and clean status. Claims in a PR body must match commands actually run.

## Escalation and completion

Escalate uncertain normative meaning, incompatible migration, licensing,
security-sensitive guarantee change, unsafe proof, or policy choice. Present
source evidence and a recommended RFC path.

Maintenance completes when canonical and generated content agree, versions and
provenance are synchronized, examples/MSRV pass, manifests validate, links and
dependency policy pass, the final audit is recorded, and release notes state
real limitations. Do not declare completion while any required check fails.

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
identifies compensating evidence where needed, and survives review. “Preference” alone is
insufficient.

**SHOULD NOT** marks a normally prohibited choice whose exceptional use requires the same
documented judgment. The exception must explain why the usual failure mode does not apply or
how another control contains it.

**MAY** grants permission or identifies a genuinely optional mechanism. It does not remove
obligations from other rules. A design MAY use typestate in a suitable local protocol, but it
must still model transition failure honestly and preserve persistence boundaries.

## Scope and applicability

Every doctrine rule states applicability. The normative term governs only within that scope,
but applicability is evaluated by system behavior rather than file layout or labels. A
database adapter that constructs a domain value is a trusted-construction path even if it is
placed in an infrastructure crate. A background task that captures payment is an external
effect even if called from a method named `advance`.

“Not applicable” is a review result, not an omission. The reviewer records why the triggering
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

A rule's “allowed exceptions” section defines conditions under which its default statement
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

Silence, an inline allow attribute, a generic “legacy” label, or a passing CI job is not a
waiver. A waiver does not change the doctrine for other work. Repeated waivers can reveal that
a rule is wrong or adoption is blocked; that observation should trigger doctrine review, not
automatic normalization.

## Rule writing

A normative rule uses one stable ID such as `RUST-DOC-0001-R004` and includes:

- **Statement:** one testable obligation or tightly related contract.
- **Intent:** the failure mode or invariant protected.
- **Applicability:** the systems, paths, or conditions that trigger it.
- **Allowed exceptions:** bounded conditions or “none.”
- **Review evidence:** artifacts and observations sufficient to assess it.

Avoid combining unrelated requirements merely to reduce rule count. Avoid vague verbs such as
“handle appropriately” without defining outcomes. Name owners and failure semantics. A rule
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

| Claim | Established by | Protected construction | Boundary preservation | Escape hatches | Does not prove | Residual runtime risk |
|---|---|---|---|---|---|---|
| `PositiveMoney` is non-zero | `NonZeroU64` accepted by a fallible constructor | private field; no unchecked public constructor | DTO and row conversions call constructor | scoped migration conversion if reviewed | sufficient funds, correct FX, tax or allocation policy | overflow on later arithmetic, currency mismatch |
| `VerifiedEmailAddress` passed ownership verification | verifier-only proof token after completed challenge | private fields and restricted proof-token constructor | persisted issuer, scope, time, and address revalidated on load | administrative import with audit | future deliverability, continued control, RFC-complete validity | revocation, expiry, provider error |
| `Connection<Open>` completed local connection transition | consuming `connect` returned `Ok` | state marker and constructor visibility | not normally serialized; restoration requires a new connection | test transport factory | remote liveness at next send | immediate network failure, peer closure |
| `AuthorizedPayment` passed local authorization transition | accepted authorization response and identity/amount checks | consuming transition; capability not freely cloneable | row decode validates status and authorization reference | repair tool with scoped authorization | capture success, settlement, absence of provider reversal | timeout, expiry, provider rejection |
| `UnknownCapture` has reconciliation identity | explicit outcome constructor after ambiguous transport result | private operation and token fields | durable row stores operation identity and provider scope | manual reconciliation record with audit | whether capture succeeded or failed | delayed visibility, concurrent reconciliation |

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

Prefer “establishes,” “prevents through safe public construction,” “records that,” and “was
observed” over absolute terms such as “ensures forever.” Pair a guarantee with its
non-guarantee in the same section. If a type name repeatedly invites a stronger inference,
rename it rather than relying on distant caveats.

Honesty is not pessimism. Narrow guarantees compose. A type that accurately proves one fact is
more useful than a type that vaguely claims a whole business outcome. Explicit uncertainty
lets systems recover without corrupting their own account of reality.

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
type when domain states are mutually exclusive and carry state-specific data.

**Intent.** Remove combinations such as `is_paid = true` with no receipt or simultaneous paid
and failed flags from ordinary construction.

**Applicability.** Booleans, nullable fields, option groups, string discriminants, or structs
whose validity depends on exclusive combinations.

**Allowed exceptions.** A foreign persistence or wire DTO may retain its external shape if it
is untrusted and converted into a validated domain enum before use.

**Review evidence.** State table, exhaustive matching, invalid-combination rejection at the
boundary, and persistence evolution policy.

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

**Applicability.** nullability, ranges, uniqueness, referential integrity,
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

**Applicability.** lifecycle tables, optional payload columns, and soft-state
flags.

**Allowed exceptions.** A deliberately incomplete staging record may exist in a
separate type and table whose lifecycle never exposes it as the completed
domain entity.

**Review evidence.** row-state truth table, schema checks where feasible, and
conversion tests for every invalid combination.

## RUST-DOC-0005-R006 — Make migrations invariant-aware

**Statement.** Every migration MUST state which invariants it preserves,
strengthens, weakens, or transforms, and MUST define handling for rows that do
not satisfy the target invariant.

**Intent.** Treat migration as a domain transition rather than only a shape
change.

**Applicability.** schema, data, index, encoding, and enum migrations.

**Allowed exceptions.** A metadata-only operation may state that domain
invariants are unaffected, with evidence.

**Review evidence.** precondition query, transformation, postcondition query,
rollback or forward-repair strategy, and representative migration test.

## RUST-DOC-0005-R007 — Version durable representations

**Statement.** Persisted formats that can outlive one release MUST be versioned
or have an explicit compatibility and migration strategy.

**Intent.** Keep old values decodable without silently assigning new meaning.

**Applicability.** JSON blobs, snapshots, event payloads, files, cache entries
that survive deployment, and database schemas.

**Allowed exceptions.** Ephemeral caches may be invalidated atomically when
version changes, if stale values cannot be interpreted.

**Review evidence.** version field or schema version, supported-reader matrix,
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

**Review evidence.** stable encoding table, unknown-value path, migration plan,
and old/new reader tests.

## RUST-DOC-0005-R009 — Align transactions with cross-entity invariants

**Statement.** A cross-entity invariant that requires atomic observation and
mutation MUST be enforced within a transaction boundary and isolation mechanism
capable of protecting that invariant, or through an explicit alternative
coordination protocol.

**Intent.** Prevent application prechecks from racing concurrent writers.

**Applicability.** balances, uniqueness, inventory, state transitions,
aggregate versions, and paired records.

**Allowed exceptions.** Eventual convergence is permitted when temporary
violation is a documented domain state with bounded detection and repair.

**Review evidence.** transaction scope, isolation analysis, locking or
constraint mechanism, concurrent test, and residual anomaly statement.

## RUST-DOC-0005-R010 — Prevent lost updates

**Statement.** Read-modify-write operations subject to concurrent writers MUST
use optimistic version checks, locking, commutative updates, or another explicit
lost-update prevention strategy.

**Intent.** Stop later writes from silently erasing changes based on stale
state.

**Applicability.** mutable entities, counters with derived fields, and
administrative edits.

**Allowed exceptions.** Last-write-wins is allowed only when it is the explicit
business policy and discarded updates are acceptable and observable where
needed.

**Review evidence.** version predicate or locking query, conflict error,
concurrency test, and caller conflict policy.

## RUST-DOC-0005-R011 — Preserve transaction-handle lifecycle

**Statement.** Transaction APIs SHOULD prevent use after commit or rollback
through consuming methods or an equivalent runtime lifecycle guard. Commit
failure MUST preserve the distinction between confirmed rollback, confirmed
commit, and ambiguous outcome when the driver or protocol permits ambiguity.

**Intent.** Prevent stale transaction reuse and dishonest commit status.

**Applicability.** database clients, unit-of-work abstractions, and transactional
repositories.

**Allowed exceptions.** A library-owned mutable transaction handle may enforce
the same lifecycle at runtime when consuming APIs are incompatible with the
driver.

**Review evidence.** handle transition tests, compile-fail evidence where
useful, and connection-loss behavior.

## RUST-DOC-0005-R012 — Do not extend database atomicity to external effects

**Statement.** Database transaction success MUST NOT be claimed to include a
message, payment, file, or network effect outside the transaction's actual
resource boundary.

**Intent.** Prevent fictional atomicity across independent systems.

**Applicability.** state changes coupled to publishing or external calls.

**Allowed exceptions.** A documented distributed transaction mechanism may
state only the boundary and failure model it actually provides.

**Review evidence.** effect inventory, atomic boundary diagram, failure matrix,
and reconciliation path.

## RUST-DOC-0005-R013 — Coordinate persistence and messaging durably

**Statement.** When a domain transition and message publication must not be
silently separated, the design SHOULD use a transactional outbox, inbox, event
log, or equivalent durable coordination protocol.

**Intent.** Make retry and recovery possible after process or network failure.

**Applicability.** event publication, job enqueueing, and integration messages.

**Allowed exceptions.** A best-effort notification may remain outside durable
coordination when loss is an accepted, documented outcome.

**Review evidence.** atomic write, publisher retry, deduplication identity,
retention, ordering scope, and operational lag metrics.

## RUST-DOC-0005-R014 — Quarantine invalid historical data

**Statement.** A stored representation that fails current domain validation
MUST be rejected, quarantined, repaired through an audited migration, or exposed
as an explicit invalid-record type. It MUST NOT be forged into the trusted type.

**Intent.** Preserve the meaning of trusted domain values while allowing
operational recovery.

**Applicability.** production reads, imports, restores, and migration scans.

**Allowed exceptions.** None for trusted construction.

**Review evidence.** diagnostic classification, record identity, sensitive-data
handling, repair workflow, and metrics.

## RUST-DOC-0005-R015 — Preserve unknown fields and values deliberately

**Statement.** Readers MUST choose and document whether unknown fields or values
are rejected, ignored, retained, or mapped to an explicit unknown variant.

**Intent.** Make forward compatibility and security posture deliberate.

**Applicability.** flexible records, events, snapshots, and rolling upgrades.

**Allowed exceptions.** None; the chosen policy may be implicit in a format only
if documented and tested.

**Review evidence.** compatibility matrix and tests for extra fields, missing
fields, and unknown discriminators.

## RUST-DOC-0005-R016 — Bound stored-input resource use

**Statement.** Decoding durable values MUST enforce appropriate limits on
length, nesting, allocation, decompression, and batch size before constructing
trusted in-memory state.

**Intent.** Prevent validly encoded but hostile or corrupted records from
exhausting resources.

**Applicability.** blobs, arrays, compressed payloads, large text, and batch
queries.

**Allowed exceptions.** A format with a proven small physical bound may rely on
that bound and document it.

**Review evidence.** limits, streaming behavior, oversized fixtures, and failure
mapping.

## RUST-DOC-0005-R017 — Record persistence guarantees and non-guarantees

**Statement.** Persistence designs MUST document the exact durability,
consistency, isolation, freshness, and external-effect claims they rely on,
including configuration assumptions.

**Intent.** Prevent product names or successful calls from implying stronger
guarantees than deployed behavior.

**Applicability.** every durable domain component.

**Allowed exceptions.** None.

**Review evidence.** guarantee ledger linked to database documentation,
configuration, tests, monitoring, and residual failure modes.

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

**Review evidence.** alternatives, benchmark evidence for performance claims,
and explicit scope.

## RUST-DOC-0007-R002 — State the safety invariant

**Statement.** Every unsafe block MUST be associated with a `SAFETY:` argument
that states the relevant invariant and explains why each unsafe operation's
preconditions hold at that point.

**Intent.** Make transferred proof obligations inspectable beside the code.

**Applicability.** Explicit and compiler-required unsafe operations.

**Allowed exceptions.** Repeated operations inside one tightly bounded block may
share one complete argument when their obligations are identical.

**Review evidence.** comment names aliasing, validity, lifetime, alignment,
provenance, initialization, concurrency, and panic considerations that apply.

## RUST-DOC-0007-R003 — Minimize and encapsulate unsafe

**Statement.** Unsafe operations MUST be kept in the smallest practical lexical
and API scope and encapsulated behind a safe abstraction whenever safe callers
can use the capability.

**Intent.** Reduce proof surface and prevent invariant-dependent values from
escaping unchecked.

**Applicability.** low-level modules, FFI wrappers, containers, and optimized
algorithms.

**Allowed exceptions.** A public unsafe primitive may be appropriate when
callers must supply obligations that cannot be checked.

**Review evidence.** unsafe inventory, module visibility, private fields, and
safe wrapper tests.

## RUST-DOC-0007-R004 — Make safe APIs sound for every safe caller

**Statement.** A safe public API implemented with unsafe code MUST uphold
memory-safety requirements for all values and call sequences constructible in
safe Rust, including reentrancy, panic, cancellation, and concurrent use allowed
by its traits.

**Intent.** Prevent hidden caller obligations from leaking through a safe
signature.

**Applicability.** all safe wrappers over unsafe internals.

**Allowed exceptions.** None.

**Review evidence.** adversarial safe-call analysis, invariant ownership,
panic/drop paths, and executable evidence.

## RUST-DOC-0007-R005 — Document unsafe caller obligations

**Statement.** Every public or cross-module `unsafe fn` and unsafe trait MUST
have a `# Safety` section specifying complete caller obligations in testable,
non-circular terms.

**Intent.** Define exactly what the compiler no longer checks for the caller.

**Applicability.** unsafe functions, methods, traits, and constructors.

**Allowed exceptions.** Private functions used once may state obligations at
the function or call site, but the proof chain MUST remain explicit.

**Review evidence.** obligations name valid ranges, lifetime, ownership,
aliasing, initialization, thread, and provenance constraints as relevant.

## RUST-DOC-0007-R006 — Protect representation validity

**Statement.** Unsafe code MUST preserve Rust validity requirements for every
value that becomes observable as a typed value. It MUST NOT create invalid enum
discriminants, references, booleans, characters, nonzero values, or other
restricted representations.

**Intent.** Avoid undefined behavior before ordinary code can validate.

**Applicability.** casts, reads, transmutation, FFI, serialization shortcuts,
and uninitialized memory.

**Allowed exceptions.** Bytes may remain untyped storage until validity is
established; they MUST NOT be observed through an invalid typed value.

**Review evidence.** representation source, validation, layout reference, and
invalid-input tests.

## RUST-DOC-0007-R007 — Prove aliasing and lifetime

**Statement.** Creation or use of references from raw pointers MUST establish
non-nullness, alignment, dereferenceability, initialization, permitted aliasing,
and a lifetime no longer than the backing allocation and authority.

**Intent.** Prevent references from asserting guarantees the pointer does not
provide.

**Applicability.** raw-pointer dereference, slices from raw parts, FFI pointers,
and self-referential structures.

**Allowed exceptions.** None; only the proof mechanism varies.

**Review evidence.** allocation owner, mutation paths, reallocation analysis,
and borrow duration.

## RUST-DOC-0007-R008 — Respect provenance and bounds

**Statement.** Raw-pointer arithmetic and integer-pointer conversions MUST have
a documented provenance, allocation, element-bound, alignment, and one-past-end
argument consistent with the supported Rust model and target APIs.

**Intent.** Prevent address arithmetic from being treated as sufficient pointer
authority.

**Applicability.** allocators, buffers, intrusive structures, memory maps, and
FFI.

**Allowed exceptions.** None.

**Review evidence.** originating allocation, range proof, zero-sized-type
behavior, overflow handling, and Miri coverage where supported.

## RUST-DOC-0007-R009 — Handle partial initialization and drop

**Statement.** `MaybeUninit` and manual initialization MUST track exactly which
elements are initialized and MUST drop each initialized value exactly once on
success, error, and panic paths.

**Intent.** Prevent reads of uninitialized memory, leaks of owned resources, and
double drop.

**Applicability.** arrays, FFI output buffers, custom collections, and
performance-sensitive construction.

**Allowed exceptions.** Trivially non-dropping byte storage still requires proof
against uninitialized typed reads.

**Review evidence.** initialization counter or state, guard behavior, panic
injection, and destructor tests.

## RUST-DOC-0007-R010 — Require exceptional justification for transmute

**Statement.** `transmute` MUST require stronger justification than convenience:
source and destination size, alignment, validity, lifetime, ownership, and
layout compatibility MUST be established from authoritative contracts.

**Intent.** Expose the many simultaneous obligations hidden by one operation.

**Applicability.** every transmute or equivalent bit reinterpretation.

**Allowed exceptions.** None; a narrower cast or conversion SHOULD be used when
it expresses fewer obligations.

**Review evidence.** primary layout citation, static assertions where possible,
and tests across supported targets.

## RUST-DOC-0007-R011 — Define FFI representation and ABI

**Statement.** FFI declarations MUST specify the correct ABI and use
representations whose layout is defined for that boundary. Rust-native layout
MUST NOT be assumed stable without an applicable representation contract.

**Intent.** Prevent caller/callee disagreement about call convention and data
layout.

**Applicability.** foreign functions, callbacks, shared structs, unions, and
opaque handles.

**Allowed exceptions.** Bindings generated from an authoritative interface may
derive declarations, but generated output and generator version remain reviewed
inputs.

**Review evidence.** header/specification match, `repr` choice, target matrix,
and ABI tests.

## RUST-DOC-0007-R012 — Define FFI ownership and allocation

**Statement.** Every pointer crossing FFI MUST define nullability, length,
ownership transfer, lifetime, mutability, thread access, allocator of origin,
and the matching release operation.

**Intent.** Prevent double frees, leaks, allocator mismatch, and dangling
access.

**Applicability.** buffers, strings, handles, callbacks, and allocated objects.

**Allowed exceptions.** None; an opaque handle still requires a lifecycle
contract.

**Review evidence.** boundary table, constructor/destructor pairs, null and
length tests, and foreign-side documentation.

## RUST-DOC-0007-R013 — Control unwinding across FFI

**Statement.** Panic or foreign exception unwinding across an ABI boundary MUST
be prevented or handled according to an explicitly selected ABI and supported
runtime contract.

**Intent.** Avoid undefined behavior and uncontrolled process state.

**Applicability.** exported Rust functions, imported callbacks, and foreign
exceptions.

**Allowed exceptions.** An unwind-capable ABI may be used only with documented
cross-language behavior and target support.

**Review evidence.** catch/abort policy, destructor implications, and panic-path
test.

## RUST-DOC-0007-R014 — Prove unsafe `Send` and `Sync`

**Statement.** Every unsafe implementation of `Send` or `Sync` MUST state a
concurrency proof covering all contained state, aliasing, mutation,
destruction, callbacks, and foreign-library thread guarantees.

**Intent.** Ensure marker traits do not grant unsupported cross-thread
authority.

**Applicability.** custom containers, raw handles, FFI wrappers, and
self-referential values.

**Allowed exceptions.** None.

**Review evidence.** trait invariant, synchronization model, adverse schedule
tests, and upstream thread-safety contract.

## RUST-DOC-0007-R015 — Preserve panic safety

**Statement.** Unsafe abstractions MUST remain memory-safe if safe callbacks,
allocation, cloning, comparison, formatting, or destruction panic at any
permitted point.

**Intent.** Prevent partial mutation from violating assumptions later consumed
by unsafe code.

**Applicability.** collections, sorting, initialization, callback-based APIs,
and guards.

**Allowed exceptions.** Logical corruption after panic may be allowed only if
memory safety remains intact and the object cannot be used as though valid.

**Review evidence.** unwind-state analysis, guards, injected panics, and drop
accounting.

## RUST-DOC-0007-R016 — Use complementary dynamic evidence

**Statement.** Unsafe code SHOULD be exercised with Miri and relevant
sanitizers, fuzzing, model checking, or target-specific integration tests where
the tools support its behavior.

**Intent.** Detect violations that code review and ordinary tests can miss.

**Applicability.** pointer, initialization, FFI, and concurrency code.

**Allowed exceptions.** Unsupported operations or targets may use alternative
evidence, with the limitation documented.

**Review evidence.** exact commands, supported targets, findings resolved, and
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

**Review evidence.** dependency inventory, versions, advisory status, unsafe
surface, upstream audit evidence, and update policy.

## RUST-DOC-0007-R018 — Re-audit when assumptions change

**Statement.** Unsafe code MUST be re-reviewed when compiler behavior, target,
ABI, dependency, layout, allocation, synchronization, or surrounding safe API
assumptions change.

**Intent.** Keep proof obligations synchronized with their premises.

**Applicability.** upgrades, ports, refactors, and feature changes.

**Allowed exceptions.** A change proven outside the unsafe dependency cone may
document that conclusion.

**Review evidence.** assumption inventory, changed-premise analysis, repeated
dynamic evidence, and reviewer approval.

---

## Source: `doctrines/0008-testing-and-evidence/doctrine.md`

# Normative doctrine

## RUST-DOC-0008-R001 — Trace tests to invariants and risks

**Statement.** Tests MUST identify the invariant, contract, failure mode, or
regression risk they support.

**Intent.** Make suites evidence-oriented rather than collections of incidental
examples.

**Applicability.** all canonical tests and verification jobs.

**Allowed exceptions.** A compact regression test may reference an issue,
incident, or neighboring test module rather than repeat the full invariant.

**Review evidence.** names, documentation, or manifest mapping from claim to
test.

## RUST-DOC-0008-R002 — Test constructor acceptance and rejection

**Statement.** Validated constructors MUST have positive and negative tests at
meaningful boundaries, including normalization and error categories.

**Intent.** Demonstrate both admitted and excluded value sets.

**Applicability.** parsers, smart constructors, newtypes, collections, and
configuration.

**Allowed exceptions.** A constructor delegated entirely to a separately tested
primitive may cite that evidence and test its integration.

**Review evidence.** boundary-value table and assertions on structured errors.

## RUST-DOC-0008-R003 — Use properties for generative invariants

**Statement.** Property-based tests SHOULD cover algebraic, round-trip,
ordering, normalization, parser, and collection invariants when a small list of
examples leaves substantial input space.

**Intent.** Explore classes of inputs and produce minimized counterexamples.

**Applicability.** serialization, arithmetic, state-machine commands, parsers,
and collection operations.

**Allowed exceptions.** Exhaustive finite domains or directly proven simple
functions may use table tests.

**Review evidence.** generator domain, shrinking behavior, seed retention, and
property statement.

## RUST-DOC-0008-R004 — Prove prohibited programs where valuable

**Statement.** Compile-fail tests SHOULD preserve important API prohibitions
whose guarantee depends on privacy, ownership, traits, or typestate.

**Intent.** Detect accidental widening of legal programs.

**Applicability.** trusted construction, capability forgery, consumed handles,
state-specific operations, and trait bounds.

**Allowed exceptions.** Fragile diagnostics may be avoided when a stable API
surface check or compile test provides clearer evidence.

**Review evidence.** minimal failing programs and reviewed compiler diagnostics.

## RUST-DOC-0008-R005 — Inspect compiler-diagnostic changes

**Statement.** Committed compile-fail `.stderr` or equivalent evidence MUST NOT
be rewritten mechanically without reviewing whether the prohibited program
still fails for the intended reason.

**Intent.** Prevent snapshot acceptance from hiding weakened construction or
transition rules.

**Applicability.** trybuild and other UI test suites.

**Allowed exceptions.** Pure path, line, or diagnostic wording changes may be
accepted after semantic inspection.

**Review evidence.** diff review and assertion that the intended error remains.

## RUST-DOC-0008-R006 — Cross real boundaries

**Statement.** Integration tests SHOULD cross the real parser, protocol,
database, filesystem, or process boundary when practical and consequential.

**Intent.** Exercise adapters and assumptions that unit tests omit.

**Applicability.** boundary conversions and external integrations.

**Allowed exceptions.** Unavailable or costly systems may use faithful
emulators plus scheduled real-system evidence, with gaps documented.

**Review evidence.** environment description, real components, setup isolation,
and cleanup.

## RUST-DOC-0008-R007 — Protect protocol contracts

**Statement.** Contract tests SHOULD verify request and response schemas,
semantic categories, compatibility, idempotency, versioning, and unknown-value
behavior relied on across independently deployed components.

**Intent.** Detect integration drift before deployment.

**Applicability.** HTTP/RPC, messages, FFI, durable events, and public libraries.

**Allowed exceptions.** One jointly released private component may rely on
end-to-end integration evidence when independent compatibility is irrelevant.

**Review evidence.** provider/consumer contract, version matrix, and failure
fixtures.

## RUST-DOC-0008-R008 — Control concurrency evidence

**Statement.** Concurrency tests MUST use explicit synchronization, schedule
control, model checking, or observable events rather than sleeps as the primary
means of establishing an interleaving.

**Intent.** Avoid flaky timing guesses and unexercised schedules.

**Applicability.** locks, channels, atomics, cancellation, and shutdown.

**Allowed exceptions.** A sleep may enforce an outer deadline but MUST NOT be
the evidence that an ordering occurred.

**Review evidence.** barriers, controlled clock, Loom model, event trace, or
equivalent mechanism.

## RUST-DOC-0008-R009 — Test cancellation and cleanup

**Statement.** Async and concurrent operations MUST test cancellation at
consequential suspension points and verify resource, partial-state, and
external-outcome handling.

**Intent.** Exercise future-drop control flow.

**Applicability.** partial writes, permits, transactions, external calls, and
task supervision.

**Allowed exceptions.** Pure cancellation-safe reads may share representative
evidence when the reasoning applies identically.

**Review evidence.** controlled cancellation and postcondition assertions.

## RUST-DOC-0008-R010 — Inject partial failure

**Statement.** Fault-injection tests SHOULD exercise failures before, during,
and after durable or external steps in proportion to consequence.

**Intent.** Verify recovery rather than only returned errors.

**Applicability.** persistence, messaging, payments, filesystems, and
multi-stage operations.

**Allowed exceptions.** Low-risk pure transformations may not need fault
injection.

**Review evidence.** crash-point matrix, injected faults, resulting state, and
recovery.

## RUST-DOC-0008-R011 — Exercise distributed uncertainty

**Statement.** Distributed tests MUST exercise duplicate, delay, reordering,
lost acknowledgement, retry, and unknown outcomes when the production protocol
permits them.

**Intent.** Prevent perfect-network doubles from defining false behavior.

**Applicability.** brokers, remote APIs, reconcilers, and distributed workflows.

**Allowed exceptions.** A protocol may exclude a scenario only with
authoritative evidence.

**Review evidence.** scenario matrix and explicit terminal or unknown states.

## RUST-DOC-0008-R012 — Preserve failure modes in test doubles

**Statement.** Test doubles MUST NOT erase failure categories, cancellation,
latency, capacity, ordering, duplicate, or uncertainty behavior that is
material to the tested claim.

**Intent.** Keep tests faithful to the risk being evaluated.

**Applicability.** mocks, fakes, emulators, in-memory repositories, and clocks.

**Allowed exceptions.** A narrow unit test may use a simpler double when the
omitted behavior is outside its claim and covered elsewhere.

**Review evidence.** double-to-real contract comparison and gap ownership.

## RUST-DOC-0008-R013 — Review snapshots semantically

**Statement.** Snapshot changes MUST be reviewed as semantic output changes.
Bulk acceptance MUST NOT replace explanation of why each affected behavior is
correct.

**Intent.** Prevent expected-output updates from blessing regressions.

**Applicability.** serialized output, diagnostics, UI, plans, and compiler UI
tests.

**Allowed exceptions.** Deterministic formatting-only migrations may group
equivalent changes with one documented rationale.

**Review evidence.** focused diff, invariant impact, and reviewer sign-off.

## RUST-DOC-0008-R014 — Treat flakiness as evidence

**Statement.** A flaky test MUST be investigated as evidence of uncontrolled
time, state, environment, scheduling, isolation, or product behavior. Retries
MUST NOT be the sole resolution.

**Intent.** Prevent nondeterminism from being normalized.

**Applicability.** all test and benchmark automation.

**Allowed exceptions.** A temporary bounded retry may gather diagnostics while
the issue is owned and visible.

**Review evidence.** failure signatures, root cause, deterministic fix, or
time-bounded quarantine with owner.

## RUST-DOC-0008-R015 — Do not substitute coverage for invariant evidence

**Statement.** Coverage percentages MUST NOT be used as the sole claim that
behavior or invariants are adequately tested.

**Intent.** Distinguish executed lines from asserted semantics and input space.

**Applicability.** coverage gates and quality reports.

**Allowed exceptions.** Coverage may serve as a supplemental regression and gap
discovery metric.

**Review evidence.** invariant-to-evidence matrix in addition to coverage.

## RUST-DOC-0008-R016 — Separate benchmarks from correctness

**Statement.** Benchmarks MUST NOT substitute for correctness tests, and
correctness assertions inside benchmark setup MUST remain independently
executable where feasible.

**Intent.** Prevent performance samples from becoming weak semantic evidence.

**Applicability.** microbenchmarks, load tests, and profiling harnesses.

**Allowed exceptions.** A benchmark may validate setup defensively, but the
invariant still needs appropriate tests.

**Review evidence.** corresponding correctness suite and benchmark methodology.

## RUST-DOC-0008-R017 — Use model checking proportionally

**Statement.** Small consequential concurrent protocols SHOULD be considered
for Loom or equivalent model checking, with the model's abstraction and bounds
documented.

**Intent.** Explore scheduler interleavings ordinary runs rarely reach.

**Applicability.** atomics, locks, channels, once initialization, and ownership
handoff.

**Allowed exceptions.** Unsupported primitives or state explosion may use a
simplified model plus stress and reasoning.

**Review evidence.** modeled invariant, bounds, results, and mismatch from
production code.

## RUST-DOC-0008-R018 — Exercise unsafe code with specialized tools

**Statement.** Unsafe code SHOULD run under Miri and relevant sanitizers,
fuzzing, or target-specific tests as required by RUST-DOC-0007.

**Intent.** Add dynamic evidence for memory-model and boundary violations.

**Applicability.** unsafe internals and FFI wrappers.

**Allowed exceptions.** Tool incompatibility must be documented with
alternative evidence.

**Review evidence.** commands, results, supported targets, and blind spots.

## RUST-DOC-0008-R019 — Use production evidence carefully

**Statement.** Production telemetry and incidents SHOULD refine tests and risk
models, but MUST NOT be treated as proof that unobserved failures cannot occur.

**Intent.** Learn from real workloads without confusing absence of observation
with absence of defects.

**Applicability.** operational services and libraries with field data.

**Allowed exceptions.** None for universal claims.

**Review evidence.** telemetry coverage, detection limits, incident-derived
regressions, and residual uncertainty.

## RUST-DOC-0008-R020 — Keep tests deterministic and isolated

**Statement.** Tests MUST control or uniquely scope mutable external state,
clocks, randomness, ports, files, and environment variables required for their
claim.

**Intent.** Make failures reproducible and parallel execution safe.

**Applicability.** workspace tests and CI.

**Allowed exceptions.** Deliberate randomized or stress tests may vary inputs
but MUST record reproducible seeds and isolate effects.

**Review evidence.** temporary resource strategy, seed capture, controlled
clock, and parallel-run results.

## RUST-DOC-0008-R021 — State evidence limits

**Statement.** Every consequential evidence plan MUST state what each selected
test class proves, what it does not prove, and which risks remain observed only
in production or external systems.

**Intent.** Preserve guarantee honesty.

**Applicability.** feature plans, reviews, and release audits.

**Allowed exceptions.** Trivial local changes may reference an existing suite
contract.

**Review evidence.** evidence ledger tied to invariant inventory.

---

## Source: `doctrines/0009-performance-and-measurement/doctrine.md`

# Normative doctrine

## RUST-DOC-0009-R001 — Define objective and workload

**Statement.** Optimization MUST begin with a quantified objective and a
workload representing the input distribution, concurrency, and system boundary
that matter.

**Intent.** Prevent work on irrelevant micro-costs.

**Applicability.** performance changes, capacity plans, and regression gates.

**Allowed exceptions.** Removing an obviously unnecessary operation may proceed
as ordinary cleanup if no performance claim is made.

**Review evidence.** metric, target, baseline, workload, and correctness
constraints.

## RUST-DOC-0009-R002 — Scope every performance claim

**Statement.** Performance claims MUST include environment, toolchain, build
profile, input distribution, concurrency, warmup/cache state, measurement
method, and comparison baseline sufficient for reproduction.

**Intent.** Make numbers interpretable and falsifiable.

**Applicability.** documentation, pull requests, releases, and design decisions.

**Allowed exceptions.** A local exploratory note may be labeled preliminary and
must not support a merge claim.

**Review evidence.** reproducible command, environment manifest, raw or
summarized samples, and commit identities.

## RUST-DOC-0009-R003 — Profile before optimizing

**Statement.** Profiling SHOULD precede nontrivial optimization and MUST precede
claims about a dominant bottleneck.

**Intent.** Direct effort to measured cost centers.

**Applicability.** latency, CPU, allocation, contention, I/O, and size work.

**Allowed exceptions.** Algorithmic complexity defects apparent from complete
input bounds may be corrected without a profile, while still measuring outcome.

**Review evidence.** flamegraph, trace, allocation profile, system metrics, or
equivalent relevant evidence.

## RUST-DOC-0009-R004 — Preserve correctness independently

**Statement.** A performance change MUST preserve domain invariants,
error/uncertainty semantics, security properties, and boundary validation, with
correctness evidence independent of the benchmark.

**Intent.** Reject faster incorrect behavior.

**Applicability.** all optimizations.

**Allowed exceptions.** An explicit product tradeoff may change semantics only
as a separately reviewed normative or API change, not as hidden optimization.

**Review evidence.** invariant-linked tests and guarantee-ledger diff.

## RUST-DOC-0009-R005 — Defend benchmark execution

**Statement.** Benchmark code MUST prevent dead-code elimination, constant
folding, unintended setup measurement, and unrealistic reuse from invalidating
the intended workload.

**Intent.** Ensure measured work corresponds to the claim.

**Applicability.** microbenchmarks and component benchmarks.

**Allowed exceptions.** None; framework facilities may provide the mechanism.

**Review evidence.** input generation, black-boxing where appropriate,
setup/measurement separation, and result consumption.

## RUST-DOC-0009-R006 — Separate wall-clock and CPU claims

**Statement.** Measurements MUST distinguish wall-clock latency, CPU time, and
aggregate CPU consumption when their interpretations differ.

**Intent.** Prevent waiting and parallel work from being described as reduced
compute cost.

**Applicability.** async, parallel, I/O-bound, and multi-process workloads.

**Allowed exceptions.** A single-threaded CPU-bound benchmark may report one
measure with its assumption stated.

**Review evidence.** metric definition and collection method.

## RUST-DOC-0009-R007 — Report distributions

**Statement.** User-visible or service latency claims MUST report appropriate
distributions such as p50, p95, and p99 rather than only arithmetic averages.

**Intent.** Reveal tail behavior and multimodal workloads.

**Applicability.** requests, queues, storage, and batch completion.

**Allowed exceptions.** Deterministic fixed-cost operations may use a narrow
summary after showing low variance.

**Review evidence.** sample count, percentile method, confidence or variability,
and outlier policy.

## RUST-DOC-0009-R008 — Document warmup and cache state

**Statement.** Measurements MUST state process warmup, JIT or runtime
initialization where applicable, filesystem/page/cache state, connection reuse,
and dataset residency relevant to the claim.

**Intent.** Prevent cold and warm behavior from being mixed invisibly.

**Applicability.** storage, network, serialization, and repeated services.

**Allowed exceptions.** A test may deliberately mix states only if the workload
distribution matches production and is documented.

**Review evidence.** preparation sequence and separate cold/warm results where
both matter.

## RUST-DOC-0009-R009 — Measure allocation claims

**Statement.** Claims that code allocates less, performs no allocation, or
reduces memory MUST be supported by an allocator-aware measurement and MUST
identify retained as well as peak memory where relevant.

**Intent.** Avoid inferring allocation from syntax or clone count.

**Applicability.** buffering, parsing, collections, async boxing, and caching.

**Allowed exceptions.** A direct removal of the only allocation call may be
noted structurally, but broader runtime claims still require measurement.

**Review evidence.** allocation count/bytes, allocator, peak/resident set, and
workload.

## RUST-DOC-0009-R010 — Scope zero-copy claims

**Statement.** A zero-copy claim MUST identify every copy avoided within the
specified path and the lifetime, pinning, retention, fragmentation, API, and
ownership costs introduced.

**Intent.** Prevent one avoided copy from becoming a broad slogan.

**Applicability.** parsers, networking, serialization, buffers, and FFI.

**Allowed exceptions.** None for the phrase "zero-copy."

**Review evidence.** data-flow diagram, measured copy/allocation evidence, and
non-guarantees.

## RUST-DOC-0009-R011 — Do not equate async with speedup

**Statement.** Async concurrency MUST NOT be described as parallel CPU speedup
without evidence of parallel execution and a workload that benefits.

**Intent.** Distinguish overlap of waiting from reduced compute time.

**Applicability.** runtime migrations, fan-out, and worker design.

**Allowed exceptions.** None for the claim; async may still improve resource
efficiency or concurrent latency.

**Review evidence.** executor configuration, CPU utilization, throughput,
latency, and contention.

## RUST-DOC-0009-R012 — Make throughput/latency tradeoffs explicit

**Statement.** Batching, buffering, pipelining, and concurrency changes MUST
report both throughput and relevant latency/queue consequences.

**Intent.** Prevent aggregate gains from hiding worse tails or freshness.

**Applicability.** brokers, databases, serializers, and service queues.

**Allowed exceptions.** Offline throughput-only jobs may state that latency has
no objective while still bounding resource use.

**Review evidence.** batch/concurrency sweep and distribution results.

## RUST-DOC-0009-R013 — Measure contention and backpressure

**Statement.** Concurrent performance analysis MUST include queue depth, wait
time, saturation, lock or permit contention, rejection, and downstream load
where relevant.

**Intent.** Reveal whether local throughput shifts cost elsewhere.

**Applicability.** shared state, pools, channels, and fan-out.

**Allowed exceptions.** Pure independent parallel work may document absence of
shared contention.

**Review evidence.** contention profile, load curve, and overload behavior.

## RUST-DOC-0009-R014 — Count boundary costs

**Statement.** Performance investigations MUST consider serialization,
allocation, copies, syscalls, context switches, database queries, network
round-trips, and external rate limits before attributing cost solely to Rust
source constructs.

**Intent.** Optimize the actual end-to-end path.

**Applicability.** integrated and service workloads.

**Allowed exceptions.** A deliberately isolated microbenchmark may narrow scope
and state that it excludes boundary cost.

**Review evidence.** trace or component budget.

## RUST-DOC-0009-R015 — Review clone removal architecturally

**Statement.** Avoiding `clone` MUST NOT introduce worse algorithmic complexity,
excessive borrowing, global sharing, lock contention, or retention without
measurement and ownership analysis.

**Intent.** Prevent syntax-focused optimization from degrading architecture.

**Applicability.** buffers, collections, async tasks, and shared state.

**Allowed exceptions.** Removal of a proven redundant clone with unchanged
ownership shape may be a local cleanup.

**Review evidence.** data ownership, allocation profile, complexity, and
contention.

## RUST-DOC-0009-R016 — Govern unsafe optimization

**Statement.** Unsafe performance changes MUST satisfy RUST-DOC-0007 and MUST
show a material measured benefit under the target workload.

**Intent.** Charge proof risk to the benefit it buys.

**Applicability.** unchecked indexing, custom allocation, SIMD, FFI, and
lock-free code.

**Allowed exceptions.** Unsafe may be necessary for an external API even when
performance is not its justification; that case is not an optimization claim.

**Review evidence.** safe baseline, benchmark, profile, safety proof, and
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

**Review evidence.** baseline history, variance, threshold, hardware stability,
and rerun policy.

## RUST-DOC-0009-R018 — Do not generalize microbenchmarks

**Statement.** Microbenchmark results MUST NOT be generalized to end-to-end
performance without evidence connecting the measured operation to overall
workload contribution.

**Intent.** Prevent large local ratios from masking tiny system impact.

**Applicability.** library and application optimization claims.

**Allowed exceptions.** A microbenchmark may establish the cost of the exact
isolated primitive it measures.

**Review evidence.** profile share, integrated benchmark, or component budget.

## RUST-DOC-0009-R019 — Account for build and binary cost

**Statement.** Abstraction choices involving generics, code generation, feature
sets, or dependencies SHOULD assess compile time, monomorphization, binary size,
incremental behavior, and diagnostic cost when material.

**Intent.** Treat developer and deployment resources as performance dimensions.

**Applicability.** public generic APIs, macro-heavy code, and constrained
artifacts.

**Allowed exceptions.** Small local code with immaterial measured impact may
document no concern.

**Review evidence.** build timing, artifact sections, generic instantiations, or
dependency analysis.

## RUST-DOC-0009-R020 — Retain reproducible evidence

**Statement.** Accepted performance decisions MUST retain commands, commits,
configuration, result summaries, and raw-data location or format sufficient to
repeat or challenge the result.

**Intent.** Make optimization decisions durable and auditable.

**Applicability.** merged performance changes and release claims.

**Allowed exceptions.** Sensitive production traces may be retained in
controlled storage with a sanitized reproducible summary.

**Review evidence.** benchmark record and provenance.

---

## Source: `reviews/final-correctness-audit.md`

# Final correctness audit

## Record

Run before merge or release for material changes. Record change/release,
commit, auditor, date, applicable doctrines, focused-review references, and
**pass**, **fail**, **not applicable**, or **waiver reference** for every gate.
This audit checks evidence; it does not infer completion from CI color.

## Repository and scope integrity

| ID | Question | Pass evidence |
|---|---|---|
| FCA-01 | Does the diff match the approved scope? | complete diff review |
| FCA-02 | Are unrelated user changes preserved? | status/diff provenance |
| FCA-03 | Are all new files intentional and reviewable? | full file inventory |
| FCA-04 | Are archives, encoded payloads, generated source commits, and transient artifacts absent? | inventory/scan |
| FCA-05 | Are secrets, credentials, personal paths, and internal identifiers absent? | positive-controlled secret/PII scan |
| FCA-06 | Are canonical and generated paths separated? | architecture check |
| FCA-07 | Are generated files derived only by the declared tool? | clean regeneration |
| FCA-08 | Are dependency additions justified and licensed? | dependency review |
| FCA-09 | Is MSRV/toolchain policy preserved? | toolchain matrix |
| FCA-10 | Is repository version/change log accurate? | metadata comparison |

## Invariants, construction, and authority

| ID | Question | Pass evidence |
|---|---|---|
| FCA-11 | Is the invariant inventory current? | reviewed artifact |
| FCA-12 | Does every changed trusted type have exact proof and non-proof statements? | documentation/ledger |
| FCA-13 | Are trusted fields and constructors protected? | visibility/construction audit |
| FCA-14 | Do all decoders preserve construction evidence? | Serde/DB/boundary trace |
| FCA-15 | Are contradictory states structurally absent or explicitly rejected? | state truth table |
| FCA-16 | Are legal transitions and authority explicit? | state/authority graph |
| FCA-17 | Are capability cloning, transfer, expiry, and revocation honest? | lifecycle contract |
| FCA-18 | Are secret types protected from formatting and serialization? | trait audit |
| FCA-19 | Are cross-entity invariants enforced transactionally/runtime? | service/query evidence |
| FCA-20 | Are escape hatches enumerated, scoped, and reviewed? | ledger |

## Boundaries, persistence, and evolution

| ID | Question | Pass evidence |
|---|---|---|
| FCA-21 | Is every ingress represented raw → structural → trusted? | boundary map |
| FCA-22 | Are resource limits enforced before expensive processing? | limits/tests |
| FCA-23 | Are authentication and authorization distinct? | request flow |
| FCA-24 | Are unknown fields/versions/variants handled deliberately? | compatibility policy |
| FCA-25 | Are durable formats and enum tags stable/versioned? | schema/encoding |
| FCA-26 | Do migrations state and verify invariant transformations? | migration evidence |
| FCA-27 | Are invalid historical values rejected or quarantined? | tests/operations |
| FCA-28 | Are lost updates and conflicts explicit? | version/lock protocol |
| FCA-29 | Are transaction isolation claims mechanism-specific? | database analysis |
| FCA-30 | Are public errors structured and redacted? | error tests |

## Concurrency, effects, and uncertainty

| ID | Question | Pass evidence |
|---|---|---|
| FCA-31 | Is shared mutable state ownership explicit? | ownership map |
| FCA-32 | Are locks scoped and ordered? | lock graph |
| FCA-33 | Is async blocking work isolated and bounded? | pool/capacity design |
| FCA-34 | Are cancellation points and cleanup reviewed? | cancellation matrix |
| FCA-35 | Are tasks supervised and shutdown bounded? | task tree/tests |
| FCA-36 | Are queues and concurrency bounded with backpressure? | capacity/overload tests |
| FCA-37 | Does every external effect remain fallible? | APIs |
| FCA-38 | Does timeout preserve unknown execution? | outcome states |
| FCA-39 | Are idempotency scope, binding, retention, and replay defined? | key contract |
| FCA-40 | Are duplicates and acknowledgement loss expected? | consumer evidence |
| FCA-41 | Are ordering and exactly-once claims scoped? | guarantee ledger |
| FCA-42 | Is persistence plus side effect coordinated without fictional atomicity? | outbox/reconciliation |
| FCA-43 | Are compensations fallible new effects? | saga model |
| FCA-44 | Are unknown outcomes durable, owned, and reconcilable? | operations plan |

## Unsafe, evidence, and performance

| ID | Question | Pass evidence |
|---|---|---|
| FCA-45 | Is unsafe code absent or fully reviewed under doctrine 0007? | unsafe inventory/proof |
| FCA-46 | Does each unsafe block state complete safety premises? | local comments |
| FCA-47 | Are FFI ABI, ownership, unwind, and threading explicit? | boundary contract |
| FCA-48 | Are unsafe dependencies proportionally reviewed? | dependency audit |
| FCA-49 | Do tests trace to invariants and failure risks? | evidence matrix |
| FCA-50 | Are positive, negative, and prohibited programs covered? | test suite |
| FCA-51 | Are real boundaries exercised where consequential? | integration evidence |
| FCA-52 | Are cancellation, duplicate, reordering, and partial failures injected? | fault matrix |
| FCA-53 | Were compile-fail diagnostics inspected semantically? | reviewed stderr diff |
| FCA-54 | Are snapshots reviewed rather than bulk accepted? | focused rationale |
| FCA-55 | Is flakiness resolved rather than retried away? | failure records |
| FCA-56 | Are model/Miri/sanitizer limits stated? | evidence limits |
| FCA-57 | Are performance claims workload- and environment-scoped? | benchmark record |
| FCA-58 | Does profiling support optimization? | profile |
| FCA-59 | Are latency distributions, allocation, contention, and boundary costs measured as relevant? | results |
| FCA-60 | Is correctness evidence independent from benchmarks? | suite linkage |

## Governance and reproducibility

| ID | Question | Pass evidence |
|---|---|---|
| FCA-61 | Are normative changes identified rather than called wording edits? | doctrine diff classification |
| FCA-62 | Does every required normative change have an accepted RFC? | RFC link |
| FCA-63 | Are doctrine IDs and versions preserved or changed by policy? | manifest comparison |
| FCA-64 | Are source notes and attribution current? | provenance review |
| FCA-65 | Do manifests and JSON Schemas agree? | lint/schema result |
| FCA-66 | Does doctrine lint pass on the complete tree? | exact command/result |
| FCA-67 | Does deterministic bundle generation produce no diff? | generate/check result |
| FCA-68 | Do format, Clippy, tests, compile-fail, and dependency policy pass? | exact commands/results |
| FCA-69 | Do Markdown links pass with only narrow documented exclusions? | link-check result |
| FCA-70 | Is the working tree clean after regeneration and validation? | `git status --short` |

## Required guarantee ledger

Every major domain or case-study claim uses:

| Claim | Established by | Protected construction | Boundary preservation | Escape hatches | Does not prove | Residual runtime risk |
|---|---|---|---|---|---|---|
| exact claim | constructor, transition, protocol, or evidence | privacy/authority mechanism | decoding and persistence path | privileged paths | excluded facts | failure/uncertainty |

The auditor rejects rows whose claim is broader than establishment evidence.
External mutable facts state observation time and reconciliation. Passing tests
appear under evidence, never as universal proof.

## Exit criteria

Release or merge approval requires every critical item to pass, all focused
reviews to be referenced, the guarantee ledger to be complete, generation and
validation to reproduce cleanly, and residual limitations to be written in the
change record. CI confirms locally discovered results; it does not replace this
audit.
