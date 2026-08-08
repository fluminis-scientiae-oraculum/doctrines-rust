# References

References identify where a mechanical fact or an established practice comes from. They do not
transfer authority to this doctrine's obligations, which are repository governance.

## Practices this doctrine restricts

The architecture decision record was introduced by Michael Nygard as a lightweight, numbered file
capturing the context, decision, and consequences of one architecturally significant choice, and
was later collected and extended by Joel Parker Henderson and others. The form is cited here for
what it is, not as an endorsement of the restriction this doctrine places on it. Nygard's original
proposal was a reaction to unread architecture documents and already argued for small, dated,
single-decision files; the additional obligations here, that a record must name the fact no
artifact can carry, must carry an owner and an end condition, and must not be cited without
confirming applicability, are this repository's.

Documentation-generation practice, in which a human-readable view is produced from the artifact
it describes rather than maintained beside it, is long established across tooling ecosystems.
Rust's own `rustdoc` and the doc-test mechanism are the instance nearest to hand: the
documentation is derived from the item it documents, and the examples in it are compiled and run,
so a divergence fails the build rather than accumulating. That mechanism is cited as an existence
proof for `RUST-DOC-0011-R005`, not as a claim that every derived view can be produced so cheaply.

## Mechanisms an obligation can move into

Rust language mechanics for visibility, module privacy, traits and their bounds, associated
items, and move semantics are cited from the Rust Reference and the Rust Book, maintained by the
Rust project under their published terms, for the pinned toolchain 1.97.1 and the minimum
supported version 1.85.0, checked 2026-08-04. These supply the construction restrictions,
capability boundaries, and ordering constraints that `RUST-DOC-0011-R002` prefers over prose.

PostgreSQL documentation is cited for base types, domains, constraints, and explicit casts, which
carry a nominal distinction between identifier species into the schema so that comparing two of
them requires a stated conversion. The claim borrowed here is only the mechanical one; persistence
obligations are governed by RUST-DOC-0005.

JSON Schema Draft 2020-12 is cited for the machine-readable validation of the decision-record
registry, consistent with the doctrine and agent-pack manifests already validated in this
repository.

## Related repository material

[`foundations/guarantee-honesty.md`](../../foundations/guarantee-honesty.md) supplies the discipline that separates a
claim from its limits, which is what `RUST-DOC-0011-R003` relies on when it requires the unenforced part of a claim to
be stated separately. [`foundations/evidence.md`](../../foundations/evidence.md) supplies the evidence classes the
decision framework selects between. [`foundations/complexity-budget.md`](../../foundations/complexity-budget.md)
supplies the assessment `RUST-DOC-0011-R002` requires before an obligation is left prose-carried.

RUST-DOC-0010 applies this doctrine's partition to staged protocols in `RUST-DOC-0010-R022`, and
its `RUST-DOC-0010-R018` and `RUST-DOC-0010-R019` are worked instances of an obligation moved into
an executable artifact because prose could not detect its violation.

## Research limit

This package records the sources actually used. It does not claim exhaustive coverage of
architecture-documentation literature, knowledge-management research, technical-debt studies, or
the empirical work on documentation decay. The absence of a source is not a judgment about it.

No quotation long enough to require separate license analysis is reproduced. No external media,
transcript, or specification text is mirrored. Repository licensing applies to the original
doctrine prose here and makes no claim over the cited works.
