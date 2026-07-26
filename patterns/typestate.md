# Typestate

## 1. Problem

A locally controlled handle supports operations only after certain transitions.
Calling `send` before `connect`, capturing before authorization, or reusing a
closed transaction is programmer misuse. Runtime checks repeat in every method
and discover the problem only during execution.

## 2. Forces

The protocol has few stable states and a mostly static API. One owner controls
the handle and transitions. State-specific methods improve callers. Transitions
may perform fallible I/O. The state may later need persistence, heterogeneous
collections, dynamic dispatch, or runtime inspection. Generic diagnostics,
compile time, and code size matter.

## 3. Weak representation

```rust
struct Connection {
    open: bool,
}

impl Connection {
    fn send(&mut self, bytes: &[u8]) -> Result<(), SendError> {
        if !self.open { /* runtime rejection */ }
        // ...
    }
}
```

Every method carries a branch, and the public API advertises operations that are
illegal for the current state.

## 4. Improved representation

```rust
struct Closed;
struct Open;

struct Connection<State> {
    transport: Transport,
    state: PhantomData<State>,
}

impl Connection<Closed> {
    fn connect(self) -> Result<Connection<Open>, ConnectError> { /* ... */ }
}

impl Connection<Open> {
    fn send(&mut self, bytes: &[u8]) -> Result<Receipt, SendError> { /* ... */ }
}
```

Marker zero-sized types carry compile-time local protocol evidence. Consuming
transitions prevent reuse of the prior handle.

## 5. Exact guarantee gained

Safe code holding `Connection<Closed>` cannot call methods implemented only for
`Connection<Open>`. A consuming transition can ensure the previous handle is no
longer usable after success or after a deliberately consuming attempt. The
state parameter records that the local transition returned successfully.

## 6. Guarantees not gained

`Connection<Open>` does not prove the remote peer remains reachable. `Authorized`
does not prove a later capture succeeds. Typestate does not establish external
effects, persisted status, or authorization unless construction actually
obtained that evidence. Fallible operations remain `Result`; timeouts may create
unknown outcomes.

## 7. Boundary considerations

External input determines runtime facts and must first decode into a runtime
representation. Do not deserialize a marker state directly from an untrusted
tag. A checked restoration service may inspect a persisted status, validate
resources and authority, then issue a local handle whose proof is accurately
scoped.

## 8. Persistence considerations

Generic state types are awkward for heterogeneous rows and evolving storage.
Persist a stable runtime enum and associated evidence. Rehydrate typestate only
through checked code. A hybrid state machine often uses typestate for one
in-process operation and a runtime enum for durable lifecycle. Never treat
serialized marker spelling as proof.

## 9. Testing evidence

Unit-test successful and failed transitions, resource cleanup, and state
payloads. Compile-fail tests prove illegal state-specific calls and consumed
handle reuse. Integration-test external failure after a locally successful
transition. Test decoding/restoration separately. Measure generic
monomorphization if the state/API set is large.

## 10. Costs

State parameters spread through signatures, trait bounds, mocks, and error
types. Fallible consuming transitions need careful recovery ergonomics. Many
orthogonal states create a type cross-product. Dynamic dispatch and containers
may require erasure. Monomorphization increases code size. Compiler diagnostics
can become harder than a clear runtime enum.

## 11. When not to use it

Do not use typestate for externally determined, frequently inspected, persisted,
large, plugin-defined, or highly dynamic states. Do not use it when callers
must switch over heterogeneous states at runtime. Do not encode every boolean
property as a marker. Prefer a runtime enum or plain validation when complexity
exceeds prevented misuse.

## 12. Related doctrines

RUST-DOC-0001 limits typestate to proportionate locally controlled sequencing.
RUST-DOC-0003 covers ownership-consuming authority. RUST-DOC-0004 governs async
fallibility and cancellation. RUST-DOC-0005 governs persistence.

## 13. Executable example

See [`../examples/typestate/src/lib.rs`](../examples/typestate/src/lib.rs) and
the compile-fail cases under [`../examples/compile-fail/ui/`](../examples/compile-fail/ui/).

## 14. Worked application

A `Connection<Closed>` owns local transport configuration. `connect(self)`
attempts I/O and returns `Connection<Open>` only after the local connect
protocol reports success. `send(&mut self)` remains fallible because the peer
can disappear immediately. `close(self)` may also fail; its error must state
whether the caller receives recoverable local ownership or only an uncertain
cleanup result.

This API is appropriate for one owned connection. It is less suitable for a
dashboard holding thousands of persisted connection statuses or a plugin API
whose states are discovered dynamically. There, a runtime enum with explicit
operations usually yields clearer storage, dispatch, and diagnostics.

## 15. Review prompts

- Is state locally controlled rather than merely observed externally?
- Is the graph small enough to understand in signatures?
- Does each fallible consuming transition preserve recovery evidence honestly?
- Can marker construction be forged?
- Are remote liveness and success claims explicitly excluded?
- Must instances be persisted, erased, or stored heterogeneously?
- Would a runtime enum produce simpler caller behavior?
- Has monomorphization and diagnostic complexity been assessed?
