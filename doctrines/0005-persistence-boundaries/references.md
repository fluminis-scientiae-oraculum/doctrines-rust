# References

- [Rust standard library: `TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)
  defines fallible conversion used to separate raw rows from trusted domain
  values.
- [PostgreSQL documentation: constraints](https://www.postgresql.org/docs/current/ddl-constraints.html)
  documents checks, uniqueness, primary keys, and referential constraints.
- [PostgreSQL documentation: transaction isolation](https://www.postgresql.org/docs/current/transaction-iso.html)
  describes phenomena and guarantees for its isolation levels. Other databases
  require their own primary documentation.
- [PostgreSQL documentation: explicit locking](https://www.postgresql.org/docs/current/explicit-locking.html)
  documents row and table lock behavior and deadlock considerations.
- [PostgreSQL documentation: enumerated types](https://www.postgresql.org/docs/current/datatype-enum.html)
  illustrates product-specific persisted-enum properties and evolution limits.
- [Serde enum representations](https://serde.rs/enum-representations.html) and
  [custom conversion attributes](https://serde.rs/container-attrs.html#try_from)
  document serialization mechanisms relevant to durable formats.
- [CloudEvents specification](https://github.com/cloudevents/spec/blob/main/cloudevents/spec.md)
  provides an example of a versioned event envelope and stable context
  attributes.

These sources establish mechanics for particular language, format, or database
boundaries. The repository adds invariant mapping, migration evidence,
quarantine, and guarantee-ledger review requirements.
