# Glossary

**Ambiguous commit**
: A commit attempt whose client-side result does not establish whether the
database made the transaction durable.

**Data migration**
: A controlled transformation of stored values, including the evidence and
invariant meaning attached to them.

**Durable intent**
: A persisted record that an external action should be attempted, written so
process failure cannot silently forget the obligation.

**Expand-and-contract**
: A rollout sequence that first adds a representation compatible with old code,
migrates readers and writers, then removes the old representation.

**Historical invalid data**
: A stored representation that exists but cannot establish current trusted
domain invariants.

**Inbox**
: Durable consumer-side recording used to recognize and coordinate repeated
message delivery.

**Optimistic concurrency**
: A strategy that performs an update only if a previously observed version or
predicate remains current.

**Outbox**
: A durable publication-intent record written in the same local transaction as
the associated domain transition.

**Persistence model**
: A representation designed for storage shape, compatibility, and physical
decoding, not automatically a trusted domain entity.

**Quarantine**
: Isolation of invalid stored evidence with identity and diagnostics so it can
be audited and repaired without entering trusted operations.

**Schema constraint**
: A database-enforced predicate such as nullability, uniqueness, referential
integrity, or a check expression.

**Storage discriminator**
: A stable encoded value selecting one variant or lifecycle state in a durable
representation.
