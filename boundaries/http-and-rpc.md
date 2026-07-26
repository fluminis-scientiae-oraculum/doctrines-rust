# HTTP and RPC boundary guide

## 1. What is untrusted?

Method, path, query, headers, body, peer metadata, forwarded identity, cookies,
tokens, deadlines, and idempotency keys are untrusted until their responsible
layer validates them. TLS termination or authenticated transport does not make
the payload domain-valid. Proxy-added headers require a configured trusted-proxy
boundary. Client-supplied correlation IDs and resource names can be malformed,
oversized, or misleading.

## 2. What parsing occurs?

Enforce method, route, header count/size, body bytes, decompression ratio,
content type, nesting, field count, and deadline limits before or during decode.
Parse into request DTOs whose shape follows the protocol, not directly into
trusted aggregates. Parse typed durations and sizes with explicit units. Reject
ambiguous duplicate security headers according to the server framework's
documented behavior.

Protocol parsing establishes structural representation only.

## 3. What validation occurs?

Validate DTO field formats, ranges, cross-field combinations, version,
idempotency-key syntax, and resource identifiers. Authenticate the credential
to obtain a principal. Authorize that principal for the specific action and
resource, producing a scoped capability or explicit decision. Validate
cross-entity business rules transactionally where needed.

Authentication establishes identity evidence, not universal authorization. Under
[RUST-DOC-0003-R004](../doctrines/0003-ownership-and-capabilities/doctrine.md#rust-doc-0003-r004--restrict-capability-issuance-and-surface),
an authorization decision or capability grants only its documented action and resource scope.
Frontend state or client-provided role data is never authority.

## 4. How is a trusted type constructed?

Adapters convert request DTO fields through smart constructors and then call a
domain command/service. The domain layer receives trusted values and
authenticated/authorized evidence rather than framework extractors.

For mutation, create a stable operation ID and bind the idempotency key to
principal, endpoint, target, and request fingerprint before execution. Response
DTOs are constructed deliberately from output evidence; broad domain
serialization can expose internal or secret fields.

## 5. How can construction be bypassed?

Common bypasses include deriving request decoding directly on trusted newtypes,
accepting user-supplied principal structures, checking authorization only in UI
or middleware paths that some routes skip, calling domain methods from an
alternate endpoint without capability, using unchecked `From<String>`, and
letting internal administrative routes share public constructors without audit.

Generated RPC bindings still require a domain conversion layer. Internal
network placement is not a constructor.

## 6. How is failure represented?

Preserve parse, validation, authentication, authorization, not-found, conflict,
rate limit, timeout, cancellation, confirmed rejection, unknown outcome, and
internal failure when clients act differently. Map to HTTP/RPC codes
consistently without leaking secrets or existence information contrary to
policy. Include machine-actionable error codes and safe correlation identity.

A timeout after possible dispatch becomes an explicit unknown operation state,
not an automatic rejection.

## 7. How are unknown or future values handled?

Version public contracts deliberately. Additive fields may be ignored or
retained according to protocol; unknown enum values need reject or explicit
unknown behavior. Support rolling clients with compatibility tests. Deprecation
needs observable usage and a removal policy. Preserve unknown provider outcomes
separately from unknown schema values.

Idempotency response replay must define behavior when server response schemas
evolve during key retention.

## 8. How is sensitive data protected?

Never log authorization headers, cookies, API keys, private request bodies, or
secret response fields. Limit validation excerpts and tracing attributes.
Authenticate correlation and forwarded-client metadata only from trusted
infrastructure. Response errors do not expose stack traces or internal source
messages. Apply cache-control and browser security policy according to endpoint
sensitivity.

Credential parsing minimizes copies; zeroization claims must account for
framework buffers.

## 9. How is evidence tested?

Test method/content type, malformed body, size/decompression limits, missing and
duplicate fields, unknown versions, constructor rejection, authentication
failure, resource-scoped authorization, sensitive error mapping, and rate
limits. Contract tests protect wire schemas and codes. Integration tests use the
real router, middleware order, codec, and identity verifier.

For mutable effects, test concurrent same-key requests, payload conflict,
response replay, timeout after dispatch, safe retry, reconciliation, and key
expiry.

## 10. What remains uncertain?

An authenticated request proves only the verifier's evidence at that time.
Authorization can become stale. A successful server response proves the server
boundary's stated result, not necessarily downstream settlement or durable
client receipt. Client disconnect does not prove cancellation. Proxies and
retries can duplicate requests. External observations can change immediately
after return.

## Decision table

| Concern           | Boundary mechanism                       |
| ----------------- | ---------------------------------------- |
| request shape     | bounded DTO decode                       |
| value invariant   | smart constructor                        |
| identity          | credential verification                  |
| authority         | action/resource authorization capability |
| concurrent update | version/conflict protocol                |
| repeat mutation   | scoped idempotency contract              |
| ambiguous effect  | operation ID plus unknown/reconciliation |
| public failure    | stable code and redacted message         |

## Review prompts

- Are route, middleware, and generated-RPC alternate paths all protected?
- Do request limits apply before decompression and allocation?
- Is authentication evidence distinct from action/resource authorization?
- Does one logical retry reuse operation and idempotency identity?
- Can client cancellation occur after external dispatch?
- Are correlation and error diagnostics useful without exposing credentials or
  internal topology?
- Does version evolution preserve old-client behavior intentionally?
- What response evidence can later become stale or be reversed?
