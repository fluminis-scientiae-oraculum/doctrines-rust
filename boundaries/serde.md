# Serde boundary guide

## 1. What is untrusted?

Every byte sequence or data model supplied by a request, message, file,
environment-derived document, cache, or old persisted blob is untrusted. A
successful Serde decode establishes compatibility with the selected Serde data
model and deserializer behavior, not current domain validity. Declared lengths,
nested structures, map keys, duplicate fields, enum tags, and textual encodings
can be hostile or historically incompatible.

## 2. What parsing occurs?

The format parser establishes syntax and physical Rust representations:
integers, strings, sequences, maps, and a raw version envelope. Enforce
transport and decompression limits before Serde where possible. Configure
recursion, input length, collection length, and numeric behavior through the
format implementation. A format parser must not allocate based solely on an
untrusted length without a reviewed bound.

Parsing and normalization are distinct. Preserve raw text long enough to apply
the intended Unicode, case, whitespace, and canonicalization policy.

## 3. What validation occurs?

Validate required combinations, numeric bounds, identifiers, collection
invariants, stable version, and cross-field state after structural decoding.
Use a raw DTO when multiple fields jointly establish a domain state. Scalar
newtypes use their canonical parser or `TryFrom`.

```rust
#[derive(Deserialize)]
#[serde(try_from = "RawEmail")]
pub struct EmailAddress(String);
```

The `TryFrom<RawEmail>` implementation must call the complete constructor, not
reproduce a weaker subset.

## 4. How is a trusted type constructed?

Preferred routes are `#[serde(try_from = "...")]`, manual `Deserialize` that
delegates to a checked conversion, or a remote derive applied to a raw adapter.
`into` is suitable for trusted-to-wire serialization; `try_from` is appropriate
when wire-to-domain conversion can fail. Manual visitors are justified for
streaming, allocation control, compatibility, or precise diagnostics, but still
finish at protected domain construction.

Construction errors map into the deserializer's error type while retaining a
stable safe category in application code.

## 5. How can construction be bypassed?

Risk paths include deriving `Deserialize` directly on private-field newtypes,
adding a default that invents evidence, exposing an unchecked `From<String>`,
using `serde_json::Value` followed by field assignment, test-only constructors
enabled in production features, and custom visitors that write representation
without validation. An unsafe layout shortcut is never an acceptable decoder.

Search every `Deserialize`, `from_value`, raw cache decode, and compatibility
adapter. Privacy alone does not protect against code inside the defining module.

## 6. How is failure represented?

Separate syntax failure, unsupported version, structural mismatch, domain
validation, size/resource rejection, and security policy when callers act
differently. Public error responses can combine sensitive distinctions while
internal error chains retain safe context. Never panic on malformed external
data. Do not replace an unknown future variant with an arbitrary current
variant.

Batch decoding should report whether processing is atomic, partial, or
quarantined, with bounded diagnostics.

## 7. How are unknown or future values handled?

Choose deliberately among reject, ignore, retain, and explicit unknown. `deny_unknown_fields`
is a compatibility and security policy, not a universal default: it detects
misspellings and surplus fields but prevents additive forward compatibility.
Ignoring unknown fields can hide client mistakes. Retaining them can support
round-trip proxies but expands memory and sensitive-data exposure.

Version durable envelopes. Test old-reader/new-writer and
new-reader/old-writer combinations. Use stable external enum tags rather than
incidental variant spelling.

## 8. How is sensitive data protected?

Avoid derived `Debug` and broad serialization on secret-bearing domain types.
Use dedicated response DTOs so internal secrets cannot be emitted accidentally.
Limit error excerpts; never log complete tokens, credentials, private keys, or
regulated payloads. Unknown fields may themselves be sensitive and require the
same retention policy as known fields. Zeroization claims remain limited by
copies made by parsers and allocators.

## 9. How is evidence tested?

Test valid fixtures, malformed syntax, missing and duplicate fields, unknown
fields/variants, old versions, bounds, excessive nesting, large declared
lengths, normalization collisions, and invalid nested newtypes. Property tests
can exercise round-trip and parser non-panic properties with bounded generators.
Fuzz the raw decoder when input exposure and parser risk justify it.

Compile-time privacy evidence must be paired with integration tests proving
Serde uses the checked constructor. Review snapshot changes semantically.

## 10. What remains uncertain?

Deserialization does not prove authentication, authorization, referenced entity
existence, mailbox ownership, remote state, current policy, or later external
success. A version accepted today can become obsolete. An authenticated message
can still carry invalid business intent. Record these non-guarantees in the
boundary ledger and establish stronger evidence through the appropriate domain
service.

## Decision table

| Situation | Preferred approach |
|---|---|
| scalar with local invariant | raw scalar plus `try_from` |
| cross-field state | raw DTO then domain conversion |
| untrusted large sequence | streaming visitor with explicit cap |
| long-lived format | versioned envelope and compatibility fixtures |
| proxy retaining unknown data | bounded extension map with sensitivity policy |
| secret-bearing output | dedicated response DTO, no broad domain serialization |

## Review prompts

- Does every trusted target use a fallible checked conversion?
- Which byte, nesting, sequence, and decompression limits apply before allocation?
- Is unknown-field policy compatible with both security and rolling deployment?
- Can defaults or aliases create evidence not present in the input?
- Do serialization and debug implementations expose sensitive domain fields?
- Which external facts remain outside deserialization evidence?
