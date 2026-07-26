# Glossary

**Actor ownership**
: A model in which one task owns mutable state and other tasks request changes
through messages. It simplifies mutation authority but still requires mailbox
capacity, supervision, and shutdown semantics.

**Backpressure**
: A policy that makes downstream capacity constrain upstream production through
waiting, rejection, shedding, coalescing, or durable buffering.

**Cancellation safety**
: The property that dropping an incomplete future at a stated suspension point
does not violate its documented invariant or lose unrecoverable progress.

**Channel closure**
: The protocol event produced when the relevant sender or receiver set has
disappeared. Meaning depends on channel kind and component lifecycle.

**Convoying**
: Delayed progress caused when many operations serialize behind a slow holder or
resource.

**Detached task**
: A task whose completion is not structurally awaited by the immediate caller.
It still requires an operational owner.

**Graceful shutdown**
: A bounded lifecycle protocol that stops admission, accounts for outstanding
work, releases resources, and defines behavior at its deadline.

**Happens-before**
: A synchronization relationship used to reason that memory effects become
observable in a required order. It is stronger and more precise than
wall-clock intuition.

**Lock graph**
: A directed graph in which an edge represents acquiring one lock while holding
another. A cycle indicates a potential deadlock protocol.

**Lock poisoning**
: A mechanism that records that a panic occurred while exclusive state was
guarded. It signals possible invariant damage; recovery policy remains a
component decision.

**Retry amplification**
: Multiplication of attempts when independent layers retry one logical
operation.

**Structured concurrency**
: Task lifecycle organization in which child work has an accountable owner and
bounded relationship to parent completion, failure, and cancellation.

**Task supervision**
: Observation and policy for task completion, panic, cancellation, failure,
restart, and terminal degradation.

**Thundering herd**
: A synchronized group of waiters or retries that simultaneously contend for a
recovering resource.

**Unknown outcome**
: A state in which local evidence cannot determine whether an external effect
occurred. It requires reconciliation rather than assumed rejection.
