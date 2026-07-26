# Capability types

## 1. Problem

An operation requires authority, but APIs accept ambient identifiers, booleans,
or a broadly privileged service handle. Any caller that can name a resource can
attempt the operation. Authorization is repeatedly checked, easy to omit, and
hard to trace through helper layers.

## 2. Forces

Authority may be scoped to action, resource, tenant, time, or workflow. It must
be hard to forge and easy to pass to authorized code. Cloning, transfer,
revocation, expiry, serialization, logging, and task movement affect meaning.
External policy can change after issuance. Some operations require one-time or
exclusive authority.

## 3. Weak representation

```rust
fn capture(payment_id: PaymentId, authorized: bool) -> Result<Receipt, Error> {
    if !authorized { /* reject */ }
    // ...
}
```

The flag is forgeable, carries no scope or provenance, and can separate from the
authorization decision. A generic client with all methods grants excess
authority to every caller.

## 4. Improved representation

```rust
pub struct CapturePermit {
    payment: PaymentId,
    grant: GrantId,
    expires_at: Instant,
}

impl PaymentAuthorizer {
    pub fn authorize_capture(
        &self,
        principal: &AuthenticatedPrincipal,
        payment: PaymentId,
    ) -> Result<CapturePermit, AuthorizationError> {
        // Check policy, then construct private fields.
    }
}
```

The capture API requires possession of `CapturePermit`. Constructors remain in
the authorization component. Methods expose only permitted operations.

## 5. Exact guarantee gained

Safe callers outside privileged construction cannot forge the capability's
private representation. An API requiring the capability cannot be called
accidentally with only an identifier. Non-`Clone` ownership can express
single-custody transfer, and consuming methods can prevent local reuse.

## 6. Guarantees not gained

Possession does not prove external policy remains current, a session has not
been revoked, clocks agree, or the downstream effect will succeed. A cloneable
capability is not exclusive. A serialized token can be copied unless the
receiver enforces use and revocation. Rust privacy does not constrain privileged
database or raw network access outside the process.

## 7. Boundary considerations

Authentication establishes principal evidence; authorization constructs a
scoped capability. Do not deserialize an authority-bearing Rust type directly
from an untrusted request. Verify signatures, audience, issuer, expiry, replay,
and revocation before construction. Redact secrets and avoid derived formatting
that exposes bearer material. Map authorization failure without leaking
sensitive policy.

## 8. Persistence considerations

Persist an authorization grant or token record only when recovery needs it.
Store stable identity, scope, issuance evidence, expiry, version, and revocation
state; minimize secrets. Rehydrate through the authorizer rather than raw row
decoding. Revocation requires a runtime observation or an effect resource that
checks current grant state.

## 9. Testing evidence

Compile-fail test private construction and disallowed methods. Unit-test scope,
resource mismatch, expiry boundaries, consuming use, and non-clone behavior.
Integration-test authentication-to-authorization conversion and revoked/stale
grants. Audit every privileged constructor and serialization path. Concurrency
tests cover duplicate use when one-time semantics matter.

## 10. Costs

Capabilities add types, issuance services, dependency threading, and explicit
revocation checks. Fine-grained types can multiply. Non-clone authority can be
ergonomically harder across async tasks. Bearer capabilities can increase
security risk if logged or leaked. Distributed enforcement needs storage or a
remote authority, not only a Rust wrapper.

## 11. When not to use it

Do not create capability types for ordinary data access with no authority
distinction. Do not wrap a forgeable boolean and call it authority. Do not use a
long-lived capability when policy must be checked on every action. A normal
authorization service call may be clearer for dynamic, cross-entity decisions.

## 12. Related doctrines

RUST-DOC-0003 defines ownership as authority and lifecycle. RUST-DOC-0001
requires protected construction and honest evidence names. RUST-DOC-0004 covers
transfer across tasks. RUST-DOC-0006 covers leases, stale authority, and
fencing.

## 13. Executable example

The authenticated-session case study demonstrates principal-to-capability
conversion. Payment types in
[`../examples/compile-fail/src/lib.rs`](../examples/compile-fail/src/lib.rs)
demonstrate capture authority in a local protocol.

## 14. Worked application

A shutdown coordinator can issue one `ShutdownPermit` to the component allowed
to stop admission and join workers. The permit is non-`Clone`, carries the
service generation, and is consumed by `begin_shutdown`. This prevents ordinary
request code from invoking the transition and makes transfer across a
supervisor task explicit. It does not guarantee every task cooperates or every
external resource closes.

For revocable payment authority, the capability can carry a grant identity
rather than a bearer secret. The capture boundary checks current grant status
and payment version before acting. The Rust value improves local least
privilege; the runtime check handles revocation and stale policy.

## 15. Review prompts

- Who alone can construct the capability?
- Which action, resource, tenant, and lifetime does it cover?
- Is cloning semantically safe?
- Can serialization turn it into an uncontrolled bearer value?
- How are transfer, consumption, expiry, and revocation enforced?
- Can logs or debug output leak authority?
- Does downstream enforcement check the capability's identity?
- Which external policy can change after issuance?
