<!--
GENERATED FILE. DO NOT EDIT DIRECTLY.
Canonical sources live under /foundations, /doctrines, /patterns,
 /boundaries, /reviews, and /agents.
-->

# Planner agent doctrine pack

Produce invariant-first designs, boundary maps, and evidence plans before implementation.

## Assembly

Ceiling `focused`, declared for the `planner` pack in `manifest/agents.yaml`. A section annotated above that ceiling is withheld here. Nothing was withheld at this ceiling.

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

## Source: `agents/planner.md`

# Planner overlay

## Purpose

Transform requirements into a reviewable invariant-first design without
prematurely choosing Rust mechanisms. The planner produces artifacts that an
implementer can follow and a reviewer can challenge. Read [`shared.md`](../../agents/shared.md),
foundations on invariants, trust boundaries, guarantee honesty, and complexity,
then the doctrine packages selected for the domain.

## Required outputs

### Domain vocabulary

Define each business value, actor, resource, state, effect, and evidence level.
Split ambiguous terms. For example, distinguish raw email input, syntactically
accepted address, verifier-confirmed ownership, and current deliverability.
Avoid importing implementation names before meaning is stable.

### Invariant inventory

For every consequential rule record:

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

Separate invariant, precondition, postcondition, policy, assumption,
observation, and desired outcome. Identify which facts are local and stable,
which cross entities, and which can change externally.

### Trust-boundary map

Show every HTTP/RPC request, message, row, serialized value, file,
configuration source, external service, UI input, and FFI call. For each, name
raw representation, parser/limits, structural DTO, validation, trusted
constructor, failure mapping, version behavior, sensitive-data policy, and
remaining uncertainty. Include administrative, migration, cache, replay, and
restore paths.

### State graph

List runtime states, associated evidence, legal edges, actor/authority, failure
edges, cancellation, persistence, and unknown outcomes. Separate independent
dimensions. Show which transitions one local owner controls and which are
observations of external reality. Do not prescribe typestate merely because
Rust supports it.

### Authority map

Identify principals, authenticators, authorizers, capabilities, privileged
constructors, resource scope, cloning, transfer, expiry, revocation, and audit.
Record where a Rust type improves local least privilege and where current
runtime policy must still be consulted.

### External-effect and uncertainty inventory

For every effect name stable operation identity, dispatch boundary, possible
timeout ambiguity, idempotency scope/binding/retention, retry classification,
duplicate behavior, ordering scope, durable intent, reconciliation evidence,
owner, age target, compensation, and audit causality.

### Persistence representation

Define raw row/document/event models, stable tags, versions, constraints,
transaction boundaries, optimistic concurrency, invalid-history behavior,
migration compatibility, and outbox/inbox coordination. State which persisted
fact is an observation rather than immutable current truth.

### Complexity-budget decision

Compare enum, newtype, validated wrapper, consuming transition, typestate,
capability, runtime service, and plain code as relevant. Include state/transition
count, public API, serialization, dynamic dispatch, async complexity,
diagnostics, compile time, monomorphization, binary size, team familiarity,
migration, and misuse consequence. Recommend the simplest sufficient mechanism.

### Evidence plan

Map invariants to compiler rejection, unit, property, compile-fail, integration,
contract, concurrency, fault injection, model checking, unsafe-specific tools,
telemetry, and incident signals. State what each layer does not prove.

## Planning workflow

1. Read requirements and repository context completely.
2. Produce vocabulary and invariant inventory.
3. Draw boundary, state, authority, persistence, and effect maps.
4. Classify each invariant before selecting representation.
5. Develop at least one simpler alternative for type-heavy choices.
6. Create guarantee ledger entries for proposed trusted types and effects.
7. Apply [`../reviews/pre-implementation.md`](../../reviews/pre-implementation.md).
8. Resolve fails or document governance disposition.
9. Hand off exact artifacts and rule IDs to implementation.

## Obligation placement

Before proposing any document, decide where the obligation lives. Under RUST-DOC-0011:

- classify each claim as enforced local truth, external or durable fact, rationale,
  non-guarantee or accepted risk, or change authority, and name one authority for each;
- prefer an available mechanism to a description; an ordering, a construction restriction, a
  capability boundary, or a negative guarantee that a type, schema, manifest, or test can enforce
  is placed there, not in the plan;
- count the maintained representations the plan would leave behind, and remove those that are
  neither authoritative, generated, nor irrecoverable rationale;
- propose a decision record only after that assessment fails, and only for an external mandate,
  an irreversible or externally expensive commitment, a rejected alternative whose rejection
  depends on evidence the implementation does not carry, a decision no single system owns, an
  accepted residual risk, or a compatibility obligation from shipped behavior;
- state, for every record proposed, the exact fact no artifact can carry, the owner, the
  revalidation trigger, the obsolescence condition, and the artifacts that stay authoritative.

That a decision is large, was debated, or might be forgotten is not a justification for a record.
Record the outcome even when it is that no document is added, so a later reader can tell the
assessment happened.

## Forbidden planning shortcuts

Do not make "use typestate" the first requirement. Do not say "validate at the
edge" without naming every edge and bypass. Do not assume a database transaction
coordinates a message or payment. Do not call a timeout failure. Do not use
`Arc<Mutex<_>>` as the ownership model. Do not select an error crate before
classifying domain outcomes. Do not propose unsafe optimization without a
workload and profile plan.

## Evidence and escalation

Cite primary protocol/database/runtime sources for changing or product-specific
facts. Mark inference. Escalate missing product policy when it changes legal
state, authority, retry, retention, compatibility, or user behavior. Present
options with guarantee, cost, migration, and residual risk rather than asking an
ungrounded question.

## Completion contract

Planning is complete only when implementers can identify every protected
constructor, state transition, boundary conversion, external effect, durable
record, authority path, and required test. The plan includes non-goals,
complexity decision, initial guarantee ledger, review result, and real
limitations. Syntax sketches may illustrate, but artifacts remain
contract-shaped rather than implementation-first.

## Handoff and change control

The handoff names canonical paths to change, expected new public API, schema or
wire compatibility constraints, generated outputs affected, exact local
validation commands, and focused review procedures. Assign unresolved
assumptions to an owner and prevent implementation from treating them as facts.

If implementation discovers a new state, alternate writer, cancellation edge,
privileged constructor, migration constraint, or ambiguous external outcome,
the planner updates the corresponding artifact and guarantee ledger. A changed
mechanism is acceptable when it preserves approved intent and receives review;
a weakened invariant, broader authority, or new escape hatch returns through
RFC or product-decision governance rather than entering as incidental code.

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

**Enforcement.** Unenforceable: No failure-inventory table in repo; inventory is a
pre-implementation design act

## RUST-DOC-0002-R002 — Use structured library errors

**Statement.** Library APIs MUST NOT use opaque string errors as their primary public contract
when callers can respond differently to failure categories.

**Intent.** Preserve machine-actionable meaning independently of human wording.

**Applicability.** Reusable crates and module boundaries with multiple operational outcomes.

**Allowed exceptions.** An opaque non-exhaustive error object MAY be used when no stable
category can be promised, provided callers have documented inspection or reporting semantics.

**Review evidence.** Public enum or equivalent typed interface, match examples, and stability
policy.

**Enforcement.** [`examples/boundary-validation/src/lib.rs`](../../examples/boundary-validation/src/lib.rs)
— typed ContactError; tests match variants

## RUST-DOC-0002-R003 — Distinguish actionable categories

**Statement.** Validation failure, policy rejection, authorization denial, conflict,
cancellation, timeout, resource exhaustion, local I/O failure, and indeterminate outcome MUST
remain distinguishable when they require different caller or operator action.

**Intent.** Prevent unsafe retry, misleading user messages, and loss of reconciliation.

**Applicability.** Any operation where at least two listed outcomes differ operationally.

**Allowed exceptions.** Categories MAY be coarsened at an outer recipient boundary when the
recipient cannot act differently and observability retains safe internal detail.

**Review evidence.** Outcome-to-action matrix and conversion tests.

**Enforcement.** [`examples/distributed-outcomes/src/lib.rs`](../../examples/distributed-outcomes/src/lib.rs)
— retry matrix keeps the three cases distinct

## RUST-DOC-0002-R004 — Preserve sources

**Statement.** Error wrapping and conversion SHOULD preserve the originating error through a
source chain when doing so is safe and useful for diagnosis.

**Intent.** Retain causal evidence while adding domain context.

**Applicability.** I/O, parsing, serialization, database, protocol, and dependency errors.

**Allowed exceptions.** Security, privacy, compatibility, or cross-process boundaries MAY
replace the exposed source with a sanitized internal correlation record.

**Review evidence.** `source()` chain tests or report inspection, plus redaction review.

**Enforcement.** [`examples/boundary-validation/src/lib.rs`](../../examples/boundary-validation/src/lib.rs)
— Error::source forwards the wrapped cause

## RUST-DOC-0002-R005 — Add context without erasing category

**Statement.** Application context SHOULD identify the failed operation and relevant
non-sensitive identity without replacing machine-actionable categories with formatted text.

**Intent.** Make diagnosis specific while retaining programmatic action.

**Applicability.** Layered application operations, job processing, and boundary adapters.

**Allowed exceptions.** A terminal application boundary MAY use an opaque report after all
control decisions have been made.

**Review evidence.** Context chain, correlation ID, structured fields, and user-facing
redaction.

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— stage context added while keeping the typed cause

## RUST-DOC-0002-R006 — State recoverability

**Statement.** Recoverability MUST be explicit at the decision point; callers MUST NOT infer
that every `Err` leaves state unchanged or reusable.

**Intent.** Account for partial mutation, consumed authority, cancellation, ambiguous commit,
and external side effects.

**Applicability.** Stateful, consuming, transactional, asynchronous, and external operations.

**Allowed exceptions.** Pure functions MAY document the conventional no-side-effect error
contract once at module level.

**Review evidence.** Post-error state contract, returned recovery value or token, and tests.

**Enforcement.** Unenforceable: No operation returns a recovery value with Err; post-error state
undocumented

## RUST-DOC-0002-R007 — Type retry guidance

**Statement.** Retryability MUST NOT be inferred solely from a generic transport class,
status family, or error string. Retry policy MUST account for operation semantics,
idempotency, attempt budget, backoff, and external commitment.

**Intent.** Prevent duplicates, retry storms, and repeated permanent rejection.

**Applicability.** Network, database, broker, and other transient-looking errors.

**Allowed exceptions.** None where the operation can cause a consequential effect.

**Review evidence.** Typed retry decision, idempotency analysis, budget, jitter, and fault
tests.

**Enforcement.** [`examples/distributed-outcomes/src/lib.rs`](../../examples/distributed-outcomes/src/lib.rs)
— retry typed by observation and idempotency

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

**Enforcement.** [`examples/distributed-outcomes/src/lib.rs`](../../examples/distributed-outcomes/src/lib.rs)
— unknown outcome never becomes rejection

## RUST-DOC-0002-R009 — Bound panic to programmer faults

**Statement.** Panics MUST be reserved for violated internal invariants or unrecoverable
programmer errors, not expected external, user, configuration, or data failure.

**Intent.** Keep expected failure in the declared control-flow and cleanup model.

**Applicability.** Production library and application paths.

**Allowed exceptions.** Process startup MAY deliberately abort on invalid required
configuration after producing a clear sanitized diagnostic, when continued operation is
unsafe and no caller can recover.

**Review evidence.** Panic-site inventory, unwind/abort policy, and boundary failure tests.

**Enforcement.** [`examples/boundary-validation/src/lib.rs`](../../examples/boundary-validation/src/lib.rs)
— malformed input returns typed Err, not panic

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

**Enforcement.**
[`tools/bundle-agent-context/src/main.rs`](../../tools/bundle-agent-context/src/main.rs) — two
production `expect` calls whose message states the invariant that makes failure a defect

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

**Enforcement.** Unenforceable: No security-denial mapping, audit fields, or conversion test exists
in repo

## RUST-DOC-0002-R012 — Prevent secret disclosure

**Statement.** Error display, debug, source chains, protocol responses, logs, and telemetry
MUST NOT expose secrets or sensitive internal data to unauthorized recipients.

**Intent.** Ensure diagnosis does not create a confidentiality breach.

**Applicability.** Credentials, tokens, personal data, SQL, paths, provider payloads, and
security decisions.

**Allowed exceptions.** Restricted forensic storage MAY retain necessary evidence under
explicit access and retention policy.

**Review evidence.** Recipient map, redaction tests, debug implementations, and sample logs.

**Enforcement.** Unenforceable: No redaction test, sanitized Debug, or recipient map anywhere in
repo

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

**Enforcement.** Unenforceable: No non_exhaustive error type, semver analysis, or migration notes
exist

## RUST-DOC-0002-R014 — Log once at an ownership boundary

**Statement.** Errors SHOULD be logged by the layer that owns the final handling decision,
rather than at every propagation layer.

**Intent.** Prevent duplicate events, contradictory severity, and noisy alerts.

**Applicability.** Layered services, jobs, and request handlers.

**Allowed exceptions.** A lower layer MAY emit a distinct metric or trace event when it adds
unique timing or state evidence and correlation prevents double counting.

**Review evidence.** Error path trace, log ownership, event IDs, and alert mapping.

**Enforcement.** Unenforceable: Workspace has no logging or tracing dependency; no error-path log
ownership shown

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

**Enforcement.** Unenforceable: No artifact enumerates owners; authority-map completeness is judged
against the design

## RUST-DOC-0003-R002 — Encode exclusive authority with ownership

**Statement.** Ownership SHOULD express exclusive authority when only one actor may legally
exercise or complete an operation.

**Intent.** Prevent duplicated commit, shutdown, claim, or single-use token consumption.

**Applicability.** Exclusive domain actions with natural transfer or consumption.

**Allowed exceptions.** Durable external coordination MAY require runtime exclusivity when
multiple processes or persisted actors participate.

**Review evidence.** Non-cloneable type, consuming operation, and concurrency or compile-fail
tests.

**Enforcement.** [`reuse_consumed_transaction.rs`](../../examples/compile-fail/ui/reuse_consumed_transaction.rs)
— a consumed handle cannot be reused

## RUST-DOC-0003-R003 — Bound borrowed authority

**Statement.** A borrowed reference MUST NOT accidentally grant mutation, ownership transfer,
serialization, or authority beyond the documented borrow scope.

**Intent.** Keep read access from becoming lasting or privileged access.

**Applicability.** References, guards, views, callbacks, and borrowed service handles.

**Allowed exceptions.** Interior mutability MAY permit mutation when that aliasing contract is
the explicit design and synchronization is correct.

**Review evidence.** Method receiver audit, returned-lifetime analysis, and mutation tests.

**Enforcement.** Unenforceable: No borrowed guard or view in workspace; granted rights judged from
receiver and lifetime

## RUST-DOC-0003-R004 — Restrict capability issuance and surface

**Statement.** Capability constructors MUST be restricted to authorized issuers, and a
capability MUST expose only the operations and scope it grants.

**Intent.** Make capabilities hard to forge and consistent with least privilege.

**Applicability.** Authorization, verification proof, shutdown, transaction, secret, and
resource capabilities.

**Allowed exceptions.** None for security-relevant authority.

**Review evidence.** Visibility, fields, re-exports, operation methods, and issuer tests.

**Enforcement.**
[`construct_verified_email_directly.rs`](../../examples/compile-fail/ui/construct_verified_email_directly.rs)
— only the issuer may construct the proof

## RUST-DOC-0003-R005 — Justify cloning authority

**Statement.** Cloning or copying an authority-bearing value MUST require explicit
justification consistent with exclusivity, use count, scope, and revocation.

**Intent.** Prevent convenience derives from amplifying authority.

**Applicability.** Capabilities, tokens, guards, handles, and credentials.

**Allowed exceptions.** A shareable read capability MAY be cloneable when duplication is part
of the documented authority model.

**Review evidence.** `Clone`/`Copy` audit, clone semantics, and duplicate-use tests.

**Enforcement.** [`clone_stage_to_duplicate.rs`](../../examples/compile-fail/ui/clone_stage_to_duplicate.rs)
— a single-use stage has no Clone

## RUST-DOC-0003-R006 — Define transfer and revocation

**Statement.** Tokens, sessions, transaction guards, leases, and resource handles MUST define
transfer, expiry, revocation, and post-revocation behavior when those concepts apply.

**Intent.** Prevent local possession from being treated as perpetual external permission.

**Applicability.** Mutable authority, leased resources, sessions, and cross-task custody.

**Allowed exceptions.** Irrevocable process-local values MAY state that revocation is not part
of their contract.

**Review evidence.** State transitions, clocks or versions, revocation check, and stale-use
tests.

**Enforcement.** Unenforceable: No lease, expiry, or revocation type in workspace; applicability
judged per design

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

**Enforcement.** Unenforceable: Workspace has no Drop impl for resources; drop-claim wording judged
per failure mode

## RUST-DOC-0003-R008 — Protect secret-bearing types

**Statement.** Secret-bearing types MUST minimize accidental `Debug`, `Display`, cloning,
serialization, logging, and long-lived borrowing; exposure MUST be explicit and scoped.

**Intent.** Reduce unintended copies and recipient leakage.

**Applicability.** Passwords, tokens, private keys, session secrets, and decrypted material.

**Allowed exceptions.** None for ordinary formatting. Controlled serialization MAY be
required for a protected secret store under a distinct API.

**Review evidence.** Trait implementation audit, redaction tests, exposure call sites, and
storage contract.

**Enforcement.** Unenforceable: No secret type in workspace; newtype Debug redaction is not
secret-handling evidence

## RUST-DOC-0003-R009 — Limit zeroization claims

**Statement.** Zeroization claims MUST state the exact owned buffer cleared and MUST NOT imply
removal of compiler-created copies, allocator remnants, swap, logs, external stores, or prior
serialization unless those paths are controlled and evidenced.

**Intent.** Prevent a local overwrite mechanism from becoming a universal secrecy guarantee.

**Applicability.** Secret memory and cryptographic material.

**Allowed exceptions.** None to claim accuracy.

**Review evidence.** Ownership and copy analysis, drop path, memory-locking policy where used,
and explicit non-guarantees.

**Enforcement.** Unenforceable: No zeroization code or claim exists; uncovered paths judged per
platform and allocator

## RUST-DOC-0003-R010 — Design before `Arc<Mutex<T>>`

**Statement.** `Arc<Mutex<T>>` MUST NOT be the default substitute for identifying ownership,
task responsibility, mutation protocol, lock scope, and shutdown.

**Intent.** Avoid shared mutable bags that compile but hide contention, deadlock, and authority.

**Applicability.** Concurrent shared state and service handles.

**Allowed exceptions.** It MAY be the simplest correct mechanism after the ownership and
synchronization contract is documented.

**Review evidence.** Owner, lock invariant, contention and poisoning policy, alternatives, and
tests.

**Enforcement.** Unenforceable: No concurrency code; whether ownership preceded lock choice is
unobservable from code

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

**Enforcement.** Unenforceable: Only test-harness RefCell exists; necessity of an aliasing contract
judged per design

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

**Enforcement.** Unenforceable: Examples declare no named lifetimes; ornamental versus real referent
judged from signatures

## RUST-DOC-0003-R013 — Define cross-task ownership

**Statement.** Transfer of authority or resources across tasks MUST identify the new owner,
completion signal, cancellation behavior, shutdown responsibility, and behavior if the task
is dropped or panics.

**Intent.** Prevent detached custody and resources with no accountable closer.

**Applicability.** Spawned tasks, worker actors, channels carrying handles, and supervisors.

**Allowed exceptions.** Truly process-lifetime services MAY be owned by the process supervisor.

**Review evidence.** Task tree, join/abort contract, channel closure, and shutdown tests.

**Enforcement.** Unenforceable: No spawned tasks or async; join, cancel, shutdown ownership judged
per design

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

**Enforcement.** Unenforceable: No revocation backend; freshness window and revalidation boundary
are policy judgments

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

**Enforcement.** [`examples/distributed-outcomes/src/lib.rs`](../../examples/distributed-outcomes/src/lib.rs)
— ExecutionAmbiguous routes to Unknown/reconcile

## RUST-DOC-0006-R002 — Model operationally distinct outcomes

**Statement.** Outcome types MUST distinguish confirmed success, confirmed
rejection, local failure before dispatch, and unknown outcome when callers
require different recovery.

**Intent.** Prevent transport symptoms from erasing domain knowledge.

**Applicability.** Consequential external operations.

**Allowed exceptions.** Categories may combine when no caller action, audit
meaning, security consequence, or reconciliation path differs.

**Review evidence.** Outcome decision table and exhaustive caller handling.

**Enforcement.** [`examples/distributed-outcomes/src/lib.rs`](../../examples/distributed-outcomes/src/lib.rs)
— Confirmed/Rejected/Unknown plus NotDispatched

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

**Enforcement.** [`examples/distributed-outcomes/src/lib.rs`](../../examples/distributed-outcomes/src/lib.rs)
— ReconciliationToken carries OperationId

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

**Enforcement.** [`examples/distributed-outcomes/src/lib.rs`](../../examples/distributed-outcomes/src/lib.rs)
— decide_retry maps observation to retry posture

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

**Enforcement.** Unenforceable: No artifact defines key scope, uniqueness, retention, payload
binding, or replay

## RUST-DOC-0006-R006 — Reuse operation identity across attempts

**Statement.** Retries of one logical operation MUST reuse its operation and
idempotency identity. A new identity MUST mean a new requested effect.

**Intent.** Allow receivers and reconcilers to distinguish replay from new
intent.

**Applicability.** External API requests, published commands, and repair tools.

**Allowed exceptions.** A protocol-mandated new transport attempt identifier may
be added, but it MUST remain correlated to the stable logical operation.

**Review evidence.** Identity lifecycle and attempt log.

**Enforcement.** [`examples/distributed-outcomes/src/lib.rs`](../../examples/distributed-outcomes/src/lib.rs)
— retries reuse the same operation and idempotency key

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

**Enforcement.** Unenforceable: No consumer, broker, or duplicate-delivery path exists in any
example crate

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

**Enforcement.** Unenforceable: No durable store or transaction exists; examples avoid database and
broker dependencies

## RUST-DOC-0006-R009 — State ordering scope

**Statement.** Ordering claims MUST identify key or partition, producer set,
consumer concurrency, retry behavior, failover behavior, and observation point.

**Intent.** Prevent partition-local or producer-local order from becoming a
false global guarantee.

**Applicability.** Brokers, streams, event logs, RPC sequencing, and replication.

**Allowed exceptions.** None when business behavior relies on order.

**Review evidence.** Ordering contract and tests for retries, multiple
producers, and failover.

**Enforcement.** Unenforceable: No ordering, partition, producer-set, or consumer-concurrency model
exists

## RUST-DOC-0006-R010 — Qualify exactly-once claims

**Statement.** Any "exactly once" claim MUST identify the precise boundary,
identity, transactional mechanism, failure assumptions, retention, and effects
included. It MUST NOT imply exactly-once behavior beyond that boundary.

**Intent.** Replace a broad slogan with an auditable scoped guarantee.

**Applicability.** Messaging, stream processing, payments, jobs, and APIs.

**Allowed exceptions.** None.

**Review evidence.** Guarantee ledger, protocol documentation, duplicate tests,
and excluded effects.

**Enforcement.** Unenforceable: No exactly-once mechanism or transactional boundary is implemented
anywhere

## RUST-DOC-0006-R011 — Coordinate acknowledgement with effect

**Statement.** A consumer MUST define the order and atomic relationship among
effect execution, durable progress, and acknowledgement.

**Intent.** Make the duplicate-versus-loss tradeoff visible.

**Applicability.** Message and job consumers.

**Allowed exceptions.** Best-effort consumers may acknowledge early only when
loss is accepted and measured.

**Review evidence.** Crash-point matrix and tests before and after each durable
step.

**Enforcement.** Unenforceable: No consumer, acknowledgement, or durable-progress sequencing is
implemented

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

**Enforcement.** Unenforceable: No saga, compensation, or forward/compensation pair exists in any
example crate

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

**Enforcement.** Unenforceable: No timestamp, version, or freshness field exists on any outcome or
observation

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

**Enforcement.** Unenforceable: No lease, fencing token, clock source, or concurrency backend exists

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

**Enforcement.** Unenforceable: No retry loop, attempt budget, backoff, queue, or escalation is
implemented

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

**Enforcement.** Unenforceable: No audit event schema, attempt log, trace propagation, or request
fingerprint exists

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

**Enforcement.** Unenforceable: No field classification, redaction, or sensitive-data handling
exists in any example

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

**Enforcement.** Unenforceable: Tests inject no loss, delay, duplication, reordering, or crash
points

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

**Enforcement.** Unenforceable: Needs stated unknown-duration, reconciliation owner, operator
guidance; none carried

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

**Enforcement.** [`examples/src/lib.rs`](../../examples/src/lib.rs) — every read in the inventory
test derives from `CARGO_MANIFEST_DIR` rather than the working directory

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

**Enforcement.** [`improved.md`](../../case-studies/registration-onboarding/improved.md) — stage
graph plus per-stage evidence rows

## RUST-DOC-0010-R002 — Name each stage by the fact it proves

**Statement.** A stage type MUST be named for the fact its construction establishes, and MUST
NOT be named for its position, its processing step, or a version counter.

**Intent.** Keep the stage graph readable as a sequence of proofs rather than an ordering of
implementation steps.

**Applicability.** Every named stage type and type-level state marker in a staged protocol.

**Allowed exceptions.** None. A stage whose proven fact cannot be named is evidence that the
boundary is not a real one.

**Review evidence.** Stage names, their documented guarantees, and the guarantee ledger.

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— stages named for proven facts, not positions

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

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— each capability declares its successor as an associated type

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

**Enforcement.** Unenforceable: No file records bound-change history; a relaxation is visible only
in review records

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

**Enforcement.** [`reuse_consumed_stage.rs`](../../examples/compile-fail/ui/reuse_consumed_stage.rs)
— a consumed stage cannot be reused or cloned

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

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— canonical values survive every transition

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

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— the infallible signature cites this rule; four per-stage error types

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

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— a branch outcome over distinct successor types

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

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— revision re-enters at the first stage; abandonment is terminal

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

**Enforcement.** [`forge_stage_evidence.rs`](../../examples/compile-fail/ui/forge_stage_evidence.rs)
— a private field blocks forged stage evidence

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

**Enforcement.** Unenforceable: Example crate has no trusted construction path; inventory
completeness has no checkable source

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

**Enforcement.** Unenforceable: Stage-count proportionality and budget justification appear in no
file for this protocol

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

**Enforcement.** [`improved.md`](../../case-studies/registration-onboarding/improved.md) — all four
transitions disclosed write-free

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

**Enforcement.** Unenforceable: No durable store or competing-writer test exists; EVIDENCE.md
records this as unevidenced

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

**Enforcement.** Unenforceable: No persisted model or restoration path in repo; EVIDENCE.md records
this as unevidenced

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

**Enforcement.** Unenforceable: Example has no async transition; no cancellation table, retry
identity, or fault test

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

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— origin erasure happens once, at the persistence boundary

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

**Enforcement.** [`examples/compile-fail/tests/ui.rs`](../../examples/compile-fail/tests/ui.rs) —
trybuild runs the four staged-protocol rejections

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

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— contract assertions derive successors from the trait alone

## RUST-DOC-0010-R020 — Record a guarantee ledger row per stage

**Statement.** Each stage MUST have a guarantee ledger row stating the claim it establishes, the
transition that establishes it, how its construction is protected, how boundary decoding
preserves it, its escape hatches, what it does not prove, and the residual runtime risk.

**Intent.** Keep the protocol's honesty auditable at the granularity at which its claims are
made.

**Applicability.** Every stage type and every piece of stage evidence.

**Allowed exceptions.** None.

**Review evidence.** The completed ledger and its agreement with the stage definitions.

**Enforcement.** [`improved.md`](../../case-studies/registration-onboarding/improved.md) — a
guarantee ledger row per stage

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

**Enforcement.** [`doctrines/0010-staged-protocols/glossary.md`](../../doctrines/0010-staged-protocols/glossary.md)
— the local term is marked local and attributed

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

**Enforcement.** Unenforceable: No per-claim authority classification exists for this protocol; only
gates state it

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
