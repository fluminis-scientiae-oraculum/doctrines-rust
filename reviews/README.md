# Operational reviews

Review procedures turn doctrine into repeatable decision gates. They are not
surveys and do not replace reading the applicable doctrine packages. Each item
must be recorded as:

- **pass** — cited evidence satisfies the question;
- **fail** — evidence is missing or contradicts the requirement;
- **not applicable** — scope explanation shows the risk is absent;
- **waiver reference** — approved waiver names scope, owner, consequence,
  compensating control, expiry, and removal condition.

Blank status is not approval. A critical doctrine violation cannot be converted
into soundness or external certainty through a waiver; a waiver records accepted
residual risk only where governance permits it.

## Procedures

| Procedure                                                   | Use                                                    |
| ----------------------------------------------------------- | ------------------------------------------------------ |
| [Pre-implementation](pre-implementation.md)                 | before representation and API commitments              |
| [Domain model review](domain-model-review.md)               | values, states, construction, transition, authority    |
| [Boundary review](boundary-review.md)                       | DTO, Serde, database, protocol, size, version, secrecy |
| [Typestate review](typestate-review.md)                     | proportional use of type-level sequencing              |
| [Distributed-effects review](distributed-effects-review.md) | timeout, retry, duplicate, reconciliation, ordering    |
| [Final correctness audit](final-correctness-audit.md)       | release/merge guarantee ledger and aggregate gates     |

## Evidence rule

Reviewers cite concrete artifacts: invariant IDs, state graphs, source paths,
constructor visibility, queries, protocol documents, tests, fault matrices,
measurements, and generated-bundle checks. "Idiomatic Rust," compilation, or a
green suite alone is not enough. Evidence must match the claim and identify its
limits.

## Severity and disposition

Use **critical** when failure can forge trusted evidence, bypass authority,
cause undefined behavior, repeat a consequential effect, lose durable state, or
make an externally false guarantee. Use **high** for likely correctness,
recovery, compatibility, or operational failures. Use **medium** for material
maintainability, diagnostic, or evidence weakness. Every fail has remediation
or a governance decision.

The final audit references completed focused reviews rather than copying their
answers. Generated agent packs may embed selected gates, but these canonical
files remain authoritative.
