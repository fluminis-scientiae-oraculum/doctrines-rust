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
