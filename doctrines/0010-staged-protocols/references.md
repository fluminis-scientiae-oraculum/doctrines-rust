# References

Primary sources for the language mechanics this doctrine relies on, and the literature for the
families it refines. Each entry states what the source establishes; repository governance
supplies the obligations.

## Language mechanics

- [The Rust Reference: associated items](https://doc.rust-lang.org/reference/items/associated-items.html)
  — establishes associated types and the trait bounds that may be placed on them, which is the
  mechanism `RUST-DOC-0010-R003` requires for exposing a successor capability.
- [The Rust Reference: traits](https://doc.rust-lang.org/reference/items/traits.html) — establishes
  supertraits, `Self: Sized` requirements, and the conditions under which a trait is
  dyn-compatible, which bound what a stage capability can express.
- [The Rust Book: what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
  — establishes move semantics, the basis of the consuming transition in `RUST-DOC-0010-R005`
  and the exact limit of the claim in `RUST-DOC-0010-R014`.
- [The Rust Reference: visibility and privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html)
  — establishes the field and constructor privacy that makes stage evidence unforgeable under
  `RUST-DOC-0010-R010` and `RUST-DOC-0010-R011`.
- [The Rust Reference: generic parameters](https://doc.rust-lang.org/reference/items/generics.html)
  — establishes monomorphization, relevant to the cost assessment in `RUST-DOC-0010-R012`.
- [`trybuild`](https://docs.rs/trybuild) — the harness used for the compiler-rejection evidence
  required by `RUST-DOC-0010-R018`. Version scope: 1.0.118, checked 2026-08-04.

## Families this doctrine refines

- Strom and Yemini, "Typestate: A Programming Language Concept for Enhancing Software
  Reliability," _IEEE Transactions on Software Engineering_ SE-12(1), 1986 — establishes
  typestate as a compile-time discipline in which an object's legal operations depend on its
  state. It is the origin of the family; it does not define the trait-based successor mechanism.
- Aldrich, Sunshine, Saini, and Sparks, "Typestate-Oriented Programming," _OOPSLA Onward!_ 2009
  — establishes state as a first-class unit with state-specific members, and the framing of
  valid method-call sequences as an object protocol.
- Honda, Vasconcelos, and Kubo, "Language Primitives and Type Discipline for Structured
  Communication-Based Programming," _ESOP_ 1998 — establishes session types for communication
  protocols with dual participants. A close relative; the distinction is recorded in the source
  notes, and this doctrine governs an object's internal protocol rather than communication
  duality.

## Durability and concurrency

- [PostgreSQL: transaction isolation](https://www.postgresql.org/docs/current/transaction-iso.html)
  — establishes that concurrent readers observe snapshots and that serialization failures must be
  retried, which is why an observation taken during one stage does not remain true at write time.
- [PostgreSQL: MVCC introduction](https://www.postgresql.org/docs/current/mvcc-intro.html) —
  establishes that readers obtain a copy of committed state rather than exclusive custody of it.
  This is the mechanical basis of `RUST-DOC-0010-R014`: a Rust move consumes a local value, and
  a row read into that value can be read again by another worker.
- Gray and Cheriton, "Leases: An Efficient Fault-Tolerant Mechanism for Distributed File Cache
  Consistency," _SOSP_ 1989 — establishes time-bounded distributed authority, relevant to the
  fencing token required where durable advancement is contended. See also RFC-0001, which
  records the repository's accepted position on time-based authority.

## Attribution discipline

Quotations are short and used only where wording matters; sources are otherwise summarized and
linked. No external media, transcript, or specification text is mirrored here. Source notes
under [`sources/0010-staged-protocols/`](../../sources/0010-staged-protocols/source-notes.md)
classify which ideas this package accepts, refines, rejects, and adds, and identify which
vocabulary is local to this repository rather than external.

References are informative. A normative obligation exists only where [`doctrine.md`](doctrine.md) states it
with a rule identifier. Changing facts, including tool versions and product behavior, carry the
version or date checked and are rechecked when the package is maintained.
