# Anti-patterns

## `Result<T, String>`

**Weak example.** A library returns formatted prose for validation, I/O, and conflict.
**Why it fails.** Callers parse unstable wording. **Risk.** Wrong response or retry.
**Improved direction.** Structured domain categories with `Display` for humans.
**Justified appearance.** A tiny private helper may return text consumed immediately by one
formatting layer when no action distinction exists.

## One giant `AppError`

**Weak example.** Every subsystem variant is placed in one global enum and propagated
everywhere. **Why it fails.** It couples unrelated layers and makes any dependency a public
concern. **Risk.** Unstable API and meaningless catch-all handling. **Improved direction.**
Use bounded errors per contract and convert at ownership boundaries. **Justified appearance.**
A process entrypoint can have one final report type after domain decisions.

## Retry every error

**Weak example.** Middleware retries any `Err` three times. **Why it fails.** Validation,
denial, conflict, and unknown effects have different semantics. **Risk.** Duplicate effects
and load amplification. **Improved direction.** Typed retry/reconcile decision with budget and
jitter. **Justified appearance.** A proven idempotent read may use a narrow transport policy.

## Log and discard

**Weak example.** A lower layer logs an error and returns success or `None`. **Why it fails.**
The caller cannot respond and telemetry contradicts state. **Risk.** Silent data loss.
**Improved direction.** Return the error; log at final handling. **Justified appearance.**
Best-effort cleanup may record and continue when failure is explicitly non-critical and
observable.

## Double logging

**Weak example.** Every propagation layer logs the same source. **Why it fails.** One event
looks like many incidents. **Risk.** alert noise and false counts. **Improved direction.**
Attach context during propagation and log once at the handling owner. **Justified appearance.**
Distinct trace events can measure layer timing when correlated and not counted as failures.

## Panic on malformed input

**Weak example.** JSON parsing uses `unwrap`. **Why it fails.** External failure is expected.
**Risk.** denial of service and lost cleanup. **Improved direction.** Return validation or
parse error. **Justified appearance.** Fixed compile-time literals can use a locally proven
expectation.

## Timeout becomes rejection

**Weak example.** Provider timeout maps to `PaymentDeclined`. **Why it fails.** No rejection
evidence exists. **Risk.** duplicate capture and false user message. **Improved direction.**
Unknown outcome with reconciliation. **Justified appearance.** Only a protocol-proven
pre-commit timeout.

## Hide the source

**Weak example.** `map_err(|_| DomainError::Storage)` drops database evidence. **Why it fails.**
Diagnosis loses cause. **Risk.** slow repair and misclassification. **Improved direction.**
Preserve a source or correlated protected diagnostic. **Justified appearance.** Withhold a
sensitive source from an untrusted boundary while retaining it internally.

## Expose internal secrets

**Weak example.** Raw provider body or SQL appears in an HTTP error. **Why it fails.**
Diagnostic detail crosses recipient scope. **Risk.** credential or data disclosure.
**Improved direction.** Stable public code, safe message, protected correlated detail.
**Justified appearance.** Restricted forensic storage under access and retention control.

## String-based retry

**Weak example.** Code retries when `message.contains("temporary")`. **Why it fails.** Wording
and locale change. **Risk.** unsafe or missed retry. **Improved direction.** Structured
category or protocol code. **Justified appearance.** Compatibility parsing of a broken legacy
protocol may be isolated, versioned, and heavily tested.
