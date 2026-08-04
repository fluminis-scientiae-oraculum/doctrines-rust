# Implementer overlay

## Purpose

Translate approved invariant and boundary artifacts into protected Rust APIs,
runtime checks, persistence mappings, failure models, and executable evidence.
Implement only guarantees the design establishes. If implementation reveals a
missing invariant, effect, or authority decision, update or escalate the plan
instead of silently choosing a broader contract.

## Construction workflow

1. Locate the invariant ID and evidence level.
2. Create a raw or structural input representation where needed.
3. Keep trusted representation fields private.
4. Implement one complete checked constructor or `TryFrom`.
5. Make fallibility and structured errors visible.
6. Expose read/borrow methods that cannot invalidate the value.
7. Restrict privileged and unsafe construction.
8. Route Serde, database, configuration, message, HTTP/RPC, file, and FFI
   inputs through checked construction.
9. Add positive, negative, boundary, and bypass evidence.

Never use derived deserialization or ORM convenience if it assigns trusted state
without the invariant gate. Do not add `From<Raw>` for a fallible conversion.

## State and ownership workflow

Use enums for mutually exclusive dynamic state and place associated data in the
relevant variant. Use consuming transitions when prior-handle reuse is invalid.
Adopt typestate only when the approved complexity decision shows a small,
locally controlled protocol. Keep external operations fallible in every state.

Capability constructors live with the authorizer or resource owner. Avoid
unreviewed `Clone` on authority, handles, single-use tokens, sessions, and
secrets. Define resource cleanup through RAII only for locally controllable
release; provide explicit close/rollback or reconciliation where cleanup can
fail externally.

## Boundary workflow

Implement:

```text
bounded raw input
  → physical parse
  → raw DTO/row/foreign representation
  → checked domain construction
  → authorized operation
  → structured result or explicit unknown
```

Enforce length, allocation, nesting, batch, and concurrency limits before costly
work. Preserve version and unknown-value policy. Redact public failures and
logs. Authentication yields principal evidence; authorization yields
action/resource permission. Client or UI claims do not create authority.

For persistence, decode raw rows and quarantine invalid history. Use
constraints and transaction semantics that actually protect cross-entity rules.
Expose optimistic conflicts. Use outbox/inbox or an approved durable protocol
when state and messaging intent cannot be forgotten independently.

## External effects

Create a stable logical operation ID before dispatch. Reuse identity across
attempts. Bind idempotency to target and payload according to the approved
contract. Do not retry transport errors generically. Preserve:

- confirmed success with evidence;
- confirmed rejection;
- failure proven before dispatch;
- unknown execution with reconciliation identity.

Unknown outcomes are persisted and owned when process loss matters.
Compensation is implemented as a new fallible, idempotency-analyzed operation.

## Concurrency and async

Implement the approved state/task ownership model. Bound task count, channels,
pools, permits, and retry queues. Handle channel closure. Avoid blocking
executor workers; bound blocking work too. Review every `.await` after partial
mutation for cancellation safety. Do not hold synchronous locks across await.
Supervise spawned tasks and implement admission stop, drain, deadline, and
forced shutdown behavior.

Atomics require a happens-before argument. Do not manually implement `Send` or
`Sync` without doctrine 0007 proof.

## Unsafe policy

Avoid unsafe unless explicitly approved. If required, minimize it, state a
complete `SAFETY:` argument, protect the safe abstraction for every safe caller,
document unsafe caller obligations, and add Miri/sanitizer/fuzz/model evidence
where applicable. Never use unsafe to bypass trusted construction or silence
the borrow checker.

## Evidence implementation

Add unit tests for constructor and transition behavior, property tests for
generative invariants, compile-fail tests for important prohibited programs,
integration tests for real boundaries, and fault injection for partial effects.
Compile-fail sources remain minimal. Inspect expected diagnostics before
committing changes. Control clocks, schedules, ports, files, randomness, and
external state for reproducibility.

Do not update snapshots or `.stderr` outputs blindly. Do not accept flaky tests
through permanent retries. Benchmarks remain separate from correctness and
follow doctrine 0009.

## Obligation placement

Encode the obligation rather than describing it. Under RUST-DOC-0011:

- put ordering, invariants, construction restrictions, and capability boundaries into types,
  visibility, schemas, manifests, and configuration;
- add the negative evidence that demonstrates the claimed impossibility, since a claim asserted in
  prose is not enforced by anything;
- generate a derived view from its source, declare that source in the output, and add the drift
  check; never hand-edit a generated artifact;
- add no prose copy of a topology, interface, or invariant an artifact already enforces;
- state the part of a claim the mechanism does not enforce separately, rather than letting the
  enforced part imply it;
- name the external system authoritative for each durable or remote fact, and the check that
  consults it;
- link any rare decision record to the artifacts that remain authoritative for current behavior,
  and keep the record out of the code path it describes.

Keep the enforcing artifact legible: domain names, states named for the facts they establish,
disclosed effects, narrow capabilities, and erasure delayed to a named boundary. An authority
nobody can read produces the prose duplicate this doctrine removes.

## Forbidden implementation claims

Do not state that private fields alone prove constructor correctness; tests
alone prove all inputs; `Open` proves remote liveness; `NonZeroU64` solves money
policy; database rows are trusted; timeout is rejection; outbox is end-to-end
exactly once; or an async rewrite is faster without measurement.

## Escalation and completion

Escalate when code needs a new escape hatch, weaker guarantee, different state
graph, authority expansion, schema incompatibility, unsafe block, dependency,
or retry behavior. Cite the affected invariant and proposed tradeoff.

Implementation is complete when canonical claims match code, all construction
paths are protected, relevant focused reviews pass, evidence covers prohibited
and failure paths, generation reproduces, and every requested local validation
passes. Report any unrun or failing command precisely.
