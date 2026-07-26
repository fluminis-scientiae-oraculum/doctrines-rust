# Contributing

Contributions improve a normative engineering corpus, executable evidence, or the mechanisms
that keep them synchronized. The standard is semantic accuracy: concise wording is welcome,
but a short change can alter obligations, construction rights, or claimed guarantees.

## Issue first or direct pull request

A direct pull request is appropriate for spelling, broken internal links, source citation
corrections, clearer non-normative explanation, test strengthening that preserves the public
claim, and tooling fixes with unchanged doctrine meaning. Open an issue first when the problem
needs domain discussion, affects several doctrines, introduces a new representation policy,
or reports a guarantee overclaim whose safe resolution is uncertain.

An RFC is mandatory for a new doctrine, a normative rule addition or weakening, a new escape
hatch, doctrine supersession, a change to normative-term meaning, significant distribution
restructuring, license change, or MSRV policy change. Follow `rfcs/README.md`; an issue can
establish motivation, but it does not replace the RFC decision record.

## Doctrine package anatomy

Every doctrine contains eight substantive files:

1. `README.md` with validated metadata, scope, relationships, and reading order;
2. `doctrine.md` with stable rule IDs and normative contracts;
3. `rationale.md` with failure modes, alternatives, costs, and limits;
4. `decision-framework.md` with selection tables, trees, stop conditions, and simpler cases;
5. `review-standard.md` with auditable gates and evidence;
6. `anti-patterns.md` with weak examples, risk, repair, and justified exceptions;
7. `glossary.md` with doctrine-specific terms;
8. `references.md` with authoritative sources.

Use `templates/doctrine/` when proposing a package. Do not copy an existing doctrine and leave
its IDs, cross-links, or assumptions unchanged.

## Normative language

Read `foundations/normative-language.md`. Uppercase MUST, MUST NOT, SHOULD, SHOULD NOT, and MAY
carry their defined normative meanings. Each rule has one stable
`RUST-DOC-####-R###` identifier and records statement, intent, applicability, allowed
exceptions, and expected review evidence.

Rationale is informative unless a rule incorporates it. Examples illustrate a mechanism
unless marked required. A waiver is explicit, scoped, reviewed, owned, risk-assessed, and
linked from the affected work; silence is not a waiver.

## Source provenance

Update the matching `sources/` package when external material influences a rule or its
rationale. Prefer the Rust Reference, standard-library documentation, Rustonomicon, Rust
RFCs, protocol specifications, official tool documentation, and foundational or
peer-reviewed systems literature. Record which idea was accepted, refined, rejected, or
added. Keep quotations short and link the source; do not mirror media or transcripts.

## Executable examples

Example code must compile on the workspace MSRV and pinned stable toolchain, contain meaningful
tests, expose only guarantees described in prose, and demonstrate failure boundaries.
Constructors test both acceptance and rejection. External effects stay fallible even when
state sequencing is encoded. A value's name must match its evidence.

Keep dependencies minimal. A new dependency needs a written reason in the pull request,
license and MSRV review, deliberate feature selection, and `cargo deny check`. Do not broaden
Clippy allows to hide a local warning.

## Compile-fail evidence

Use the `trybuild` suite for important programs that the API intends to reject. Each case
should fail for one clear semantic reason and import the real public API under test. Generate
diagnostics with the pinned stable toolchain, inspect the error location and message, then
commit the corresponding `.stderr`.

A compiler upgrade may legitimately change wording. Never accept new diagnostics blindly.
Confirm that direct construction, wrong-state method use, or reuse after a consuming
transition remains the rejected behavior. If the failure moved to an incidental error, repair
the test.

## Generated content

`dist/` is generated. Edit canonical sources or bundler behavior, then run:

```bash
cargo run -p bundle-agent-context -- generate
cargo run -p bundle-agent-context -- check
```

Review source-path headings and scope. Role packs should select doctrine relevant to their
workflow rather than reproduce the corpus indiscriminately.

## Commits and review

Use a small number of intention-revealing commits. A typical substantial change separates
canonical content, executable evidence, and validation or generated output when that improves
review. Avoid one-file-per-commit histories and unrelated cleanup.

A reviewer checks:

- affected doctrine and rule IDs;
- RFC requirement and decision state;
- normative versus editorial classification;
- guarantee ledger and non-guarantees;
- construction and boundary bypasses;
- source attribution;
- MSRV, dependency, and license compatibility;
- positive, negative, compile-fail, and integration evidence as applicable;
- manifests, schemas, and generated outputs;
- all local validation results.

At least one reviewer should understand the affected risk domain. Unsafe changes require
RUST-DOC-0007 review. Distributed-effect changes require explicit timeout, retry,
idempotency, duplicate, and reconciliation review.

## Validation

Run the complete command set in the root README and record exact successful commands in the
pull request. CI uses read-only permissions and confirms those checks. Do not push work whose
first formatter, compiler, test, or linter run will be CI.

## Reporting a guarantee overclaim

Use the guarantee-overclaim issue form. Identify the exact prose claim or type name, the
implementation that establishes evidence, the stronger fact not proved, any bypass or
external mutability, operational severity, and safer wording or design. Security-sensitive
claims may instead use private reporting described in `SECURITY.md`.
