# Opaque newtypes

## 1. Problem

Primitive values such as `String` and `u64` carry too little evidence.
Unvalidated email input, a syntactically accepted address, and a
ownership-verified address may all share the same representation. Raw integers
can be mixed across units or admit zero where policy requires a positive value.

## 2. Forces

The invariant is stable and local to one value. Construction must be
centralized without making ordinary reads awkward. Values cross Serde,
database, logging, and API boundaries. Conversion and borrowing should remain
ergonomic. Type names must not imply evidence unavailable to constructors.
Occasional privileged imports may need explicit escape paths.

## 3. Weak representation

```rust
fn send(to: String, cents: u64) { /* ... */ }
```

Every caller can supply empty, malformed, zero, wrong-unit, or unchecked data.
Validation becomes scattered, may drift, and is easy to omit after decoding.

## 4. Improved representation

```rust
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn parse(input: String) -> Result<Self, EmailSyntaxError> {
        validate_syntax(&input)?;
        Ok(Self(input))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

The field is private. Construction runs the complete documented invariant.
Conversions are fallible; accessors expose only operations that cannot violate
the invariant.

## 5. Exact guarantee gained

Every instance obtainable through the public safe API has passed the specified
constructor policy, assuming no privileged escape path violated its obligation.
Different newtypes prevent accidental parameter mixing even when their bytes
match. Private representation centralizes invariant evolution and formatting
policy.

## 6. Guarantees not gained

An `EmailAddress` does not prove ownership, deliverability, or future validity.
A `NonZeroU64` proves nonzero, not correct currency, sufficient funds, or tax
policy. A locally validated identifier does not prove the referenced entity
exists. Newtype names must match the actual evidence level.

## 7. Boundary considerations

Deserialize through `try_from` or a manual implementation that calls the
constructor. Request DTOs and CLI values remain raw until validation. Protect
allocation and length before complex parsing. Debug and display output should
avoid accidental secret exposure. A verifier should produce a separate stronger
type rather than mutate the meaning of the weaker one invisibly.

## 8. Persistence considerations

Decode a physical row value then use `TryFrom`; do not let an ORM assign private
bytes unchecked. Schema constraints can reinforce stable predicates. Historical
invalid values become conversion failures or quarantine records. Persisted
normalization rules need versioning because later policy changes can alter
equality and uniqueness.

## 9. Testing evidence

Test accepted values, rejection partitions, exact bounds, normalization
idempotence, formatting/redaction, conversions, and boundary decoding.
Compile-fail tests can prove direct field construction is unavailable outside
the crate. Property tests can cover round trips and generator-defined valid
sets. Database fixtures must include invalid historical values.

## 10. Costs

Newtypes add conversion and API surface, can produce wrapper proliferation, and
may complicate generic code or serialization. Excessive micro-types obscure
data flow. Changing the invariant can become a migration. Private fields may
require deliberate borrowing and formatting implementations.

## 11. When not to use it

Do not wrap values without a meaningful invariant, unit, identity, secrecy, or
misuse distinction. Do not use a newtype to represent dynamic cross-entity
policy that requires external state. Do not name a value `Verified` when any
public parser can construct it. A type alias or plain primitive is appropriate
when interchangeability is intentional.

## 12. Related doctrines

RUST-DOC-0001 governs trusted construction and evidence-accurate naming.
RUST-DOC-0003 applies to secret and authority-bearing wrappers.
RUST-DOC-0005 governs checked decoding. RUST-DOC-0008 requires negative and
boundary evidence.

## 13. Executable example

See [`../examples/validated-newtypes/src/lib.rs`](../examples/validated-newtypes/src/lib.rs)
and [`../examples/domain-modeling/src/lib.rs`](../examples/domain-modeling/src/lib.rs).

## 14. Worked application

`PositiveMoney` can contain a `NonZeroU64` minor-unit amount and a `Currency`.
This proves a nonzero amount in one named currency. Checked addition first
requires currency equality and then handles integer overflow. The type still
does not choose an FX rate, tax regime, decimal scale migration, allocation
rule, or rounding policy. Those facts belong to services and explicit policies.

For an API request, the DTO may contain an integer and currency code. Parse the
currency, reject zero, then construct the newtype. For a database row, perform
the same checked conversion. Exposing `as_minor_units` is safe because reading
does not invalidate the value; exposing `&mut u64` would destroy the invariant.

## 15. Review prompts

- Is the private field inaccessible from every unprivileged module?
- Does every constructor establish every guarantee named by the type?
- Are parsing, normalization, policy acceptance, and external verification
  distinguished?
- Do Serde and database paths call the checked constructor?
- Can formatting leak sensitive representation?
- Are mutation and conversion paths invariant-preserving?
- Is an escape hatch visible, scoped, and audited?
- Would a type alias communicate the intended distinction just as well?
