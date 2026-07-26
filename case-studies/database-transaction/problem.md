# Database transaction: problem

## Domain

A service changes an account and writes an audit/outbox record in one database
transaction. The local workflow is:

```text
begin → active mutation(s) → commit
                         └→ rollback
```

After commit or rollback, the transaction handle must not be reused. A commit
can fail definitively before reaching the server, fail with a confirmed database
rejection, or become ambiguous if the server may have committed before the
connection was lost. Database isolation must protect the actual cross-entity
invariant, not merely group statements.

An external message or payment call is outside the transaction unless a
specific distributed mechanism says otherwise.

## Invariants

| ID | Statement | Mechanism |
|---|---|---|
| TX-01 | Only an active handle may mutate, commit, or roll back. | consuming local handle |
| TX-02 | Commit/rollback consumes local transaction authority. | ownership API |
| TX-03 | Account update uses the version read or reports conflict. | optimistic predicate |
| TX-04 | Cross-row balance/audit intent changes atomically in the database. | transaction plus constraint/isolation |
| TX-05 | Commit error preserves confirmed rejection versus unknown outcome. | structured outcome |
| TX-06 | An external publish is not called database-atomic. | outbox boundary |
| TX-07 | Cancellation does not silently abandon an owned transaction without cleanup policy. | guard/supervision |
| TX-08 | Historical rows decode through current domain validation. | raw row `TryFrom` |

## Boundaries

The database driver controls protocol behavior, transaction isolation,
connection pooling, and error detail. Application code controls handle exposure,
query sequence, operation identity, and result mapping. The chosen database's
documentation and deployment configuration determine whether a connection-loss
commit error can be classified.

Input to the mutation has already passed syntax validation and authorization,
but account state can change between request parsing and transaction execution.
The transaction loads the current version and applies policy to that snapshot.

## Failure points

Important points include failure before begin, after begin, between mutations,
during rollback, while sending commit, after server commit before response,
after confirmed commit before outbox publication worker observes it, and during
external message acknowledgement. The design must state the durable state and
safe next action at every point.

The objective is not to claim the database cannot fail. It is to constrain local
handle use, protect the database-scoped invariant, and preserve uncertainty
when the client lacks commit evidence.
