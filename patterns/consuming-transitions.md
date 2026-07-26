# Consuming transitions

## 1. Problem

After commit, close, capture, submit, or token redemption, the prior handle must
not be reused. A mutable method can update an internal flag, but aliases or
missed checks retain an apparently usable value. The compiler cannot help if the
API leaves the old state alive.

## 2. Forces

One owner controls the lifecycle. The transition may be infallible, fallible
before any change, partially executed, or externally ambiguous. Callers may need
the original handle back after a recoverable failure. Resource cleanup and
destructors matter. Persistent state remains a separate runtime concern.

## 3. Weak representation

```rust
impl Transaction {
    pub fn commit(&mut self) -> Result<(), CommitError> {
        self.committed = true;
        // ...
    }
}
```

The same handle remains callable. Every later method must inspect a flag.
Aliases through shared containers make lifecycle authority unclear.

## 4. Improved representation

```rust
impl ActiveTransaction {
    pub fn commit(self) -> Result<CommitReceipt, CommitFailure> {
        // self cannot be used by the caller after the attempt
    }
}
```

For a failure known to occur before the transition, return
`Result<Committed, (ActiveTransaction, CommitError)>`. For ambiguous external
execution, consume the handle and return an explicit unknown/reconciliation
state rather than restoring authority dishonestly.

## 5. Exact guarantee gained

After passing an owned value to a consuming method, safe caller code cannot use
that same value again. A returned successor type can expose only successor
operations. Returning the original handle only on proven pre-transition failure
can preserve ergonomic retry without weakening lifecycle evidence.

## 6. Guarantees not gained

Consumption does not prove an external commit, close, or capture occurred. Drop
does not guarantee remote rollback. Another separately constructed handle may
still act on the same resource. Persistence may contain stale status. Unsafe or
privileged constructors can bypass local lifecycle if misused.

## 7. Boundary considerations

External responses determine which successor evidence can be constructed.
Authenticate response source and preserve structured errors. If response loss
makes execution unknown, return an unknown outcome with stable operation
identity. Cancellation of an async consuming transition needs a guard or owner
that accounts for the consumed resource.

## 8. Persistence considerations

Store runtime lifecycle state and optimistic version independently. A local
consuming handle can own a transaction connection, while the database remains
the authority for durable status. Rehydration checks current state and issues a
new handle; it cannot deserialize the old moved value. Commit ambiguity needs
database-specific reconciliation.

## 9. Testing evidence

Compile-fail tests demonstrate reuse after move is rejected. Unit tests cover
every transition result, returned recovery handle, and destructor behavior.
Fault tests cancel or disconnect at each partial step. Concurrent tests ensure
one operation identity cannot be claimed twice where required. Boundary tests
preserve unknown rather than returning the prior state.

## 10. Costs

Ownership-consuming APIs can complicate error handling and chaining. Returning a
handle with error creates larger types. Async transitions may need background
completion or reconciliation. Generic successor states can spread through API
signatures. Callers sometimes need runtime collections of mixed states, where a
single runtime enum is simpler.

## 11. When not to use it

Do not consume immutable value objects merely for style. Do not return a prior
handle after an error if the transition may already have happened. Do not force
consumption where shared observation is the real model. A mutable runtime state
machine is appropriate when many actors inspect or coordinate one persisted
entity.

## 12. Related doctrines

RUST-DOC-0001 governs legal transitions and external fallibility.
RUST-DOC-0003 governs custody and RAII. RUST-DOC-0005 covers transaction
lifecycle and commit ambiguity. RUST-DOC-0006 covers unknown outcomes.

## 13. Executable example

See transaction and connection flows in
[`../examples/typestate/src/lib.rs`](../examples/typestate/src/lib.rs) and the
reuse compile-fail case under
[`../examples/compile-fail/ui/`](../examples/compile-fail/ui/).

## 14. Worked application

A single-use invitation token can be redeemed by passing ownership to
`redeem(self)`. Validation failure before contacting storage can return
`(self, ValidationError)` if correcting local context and retrying is safe.
After an atomic storage claim is attempted, returning the original token would
misrepresent its status; the result should be redeemed, rejected as already
used, or unknown with a lookup identity.

Similarly, a database `commit(self)` blocks local reuse but cannot declare the
remote outcome after connection loss. Consumption protects the caller's handle
lifecycle. A transaction operation ID and database observation protect the
distributed outcome.

## 15. Review prompts

- Is reuse actually invalid and consequential?
- At which point does the old authority cease to be truthful?
- Can a failure safely return the original value?
- Does cancellation consume, recover, or reconcile the resource?
- Are destructor side effects limited to what RAII can guarantee?
- Can a second handle target the same external resource?
- Is persistent versioning needed in addition to local ownership?
- Would a clear runtime guard be more ergonomic at equal safety?
