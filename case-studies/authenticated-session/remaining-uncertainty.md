# Authenticated session: remaining uncertainty

## Identity evidence changes

Authentication proves that accepted credential evidence matched configured
rules at an observation time. It does not prove the human remains in control,
the credential was not stolen, the issuer remains uncompromised, or the account
will not be disabled immediately. Key compromise can retroactively reduce trust
in prior evidence.

External identity events and revocation feeds can be delayed, duplicated, or
lost. Polling creates a freshness window. Critical operations may require recent
reverification, but that reduces availability during provider outage.

## Session revocation

A server-side revocation record helps only if every operation checks it or uses
a bounded cache. Already issued capabilities, queued tasks, and downstream
requests may continue. Revocation semantics must say whether it stops admission,
cancels local work, fences resource changes, or merely prevents future permit
issuance.

Clock skew, process pause, and long-lived connections complicate expiry. An
`Instant` is process-local and cannot be persisted directly as universal time;
persisted timestamps require wall-clock policy and conversion.

## Authorization staleness

A capability represents one authorization decision. Resource ownership,
membership, risk score, legal hold, and policy can change. Binding to resource
version and short expiry reduces stale use; rechecking at the destructive
boundary provides stronger current evidence with more latency and failure.

A Rust non-`Clone` type cannot revoke copies already serialized or permissions
accepted by another service. Downstream enforcement needs operation identity,
current policy, or fencing.

## Availability choices

IdP outage forces a product decision among fail closed, bounded cached evidence,
or degraded access. No universal doctrine chooses the business risk. Cached
authentication must state age, issuer-key status, revocation blind spot, and
allowed actions. A network timeout does not show whether an introspection
request executed, though authentication normally depends on the received
response rather than remote side effect.

## Secret handling

Redacted wrappers prevent ordinary formatting, not memory disclosure, privileged
debuggers, swap, crash dumps, framework logging, foreign libraries, or copied
buffers. Token hashing is not always sufficient for low-entropy secrets and may
still create a replay verifier. Storage and transport controls remain.

## Cross-service propagation

A downstream service receiving only a principal ID cannot reconstruct the
authentication method, audience, session status, or upstream authorization.
Forwarding the original bearer credential broadens secret exposure and may give
the receiver more authority than needed. Prefer a narrowly scoped,
audience-bound service credential or signed delegation whose issuer, action,
resource, expiry, and correlation are verified by the receiver.

Delegation still cannot freeze upstream revocation. A queue can deliver an
authorized command after the initiating session expires. Product policy must
decide whether authority is captured at durable admission or rechecked at
execution. The audit trail records both times and policy versions. Correlation
shows causality but does not prove every hop logged honestly; protected audit
storage and cross-service contract tests provide supporting evidence.

## Final statement

The improved model makes evidence and authority transitions explicit and blocks
ordinary forgery. It cannot freeze identity-provider integrity, account control,
revocation propagation, clocks, or mutable policy. Those facts remain
time-scoped runtime observations with consequence-driven revalidation.

> [!TIP]
> [problem](problem.md) · [naive design](naive.md) · [improved design](improved.md) · **remaining uncertainty**
> Index: [all case studies](../README.md).
> Executable mechanics live under [`../../examples/`](../../examples/).
