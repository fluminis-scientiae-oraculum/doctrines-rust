# Smart constructors

## 1. Problem

A type documents an invariant but ordinary constructors, derived decoding, or
mutation permit instances that do not satisfy it. Checks appear near some
callers and are omitted near others. Parsing, normalization, and business policy
are combined without clear evidence levels.

## 2. Forces

Construction may begin from strings, bytes, database columns, or already parsed
values. Callers need actionable errors. Normalization may change representation.
Some checks are pure and stable; others require policy or external observation.
Rust conventions include `new`, `parse`, and `TryFrom`, but naming must reveal
fallibility and evidence.

## 3. Weak representation

```rust
pub struct BoundedName(pub String);

fn make_name(raw: String) -> BoundedName {
    BoundedName(raw)
}
```

Call sites may remember to check length, but public construction does not
protect the documented bound. A later Serde derive can bypass whatever checks
exist.

## 4. Improved representation

```rust
pub struct BoundedName(String);

impl TryFrom<String> for BoundedName {
    type Error = NameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let normalized = value.trim().to_owned();
        validate_name(&normalized)?;
        Ok(Self(normalized))
    }
}
```

Use `parse` when interpreting textual syntax, fallible `new` or `try_from` for
validated values, and a separate domain service when external policy or
cross-entity facts are required.

## 5. Exact guarantee gained

Every value created through the protected construction path has passed the
complete invariant implemented there, after the documented normalization.
Callers receive a stable structured error contract. Centralization makes review
and invariant changes discoverable.

## 6. Guarantees not gained

Constructor correctness is not proven merely by centralization. A pure
constructor cannot establish mailbox ownership, database uniqueness under
concurrency, authorization, or mutable external facts. Normalization may have
policy and collision consequences. A constructor does not protect later
mutation unless mutation APIs also preserve the invariant.

## 7. Boundary considerations

Boundary adapters should decode physical structure and delegate to the smart
constructor. Keep raw DTOs distinct when parsing and policy validation have
different error mappings. Apply size limits before allocating or normalizing
untrusted input. Do not expose internal validation diagnostics that leak
secrets or security distinctions beyond policy.

## 8. Persistence considerations

Every database reader calls the same constructor or an equivalent complete
validation function. Store normalized or original forms according to explicit
requirements; if both matter, represent both. Schema constraints reinforce
stable invariants. Policy changes require migration or a version-aware
constructor rather than silent reinterpretation.

## 9. Testing evidence

Create a boundary table: below, at, and above bounds; empty; malformed; valid;
normalization collision; and representative Unicode. Assert structured errors.
Property-test normalization idempotence and accepted-value round trips.
Integration-test Serde and database paths. Search for all construction sites and
compile-test private representation when consequential.

## 10. Costs

Fallible construction propagates errors and creates conversion code.
Normalization can allocate. A large constructor that checks unrelated external
policy becomes difficult to reuse and test. Too many synonymous constructors
can form bypasses or unclear evidence levels. Public error enums add API
compatibility obligations.

## 11. When not to use it

Do not hide fallible behavior behind an infallible-looking function. Do not put
database or network calls into a scalar constructor. Do not use one constructor
to produce a type named for stronger verification than it performs. Plain
construction is sufficient for unconstrained internal data.

## 12. Related doctrines

RUST-DOC-0001 defines complete protected construction. RUST-DOC-0002 governs
constructor errors. RUST-DOC-0005 governs persistence conversions.
RUST-DOC-0008 governs negative and property evidence.

## 13. Executable example

See [`../examples/validated-newtypes/src/lib.rs`](../examples/validated-newtypes/src/lib.rs)
and [`../examples/boundary-validation/src/lib.rs`](../examples/boundary-validation/src/lib.rs).

## 14. Worked application

An email syntax constructor can trim surrounding transport whitespace if policy
allows, cap total length, require one nonempty local and domain part, reject
control characters, and retain the normalized string. Its name should remain
`EmailAddress` or `SyntacticallyValidEmail`, not `VerifiedEmailAddress`.
Ownership verification consumes separate verifier evidence and constructs the
stronger type through a restricted path.

Constructor errors might distinguish empty input, excessive length, missing
separator, invalid local part, and invalid domain. An HTTP adapter can map these
to a public validation response while preserving the internal category. A
database import uses the identical parser and quarantines failures. No caller
reimplements `contains('@')`.

## 15. Review prompts

- Does function naming expose fallibility?
- Is validation complete or followed by undocumented caller checks?
- Is normalization performed before every dependent predicate?
- Can two inputs normalize to one identity, and is collision policy explicit?
- Are errors structured for callers without leaking secrets?
- Does `TryFrom` delegate to one canonical validation path?
- Are external facts kept out of pure constructors?
- Can any derived decoder or convenience conversion bypass construction?
