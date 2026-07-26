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

| Field | Content |
|---|---|
| ID | INV-PAY-004 |
| Statement | A locally requested capture references an accepted authorization for the same payment and amount |
| Scope | capture command construction |
| Owner | payment domain |
| Classification | transition and cross-entity invariant |
| Enforcement mechanism | verifier-issued capability plus runtime amount comparison |
| Trust boundary | provider authorization response and persisted reload |
| Evidence | constructor tests, compiler rejection before authorization, integration contract test |
| Failure consequence | unauthorized or wrong-amount capture |
| Residual uncertainty | provider may reject, time out, or accept without returning acknowledgement |

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

| Invariant shape | Usual first mechanism |
|---|---|
| Mutually exclusive state | enum with variant-specific data |
| Stable local scalar rule | opaque validated newtype |
| Whole-collection rule | validated collection wrapper |
| Small locally controlled sequence | consuming transition or typestate |
| Authority possession | capability type |
| Dynamic or persisted lifecycle | runtime enum and validated state machine |
| External input | parse and runtime validation |
| Cross-entity fact | domain service plus transactional validation |
| External effect result | structured `Result` |
| Ambiguous distributed effect | explicit unknown state and reconciliation |

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
