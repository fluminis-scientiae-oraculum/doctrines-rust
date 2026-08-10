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

| Procedure                                                     | Use                                                          |
| ------------------------------------------------------------- | ------------------------------------------------------------ |
| [Pre-implementation](pre-implementation.md)                   | before representation and API commitments                    |
| [Domain model review](domain-model-review.md)                 | values, states, construction, transition, authority          |
| [Boundary review](boundary-review.md)                         | DTO, Serde, database, protocol, size, version, secrecy       |
| [Typestate review](typestate-review.md)                       | proportional use of type-level sequencing                    |
| [Distributed-effects review](distributed-effects-review.md)   | timeout, retry, duplicate, reconciliation, ordering          |
| [Executable narrative review](executable-narrative-review.md) | where an obligation lives, and whether a record is justified |
| [Final correctness audit](final-correctness-audit.md)         | release/merge guarantee ledger and aggregate gates           |

## Evidence rule

Reviewers cite concrete artifacts: invariant IDs, state graphs, source paths,
constructor visibility, queries, protocol documents, tests, fault matrices,
measurements, and generated-bundle checks. "Idiomatic Rust," compilation, or a
green suite alone is not enough. Evidence must match the claim and identify its
limits.

## Procedure format policy

Compact tables are the default for dense operational checklists because they
make gate, evidence, failure, severity, and remediation fields easy to scan.
Expanded gate sections remain valid when a procedure needs a fuller argument
per question. The two forms carry the same disposition and traceability
requirements.

Every review artifact in the corpus now uses the table form: all eleven package
review standards, and all seven procedures listed above. The expanded form has no
current instance.

That is a change, not the original design. RUST-DOC-0001's review standard was
written in the expanded form and stayed there while the corpus grew; it became a
table when every gate acquired the `Check` column that declares whether it is
judgment or a named mechanical command, because that column has to be readable
across gates rather than buried in one section per gate. This paragraph went on
describing the old shape, and named RUST-DOC-0002 through RUST-DOC-0009 as the
packages using tables — an enumeration that had already been stale since
RUST-DOC-0010 and RUST-DOC-0011 were added. It instructed readers to preserve an
exception that no longer existed, which is worse than saying nothing: it invited
someone to restore a form the enforcement column cannot carry.

The expanded form remains valid where a procedure genuinely needs a fuller
argument per question. Reintroducing it means keeping the `Check` declaration
legible per gate, and saying what review value the expansion supplies.

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
