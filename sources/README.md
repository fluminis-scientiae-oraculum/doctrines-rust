# Source provenance

Source notes explain intellectual and technical provenance. They are informative,
not normative. Normative obligations live in doctrine packages with stable rule
IDs. A source can explain language or protocol mechanics without determining
repository policy, severity, waiver, or review structure.

Each doctrine has:

- `source-notes.md` — what was consulted, what the source establishes, and how
  the doctrine accepts, refines, rejects, or extends ideas;
- `attribution.md` — citation, licensing/copyright posture, quotation policy,
  and limitations of the research performed.

## Classification

**Accepted** means the doctrine adopts the useful core within a stated scope.
**Refined** means a broad or pedagogical claim is narrowed to evidence actually
established. **Rejected** means the repository intentionally does not adopt the
claim. **Added** means repository governance supplies a concern not derived from
the cited source.

The classification prevents borrowed authority from becoming a stronger claim
than the source or implementation supports.

## Source quality

For Rust language claims prefer the Rust Reference, standard-library
documentation, Rust RFCs, official project books, and compiler/tool
documentation. For runtime-specific facts use the runtime's official
documentation and state the version scope. For databases and protocols use the
vendor/specification source. For distributed systems use primary specifications
and foundational literature where practical.

Third-party explanatory material may orient or teach but is not the sole
authority for language guarantees. These packages record a careful selection,
not exhaustive academic research.

## Copyright and external media

Use short quotations only where wording itself matters; otherwise summarize and
link. Do not mirror videos, transcripts, books, articles, specifications, or
tool documentation. External media remains at its source. Citation does not
change repository licensing of original doctrine prose, nor does repository
licensing claim ownership of cited works.

Changing facts — tool versions, stable releases, action versions, MSRV, and
product behavior — must be rechecked when maintained. The note should identify
the relevant version/date when the fact affects a normative or executable
choice.
