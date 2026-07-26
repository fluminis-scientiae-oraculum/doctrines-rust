# Database transaction: naive design

## Mutable lifecycle

```rust
struct Transaction {
    connection: Connection,
    committed: bool,
}

impl Transaction {
    fn commit(&mut self) -> Result<(), String> {
        self.connection.execute("COMMIT")?;
        self.committed = true;
        Ok(())
    }

    fn execute(&mut self, sql: &str) -> Result<(), String> {
        self.connection.execute(sql)
    }
}
```

The caller can execute after commit or call commit twice. Every method needs a
runtime flag but this one checks none. Raw SQL accepts untyped identifiers and
values. String errors erase conflict, constraint, disconnection, serialization
failure, rollback failure, and ambiguous commit.

## Check then update

The service reads an account outside the transaction, checks its balance and
version in application code, begins a transaction, and writes the new amount
without a version predicate. Another writer can update between read and write.
The later write silently loses it.

An audit row is inserted in the same transaction, which is useful. The code then
publishes a message before commit so consumers see the change quickly. If commit
rolls back, the message describes state that never became durable. Moving
publish after commit creates the opposite gap: process loss can commit state but
forget the message.

## Error fiction

Any commit error triggers:

```rust
let _ = tx.rollback();
return Err("transaction failed".to_owned());
```

If the database committed and the response was lost, rollback on that connection
cannot undo it. The application tells the caller failure and permits a new
operation under a new identity. A duplicate account mutation can follow.
Rollback error is discarded, so connection state and resource cleanup are
unknown.

Cancellation is also ignored. An async request may drop the future while the
transaction holds locks. The pool eventually discards or rolls back the
connection according to library behavior, but the application has no tested
contract or timeout.

## Isolation overclaim

Documentation says "transactions make updates serializable," while the deployed
isolation is never recorded and the query does not lock/version the protected
predicate. The code assumes an `BEGIN` statement automatically prevents every
application anomaly.

## Evidence weakness

Tests use an in-memory map and synchronous fake transaction. It cannot model
driver protocol, isolation, locks, connection loss, ambiguous commit, pool
cleanup, or process crash. A unit test asserts two statements ran in order and
calls that atomicity. No concurrent writer or failure between durable steps is
exercised.

The design is memory-safe but does not preserve lifecycle, conflict, commit
truth, or external publication intent.
