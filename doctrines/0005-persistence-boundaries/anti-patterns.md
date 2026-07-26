# Anti-pattern catalogue

## The database already validated it

**Weak example.** A row's `String` is placed directly inside a trusted email
newtype because it came from a controlled database.

**Why it fails.** The database may enforce only a physical text type, and other
writers or old data may violate current policy.

**Risk.** Every downstream user receives forged evidence.

**Improved direction.** Decode a raw row and call the checked constructor.

**When justified.** Direct mapping is reasonable only for a type with no
stronger invariant than its physical representation.

## Domain type equals row type

**Weak example.** Domain fields become optional because a rolling migration
needs nullable columns.

**Why it fails.** Temporary storage compatibility becomes permanent invalid
domain state.

**Risk.** Scattered checks and contradictory objects.

**Improved direction.** Keep a raw persistence model and fallibly convert to the
complete domain type.

**When justified.** A simple immutable record may share a representation when
all contracts truly match.

## Derived decoding bypass

**Weak example.** A serialization or ORM derive assigns a private
representation without executing validation.

**Why it fails.** A second construction path establishes less evidence under
the same type name.

**Risk.** hostile or historical data bypasses policy.

**Improved direction.** Use a raw adapter plus `TryFrom` or a manual checked
decoder.

**When justified.** A derive is safe when it delegates to the complete
validated conversion and tests prove rejection.

## Default as invented evidence

**Weak example.** A migration fills missing `authorized_by` with `"system"` so a
new non-null constraint can be added.

**Why it fails.** The value states an event that was not observed.

**Risk.** audit corruption and unauthorized behavior.

**Improved direction.** retain an explicit legacy/unknown state, derive from
authoritative history, or quarantine.

**When justified.** A default is valid for a true policy default that makes no
historical claim.

## Application uniqueness check

**Weak example.** Code queries for an identifier, sees none, then inserts it.

**Why it fails.** Concurrent writers can both pass the check.

**Risk.** duplicate identity and ambiguous lookup.

**Improved direction.** use a unique constraint and map its conflict.

**When justified.** A precheck may improve error messages but cannot be the
enforcement mechanism.

## Transaction therefore safe

**Weak example.** Review approves a cross-row update solely because it uses a
transaction.

**Why it fails.** Isolation level and query shape may permit the relevant
anomaly.

**Risk.** lost updates, overspending, or invalid state.

**Improved direction.** connect the invariant to constraints, locking,
versions, or suitable isolation.

**When justified.** The claim is valid after the actual mechanism is documented
and tested.

## Blind upsert

**Weak example.** An upsert overwrites every column from a stale object.

**Why it fails.** It erases concurrent updates and hides conflicts.

**Risk.** silent data loss.

**Improved direction.** update intended fields with a version predicate or use
commutative operations.

**When justified.** Last-write-wins data such as replaceable cache material may
accept it explicitly.

## Persist Rust variant spelling

**Weak example.** An enum derives text serialization and the resulting variant
name becomes a permanent database value without policy.

**Why it fails.** source refactors become data migrations and unknown values
break older readers.

**Risk.** incompatible rollout and replay failure.

**Improved direction.** define stable external tags and unknown handling.

**When justified.** Disposable, version-locked data may use direct spelling.

## External call inside transaction

**Weak example.** Code sends a message or captures a payment before committing
the database transaction and calls the whole operation atomic.

**Why it fails.** The external effect cannot be rolled back with the database.

**Risk.** effect without state, long locks, and ambiguous retry.

**Improved direction.** persist intent atomically and deliver through a
retriable, observable protocol.

**When justified.** A true shared transaction manager may coordinate specific
resources, but its exact failure boundary must be stated.

## Commit error equals rollback

**Weak example.** Any commit I/O error is mapped to `NotCommitted`.

**Why it fails.** The server may have committed before the response was lost.

**Risk.** duplicate retry or inconsistent reconciliation.

**Improved direction.** classify the outcome according to driver evidence and
retain operation identity.

**When justified.** A protocol may provide definitive non-commit evidence; cite
that mechanism.

## Outbox means exactly once

**Weak example.** A transactional outbox is described as exactly-once message
delivery.

**Why it fails.** publisher acknowledgement can be lost and delivery can repeat.

**Risk.** consumers perform duplicate effects.

**Improved direction.** state the atomic-intent guarantee, use stable message
identity, and design consumer idempotency.

**When justified.** A narrower exactly-once claim may hold inside a specifically
defined transactional boundary with proof.

## Skip invalid rows

**Weak example.** A query iterator silently omits rows that fail domain
conversion.

**Why it fails.** Corruption becomes missing business data with no owner.

**Risk.** incorrect totals, incomplete processing, and prolonged integrity
failure.

**Improved direction.** fail, quarantine, or return an explicit mixed result
with diagnostics.

**When justified.** An administrative scan may continue collecting all invalid
rows, but must report every omission.
