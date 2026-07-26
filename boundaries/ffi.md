# FFI boundary guide

## 1. What is untrusted?

Foreign pointers, lengths, return codes, discriminants, callbacks, user-data
contexts, thread origin, string bytes, handles, and allocation provenance are
untrusted until established by the foreign contract and checked locally.
Generated bindings reduce transcription error but do not prove the library
implementation obeys its header or that Rust usage satisfies every lifetime and
thread rule.

FFI is both a trust boundary and an unsafe proof boundary.

## 2. What parsing occurs?

Raw declarations parse ABI-level values: integer widths, pointers, structs,
unions, and opaque handles. Convert integer lengths with checked arithmetic and
correct byte/element units. Validate nullability before access. Keep arbitrary
string bytes in a representation compatible with the specified encoding until
validated. Return-code and out-parameter combinations form a raw response type.

Never construct a typed reference or enum before Rust validity is established.

## 3. What validation occurs?

Validate ABI and target, representation layout, pointer alignment and bounds,
initialization, discriminants, string encoding, handle state, ownership,
callback lifetime, thread affinity, and error combinations. Business values
then pass normal domain constructors. Cross-call lifecycle rules belong to a
safe wrapper or explicit unsafe caller contract.

If a foreign library can mutate a buffer asynchronously, validation must occur
after exclusive completion evidence or the buffer cannot be exposed as an
immutable Rust slice.

## 4. How is a trusted type constructed?

Keep raw bindings in a private low-level module. A wrapper validates outputs,
copies or takes ownership according to the contract, and constructs safe domain
types. Opaque handles live in private fields with a matching destructor.
Borrowed outputs receive the shortest lifetime tied to the owning handle.

A safe public API is allowed only when all safety obligations can be enforced
for every safe caller. Otherwise expose a narrowly scoped `unsafe fn` with a
complete `# Safety` section.

## 5. How can construction be bypassed?

Bypasses include public raw bindings, transmuting foreign integers to enums,
assuming `repr(Rust)` layout, building slices from unchecked length, forging
lifetimes, implementing `Send`/`Sync` without upstream guarantees, freeing with
the wrong allocator, retaining stack callback context, and allowing panics to
cross an incompatible ABI.

Feature flags and alternate targets can compile different bindings, so audit the
complete supported matrix.

## 6. How is failure represented?

Map foreign status codes to structured Rust errors while preserving unknown
codes and source context. Distinguish invalid foreign output, library rejection,
resource exhaustion, unsupported feature, callback panic containment, and
indeterminate external effect where applicable. Do not call `last_error` unless
the API contract says it is meaningful for the specific failure.

Destructors cannot report ordinary errors through `Drop`; provide explicit
close when cleanup failure matters and make drop behavior conservative.

## 7. How are unknown or future values handled?

Represent unknown integer codes as raw/unknown variants rather than transmute.
Use size/version fields in extensible structs according to the C API contract.
Bindings and wrappers track library and ABI versions. New optional fields remain
zeroed or initialized exactly as required. Dynamic symbol absence becomes a
structured unsupported-feature result.

Do not assume an undocumented reserved value remains unused.

## 8. How is sensitive data protected?

Minimize copies of secrets into foreign memory, but state zeroization limits
across allocators and library internals. Avoid debug output of buffers and
handles. Define ownership and cleanup for credentials on every error path.
Foreign crash dumps and logging can expose values outside Rust controls.

Callbacks must not capture broader authority or secrets than required. Pointer
addresses can be sensitive and should not appear in routine public diagnostics.

## 9. How is evidence tested?

Compare bindings to authoritative headers; use generated/layout assertions and
C-side integration tests. Test null, zero length, maximum length, unknown codes,
invalid encoding, partial output, allocation failure where injectable,
callback reentrancy, callback on unexpected threads, panic containment, and
double-close prevention. Run Miri on pure-Rust wrapper paths where supported and
sanitizers/Valgrind-like tools on real foreign integration.

Test every supported target ABI. Record tool blind spots and foreign-library
versions.

## 10. What remains uncertain?

A sound wrapper cannot prove the foreign implementation lacks memory errors,
races, or undocumented behavior. A successful call may not prove an external
device or service remains available. Resource release can fail internally even
when the ABI returns no status. Library upgrades, compile flags, allocator
changes, and target ABIs can invalidate premises and trigger re-audit.

## FFI contract table

| Dimension   | Required record                                        |
| ----------- | ------------------------------------------------------ |
| ABI/layout  | calling convention, `repr`, widths, alignment, targets |
| pointer     | nullability, bounds, mutability, provenance            |
| ownership   | borrow/take/give, lifetime, matching release           |
| strings     | encoding, length, terminator, interior null            |
| errors      | status/out-param rules, unknown code                   |
| callbacks   | retention, thread, reentrancy, unregister              |
| unwind      | catch, abort, or supported unwind ABI                  |
| concurrency | affinity, synchronization, `Send`/`Sync` proof         |
| allocation  | allocator of origin and cleanup                        |
