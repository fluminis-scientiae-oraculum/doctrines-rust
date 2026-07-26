# References

- [The Rust Book: writing automated tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
  describes Rust's built-in test structure and organization.
- [Cargo reference: tests](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
  defines workspace test execution behavior.
- [trybuild documentation](https://docs.rs/trybuild/latest/trybuild/) documents
  stable compile-fail/UI testing and expected diagnostic files.
- [proptest documentation](https://docs.rs/proptest/latest/proptest/) documents
  generated property testing, strategies, shrinking, and persistence.
- [Loom documentation](https://docs.rs/loom/latest/loom/) describes permutation
  testing for concurrent Rust code under a model.
- [Miri](https://github.com/rust-lang/miri) documents interpreted execution for
  detecting many undefined-behavior violations.
- [Rust compiler sanitizer documentation](https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html)
  documents available sanitizers and platform constraints.
- [Criterion.rs documentation](https://bheisler.github.io/criterion.rs/book/)
  documents statistical benchmarking; RUST-DOC-0009 governs claims made from
  measurements.

The doctrine adds invariant traceability, evidence-limit statements,
compiler-diagnostic review, double-fidelity analysis, incident feedback, and
operational review gates.
