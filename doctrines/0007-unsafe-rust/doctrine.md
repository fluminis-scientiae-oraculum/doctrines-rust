# Normative doctrine

## RUST-DOC-0007-R001 — Justify the need for unsafe

**Statement.** Introduction or expansion of unsafe code MUST document the
required capability, safe alternatives considered, and why their cost or
limitations are unacceptable for the stated risk domain.

**Intent.** Prevent unsafe from becoming a convenience escape from design or
borrowing work.

**Applicability.** Every new unsafe block, function, trait implementation, or
FFI boundary.

**Allowed exceptions.** Mechanically generated binding declarations may share
one reviewed justification for a generated unit.

**Review evidence.** Required capability, safe alternatives, explicit scope,
and benchmark evidence when performance justifies the risk.

**Enforcement.** [`examples/unsafe-evidence/README.md`](../../examples/unsafe-evidence/README.md) —
required capability, rejected safe alternatives, scope

## RUST-DOC-0007-R002 — State the safety invariant

**Statement.** Every unsafe block MUST be associated with a `SAFETY:` argument
that states the relevant invariant and explains why each unsafe operation's
preconditions hold at that point.

**Intent.** Make transferred proof obligations inspectable beside the code.

**Applicability.** Explicit and compiler-required unsafe operations.

**Allowed exceptions.** Repeated operations inside one tightly bounded block may
share one complete argument when their obligations are identical.

**Review evidence.** The `SAFETY:` comment names the applicable aliasing,
validity, lifetime, alignment, provenance, initialization, concurrency, and
panic considerations.

**Enforcement.** [`examples/unsafe-evidence/src/lib.rs`](../../examples/unsafe-evidence/src/lib.rs)
— four unsafe blocks, each with a local safety argument

## RUST-DOC-0007-R003 — Minimize and encapsulate unsafe

**Statement.** Unsafe operations MUST be kept in the smallest practical lexical
and API scope and encapsulated behind a safe abstraction whenever safe callers
can use the capability.

**Intent.** Reduce proof surface and prevent invariant-dependent values from
escaping unchecked.

**Applicability.** Low-level modules, FFI wrappers, containers, and optimized
algorithms.

**Allowed exceptions.** A public unsafe primitive may be appropriate when
callers must supply obligations that cannot be checked.

**Review evidence.** Unsafe inventory, module visibility, private fields, and
safe wrapper tests.

**Enforcement.** [`examples/unsafe-evidence/src/lib.rs`](../../examples/unsafe-evidence/src/lib.rs)
— narrow blocks, private guard, safe wrapper, scoped hatch

## RUST-DOC-0007-R004 — Make safe APIs sound for every safe caller

**Statement.** A safe public API implemented with unsafe code MUST uphold
memory-safety requirements for all values and call sequences constructible in
safe Rust, including reentrancy, panic, cancellation, and concurrent use allowed
by its traits.

**Intent.** Prevent hidden caller obligations from leaking through a safe
signature.

**Applicability.** All safe wrappers over unsafe internals.

**Allowed exceptions.** None.

**Review evidence.** Adversarial safe-call analysis, invariant ownership,
panic/drop paths, and executable evidence.

**Enforcement.** [`examples/unsafe-evidence/src/lib.rs`](../../examples/unsafe-evidence/src/lib.rs)
— tests drive panicking builder, error, zero-length, zero-sized paths

## RUST-DOC-0007-R005 — Document unsafe caller obligations

**Statement.** Every public or cross-module `unsafe fn` and unsafe trait MUST
have a `# Safety` section specifying complete caller obligations in testable,
non-circular terms.

**Intent.** Define exactly what the compiler no longer checks for the caller.

**Applicability.** Unsafe functions, methods, traits, and constructors.

**Allowed exceptions.** Private functions used once may state obligations at
the function or call site, but the proof chain MUST remain explicit.

**Review evidence.** Caller obligations name valid ranges, lifetime, ownership,
aliasing, initialization, thread, and provenance constraints as relevant.

**Enforcement.** Unenforceable: Workspace exports no unsafe fn or unsafe trait needing a safety
section

## RUST-DOC-0007-R006 — Protect representation validity

**Statement.** Unsafe code MUST preserve Rust validity requirements for every
value that becomes observable as a typed value. It MUST NOT create invalid enum
discriminants, references, booleans, characters, nonzero values, or other
restricted representations.

**Intent.** Avoid undefined behavior before ordinary code can validate.

**Applicability.** Casts, reads, transmutation, FFI, serialization shortcuts,
and uninitialized memory.

**Allowed exceptions.** Bytes may remain untyped storage until validity is
established; they MUST NOT be observed through an invalid typed value.

**Review evidence.** Representation source, validation, layout reference, and
invalid-input tests.

**Enforcement.** [`examples/unsafe-evidence/src/lib.rs`](../../examples/unsafe-evidence/src/lib.rs)
— storage stays uninitialized until all writes complete

## RUST-DOC-0007-R007 — Prove aliasing and lifetime

**Statement.** Creation or use of references from raw pointers MUST establish
non-nullness, alignment, dereferenceability, initialization, permitted aliasing,
and a lifetime no longer than the backing allocation and authority.

**Intent.** Prevent references from asserting guarantees the pointer does not
provide.

**Applicability.** Raw-pointer dereference, slices from raw parts, FFI pointers,
and self-referential structures.

**Allowed exceptions.** None; only the proof mechanism varies.

**Review evidence.** Allocation owner, mutation paths, reallocation analysis,
and borrow duration.

**Enforcement.** [`examples/unsafe-evidence/src/lib.rs`](../../examples/unsafe-evidence/src/lib.rs)
— the guard proves unique aliasing bounded by the storage

## RUST-DOC-0007-R008 — Respect provenance and bounds

**Statement.** Raw-pointer arithmetic and integer-pointer conversions MUST have
a documented provenance, allocation, element-bound, alignment, and one-past-end
argument consistent with the supported Rust model and target APIs.

**Intent.** Prevent address arithmetic from being treated as sufficient pointer
authority.

**Applicability.** Allocators, buffers, intrusive structures, memory maps, and
FFI.

**Allowed exceptions.** None.

**Review evidence.** Originating allocation, range proof, zero-sized-type
behavior, overflow handling, and Miri coverage where supported.

**Enforcement.** [`examples/unsafe-evidence/src/lib.rs`](../../examples/unsafe-evidence/src/lib.rs)
— pointer provenance and bounds, interpreted under Miri

## RUST-DOC-0007-R009 — Handle partial initialization and drop

**Statement.** `MaybeUninit` and manual initialization MUST track exactly which
elements are initialized and MUST drop each initialized value exactly once on
success, error, and panic paths.

**Intent.** Prevent reads of uninitialized memory, leaks of owned resources, and
double drop.

**Applicability.** Arrays, FFI output buffers, custom collections, and
performance-sensitive construction.

**Allowed exceptions.** Trivially non-dropping byte storage still requires proof
against uninitialized typed reads.

**Review evidence.** Initialization counter or state, guard behavior, panic
injection, and destructor tests.

**Enforcement.** [`examples/unsafe-evidence/src/lib.rs`](../../examples/unsafe-evidence/src/lib.rs)
— the prefix counter drops exactly the initialized prefix

## RUST-DOC-0007-R010 — Require exceptional justification for transmute

**Statement.** `transmute` MUST require stronger justification than convenience:
source and destination size, alignment, validity, lifetime, ownership, and
layout compatibility MUST be established from authoritative contracts.

**Intent.** Expose the many simultaneous obligations hidden by one operation.

**Applicability.** Every transmute or equivalent bit reinterpretation.

**Allowed exceptions.** None; a narrower cast or conversion SHOULD be used when
it expresses fewer obligations.

**Review evidence.** Primary layout citation, static assertions where possible,
and tests across supported targets.

**Enforcement.** Unenforceable: No transmute or bit reinterpretation exists in any workspace crate

## RUST-DOC-0007-R011 — Define FFI representation and ABI

**Statement.** FFI declarations MUST specify the correct ABI and use
representations whose layout is defined for that boundary. Rust-native layout
MUST NOT be assumed stable without an applicable representation contract.

**Intent.** Prevent caller/callee disagreement about call convention and data
layout.

**Applicability.** Foreign functions, callbacks, shared structs, unions, and
opaque handles.

**Allowed exceptions.** Bindings generated from an authoritative interface may
derive declarations, but generated output and generator version remain reviewed
inputs.

**Review evidence.** Header/specification match, `repr` choice, target matrix,
and ABI tests.

**Enforcement.** Unenforceable: No extern block, repr(C) type, or FFI declaration exists in the
workspace

## RUST-DOC-0007-R012 — Define FFI ownership and allocation

**Statement.** Every pointer crossing FFI MUST define nullability, length,
ownership transfer, lifetime, mutability, thread access, allocator of origin,
and the matching release operation.

**Intent.** Prevent double frees, leaks, allocator mismatch, and dangling
access.

**Applicability.** Buffers, strings, handles, callbacks, and allocated objects.

**Allowed exceptions.** None; an opaque handle still requires a lifecycle
contract.

**Review evidence.** Boundary table, constructor/destructor pairs, null and
length tests, and foreign-side documentation.

**Enforcement.** Unenforceable: No pointer crosses FFI; workspace has no foreign functions, handles,
or buffers

## RUST-DOC-0007-R013 — Control unwinding across FFI

**Statement.** Panic or foreign exception unwinding across an ABI boundary MUST
be prevented or handled according to an explicitly selected ABI and supported
runtime contract.

**Intent.** Avoid undefined behavior and uncontrolled process state.

**Applicability.** Exported Rust functions, imported callbacks, and foreign
exceptions.

**Allowed exceptions.** An unwind-capable ABI may be used only with documented
cross-language behavior and target support.

**Review evidence.** Catch/abort policy, destructor implications, and panic-path
test.

**Enforcement.** Unenforceable: No exported extern function or foreign callback; unwinding never
crosses an ABI

## RUST-DOC-0007-R014 — Prove unsafe `Send` and `Sync`

**Statement.** Every unsafe implementation of `Send` or `Sync` MUST state a
concurrency proof covering all contained state, aliasing, mutation,
destruction, callbacks, and foreign-library thread guarantees.

**Intent.** Ensure marker traits do not grant unsupported cross-thread
authority.

**Applicability.** Custom containers, raw handles, FFI wrappers, and
self-referential values.

**Allowed exceptions.** None.

**Review evidence.** Trait invariant, synchronization model, adverse schedule
tests, and upstream thread-safety contract.

**Enforcement.** Unenforceable: No unsafe Send or Sync impl exists; the concurrency proof is never
exercised

## RUST-DOC-0007-R015 — Preserve panic safety

**Statement.** Unsafe abstractions MUST remain memory-safe if safe callbacks,
allocation, cloning, comparison, formatting, or destruction panic at any
permitted point.

**Intent.** Prevent partial mutation from violating assumptions later consumed
by unsafe code.

**Applicability.** Collections, sorting, initialization, callback-based APIs,
and guards.

**Allowed exceptions.** Logical corruption after panic may be allowed only if
memory safety remains intact and the object cannot be used as though valid.

**Review evidence.** Unwind-state analysis, guards, injected panics, and drop
accounting.

**Enforcement.** [`examples/unsafe-evidence/src/lib.rs`](../../examples/unsafe-evidence/src/lib.rs)
— an injected panic asserts drop accounting

## RUST-DOC-0007-R016 — Use complementary dynamic evidence

**Statement.** Unsafe code SHOULD be exercised with Miri and relevant
sanitizers, fuzzing, model checking, or target-specific integration tests where
the tools support its behavior.

**Intent.** Detect violations that code review and ordinary tests can miss.

**Applicability.** Pointer, initialization, FFI, and concurrency code.

**Allowed exceptions.** Unsupported operations or targets may use alternative
evidence, with the limitation documented.

**Review evidence.** Exact commands, supported targets, findings resolved, and
known blind spots.

**Enforcement.** [`.github/workflows/rust-examples.yml`](../../.github/workflows/rust-examples.yml)
— the Miri job reruns the tests on a pinned nightly

## RUST-DOC-0007-R017 — Review unsafe dependencies

**Statement.** Dependencies containing material unsafe code MUST be identified
and reviewed proportionally to reachability, privilege, input exposure,
maintenance, advisories, and substitutability.

**Intent.** Include transitive proof trust in the system risk model.

**Applicability.** FFI bindings, parsers, runtimes, allocators, cryptography, and
highly privileged libraries.

**Allowed exceptions.** Low-risk unreachable target-specific code may receive a
documented reduced review.

**Review evidence.** Dependency inventory, versions, advisory status, unsafe
surface, upstream audit evidence, and update policy.

**Enforcement.** Unenforceable: Example crate has zero third-party dependencies; no unsafe
dependency surface exists

## RUST-DOC-0007-R018 — Re-audit when assumptions change

**Statement.** Unsafe code MUST be re-reviewed when compiler behavior, target,
ABI, dependency, layout, allocation, synchronization, or surrounding safe API
assumptions change.

**Intent.** Keep proof obligations synchronized with their premises.

**Applicability.** Upgrades, ports, refactors, and feature changes.

**Allowed exceptions.** A change proven outside the unsafe dependency cone may
document that conclusion.

**Review evidence.** Assumption inventory, changed-premise analysis, repeated
dynamic evidence, and reviewer approval.

**Enforcement.** Unenforceable: Only a trigger list is documented; no artifact records a re-audit
after a changed premise
