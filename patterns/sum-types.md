# Sum types

## 1. Problem

A record uses booleans and optional fields to describe mutually exclusive
states. Many combinations have no domain meaning: `paid = true` with no receipt,
`failed = true` while `paid = true`, or a failure reason attached to a pending
invoice. Every operation must rediscover which combinations are legal.

## 2. Forces

The state must be inspected at runtime, stored, serialized, logged, matched by
heterogeneous consumers, and evolved over releases. Each state carries different
data. Exhaustive handling is valuable, but public API compatibility and unknown
future persisted values matter. Independent dimensions should not be forced
into one enormous cross-product.

## 3. Weak representation

```rust
struct Invoice {
    paid: bool,
    failed: bool,
    receipt: Option<String>,
    failure: Option<String>,
}
```

The representation admits contradictory values and permits code to forget the
relationship among fields. A validator can reject combinations, but nothing
prevents later mutation from breaking them.

## 4. Improved representation

```rust
enum InvoiceState {
    Pending,
    Paid { receipt: ReceiptId },
    Failed { reason: FailureReason },
}
```

Associated data appears only where meaningful. Transitions replace the complete
state or occur through methods that enforce legal edges. Use separate enums for
independent dimensions rather than multiplying variants.

## 5. Exact guarantee gained

A value constructed through safe Rust is exactly one declared variant, and its
associated fields have the types required by that variant. Matching can be
exhaustive within the current crate/API contract. The representation rules out
the particular contradictory field combinations removed by the design.

## 6. Guarantees not gained

The enum does not prove the transition history was authorized, that associated
IDs exist, that a persisted row is current, or that external reality agrees.
Public construction may still permit an unauthorized `Paid` value. Variant data
can contain weaker invariants. Exhaustive matching today does not make a durable
protocol closed to future values.

## 7. Boundary considerations

Decode external discriminators into a raw representation first when unknown
values or invalid associated fields are possible. Validate each payload through
its constructors. Decide deliberately whether unknown fields/variants are
rejected, retained, or mapped to an explicit `Unknown` form. Authentication and
authorization occur separately from structural decoding.

## 8. Persistence considerations

Choose stable tags independent of incidental Rust spelling. Document rename,
addition, downgrade, and unknown-value policy. Database columns may use a tag
plus associated values, a JSON envelope, or normalized state tables; each needs
a constraint or fallible conversion that rejects invalid combinations. A
runtime enum is generally more suitable than typestate for heterogeneous stored
states.

## 9. Testing evidence

Test construction and behavior for every variant, legal transition edges, and
rejection of invalid raw combinations. Compile exhaustiveness helps when adding
variants inside a controlled codebase. Boundary fixtures must include unknown
tags, missing associated data, surplus fields, old versions, and invalid nested
newtypes. Migration tests preserve stable tags.

## 10. Costs

Variant additions can break exhaustive downstream matches. Large enums can
couple unrelated lifecycle dimensions. Serialization shape becomes a protocol.
Code may need adapters for storage and UI state. Associated payloads can
increase enum size, though boxing without measurement may create worse cost.

## 11. When not to use it

Do not use one enum for independent facts that can combine legitimately. Do not
generate a variant for every permutation when a product of smaller validated
types is clearer. Do not use a closed enum for open third-party identifiers
unless unknown values remain representable. Plain booleans remain appropriate
for genuinely independent binary properties.

## 12. Related doctrines

RUST-DOC-0001 requires sum types for mutually exclusive state.
RUST-DOC-0002 governs errors represented as enums. RUST-DOC-0005 governs stable
persistence and unknown values. RUST-DOC-0006 governs outcome enums whose
unknown state reflects distributed ambiguity.

## 13. Executable example

See [`../examples/domain-modeling/src/lib.rs`](../examples/domain-modeling/src/lib.rs)
for `InvoiceState`, and the invoice and payment case studies for boundary and
transition qualifications.

## 14. Worked application

Consider a support ticket with `Open`, `WaitingForCustomer { requested_at }`,
and `Resolved { resolution, resolved_at }`. A sum type prevents a resolved
ticket from carrying an active waiting request and makes the timestamp
requirements visible. Priority and confidentiality remain separate fields
because they can combine with every lifecycle state. Resolution authority
remains a transition service concern; making the variant public would let any
caller fabricate it.

When this ticket is stored, a raw row might contain `status`, `requested_at`,
`resolution`, and `resolved_at`. Fallible conversion matches the truth table and
constructs the enum. An unknown status from a newer writer is retained or
rejected according to compatibility policy. That conversion, not SQL text
decoding alone, establishes the associated-data invariant.

## 15. Review prompts

- Are the represented conditions truly mutually exclusive?
- Does each variant carry only data meaningful in that state?
- Are independent dimensions kept independent?
- Can public construction forge a transition that requires authority?
- Is external tagging stable across rename and rolling deployment?
- What happens when a newer variant reaches an older reader?
- Does every transition preserve required history and structured failure?
- Does the guarantee ledger distinguish enum shape from external truth?
