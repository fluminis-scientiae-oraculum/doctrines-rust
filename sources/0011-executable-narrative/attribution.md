# RUST-DOC-0011 attribution

## Internal source

- Origin: the same internal working document that supplied RUST-DOC-0010, specifically its
  argument about where an architectural obligation should live and its position on manually
  maintained decision records
- Medium: unpublished internal Markdown
- Use: problem framing, the executable-authority claim, the last-resort stance on decision
  records, and the argument that each maintained representation raises the cost of a future
  improvement

The document is not stored in this repository and is not reproduced in it. The doctrine prose,
review gates, decision framework, pattern guide, registry, and linter checks are independently
written. The package materially extends the source through the five-class authority partition,
the registry and its validation, the retirement of implemented proposals, the external-authority
naming obligation, and the requirement that an absent rationale be recorded as unknown rather
than inferred.

One attribution obligation applies to this package specifically. The earlier absorption recorded
a claim the source did not make and rejected it. `sources/0011-executable-narrative/source-notes.md`
records what the source actually argued, what this repository accepts, and where it declines to
follow, and the erroneous earlier record is corrected in place rather than deleted, so the
correction is auditable.

## External sources

The architecture decision record is attributed to Michael Nygard, who introduced the form, and to
the later collected work that extended it. Citation identifies the practice this doctrine
restricts; it does not imply that the practice's authors endorse the restriction, and it does not
incorporate the text of those works.

Rust language claims cite the Rust Reference and the Rust Book, maintained by the Rust project
under their published terms. `rustdoc` and its doc-test behavior are cited as an existence proof
for derived documentation that fails the build when it diverges. PostgreSQL claims cite the
PostgreSQL documentation for base types, domains, constraints, and explicit casts. JSON Schema
Draft 2020-12 is cited for registry validation.

Tool behavior is cited with a version: Rust 1.97.1 pinned and 1.85.0 minimum, checked
2026-08-04.

## Research limit

This package records the sources actually used. It does not claim exhaustive coverage of
architecture-documentation practice, decision-record variants, knowledge-management research, or
the empirical literature on documentation decay and technical debt. The absence of a source is
not a judgment about it.

No quotation long enough to require separate license analysis is reproduced. No external media,
transcript, or specification text is mirrored. Repository licensing applies to the original
doctrine prose and examples here, and makes no claim over the cited works.
