# RUST-DOC-0006 source notes

## Protocol and foundational sources

[RFC 9110](https://www.rfc-editor.org/rfc/rfc9110.html#name-idempotent-methods)
defines HTTP idempotent methods in terms of intended effect, illustrating that
idempotency is semantic rather than a retry-library label. Apache Kafka's
[delivery semantics documentation](https://kafka.apache.org/documentation/#semantics)
describes at-most-once, at-least-once, and transaction-scoped exactly-once
mechanisms.

Stripe's
[idempotent requests documentation](https://docs.stripe.com/api/idempotent_requests)
is a concrete API contract covering key retention, request comparison, and
response replay. Amazon's
[Making retries safe with idempotent APIs](https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/)
explains production operation identity. Gray and Cheriton's
["Leases" paper](https://dl.acm.org/doi/10.1145/74850.74870) supplies
foundational time-bounded authority context.

## Accepted ideas

The doctrine accepts that communication failure can be independent of remote
execution, at-least-once delivery entails duplicates, idempotent operations and
deduplication require stable identity, and leases are time-scoped authority.
Transaction-scoped exactly-once mechanisms can provide valuable guarantees
within their actual boundary.

Retry requires a deadline and load policy. Duplicate consumers need durable
identity when protecting durable effects. Ordering must be scoped to the
mechanism that supplies it.

## Refined ideas

"Timeout is failure" is refined into a failure-point analysis. A timeout means
the caller stopped waiting; execution is unknown whenever the request may have
crossed the effect boundary.

"Use an idempotency key" is refined into scope, uniqueness, payload binding,
concurrent-call behavior, response replay, retention, and expiry. A key string
without receiver semantics proves nothing.

"Exactly once" is retained only as a boundary-specific claim naming identity,
transaction, effects, failure assumptions, and retention. It is rejected as an
end-to-end slogan.

Leases are refined with stale-owner analysis and fencing at the protected
resource. Local ownership of a lease handle cannot revoke remote writes already
accepted. Time-based authority additionally names its clock source, clock kind,
skew/pause/renewal bounds, safety margin, and behavior after an assumption
fails.

## Rejected ideas

The doctrine rejects retrying every transport error, generating a new key per
attempt, in-memory deduplication for durable effects, global ordering inferred
from a FIFO channel/partition, and compensation described as rollback. It
rejects absence in a lagging observation as immediate proof of non-execution.

## Repository additions

The repository adds a required confirmed/rejected/local-failure/unknown
decision, durable reconciliation evidence, unknown-state ownership and age,
operator-decision audit, compensation uncertainty, sensitive reconciliation
data minimization, effect failure-point tests, an enforceable time-authority
contract, and a sixty-gate distributed review.

## Source-to-rule application

The outcome rules turn communication ambiguity into domain states. Idempotency
rules use vendor examples only as evidence of dimensions every concrete
contract must specify; they do not impose Stripe's exact retention or response
behavior on other APIs. Duplicate and order rules use broker semantics as
scoped examples. Lease rules require effect-level fencing when stale owners can
harm, a stronger operational qualification than local handle ownership. R014
makes the clock and failure assumptions enforceable whenever time contributes
to authority.

Reconciliation is repository-added lifecycle governance: stable identity,
request fingerprint, authoritative observation, freshness, repeated unknown,
bounded attempts, escalation, and audit. It is not described as consensus or a
proof the source will eventually answer.

## Maintenance triggers

Recheck provider idempotency, retention, finality, query, and webhook contracts
when API versions change. Re-evaluate replay horizons when broker retention or
operator replay changes. New proxies, retries, regions, coordinators, or
compensations expand the effect timeline and can invalidate a prior retry
classification. Recheck lease contracts when clock sources, pause behavior,
renewal cadence, failover timing, or fencing support changes.

> [!TIP]
> [attribution](attribution.md) · **source notes**
> Index: [all source packages](../README.md).
> Doctrine: [`doctrines/0006-distributed-uncertainty/`](../../doctrines/0006-distributed-uncertainty/README.md).
