# Domain modeling: currency-tagged money and invoice states

[`src/lib.rs`](src/lib.rs) makes two representation choices executable: a money
value that cannot be zero, negative, or currency-ambiguous, and an invoice whose
states are a sum type rather than a struct of optional fields.

## What it establishes

`PositiveMoney` carries nonzero minor units and exactly one `Currency`. Its
constructor is the only way in, so no code path holds a zero or unsigned-wrapped
amount. `checked_add` refuses two different currencies and refuses overflow,
returning a named `AdditionError` variant for each rather than a silent
saturation. `ReceiptId` rejects the empty string. `InvoiceState` gives each
stage its own data, so a paid invoice has no nullable payment reference and a
draft has no way to expose one.

## What it does not establish

The type carries no tax, foreign exchange, allocation, or rounding policy — a
currency tag proves two amounts agree on units, not that either is correct.
Minor-unit scale is assumed uniform across currencies, which is untrue for
several real ones. Nothing here concerns persistence or serialization; that
boundary belongs to [`boundary-validation`](../boundary-validation/README.md).

## Evidence

Four unit tests cover the rejection of a zero amount, same-currency addition,
the cross-currency refusal, and the invoice state that cannot exist without its
receipt.

```text
cargo test --locked -p domain-modeling
```

## Doctrine

Cited by [RUST-DOC-0001](../../doctrines/0001-invalid-states/README.md), and by
the [sum types](../../patterns/sum-types.md) and
[opaque newtypes](../../patterns/opaque-newtypes.md) patterns.
