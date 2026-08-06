# RUST-DOC-0007 source notes

## Primary Rust sources

[The Rust Reference: unsafety](https://doc.rust-lang.org/reference/unsafe-keyword.html)
defines unsafe functions, blocks, traits, and implementations. Its
[undefined behavior chapter](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
states a non-exhaustive language contract. The
[Rustonomicon](https://doc.rust-lang.org/nomicon/) provides official unsafe-Rust
guidance on ownership, aliasing, FFI, concurrency, and representation.

Standard-library documentation for
[`MaybeUninit`](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html),
[raw pointers](https://doc.rust-lang.org/std/ptr/index.html), and Reference
[type layout](https://doc.rust-lang.org/reference/type-layout.html) establishes
specific operation preconditions and layout guarantees. The
[Miri project](https://github.com/rust-lang/miri) and compiler
[sanitizer documentation](https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html)
describe dynamic evidence tools and limitations.

## Accepted ideas

Unsafe operations require the programmer to satisfy preconditions the compiler
cannot verify. Safe abstractions must be sound for all safe callers. Unsafe
functions expose caller obligations. Raw pointers, uninitialized storage, FFI,
manual marker-trait implementations, and layout-sensitive operations require
precise reasoning.

The doctrine accepts minimization and encapsulation because they improve proof
locality. It accepts Miri, sanitizers, fuzzing, and model checking as
complementary ways to discover violations in exercised code.

## Refined ideas

"Add a safety comment" is refined: the comment states the safety invariant and
connects each operation to provenance, bounds, alignment, initialization,
validity, aliasing, lifetime, concurrency, and panic premises that apply. A
syntax narration supplies no proof.

`MaybeUninit` is refined from "avoid initialization" to an exact partial-state
and drop protocol. Bytes cannot be observed as a typed value before validity is
established.

`repr(C)` is refined to its documented layout role; it does not make arbitrary
Rust types a portable C API. FFI additionally requires ABI, ownership,
nullability, lengths, allocator, callback, thread, encoding, error, and unwind
contracts.

"Miri passed" is refined to evidence for executions and operations Miri models.
It does not prove every input, target, foreign implementation, or compiler
behavior.

## Rejected ideas

The doctrine rejects unsafe used to silence the borrow checker, forged
lifetimes, arbitrary integer-to-enum transmute, layout inferred from one build,
references held across possible reallocation, broad unsafe functions, and manual
`Send`/`Sync` without a complete concurrency proof. It rejects performance as an
unmeasured justification.

## Repository additions

The repository adds a necessity gate, per-operation proof table, safe-caller
adversarial review, panic/drop analysis, dependency unsafe inventory, target
matrix, re-audit triggers, performance-benefit gate, guarantee ledger, and more
than fifty operational unsafe review checks.

## Source-to-rule application

Validity, pointer, layout, and initialization rules follow current primary Rust
contracts. The doctrine's `SAFETY:` format is a review convention: each comment
connects concrete premises to those contracts. FFI rules require the foreign
ABI/header and target documents in addition to Rust sources. Manual `Send` and
`Sync` rules require both Rust marker semantics and upstream thread guarantees.

Dynamic tools are selected by risk. Miri supports many pure-Rust validity and
aliasing checks; sanitizers and foreign integration cover different target
executions. Neither changes the proof obligation for safe callers.

## Maintenance triggers

Unsafe premises require review after compiler, target, ABI, dependency,
allocator, layout, feature, or surrounding API change. Pointer-provenance and
undefined-behavior documentation evolves; maintainers must use the current
Reference and tool limitations. New performance justification requires a fresh
safe baseline and workload measurement.

> [!TIP]
> [attribution](attribution.md) · **source notes**
> Index: [all source packages](../README.md).
> Doctrine: [`doctrines/0007-unsafe-rust/`](../../doctrines/0007-unsafe-rust/README.md).
