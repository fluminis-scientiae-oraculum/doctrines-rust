# Anti-pattern catalogue

## Unsafe to satisfy the borrow checker

**Weak example.** A lifetime is extended or alias created because restructuring
ownership is inconvenient.

**Why it fails.** The borrow error often identifies a real custody ambiguity.

**Risk.** dangling reference or aliasing violation.

**Improved direction.** change ownership, shorten borrows, use indices or owned
values, or encode lifecycle.

**When justified.** Self-referential or intrusive structures may require unsafe
after pinning and movement proofs are complete.

## "It seems safe"

**Weak example.** A safety comment expresses confidence without facts.

**Why it fails.** Reviewers cannot connect operation preconditions to evidence.

**Risk.** undocumented undefined behavior.

**Improved direction.** enumerate allocation, bounds, alignment,
initialization, validity, aliasing, lifetime, and concurrency premises.

**When justified.** Never as the complete argument.

## Syntax narration

**Weak example.** `SAFETY: dereferencing the pointer here`.

**Why it fails.** It says what happens, not why the operation is permitted.

**Risk.** false review signal.

**Improved direction.** state pointer origin, live allocation, checked range,
alignment, initialization, and aliases.

**When justified.** Syntax may orient a longer proof but cannot replace it.

## Whole unsafe function

**Weak example.** A large implementation is marked unsafe so any operation can
be used inside.

**Why it fails.** Safe computations and proof-requiring steps become
indistinguishable.

**Risk.** later edits add unchecked operations.

**Improved direction.** keep the function safe where possible and use narrow
unsafe blocks.

**When justified.** Caller-facing unsafe remains necessary when callers supply
uncheckable premises, but internals should still localize operations.

## Forge `'static`

**Weak example.** A borrowed reference is transmuted to `'static` because an
async task needs it.

**Why it fails.** type lifetime outlives the allocation contract.

**Risk.** use after free.

**Improved direction.** move owned data, scope the task, or redesign storage.

**When justified.** A genuinely process-lifetime allocation can produce a
long-lived reference through a direct ownership argument, usually without
transmute.

## Transmute an enum

**Weak example.** An integer from FFI is transmuted into a Rust enum.

**Why it fails.** unknown discriminants can be invalid typed values before
matching.

**Risk.** immediate undefined behavior.

**Improved direction.** match accepted integers into variants and retain an
unknown/error path.

**When justified.** No convenience justification makes arbitrary input valid.

## Assume observed layout

**Weak example.** A test checks current size and the code relies on unannotated
Rust field order.

**Why it fails.** observed compiler output is not a stable layout contract.

**Risk.** corruption on compiler, target, or optimization changes.

**Improved direction.** use an applicable `repr`, explicit boundary fields, and
primary ABI definitions.

**When justified.** An assertion reinforces a defined contract; it does not
create one.

## Reference across reallocation

**Weak example.** A pointer into a vector becomes a reference, then the vector
may grow.

**Why it fails.** growth can move the allocation.

**Risk.** dangling access.

**Improved direction.** use indices, reserve under proven bounds, stable
allocation, or pinning appropriate to the structure.

**When justified.** The allocation must be proven immovable for the complete
reference lifetime.

## `assume_init` after partial success

**Weak example.** An array is treated as initialized after a loop that can exit
early.

**Why it fails.** some elements may contain no valid `T`.

**Risk.** invalid reads and incorrect drop.

**Improved direction.** track initialized prefix with a cleanup guard and
convert only after completion.

**When justified.** Only when every element's initialization is proven.

## Manual `Send` because mutex

**Weak example.** A raw foreign handle gets unsafe `Send` because the wrapper
contains a mutex.

**Why it fails.** the library may require thread affinity, callbacks may race,
and drop may occur on the wrong thread.

**Risk.** foreign corruption and races.

**Improved direction.** establish the full foreign thread contract or keep
ownership on one task/thread.

**When justified.** The mutex participates in a complete proof covering all
operations and destruction.

## Catch panic and continue blindly

**Weak example.** FFI wrapper catches a panic and reports an error while leaving
foreign and Rust state unexamined.

**Why it fails.** containment of unwind does not prove logical state remains
usable.

**Risk.** later unsafe assumptions consume damaged state.

**Improved direction.** mark handle poisoned, rebuild, or terminate according to
the state invariant.

**When justified.** Continue only when cleanup guarantees a valid recoverable
state.

## Miri passed, therefore sound

**Weak example.** A clean test run replaces safety reasoning.

**Why it fails.** only executed paths and supported operations were checked.

**Risk.** untested targets or inputs retain undefined behavior.

**Improved direction.** combine proof, Miri, sanitizers, fuzzing, target tests,
and review.

**When justified.** Miri is strong supporting evidence, never universal proof.
