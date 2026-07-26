# Glossary

**Acknowledgement ambiguity**
: Uncertainty caused when processing may have completed but acknowledgement was
  lost or not durably coordinated.

**Compensation**
: A new action intended to mitigate a prior effect. It is not equivalent to
  erasing history and may fail independently.

**Deduplication**
: Recognition of a previously seen logical identity to suppress repeated
  processing or replay a stored result.

**Fencing token**
: A monotonically ordered authority value checked by the protected resource so
  operations from stale lease holders can be rejected.

**Idempotency key**
: Stable identity used by a receiver to bind repeated attempts of one logical
  operation to one scoped result.

**Logical operation**
: One domain intent, potentially represented by several transport attempts.

**Reconciliation**
: Acquisition and evaluation of authoritative evidence to resolve or continue
  an unknown outcome.

**Replay horizon**
: The maximum period over which a prior message or request identity may
  legitimately return.

**Saga**
: A distributed workflow of independently committed actions with explicit
  follow-up or compensating actions.

**Unknown outcome**
: A state in which available evidence does not establish either execution
  success or definitive rejection.
