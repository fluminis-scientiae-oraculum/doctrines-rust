# Invoice: problem

## Domain

An invoice belongs to one account and is addressed to a recipient. It carries a
positive amount in one explicit currency. Its lifecycle is:

- pending and eligible for delivery;
- paid with a receipt;
- failed with a structured reason.

Delivery is an external email-provider effect. Provider acceptance can fail, be
confirmed, or become unknown after the request may have executed but its
acknowledgement is lost. Retrying can send a duplicate message. An accepted
address is syntax-level evidence; mailbox ownership and deliverability require
different evidence.

## Invariants

| ID | Statement | Classification | Consequence |
|---|---|---|---|
| INV-01 | Invoice state is exactly pending, paid-with-receipt, or failed-with-reason. | state | contradictory business decisions |
| INV-02 | Amount is nonzero minor units with an explicit currency. | value | zero invoice or currency mixing |
| INV-03 | Same-currency checked arithmetic is required for totals. | transition/value | overflow or invalid total |
| INV-04 | Recipient passes the documented syntax policy. | boundary/value | malformed provider request |
| INV-05 | Only pending invoices may begin ordinary delivery. | transition | duplicate or misleading notification |
| INV-06 | Delivery operation identity remains stable across retries. | distributed | duplicate external effect |
| INV-07 | Timeout after dispatch becomes unknown, not confirmed failure. | distributed | unsafe repeat |
| INV-08 | Invoice state change and durable delivery intent cannot be silently separated. | persistence | forgotten or fictional send |

Tax calculation, currency conversion, allocation, legal issuance requirements,
account balance, mailbox ownership, and remote deliverability are outside the
`PositiveMoney` and `EmailAddress` value invariants.

## Boundaries

An HTTP command carries account/invoice IDs, amount, currency, recipient text,
and optional idempotency identity. Authentication yields a principal;
authorization determines whether the principal may issue or send for the
account. DTO parsing does not grant authority.

Database rows may contain historical zero amounts, unknown currency codes,
invalid state/receipt combinations, or a recipient accepted under old policy.
They decode through a raw row and current domain conversion. Invalid history is
quarantined rather than forged into `Invoice`.

The provider boundary receives a stable delivery operation ID. Provider
response is external evidence. Network timeout, cancellation, and process loss
can occur before dispatch, after dispatch, after provider acceptance, after
local outcome persistence, or before message acknowledgement reaches an API
caller.

## Required outputs

The design must provide domain types, structured construction/transition errors,
durable invoice and delivery states, an outbox or equivalent intent mechanism,
retry decisions, reconciliation, bounded workers, and evidence for both
forbidden programs and runtime failure. The UI must distinguish queued,
confirmed provider acceptance, rejected, and delivery unknown without calling
accepted mail delivered.
