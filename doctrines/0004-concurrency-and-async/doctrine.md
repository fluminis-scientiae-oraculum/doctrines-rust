# Normative doctrine

## RUST-DOC-0004-R001 — Define the concurrency model

**Statement.** Every component with shared mutable state or overlapping work
MUST document its ownership and synchronization model.

**Intent.** Make custody, mutation authority, and coordination visible before
interleavings obscure them.

**Applicability.** Threads, async tasks, callbacks, actors, shared caches,
worker pools, and foreign callbacks.

**Allowed exceptions.** A leaf function using no shared state may rely on the
surrounding component contract.

**Review evidence.** An ownership map identifies state owner, permitted
mutators, synchronization primitive, task owner, and shutdown authority.

## RUST-DOC-0004-R002 — Protect invariants, not fields

**Statement.** Synchronization boundaries MUST cover the complete invariant
they protect. Related fields MUST NOT be independently locked when an operation
requires their values to change atomically as a group.

**Intent.** Prevent logically torn state even when every individual memory
access is race-free.

**Applicability.** Multi-field state, counters paired with collections, status
paired with resources, and cross-index data.

**Allowed exceptions.** Independent locks are permitted when the invariant
inventory proves the fields are independent or a documented reconciliation
protocol tolerates temporary divergence.

**Review evidence.** Invariant-to-lock mapping plus tests or model evidence for
multi-step updates.

## RUST-DOC-0004-R003 — Bound lock scope

**Statement.** Lock scope MUST be minimized to the protected operation and MUST
be documented on latency-sensitive or contention-sensitive paths. A blocking
call or `.await` MUST NOT occur while holding a synchronous lock unless a
specific correctness argument and bounded behavior justify it.

**Intent.** Reduce deadlock, convoying, executor blockage, and accidental
serialization.

**Applicability.** Mutexes, read-write locks, semaphore permits, and
transaction-like guards.

**Allowed exceptions.** A deliberately serialized critical section may include
a bounded non-suspending operation when splitting it would violate an
invariant.

**Review evidence.** Critical-section boundaries, timing assumptions, and
contention measurements where performance matters.

## RUST-DOC-0004-R004 — Review lock order and poisoning

**Statement.** Components that may acquire more than one lock MUST define a
global acquisition order or another deadlock-avoidance protocol. Lock poisoning
behavior MUST be chosen consciously rather than inherited without review.

**Intent.** Make cyclic waits and post-panic state handling explicit.

**Applicability.** Nested locks, callbacks under locks, standard-library mutexes,
and libraries with different poisoning semantics.

**Allowed exceptions.** A proof that locks cannot overlap may replace a global
order.

**Review evidence.** Lock graph, callback analysis, and documented recovery,
fail-stop, or invariant-rebuild policy after panic.

## RUST-DOC-0004-R005 — Isolate blocking work

**Statement.** Potentially blocking filesystem, process, DNS, compression,
cryptographic, database, or CPU-intensive work MUST NOT run on async executor
worker threads without deliberate isolation.

**Intent.** Preserve progress for unrelated tasks and keep runtime scheduling
assumptions honest.

**Applicability.** Async services and libraries running work whose latency is
not cooperatively yielded.

**Allowed exceptions.** Operations proven bounded below the component's
documented scheduling budget may remain inline.

**Review evidence.** Classification of blocking calls, isolation mechanism,
pool capacity, cancellation behavior, and overload limits.

## RUST-DOC-0004-R006 — Analyze cancellation at every suspension point

**Statement.** Every `.await` or equivalent suspension point inside a partial
operation MUST be reviewed for cancellation safety.

**Intent.** Prevent abandoned state mutations, lost data, leaked authority, and
unobserved external effects when a future is dropped.

**Applicability.** Select loops, timeouts, request handlers, retries, and
multi-stage operations.

**Allowed exceptions.** A suspension point may be classified cancellation-safe
when dropping the future cannot lose progress or violate an invariant; the
classification still requires evidence.

**Review evidence.** A cancellation table showing state before suspension,
drop effect, cleanup owner, resumability, and external uncertainty.

## RUST-DOC-0004-R007 — Define cancellation cleanup

**Statement.** Resources and partial state created before a cancellable
suspension MUST have a defined cleanup, commit, compensation, or reconciliation
path.

**Intent.** Ensure that future destruction is part of the protocol instead of
an invisible control-flow edge.

**Applicability.** Locks, permits, temporary files, transactions, leases,
messages, and external requests.

**Allowed exceptions.** Abandonment is permitted only when the resource is
designed to expire or be collected and the delay and capacity consequences are
acceptable.

**Review evidence.** Drop behavior, explicit cleanup calls, expiry bounds,
reconciliation identifiers, and cancellation tests.

## RUST-DOC-0004-R008 — Bound concurrency

**Statement.** Concurrency SHOULD be bounded by a reviewed resource limit.
Unbounded task spawning requires explicit justification and an overload
analysis.

**Intent.** Prevent memory growth, connection exhaustion, scheduler collapse,
and downstream overload.

**Applicability.** Per-request tasks, batch fan-out, consumer loops, and retry
workers.

**Allowed exceptions.** A finite, statically bounded input set may establish a
safe upper bound without a runtime semaphore.

**Review evidence.** Capacity source, queue bound, rejection or waiting policy,
and stress evidence at and above capacity.

## RUST-DOC-0004-R009 — Make backpressure explicit

**Statement.** Producers and consumers MUST define what happens when demand
exceeds service capacity: wait, reject, shed, coalesce, persist, or degrade.

**Intent.** Replace accidental memory buffering with an operational contract.

**Applicability.** Channels, queues, streams, batching, connection pools, and
API ingress.

**Allowed exceptions.** None for an open-ended producer. A fixed small batch
may document its finite bound.

**Review evidence.** Capacity values, overflow behavior, fairness, metrics, and
caller-visible failure semantics.

## RUST-DOC-0004-R010 — Handle channel closure

**Statement.** Send and receive paths MUST handle channel closure as a normal
protocol event. Closure MUST NOT be converted silently into an endless retry,
busy loop, or fabricated success.

**Intent.** Make owner departure, shutdown, and worker failure observable.

**Applicability.** Bounded and unbounded channels, watch streams, broadcast
channels, and actor mailboxes.

**Allowed exceptions.** A process-terminating invariant breach may escalate
closure after recording adequate context.

**Review evidence.** Closure branches, sender/receiver ownership, drain policy,
and tests for last-sender and receiver-drop behavior.

## RUST-DOC-0004-R011 — Structure task ownership

**Statement.** Every spawned task MUST have an owner responsible for observing
completion, failure, cancellation, and shutdown.

**Intent.** Prevent invisible task failure and work that outlives its authority
or dependencies.

**Applicability.** Runtime tasks, threads, worker pools, and background
maintenance loops.

**Allowed exceptions.** Process-lifetime infrastructure may be supervised by a
top-level owner rather than joined by the immediate caller.

**Review evidence.** Task tree, join or supervision strategy, failure
propagation, restart limits, and shutdown trigger.

## RUST-DOC-0004-R012 — Restrict detached tasks

**Statement.** Detached tasks MUST be exceptional, named, observable, bounded,
and documented with their process-lifetime contract.

**Intent.** Avoid fire-and-forget work whose success, failure, or resource use
cannot be accounted for.

**Applicability.** Telemetry flushers, cache refreshers, cleanup work, and
best-effort notifications.

**Allowed exceptions.** A deliberately lossy best-effort action may detach if
loss is acceptable, resource use is bounded, and failures are measured.

**Review evidence.** Owner rationale, task name, metrics, capacity, panic
handling, and termination behavior.

## RUST-DOC-0004-R013 — Define graceful shutdown

**Statement.** Concurrent services MUST define admission stop, cancellation,
queue drain, resource release, deadline, forced termination, and observability
semantics for shutdown.

**Intent.** Turn shutdown into a tested lifecycle transition.

**Applicability.** Services, consumers, worker pools, and long-running tools.

**Allowed exceptions.** Short-lived pure computations may rely on process
completion when they own no persistent or external effect.

**Review evidence.** Ordered shutdown procedure, time budget, outstanding-work
accounting, and tests for idle and loaded shutdown.

## RUST-DOC-0004-R014 — State ordering guarantees precisely

**Statement.** Ordering claims MUST identify their scope, key, producer set,
buffering boundary, and behavior during retry or failover.

**Intent.** Prevent local FIFO behavior from being described as global order.

**Applicability.** Channels, brokers, actor mailboxes, logs, and concurrent
state updates.

**Allowed exceptions.** None when callers depend on order.

**Review evidence.** Ordering contract plus tests that include multiple
producers, retries, and closure where relevant.

## RUST-DOC-0004-R015 — Justify atomic ordering

**Statement.** Every nontrivial atomic operation MUST document why its memory
ordering is sufficient for the associated synchronization invariant.

**Intent.** Prevent atomics from becoming unexplained race-free but incorrect
protocols.

**Applicability.** `Atomic*`, fences, lock-free structures, and unsafe
concurrency.

**Allowed exceptions.** A simple standalone statistics counter may use relaxed
ordering when no other memory is synchronized through it.

**Review evidence.** Happens-before argument, invariant, permitted
interleavings, Loom or equivalent model evidence where tractable, and
RUST-DOC-0007 review if unsafe code is present.

## RUST-DOC-0004-R016 — Preserve failure and ordering through supervision

**Statement.** Task supervision MUST distinguish normal completion,
cancellation, panic, retryable failure, permanent failure, and exhausted
restart policy when those outcomes require different action.

**Intent.** Prevent restart loops and silent partial service.

**Applicability.** Actors, consumers, background workers, and service task
trees.

**Allowed exceptions.** Outcomes may be combined only when no caller or
operator acts differently and diagnostic evidence remains adequate.

**Review evidence.** Supervision decision table, restart budget, backoff,
jitter, terminal-state reporting, and panic policy.

## RUST-DOC-0004-R017 — Review async abstraction costs

**Statement.** Async traits, boxed futures, dynamic dispatch, and generated
state machines MUST be evaluated for allocation, object-safety, API stability,
diagnostic, and monomorphization tradeoffs.

**Intent.** Keep async abstraction proportional to actual polymorphism needs.

**Applicability.** Public traits, plugin boundaries, high-volume paths, and
generic middleware.

**Allowed exceptions.** Local low-volume code may choose the clearest interface
without benchmark evidence when its cost is immaterial.

**Review evidence.** Required dispatch mode, allocation expectations, public
API consequences, and measurements for performance claims.

## RUST-DOC-0004-R018 — Coordinate timeouts and retries

**Statement.** Timeout and retry layers MUST be inventoried end to end.
Independent layers MUST NOT multiply attempts or synchronize retries without a
documented load and idempotency analysis.

**Intent.** Prevent retry storms, thundering herds, duplicated effects, and
latency that exceeds caller budgets.

**Applicability.** Clients, middleware, proxies, services, databases, brokers,
and supervisors.

**Allowed exceptions.** Nested retries may exist when attempt budgets compose
within one deadline and each layer has distinct, proven safe semantics.

**Review evidence.** Attempt equation, total deadline, backoff and jitter,
idempotency classification, downstream capacity, and unknown-outcome handling.

## RUST-DOC-0004-R019 — Separate concurrency safety from external correctness

**Statement.** A race-free local transition MUST NOT be claimed to establish
remote liveness, durable completion, unique execution, or current external
state.

**Intent.** Keep local synchronization evidence distinct from mutable external
reality.

**Applicability.** Network connections, acknowledgements, leases, distributed
locks, and database commits.

**Allowed exceptions.** None.

**Review evidence.** Guarantee ledger identifying local proof, observation
time, runtime failures, timeout ambiguity, and reconciliation path.

## RUST-DOC-0004-R020 — Test adverse schedules and overload

**Statement.** Evidence for a consequential concurrent protocol MUST include
adverse scheduling, closure, cancellation, overload, and shutdown behavior,
using model checking or fault control when ordinary tests cannot reliably
exercise the interleavings.

**Intent.** Test the protocol edges that happy-path scheduling conceals.

**Applicability.** Shared-state protocols, supervisors, queues, and
cancellation-sensitive operations.

**Allowed exceptions.** Trivial immutable parallel computation may document why
these hazards are absent.

**Review evidence.** Invariant-linked tests, stress or model results, failure
injection, and known evidence limits.
