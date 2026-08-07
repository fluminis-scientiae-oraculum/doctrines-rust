# Rationale

Rust ownership is a strong vocabulary for custody: a value has an owner, can be moved, can be
borrowed, and is dropped. Domain design can align those facts with exclusive authority or a
single lifecycle. A transaction guard consumed by `commit(self)` cannot be committed again
through the same value. A shutdown permit moved to a supervisor identifies who may initiate
shutdown. A single-use token without `Clone` makes local duplication unavailable.

This alignment is useful only when the domain is actually exclusive and local. A process-local
`FileLock` handle records that an operating-system lock acquisition succeeded and that this
handle owns release. It does not prove another host follows the same locking convention or
that a network filesystem implements expected semantics. A distributed lease needs identity,
expiry, fencing, clock assumptions, and server enforcement.

Capabilities separate authority from ambient service access. Rather than pass a broad payment
service plus an ID, a validated authorization step can produce `CaptureCapability` scoped to
payment, amount, provider, and expiry. Only it exposes capture. Constructor privacy prevents
ordinary forgery. Yet cloning, serialization, and stale revocation can defeat the story, so
capability design includes the whole lifecycle.

Borrowing can express temporary access. An immutable borrowed view supports inspection without
ownership transfer. A mutable borrow grants exclusive mutation for its duration. Returning a
reference tied to a guard can prevent use after the guard ends. Lifetimes prove local reference
validity; they do not prove a remote lease, session, or socket remains accepted.

RAII is effective for local release because `Drop` runs when an owned value leaves scope during
ordinary unwinding. `Drop` cannot return an error. A database rollback, remote lease release,
or durable file sync can fail. Provide explicit `commit`, `rollback`, `close`, or `release`
methods whose failures are visible; use drop as a best-effort fallback and make fallback
failure observable where possible. Compensation is a new effect, not automatic rollback.

Secrets need restrictive traits. An ordinary derived `Debug` can print a token into logs.
`Clone` multiplies copies. Serde can place plaintext into an intermediate buffer. Borrowed
exposure can outlive the intended operation if returned or captured. A secret wrapper should
redact debug, omit display, avoid clone, and expose bytes only through a scoped closure or
deliberate method.

Zeroization narrows memory exposure but is often overclaimed. Clearing the owned buffer does
not clear copies made before wrapping, formatting buffers, allocator pages, swap, crash dumps,
remote logs, or serialized records. Compiler optimization and memory model details matter.
Accurate documentation states what buffer and lifecycle are controlled.

`Arc<Mutex<T>>` is sometimes correct. It provides shared ownership and mutually exclusive
runtime mutation. It does not identify which task owns progress, bound lock duration, prevent
deadlock, provide backpressure, define poisoning recovery, or stop a caller from performing an
external effect while holding the lock. Actor ownership, message passing, partitioned state,
or a single supervisor may fit better.

Interior mutability is similarly a contract, not an escape from the borrow checker. `RefCell`
moves borrow failure to runtime and can panic on reentrancy. A mutex introduces blocking,
poisoning, and contention. Atomics require ordering arguments. The mechanism should match the
aliasing relationship actually required.

Task transfer completes the ownership story. Moving a resource into a spawned task makes that
task responsible for it, but detached tasks can outlive request scope and lose error
reporting. Structured supervision, join handles, cancellation tokens, channel closure, and
graceful shutdown identify who reclaims resources and observes failure.

Examples compose these ideas:

- an authorization capability is issuer-created, operation-scoped, non-forgeable, and
  revalidated if revocable;
- a transaction guard is consumed by commit or rollback and reports ambiguous commit;
- a secret wrapper redacts formatting and controls exposure;
- a single-use token is non-cloneable and consumed;
- a leased resource carries expiry and fencing identity;
- a file lock states local operating-system semantics and releases on drop;
- a shutdown permit moves to one supervisor;
- a task-owned handle returns completion through a join path.

Ownership removes certain local invalid programs. It does not create external truth. The
guarantee ledger must keep those scopes separate.

## Guarantee ledger examples

| Claim                                                     | Established by                                          | Does not prove                                                         |
| --------------------------------------------------------- | ------------------------------------------------------- | ---------------------------------------------------------------------- |
| `CommitPermit` has not been consumed through this value   | private, non-cloneable ownership and consuming `commit` | database commit will succeed or acknowledgement will arrive            |
| `FileLock` owns one acquired operating-system lock handle | successful acquisition and owned handle                 | every other process or host honors the same locking protocol           |
| `SessionCapability` was issued for a principal and scope  | protected issuer and signed or server-side grant        | session remains unexpired, unrevoked, or sufficient for changed policy |
| `SecretBytes` redacts ordinary formatting                 | custom trait implementations and absent display         | copies never existed or memory is absent from swap and crash dumps     |
| `ShutdownPermit` has one process-local owner              | private non-cloneable value                             | all external workers will acknowledge shutdown                         |

These entries show why a capability's constructor, methods, and documentation must be reviewed
together. A name such as `ExclusiveLease` is dishonest if the server does not reject stale
holders with a fencing value. A name such as `RolledBackTransaction` is dishonest when drop
only sent a best-effort request.

## Transaction guards

A transaction guard often borrows a connection or owns a pooled connection. The lifetime can
ensure the guard does not outlive the local connection borrow. Consuming `commit` prevents
continued local mutation through the same guard. The database remains an independent system:
isolation level, server failover, connection loss, and ambiguous acknowledgement determine the
actual outcome.

An API can return `CommitOutcome::Confirmed`, a definite pre-commit failure with a reusable or
released guard, or an unknown outcome with transaction identity. It should not return the
original guard after possible commit as though the transaction were safely reusable. Drop may
attempt rollback only while local protocol evidence still supports it.

## Leases and file locks

Leases require more than an expiry timestamp stored in a Rust struct. State the clock source,
skew assumption, renewal protocol, server enforcement, fencing rule, and behavior after
renewal uncertainty. A task that pauses past expiry can still hold the local value; the
resource owner must reject stale fencing tokens.

File locks need a declared scope: process, host, mount, or network filesystem. Advisory locks
work only among cooperating actors. Paths can alias through links or mounts. Inheritance
across fork, duplication of descriptors, and close semantics can affect ownership. The wrapper
should claim only the documented operating-system behavior.

## Shutdown and cancellation

One shutdown permit can serialize initiation, while a broadcast cancellation token can notify
many tasks. These are different authorities. Notification does not prove completion. The
supervisor owns the task registry, waits within a deadline, records stragglers, and chooses
forced termination behavior. Dropping a permit should not silently initiate a destructive
shutdown unless that surprising contract is unavoidable and prominent.

Transferring a handle through a channel moves custody only when send succeeds. A failed send
returns the value, leaving the sender responsible. Once received, channel closure and task
panic determine recovery. Tests should cover send failure, receiver cancellation, and
supervisor shutdown — not only the successful handoff.

## Choosing less machinery

Not every right needs a new capability type. If a private function is called only after one
obvious authorization branch and no value crosses asynchronous or module boundaries, a
separate capability can add ceremony without reducing risk. Conversely, a capability is
valuable when authority would otherwise travel as a boolean, broad context object, or repeated
comment.

The complexity decision should compare an owned token, runtime policy check, narrow trait,
closure-based authority, and ordinary parameter passing. Select the mechanism that keeps
issuance, use, and end of authority visible with the least misleading surface.
