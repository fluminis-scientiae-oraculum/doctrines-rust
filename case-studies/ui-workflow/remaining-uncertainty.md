# UI workflow: remaining uncertainty

## Browser state is advisory

Users, extensions, developer tools, compromised scripts, and stale cached code
can alter client state. Private fields or typestate in a Rust/Wasm frontend help
developers but cannot establish backend authorization. Every server boundary
revalidates values, session, action, resource, version, and idempotency.

Client validation and server validation can differ during rolling deployment.
Responses need stable error categories and versions. The client may show a
generic fallback for a newer category without accepting invalid state.

## Network and external execution

A browser timeout or tab close does not cancel server work already admitted.
Even explicit HTTP cancellation may only stop local processing or the
connection. Durable operation identity and status are needed for consequential
commands. If status storage is lost or a downstream effect remains unknown, the
UI cannot manufacture terminal certainty.

The user can open another device that lacks the local operation record.
Server-side operation history scoped to principal/resource provides recovery.
Retention expiry creates a policy decision for old unknowns.

## Draft durability and secrecy

Browser storage can be cleared, evicted, corrupted, synchronized unexpectedly,
or read by malicious same-origin code. Encryption keys available to the same
compromised origin may not protect against script execution. Preserve only the
data needed for usability and state the retention/clearing policy.

Draft restoration can use stale resource versions or policy. Restored values
return to `Draft` and must revalidate; they do not recover old authorization.
Sensitive financial or identity values may require no browser persistence at
all.

## Multiple actors and versions

Optimistic concurrency prevents silent overwrite but cannot automatically merge
all domain changes. The UI must show conflicts and let the user refetch, compare,
or abandon. Retrying after conflict may require a new authorization decision and
possibly a new operation intent.

Multiple tabs can race before coordination messages arrive. Server idempotency
and version checks remain authoritative. Client coordination improves
experience, not correctness on its own.

## Routing and deployment

Hash routing avoids server fallback requirements, but CDN caches, base paths,
service workers, content-security policy, and asset versioning can still break
deep links. History routing can work safely with precise fallbacks. Neither
choice protects secrets embedded in URLs, and fragments remain visible to
client-side code.

## Accessibility and user interpretation

Screen readers, translation, reconnect, and delayed status updates affect how
evidence is communicated. "Submitted" and "confirmation pending" must remain
semantically distinct across localization. A user may intentionally choose a
new submission despite duplicate risk; the interface records that explicit
decision.

## Final statement

The improved workflow removes contradictory local states, preserves user input,
binds one intent to one identity, and represents network ambiguity honestly.
Backend authorization, browser integrity, storage durability, concurrent
reality, hosting configuration, and permanently missing external evidence remain
runtime concerns.
