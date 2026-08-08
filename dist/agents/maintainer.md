<!--
GENERATED FILE. DO NOT EDIT DIRECTLY.
Canonical sources live under /foundations, /doctrines, /patterns,
 /boundaries, /reviews, and /agents.
-->

# Maintainer agent doctrine pack

Evolve doctrine versions and generated artifacts without eroding guarantees or provenance.

## Assembly

Ceiling `operational`, declared for the `maintainer` pack in `manifest/agents.yaml`. A section annotated above that ceiling is withheld here. Nothing was withheld at this ceiling.

Obligations are never withheld. A doctrine's normative file, every foundation, every agent overlay, and every review checklist carry no annotation, and generation rejects one. Canonical sources carry every section, and `dist/full-doctrine.md` carries the corpus with no ceiling applied.

---

## Source: `agents/shared.md`

# Shared agent obligations

## Mission

Produce Rust systems whose important guarantees are discoverable, accurately
named, protected at construction and transition, preserved at boundaries, and
supported by proportionate evidence. Compilation and test success are evidence
layers, not the definition of correctness. Follow repository [`AGENTS.md`](../../AGENTS.md) and
read applicable canonical doctrine before changing code or doctrine.

## Required reasoning order

1. State domain vocabulary and desired outcome.
2. Inventory invariants using
   [`../foundations/invariants.md`](../../foundations/invariants.md).
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
index [`rfcs/accepted/README.md`](../../rfcs/accepted/README.md), and the doctrine coverage map
[`doctrines/map.md`](../../doctrines/map.md). Each carries a banner naming its sources. Change canonical
material, update manifests where selection changes, regenerate, and check
deterministic output. Generated text must retain its banner and source
provenance. A bundle mismatch is a failed repository state.

A pack carries the doctrine its role routinely applies. A doctrine absent from
this pack is not thereby out of force: read the applicable canonical doctrine
from [`doctrines/`](../../doctrines/) when the work turns on it.

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
[`manifest/doctrines.yaml`](../../manifest/doctrines.yaml) synchronized: ID, slug, title, status, version, path,
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

```bash
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
`Cargo.lock` committed. Update [`deny.toml`](../../deny.toml) narrowly when policy changes, never to
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

## Decision-record lifecycle

Under RUST-DOC-0011, the record set is maintained, not accumulated:

- revalidate each active record at its stated trigger, and record the confirmation and its date;
- expire or archive a record whose external constraint, commitment, or accepted risk no longer
  applies; a record does not stay active because nobody revisited it;
- move an archived record under `decisions/archive/`, mark it as not current operational
  authority in its own text, and update the registry entry with the reason;
- keep the active set narrow, and keep archived records out of generated agent context;
- regenerate the bundles after any change to the registry or the canonical sources, and inspect
  the generated difference rather than trusting it;
- when retiring a record, check whether the obligation it carried can now be enforced by an
  artifact instead, and prefer that to a replacement record.

Do not let the archive become the default destination. A record whose reason has ended and that
carries no compatibility or audit obligation is deleted.

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
Repository governance contracts such as [`AGENTS.md`](../../AGENTS.md), [`CONTRIBUTING.md`](../../CONTRIBUTING.md), and [`rfcs/README.md`](../../rfcs/README.md)
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

**Enforcement.** Unenforceable: Nothing shows discovery preceded representation, nor that the
inventory is complete

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

**Enforcement.** [`examples/domain-modeling/src/lib.rs`](../../examples/domain-modeling/src/lib.rs)
— InvoiceState binds receipt to Paid, reason to Failed

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

**Enforcement.**
[`construct_verified_email_fields.rs`](../../examples/compile-fail/ui/construct_verified_email_fields.rs)
— compiler rejects writing the private evidence field

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

**Enforcement.** [`examples/validated-newtypes/src/lib.rs`](../../examples/validated-newtypes/src/lib.rs)
— TryFrom delegates to the single parse policy

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

**Enforcement.** Unenforceable: No check compares a name's implied evidence against what
construction establishes

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

**Enforcement.** [`examples/boundary-validation/src/lib.rs`](../../examples/boundary-validation/src/lib.rs)
— serde try_from; invalid JSON email rejected

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

**Enforcement.** [`examples/boundary-validation/src/lib.rs`](../../examples/boundary-validation/src/lib.rs)
— TryFrom row; invalid-history rows rejected

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

**Enforcement.** Unenforceable: No compiled collection wrapper exists; mutation-surface completeness
is a per-API audit

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

**Enforcement.** [`reuse_consumed_transaction.rs`](../../examples/compile-fail/ui/reuse_consumed_transaction.rs)
— staging after commit fails to compile

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

**Enforcement.** Unenforceable: Weighing typestate cost against the invalid programs prevented is
unmeasured

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

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— origin evidence erased to runtime OriginKind

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

**Enforcement.** Unenforceable: No capability type defining issuance, clone, transfer, expiry,
revocation exists here

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

**Enforcement.** [`examples/typestate/src/lib.rs`](../../examples/typestate/src/lib.rs) — Open
connection send stays fallible

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

**Enforcement.** [`examples/distributed-outcomes/src/lib.rs`](../../examples/distributed-outcomes/src/lib.rs)
— ambiguity maps to reconcile, never rejection

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

**Enforcement.** [`examples/distributed-outcomes/src/lib.rs`](../../examples/distributed-outcomes/src/lib.rs)
— Unknown carries reconciliation identity

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

**Enforcement.** [`examples/unsafe-evidence/Cargo.toml`](../../examples/unsafe-evidence/Cargo.toml)
— the sole unsafe bypass, named and scoped

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

**Enforcement.** Unenforceable: No unsafe constructor exists; proof-obligation completeness is a
soundness argument

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

**Enforcement.** [`examples/compile-fail/tests/ui.rs`](../../examples/compile-fail/tests/ui.rs) —
trybuild runs nine cases against recorded stderr

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

**Enforcement.** Unenforceable: No check ties a stated guarantee to its actual construction and
boundaries

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

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— uniqueness rechecked at runtime and scoped

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

**Enforcement.** [`examples/domain-modeling/src/lib.rs`](../../examples/domain-modeling/src/lib.rs)
— PositiveMoney rejects mismatch and overflow

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

**Enforcement.** [`examples/validated-newtypes/src/lib.rs`](../../examples/validated-newtypes/src/lib.rs)
— syntax policy versus ownership-proof evidence

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

**Enforcement.** [`examples/boundary-validation/src/lib.rs`](../../examples/boundary-validation/src/lib.rs)
— fallible row conversion treats a stored row as untrusted

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

**Enforcement.** [`examples/boundary-validation/src/lib.rs`](../../examples/boundary-validation/src/lib.rs)
— raw row kept distinct from the domain type

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

**Enforcement.** [`examples/boundary-validation/src/lib.rs`](../../examples/boundary-validation/src/lib.rs)
— serde and row decode both route through the checked constructor

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

**Enforcement.** Unenforceable: No schema in repo; zero SQL or DDL files exist

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

**Enforcement.** Unenforceable: No example row carries nullable or flag columns forming
contradictory combinations

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

**Enforcement.** Unenforceable: Repository ships no migrations; zero migration or SQL files exist

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

**Enforcement.** Unenforceable: No persisted format carries a version field or supported-reader
matrix

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

**Enforcement.** [`examples/boundary-validation/src/lib.rs`](../../examples/boundary-validation/src/lib.rs)
— unknown persisted discriminator is rejected

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

**Enforcement.** Unenforceable: No database, isolation level, or concurrent-writer test exists in
workspace

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

**Enforcement.** Unenforceable: Only a conflict enum name; no version predicate or competing-writer
test

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

**Enforcement.** [`reuse_consumed_transaction.rs`](../../examples/compile-fail/ui/reuse_consumed_transaction.rs)
— staging after commit does not compile

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

**Enforcement.** Unenforceable: Examples ship no database transaction, so no commit boundary can be
overclaimed

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

**Enforcement.** Unenforceable: No outbox, inbox, or event log exists; examples avoid messaging
entirely

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

**Enforcement.** [`examples/boundary-validation/src/lib.rs`](../../examples/boundary-validation/src/lib.rs)
— invalid stored value yields a structured error, not a forged domain type

## RUST-DOC-0005-R015 — Preserve unknown fields and values deliberately

**Statement.** Readers MUST choose and document whether unknown fields or values
are rejected, ignored, retained, or mapped to an explicit unknown variant.

**Intent.** Make forward compatibility and security posture deliberate.

**Applicability.** Flexible records, events, snapshots, and rolling upgrades.

**Allowed exceptions.** None; the chosen policy may be implicit in a format only
if documented and tested.

**Review evidence.** Compatibility matrix and tests for extra fields, missing
fields, and unknown discriminators.

**Enforcement.** [`examples/boundary-validation/src/lib.rs`](../../examples/boundary-validation/src/lib.rs)
— unknown persisted value policy is reject, and is asserted

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

**Enforcement.** Unenforceable: Only a name-length constant; no oversized, nested, compressed, or
batch fixtures

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

**Enforcement.** Unenforceable: No deployed database; no durability or isolation configuration
exists to document

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

**Enforcement.** [`examples/unsafe-evidence/README.md`](../../examples/unsafe-evidence/README.md) —
required capability, rejected safe alternatives, scope

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

**Enforcement.** [`examples/unsafe-evidence/src/lib.rs`](../../examples/unsafe-evidence/src/lib.rs)
— four unsafe blocks, each with a local safety argument

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

**Enforcement.** [`examples/unsafe-evidence/src/lib.rs`](../../examples/unsafe-evidence/src/lib.rs)
— narrow blocks, private guard, safe wrapper, scoped hatch

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

**Enforcement.** [`examples/unsafe-evidence/src/lib.rs`](../../examples/unsafe-evidence/src/lib.rs)
— tests drive panicking builder, error, zero-length, zero-sized paths

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

**Enforcement.** Unenforceable: Workspace exports no unsafe fn or unsafe trait needing a safety
section

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

**Enforcement.** [`examples/unsafe-evidence/src/lib.rs`](../../examples/unsafe-evidence/src/lib.rs)
— storage stays uninitialized until all writes complete

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

**Enforcement.** [`examples/unsafe-evidence/src/lib.rs`](../../examples/unsafe-evidence/src/lib.rs)
— the guard proves unique aliasing bounded by the storage

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

**Enforcement.** [`examples/unsafe-evidence/src/lib.rs`](../../examples/unsafe-evidence/src/lib.rs)
— pointer provenance and bounds, interpreted under Miri

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

**Enforcement.** [`examples/unsafe-evidence/src/lib.rs`](../../examples/unsafe-evidence/src/lib.rs)
— the prefix counter drops exactly the initialized prefix

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

**Enforcement.** Unenforceable: No transmute or bit reinterpretation exists in any workspace crate

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

**Enforcement.** Unenforceable: No extern block, repr(C) type, or FFI declaration exists in the
workspace

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

**Enforcement.** Unenforceable: No pointer crosses FFI; workspace has no foreign functions, handles,
or buffers

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

**Enforcement.** Unenforceable: No exported extern function or foreign callback; unwinding never
crosses an ABI

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

**Enforcement.** Unenforceable: No unsafe Send or Sync impl exists; the concurrency proof is never
exercised

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

**Enforcement.** [`examples/unsafe-evidence/src/lib.rs`](../../examples/unsafe-evidence/src/lib.rs)
— an injected panic asserts drop accounting

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

**Enforcement.** [`.github/workflows/rust-examples.yml`](../../.github/workflows/rust-examples.yml)
— the Miri job reruns the tests on a pinned nightly

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

**Enforcement.** Unenforceable: Example crate has zero third-party dependencies; no unsafe
dependency surface exists

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

**Enforcement.** Unenforceable: Only a trigger list is documented; no artifact records a re-audit
after a changed premise

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

**Enforcement.** [`examples/src/lib.rs`](../../examples/src/lib.rs) — the module doc names the rule
its tests support

## RUST-DOC-0008-R002 — Test constructor acceptance and rejection

**Statement.** Validated constructors MUST have positive and negative tests at
meaningful boundaries, including normalization and error categories.

**Intent.** Demonstrate both admitted and excluded value sets.

**Applicability.** Parsers, smart constructors, newtypes, collections, and
configuration.

**Allowed exceptions.** A constructor delegated entirely to a separately tested
primitive may cite that evidence and test its integration.

**Review evidence.** Boundary-value table and assertions on structured errors.

**Enforcement.** [`examples/validated-newtypes/src/lib.rs`](../../examples/validated-newtypes/src/lib.rs)
— accept and reject at bounds, asserting categories

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

**Enforcement.** Unenforceable: No property harness in workspace; substantial input space is a
judgment threshold

## RUST-DOC-0008-R004 — Prove prohibited programs where valuable

**Statement.** Compile-fail tests SHOULD preserve important API prohibitions
whose guarantee depends on privacy, ownership, traits, or typestate.

**Intent.** Detect accidental widening of legal programs.

**Applicability.** Trusted construction, capability forgery, consumed handles,
state-specific operations, and trait bounds.

**Allowed exceptions.** Fragile diagnostics may be avoided when a stable API
surface check or compile test provides clearer evidence.

**Review evidence.** Minimal failing programs and reviewed compiler diagnostics.

**Enforcement.** [`examples/compile-fail/tests/ui.rs`](../../examples/compile-fail/tests/ui.rs) —
trybuild harness over nine prohibited programs

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

**Enforcement.** Unenforceable: Nothing distinguishes a reviewed stderr regeneration from a
mechanical overwrite

## RUST-DOC-0008-R006 — Cross real boundaries

**Statement.** Integration tests SHOULD cross the real parser, protocol,
database, filesystem, or process boundary when practical and consequential.

**Intent.** Exercise adapters and assumptions that unit tests omit.

**Applicability.** Boundary conversions and external integrations.

**Allowed exceptions.** Unavailable or costly systems may use faithful
emulators plus scheduled real-system evidence, with gaps documented.

**Review evidence.** Environment description, real components, setup isolation,
and cleanup.

**Enforcement.** [`examples/boundary-validation/src/lib.rs`](../../examples/boundary-validation/src/lib.rs)
— deserializes through the real codec into checked types

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

**Enforcement.** Unenforceable: No independently deployed components; a version matrix is
unrepresentable here

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

**Enforcement.** Unenforceable: No concurrent tests exist; sleep as deadline versus evidence needs
reviewer judgment

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

**Enforcement.** Unenforceable: Workspace has no async or cancellable operations; suspension points
are project-specific

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

**Enforcement.** Unenforceable: No durable or external steps here; proportion to consequence fixes
no threshold

## RUST-DOC-0008-R011 — Exercise distributed uncertainty

**Statement.** Distributed tests MUST exercise duplicate, delay, reordering,
lost acknowledgement, retry, and unknown outcomes when the production protocol
permits them.

**Intent.** Prevent perfect-network doubles from defining false behavior.

**Applicability.** Brokers, remote APIs, reconcilers, and distributed workflows.

**Allowed exceptions.** A protocol may exclude a scenario only with
authoritative evidence.

**Review evidence.** Scenario matrix and explicit terminal or unknown states.

**Enforcement.** [`examples/distributed-outcomes/src/lib.rs`](../../examples/distributed-outcomes/src/lib.rs)
— unknown stays unknown and retries reuse identity

## RUST-DOC-0008-R012 — Preserve failure modes in test doubles

**Statement.** Test doubles MUST NOT erase failure categories, cancellation,
latency, capacity, ordering, duplicate, or uncertainty behavior that is
material to the tested claim.

**Intent.** Keep tests faithful to the risk being evaluated.

**Applicability.** Mocks, fakes, emulators, in-memory repositories, and clocks.

**Allowed exceptions.** A narrow unit test may use a simpler double when the
omitted behavior is outside its claim and covered elsewhere.

**Review evidence.** Double-to-real contract comparison and gap ownership.

**Enforcement.** Unenforceable: No mocks or fakes in workspace; double-to-real fidelity is reviewer
judgment

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

**Enforcement.** Unenforceable: Whether a snapshot diff blesses a regression is decidable only by
reading it

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

**Enforcement.** Unenforceable: Flakiness lives in CI history; root cause versus retry is a human
call

## RUST-DOC-0008-R015 — Do not substitute coverage for invariant evidence

**Statement.** Coverage percentages MUST NOT be used as the sole claim that
behavior or invariants are adequately tested.

**Intent.** Distinguish executed lines from asserted semantics and input space.

**Applicability.** Coverage gates and quality reports.

**Allowed exceptions.** Coverage may serve as a supplemental regression and gap
discovery metric.

**Review evidence.** Invariant-to-evidence matrix in addition to coverage.

**Enforcement.** Unenforceable: No coverage tooling configured; sole claim is a property of an
argument

## RUST-DOC-0008-R016 — Separate benchmarks from correctness

**Statement.** Benchmarks MUST NOT substitute for correctness tests, and
correctness assertions inside benchmark setup MUST remain independently
executable where feasible.

**Intent.** Prevent performance samples from becoming weak semantic evidence.

**Applicability.** Microbenchmarks, load tests, and profiling harnesses.

**Allowed exceptions.** A benchmark may validate setup defensively, but the
invariant still needs appropriate tests.

**Review evidence.** Corresponding correctness suite and benchmark methodology.

**Enforcement.** Unenforceable: Workspace ships no benchmarks, so no benchmark separation can be
observed

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

**Enforcement.** Unenforceable: No model checker or concurrent protocol; proportional consideration
leaves no trace

## RUST-DOC-0008-R018 — Exercise unsafe code with specialized tools

**Statement.** Unsafe code SHOULD run under Miri and relevant sanitizers,
fuzzing, or target-specific tests as required by RUST-DOC-0007.

**Intent.** Add dynamic evidence for memory-model and boundary violations.

**Applicability.** Unsafe internals and FFI wrappers.

**Allowed exceptions.** Tool incompatibility must be documented with
alternative evidence.

**Review evidence.** Commands, results, supported targets, and blind spots.

**Enforcement.** [`.github/workflows/rust-examples.yml`](../../.github/workflows/rust-examples.yml)
— the Miri job reruns unsafe evidence on a pinned nightly

## RUST-DOC-0008-R019 — Use production evidence carefully

**Statement.** Production telemetry and incidents SHOULD refine tests and risk
models, but MUST NOT be treated as proof that unobserved failures cannot occur.

**Intent.** Learn from real workloads without confusing absence of observation
with absence of defects.

**Applicability.** Operational services and libraries with field data.

**Allowed exceptions.** None for universal claims.

**Review evidence.** Telemetry coverage, detection limits, incident-derived
regressions, and residual uncertainty.

**Enforcement.** Unenforceable: Repository has no deployment or telemetry; misuse is a claim about
wording

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

**Enforcement.** [`examples/src/lib.rs`](../../examples/src/lib.rs) — the inventory test scopes its
reads to the manifest directory

## RUST-DOC-0008-R021 — State evidence limits

**Statement.** Every consequential evidence plan MUST state what each selected
test class proves, what it does not prove, and which risks remain observed only
in production or external systems.

**Intent.** Preserve guarantee honesty.

**Applicability.** Feature plans, reviews, and release audits.

**Allowed exceptions.** Trivial local changes may reference an existing suite
contract.

**Review evidence.** Evidence ledger tied to invariant inventory.

**Enforcement.** [`EVIDENCE.md`](../../EVIDENCE.md) — per-doctrine ledger giving evidence class and
what it does not establish

## RUST-DOC-0008-R022 — Prove the observer looked before accepting absence

**Statement.** An assertion that a condition is absent at runtime MUST establish
that its predicate can observe the condition, through a self-validating
predicate that fails when its subject is missing, a positive control asserted
alongside it, or a paired assertion whose expected count is non-zero.

**Intent.** Separate "the condition was searched for and not found" from "the
search matched nothing", which an empty result reports identically.

**Applicability.** Runtime assertions whose expected result is an empty
collection, a zero count, an unset value, or an uncalled test double, in tests
and in checks that gate a build.

**Allowed exceptions.** An assertion MAY omit the control when the same test
first observes the condition present and then removes it, because the transition
is itself the proof of observation.

**Review evidence.** The control and its assertion, or the non-zero paired case,
shown beside the absence assertion they protect.

**Enforcement.** [`examples/src/lib.rs`](../../examples/src/lib.rs) — the evidence-of-absence trio:
vacuous pass, control, non-zero pair

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

**Enforcement.** Unenforceable: No workload or objective artifact in repo; metric, target, baseline
recorded nowhere

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

**Enforcement.** Unenforceable: No performance claim, environment manifest, or reproduction command
exists in the repo

## RUST-DOC-0009-R003 — Profile before optimizing

**Statement.** Profiling SHOULD precede nontrivial optimization and MUST precede
claims about a dominant bottleneck.

**Intent.** Direct effort to measured cost centers.

**Applicability.** Latency, CPU, allocation, contention, I/O, and size work.

**Allowed exceptions.** Algorithmic complexity defects apparent from complete
input bounds may be corrected without a profile, while still measuring outcome.

**Review evidence.** Flamegraph, trace, allocation profile, system metrics, or
equivalent relevant evidence.

**Enforcement.** Unenforceable: Repo ships no profile, flamegraph, or trace capture to precede
optimization

## RUST-DOC-0009-R004 — Preserve correctness independently

**Statement.** A performance change MUST preserve domain invariants,
error/uncertainty semantics, security properties, and boundary validation, with
correctness evidence independent of the benchmark.

**Intent.** Reject faster incorrect behavior.

**Applicability.** All optimizations.

**Allowed exceptions.** An explicit product tradeoff may change semantics only
as a separately reviewed normative or API change, not as hidden optimization.

**Review evidence.** Invariant-linked tests and guarantee-ledger diff.

**Enforcement.** Unenforceable: No optimization exists; benchmark-independent correctness evidence
has nothing to attach to

## RUST-DOC-0009-R005 — Defend benchmark execution

**Statement.** Benchmark code MUST prevent dead-code elimination, constant
folding, unintended setup measurement, and unrealistic reuse from invalidating
the intended workload.

**Intent.** Ensure measured work corresponds to the claim.

**Applicability.** Microbenchmarks and component benchmarks.

**Allowed exceptions.** None; framework facilities may provide the mechanism.

**Review evidence.** Input generation, black-boxing where appropriate,
setup/measurement separation, and result consumption.

**Enforcement.** Unenforceable: No benchmark code exists; black-boxing and setup separation have
nothing to inspect

## RUST-DOC-0009-R006 — Separate wall-clock and CPU claims

**Statement.** Measurements MUST distinguish wall-clock latency, CPU time, and
aggregate CPU consumption when their interpretations differ.

**Intent.** Prevent waiting and parallel work from being described as reduced
compute cost.

**Applicability.** Async, parallel, I/O-bound, and multi-process workloads.

**Allowed exceptions.** A single-threaded CPU-bound benchmark may report one
measure with its assumption stated.

**Review evidence.** Metric definition and collection method.

**Enforcement.** Unenforceable: No timing measurement exists; wall-clock versus CPU distinction is
unobservable here

## RUST-DOC-0009-R007 — Report distributions

**Statement.** User-visible or service latency claims MUST report appropriate
distributions such as p50, p95, and p99 rather than only arithmetic averages.

**Intent.** Reveal tail behavior and multimodal workloads.

**Applicability.** Requests, queues, storage, and batch completion.

**Allowed exceptions.** Deterministic fixed-cost operations may use a narrow
summary after showing low variance.

**Review evidence.** Sample count, percentile method, confidence or variability,
and outlier policy.

**Enforcement.** Unenforceable: No latency samples or percentile output exist anywhere in the
repository

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

**Enforcement.** Unenforceable: No measurement runs, so warmup and cache state are never documented

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

**Enforcement.** Unenforceable: No allocator-aware measurement, heap profile, or allocation claim
exists in repo

## RUST-DOC-0009-R010 — Scope zero-copy claims

**Statement.** A zero-copy claim MUST identify every copy avoided within the
specified path and the lifetime, pinning, retention, fragmentation, API, and
ownership costs introduced.

**Intent.** Prevent one avoided copy from becoming a broad slogan.

**Applicability.** Parsers, networking, serialization, buffers, and FFI.

**Allowed exceptions.** None for the phrase "zero-copy."

**Review evidence.** Data-flow diagram, measured copy/allocation evidence, and
non-guarantees.

**Enforcement.** Unenforceable: No zero-copy claim or data-flow evidence appears in any shipped
crate

## RUST-DOC-0009-R011 — Do not equate async with speedup

**Statement.** Async concurrency MUST NOT be described as parallel CPU speedup
without evidence of parallel execution and a workload that benefits.

**Intent.** Distinguish overlap of waiting from reduced compute time.

**Applicability.** Runtime migrations, fan-out, and worker design.

**Allowed exceptions.** None for the claim; async may still improve resource
efficiency or concurrent latency.

**Review evidence.** Executor configuration, CPU utilization, throughput,
latency, and contention.

**Enforcement.** Unenforceable: No async runtime, executor trace, or CPU-utilization evidence ships
in examples

## RUST-DOC-0009-R012 — Make throughput/latency tradeoffs explicit

**Statement.** Batching, buffering, pipelining, and concurrency changes MUST
report both throughput and relevant latency/queue consequences.

**Intent.** Prevent aggregate gains from hiding worse tails or freshness.

**Applicability.** Brokers, databases, serializers, and service queues.

**Allowed exceptions.** Offline throughput-only jobs may state that latency has
no objective while still bounding resource use.

**Review evidence.** Batch/concurrency sweep and distribution results.

**Enforcement.** Unenforceable: No batching or concurrency sweep results exist pairing throughput
with latency

## RUST-DOC-0009-R013 — Measure contention and backpressure

**Statement.** Concurrent performance analysis MUST include queue depth, wait
time, saturation, lock or permit contention, rejection, and downstream load
where relevant.

**Intent.** Reveal whether local throughput shifts cost elsewhere.

**Applicability.** Shared state, pools, channels, and fan-out.

**Allowed exceptions.** Pure independent parallel work may document absence of
shared contention.

**Review evidence.** Contention profile, load curve, and overload behavior.

**Enforcement.** Unenforceable: No queue, lock, or backpressure instrumentation; examples avoid
concurrency entirely

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

**Enforcement.** Unenforceable: No trace or component budget; repo integrates no database or network

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

**Enforcement.** Unenforceable: No measured clone removal; examples carry no allocation profile or
ownership analysis

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

**Enforcement.** Unenforceable: unsafe-evidence crate makes no performance claim and has no
safe-baseline benchmark

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

**Enforcement.** Unenforceable: CI defines no benchmark, size, or allocation regression gate; no
variance history

## RUST-DOC-0009-R018 — Do not generalize microbenchmarks

**Statement.** Microbenchmark results MUST NOT be generalized to end-to-end
performance without evidence connecting the measured operation to overall
workload contribution.

**Intent.** Prevent large local ratios from masking tiny system impact.

**Applicability.** Library and application optimization claims.

**Allowed exceptions.** A microbenchmark may establish the cost of the exact
isolated primitive it measures.

**Review evidence.** Profile share, integrated benchmark, or component budget.

**Enforcement.** Unenforceable: No microbenchmark exists to generalize; no profile share or
integrated benchmark

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

**Enforcement.** Unenforceable: No build-timing, binary-size, or monomorphization measurement is
collected or retained

## RUST-DOC-0009-R020 — Retain reproducible evidence

**Statement.** Accepted performance decisions MUST retain commands, commits,
configuration, result summaries, and raw-data location or format sufficient to
repeat or challenge the result.

**Intent.** Make optimization decisions durable and auditable.

**Applicability.** Merged performance changes and release claims.

**Allowed exceptions.** Sensitive production traces may be retained in
controlled storage with a sanitized reproducible summary.

**Review evidence.** Benchmark record and provenance.

**Enforcement.** Unenforceable: No retained benchmark record, raw data, or provenance; EVIDENCE.md
states none

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

**Enforcement.** Unenforceable: No check classifies claims; classification exists only in review
prose

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
[`foundations/complexity-budget.md`](../../foundations/complexity-budget.md) MAY remain prose-carried when the
assessment, its owner, and the residual risk are recorded on the terms of RUST-DOC-0011-R020.

**Review evidence.** The enforcing artifact, or the recorded assessment showing that no available
mechanism enforces the obligation proportionately.

**Enforcement.** [`tools/doctrine-lint/src/main.rs`](../../tools/doctrine-lint/src/main.rs) —
check_rule_enforcement

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

**Enforcement.** Unenforceable: Nothing detects prose being cited as authority over the enforcing
artifact

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

**Enforcement.** [`tools/doctrine-lint/src/main.rs`](../../tools/doctrine-lint/src/main.rs) —
check_doctrine_index

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

**Enforcement.** [`tools/bundle-agent-context/src/main.rs`](../../tools/bundle-agent-context/src/main.rs)
— the drift check, with check_generated_files

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

**Enforcement.** Unenforceable: Registry stores membership only; no check judges whether a record
should exist

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

**Enforcement.** [`tools/doctrine-lint/src/main.rs`](../../tools/doctrine-lint/src/main.rs) —
check_active_record

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

**Enforcement.** Unenforceable: Only a non-empty scope field is checked; narrowness and exclusions
are not

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

**Enforcement.** [`tools/doctrine-lint/src/main.rs`](../../tools/doctrine-lint/src/main.rs) —
check_archived_record and status agreement

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

**Enforcement.** Unenforceable: No mechanism observes record citations in review comments or agent
reasoning

## RUST-DOC-0011-R011 — Retire an implemented proposal from operational authority

**Statement.** A proposal governs review and acceptance before implementation. After
implementation the accepted proposal MUST be treated as decision history, and MUST NOT be
maintained or cited as a current specification of behavior that canonical doctrine and executable
artifacts now carry.

**Intent.** Keep an accepted RFC from becoming a competing specification that future maintainers
must reconcile against current behavior.

**Applicability.** Accepted RFCs and equivalent proposal documents after their implementation has
landed. This rule does not weaken the RFC obligations stated in [`AGENTS.md`](../../AGENTS.md) and
[`rfcs/README.md`](../../rfcs/README.md), which govern the change process rather than the resulting contract.

**Allowed exceptions.** A proposal MAY remain cited for its decision, its date, its owners, its
accepted conditions, and its recorded alternatives, which are rationale rather than
specification.

**Review evidence.** The canonical doctrine and executable artifacts the proposal points to, and
the absence of a normative obligation stated only in the proposal.

**Enforcement.** Unenforceable: No check detects an accepted RFC being cited as current
specification

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

**Enforcement.** Unenforceable: No check separates irrecoverable rationale from restatement of
enforced topology

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

**Enforcement.** Unenforceable: No mechanism distinguishes a recorded reason from an inferred one

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

**Enforcement.** Unenforceable: No check identifies external facts or verifies the named
authoritative system

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

**Enforcement.** Unenforceable: No check links a compatibility promise to a test, schema, or fixture

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

**Enforcement.** Unenforceable: No lint judges domain naming, effect disclosure, capability width,
or erasure timing

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

**Enforcement.** [`tools/doctrine-lint/src/main.rs`](../../tools/doctrine-lint/src/main.rs) —
check_validation_sequence_copies

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

**Enforcement.** [`tools/doctrine-lint/src/main.rs`](../../tools/doctrine-lint/src/main.rs) —
check_agent_packs_exclude_archive

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

**Enforcement.** [`tools/doctrine-lint/src/main.rs`](../../tools/doctrine-lint/src/main.rs) —
check_normative_scope

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

**Enforcement.** Unenforceable: No schema or check requires the five exception terms anywhere

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

Non-guarantee and residual-risk statements are authoritative for what a design deliberately does not prove and who
accepted the remainder, on the terms [`foundations/guarantee-honesty.md`](../../foundations/guarantee-honesty.md)
states.

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

Where an obligation crosses a boundary, the enforcing mechanism changes and the authority moves with it. A wire contract
is enforced by its canonical encoder, decoder, schema, and compatibility suite under
[`boundaries/serde.md`](../../boundaries/serde.md) and [`boundaries/http-and-rpc.md`](../../boundaries/http-and-rpc.md).
A persistence invariant is enforced by schema constraints, checked decoding, and transaction predicates under
RUST-DOC-0005 and [`boundaries/database-decoding.md`](../../boundaries/database-decoding.md). An operational policy is
enforced by deployable configuration and machine-checked manifests under
[`boundaries/configuration.md`](../../boundaries/configuration.md). A claim that crosses into another system's ownership
becomes an external claim governed by RUST-DOC-0011-R014.

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
