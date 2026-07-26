# Invoice: improved design

## Domain values

```rust
pub enum Currency {
    Idr,
    Usd,
}

pub struct PositiveMoney {
    minor_units: NonZeroU64,
    currency: Currency,
}

pub struct EmailAddress(String);

pub enum InvoiceState {
    Pending,
    Paid { receipt: ReceiptId },
    Failed { reason: InvoiceFailure },
}

pub struct Invoice {
    id: InvoiceId,
    account: AccountId,
    recipient: EmailAddress,
    amount: PositiveMoney,
    state: InvoiceState,
    version: u64,
}
```

Fields are private. `PositiveMoney::new` rejects zero and accepts a defined
currency. `checked_add` rejects currency mismatch and overflow.
`EmailAddress::parse` enforces the documented example syntax and length policy;
it does not claim ownership or deliverability. Invoice state is one variant with
its required data.

Request parsing builds an `IssueInvoiceDto`, validates value types, authenticates
the principal, and authorizes account issuance. A domain service checks
cross-entity policy in a database transaction, creates the invoice, and returns
structured validation, authorization, conflict, or persistence errors.

## Delivery protocol

Delivery has its own runtime state:

```rust
pub enum DeliveryStatus {
    Queued { operation_id: OperationId },
    ConfirmedAccepted { provider_receipt: ProviderReceipt },
    Rejected { reason: DeliveryRejection },
    Unknown { reconciliation: ReconciliationToken },
}
```

`request_delivery` requires a pending invoice and a local `SendInvoicePermit`
issued by the authorization service. Within one database transaction it:

1. checks invoice version and eligibility;
2. allocates one stable operation ID and scoped idempotency key;
3. inserts a delivery row in `Queued`;
4. inserts an outbox command with the same identity;
5. commits.

The database transaction establishes durable invoice/delivery intent. It does
not send email. A bounded publisher reads outbox rows, claims work with version
or lease protection, and sends the provider request using the stable key. Every
transport attempt records an attempt ID under the logical operation.

Provider outcomes map as follows:

| Observation                                | State                              |
| ------------------------------------------ | ---------------------------------- |
| authenticated accepted response            | `ConfirmedAccepted`                |
| authenticated permanent rejection          | `Rejected`                         |
| failure proven before dispatch             | queued with bounded retry guidance |
| timeout/disconnect after possible dispatch | `Unknown`                          |

The worker persists the observed outcome and acknowledgement progress. Repeated
outbox delivery reuses the same provider key. The provider idempotency contract
defines scope, payload binding, concurrent same-key calls, response replay, and
retention. If the provider lacks useful idempotency, unknown outcomes must
reconcile before a repeat.

## Reconciliation

`Unknown` includes operation ID, provider key, request fingerprint, first and
last attempt time, and next observation. A bounded reconciler queries provider
status or consumes authenticated provider events. It may transition to
confirmed accepted, rejected, or still unknown. Absence is final only if the
provider contract makes it final after a defined window.

The UI calls confirmed provider acceptance "accepted by provider," not
"delivered." Bounce and mailbox-delivery evidence would be later events with
their own limitations.

## Persistence conversion

`InvoiceRow` and `DeliveryRow` preserve raw nullability and tags. `TryFrom`
validates money, currency, recipient, state truth tables, versions, and
reconciliation fields. Schema checks reinforce nonzero amount and tag/payload
combinations. Invalid historical records enter a quarantine workflow with
record ID and redacted diagnostics.

Optimistic version predicates prevent two send commands from silently creating
independent operations for one policy-limited invoice. A conflict becomes a
structured result, not generic internal failure.

## Evidence

- unit tests cover money construction, currency mismatch, invoice variants,
  email rejection, and transition policy;
- property tests cover checked same-currency addition within bounded domains;
- compile-fail evidence protects direct stronger-type construction where used;
- database integration tests load invalid state combinations and verify
  quarantine;
- concurrent tests issue two delivery requests and establish one accepted
  operation or explicit conflict;
- fault tests stop before dispatch, after dispatch, after provider acceptance,
  and before local persistence;
- duplicate tests reuse one key;
- reconciliation tests remain unknown before final evidence and then resolve;
- overload tests verify bounded publisher and queue behavior.

## Guarantee ledger

| Claim                                          | Established by                                   | Protected construction         | Boundary preservation                 | Escape hatches                             | Does not prove                        | Residual runtime risk       |
| ---------------------------------------------- | ------------------------------------------------ | ------------------------------ | ------------------------------------- | ------------------------------------------ | ------------------------------------- | --------------------------- |
| `PositiveMoney` is nonzero in one currency     | `NonZeroU64` plus `Currency` constructor         | private fields                 | DTO/row `TryFrom`                     | audited migration repair                   | tax, FX, allocation, sufficient funds | overflow and policy changes |
| `EmailAddress` satisfies syntax policy         | checked parser                                   | private string                 | every DTO/row decode calls parser     | quarantine tool cannot issue trusted value | ownership, future deliverability      | policy evolution            |
| invoice has one legal state shape              | enum                                             | private aggregate construction | row truth-table conversion            | privileged repair                          | transition authorization/history      | concurrent persisted change |
| delivery intent is durable with invoice update | outbox and state in one DB transaction           | repository API                 | publisher consumes operation identity | direct DB administration                   | provider execution                    | DB availability             |
| provider acceptance was confirmed              | authenticated provider response/reconciled event | checked outcome transition     | receipt persisted                     | audited operator resolution                | inbox delivery or future bounce       | provider reversal/event lag |
| unknown has reconciliation identity            | explicit state constructor                       | required fields                | durable row conversion                | destructive raw DB edit                    | whether email was accepted            | provider observation outage |
