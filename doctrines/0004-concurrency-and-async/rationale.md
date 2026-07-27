# Rationale

## Memory-race freedom is one layer

Rust's ownership rules and `Send`/`Sync` traits reject important classes of
unsafe sharing. They do not decide whether the program chose the right
synchronization boundary. Two fields can each be protected correctly while
their relationship is observed in an impossible combination. A channel can be
memory-safe while its unbounded queue consumes all memory. A mutex can be
correctly implemented while two call paths acquire locks in opposite order.
An async task can compile while never yielding during a long computation.

The doctrine therefore begins with a protocol description: state ownership,
mutation authority, synchronization, task custody, queue capacity, and
shutdown. Primitives are selected after the invariant is known.

## Ownership model choices

Single-owner designs remove many interleavings. An actor or dedicated worker
can own mutable state and accept commands through a bounded channel. This makes
mutation order local, but introduces mailbox capacity, response cancellation,
owner failure, and shutdown questions. Shared-lock designs can reduce message
overhead and allow direct reads, but require a lock graph and careful critical
sections. Immutable snapshots can simplify reads while making update and memory
retention costs visible. Atomics can serve narrow protocols, but their compact
syntax hides a demanding memory-order proof.

No model is uniformly superior. The correct question is which model makes the
important invariant and overload behavior easiest to establish and audit.

## Cancellation is control flow

An async future may be dropped whenever its owner abandons it, a timeout wins,
or a selection chooses another branch. Drop is therefore an ordinary
control-flow edge. If an operation removed an item from a queue before waiting
to write it, cancellation may lose the item. If it sent an external request
before awaiting the response, cancellation cannot establish that the request
did not execute. If it acquired a permit through an RAII guard, drop may
correctly release local capacity, but it cannot undo an external effect.

Cancellation analysis records:

| Question                               | Required answer                                       |
| -------------------------------------- | ----------------------------------------------------- |
| What changed before suspension?        | local and external mutations                          |
| What happens if the future is dropped? | destructor, abandonment, or no action                 |
| Who owns recovery?                     | current task, supervisor, lease expiry, or reconciler |
| Can the operation resume safely?       | cursor, transaction, or idempotency evidence          |
| Can success be unknown?                | explicit reconciliation state and identity            |

Cancellation-safe does not mean infallible. It means that dropping the future
at the specified point does not violate its documented protocol.

## Backpressure is part of the API

When producers can outpace consumers, some resource accumulates: memory,
threads, file descriptors, database rows, broker depth, or caller latency.
Calling a channel "internal" does not remove this fact. A bounded channel makes
capacity observable, but the choice at capacity still matters. Waiting
propagates pressure upstream. Rejection preserves the service but requires a
caller policy. Shedding or coalescing is valid only when the lost distinctions
are unimportant. Persisting creates a durable queue with its own retention and
replay contract.

Capacity should derive from resource budgets and service objectives rather than
an arbitrary large number. Stress evidence must include behavior after the
limit, not only throughput before it.

## Structured task ownership

A spawned task creates a lifecycle. Someone must observe its completion and
failure, decide whether to cancel siblings, and stop it during shutdown. Merely
retaining a runtime handle is insufficient if the handle is never awaited or
supervised. Structured ownership forms a task tree whose children do not
silently outlive the authority, configuration, or resources of their parent.

Some process-lifetime tasks cannot be lexically scoped to a request. They still
need a top-level supervisor, name, health signal, restart budget, and shutdown
path. Detachment describes an implementation relationship; it must not erase
operational accountability.

## Locks and suspension

Holding a synchronous lock across `.await` can block an executor worker and
allow the suspended task to retain exclusive access for an unbounded duration.
Even an async-aware mutex can create long convoys or deadlocks if callbacks and
other resources form a cycle. The appropriate remedy is usually to extract the
needed state, release the guard, perform the fallible or slow work, then
reacquire and validate that assumptions still hold. Sometimes the invariant
requires serialization across the slow operation; then a dedicated owner task
or explicit operation queue often expresses the design better.

Poisoning is also policy, not proof. A panic while holding a standard-library
mutex marks possible invariant damage. Blind recovery may expose corrupt state;
blind termination may be excessive when state can be rebuilt. The component
must choose.

Choosing a non-poisoning lock removes the poison signal, not the possibility
that a panic interrupted a multi-step invariant update. The same review must
therefore define whether unwinding can expose partial state and how the
component repairs or abandons it. In the pinned Rust 1.97.1 documentation,
`std::sync::nonpoison` is present but nightly-only and experimental; consumers
must check the documentation for their actual toolchain rather than infer
stability from the namespace.

## Blocking isolation is bounded too

Moving blocking work to a blocking pool protects async workers only if that pool
has capacity, admission control, and cancellation semantics. CPU-heavy work can
still saturate all cores. Blocking calls may not stop when their async wrapper
is cancelled. A detached blocking job can continue consuming resources after
the request disappears. Isolation moves contention to an explicit subsystem; it
does not delete it.

## Retry amplification

Suppose a client tries three times, a proxy tries twice, and the service worker
tries four times. One logical request can create twenty-four downstream
attempts, before broker or database retries are counted. If every layer starts
retrying after a common timeout, the load spike becomes synchronized. Backoff,
jitter, deadlines, retry budgets, and idempotency must be designed as one
system.

Retry safety depends on operation semantics. A transport error before a response
does not reveal whether the remote effect occurred. RUST-DOC-0006 governs the
resulting unknown outcome and reconciliation. Concurrency limits without retry
coordination can still admit a sustained overload loop.

## Ordering claims

A single channel receiver may observe messages from each individual sender in
order while interleaving multiple senders nondeterministically. A broker may
order within a partition but not across partitions. Restart, redelivery, retry,
and parallel consumption can change visible order. An ordering contract must
therefore name the key, producer set, scope, and exceptional behavior.

Global order is expensive and often unnecessary. Per-aggregate sequencing or
commutative operations may provide the actual business guarantee with less
coordination.

## Atomics and proof cost

Atomicity prevents torn access to the atomic value. Memory ordering determines
how that access relates to other memory. A relaxed counter is suitable for
independent telemetry because it does not publish other data. A readiness flag
that publishes initialized memory needs a happens-before relationship.
Copying an ordering from nearby code is not an argument.

Small lock-free protocols benefit from model checking because ordinary tests
sample schedules. Unsafe lock-free code also inherits the proof obligations of
RUST-DOC-0007. A mutex is often the lower-risk choice when measured contention
does not justify the atomic protocol.

## Complexity and operational truth

Async traits, boxed futures, generic middleware, and elaborate actor systems can
improve composition, but they add allocation, diagnostics, dynamic dispatch,
monomorphization, and lifecycle complexity. The type system cannot encode every
schedule, queue depth, deadline, or external state. Runtime metrics and
supervision remain necessary.

The simplest correct design may be sequential code with a bounded worker pool.
Concurrency is justified by workload and latency evidence, not by language
capability. Its complexity budget includes failure analysis, tests, operator
visibility, and future maintenance—not only source-line count.
