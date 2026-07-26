# References

- [The Rust Reference: unsafety](https://doc.rust-lang.org/reference/unsafe-keyword.html)
  defines unsafe functions, blocks, traits, and implementations.
- [The Rust Reference: behavior considered undefined](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
  provides the language's non-exhaustive undefined-behavior contract.
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) is the official book
  for unsafe Rust topics including aliasing, ownership, FFI, and concurrency.
- [`std::mem::MaybeUninit`](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html)
  documents initialization invariants and common patterns.
- [Rust standard library pointer module](https://doc.rust-lang.org/std/ptr/index.html)
  documents raw-pointer operations and safety requirements.
- [The Rust Reference: type layout](https://doc.rust-lang.org/reference/type-layout.html)
  defines representation guarantees and their limits.
- [The Rust Reference: function ABI](https://doc.rust-lang.org/reference/items/functions.html#extern-function-qualifier)
  documents external function qualifiers and ABI strings.
- [Miri](https://github.com/rust-lang/miri) is the Rust project's interpreter
  for detecting many undefined-behavior violations in executed code.
- [Rust Sanitizers](https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html)
  documents compiler sanitizer support and target limitations.

The doctrine adds repository requirements for necessity, local proof comments,
adversarial safe-call review, dependency inventory, evidence composition, and
re-audit triggers.
