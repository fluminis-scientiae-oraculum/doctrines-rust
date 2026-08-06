# Authenticated session: naive design

## Representation

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
struct User {
    id: String,
    authenticated: bool,
    admin: bool,
    token: String,
}
```

The request body can deserialize `authenticated = true` and `admin = true`.
Fields are public inside broad modules. `Clone`, `Debug`, and serialization copy
and expose the bearer token. User ID has no issuer or tenant scope. A boolean
admin flag cannot state resource, action, policy version, expiry, or revocation.

## Middleware shortcut

One HTTP middleware checks that the authorization header contains a dot and
decodes token claims without verifying signature, audience, or expiry. It
inserts `User` into request extensions. Some administrative and WebSocket routes
skip the middleware and accept user ID from query parameters.

Handlers use:

```rust
if user.authenticated && user.admin {
    delete_account(account_id).await?;
}
```

Authentication and authorization are combined. Any admin can delete any
tenant's account. Policy changes after token issue are invisible. The handler
passes the complete user/token object to helpers that need only account-delete
authority.

## Session behavior

Sessions never rotate. Logout deletes a browser cookie but does not revoke the
server token. Expiry is checked only at initial login, so a long-lived
connection keeps authority forever. A session cached in memory is treated as
current after account suspension.

When the external IdP times out, code falls back to decoded claims "for
availability." When introspection returns not found during IdP lag, it rejects
permanently. The two choices invent authentication and rejection without
defined evidence.

## Error and audit weakness

All failures become `Unauthorized`. This hides malformed input, bad signature,
expired session, revoked session, IdP unavailable, and insufficient resource
permission from internal callers. Conversely, response messages sometimes
expose whether an account exists. Audit logs print the full `User` debug output,
including token, and state only "admin operation" without policy or target.

## Evidence weakness

Tests create `User { authenticated: true, admin: true, ... }` directly. They
never execute real middleware or token verification. No compile-fail evidence
protects principal construction. Expiry uses wall-clock sleeps. Revocation,
key rotation, alternate routes, tenant mismatch, IdP timeout, and WebSocket
revalidation are absent.

The design's types carry aspirational names but no protected evidence. Memory
safety does not prevent authority forgery.

> [!TIP]
> [problem](problem.md) · **naive design** · [improved design](improved.md) · [remaining uncertainty](remaining-uncertainty.md)
> Index: [all case studies](../README.md).
> Executable mechanics live under [`../../examples/`](../../examples/).
