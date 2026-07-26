# RUST-DOC-0004 attribution

Rust, Tokio, and Loom documentation is linked and summarized. Tokio-specific
statements remain scoped to its documented APIs and version; the doctrine does
not assert that every executor behaves identically.

Task ownership, capacity, cancellation, retry-composition, and review artifacts
are repository governance synthesis. No runtime documentation is copied at
length.

This package is not an exhaustive survey of async runtimes, formal memory
models, lock-free algorithms, or structured-concurrency libraries. Maintainers
must recheck runtime behavior and MSRV when executable dependencies change.
