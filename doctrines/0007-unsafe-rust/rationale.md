# Rationale

## The boundary of compiler proof

Safe Rust establishes a set of memory-safety properties for safe programs,
assuming sound compiler and library behavior. Unsafe operations are required
for capabilities such as dereferencing raw pointers or calling unsafe foreign
functions because the compiler lacks enough information to establish their
preconditions. The programmer does not receive permission to ignore those
preconditions; responsibility moves into a proof argument.

A safety comment that merely restates "this pointer is dereferenced here" adds

## Safe abstraction means adversarial safe callers

Hidden documentation such as "do not call twice" cannot repair a safe signature.
The API must enforce the lifecycle, perform a runtime check, consume a value, or
be unsafe with explicit caller obligations.

## Lexical minimization improves proof locality

An entire function marked unsafe permits operations to be added later without a
local review cue. A narrow block shows which steps rely on external proof and
keeps parsing, range checking, and ownership setup in safe code. Encapsulation
protects private invariants so callers cannot manufacture a state that makes
the unsafe implementation invalid.

Small blocks are not automatically sound. They are useful because reviewers can
map each operation to its premises and because safe surrounding code carries
more of the argument.

## Validity precedes business validation

It is undefined behavior to create some invalid Rust typed values even if code
intends to check them immediately. Reading an arbitrary byte as `bool`, creating
a reference from null, or materializing an enum with an invalid discriminant
crosses the validity boundary before a match or condition can reject it.
Untrusted bytes should remain bytes or a representation that accepts every bit
pattern until validated.

This distinction mirrors domain construction: physical representation
validation must occur before trusted interpretation, but unsafe Rust adds
language validity obligations that cannot be repaired after violation.

## Aliasing, lifetime, and provenance

A raw numeric address does not establish ownership or provenance. A reference
asserts alignment, dereferenceability, validity, and aliasing properties for its
lifetime. Reallocation can invalidate addresses into vectors. Foreign libraries
may retain callbacks or buffers beyond the call. A slice length can overflow or
extend beyond its allocation even when the starting pointer is valid.

## Partial initialization

Leaking values may be memory-safe for some `T`, but can leak locks, file
descriptors, or secrets. Correct resource behavior remains part of the broader
contract even where language-level undefined behavior is absent.

## Unwinding and panic

Unsafe collection code can become unsound when a user-provided comparator or
clone panics mid-transition. Guards should ensure the object remains droppable
and no value is dropped twice. Exported Rust callbacks must prevent panic from
crossing an incompatible foreign ABI, commonly by catching it at the boundary
or using process abort according to policy. Catching panic does not guarantee
the foreign library remains logically usable; that is another contract.

## `Send` and `Sync` are authority declarations

Unsafe marker implementations tell the compiler that transfer or shared
reference across threads is safe. A raw pointer field can suppress automatic
traits, but manually restoring them requires reasoning about the pointed-to
allocation, foreign thread rules, mutation synchronization, callbacks, and
destruction. A mutex around one field does not establish the foreign handle is
thread-safe.

The proof must cover every safe method and drop, not only the operation that
motivated cross-thread use.

## Dynamic tools are evidence, not universal proof

Miri can detect many undefined behaviors in executions it explores under its
supported model. Sanitizers can detect target executions involving address,
thread, or memory errors. Fuzzing explores input space; Loom explores schedules
for modeled synchronization. None explores all code, inputs, compilers,
platforms, or foreign components.

Use several forms of evidence and record their limits. A clean run raises
confidence; it does not replace the safety argument.

## Performance is not presumed

Unsafe code often claims to remove checks, copies, or synchronization. The
change must be measured under a defined workload and must preserve correctness.
Compiler optimizations can make clear safe code equivalent. A slightly slower
safe implementation may be the correct complexity-budget choice when proof and
maintenance costs dominate.

## Guarantee ledger

| Claim                                   | Established by                    | Protected construction        | Boundary preservation                 | Escape hatches         | Does not prove                    | Residual runtime risk            |
| --------------------------------------- | --------------------------------- | ----------------------------- | ------------------------------------- | ---------------------- | --------------------------------- | -------------------------------- |
| slice references initialized allocation | bounds, alignment, lifetime proof | private wrapper               | raw input checked before slice        | unsafe internal helper | business validity of bytes        | allocator or FFI contract breach |
| FFI handle is released once             | ownership wrapper and `Drop`      | private field                 | constructor accepts only owned handle | raw binding layer      | remote resource cleanup succeeded | foreign destructor failure       |
| wrapper is safe to transfer             | complete `Send` proof             | no aliasing escape            | foreign thread contract checked       | direct bindings        | external library bug              | upstream version change          |
| array is fully initialized              | progress guard then conversion    | `MaybeUninit` remains private | errors drop initialized prefix        | manual raw access      | element semantic validity         | panicking foreign destructor     |

## Cost of overapplication

Avoiding unsafe at any cost can create excessive copying or make necessary FFI
impossible. The doctrine permits unsafe where capability and evidence justify
it. It rejects prestige, convenience, and unmeasured optimization as proof.
Centralizing a small reviewed kernel often produces a simpler whole system than
spreading workarounds or unchecked foreign assumptions.
