<!--
GENERATED FILE. DO NOT EDIT DIRECTLY.
Canonical sources live under /foundations, /doctrines, /patterns,
 /boundaries, /reviews, and /agents.
-->

# Planner agent doctrine pack

Produce invariant-first designs, boundary maps, and evidence plans before implementation.

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

## Source: `agents/planner.md`

# Planner overlay

## Purpose

Transform requirements into a reviewable invariant-first design without
prematurely choosing Rust mechanisms. The planner produces artifacts that an
implementer can follow and a reviewer can challenge. Read `shared.md`,
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
7. Apply [`../reviews/pre-implementation.md`](../reviews/pre-implementation.md).
8. Resolve fails or document governance disposition.
9. Hand off exact artifacts and rule IDs to implementation.

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

“The invoice is correct” is not a useful invariant. “A paid invoice carries a receipt issued
for that invoice” is a state invariant. “Capture can occur only after authorization” is a
transition invariant. “Only a capability created by the authorization service permits
capture” adds an authority invariant. Each can receive a different enforcement mechanism and
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

A distributed invariant spans independent failure domains, such as “at most one capture is
accepted for an idempotency key” or “every committed outbox record is eventually attempted.”
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

Consider `Connection<Open>`. “The local connect transition returned success” is historical
evidence encoded by the state. “The remote peer is reachable now” is a mutable observation,
not a lasting invariant of the value. “`send` is called only after local connection” is a
sequencing invariant. “The next send succeeds” is a desired outcome and must remain fallible.

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

## What “untrusted” means

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

The contract names an owner. “Serde validates it” is not enough when the derive writes private
fields directly. “The database enforces it” is not enough when replicas, old rows, or migration
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
failure is represented, and evidence covers violation. “Compile time” is not automatically
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

**Review evidence.** trait implementation audit, redaction tests, exposure call sites, and
storage contract.

## RUST-DOC-0003-R009 — Limit zeroization claims

**Statement.** Zeroization claims MUST state the exact owned buffer cleared and MUST NOT imply
removal of compiler-created copies, allocator remnants, swap, logs, external stores, or prior
serialization unless those paths are controlled and evidenced.

**Intent.** Prevent a local overwrite mechanism from becoming a universal secrecy guarantee.

**Applicability.** Secret memory and cryptographic material.

**Allowed exceptions.** None to claim accuracy.

**Review evidence.** ownership and copy analysis, drop path, memory-locking policy where used,
and explicit non-guarantees.

## RUST-DOC-0003-R010 — Design before `Arc<Mutex<T>>`

**Statement.** `Arc<Mutex<T>>` MUST NOT be the default substitute for identifying ownership,
task responsibility, mutation protocol, lock scope, and shutdown.

**Intent.** Avoid shared mutable bags that compile but hide contention, deadlock, and authority.

**Applicability.** Concurrent shared state and service handles.

**Allowed exceptions.** It MAY be the simplest correct mechanism after the ownership and
synchronization contract is documented.

**Review evidence.** owner, lock invariant, contention and poisoning policy, alternatives, and
tests.

## RUST-DOC-0003-R011 — Justify interior mutability

**Statement.** Interior mutability MUST be justified by a required aliasing contract and MUST
preserve the domain's synchronization and authority invariants.

**Intent.** Prevent `Cell`, `RefCell`, locks, or atomics from bypassing a better ownership
design.

**Applicability.** Mutation through shared references.

**Allowed exceptions.** Local caching or instrumentation MAY use it when invisible to domain
semantics and reentrancy is safe.

**Review evidence.** aliasing rationale, borrow/panic behavior, synchronization, and reentrancy
tests.

## RUST-DOC-0003-R012 — Use lifetimes for real relationships

**Statement.** Lifetime parameters SHOULD express actual borrowing or validity relationships,
not ornamental complexity or an inaccurate claim that an external resource remains valid.

**Intent.** Keep APIs readable and prevent local borrow duration from implying remote
liveness.

**Applicability.** Borrowed views, guards, transactions, callbacks, and FFI.

**Allowed exceptions.** Internal generic abstraction MAY carry a lifetime required by a
dependency, with its relationship documented.

**Review evidence.** referent and duration explanation, escape analysis, and simpler owned
alternative.

## RUST-DOC-0003-R013 — Define cross-task ownership

**Statement.** Transfer of authority or resources across tasks MUST identify the new owner,
completion signal, cancellation behavior, shutdown responsibility, and behavior if the task
is dropped or panics.

**Intent.** Prevent detached custody and resources with no accountable closer.

**Applicability.** Spawned tasks, worker actors, channels carrying handles, and supervisors.

**Allowed exceptions.** Truly process-lifetime services MAY be owned by the process supervisor.

**Review evidence.** task tree, join/abort contract, channel closure, and shutdown tests.

## RUST-DOC-0003-R014 — Keep external authority revalidation explicit

**Statement.** A local capability MUST NOT claim current external authority when revocation,
expiry, tenant membership, or resource ownership can change without local control; current
use MUST revalidate or carry a bounded lease.

**Intent.** Prevent stale authorization.

**Applicability.** Sessions, identity-provider grants, distributed locks, and policy decisions.

**Allowed exceptions.** Immutable operation-scoped grants MAY remain valid for their defined
commit window.

**Review evidence.** lease or recheck boundary, stale-state handling, and revocation race
tests.

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

## Source: `doctrines/0006-distributed-uncertainty/doctrine.md`

# Normative doctrine

## RUST-DOC-0006-R001 — Do not equate timeout with non-execution

**Statement.** A timeout MUST NOT be represented as confirmed failure when the
remote operation may have executed.

**Intent.** Preserve the distinction between stopping local waiting and learning
remote outcome.

**Applicability.** network requests, database commit, broker acknowledgement,
filesystem operations over remote mounts, and subprocess protocols.

**Allowed exceptions.** A timeout may be definitive only when protocol evidence
establishes that execution could not have begun or was atomically cancelled.

**Review evidence.** protocol timeline, cancellation semantics, and explicit
unknown-outcome path.

## RUST-DOC-0006-R002 — Model operationally distinct outcomes

**Statement.** Outcome types MUST distinguish confirmed success, confirmed
rejection, local failure before dispatch, and unknown outcome when callers
require different recovery.

**Intent.** Prevent transport symptoms from erasing domain knowledge.

**Applicability.** consequential external operations.

**Allowed exceptions.** Categories may combine when no caller action, audit
meaning, security consequence, or reconciliation path differs.

**Review evidence.** outcome decision table and exhaustive caller handling.

## RUST-DOC-0006-R003 — Carry reconciliation evidence

**Statement.** An unknown outcome MUST carry or reference sufficient evidence
to reconcile it, including stable operation identity and the external target.

**Intent.** Make uncertainty actionable and auditable.

**Applicability.** payments, messages, provisioning, commits, and any effect that
cannot safely be repeated blindly.

**Allowed exceptions.** An explicitly irreconcilable best-effort action may
retain only audit evidence if business policy accepts permanent uncertainty.

**Review evidence.** reconciliation token, operation ID, request fingerprint,
target, attempt history, and observation method.

## RUST-DOC-0006-R004 — Analyze before retry

**Statement.** Every retry policy MUST classify the operation as safe to retry,
unsafe to retry, or reconcile-before-retry for each relevant failure point.

**Intent.** Prevent duplicate effects and unsafe assumptions.

**Applicability.** clients, consumers, publishers, schedulers, and operator
runbooks.

**Allowed exceptions.** Pure reads may use a simpler safe-retry classification
when staleness and load remain documented.

**Review evidence.** failure-point matrix, idempotency mechanism, deadline, and
attempt budget.

## RUST-DOC-0006-R005 — Define idempotency-key semantics

**Statement.** An idempotency key MUST have defined uniqueness, caller and
resource scope, payload binding, retention, concurrency, replay, and conflict
semantics.

**Intent.** Prevent a string field from being mistaken for idempotent behavior.

**Applicability.** mutable external APIs and durable commands.

**Allowed exceptions.** Naturally idempotent operations may omit keys when their
semantic identity and repeated-result behavior are established independently.

**Review evidence.** key contract, storage constraint, same-key/same-payload and
same-key/different-payload tests, and expiry policy.

## RUST-DOC-0006-R006 — Reuse operation identity across attempts

**Statement.** Retries of one logical operation MUST reuse its operation and
idempotency identity. A new identity MUST mean a new requested effect.

**Intent.** Allow receivers and reconcilers to distinguish replay from new
intent.

**Applicability.** external API requests, published commands, and repair tools.

**Allowed exceptions.** A protocol-mandated new transport attempt identifier may
be added, but it MUST remain correlated to the stable logical operation.

**Review evidence.** identity lifecycle and attempt log.

## RUST-DOC-0006-R007 — Expect duplicate delivery

**Statement.** Consumers in at-least-once systems MUST expect duplicate
delivery and MUST define whether repeated processing is deduplicated,
idempotent, commutative, or safely rejected.

**Intent.** Make acknowledgement loss and redelivery ordinary protocol paths.

**Applicability.** brokers, job queues, webhook delivery, change feeds, and
replayed logs.

**Allowed exceptions.** A verified at-most-once boundary may accept loss instead
of duplicates, with that loss documented.

**Review evidence.** duplicate test, stable message identity, and effect-level
handling.

## RUST-DOC-0006-R008 — Persist deduplication durably

**Statement.** Deduplication that protects a durable effect MUST itself use
durable state with atomic relationship to that effect, and MUST define
retention.

**Intent.** Prevent process restart or pruning from reopening duplicate effects.

**Applicability.** consumer inboxes, payment commands, and webhook handlers.

**Allowed exceptions.** In-memory deduplication may protect only ephemeral
best-effort work whose duplicate cost is accepted.

**Review evidence.** unique key, transaction boundary, retention calculation,
and replay-after-restart test.

## RUST-DOC-0006-R009 — State ordering scope

**Statement.** Ordering claims MUST identify key or partition, producer set,
consumer concurrency, retry behavior, failover behavior, and observation point.

**Intent.** Prevent partition-local or producer-local order from becoming a
false global guarantee.

**Applicability.** brokers, streams, event logs, RPC sequencing, and replication.

**Allowed exceptions.** None when business behavior relies on order.

**Review evidence.** ordering contract and tests for retries, multiple
producers, and failover.

## RUST-DOC-0006-R010 — Qualify exactly-once claims

**Statement.** Any "exactly once" claim MUST identify the precise boundary,
identity, transactional mechanism, failure assumptions, retention, and effects
included. It MUST NOT imply exactly-once behavior beyond that boundary.

**Intent.** Replace a broad slogan with an auditable scoped guarantee.

**Applicability.** messaging, stream processing, payments, jobs, and APIs.

**Allowed exceptions.** None.

**Review evidence.** guarantee ledger, protocol documentation, duplicate tests,
and excluded effects.

## RUST-DOC-0006-R011 — Coordinate acknowledgement with effect

**Statement.** A consumer MUST define the order and atomic relationship among
effect execution, durable progress, and acknowledgement.

**Intent.** Make the duplicate-versus-loss tradeoff visible.

**Applicability.** message and job consumers.

**Allowed exceptions.** Best-effort consumers may acknowledge early only when
loss is accepted and measured.

**Review evidence.** crash-point matrix and tests before and after each durable
step.

## RUST-DOC-0006-R012 — Treat compensation as a new effect

**Statement.** Sagas and compensating operations MUST NOT be described as
rollback. Each compensation MUST remain fallible, idempotency-analyzed, and
capable of an unknown outcome.

**Intent.** Preserve real-world irreversibility and changed conditions.

**Applicability.** distributed workflows, reservations, payments, and
provisioning.

**Allowed exceptions.** A local database rollback may be called rollback within
its actual transaction boundary.

**Review evidence.** forward/compensation pairs, business non-equivalence,
failure handling, and reconciliation.

## RUST-DOC-0006-R013 — Treat observations as time-scoped evidence

**Statement.** External observations MUST record or imply their observation
time and MUST NOT be presented as immutable current truth when the external
state can change.

**Intent.** Prevent stale reads from becoming permanent authority.

**Applicability.** status queries, authorization, inventory, leases, and
reconciliation.

**Allowed exceptions.** Immutable append-only facts may remain stable when the
source contract establishes immutability.

**Review evidence.** freshness policy, version or timestamp, cache behavior, and
revalidation trigger.

## RUST-DOC-0006-R014 — Address concurrent execution and split brain

**Statement.** Where multiple workers or coordinators can act on one logical
operation, the design MUST address concurrent execution using ownership,
leases with fencing, compare-and-set state, consensus-backed leadership, or an
effect-level idempotency mechanism.

**Intent.** Prevent stale owners and duplicate coordinators from acting with
equal authority.

**Applicability.** reconciliation workers, schedulers, failover, and distributed
locks.

**Allowed exceptions.** Concurrent execution is allowed for commutative,
duplicate-safe operations with evidence.

**Review evidence.** authority protocol, expiry, fencing token use, clock
assumptions, and overlap test.

## RUST-DOC-0006-R015 — Bound retries and reconciliation

**Statement.** Retry and reconciliation loops MUST have bounded concurrency,
attempt or time budgets, backoff where appropriate, terminal escalation, and
observability.

**Intent.** Prevent uncertainty from turning into permanent load or hidden
backlog.

**Applicability.** retry queues, reconcilers, publishers, and operator repair.

**Allowed exceptions.** A durable obligation may remain pending indefinitely,
but each execution cycle still requires bounded work and visible age.

**Review evidence.** queue capacity, schedule, age metrics, dead-letter or
manual escalation, and overload test.

## RUST-DOC-0006-R016 — Preserve correlation and causality

**Statement.** Audit trails MUST preserve stable operation identity, attempt
identity, triggering event, parent correlation, request fingerprint, outcome
observations, and reconciliation decisions where these affect accountability.

**Intent.** Reconstruct what was requested, attempted, observed, and resolved.

**Applicability.** consequential distributed effects.

**Allowed exceptions.** Low-risk telemetry may use aggregated correlation when
individual reconstruction is unnecessary.

**Review evidence.** event schema, trace propagation, redaction, and end-to-end
incident query.

## RUST-DOC-0006-R017 — Protect sensitive reconciliation data

**Statement.** Reconciliation and audit evidence MUST contain enough identity
to act without unnecessarily storing credentials, secret payloads, or sensitive
personal data.

**Intent.** Avoid turning operational evidence into a second secret database.

**Applicability.** operation logs, dead-letter records, tracing, and support
tools.

**Allowed exceptions.** Required regulated evidence may be retained with
documented access, encryption, minimization, and deletion policy.

**Review evidence.** field classification, redaction tests, access policy, and
retention.

## RUST-DOC-0006-R018 — Test failure points, not only final errors

**Statement.** Distributed-effect tests MUST inject loss, delay, duplication,
reordering, concurrent execution, and crash points between durable steps in
proportion to risk.

**Intent.** Exercise ambiguity and replay paths hidden by happy-path mocks.

**Applicability.** integrations, consumers, publishers, and reconcilers.

**Allowed exceptions.** A low-risk pure read may narrow the matrix and state
why.

**Review evidence.** fault matrix linked to invariants, test results, and
unexercised assumptions.

## RUST-DOC-0006-R019 — State residual uncertainty

**Statement.** Public and internal contracts MUST state which outcomes can
remain unknown, how long, who owns reconciliation, and what users or operators
may safely do meanwhile.

**Intent.** Make uncertainty an owned lifecycle state rather than an error
message.

**Applicability.** every consequential effect with ambiguous execution.

**Allowed exceptions.** None.

**Review evidence.** state machine, service-level target, escalation path, and
guarantee ledger.

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
