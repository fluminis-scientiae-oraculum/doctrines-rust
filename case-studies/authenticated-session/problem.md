# Authenticated session: problem

## Domain

A service receives a credential, verifies it through local cryptography or an
external identity provider, constructs an authenticated principal, and
authorizes specific resource actions. Sessions expire and can be revoked.
Identity-provider availability, key rotation, clock skew, cached policy, and
concurrent account changes create uncertainty and staleness.

The evidence sequence is:

```text
credential bytes
    → parsed credential
    → authenticated principal
    → action/resource authorization
    → scoped capability
    → fallible operation
```

Each arrow adds evidence. None should be skipped or named as a stronger stage.

## Invariants

| ID      | Statement                                                                            | Classification     |
| ------- | ------------------------------------------------------------------------------------ | ------------------ |
| AUTH-01 | Untrusted credential representation never directly becomes principal.                | boundary           |
| AUTH-02 | Authentication evidence binds issuer, subject, audience, method, and time.           | authority/evidence |
| AUTH-03 | Authorization is action-, resource-, tenant-, and policy-scoped.                     | authority          |
| AUTH-04 | A session has explicit issue, expiry, rotation, and revocation semantics.            | lifecycle          |
| AUTH-05 | Bearer secrets are minimized in logs, cloning, formatting, and storage.              | security/authority |
| AUTH-06 | Stale authorization is rechecked according to consequence and policy.                | temporal           |
| AUTH-07 | External IdP timeout does not authenticate or definitively reject by assumption.     | distributed        |
| AUTH-08 | Capability possession does not claim external policy remains current.                | guarantee honesty  |
| AUTH-09 | Session and capability construction are restricted.                                  | construction       |
| AUTH-10 | Audit records who authenticated and which policy authorized without storing secrets. | evidence           |

## Boundaries

HTTP cookies, authorization headers, API keys, client certificates, forwarded
identity, and UI state are untrusted. Only configured trusted proxies may
assert forwarded identity. Credential parsing enforces length/encoding before
cryptographic work. Authentication verifies source and claims. Authorization
loads current account/resource policy or a versioned snapshot.

An external IdP is another distributed system. A timeout can mean no response;
it does not produce principal evidence. Previously cached keys or introspection
results may permit bounded operation under explicit freshness policy.

## Required behavior

The design must prevent principal/capability forgery through safe public APIs,
define session transfer and revocation, separate authentication errors from
authorization outcomes where useful, protect secrets, and test expiry,
revocation, stale policy, alternate routes, concurrent requests, and IdP
failure. Frontend typestate may guide UI but cannot secure the backend.
