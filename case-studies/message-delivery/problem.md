# Message delivery: problem

## Domain

An at-least-once broker delivers account commands to a consumer. Each valid
command may update a local database and request an external notification. The
broker can redeliver after consumer crash or lost acknowledgement. Multiple
producers and partitions can reorder messages. Malformed or unsupported
messages must not block a hot partition forever.

The consumer must distinguish message receipt, claim, local completion,
external-effect status, durable progress, and broker acknowledgement.

## Invariants

| ID     | Statement                                                      | Classification            |
| ------ | -------------------------------------------------------------- | ------------------------- |
| MSG-01 | Message identity is stable and scoped to producer/source.      | boundary/distributed      |
| MSG-02 | Same identity with different payload is a conflict.            | authority/integrity       |
| MSG-03 | Local durable effect and processed identity are atomic.        | persistence               |
| MSG-04 | Redelivery does not repeat the protected local effect.         | distributed               |
| MSG-05 | External effects use their own stable operation identity.      | distributed               |
| MSG-06 | Acknowledgement loss is not treated as processing failure.     | distributed               |
| MSG-07 | Required per-aggregate order uses explicit version/gap policy. | transition                |
| MSG-08 | Poison messages are isolated, bounded, and auditable.          | lifecycle                 |
| MSG-09 | Schema/unknown-value behavior is versioned.                    | boundary                  |
| MSG-10 | Consumer concurrency and queues are bounded.                   | environmental/concurrency |

## Boundaries

The broker envelope supplies event ID, producer/source, type, schema version,
aggregate key/version, correlation and causation IDs, timestamp, and payload.
Broker authentication can establish transport producer identity; payload
authorization and domain validity still require checks.

The database is one local atomic resource. An external email or HTTP API is
another. An inbox can coordinate message identity with a local database update,
but it cannot make the remote effect part of that transaction. The consumer
must persist remote intent or unknown outcome.

## Failure points

Consider crashes:

- before inbox claim;
- after claim before local effect;
- after local effect before commit;
- after commit before acknowledgement;
- after external request execution before response;
- after response before local outcome persistence;
- while moving poison data to quarantine.

Also consider duplicate concurrent deliveries, same ID with altered bytes,
event version gaps, old schema replay, dead-letter replay, expired dedup records,
consumer shutdown, and broker outage.

## Objective

The improved protocol should guarantee a scoped one-time local mutation per
retained message identity, not universal exactly-once delivery. It should make
external uncertainty visible, preserve causality for audit, apply backpressure,
and define safe replay.
