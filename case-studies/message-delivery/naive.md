# Message delivery: naive design

## Consumer loop

```rust
while let Some(message) = broker.next().await {
    let command: Command = serde_json::from_slice(&message.body).unwrap();
    handle(command).await.unwrap();
    message.ack().await.unwrap();
}
```

Derived deserialization constructs trusted command fields directly. Oversized
payloads allocate before limits. Malformed data panics the consumer. Unknown
schema or enum values also panic, causing a restart and repeated poison delivery.
The loop has no bounded concurrency, shutdown ownership, or channel-closure
policy.

## Duplicate processing

`handle` updates a balance, then sends an email. If the process crashes before
acknowledgement, the broker redelivers and both actions repeat. An in-memory
`HashSet` of message IDs is later added. Restart clears it; two consumer
instances have separate sets; it cannot atomically coordinate with the
database.

Another change acknowledges before handling to avoid duplicates. A crash after
ack loses the command permanently. Both choices are described as exactly once,
although one chooses duplicates and the other loss.

## Ordering assumptions

The team assumes broker FIFO is global. Two producers publish aggregate version
8 and 9 through different paths. Version 9 arrives first and is applied; version
8 later overwrites part of the state. Retry and dead-letter replay create more
reordering. No aggregate version or predecessor check exists.

## External effects

The email call uses the broker message ID as a new provider key but regenerates
it during retry. A timeout becomes an error; the consumer returns failure and
the broker redelivers. If email was accepted, another send follows. The database
may already contain the local mutation.

## Poison handling

After several failures, the broker moves a message to a dead-letter queue.
Operations copy the body into a ticket and later republish it with a new ID.
The new ID defeats any deduplication and the ticket exposes personal data. There
is no record of original producer, attempts, validation category, schema, or
repair.

## Evidence weakness

A fake queue delivers each command once, in insertion order, with immediate
acknowledgement. An in-memory repository cannot model transaction atomicity.
The email fake either succeeds or fails before execution. Tests never crash
between steps or run two consumers.

The design compiles and is memory-safe, but it cannot distinguish receipt from
completion, cannot make a scoped one-time local effect, cannot replay safely,
and cannot tell whether an external notification executed.

> [!TIP]
> [problem](problem.md) · **naive design** · [improved design](improved.md) · [remaining uncertainty](remaining-uncertainty.md)
> Index: [all case studies](../README.md).
> Executable mechanics live under [`../../examples/`](../../examples/).
