# Messaging boundary guide

## 1. What is untrusted?

Message bytes, headers, event identity, producer identity, schema tag,
timestamp, partition key, ordering metadata, delivery count, and broker
attributes are untrusted until validated against the broker and application
contract. A message may be duplicated, delayed, reordered, replayed, truncated,
poisoned, produced by an old writer, or deliberately hostile. Broker
acknowledgement is not domain authorization.

At-least-once delivery makes repeated valid messages normal, not exceptional.

## 2. What parsing occurs?

Apply broker frame and consumer batch limits before payload decoding. Verify
compression and decompression bounds. Parse a versioned envelope containing
stable event/message ID, source, type, schema version, correlation/causation,
timestamp, partition or aggregate key, and payload. Then parse the payload into
a raw DTO.

Do not allocate according to an untrusted collection length or retain unlimited
unknown headers. Preserve raw evidence needed for quarantine within sensitivity
limits.

## 3. What validation occurs?

Validate envelope version, producer/source authorization, identifier syntax,
payload shape, domain values, aggregate version, and allowed command/event type.
For commands, authenticate the producer and authorize the requested operation.
For events, distinguish a statement from authority to mutate local state.

Validate ordering or predecessor rules at the aggregate boundary. Duplicate
identity is handled before non-idempotent effects. Cross-entity changes use the
required local transaction.

## 4. How is a trusted type constructed?

Decode envelope and raw payload, then call domain constructors. Construct a
trusted command only after source and authority evidence is attached. Claim a
stable message ID in an inbox when duplicate suppression protects a durable
local effect. Atomically record inbox identity, local mutation, and durable
outbox intent where one database can coordinate them.

External effects receive a separate stable operation ID and explicit unknown
outcome; inbox presence alone does not prove the external effect.

## 5. How can construction be bypassed?

Bypasses include deriving domain deserialization, trusting a topic name as
authorization, direct administrative replay into business handlers, using
payload business IDs as delivery identity without scope, in-memory-only dedup,
calling handlers outside the inbox transaction, and converting unknown schemas
to a default variant.

Dead-letter and replay tools are privileged writers. They must preserve original
identity and pass the same validation unless an audited repair establishes new
evidence.

## 6. How is failure represented?

Distinguish temporary broker/consumer failure, malformed envelope, unsupported
version, validation rejection, unauthorized source, duplicate already
completed, duplicate still in progress, ordering gap, stale version, domain
conflict, poison message, external unknown outcome, and internal defect.

Retry only categories with safe semantics. Poison messages enter a bounded
quarantine or dead-letter process with redacted diagnostics, owner, and replay
policy. Acknowledgement errors preserve ambiguity.

## 7. How are unknown or future values handled?

Use versioned envelopes and stable external tags. Define forward/backward
compatibility for rolling producers and consumers. An unknown event type may be
ignored only if omission is safe and observable; commands should usually reject
unsupported meaning. Retaining raw unknown payloads supports later replay but
requires size and sensitivity controls.

Schema registry compatibility is supporting evidence, not proof every semantic
change is compatible. Event meaning changes require new versions or types.

## 8. How is sensitive data protected?

Minimize payload data and avoid credentials in messages. Encrypt where threat
model requires, while preserving broker metadata sensitivity. Logs contain
message IDs, categories, and safe fingerprints rather than bodies. Dead-letter
queues need access control, retention, deletion, and encryption matching the
original data. Correlation IDs can become personal tracking data and require
policy.

Signatures and encryption do not validate domain policy or prevent authorized
producers from sending invalid values.

## 9. How is evidence tested?

Contract-test envelope and every supported schema version. Integration-test the
real codec and broker where feasible. Inject duplicates before, during, and
after local commit; lost acknowledgements; out-of-order and missing versions;
consumer restart; poison payload; dead-letter replay; and retention expiry.

Assert inbox atomicity, outbox publication retry, bounded backpressure, channel
closure, and graceful shutdown. External-effect tests preserve unknown status
after acknowledgement loss.

## 10. What remains uncertain?

Broker acceptance does not prove consumer processing. Consumer acknowledgement
does not prove an uncoordinated external side effect. Partition order is not
global order. Event time may differ from processing time and clocks may skew.
Deduplication stops working after its retained identity expires. Replayed
historical facts may no longer authorize current actions.

## Delivery decision table

| Scenario                               | Required behavior                               |
| -------------------------------------- | ----------------------------------------------- |
| repeated message ID, completed locally | return/reuse recorded outcome                   |
| repeated ID, different fingerprint     | reject as conflict/security event               |
| gap in required aggregate sequence     | wait, fetch, or reconcile; do not guess         |
| invalid payload                        | quarantine/dead-letter with bounded diagnostics |
| effect completed, ack lost             | deduplicate on redelivery                       |
| external effect response lost          | persist unknown and reconcile                   |
| newer unsupported schema               | reject or retain per compatibility policy       |

## Review prompts

- Is stable message identity separate from aggregate/business identity?
- Does inbox claim share an atomic boundary with the protected local effect?
- Are acknowledgement crash points and replay retention explicitly tested?
- Which order exists per producer, aggregate, or partition — and nowhere else?
- Can a privileged replay tool alter identity or bypass current validation?
- Are poison records bounded, access-controlled, and owned through resolution?
