# Glossary

**Authority** — Permission to cause a domain effect; distinct from memory access alone.

**Capability** — Protected value whose possession grants bounded operations. The term is
overloaded across this corpus: `RUST-DOC-0010` uses "capability" for a stage trait, where
possession conveys position in a protocol rather than permission. That glossary states the
distinction from its side; this entry states it from the authority side, so a reader meeting
the word in either package learns it has two senses.

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
