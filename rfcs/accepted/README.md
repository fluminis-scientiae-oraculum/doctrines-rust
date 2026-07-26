# Accepted RFCs

This directory contains decisions approved for implementation or already
implemented. Entry requires a completed decision record naming date, owners,
rationale, conditions, affected doctrine IDs, and migration/evidence
expectations.

After acceptance, substantive changes to the decided normative contract require
a new RFC. Factual annotations, implementation links, and status notes may be
added without rewriting the decision. Implementation tracks every condition and
updates doctrine version, manifests, schemas, source notes, examples, role
packs, generated outputs, and CHANGELOG as applicable.

Exit condition is supersession by a later accepted RFC. Move the old file to
`../superseded/` only when the replacement is accepted and the relationship is
recorded in both decision records. Rejection is not a valid later state for an
already accepted historical decision.

Relationship to doctrine status: acceptance authorizes change but does not make
canonical doctrine active. The implementation PR and validated manifest perform
that transition.
