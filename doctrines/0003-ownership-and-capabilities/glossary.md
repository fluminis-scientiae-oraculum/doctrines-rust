# Glossary

**Authority** — Permission to cause a domain effect; distinct from memory access alone.

**Capability** — Protected value whose possession grants bounded operations.

**Custody** — Responsibility for a resource's use, transfer, completion, and release.

**Fencing token** — Monotonic or otherwise ordered lease evidence used by the resource owner
to reject stale actors.

**Interior mutability** — Mutation through a shared reference using runtime borrowing,
synchronization, or atomics.

**Lease** — Time- or version-bounded authority that may expire independently.

**RAII** — Resource acquisition bound to object lifetime, with local cleanup triggered by
destruction.

**Revocation** — Withdrawal of authority before or independent of local value destruction.

**Scoped exposure** — Deliberate temporary access to secret material or authority with bounded
recipient and lifetime.

**Single-use token** — Non-forgeable authority intended to be consumed by one operation.
