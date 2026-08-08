# Rationale

## Storage is historical evidence

A row proves that bytes were accepted by a storage path. It may have been
written by an older binary, another service, an administrative tool, a partial
migration, a restored backup, or an import. Even if the database faithfully
enforces its schema, that schema may encode fewer or different invariants than
the current domain model.

Consequently, `FromRow`-style convenience cannot be allowed to forge
`PositiveMoney`, `VerifiedEmailAddress`, or `AuthorizedPayment`. Decode raw
storage values first, then call the same checked constructor used at other trust
boundaries. Failed conversion is evidence of a data-integrity incident, not a
reason to weaken the type.

## Separate representations clarify evolution

Storage favors stable encodings, compatibility fields, nullable rollout stages,
and query efficiency. Domain types favor precise legal states, private
construction, and behavior-oriented data. Combining them can be appropriate for
a simple immutable record, but it often causes storage nullability to leak into
business operations or domain refactors to rewrite durable history.

A raw row can represent exactly what exists, including malformed historical
combinations. A fallible conversion then establishes the stronger domain
evidence. This separation also gives quarantine tooling a representation for
invalid data without pretending it is valid.

## Defense in depth

Domain constructors protect normal application paths. Database constraints
protect against alternate writers and concurrency races that application
prechecks cannot see. Neither makes the other unnecessary. A uniqueness
constraint can arbitrate concurrent inserts; the domain still needs a
machine-actionable conflict result. A check constraint can reject zero amounts;
the `PositiveMoney` constructor still protects values before storage and after
decoding.

## Migrations change meaning

Adding a non-null column, splitting one state into two, changing money scale, or
normalizing an identifier transforms domain evidence. A safe migration
identifies existing records that violate the target, decides how evidence will
be established, and verifies the postcondition. Inventing a default can be
dishonest: filling `verified_at` with migration time does not prove verification
occurred.

## Persisted enums are protocols

Source variants are easy to rename. Durable discriminators are not. Text names
improve inspection but still need stable mapping. Integer tags avoid spelling
coupling but require a registry. Native database enums have product-specific
migration and compatibility behavior. Unknown variants can be rejected for
closed internal data, retained as raw evidence, or represented explicitly for
forward-compatible consumers.

The choice depends on rolling deployment, downgrade, replay, and public API
needs. Exhaustive matching in current Rust code does not make historical storage
closed forever.

## Transactions protect scoped invariants

A transaction is useful only relative to its isolation semantics and the
operations inside it. Reading availability, checking it in application code,
then later updating without a constraint or lock can race another writer.

Lost-update protection is not a complete isolation argument. Suppose two
transactions each read that at least one of two operators remains on duty, then
each marks a different operator off duty. Per-row version predicates both
succeed because the writes are disjoint, yet the cross-row invariant is false
after both commits. This is write skew. Snapshot isolation can permit it;
serializable isolation, a predicate-level lock, an invariant-enforcing
constraint, or another coordination protocol may prevent it, subject to the
selected product's exact contract.

No generic statement such as "inside a transaction" establishes serializable
business behavior. Review must connect the invariant to actual queries,
constraints, locks, configured isolation, prevented anomaly, and residual
anomaly set.

## Commit can be ambiguous

A client may send a commit and lose the connection before receiving the result.
Depending on protocol and driver evidence, the client may not know whether the
database committed. Treating every commit error as rollback can duplicate later
work; treating it as success can hide lost data. The operation needs identity,
observation, and reconciliation appropriate to the database and application.

A consuming transaction handle prevents accidental local reuse after an
attempt. It does not itself prove the database outcome.

## Persistence and external effects

A database cannot normally roll back an email already sent or a payment already
captured. Calling the external service inside a database transaction also holds
resources while waiting and still permits ambiguous combinations:

- external success followed by database rollback;
- database commit followed by publication failure;
- timeout after the external service executed;
- process loss between two steps.

A transactional outbox stores domain change and publication intent atomically
in one database. A publisher later delivers with retries. This closes the
specific "committed state but forgotten intent" gap; it does not create
exactly-once delivery. Consumers still need deduplication or idempotent effects,
and operators need lag and poison-message handling.

## Invalid historical data

Quarantine preserves two truths: the stored bytes exist, and they do not satisfy
A repair must establish evidence, not merely call an unchecked constructor.

Availability pressure can make rejection uncomfortable, but weakening a trusted
type makes every downstream use uncertain. An explicit `InvalidHistoricalRow`
or degraded read model contains the uncertainty.

## Guarantee ledger

| Claim                                       | Established by                     | Protected construction | Boundary preservation       | Escape hatches          | Does not prove                      | Residual runtime risk             |
| ------------------------------------------- | ---------------------------------- | ---------------------- | --------------------------- | ----------------------- | ----------------------------------- | --------------------------------- |
| decoded email satisfies syntax policy       | checked row conversion             | private newtype field  | all readers use `TryFrom`   | audited internal import | ownership or deliverability         | policy changes, corrupt row       |
| update used current aggregate version       | version predicate affected one row | repository API         | conflict preserved          | administrative repair   | absence of all business races       | retry conflict, isolation anomaly |
| outbox intent shares domain commit          | same local transaction             | repository operation   | publisher reads durable row | direct DB write         | single delivery or consumer success | duplicate, delay, poison message  |
| transaction handle cannot be reused locally | consuming commit/rollback          | private fields         | API lifecycle               | driver internals        | definite remote commit result       | connection loss ambiguity         |

## Cost of overapplication

Separate models add conversion code. Version envelopes add fields. Constraints
and strict decoding complicate rollout. Transactions and locks reduce
concurrency. Outboxes require workers and retention. These costs are justified
by consequential invariant and recovery needs, not ritual. Ephemeral,
rebuildable, bounded caches may use simpler handling. The design still must say
why loss or incompatibility is acceptable.
