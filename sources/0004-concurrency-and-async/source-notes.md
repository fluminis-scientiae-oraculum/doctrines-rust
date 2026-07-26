# RUST-DOC-0004 source notes

## Primary Rust and runtime sources

Standard-library documentation for
[`Send`](https://doc.rust-lang.org/std/marker/trait.Send.html),
[`Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html),
[`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html), and
[atomic ordering](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html)
establish transfer, sharing, poisoning, and memory-order APIs. The
[Rustonomicon concurrency chapter](https://doc.rust-lang.org/nomicon/concurrency.html)
provides official unsafe-concurrency context.

The [Async Book](https://rust-lang.github.io/async-book/) describes future and
executor mechanics. Tokio's official documentation for
[`select!`](https://docs.rs/tokio/latest/tokio/macro.select.html),
[`spawn_blocking`](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html),
and [graceful shutdown](https://tokio.rs/tokio/topics/shutdown) supplies
runtime-specific cancellation, blocking, and shutdown facts. The
[Loom documentation](https://docs.rs/loom/latest/loom/) establishes the
model-testing tool's intended scope.

## Accepted ideas

The doctrine accepts Rust's language-level data-race prevention and uses
ownership to reduce sharing. It accepts explicit synchronization for shared
mutation, bounded channels and concurrency for overload control, careful lock
scope, task supervision, graceful shutdown, and controlled schedule testing for
small protocols.

Cancellation is treated as ordinary control flow because dropping a future can
abandon its remaining work. Runtime documentation determines which operations
are cancellation-safe and how blocking tasks behave.

## Refined ideas

`Send + Sync` is refined to mean the marker contracts, not absence of deadlock,
starvation, logical races, or overload. A mutex protects only the invariant
actually enclosed by its protocol. An async-aware mutex does not make long
critical sections harmless.

Structured concurrency is expressed as accountable task ownership rather than
requiring one library API. A process-lifetime worker may outlive an immediate
scope if a top-level supervisor observes failure, capacity, and shutdown.

Moving work to a blocking pool is isolation, not deletion of contention. The
pool requires capacity and cancellation analysis. Async overlap is not parallel
CPU speedup.

Atomic orderings are not ranked as good/bad. Each selection follows a
happens-before argument tied to an invariant. Sequential consistency cannot
repair an incorrect algorithm.

## Rejected ideas

The doctrine rejects unbounded spawn/channel defaults, sleep-based schedule
proof, fire-and-forget effects, synchronous blocking on executor workers without
analysis, locks held across external waits by convenience, and retry at every
layer. It rejects the claim that successful async compilation establishes
cancellation or shutdown correctness.

## Repository additions

The repository adds ownership maps, task trees, capacity tables, cancellation
tables, lock graphs, attempt multiplication, channel-closure gates, detached-task
criteria, explicit ordering scope, guarantee ledgers, and fifty operational
review gates. These artifacts connect runtime mechanics to auditable system
contracts.

## Source-to-rule application

Rules R001–R004 translate safe sharing primitives into application invariant and
lock protocols. R005–R013 use runtime mechanics to require blocking isolation,
cancellation cleanup, bounded capacity, task custody, and shutdown. Atomic and
ordering rules remain tied to the Rust memory-order contract, while retry
amplification and external uncertainty connect to distributed doctrine rather
than executor behavior.

The runtime citations are examples of facts that must be checked against the
chosen executor. For instance, cancellation behavior of a queue receive or
blocking task is an API-specific contract; the repository requirement is to
identify and honor that contract.

## Maintenance triggers

Recheck sources when runtime versions, feature flags, scheduling, cancellation,
blocking-pool, or shutdown APIs change. Re-audit when a synchronous library is
introduced into async work or when worker/channel capacity changes. A clean
compiler upgrade does not establish unchanged scheduling or performance.
