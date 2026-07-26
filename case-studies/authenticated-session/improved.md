# Authenticated session: improved design

## Evidence-bearing types

```rust
pub struct CredentialInput(SecretBytes);
pub struct ParsedCredential(ParsedToken);

pub struct AuthenticatedPrincipal {
    subject: SubjectId,
    issuer: IssuerId,
    tenant: TenantId,
    authentication: AuthenticationEvidence,
    session: SessionId,
    expires_at: Instant,
}

pub struct DeleteAccountPermit {
    principal: PrincipalId,
    account: AccountId,
    policy_version: PolicyVersion,
    expires_at: Instant,
}
```

Representations and fields are private. `CredentialInput` enforces size and
redaction but proves no identity. The authenticator alone constructs
`AuthenticatedPrincipal` after cryptographic verification or authoritative
introspection. The authorizer alone constructs `DeleteAccountPermit` after
checking action, resource, tenant, session, account state, and policy.

The permit exposes only account deletion and is non-`Clone` unless parallel use
is an explicit policy. Deletion consumes it or binds it to one operation ID.
Possession is local evidence from one policy observation; the operation may
recheck critical revocation/version before effect.

## Authentication pipeline

The HTTP adapter:

1. limits credential/header size;
2. selects one supported credential scheme;
3. parses structure without logging bytes;
4. identifies configured issuer;
5. obtains a bounded-fresh verification key or introspection response;
6. verifies signature/MAC, issuer, audience, subject, expiry/not-before, nonce
   or replay requirements, and method-specific claims;
7. maps to a principal with authentication evidence and observation time.

Unknown issuer, unsupported algorithm, malformed input, expired credential,
revoked session, and IdP unavailable remain distinct internally. Public mapping
follows information-disclosure policy.

Trusted-proxy identity uses a separate authenticated channel and allowlist; a
public client cannot set the trusted header. WebSocket and long-lived tasks
receive session identity plus revalidation policy, not permanent authority.

## Session lifecycle

The session record contains hashed/opaque session identity, principal,
authentication method, issue/expiry, rotation chain, last required
reauthentication, policy/security version, and revocation state. Bearer material
is stored only where necessary with least privilege. Rotation invalidates or
links predecessor according to policy. Logout performs server-side revocation
and client cookie removal; failure is reported rather than called complete.

Expiry checks use an injectable trusted clock with documented skew. High-risk
actions can require recent authentication regardless of overall session expiry.
Long-lived connections revalidate at bounded intervals or on policy/security
version change.

## Authorization

Authorization accepts an authenticated principal plus action and resource. It
loads current resource/tenant policy and constructs a narrowly scoped
capability. A database transaction or version predicate prevents authorization
against one account version followed by destructive mutation of an unrelated
or changed record.

Handlers do not receive raw role booleans. Role membership may be one input to
policy, but the output is the exact permit or structured denial. Current policy,
not frontend visibility, decides.

## External IdP uncertainty

No IdP response means no new principal evidence. The service may:

- fail closed for critical actions;
- use previously verified session evidence within a documented freshness and
  revocation-risk window;
- degrade read-only behavior;
- return retryable unavailable.

These are product policies, not generic transport inference. Introspection
absence is definitive only under the IdP contract. Key refresh failures preserve
the last verified key only within an explicit rotation/compromise policy.

## Errors, audit, and secrets

Structured internal errors preserve parse, invalid credential, expired,
revoked, unavailable, authorization denial, stale version, and effect failure.
External responses are stable and redacted. Audit records principal ID, issuer,
authentication method, session ID, action, resource, policy version, operation
ID, decision, and correlation—never raw token.

Secret-bearing types implement deliberate redacted debug and avoid broad
serialization/clone. Zeroization claims acknowledge framework buffers, process
environment, foreign libraries, and allocator copies.

## Evidence

Compile-fail tests protect principal and permit construction. Unit/property
tests cover parser limits, claims, exact expiry/skew boundaries, and redaction.
Integration tests use the real router/middleware order, verifier, session store,
and authorization repository. Tests cover alternate routes, trusted-proxy
spoofing, tenant mismatch, revocation after issue, rotation, concurrent permit
use, long-lived revalidation, IdP timeout, key rotation, and account version
conflict.

## Guarantee ledger

| Claim                                         | Established by                              | Protected construction         | Boundary preservation       | Escape hatches          | Does not prove                                     | Residual runtime risk         |
| --------------------------------------------- | ------------------------------------------- | ------------------------------ | --------------------------- | ----------------------- | -------------------------------------------------- | ----------------------------- |
| credential is structurally parsed             | bounded parser                              | private parsed type            | adapter only                | authenticator internals | identity                                           | parser/library defect         |
| principal authenticated under stated evidence | verified signature/introspection and claims | authenticator-only constructor | session row checked on load | audited migration       | continued account control or current authorization | key compromise/revocation lag |
| delete permit was authorized for one account  | current policy decision                     | authorizer-only fields         | account/version bound       | privileged break-glass  | future policy or operation success                 | revocation race               |
| session is not expired at check time          | trusted-clock comparison                    | session service                | rechecked on use            | clock administration    | future validity                                    | skew/pause                    |
| token is not emitted by normal debug          | redacted secret wrapper                     | no derived formatting          | dedicated DTOs              | raw framework access    | absence from all memory/dumps                      | copies/crash dump             |
| IdP result is unavailable, not authenticated  | explicit failure category                   | no principal constructor       | preserved through adapter   | fallback policy         | credential invalid                                 | prolonged outage              |
