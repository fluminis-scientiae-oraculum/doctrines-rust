# Accepted RFCs

This directory contains decisions approved for implementation or already
implemented. Entry requires a completed decision record naming date, owners,
rationale, conditions, affected doctrine IDs, and migration/evidence
expectations.

After acceptance, substantive changes to the decided normative contract require
a new RFC. Factual annotations, implementation links, and status notes may be
added without rewriting the decision. Under `RUST-DOC-0011-R011` an accepted RFC
becomes decision history once implemented: cite canonical doctrine and the
executable artifacts for current behavior, and cite the RFC for its decision,
date, owners, conditions, and rejected alternatives. Implementation tracks every
condition and updates doctrine version, manifests, schemas, source notes,
examples, role packs, generated outputs, and CHANGELOG as applicable.

Exit condition is supersession by a later accepted RFC. Move the old file to
[`../superseded/`](../superseded/) only when the replacement is accepted and the relationship is
recorded in both decision records. Rejection is not a valid later state for an
already accepted historical decision.

Relationship to doctrine status: acceptance authorizes change but does not make
canonical doctrine active. The implementation PR and validated manifest perform
that transition.

The index below is generated from this directory and from each RFC's own front
matter, so it cannot drift from the files it lists. It was previously maintained
by hand and was wrong within one release: RFC-0002 was accepted and never listed.
That is the drift `RUST-DOC-0011-R005` prefers to remove rather than assign, and
this file is the prose half of the generated view rather than a second copy of
the index.
