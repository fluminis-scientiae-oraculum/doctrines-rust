# Accepted RFCs

This directory contains decisions approved for implementation or already
implemented. Entry requires a completed decision record naming date, owners,
rationale, conditions, affected doctrine IDs, and migration/evidence
expectations.

- [RFC-0001: Make isolation anomalies and time assumptions
  enforceable](RFC-0001-isolation-and-time-assumptions.md)
- [RFC-0002: Add a doctrine for staged protocols and successor
  capabilities](RFC-0002-staged-protocols-and-successor-capabilities.md)
- [RFC-0003: Partition architectural authority and make decision records a last
  resort](RFC-0003-executable-narrative-and-authority-partition.md)

This index is maintained by hand beside a directory that already holds the
answer, and it was wrong within one release: RFC-0002 was accepted and not
listed here. It is a live instance of what `RUST-DOC-0011-R005` prefers to
generate, and it is left informative and owned rather than generated, because a
generator fed by a hand-written list would be the same duplication under another
name.

After acceptance, substantive changes to the decided normative contract require
a new RFC. Factual annotations, implementation links, and status notes may be
added without rewriting the decision. Under `RUST-DOC-0011-R011` an accepted RFC
becomes decision history once implemented: cite canonical doctrine and the
executable artifacts for current behavior, and cite the RFC for its decision,
date, owners, conditions, and rejected alternatives. Implementation tracks every condition and
updates doctrine version, manifests, schemas, source notes, examples, role
packs, generated outputs, and CHANGELOG as applicable.

Exit condition is supersession by a later accepted RFC. Move the old file to
`../superseded/` only when the replacement is accepted and the relationship is
recorded in both decision records. Rejection is not a valid later state for an
already accepted historical decision.

Relationship to doctrine status: acceptance authorizes change but does not make
canonical doctrine active. The implementation PR and validated manifest perform
that transition.
