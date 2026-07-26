# Validated collections

## 1. Problem

Ordinary collections admit states the domain excludes: empty recipient lists,
too many batch items, duplicate identifiers, unsorted ranges, mismatched
currencies, or totals beyond policy. Validating once is ineffective if public
mutation can later violate the predicate.

## 2. Forces

The invariant may concern length, uniqueness, ordering, per-element validity, or
a relationship among elements. Construction can come from iterators and
deserialization. Mutations need validation. Consumers want slices and
iteration. Large inputs need bounded allocation and efficient validation.
Persistence and schema representations may not enforce the same property.

## 3. Weak representation

```rust
type Recipients = Vec<EmailAddress>;

fn send_all(recipients: Recipients) {
    assert!(!recipients.is_empty());
}
```

The alias supplies no construction boundary. Checks repeat at use sites. Any
caller can pass empty or oversized data, and later mutation can introduce
duplicates.

## 4. Improved representation

```rust
pub struct NonEmptyRecipients(Vec<EmailAddress>);

impl TryFrom<Vec<EmailAddress>> for NonEmptyRecipients {
    type Error = RecipientSetError;

    fn try_from(items: Vec<EmailAddress>) -> Result<Self, Self::Error> {
        if items.is_empty() { return Err(RecipientSetError::Empty); }
        if items.len() > 100 { return Err(RecipientSetError::TooMany); }
        validate_unique(&items)?;
        Ok(Self(items))
    }
}
```

Expose read-only slice/iterator access and only mutation methods that preserve
the complete invariant.

## 5. Exact guarantee gained

Every safely constructed wrapper satisfies the documented collection predicate
at construction. If all mutation paths are private and checked, the predicate
continues to hold. Algorithms can rely on non-empty, bounded, sorted, or unique
properties without repeating checks.

## 6. Guarantees not gained

The wrapper does not prove cross-entity existence, current permissions, external
capacity, or relationships omitted from its constructor. `NonEmpty` does not
mean maximum size is safe. Uniqueness depends on the chosen equality and
normalization. Sorted order does not prove intervals do not overlap unless
checked separately.

## 7. Boundary considerations

Apply an input element and total length limit before or during collection.
Validate each element through its own boundary constructor, then validate the
aggregate. Stream when an untrusted declared length could allocate excessive
memory. Preserve errors that identify category and safe index without logging
sensitive contents.

## 8. Persistence considerations

Database child rows may require a transaction to validate the aggregate and
prevent concurrent changes. Unique indexes can reinforce uniqueness; ordering
usually needs an explicit key. Loading a subset must not construct a wrapper
named for the complete set. Version large persisted aggregates or validate
under locking/optimistic concurrency.

## 9. Testing evidence

Test empty, minimum, maximum, over-maximum, duplicates, normalization collisions,
sorted/unsorted, and invalid elements. Property-test every public mutation to
ensure the invariant remains. Test `collect`/iterator failure and short-circuit
behavior. Boundary tests include deceptive length declarations and historical
invalid rows.

## 10. Costs

Construction may require allocation, sorting, hashing, or a full scan.
Incremental mutation can be complex. Hiding the underlying vector removes some
standard APIs. Large aggregate validation may need transactional locking.
Overly broad aggregate invariants can make harmless local edits expensive.

## 11. When not to use it

Do not wrap a collection with no consequential aggregate invariant. Do not
claim completeness when pagination returns a subset. Do not enforce a
cross-entity fact using a pure collection wrapper. Plain slices and iterators
are appropriate for transient processing where the caller already owns the
invariant.

## 12. Related doctrines

RUST-DOC-0001 covers invalid empty and bounded states. RUST-DOC-0005 governs
aggregate persistence and concurrent validation. RUST-DOC-0008 recommends
property tests for mutation-preserved invariants. RUST-DOC-0009 governs
validation cost claims.

## 13. Executable example

The bounded-name implementation in
[`../examples/validated-newtypes/src/lib.rs`](../examples/validated-newtypes/src/lib.rs)
demonstrates the scalar analogue. Case studies apply bounded and deduplicated
sets at message and invoice boundaries.

## 14. Worked application

An `Allocation` wrapper can require at least one line, unique account IDs, one
currency, and basis-point shares totaling exactly 10,000. Construction validates
elements first, then aggregate uniqueness and total using checked arithmetic.
Read-only iteration is safe. `push` is absent because arbitrary insertion could
break uniqueness and totals; a domain method replaces the complete allocation
after revalidation.

Persistence may store one row per line. Loading through pagination cannot issue
`Allocation`, because a page is not the complete set. The repository reads the
versioned aggregate in a transaction or snapshot, validates all rows, and only
then constructs the wrapper.

## 15. Review prompts

- Is the invariant about each element, the aggregate, or both?
- Are every constructor and iterator-collection path fallible?
- Can `DerefMut`, mutable slices, or retained aliases break the predicate?
- Are length and allocation bounded before accepting input?
- Does uniqueness use normalized domain equality?
- Can partial database reads masquerade as a complete collection?
- How are concurrent child-row updates coordinated?
- Would validation at one operation site be simpler than a persistent wrapper?
