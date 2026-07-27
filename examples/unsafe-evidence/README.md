# Unsafe evidence: panic-safe array initialization

This crate is the workspace's narrow exception to the root
`unsafe_code = "forbid"` policy. It provides executable evidence for selected
RUST-DOC-0007 and RUST-DOC-0008 obligations without making unsafe code a
workspace default. The crate has no dependencies.

## Concern and safe contract

`try_init_array` constructs `[T; N]` fallibly without heap allocation. Its
builder owns each returned `T`; callers receive either a fully initialized
array or the builder's error. On builder error or panic, every already
initialized element is dropped once. The safe API imposes no unsafe precondition
on callers.

The function does not catch panics, guarantee progress, constrain `T`'s
destructor, or make a panicking destructor recoverable. It establishes no FFI,
threading, allocator, or external-resource guarantee.

## Why unsafe is needed here

The implementation stores the array as `MaybeUninit<[T; N]>` and writes one
element at a time. Reading the array before all elements exist would be
undefined behavior, while ordinary Rust cannot express the changing initialized
prefix in the array type. A private guard records that prefix for cleanup.

Safe alternatives considered were an infallible array generator, which cannot
return the builder's `E`, and building a `Vec<T>` before converting it to an
array, which adds heap allocation and a second capacity invariant. An existing
reviewed implementation should be preferred in production. This
dependency-free example keeps the raw proof visible because its required
teaching capability is fallible, allocation-free partial initialization.

## Safety argument

The private representation maintains these invariants:

1. `first` retains provenance from live, aligned `[T; N]` storage owned by the
   function; logical indices `0..N` remain in bounds, including for zero-sized
   `T`.
2. `initialized` is never greater than `N`.
3. Logical slots in `0..initialized` contain fully initialized, independently
   owned `T` values.
4. Slots in `initialized..N` are never read or dropped.
5. Zero-sized elements are counted and dropped by logical index even when their
   addresses are equal.

For each element, the builder completes before the slot is written. The raw
write targets the current in-bounds uninitialized slot, then the guard count is
incremented. If the builder returns an error or panics, stack unwinding drops
the guard, which drops only the initialized prefix. After `N` successful writes,
the guard is disarmed before `assume_init`; therefore the returned array owns
all elements and no guard can drop them again.

Every unsafe block has a local `SAFETY` comment tied to alignment, validity,
lifetime, provenance, bounds, initialization, aliasing, and drop ownership. The
unsafe implementation and guard are private, so safe callers cannot forge the
prefix count or invoke `assume_init` early. The only deliberate escape hatch is
this crate's local unsafe-lint allowance.

## Doctrine compliance

| Rules                           | Disposition and evidence                                                                                                                                                              |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| RUST-DOC-0007-R001              | The required allocation-free fallible array capability, safe alternatives, scope, and non-production purpose are stated above.                                                        |
| RUST-DOC-0007-R002 through R004 | Four minimal unsafe blocks carry local proofs; the guard and raw representation are private; safe-call, error, panic, zero-length, and zero-sized paths are tested.                   |
| RUST-DOC-0007-R005              | Not applicable: the crate exports no `unsafe fn`, unsafe trait, or caller obligation.                                                                                                 |
| RUST-DOC-0007-R006 through R009 | Typed validity begins only after each owned `T` is written; provenance, bounds, aliasing, partial initialization, and exact drop ownership are covered by the invariant and comments. |
| RUST-DOC-0007-R010 through R014 | Not applicable: there is no transmute, FFI, custom allocation boundary, foreign unwind, or unsafe `Send`/`Sync` implementation.                                                       |
| RUST-DOC-0007-R015              | Builder panic is injected; the guard restores memory safety and drops the initialized prefix. Destructor panic retains ordinary Rust abort/unwind semantics.                          |
| RUST-DOC-0007-R016              | Ordinary tests and pinned-nightly Miri provide complementary dynamic evidence with explicit limits.                                                                                   |
| RUST-DOC-0007-R017              | The crate has no third-party dependencies.                                                                                                                                            |
| RUST-DOC-0007-R018              | Re-audit triggers are listed below and CI pins the specialized toolchain.                                                                                                             |

## Evidence and limits

Five unit tests cover successful construction and caller-owned drop, builder
error, builder panic, a zero-length array, and zero-sized element drop
accounting. CI reruns those same paths under Miri on pinned nightly
`nightly-2026-07-13`:

```bash
cargo +nightly-2026-07-13 miri test --locked -p unsafe-evidence
```

Miri checks the exercised interpreter paths; it does not prove soundness for
every `T`, compiler optimization, target, or panic behavior. Re-audit after
changes to pointer arithmetic, prefix accounting, storage layout, drop order,
public construction, MSRV, or Miri toolchain.

See [Unsafe Rust as a Proof
Obligation](../../doctrines/0007-unsafe-rust/README.md) and
[Testing as Layered
Evidence](../../doctrines/0008-testing-and-evidence/README.md).
