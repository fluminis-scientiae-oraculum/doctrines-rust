# Boundary validation: untrusted input becomes a checked domain value

[`src/lib.rs`](src/lib.rs) holds the two conversions every application performs
and most applications leave implicit: a deserialized request body becoming a
domain type, and a database row becoming the same domain type.

## What it establishes

`RawContactDto` is the Serde shape. `ContactRow` is the persistence shape.
Neither is the domain type. `Contact` is reachable only through
`TryFrom<RawContactDto>` and `TryFrom<ContactRow>`, so every field arrives
checked and a failure is a named `ContactError` variant rather than a panic or a
default.

Both directions reuse the checked values from
[`validated-newtypes`](../validated-newtypes/README.md) instead of re-deriving
the rules, so the request path and the storage path cannot disagree about what a
valid contact is. The crate treats a stored row as untrusted for the same reason
it treats a request as untrusted: a row was written by an earlier version of the
code, under earlier rules.

## What it does not establish

There is no database and no HTTP server here — `ContactRow` is a struct, not a
driver result, and no connection, transaction, or migration is exercised. The
example proves the conversion refuses bad input; it does not prove any
particular storage engine hands over the shape the struct expects.

## Evidence

Five unit tests cover the Serde path routed through the checked conversion, a
rejected request field, a historical row that no longer satisfies current rules,
an unknown persisted status, and the fact that the boundary applies the same
email policy as the domain.

```text
cargo test --locked -p boundary-validation
```

## Doctrine

Cited by [RUST-DOC-0005](../../doctrines/0005-persistence-boundaries/README.md),
[RUST-DOC-0001](../../doctrines/0001-invalid-states/README.md),
[RUST-DOC-0002](../../doctrines/0002-error-modeling/README.md), and
[RUST-DOC-0008](../../doctrines/0008-testing-and-evidence/README.md), and by the
[smart constructors](../../patterns/smart-constructors.md) pattern. The wider
boundary guidance lives under [`boundaries/`](../../boundaries/README.md).
