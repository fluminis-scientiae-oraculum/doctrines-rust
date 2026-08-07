# Rationale

Errors describe the states in which an operation did not produce its ordinary success value.
That makes them part of domain and protocol design. `Result<T, String>` can carry prose, but
it cannot reliably tell a caller whether to ask the user for correction, retry with the same
idempotency key, reconcile before retry, refresh authorization, or stop permanently.

Structured enums preserve distinctions:

```rust
enum CaptureError {
    Validation(ValidationError),
    Rejected(ProviderRejection),
    Conflict { current_version: u64 },
    LocalTransport(TransportError),
    Unknown { reconciliation: ReconciliationToken },
}
```

The exact shape depends on the API. The important property is that an ambiguous timeout is
not a `Rejected` variant, and an authorization denial is not hidden as transport failure.

Source chains and context serve different readers. A domain category supports control flow.
An underlying I/O or database error supports diagnosis. Context says which operation failed.
An outer report can format that chain for an operator after decisions are complete. Erasing
the category to add a sentence forces later code to parse human text, while exposing every
dependency error directly freezes implementation detail into public API.

`thiserror`-style derives can implement typed errors with sources; the doctrine does not
require that crate. `anyhow`-style opaque reports are useful at an application boundary where
the process will report or terminate and no reusable caller needs stable variants. They are a
poor primary library contract when callers need action.

Retry is semantic. A connection refusal before sending a request may be retriable. A timeout
after sending a non-idempotent capture may require reconciliation. A conflict may be retriable
only after reloading state. A validation error is generally repaired, not retried unchanged.
A provider rejection may carry its own retry window. Generic "transient" labels are evidence
only when defined by the operation's protocol.

Panics express a different contract: safe continuation through the current call stack is not
expected. They fit impossible internal states caused by programmer error, not malformed JSON,
missing files, provider timeouts, or database conflicts. Even an internal invariant panic
needs consideration of unwind versus abort, locks, FFI, and process supervision.

`expect` can document a proof close to code — for example, a regex literal compiled once when
the literal is fixed and known valid — but it should not replace a fallible path for
configuration or user input. "Cannot fail" requires an invariant, not optimism.

Errors also have recipients. A user needs safe corrective information. An operator needs
correlation and category. Telemetry needs bounded fields. A security audit may need protected
detail. Returning raw database or provider messages can disclose tokens, schema, queries,
personal data, or internal topology. Redaction should retain a correlation key rather than
destroy diagnostic evidence.

Public error variants can become semver commitments. A library should expose stable domain
categories, use `#[non_exhaustive]` or an opaque strategy where evolution requires it, and
avoid making every dependency error a top-level variant. Stable codes can support protocols,
but codes must map to documented semantics and must not become strings with unknown meaning.

Logging at each `?` boundary produces several records for one failure, often at different
severity. The handling owner should decide final log level, response, metric, and retry.
Lower layers can attach structured context or trace spans without claiming the event was
unhandled.

Error types do not prove recovery. A transaction error may have consumed a guard. An async
operation may have partially mutated local state. A remote effect may be unknown. Each variant
must state post-error state and safe next actions. This keeps errors as honest evidence rather
than a bucket for everything undesirable.

## Boundary translation

One domain failure can have several recipient-specific representations without losing its
identity. A validation variant may become an HTTP client error with field codes, a CLI
diagnostic with usage help, and a job result that is permanently rejected. The internal
category remains validation. The mapping should be exhaustive and tested so a newly added
variant cannot silently become a generic server error.

Authentication and authorization deserve particular care. Public policy may intentionally
coarsen "resource absent" and "resource forbidden" to avoid disclosure. Internally, the audit
record still needs to distinguish missing resource, invalid credential, denied capability,
and policy failure. Coarsening for one recipient is not permission to discard evidence
globally.

Provider codes can be valuable but are not automatically domain contracts. A boundary adapter
should map documented stable provider categories into domain meaning and retain the raw code
for protected diagnosis. Unknown codes need an explicit safe fallback. Matching the provider's
human message couples behavior to wording and translation.

## Cancellation and partial work

Async cancellation is a control-flow event with state consequences. Dropping a future can
occur at any `await`; local mutation, reserved capacity, locks, durable intent, or transmitted
requests may already exist. A cancellation variant is honest only when the operation defines
what cleanup completed and whether external work may continue. Otherwise cancellation can
produce the same unknown outcome as a timeout.

Consuming APIs should consider returning the original value with a pre-commit error. After a
possible commitment, returning the original authority as if unused can enable repetition.
The error shape may instead carry a reconciliation handle. The type design and error design
therefore form one lifecycle contract.

## Operational stability

Error display text serves humans and may improve without a semver change; variants, stable
codes, retry hints, and source behavior may be relied upon by programs. Documentation should
say which layer is stable. A non-exhaustive public enum lets a library add categories, but
callers still need a safe catch-all action. A stable opaque type can expose methods such as
`kind()`, `is_retryable_under(&operation)`, or `code()` without exposing dependency layout.

Metrics should count categories at the handling boundary and separate attempts from logical
operations. Three retries are one user operation with multiple attempts, not necessarily
three incidents. Unknown effects should remain visible until reconciled; deleting the initial
error after later success loses latency and reliability evidence.

The design should also define equality and cloning deliberately. Many errors contain sources
that are not comparable or cloneable. Forcing `Clone` merely to satisfy a queue may erase the
source into text. A durable job record should store a stable failure category, safe fields,
attempt metadata, and correlation — not pretend to serialize an arbitrary in-memory error
object.
