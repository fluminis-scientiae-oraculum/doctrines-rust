<!--
GENERATED FILE. DO NOT EDIT DIRECTLY.
Canonical sources live under /foundations, /doctrines, /patterns,
 /boundaries, /reviews, and /agents.
-->

# Shared agent doctrine pack

Common obligations and doctrine navigation for every agent role.

## Assembly

Ceiling `operational`, declared for the `shared` pack in `manifest/agents.yaml`. A section annotated above that ceiling is withheld here. Nothing was withheld at this ceiling.

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
[`foundations/complexity-budget.md`](../../foundations/complexity-budget.md) MAY remain prose-carried when the
assessment, its owner, and the residual risk are recorded on the terms of RUST-DOC-0011-R020.

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
landed. This rule does not weaken the RFC obligations stated in [`AGENTS.md`](../../AGENTS.md) and
[`rfcs/README.md`](../../rfcs/README.md), which govern the change process rather than the resulting contract.

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
