# Payment lifecycle: improved design

## Durable runtime state

```rust
pub enum PaymentStatus {
    Draft,
    Validated { policy_version: PolicyVersion },
    Authorized { authorization: AuthorizationEvidence },
    CaptureUnknown { reconciliation: CaptureReconciliation },
    Captured { receipt: CaptureReceipt },
    Settled { settlement: SettlementEvidence },
    Reversed { reversal: ReversalReceipt },
    Failed { failure: PaymentFailure },
}

pub struct PaymentRecord {
    id: PaymentId,
    account: AccountId,
    amount: PositiveMoney,
    status: PaymentStatus,
    version: u64,
}
```

The enum stores heterogeneous durable states and required evidence. Stable
external tags and checked row conversion protect persistence. `Failed` contains
confirmed terminal failure with phase and category; it does not absorb an
unknown provider outcome.

## Local typed workflow

Draft construction validates money and account input. A policy service consumes
or claims a draft and returns `ValidatedPayment` after local/cross-entity checks.
An authorization worker receives an `AuthorizeWork` handle only after an
optimistic database claim. The provider response constructs
`AuthorizedPayment` with authorization ID, amount/currency binding, expiry, and
provider operation ID.

Capture is exposed only on the locally authorized handle:

```rust
impl AuthorizedPayment {
    pub async fn capture(self) -> CaptureOutcome {
        // Uses stable capture operation and provider idempotency key.
    }
}
```

The handle prevents ordinary capture-before-authorize code. It does not prove
the provider authorization remains valid at the moment of capture. The method
checks expiry and remains fallible.

## Capture outcomes

One `CaptureOperation` is persisted before dispatch with operation ID,
idempotency key, payment/version, amount/currency fingerprint, provider target,
and attempt budget. Every attempt reuses the key. The provider contract defines
key scope, binding, concurrency, replay, and retention.

```rust
pub enum CaptureOutcome {
    Confirmed(CaptureReceipt),
    Rejected(CaptureRejection),
    Unknown { reconciliation: CaptureReconciliation },
}
```

Failure proven before dispatch may retain safe retry under the same operation.
After possible dispatch, timeout or connection loss becomes `Unknown`. The
repository persists `CaptureUnknown` and blocks a new capture intent until
reconciliation or explicit risk acceptance.

The worker persists outcome and outbox event in one local transaction.
Publication retries may duplicate the event; event identity remains stable and
consumers deduplicate.

## Reconciliation

A bounded worker claims unknown records by version/lease and queries provider
status with the stable key. Authenticated evidence transitions to `Captured`,
`Failed` only for confirmed rejection/non-execution, or remains unknown. A
provider "not found" becomes rejection only under documented finality windows.
Old lease holders carry fencing/version tokens so stale updates fail.

User-facing status says capture confirmation pending and disables ordinary
repeat. Operators see age, attempts, last observation, and escalation owner.

## Settlement

Settlement events have stable event IDs, authenticated provider source,
payment/provider correlation, amount/currency, event version, and effective
time. An inbox claim and local transition occur atomically. Duplicate events
return the stored result. Out-of-order settlement before confirmed local capture
enters a reconciliation path; it is not discarded or blindly applied.

Settlement confirms the provider's settlement evidence under its contract. It
does not prove funds are irrevocable or immune to later dispute.

## Reversal

Reversal is a separate command authorized under current policy:

```text
Captured/Settled
    → reversal requested with stable operation ID
    → confirmed reversed | rejected | reversal unknown
```

It is never called rollback. Its amount may be full or partial under explicit
policy. It has its own idempotency, timeout, reconciliation, audit, and user
communication.

## Evidence

Compile-fail tests demonstrate capture cannot be called on a pre-authorization
handle. Unit tests cover money and each local transition. Database integration
tests validate row truth tables, optimistic claim conflicts, and outbox
atomicity. Fault tests inject response loss after authorization/capture,
process loss around status/outbox persistence, duplicate events, stale leases,
out-of-order settlement, and reversal ambiguity.

Property/state-machine tests generate legal commands and assert that confirmed
capture requires authorization evidence and unknown never becomes rejection
without new evidence.

## Guarantee ledger

| Claim                                      | Established by                                  | Protected construction       | Boundary preservation      | Escape hatches              | Does not prove                  | Residual runtime risk             |
| ------------------------------------------ | ----------------------------------------------- | ---------------------------- | -------------------------- | --------------------------- | ------------------------------- | --------------------------------- |
| amount is positive and currency-tagged     | `PositiveMoney` constructor                     | private fields               | DTO/row conversion         | audited repair              | balance, FX, fraud decision     | overflow/policy change            |
| local capture handle follows authorization | checked provider response plus repository claim | restricted typed constructor | handle not deserialized    | privileged provider adapter | authorization still valid later | expiry/revocation/provider outage |
| capture operation has stable identity      | durable creation before dispatch                | repository API               | reused across attempts     | manual operations           | provider honors idempotency     | retention expiry                  |
| capture is unknown rather than failed      | ambiguous failure mapping                       | explicit variant             | persisted reconciliation   | operator risk decision      | success or rejection            | provider evidence missing         |
| settled state has provider evidence        | authenticated deduplicated event                | checked transition           | inbox and row transaction  | audited backfill            | irrevocability                  | dispute/reversal                  |
| reversal confirmed                         | authenticated reversal receipt                  | separate operation state     | durable outcome conversion | manual provider action      | history erased                  | later provider correction         |
