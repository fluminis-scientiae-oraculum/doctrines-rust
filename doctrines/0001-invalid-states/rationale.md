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
