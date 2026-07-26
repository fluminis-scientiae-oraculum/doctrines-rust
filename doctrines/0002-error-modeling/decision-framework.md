# Decision framework

## Choose the contract boundary

Ask who receives the error and what decisions remain:

| Boundary                  | Preferred form                                                | Reason                                      |
| ------------------------- | ------------------------------------------------------------- | ------------------------------------------- |
| Reusable library          | structured domain enum or stable opaque typed error           | callers need action and compatibility       |
| Internal domain service   | structured enum                                               | preserves business outcomes                 |
| Application orchestration | typed errors until final decision, then opaque report         | combines action with rich context           |
| CLI/process entry         | formatted report and exit code after classification           | no upstream Rust caller                     |
| HTTP/RPC                  | internal typed error mapped to stable public status/code/body | separates protocol recipient from internals |
| Background job            | typed retry/reconcile decision plus correlated report         | scheduler action must be safe               |

Use `thiserror` or equivalent when derivation reduces mechanical implementations without
changing the model. Use an `anyhow`-style report where control decisions are already complete
and arbitrary context is more valuable than public matching. Do not expose an application
report as a library's only error if callers need categories.

## Classify outcomes

For each failure, answer:

1. Was input invalid?
2. Did policy or authority reject it?
3. Did current state conflict?
4. Was the operation cancelled, and at what commitment point?
5. Did a timeout occur before or after possible external execution?
6. Is local state reusable, consumed, or partially changed?
7. Can the same request be retried safely?
8. Must state be reloaded or reconciled first?
9. Which recipient may see details?
10. Which stable code or variant should callers use?

## Retry guidance

Return or compute a typed decision:

```text
DoNotRetry
RetrySameOperation { after, remaining_budget }
RetryAfterRefresh
ReconcileBeforeRetry { operation_id }
Escalate
```

The error alone need not own policy, but the decision must use its structured evidence.
Backoff, jitter, attempt budget, overall deadline, idempotency retention, and nested retry
layers are part of the design.

## Fatal errors and panic

Use an ordinary fatal process error when startup or service continuation is unsafe but the
program can produce a controlled diagnostic and exit. Panic only when a programmer violated
an internal invariant and the chosen process policy treats that as unrecoverable or supervised.

Before panic, ask whether the input came from HTTP, configuration, storage, clock, filesystem,
network, user, or another process. If yes, expected external failure should normally be
returned.

## Coarsening

Errors may be coarsened outward when the recipient cannot act on internal categories.
Preserve:

- security-safe public wording;
- stable public code;
- internal category and source;
- correlation;
- reconciliation identity;
- retry headers or guidance where valid.

Never coarsen confirmed rejection and unknown effect into one public response if the client
must choose different behavior.

## Stop conditions

Stop adding variants when distinctions expose unstable implementation without changing action.
Stop coarsening when a caller must parse strings, retry becomes unsafe, security evidence is
lost, or an unknown outcome disappears. Review the resulting compatibility and redaction
surface.
