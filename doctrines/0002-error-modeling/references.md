# References

- [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html) defines Rust's
  standard source-chain interface.
- [`std::result::Result`](https://doc.rust-lang.org/std/result/) documents explicit
  success/error control flow.
- [Rust API Guidelines: dependability](https://rust-lang.github.io/api-guidelines/dependability.html)
  covers useful error traits and predictable behavior.
- [The Rust Book: recoverable errors with
  `Result`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
  distinguishes recoverable errors and panics.
- [RFC 2008: non-exhaustive types](https://rust-lang.github.io/rfcs/2008-non-exhaustive.html)
  informs evolvable public error enums.
- [HTTP Semantics, status codes](https://www.rfc-editor.org/rfc/rfc9110#name-status-codes)
  defines protocol response classes without making them universal retry policy.
- [Tokio tutorial: spawning and `'static`](https://tokio.rs/tokio/tutorial/spawning) and
  runtime documentation provide primary runtime context for task error handling.
- [Common Weakness Enumeration CWE-209](https://cwe.mitre.org/data/definitions/209.html)
  documents information exposure through error messages.
