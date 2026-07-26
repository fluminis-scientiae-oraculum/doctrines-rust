# Payment lifecycle: naive design

## Representation

```rust
struct Payment {
    status: String,
    validated: bool,
    authorized: bool,
    captured: bool,
    settled: bool,
    reversed: bool,
    amount: u64,
    currency: String,
    provider_id: Option<String>,
    last_error: Option<String>,
}
```

Any string becomes a status. Booleans can contradict it and one another.
`amount = 0` is representable. A provider ID does not reveal whether it belongs
to authorization, capture, or settlement. `last_error` erases structured
rejection, timeout, conflict, unknown outcome, and source error.

Methods mutate `&mut Payment`:

```rust
fn capture(payment: &mut Payment) -> Result<(), String> {
    if !payment.authorized {
        return Err("not authorized".to_owned());
    }
    provider_capture(payment)?;
    payment.captured = true;
    Ok(())
}
```

Callers can set `authorized = true` directly or load it from an unchecked row.
Two workers can both observe it, call the provider, and later overwrite each
other. The runtime check does not carry authorization identity, amount, expiry,
or provider evidence.

## Retry failure

The provider client retries every timeout three times and generates a fresh
request key on each attempt. An HTTP proxy also retries. A capture may execute,
lose its response, and execute again. The application records each timeout as
`status = "failed"`, allowing the user to press capture again.

The worker updates the database to `captured` and then publishes an event.
Process loss between steps forgets the event. Reversing the order can publish
capture before a database rollback. Calling the provider inside the transaction
only holds locks longer; it does not include provider state in database
atomicity.

## Settlement and reversal errors

A settlement webhook writes `settled = true` without verifying source,
matching amount/currency, event identity, or current capture. Reordered events
can settle a locally failed payment. Duplicate webhooks append duplicate audit
entries and trigger repeated notifications.

Reversal sets `reversed = true` before calling the provider and calls the
operation rollback. If the provider rejects or times out, local status lies.
The design has nowhere to represent reversal unknown or partial compensation.

## Evidence weakness

Tests call methods in the intended order with a synchronous always-successful
mock. No compile-fail evidence exists because every method is callable. No
concurrent worker, timeout-after-execution, duplicate webhook, stale version,
or process-loss point is tested. Coverage reaches the branches but not the
protocol.

The design overclaims:

- `authorized` means capture authority remains current;
- timeout means provider rejected;
- database transaction includes provider call;
- reversal undoes capture;
- string status matches provider reality.

Support cannot reconstruct one logical operation across retries, and users can
cause duplicates while attempting to recover.
