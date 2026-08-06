# Invoice: remaining uncertainty

## Email evidence

Syntax acceptance does not establish mailbox ownership, current deliverability,
recipient consent, absence of suppression, provider policy acceptance, or human
receipt. Even a verification event proves only the process and time represented
by its proof. Domains expire, mailboxes change owners, and provider reputation
changes. The invoice must not call an address permanently deliverable.

Provider acceptance means the provider acknowledged the request under its
contract. It can later bounce, defer, suppress, or deliver to a spam folder.
Delivery events can be delayed, duplicated, missing, or forged unless their
source is authenticated. The system needs separate event IDs and monotonic
status policy if these later states matter.

## Acknowledgement and duplicates

If the provider idempotency store expires before a delayed application replay,
the same key may execute again. Retention must cover retry and operational
replay horizons; after expiry, automation may need reconciliation or human
approval. A provider can also violate its advertised semantics. Audit and
support procedures therefore retain provider receipt, operation ID, attempts,
and request fingerprint.

The outbox prevents a committed local intent from being silently forgotten. It
does not produce exactly-once provider calls. Publisher crashes can repeat
delivery. The provider or an effect-aware deduplication mechanism absorbs
repetition only within its specified scope.

## Money and business policy

Positive integer minor units avoid binary floating-point representation error
and zero. They do not decide tax basis, invoice rounding, percentage allocation,
currency exponent changes, FX, refunds, credit notes, or legal correctness.
Those policies need explicit versioned services and evidence. Adding two
different currencies remains an error rather than an implicit conversion.

Database constraints can prevent zero rows and contradictory storage shapes,
but alternate writers and migrations can still introduce invalid history.
Quarantine protects domain meaning at an availability cost. Operators must own
repair and assess whether partial reads are safe.

## Concurrency and state freshness

A loaded pending invoice is an observation at version `n`. Another actor can
pay, fail, or request delivery before the current operation commits. Optimistic
conflict prevents silent overwrite but does not guarantee callers will choose a
safe retry. Reauthorization may be required after conflict because principal or
account policy can change.

The local `SendInvoicePermit` proves issuance under observed policy; it does not
freeze revocation or remote account state. Short lifetime and resource/version
binding reduce stale authority. The persistence transition remains the final
local arbiter.

## Operational ownership

Unknown delivery can remain unresolved if the provider offers no query, loses
records, or is unavailable past retention. The system must define age targets,
customer language, manual evidence, and a terminal policy. An operator decision
to send again is a new authorized action with duplicate risk, not proof the
first attempt failed.

Telemetry may show queue lag, retries, confirmed responses, bounce rates, and
unknown age. Absence of alerts does not establish completeness. Reconciliation
and dead-letter paths require their own capacity, secrecy, and recovery tests.

## Final statement

The improved model eliminates contradictory invoice states, centralizes
value validation, protects local delivery intent, and prevents timeout from
becoming fictional failure. It deliberately leaves remote delivery, evolving
business policy, concurrent reality, and permanently missing evidence as
runtime concerns with explicit owners.

> [!TIP]
> [problem](problem.md) · [naive design](naive.md) · [improved design](improved.md) · **remaining uncertainty**
> Index: [all case studies](../README.md).
> Executable mechanics live under [`../../examples/`](../../examples/).
