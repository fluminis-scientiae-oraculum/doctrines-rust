# Explicit uncertainty

## 1. Problem

An external operation can execute while its acknowledgement is lost. A timeout
or connection error does not reveal whether the effect occurred. Mapping the
observation to success or failure invents certainty and makes later retry
unsafe.

## 2. Forces

Callers need actionable outcomes. Confirmed success and rejection carry
different evidence. Unknown states need stable identity, persistence,
reconciliation, age, and ownership. Idempotency can make replay safer within a
defined boundary but does not automatically cover every effect. User interfaces
must prevent harmful repeat actions while still showing progress.

## 3. Weak representation

```rust
fn capture(...) -> Result<Receipt, CaptureError>;
```

If `CaptureError::Timeout` is handled as rejection, callers may submit a new
capture. If it is handled as success, records may claim money moved when it did
not. A string error loses reconciliation identity.

## 4. Improved representation

```rust
pub enum CaptureOutcome {
    Confirmed(CaptureReceipt),
    Rejected(CaptureRejection),
    Unknown {
        operation_id: OperationId,
        reconciliation: ReconciliationToken,
    },
}
```

A separate outer error may represent failure proven before dispatch. Unknown is
a durable lifecycle state, not a transient logging category.

## 5. Exact guarantee gained

The type prevents exhaustive callers from treating unknown as either confirmed
terminal result without an explicit branch. The unknown variant guarantees
availability of the fields its constructor requires, such as operation identity
and reconciliation token. State transitions can restrict resolution to new
evidence.

## 6. Guarantees not gained

The type does not determine the external result, make reconciliation succeed,
or make retry safe. A token may reference stale or incomplete data. Confirmed
success proves only the external boundary and time represented by its evidence;
later reversal or settlement may remain possible.

## 7. Boundary considerations

Classify where failure occurred. Only protocol evidence can establish
pre-dispatch non-execution. Authenticate provider responses and callbacks.
Bind idempotency keys to request fingerprints. Protect reconciliation records
from secret leakage. API error mapping must preserve unknown rather than
collapsing it into generic service failure.

## 8. Persistence considerations

Persist operation ID, external key, target, request fingerprint, attempt
history, current evidence, age, next observation, and optimistic version.
Index pending reconciliation. Retain idempotency and deduplication evidence
longer than retry/replay horizons. Operator overrides are audited decisions, not
retroactive proof.

## 9. Testing evidence

Fault-test loss before dispatch, after dispatch, after remote execution, and
after acknowledgement. Assert that only the ambiguous cases become unknown.
Test repeated reconciliation: still unknown, then confirmed or rejected.
Test concurrent reconcilers, idempotent attempts, retention expiry, and UI/API
behavior that prevents blind repeat.

## 10. Costs

Unknown adds states, storage, worker ownership, user messaging, monitoring, and
support procedures. Reconciliation can consume external quota and remain
unavailable. Stable identifiers and retention increase data-management burden.
Every downstream match must handle the additional state.

## 11. When not to use it

Do not add unknown to pure local validation or failures proven before an effect
boundary. Do not use it as a vague replacement for structured errors. A
best-effort telemetry action may accept permanent loss without reconciliation,
provided the contract states that. Conversely, do not omit unknown merely to
simplify a consequential API.

## 12. Related doctrines

RUST-DOC-0002 preserves actionable error categories. RUST-DOC-0005 governs
durable operation records. RUST-DOC-0006 supplies the complete retry,
idempotency, and reconciliation rules. RUST-DOC-0008 governs fault evidence.

## 13. Executable example

See [`../examples/distributed-outcomes/src/lib.rs`](../examples/distributed-outcomes/src/lib.rs)
and the payment, message-delivery, and database-transaction case studies.

## 14. Worked application

An email provider may accept a send request and lose the acknowledgement. The
application stores `DeliveryUnknown { operation_id, provider_key, first_attempt
}` rather than `Failed`. A reconciler queries provider status or consumes a
provider event. If the provider cannot supply final evidence, policy may keep
the state unknown and warn the user that a repeat could duplicate delivery.

The same pattern applies to database commit ambiguity, but reconciliation
sources differ. A transaction ID, durable business key, or follow-up read may
establish committed state. The generic unknown shape should not hide
domain-specific evidence or safe action.

## 15. Review prompts

- At what exact point can execution become ambiguous?
- Which response proves rejection, and is its source authenticated?
- Does unknown retain stable logical and external identity?
- Can reconciliation itself return stale or unknown evidence?
- Which retry is safe before and after observation?
- How are concurrent reconcilers controlled?
- What do API and UI callers do while pending?
- Is retention long enough for delayed acknowledgements and replay?
- Are terminal operator decisions recorded as policy rather than fabricated
  proof?
