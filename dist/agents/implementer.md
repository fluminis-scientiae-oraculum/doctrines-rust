<!--
GENERATED FILE. DO NOT EDIT DIRECTLY.
Canonical sources live under /foundations, /doctrines, /patterns,
 /boundaries, /reviews, and /agents.
-->

# Implementer agent doctrine pack

Build protected construction paths, honest failure models, and executable evidence.

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

## Source: `agents/implementer.md`

# Implementer overlay

## Purpose

Translate approved invariant and boundary artifacts into protected Rust APIs,
runtime checks, persistence mappings, failure models, and executable evidence.
Implement only guarantees the design establishes. If implementation reveals a
missing invariant, effect, or authority decision, update or escalate the plan
instead of silently choosing a broader contract.

## Construction workflow

1. Locate the invariant ID and evidence level.
2. Create a raw or structural input representation where needed.
3. Keep trusted representation fields private.
4. Implement one complete checked constructor or `TryFrom`.
5. Make fallibility and structured errors visible.
6. Expose read/borrow methods that cannot invalidate the value.
7. Restrict privileged and unsafe construction.
8. Route Serde, database, configuration, message, HTTP/RPC, file, and FFI
   inputs through checked construction.
9. Add positive, negative, boundary, and bypass evidence.

Never use derived deserialization or ORM convenience if it assigns trusted state
without the invariant gate. Do not add `From<Raw>` for a fallible conversion.

## State and ownership workflow

Use enums for mutually exclusive dynamic state and place associated data in the
relevant variant. Use consuming transitions when prior-handle reuse is invalid.
Adopt typestate only when the approved complexity decision shows a small,
locally controlled protocol. Keep external operations fallible in every state.

Capability constructors live with the authorizer or resource owner. Avoid
unreviewed `Clone` on authority, handles, single-use tokens, sessions, and
secrets. Define resource cleanup through RAII only for locally controllable
release; provide explicit close/rollback or reconciliation where cleanup can
fail externally.

## Boundary workflow

Implement:

```text
bounded raw input
  → physical parse
  → raw DTO/row/foreign representation
  → checked domain construction
  → authorized operation
  → structured result or explicit unknown
```

Enforce length, allocation, nesting, batch, and concurrency limits before costly
work. Preserve version and unknown-value policy. Redact public failures and
logs. Authentication yields principal evidence; authorization yields
action/resource permission. Client or UI claims do not create authority.

For persistence, decode raw rows and quarantine invalid history. Use
constraints and transaction semantics that actually protect cross-entity rules.
Expose optimistic conflicts. Use outbox/inbox or an approved durable protocol
when state and messaging intent cannot be forgotten independently.

## External effects

Create a stable logical operation ID before dispatch. Reuse identity across
attempts. Bind idempotency to target and payload according to the approved
contract. Do not retry transport errors generically. Preserve:

- confirmed success with evidence;
- confirmed rejection;
- failure proven before dispatch;
- unknown execution with reconciliation identity.

Unknown outcomes are persisted and owned when process loss matters.
Compensation is implemented as a new fallible, idempotency-analyzed operation.

## Concurrency and async

Implement the approved state/task ownership model. Bound task count, channels,
pools, permits, and retry queues. Handle channel closure. Avoid blocking
executor workers; bound blocking work too. Review every `.await` after partial
mutation for cancellation safety. Do not hold synchronous locks across await.
Supervise spawned tasks and implement admission stop, drain, deadline, and
forced shutdown behavior.

Atomics require a happens-before argument. Do not manually implement `Send` or
`Sync` without doctrine 0007 proof.

## Unsafe policy

Avoid unsafe unless explicitly approved. If required, minimize it, state a
complete `SAFETY:` argument, protect the safe abstraction for every safe caller,
document unsafe caller obligations, and add Miri/sanitizer/fuzz/model evidence
where applicable. Never use unsafe to bypass trusted construction or silence
the borrow checker.

## Evidence implementation

Add unit tests for constructor and transition behavior, property tests for
generative invariants, compile-fail tests for important prohibited programs,
integration tests for real boundaries, and fault injection for partial effects.
Compile-fail sources remain minimal. Inspect expected diagnostics before
committing changes. Control clocks, schedules, ports, files, randomness, and
external state for reproducibility.

Do not update snapshots or `.stderr` outputs blindly. Do not accept flaky tests
through permanent retries. Benchmarks remain separate from correctness and
follow doctrine 0009.

## Forbidden implementation claims

Do not state that private fields alone prove constructor correctness; tests
alone prove all inputs; `Open` proves remote liveness; `NonZeroU64` solves money
policy; database rows are trusted; timeout is rejection; outbox is end-to-end
exactly once; or an async rewrite is faster without measurement.

## Escalation and completion

Escalate when code needs a new escape hatch, weaker guarantee, different state
graph, authority expansion, schema incompatibility, unsafe block, dependency,
or retry behavior. Cite the affected invariant and proposed tradeoff.

Implementation is complete when canonical claims match code, all construction
paths are protected, relevant focused reviews pass, evidence covers prohibited
and failure paths, generation reproduces, and every requested local validation
passes. Report any unrun or failing command precisely.

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
resource controls may be required before parsing. Raw input is not “bad”; it is simply
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

**Applicability.** Nested locks, callbacks under locks, standard-library mutexes,
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

## Source: `reviews/domain-model-review.md`

# Domain model review

## Record

Apply to every trusted value, aggregate, state machine, and authority-bearing
handle. Record **pass**, **fail**, **not applicable**, or **waiver reference**,
with source paths and doctrine rule IDs.

## Values and names

| ID | Question | Pass evidence |
|---|---|---|
| DMR-01 | Does each trusted type name only evidence its constructors establish? | constructor/name comparison |
| DMR-02 | Are raw, parsed, policy-accepted, verified, authorized, and reconciled values distinct where operationally different? | evidence ladder |
| DMR-03 | Are primitive aliases replaced where unit or invariant mixing is consequential? | opaque type |
| DMR-04 | Are zero and empty states evaluated explicitly? | boundary table |
| DMR-05 | Does integer money include explicit currency? | money representation |
| DMR-06 | Does arithmetic reject currency mismatch and overflow? | checked operations |
| DMR-07 | Are tax, FX, scale, allocation, and rounding outside the scalar guarantee? | non-guarantees |
| DMR-08 | Does email syntax avoid ownership/deliverability claims? | evidence-accurate types |
| DMR-09 | Are identifiers nonempty/bounded/normalized according to one policy? | constructor |
| DMR-10 | Are secrets deliberately non-formatting and minimally cloneable? | trait/API audit |

## Construction and mutation

| ID | Question | Pass evidence |
|---|---|---|
| DMR-11 | Are trusted representation fields private? | visibility inspection |
| DMR-12 | Does every public constructor enforce the complete documented invariant? | constructor trace |
| DMR-13 | Is fallible construction visibly fallible? | `Result`/`TryFrom` API |
| DMR-14 | Is normalization centralized and ordered before dependent checks? | construction pipeline |
| DMR-15 | Are errors structured and actionable? | error enum/categories |
| DMR-16 | Do mutation methods preserve the complete invariant? | mutation proof/tests |
| DMR-17 | Are mutable representation escapes absent? | no `DerefMut`/raw field |
| DMR-18 | Do conversion impls preserve evidence direction? | `From` versus `TryFrom` audit |
| DMR-19 | Are unchecked constructors private, narrow, and obligation-documented? | escape-hatch inventory |
| DMR-20 | Are unsafe constructors reviewed under doctrine 0007? | safety proof |

## States and transitions

| ID | Question | Pass evidence |
|---|---|---|
| DMR-21 | Are mutually exclusive states represented by a sum type? | state shape |
| DMR-22 | Are contradictory boolean/optional combinations absent? | truth-table review |
| DMR-23 | Does associated data live only in meaningful variants? | enum payloads |
| DMR-24 | Are independent dimensions kept separate? | state decomposition |
| DMR-25 | Is variant evolution and unknown persistence planned? | encoding/version policy |
| DMR-26 | Are legal transition edges explicit? | state graph |
| DMR-27 | Are illegal transitions structurally blocked or explicitly rejected? | API/runtime checks |
| DMR-28 | Do consuming transitions prevent invalid prior-state reuse where useful? | ownership API |
| DMR-29 | Do fallible transitions preserve or consume prior authority honestly? | error shape |
| DMR-30 | Do async transitions handle cancellation and partial effects? | cancellation matrix |
| DMR-31 | Does local typestate avoid remote-liveness claims? | guarantee ledger |
| DMR-32 | Are persisted/dynamic states represented at runtime? | hybrid/runtime model |
| DMR-33 | Are unknown distributed outcomes explicit? | outcome enum |
| DMR-34 | Does reconciliation require new evidence rather than arbitrary assignment? | transition service |

## Authority, aggregates, and external rules

| ID | Question | Pass evidence |
|---|---|---|
| DMR-35 | Are privileged constructors restricted to the authority owner? | module visibility |
| DMR-36 | Does each capability expose least-privilege operations? | API surface |
| DMR-37 | Is capability cloning justified? | clone/transfer policy |
| DMR-38 | Are expiry and revocation runtime semantics explicit? | authority lifecycle |
| DMR-39 | Are single-use tokens consumed or transactionally claimed? | use protocol |
| DMR-40 | Are collection invariants protected after mutation? | wrapper API |
| DMR-41 | Are completeness claims absent from paginated subsets? | type naming |
| DMR-42 | Are cross-entity rules enforced in a domain service/transaction? | service boundary |
| DMR-43 | Are environmental assumptions represented as checks/observations? | runtime validation |
| DMR-44 | Do external effects remain fallible? | `Result`/outcome API |
| DMR-45 | Does timeout preserve unknown execution where needed? | outcome handling |
| DMR-46 | Are public escape hatches enumerated in the guarantee ledger? | ledger |
| DMR-47 | Does each type list what it proves? | documentation |
| DMR-48 | Does each type list what it does not prove? | documentation |
| DMR-49 | Does executable evidence cover acceptance and rejection? | tests |
| DMR-50 | Is type-system complexity proportional to misuse impact? | complexity decision |

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

| ID | Question | Pass evidence |
|---|---|---|
| BR-01 | Is the boundary owner and threat/error model named? | boundary record |
| BR-02 | Are all raw bytes, metadata, and alternate sources inventoried? | input list |
| BR-03 | Is transport/physical parsing separate from domain validation? | layered conversion |
| BR-04 | Is a raw DTO/row/foreign type used where contracts differ? | representation map |
| BR-05 | Are trusted types constructed only after complete validation? | call trace |
| BR-06 | Are authentication and authorization separate transitions? | evidence path |
| BR-07 | Are cross-entity checks placed transactionally or in domain services? | enforcement location |
| BR-08 | Are egress DTOs deliberate rather than broad domain serialization? | output types |
| BR-09 | Are privileged administrative paths included in the inventory? | bypass list |
| BR-10 | Are cache, replay, restore, and migration paths included? | complete map |

## Parsing and resource limits

| ID | Question | Pass evidence |
|---|---|---|
| BR-11 | Is maximum raw input size enforced before large allocation? | limit/config test |
| BR-12 | Are decompression ratios and expanded size bounded? | decompression policy |
| BR-13 | Are nesting, field, element, and batch counts bounded? | parser limits |
| BR-14 | Are numeric conversions checked for range and units? | conversion code |
| BR-15 | Are text encoding and Unicode policies explicit? | parser policy |
| BR-16 | Are duplicate fields/headers/keys handled deliberately? | fixtures |
| BR-17 | Are paths protected from traversal and lossy conversion? | path policy |
| BR-18 | Are pointer null, length, alignment, and ownership checked at FFI? | FFI contract |
| BR-19 | Are time and size values represented with typed units? | DTO/domain types |
| BR-20 | Does malformed input return structured failure without panic? | negative tests |

## Construction and bypasses

| ID | Question | Pass evidence |
|---|---|---|
| BR-21 | Does Serde delegate to checked `TryFrom` or manual validation? | implementation |
| BR-22 | Does every database read validate trusted newtypes? | row conversions |
| BR-23 | Are derived decoders prevented from assigning trusted fields unchecked? | derive audit |
| BR-24 | Are unchecked `From` conversions absent for fallible evidence? | impl search |
| BR-25 | Are defaults prevented from inventing historical or verified facts? | default audit |
| BR-26 | Are partial projections named as partial types? | query/type review |
| BR-27 | Can test-only or feature-gated constructors ship? | feature matrix |
| BR-28 | Are unsafe layout/construction shortcuts absent or fully audited? | unsafe inventory |
| BR-29 | Are UI/client claims excluded from backend authority? | authorization trace |
| BR-30 | Does every bypass have a scoped, reviewed obligation? | escape-hatch ledger |

## Evolution, errors, and secrecy

| ID | Question | Pass evidence |
|---|---|---|
| BR-31 | Is long-lived representation versioned? | envelope/schema version |
| BR-32 | Is unknown-field policy deliberate? | reject/ignore/retain rationale |
| BR-33 | Is unknown enum/version behavior explicit? | compatibility tests |
| BR-34 | Are stable external tags independent of source rename? | encoding table |
| BR-35 | Is rolling old/new compatibility tested? | version matrix |
| BR-36 | Are syntax, validation, authority, conflict, availability, and unknown outcomes distinguishable as needed? | error model |
| BR-37 | Are source errors retained internally? | error chain |
| BR-38 | Are public diagnostics redacted and stable? | error mapping tests |
| BR-39 | Are secrets absent from logs, debug, metrics, and snapshots? | redaction audit |
| BR-40 | Are quarantine/dead-letter records access-controlled and retained safely? | operations policy |
| BR-41 | Are correlation IDs bounded and sensitivity-classified? | telemetry schema |
| BR-42 | Are credentials minimized across parsing copies? | secret data flow |

## Evidence and non-guarantees

| ID | Question | Pass evidence |
|---|---|---|
| BR-43 | Do tests cover valid and invalid boundary values? | fixtures |
| BR-44 | Do tests cover oversized and resource-hostile input? | adversarial cases |
| BR-45 | Do tests cross the real codec/driver/router/ABI where consequential? | integration suite |
| BR-46 | Do tests cover old and future/unknown values? | compatibility fixtures |
| BR-47 | Do fault tests cover partial effects and acknowledgement loss? | fault matrix |
| BR-48 | Are invalid historical records rejected or quarantined? | database evidence |
| BR-49 | Is fuzz/property evidence used where input space warrants it? | test record |
| BR-50 | Does the boundary ledger state what parsing proves? | guarantee entry |
| BR-51 | Does it state mutable external facts not proved? | non-guarantees |
| BR-52 | Are observation time, freshness, and reconciliation recorded? | evidence lifecycle |

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

| ID | Question | Pass evidence |
|---|---|---|
| TSR-01 | Is the protected problem operation sequencing rather than value validation? | invariant classification |
| TSR-02 | Is the sequence locally controlled by one handle owner? | ownership map |
| TSR-03 | Is state not primarily externally determined? | boundary analysis |
| TSR-04 | Is the state graph small and stable enough for static APIs? | node/edge count |
| TSR-05 | Are illegal calls consequential and frequent enough to justify types? | risk assessment |
| TSR-06 | Can marker construction be restricted? | visibility |
| TSR-07 | Does each marker represent evidence actually established? | guarantee mapping |
| TSR-08 | Are independent dimensions kept out of a type cross-product? | decomposition |
| TSR-09 | Is the workflow unsuitable for a simpler consuming non-generic handle? | alternative comparison |
| TSR-10 | Is a runtime enum explicitly evaluated? | decision record |

## Transition design

| ID | Question | Pass evidence |
|---|---|---|
| TSR-11 | Does each legal edge have one clear method? | state API graph |
| TSR-12 | Are illegal state-specific methods absent from the type? | impl inspection |
| TSR-13 | Do transitions consume the prior state when reuse is invalid? | signatures |
| TSR-14 | Are infallible transitions truly local and infallible? | operation analysis |
| TSR-15 | Do fallible transitions return structured errors? | error types |
| TSR-16 | Is the prior handle returned only when non-transition is proven? | error/recovery shape |
| TSR-17 | Is external ambiguity represented instead of restoring old state? | unknown outcome |
| TSR-18 | Are transition payloads carried in the successor type? | successor fields |
| TSR-19 | Are authorization and capability requirements explicit? | method arguments |
| TSR-20 | Can parallel or duplicate transition attempts occur through another handle? | resource identity analysis |

## Async and external reality

| ID | Question | Pass evidence |
|---|---|---|
| TSR-21 | Is every `.await` in a transition cancellation-reviewed? | cancellation table |
| TSR-22 | Does cancellation release or reconcile consumed resources? | cleanup evidence |
| TSR-23 | Are blocking operations isolated appropriately? | async design |
| TSR-24 | Does an open/connected marker mean only local transition success? | documentation |
| TSR-25 | Do send/capture/commit operations remain fallible? | method results |
| TSR-26 | Can timeout produce explicit unknown outcome? | outcome type |
| TSR-27 | Are external facts re-observed when needed? | validation policy |
| TSR-28 | Does typestate avoid claiming current remote liveness? | guarantee ledger |
| TSR-29 | Does a lease/capability marker account for expiry/revocation? | runtime check |
| TSR-30 | Is compensation modeled as a later fallible transition? | state graph |

## Persistence and ergonomics

| ID | Question | Pass evidence |
|---|---|---|
| TSR-31 | Must states be stored heterogeneously or inspected dynamically? | usage inventory |
| TSR-32 | Is a stable runtime persisted enum defined where needed? | storage model |
| TSR-33 | Does rehydration validate before issuing a typed handle? | restoration service |
| TSR-34 | Is marker spelling excluded from durable protocol evidence? | encoding policy |
| TSR-35 | Are optimistic version or claim semantics present for multiple workers? | persistence concurrency |
| TSR-36 | Can trait objects or plugin interfaces use the design clearly? | dispatch design |
| TSR-37 | Are mocks and test harnesses comprehensible? | test API review |
| TSR-38 | Are compiler diagnostics useful at misuse sites? | compile-fail output |
| TSR-39 | Is generic propagation limited at public boundaries? | API signatures |
| TSR-40 | Are transition errors smaller/clearer than equivalent runtime checks? | caller comparison |
| TSR-41 | Has monomorphization/code-size impact been considered? | size analysis |
| TSR-42 | Has compile-time and IDE diagnostic impact been considered? | complexity budget |

## Evidence and decision

| ID | Question | Pass evidence |
|---|---|---|
| TSR-43 | Do compile-fail tests prove important prohibited calls? | UI tests |
| TSR-44 | Do they fail for the intended reason? | diagnostic inspection |
| TSR-45 | Are every successful and failed transition tested? | unit suite |
| TSR-46 | Are cancellation and external failure tested? | fault suite |
| TSR-47 | Is persisted/runtime conversion tested? | integration suite |
| TSR-48 | Are unknown outcomes and reconciliation tested? | distributed tests |
| TSR-49 | Does documentation state exact guarantees and non-guarantees? | ledger/docs |
| TSR-50 | Does benefit exceed type/API complexity? | signed decision |

## Exit criteria

Approve typestate only when local sequencing is the real risk, construction is
protected, external effects remain fallible, persistence has a runtime model,
diagnostics are acceptable, and the complexity comparison favors it. Otherwise
select a runtime enum, consuming transition, capability, or ordinary validation.
