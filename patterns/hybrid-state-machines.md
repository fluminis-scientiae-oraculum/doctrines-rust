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
