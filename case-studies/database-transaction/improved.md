# Database transaction: improved design

## Local transaction handle

```rust
pub struct ActiveTransaction<'c> {
    inner: DriverTransaction<'c>,
    operation: OperationId,
}

impl ActiveTransaction<'_> {
    pub async fn update_account(
        &mut self,
        expected: AccountVersion,
        change: AccountChange,
    ) -> Result<(), MutationError> {
        // Checked query uses expected version.
    }

    pub async fn commit(self) -> CommitOutcome {
        // self is unavailable after the attempt.
    }

    pub async fn rollback(self) -> Result<RolledBack, RollbackError> {
        // self is unavailable after the attempt.
    }
}
```

Private fields prevent forged active handles. Commit and rollback consume local
authority. Compile-fail evidence demonstrates reuse is rejected. This proves a
caller cannot use that moved handle; it does not prove remote commit outcome.

If begin fails before a transaction exists, the repository returns a structured
begin error. Cancellation while active follows the chosen driver contract: an
owner guard initiates bounded rollback or marks the connection unusable. No
destructor claims externally fallible rollback succeeded.

## Database-scoped invariant

Within one transaction, the repository:

1. loads the account and current version;
2. applies domain policy to current trusted row conversion;
3. updates with `WHERE id = ? AND version = ?`;
4. verifies exactly one affected row or returns `Conflict`;
5. inserts an audit record;
6. inserts an outbox row with the stable operation ID;
7. commits.

Database constraints reinforce ranges, unique operation ID, and references. The
isolation/locking analysis states which anomaly matters. For an invariant
requiring predicate protection beyond a single row, the design uses a suitable
constraint, lock, or documented isolation level and tests concurrent
transactions against the actual database.

## Commit outcome

```rust
pub enum CommitOutcome {
    Confirmed(CommitReceipt),
    Rejected(CommitRejection),
    Unknown { reconciliation: CommitReconciliation },
}
```

The driver adapter classifies only what its protocol evidence supports. A
serialization or constraint rejection confirmed before commit success becomes
`Rejected`. A connection loss after commit may have reached the server becomes
`Unknown`. Failure proven before commit dispatch can support safe retry under
the same operation identity.

`CommitReconciliation` contains operation ID, account ID, expected and intended
version, audit/outbox identity, first attempt, and database target. The
reconciler queries for the unique operation/audit record on a fresh connection.
Presence with the expected fingerprint confirms local commit. Definitive
absence is accepted only under database-specific evidence; otherwise state
remains unknown or escalates.

## External publication

The outbox row shares the database transaction and therefore proves durable
publication intent whenever the transaction is confirmed committed. A bounded
publisher later sends the message. It reuses message ID across retries and
marks progress after acknowledgement. Consumers expect duplicates and use an
inbox or idempotent effect.

The system claims:

- account mutation, audit, and outbox intent are atomic in this database
  transaction under its configured guarantees;
- message delivery is asynchronous and at least once;
- remote consumer effect may require separate reconciliation.

It does not claim a distributed transaction.

## Row integrity and migration

`AccountRow`, `AuditRow`, and `OutboxRow` decode physical values. Fallible
conversion checks money/currency, versions, identifiers, tags, and payload
fingerprints. Historical invalid rows are quarantined. Migrations state
preconditions and scan complete affected sets before strengthening constraints.

## Evidence

Unit tests cover handle error mapping and row conversions. Compile-fail tests
cover reuse after commit. Real database integration tests cover:

- version conflict with two transactions;
- constraints and exact row counts;
- account/audit/outbox atomicity;
- rollback after intermediate error;
- cancellation cleanup/pool behavior;
- configured isolation anomaly;
- duplicate operation identity;
- old-schema migration and invalid history.

Fault tests cut the connection at driver-supported points and verify that
ambiguous results never become confirmed rollback. Process-restart tests show
outbox recovery and duplicate delivery.

## Guarantee ledger

| Claim                                             | Established by                                             | Protected construction | Boundary preservation        | Escape hatches            | Does not prove                         | Residual runtime risk    |
| ------------------------------------------------- | ---------------------------------------------------------- | ---------------------- | ---------------------------- | ------------------------- | -------------------------------------- | ------------------------ |
| only active handle mutates locally                | private `ActiveTransaction` and consuming terminal methods | repository begin       | handle is not serialized     | driver raw API            | DB transaction remains active remotely | connection/pool failure  |
| update used expected version                      | conditional update affects exactly one row                 | repository method      | structured conflict          | direct SQL administration | broader predicate serializability      | concurrent policy change |
| account, audit, and outbox intent commit together | one configured DB transaction                              | repository transaction | checked row reads            | manual DB mutation        | message delivered                      | database failure         |
| commit is unknown after ambiguous disconnect      | adapter classification plus operation record               | explicit variant       | durable reconciliation token | operator decision         | commit or rollback                     | missing DB evidence      |
| publisher reuses one message identity             | durable outbox ID                                          | unique constraint      | consumer sees same ID        | retention/replay tool     | one delivery                           | acknowledgement loss     |

> [!TIP]
> [problem](problem.md) · [naive design](naive.md) · **improved design** · [remaining uncertainty](remaining-uncertainty.md)
> Index: [all case studies](../README.md).
> Executable mechanics live under [`../../examples/`](../../examples/).
