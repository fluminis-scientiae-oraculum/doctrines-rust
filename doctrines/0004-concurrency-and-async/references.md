# References

Primary and authoritative material:

- [Rust standard library: `std::marker::Send`](https://doc.rust-lang.org/std/marker/trait.Send.html)
  and [`Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html) define
  language-level transfer and sharing marker contracts. They do not claim
  application-level deadlock or protocol correctness.
- [Rust standard library: `std::sync::Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
  documents locking and poisoning behavior used by the review rules.
- [Rust standard library: atomic memory ordering](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html)
  defines the available orderings. The doctrine adds the requirement to connect
  every selection to an application invariant.
- [The Rustonomicon: concurrency](https://doc.rust-lang.org/nomicon/concurrency.html)
  discusses unsafe concurrency and the role of `Send` and `Sync`.
- [The Async Book](https://rust-lang.github.io/async-book/) explains Rust
  futures, executors, and async programming mechanics. This doctrine adds
  operational ownership, capacity, and evidence gates.
- [Tokio tutorial: spawning](https://tokio.rs/tokio/tutorial/spawning) and
  [Tokio topic: graceful shutdown](https://tokio.rs/tokio/topics/shutdown)
  provide runtime-specific task and shutdown guidance.
- [Tokio `select!` documentation](https://docs.rs/tokio/latest/tokio/macro.select.html)
  documents cancellation behavior and fairness considerations for that runtime
  construct.
- [Tokio `spawn_blocking` documentation](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html)
  documents blocking-task behavior, including cancellation limitations.
- [Loom documentation](https://docs.rs/loom/latest/loom/) describes controlled
  exploration of concurrent executions for small Rust protocols.

These sources establish language, library, or runtime mechanics. Requirements
for ownership maps, capacity tables, retry inventories, guarantee ledgers, and
review severities are repository governance added to make those mechanics
auditable.
