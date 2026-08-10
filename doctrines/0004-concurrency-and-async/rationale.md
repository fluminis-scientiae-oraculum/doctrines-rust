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

## Cancellation is control flow

An async future may be dropped whenever its owner abandons it, a timeout wins,
or a selection chooses another branch. Drop is therefore an ordinary
control-flow edge. If an operation removed an item from a queue before waiting
to write it, cancellation may lose the item. If it sent an external request
before awaiting the response, cancellation cannot establish that the request
did not execute. If it acquired a permit through an RAII guard, drop may
correctly release local capacity, but it cannot undo an external effect.

Cancellation-safe does not mean infallible. It means that dropping the future
at the specified point does not violate its documented protocol.

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
component repairs or abandons it. In the Rust 1.97.1 documentation,
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
visibility, and future maintenance — not only source-line count.
