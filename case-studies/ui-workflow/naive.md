# UI workflow: naive design

## State

```rust
struct FormState {
    valid: bool,
    submitting: bool,
    submitted: bool,
    error: Option<String>,
    fields: HashMap<String, String>,
}
```

Booleans permit `submitting && submitted`, valid with stale edited fields, and
submitted with an error. Any handler can set them independently. One error
string combines local syntax, server validation, authorization, conflict,
timeout, and unknown result.

## Validation and authority

The frontend checks required fields and hides the submit button from users
without an `admin` flag embedded in page data. The server handler trusts the
same `valid` and `admin` fields posted by the browser. A user can modify the
request or call the endpoint directly. Frontend types and hidden controls become
a fictional security boundary.

## Submission

Clicking submit sets `submitted = true` immediately, clears the form, and starts
an HTTP request. The button remains active long enough for a double click. Each
request generates a new random idempotency key. A timeout sets
`error = "failed"` and enables retry. If the server executed, the retry creates
a duplicate.

If server validation rejects, cleared input is lost. Users reconstruct it and
may introduce different data under a new key. A conflict response loses the
server version and offers only generic retry, causing repeated conflict.

## Navigation and storage

Draft fields, bearer token, and server error are copied into a URL query so a
refresh can restore them. Browser history, referrers, screenshots, analytics,
and server logs expose the values. Another attempt uses local storage without
tenant/user scoping or expiry, so a later user sees a prior draft.

The SPA uses history routing on a static host without fallback. Direct refresh
of `/payments/new` returns 404. A quick change to hash routing stores the token
after `#`, confusing obscurity with security; browser scripts and extensions can
still read it.

## Concurrency and evidence

Two tabs edit the same entity version. Both submit; last write wins because the
server accepts blind replacement. Client state in one tab does not see the
other. The UI calls an HTTP 202 response "completed" even though it only means
accepted for processing.

Tests click once with an immediate-success mock. They assert button text and a
snapshot. No delayed response, lost response, double click, reload, stale
version, denied server authorization, or route refresh is tested. Snapshot
approval can bless contradictory UI states without checking semantics.

> [!TIP]
> [problem](problem.md) · **naive design** · [improved design](improved.md) · [remaining uncertainty](remaining-uncertainty.md)
> Index: [all case studies](../README.md).
> Executable mechanics live under [`../../examples/`](../../examples/).
