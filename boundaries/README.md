# Trust-boundary guides

Boundary guides operationalize the shared pipeline:

```text
untrusted representation
    ↓ parse
structural representation
    ↓ validate
trusted domain representation
    ↓ execute
external side effect
    ↓ observe / reconcile
new trusted evidence or explicit uncertainty
```

Validation is relocated and centralized; it is not eliminated. A wire decoder
can establish valid JSON structure while domain construction rejects a zero
amount. A database driver can establish a valid SQL integer while a money type
rejects the value or currency combination. A successful local filesystem write
does not by itself prove durable storage after power loss. Every guide separates
these evidence levels.

## Guides

| Boundary                                  | Primary concerns                                                       |
| ----------------------------------------- | ---------------------------------------------------------------------- |
| [Serde](serde.md)                         | checked deserialization, versioning, allocation limits                 |
| [Database decoding](database-decoding.md) | raw rows, domain conversion, migration, concurrency                    |
| [HTTP and RPC](http-and-rpc.md)           | DTOs, authentication/authorization, idempotency, error mapping         |
| [Messaging](messaging.md)                 | duplicates, ordering, acknowledgement, replay, schema evolution        |
| [Configuration](configuration.md)         | startup validation, secrets, defaults, reload                          |
| [Filesystem](filesystem.md)               | path trust, symlinks, TOCTOU, atomic replacement, durability           |
| [FFI](ffi.md)                             | ABI, representation, ownership, unwind, allocator and error boundaries |

## Required boundary record

For each implementation record:

1. untrusted bytes or values;
2. parser and physical limits;
3. structural DTO/row/foreign type;
4. domain validations and constructors;
5. alternate and privileged bypass paths;
6. structured failure mapping;
7. unknown/version evolution policy;
8. sensitive-data handling;
9. positive, negative, and integration evidence;
10. external or temporal facts the boundary cannot establish.

## Layering rule

Do not expose raw transport, row, or foreign representations as trusted domain
types merely to reduce conversion code. Do not duplicate domain policy in every
adapter; adapters call the canonical constructor and map its structured errors.
Conversely, keep protocol-only concerns such as body size, field presence,
schema version, and ABI layout at the boundary.

Authentication and authorization are separate evidence transitions.
Deserializing a principal identifier does not authenticate it. Authentication
does not grant every operation. An authorized capability remains time- and
scope-limited when policy can change.

## Review use

Apply [`../reviews/boundary-review.md`](../reviews/boundary-review.md) and the
relevant doctrine package. Trace all generated derives, ORM mappings, raw SQL,
administrative imports, cache paths, feature-gated implementations, and test
helpers. A single unchecked construction path weakens the type's claim
repository-wide.
