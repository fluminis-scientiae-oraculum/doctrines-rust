# Typestate: connection states and consuming transitions

[`src/lib.rs`](src/lib.rs) holds two small protocols where the type, not a
runtime flag, records which operations are currently legal.

## What it establishes

`Connection<Closed>` and `Connection<Open>` expose different methods. `send`
exists only on the open form, so calling it on a closed handle is a compile
error rather than a runtime check someone has to remember to write. `connect`
and `close` consume the handle and return the successor state, so the previous
state cannot be used again afterwards.

`ActiveTransaction` applies the same shape to a resource whose end is
irreversible: `commit` and `rollback` both take `self` by value, so a committed
transaction cannot be committed twice or rolled back after the fact. The
compiler enforces the sequencing that an `is_open` boolean field only documents.

## What it does not establish

`Connection<Open>` proves that this value came from a successful local
`connect`, and nothing more. The peer can fail before the next `send`, so the
type is evidence about a local transition, not about the state of a remote
system. No state here survives a process restart. Distributed outcomes belong to
[`distributed-outcomes`](../distributed-outcomes/README.md); a protocol whose
successor varies by stage belongs to
[`staged-protocol`](../staged-protocol/README.md).

## Evidence

Five unit tests cover the open state following only a successful connect, the
fallibility of both connect and send, the transaction that consumes its terminal
state, the empty-mutation rejection, and receipt issuance up to sequence
exhaustion. The rejections a passing test cannot show — sending on a closed
handle, reusing a consumed transaction — are compile-fail cases in
[`doctrine-compile-fail`](../compile-fail/README.md), because a test that
compiles cannot demonstrate that a program does not.

```text
cargo test --locked -p typestate
```

## Doctrine

Cited by [RUST-DOC-0001](../../doctrines/0001-invalid-states/README.md) and
[RUST-DOC-0004](../../doctrines/0004-concurrency-and-async/README.md), and by
the [typestate](../../patterns/typestate.md),
[consuming transitions](../../patterns/consuming-transitions.md), and
[hybrid state machines](../../patterns/hybrid-state-machines.md) patterns.
