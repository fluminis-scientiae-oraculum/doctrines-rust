# References

- [Rust Book: ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
  defines moves, borrowing, and slices.
- [Rust Reference: destructors](https://doc.rust-lang.org/reference/destructors.html) defines
  destruction scope and drop behavior.
- [`std::ops::Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html) documents Rust's
  destructor hook and its non-fallible signature.
- [`std::sync::Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) and
  [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) document shared ownership,
  locking, and poisoning.
- [`std::cell`](https://doc.rust-lang.org/std/cell/) documents interior mutability and runtime
  borrow checking.
- [The Rustonomicon: Send and Sync](https://doc.rust-lang.org/nomicon/send-and-sync.html)
  explains concurrency marker obligations.
- [Dennis and Van Horn, "Programming Semantics for Multiprogrammed
  Computations"](https://dl.acm.org/doi/10.1145/360303.360308) is a foundational capability
  reference.
- [RFC 6819, OAuth threat model](https://www.rfc-editor.org/rfc/rfc6819) describes token
  leakage, replay, and lifecycle threats relevant to bearer capabilities.
