# Normative doctrine

## RUST-DOC-0006-R001 — Do not equate timeout with non-execution

**Statement.** A timeout MUST NOT be represented as confirmed failure when the
remote operation may have executed.

**Intent.** Preserve the distinction between stopping local waiting and learning
remote outcome.

**Applicability.** Network requests, database commit, broker acknowledgement,
filesystem operations over remote mounts, and subprocess protocols.

**Allowed exceptions.** A timeout may be definitive only when protocol evidence
establishes that execution could not have begun or was atomically cancelled.

**Review evidence.** Protocol timeline, cancellation semantics, and explicit
unknown-outcome path.

**Enforcement.** [`examples/distributed-outcomes/src/lib.rs`](../../examples/distributed-outcomes/src/lib.rs)
— ExecutionAmbiguous routes to Unknown/reconcile

## RUST-DOC-0006-R002 — Model operationally distinct outcomes

**Statement.** Outcome types MUST distinguish confirmed success, confirmed
rejection, local failure before dispatch, and unknown outcome when callers
require different recovery.

**Intent.** Prevent transport symptoms from erasing domain knowledge.

**Applicability.** Consequential external operations.

**Allowed exceptions.** Categories may combine when no caller action, audit
meaning, security consequence, or reconciliation path differs.

**Review evidence.** Outcome decision table and exhaustive caller handling.

**Enforcement.** [`examples/distributed-outcomes/src/lib.rs`](../../examples/distributed-outcomes/src/lib.rs)
— Confirmed/Rejected/Unknown plus NotDispatched

## RUST-DOC-0006-R003 — Carry reconciliation evidence

**Statement.** An unknown outcome MUST carry or reference sufficient evidence
to reconcile it, including stable operation identity and the external target.

**Intent.** Make uncertainty actionable and auditable.

**Applicability.** Payments, messages, provisioning, commits, and any effect that
cannot safely be repeated blindly.

**Allowed exceptions.** An explicitly irreconcilable best-effort action may
retain only audit evidence if business policy accepts permanent uncertainty.

**Review evidence.** Reconciliation token, operation ID, request fingerprint,
target, attempt history, and observation method.

**Enforcement.** [`examples/distributed-outcomes/src/lib.rs`](../../examples/distributed-outcomes/src/lib.rs)
— ReconciliationToken carries OperationId

## RUST-DOC-0006-R004 — Analyze before retry

**Statement.** Every retry policy MUST classify the operation as safe to retry,
unsafe to retry, or reconcile-before-retry for each relevant failure point.

**Intent.** Prevent duplicate effects and unsafe assumptions.

**Applicability.** Clients, consumers, publishers, schedulers, and operator
runbooks.

**Allowed exceptions.** Pure reads may use a simpler safe-retry classification
when staleness and load remain documented.

**Review evidence.** Failure-point matrix, idempotency mechanism, deadline, and
attempt budget.

**Enforcement.** [`examples/distributed-outcomes/src/lib.rs`](../../examples/distributed-outcomes/src/lib.rs)
— decide_retry maps observation to retry posture

## RUST-DOC-0006-R005 — Define idempotency-key semantics

**Statement.** An idempotency key MUST have defined uniqueness, caller and
resource scope, payload binding, retention, concurrency, replay, and conflict
semantics.

**Intent.** Prevent a string field from being mistaken for idempotent behavior.

**Applicability.** Mutable external APIs and durable commands.

**Allowed exceptions.** Naturally idempotent operations may omit keys when their
semantic identity and repeated-result behavior are established independently.

**Review evidence.** Key contract, storage constraint, same-key/same-payload and
same-key/different-payload tests, and expiry policy.

**Enforcement.** Unenforceable: No artifact defines key scope, uniqueness, retention, payload
binding, or replay

## RUST-DOC-0006-R006 — Reuse operation identity across attempts

**Statement.** Retries of one logical operation MUST reuse its operation and
idempotency identity. A new identity MUST mean a new requested effect.

**Intent.** Allow receivers and reconcilers to distinguish replay from new
intent.

**Applicability.** External API requests, published commands, and repair tools.

**Allowed exceptions.** A protocol-mandated new transport attempt identifier may
be added, but it MUST remain correlated to the stable logical operation.

**Review evidence.** Identity lifecycle and attempt log.

**Enforcement.** [`examples/distributed-outcomes/src/lib.rs`](../../examples/distributed-outcomes/src/lib.rs)
— retries reuse the same operation and idempotency key

## RUST-DOC-0006-R007 — Expect duplicate delivery

**Statement.** Consumers in at-least-once systems MUST expect duplicate
delivery and MUST define whether repeated processing is deduplicated,
idempotent, commutative, or safely rejected.

**Intent.** Make acknowledgement loss and redelivery ordinary protocol paths.

**Applicability.** Brokers, job queues, webhook delivery, change feeds, and
replayed logs.

**Allowed exceptions.** A verified at-most-once boundary may accept loss instead
of duplicates, with that loss documented.

**Review evidence.** Duplicate test, stable message identity, and effect-level
handling.

**Enforcement.** Unenforceable: No consumer, broker, or duplicate-delivery path exists in any
example crate

## RUST-DOC-0006-R008 — Persist deduplication durably

**Statement.** Deduplication that protects a durable effect MUST itself use
durable state with atomic relationship to that effect, and MUST define
retention.

**Intent.** Prevent process restart or pruning from reopening duplicate effects.

**Applicability.** Consumer inboxes, payment commands, and webhook handlers.

**Allowed exceptions.** In-memory deduplication may protect only ephemeral
best-effort work whose duplicate cost is accepted.

**Review evidence.** Unique key, transaction boundary, retention calculation,
and replay-after-restart test.

**Enforcement.** Unenforceable: No durable store or transaction exists; examples avoid database and
broker dependencies

## RUST-DOC-0006-R009 — State ordering scope

**Statement.** Ordering claims MUST identify key or partition, producer set,
consumer concurrency, retry behavior, failover behavior, and observation point.

**Intent.** Prevent partition-local or producer-local order from becoming a
false global guarantee.

**Applicability.** Brokers, streams, event logs, RPC sequencing, and replication.

**Allowed exceptions.** None when business behavior relies on order.

**Review evidence.** Ordering contract and tests for retries, multiple
producers, and failover.

**Enforcement.** Unenforceable: No ordering, partition, producer-set, or consumer-concurrency model
exists

## RUST-DOC-0006-R010 — Qualify exactly-once claims

**Statement.** Any "exactly once" claim MUST identify the precise boundary,
identity, transactional mechanism, failure assumptions, retention, and effects
included. It MUST NOT imply exactly-once behavior beyond that boundary.

**Intent.** Replace a broad slogan with an auditable scoped guarantee.

**Applicability.** Messaging, stream processing, payments, jobs, and APIs.

**Allowed exceptions.** None.

**Review evidence.** Guarantee ledger, protocol documentation, duplicate tests,
and excluded effects.

**Enforcement.** Unenforceable: No exactly-once mechanism or transactional boundary is implemented
anywhere

## RUST-DOC-0006-R011 — Coordinate acknowledgement with effect

**Statement.** A consumer MUST define the order and atomic relationship among
effect execution, durable progress, and acknowledgement.

**Intent.** Make the duplicate-versus-loss tradeoff visible.

**Applicability.** Message and job consumers.

**Allowed exceptions.** Best-effort consumers may acknowledge early only when
loss is accepted and measured.

**Review evidence.** Crash-point matrix and tests before and after each durable
step.

**Enforcement.** Unenforceable: No consumer, acknowledgement, or durable-progress sequencing is
implemented

## RUST-DOC-0006-R012 — Treat compensation as a new effect

**Statement.** Sagas and compensating operations MUST NOT be described as
rollback. Each compensation MUST remain fallible, idempotency-analyzed, and
capable of an unknown outcome.

**Intent.** Preserve real-world irreversibility and changed conditions.

**Applicability.** Distributed workflows, reservations, payments, and
provisioning.

**Allowed exceptions.** A local database rollback may be called rollback within
its actual transaction boundary.

**Review evidence.** Forward/compensation pairs, business non-equivalence,
failure handling, and reconciliation.

**Enforcement.** Unenforceable: No saga, compensation, or forward/compensation pair exists in any
example crate

## RUST-DOC-0006-R013 — Treat observations as time-scoped evidence

**Statement.** External observations MUST record or imply their observation
time and MUST NOT be presented as immutable current truth when the external
state can change.

**Intent.** Prevent stale reads from becoming permanent authority.

**Applicability.** Status queries, authorization, inventory, leases, and
reconciliation.

**Allowed exceptions.** Immutable append-only facts may remain stable when the
source contract establishes immutability.

**Review evidence.** Freshness policy, version or timestamp, cache behavior, and
revalidation trigger.

**Enforcement.** Unenforceable: No timestamp, version, or freshness field exists on any outcome or
observation

## RUST-DOC-0006-R014 — Address concurrent execution and split brain

**Statement.** Where multiple workers or coordinators can act on one logical
operation, the design MUST address concurrent execution using ownership,
leases with fencing, compare-and-set state, consensus-backed leadership, or an
effect-level idempotency mechanism. When a lease, expiry, or deadline
contributes to that authority, the design MUST define the clock source, whether
elapsed or wall time is used, accepted clock-skew, process-pause, and
renewal-delay bounds, and behavior when any timing assumption fails.

**Intent.** Prevent stale owners and duplicate coordinators from acting with
equal authority, including after a timing assumption ceases to hold.

**Applicability.** Reconciliation workers, schedulers, failover, distributed
locks, leases, and other time-based authority.

**Allowed exceptions.** Concurrent execution is allowed for commutative,
duplicate-safe operations with evidence.

**Review evidence.** Authority protocol, expiry, fencing token use, clock source
and kind, quantified timing bounds, assumption-failure behavior, and overlap
test.

**Enforcement.** Unenforceable: No lease, fencing token, clock source, or concurrency backend exists

## RUST-DOC-0006-R015 — Bound retries and reconciliation

**Statement.** Retry and reconciliation loops MUST have bounded concurrency,
attempt or time budgets, backoff where appropriate, terminal escalation, and
observability.

**Intent.** Prevent uncertainty from turning into permanent load or hidden
backlog.

**Applicability.** Retry queues, reconcilers, publishers, and operator repair.

**Allowed exceptions.** A durable obligation may remain pending indefinitely,
but each execution cycle still requires bounded work and visible age.

**Review evidence.** Queue capacity, schedule, age metrics, dead-letter or
manual escalation, and overload test.

**Enforcement.** Unenforceable: No retry loop, attempt budget, backoff, queue, or escalation is
implemented

## RUST-DOC-0006-R016 — Preserve correlation and causality

**Statement.** Audit trails MUST preserve stable operation identity, attempt
identity, triggering event, parent correlation, request fingerprint, outcome
observations, and reconciliation decisions where these affect accountability.

**Intent.** Reconstruct what was requested, attempted, observed, and resolved.

**Applicability.** Consequential distributed effects.

**Allowed exceptions.** Low-risk telemetry may use aggregated correlation when
individual reconstruction is unnecessary.

**Review evidence.** Event schema, trace propagation, redaction, and end-to-end
incident query.

**Enforcement.** Unenforceable: No audit event schema, attempt log, trace propagation, or request
fingerprint exists

## RUST-DOC-0006-R017 — Protect sensitive reconciliation data

**Statement.** Reconciliation and audit evidence MUST contain enough identity
to act without unnecessarily storing credentials, secret payloads, or sensitive
personal data.

**Intent.** Avoid turning operational evidence into a second secret database.

**Applicability.** Operation logs, dead-letter records, tracing, and support
tools.

**Allowed exceptions.** Required regulated evidence may be retained with
documented access, encryption, minimization, and deletion policy.

**Review evidence.** Field classification, redaction tests, access policy, and
retention.

**Enforcement.** Unenforceable: No field classification, redaction, or sensitive-data handling
exists in any example

## RUST-DOC-0006-R018 — Test failure points, not only final errors

**Statement.** Distributed-effect tests MUST inject loss, delay, duplication,
reordering, concurrent execution, and crash points between durable steps in
proportion to risk.

**Intent.** Exercise ambiguity and replay paths hidden by happy-path mocks.

**Applicability.** Integrations, consumers, publishers, and reconcilers.

**Allowed exceptions.** A low-risk pure read may narrow the matrix and state
why.

**Review evidence.** Fault matrix linked to invariants, test results, and
unexercised assumptions.

**Enforcement.** Unenforceable: Tests inject no loss, delay, duplication, reordering, or crash
points

## RUST-DOC-0006-R019 — State residual uncertainty

**Statement.** Public and internal contracts MUST state which outcomes can
remain unknown, how long, who owns reconciliation, and what users or operators
may safely do meanwhile.

**Intent.** Make uncertainty an owned lifecycle state rather than an error
message.

**Applicability.** Every consequential effect with ambiguous execution.

**Allowed exceptions.** None.

**Review evidence.** State machine, service-level target, escalation path, and
guarantee ledger.

**Enforcement.** Unenforceable: Needs stated unknown-duration, reconciliation owner, operator
guidance; none carried
