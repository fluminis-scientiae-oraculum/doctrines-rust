# Database decoding boundary guide

## 1. What is untrusted?

Rows are untrusted domain representations even when the database is controlled.
They may come from old application versions, alternate writers, administrative
repair, incomplete migration, import, replication, backup restoration, relaxed
constraints, or corruption. The driver can prove only that a column decoded
according to its SQL type and protocol. It does not prove current value,
cross-column, or lifecycle invariants.

## 2. What parsing occurs?

The driver parses wire values into a raw row model using physical types such as
integer, text, timestamp, nullable column, or byte buffer. Checked SQL mappings
should avoid lossy casts and surprising numeric truncation. Queries name columns
explicitly when schema order is not a stable contract. Apply row, blob, and
batch limits; paginate large results.

Keep database nullability in the raw model rather than weakening the domain
type.

## 3. What validation occurs?

`TryFrom<RawRow>` validates scalar newtypes, discriminator/associated-field
truth tables, collection bounds, timestamps, versions, and local relationships.
Cross-row or cross-entity invariants require a transaction with suitable
constraints, locks, versions, or isolation. Schema checks, uniqueness, and
foreign keys reinforce stable rules but do not replace the domain conversion.

Invalid historical rows receive a distinct integrity error, not a guessed
default.

## 4. How is a trusted type constructed?

Repository code decodes a raw row and calls protected smart constructors. A
custom driver decoding trait may delegate to the same complete conversion if it
can return structured failure. Otherwise, keep the raw/domain split explicit.

```rust
impl TryFrom<InvoiceRow> for Invoice {
    type Error = InvoiceRowError;

    fn try_from(row: InvoiceRow) -> Result<Self, Self::Error> {
        // Parse newtypes, then match a checked state truth table.
    }
}
```

Successful conversion establishes current local domain invariants for the
observed row version.

## 5. How can construction be bypassed?

Bypasses include ORM derives that assign private fields, unchecked `From` impls,
raw SQL helpers returning domain types, `unwrap_or_default` on invalid columns,
administrative imports using privileged constructors, partial projections named
as complete aggregates, and tests that expose constructors through enabled
features. Unsafe representation casts are forbidden.

Audit every read query, cache population, event replay, migration, and restore
path. A schema constraint is not evidence that every historical row satisfied
it before constraint validation.

## 6. How is failure represented?

Distinguish driver/protocol failure, missing row, cardinality mismatch, physical
type failure, domain-integrity failure, unsupported version, optimistic
conflict, constraint conflict, transient availability, and ambiguous commit
where relevant. Preserve source chains and safe row identity. Invalid data may
fail the request, enter quarantine, or appear through an explicit degraded
administrative type.

Do not silently skip invalid rows in business totals or batches.

## 7. How are unknown or future values handled?

Persist stable enum tags and versions. Readers choose reject, retain raw,
explicit unknown, or migrate. Rolling deployments require a compatibility
matrix. Schema migrations state preconditions, invariant transformation, full
postcondition checks, and forward-repair/rollback semantics. New columns may be
nullable during expansion without making the domain field optional.

An older reader must not reinterpret a new state as a semantically similar old
one without an explicit compatibility rule.

## 8. How is sensitive data protected?

Use parameterized queries. Limit database error and row logging. Redact secrets,
tokens, personal data, and encrypted values; encryption does not make logging
ciphertext harmless. Repository errors carry identifiers needed for repair, not
entire rows. Administrative quarantine and migration tools require least
privilege and an audit trail.

Connection strings and migration credentials never enter fixtures or committed
configuration.

## 9. How is evidence tested?

Unit-test raw-row conversions for every invalid combination. Integration-test
against the actual database and driver for constraints, transactions, enum
encodings, nullability, versions, and error mapping. Run concurrent
read-modify-write tests for optimistic conflicts and relevant isolation
anomalies. Migrate old fixtures and verify the complete postcondition.

Test invalid historical data quarantine, backup restoration, commit connection
loss where the driver can simulate it, and outbox atomicity.

## 10. What remains uncertain?

A valid decoded entity reflects an observation at a time and version. It does
not prove no concurrent writer changed it, a replica is current, a transaction
will commit, or an external side effect occurred. Database durability and
isolation depend on product, configuration, topology, and failure mode.
Ambiguous commit and persistence-after-external-effect require reconciliation.

## Decision table

| Invariant | Boundary treatment |
|---|---|
| scalar range/format | row field to checked newtype |
| mutually exclusive columns | raw row truth table to enum |
| uniqueness | constructor plus database constraint |
| aggregate version | optimistic update predicate |
| cross-row balance | transaction plus appropriate isolation/locking |
| historical invalid data | quarantine or audited migration |
| state plus publication | transactional outbox, not fictional cross-system atomicity |

## Review prompts

- Does every query return a raw row or a fully checked domain conversion?
- Can projections or joins create an incomplete value under a complete type name?
- Which database constraints arbitrate concurrency rather than merely duplicate checks?
- Are migration preconditions and postconditions evaluated over the full affected set?
- How do callers distinguish conflict, integrity failure, and ambiguous commit?
- Which product configuration underlies durability and isolation claims?
