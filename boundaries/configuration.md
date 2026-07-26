# Configuration boundary guide

## 1. What is untrusted?

Environment variables, files, command-line arguments, mounted secrets, remote
configuration, service discovery, and default selection are untrusted
representations. Deployment control reduces hostile input likelihood but does
not prevent typo, unit mismatch, partial rollout, stale secret, invalid
combination, excessive value, or accidental disclosure.

Configuration from several sources also has an ordering and provenance problem:
operators must know which source won.

## 2. What parsing occurs?

Collect raw strings/bytes into a source-aware raw configuration. Parse explicit
booleans, integers, addresses, paths, enums, durations, byte sizes, lists, and
version. Units belong in names or syntax: `30s` is safer than an unexplained
`30`; bytes and item counts are distinct. Bound file size, line count, list
length, and remote response size before allocation.

Reject invalid encoding deliberately. Do not apply lossy parsing to identifiers
or secrets.

## 3. What validation occurs?

Validate per-field ranges and formats, then cross-field combinations: TLS
certificate with key, min not greater than max, retry budget within request
deadline, queue capacity compatible with workers, mutually exclusive modes,
required secret for enabled integration, and nonzero resource bounds.

Startup validation should complete before accepting work. Checks requiring
network reachability are readiness observations, not permanent configuration
validity.

## 4. How is a trusted type constructed?

Use a `RawConfig` that preserves optional sources, then a fallible conversion to
an immutable or carefully reloadable `ValidatedConfig` composed of typed
durations, sizes, endpoints, paths, and secret wrappers. Defaults are applied in
one documented layer before final cross-field validation. Report provenance for
the effective non-secret values.

Subcomponents receive narrow configuration values rather than the entire
configuration object, limiting accidental authority and secret access.

## 5. How can construction be bypassed?

Bypasses include reading environment variables ad hoc inside business code,
calling `unwrap_or` with hidden defaults, exposing mutable public configuration
fields, deserializing directly to trusted types, letting test defaults ship to
production, building URLs by string concatenation, and reloading individual
fields without validating the complete new snapshot.

Library code should accept typed configuration from its owner instead of
silently reading process-global state.

## 6. How is failure represented?

Distinguish missing required value, malformed syntax, unsupported version,
out-of-range, inconsistent combination, inaccessible file, permission failure,
secret-provider failure, and reload rejection. Aggregate independent startup
errors where safe so operators can fix one deployment, but redact secret
values. Exit nonzero before admission when required configuration is invalid.

Reload failure normally preserves the previous valid snapshot and exposes a
metric/event; it must not install a partial mix.

## 7. How are unknown or future values handled?

Version durable configuration files and remote schemas where long-lived.
Unknown fields may detect misspelling through strict rejection or support
rolling deployment through deliberate ignore; choose per configuration
contract. Unknown enum values should reject unless an explicit disabled or
opaque behavior is safe. Deprecations need warnings without secret values and a
defined removal version.

Defaults can change behavior across releases, so compatibility-sensitive
defaults require versioning or explicit operator selection.

## 8. How is sensitive data protected?

Secret-bearing types avoid `Debug`, `Display`, cloning, serialization, and
metrics labels. Logs show source and presence, never value. Environment
variables can leak through process inspection and crash reports; use an
appropriate secret provider for the threat model. Files need ownership and
permission checks. Remote secret retrieval needs authenticated transport,
rotation, cache, expiry, and failure policy.

Zeroization claims must account for parser strings, process environment, and
provider SDK copies.

## 9. How is evidence tested?

Test each source, precedence, explicit/default values, unit parsing, exact
bounds, unknown fields, invalid combinations, redaction, missing secret,
unreadable file, and remote failure. Snapshot the effective non-sensitive
configuration schema, not live secret values. Integration-test the real startup
path and confirm no work is accepted after invalid configuration.

Reload tests cover atomic snapshot replacement, concurrent readers, rejected
updates, removal, secret rotation, and shutdown.

## 10. What remains uncertain?

Successful parsing does not prove endpoints are reachable, credentials remain
valid, certificates remain unrevoked, files stay unchanged, or selected
capacities suit live load. Reloaded authorization configuration can become stale
immediately. OS resource limits and topology may differ from declared values.
Readiness checks are observations at a time.

## Reload contract

| Phase | Required behavior |
|---|---|
| acquire | bounded read with source/version |
| parse | raw snapshot, no mutation of active state |
| validate | all per-field and cross-field rules |
| prepare | create dependent resources without exposing partial state |
| swap | atomic publication of complete valid snapshot |
| retire | bounded cleanup of old resources |
| fail | retain prior snapshot and report redacted diagnostics |

## Review prompts

- Is every unit explicit and parsed once?
- Are defaults documented as policy rather than hidden in call sites?
- Does startup finish complete validation before admission?
- Does reload validate one complete snapshot and retain the previous one on
  failure?
- Can any subcomponent read environment or files behind the configuration owner?
- Are secrets absent from debug, error, metric, and effective-config output?
- Which reachability or capacity assumptions require ongoing observation?
- Are precedence and provenance visible to operators without revealing values?
