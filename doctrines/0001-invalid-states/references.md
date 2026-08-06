# References

Primary and authoritative sources:

- [The Rust Reference: types](https://doc.rust-lang.org/reference/types.html) defines language
  type forms and their semantics.
- [The Rust Reference: visibility and privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html)
  defines module-scoped access used to protect construction.
- [`std::num::NonZeroU64`](https://doc.rust-lang.org/std/num/type.NonZeroU64.html) documents
  the exact non-zero integer guarantee and niche behavior.
- [`std::convert::TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) defines
  fallible value-to-value conversion used by boundary adapters.
- [Rust API Guidelines: type safety](https://rust-lang.github.io/api-guidelines/type-safety.html)
  discusses newtypes and static enforcement.
- [Serde container attributes](https://serde.rs/container-attrs.html) documents `try_from`,
  `from`, `into`, and representation controls relevant to checked decoding.
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) explains proof obligations that arise
  when unsafe code performs construction the compiler cannot verify.
- [trybuild documentation](https://docs.rs/trybuild/) defines the stable UI-test harness used
  for compiler-rejection evidence.
- [RFC 2008: non-exhaustive types](https://rust-lang.github.io/rfcs/2008-non-exhaustive.html)
  informs public enum evolution and unknown future cases.
- [Gray and Cheriton, "Leases: An Efficient Fault-Tolerant Mechanism for Distributed File
  Cache Consistency"](https://dl.acm.org/doi/10.1145/74850.74870) is a foundational treatment
  of time-bounded distributed authority and uncertainty.
- [RFC 9110: HTTP Semantics](https://www.rfc-editor.org/rfc/rfc9110) defines method semantics,
  retries, and idempotency terminology used at HTTP boundaries.

Pedagogical provenance for the originating video and the doctrine's accepted, refined, and
added claims is recorded under `sources/0001-invalid-states/`. The video is not a language
specification; normative claims in this package are bounded by the primary sources and the
explicit contracts above.
