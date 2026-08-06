# Repository contract for coding agents

This file governs AI-agent work in `doctrines-rust`. It applies to planning, authoring,
implementation, review, audit, and maintenance. A green compiler result does not override this
contract.

## Required reading

Before changing doctrine, read every document under `foundations/` in its stated order. Then
read the complete affected doctrine package: its metadata, normative rules, rationale,
decision framework, review standard, anti-pattern catalogue, glossary, and references. Read
the corresponding source notes and every boundary or pattern document named in
`manifest/doctrines.yaml`.

Before changing code only, read the doctrines implicated by the behavior and the relevant
example or tool README. Code changes can alter doctrine evidence even when prose is untouched.

## Canonical source discipline

Canonical sources live under `foundations/`, `doctrines/`, `patterns/`, `boundaries/`,
`reviews/`, `agents/`, `case-studies/`, `decisions/`, `templates/`, `rfcs/`, and `sources/`.
Files under `dist/`, and the accepted-RFC index `rfcs/accepted/README.md`, are generated
projections.

An agent MUST NOT edit a generated file manually. `dist/` is generated in full, and
`rfcs/accepted/README.md` is generated from `rfcs/accepted/overview.md` and each RFC's front
matter; both carry a banner naming their source. After any canonical change, run:

```bash
cargo run -p bundle-agent-context -- generate
```

Inspect the generated diff, then run `cargo run -p bundle-agent-context -- check`. If
generation produces unrelated change, diagnose the manifest ordering or bundler logic rather
than patching the output.

## Meaning and governance

Classify every prose change as either a wording correction, a non-normative clarification, or
a normative change. Wording corrections preserve all obligations and exceptions. A change is
normative when it adds, removes, weakens, broadens, narrows, or reinterprets a MUST, MUST NOT,
SHOULD, SHOULD NOT, MAY, exception, waiver, guarantee, or compliance condition.

An RFC is required for normative weakening, a new escape hatch, doctrine supersession, new
doctrine, new normative rule, changed normative-term meaning, significant generated-pack
restructuring, license policy, or MSRV policy. Do not hide a normative change in rationale,
examples, formatting, or a generated file. Preserve doctrine and rule IDs; supersession uses
manifest metadata and governance rather than renumbering history.

Architecture claims must be contract-shaped: identify the concern and owner; state
preconditions, postconditions, invariants, obligations, failure semantics, and evidence.
Known context is evidence, not permission to publish facts unrelated to this repository.

Before writing any document that describes an obligation, apply RUST-DOC-0011. Classify the
claim, name the single artifact authoritative for it, and prefer moving an enforceable obligation
into the mechanism that enforces it over describing it. Do not add a second manually maintained
copy of something an artifact already enforces, and prefer a generated, drift-checked view to a
synchronized one. Create a decision record only for a fact that cannot be represented, enforced,
generated, or recovered from the artifacts, and then only with an owner, a revalidation trigger,
an obsolescence condition, and links to the artifacts that stay authoritative; register it in
`manifest/decision-records.yaml`. Do not cite an existing record against a change without
confirming its constraint still applies. Where a rationale is unavailable, record it as unknown
rather than inferring one from the implementation.

## Guarantee honesty

For each trusted type or state, state:

1. what it proves;
2. how that evidence is established;
3. how construction is protected;
4. how Serde, database, FFI, and other decoding paths preserve it;
5. which explicit escape hatches exist;
6. what it does not prove;
7. what external facts can change;
8. which failures remain runtime failures;
9. which outcomes can become indeterminate.

Do not overstate type guarantees. `NonZeroU64` proves non-zero, not a complete monetary
policy. A syntax-validated address is not ownership verification. `Connection<Open>` proves a
local transition completed, not future remote liveness. A timeout may follow a committed
external effect. Tests are scoped evidence, not universal proof.

Use typestate only for locally controlled, small, static protocols when compiler enforcement
outweighs API, diagnostic, persistence, monomorphization, and maintenance cost. Prefer runtime
enums for dynamic, persisted, heterogeneous, externally chosen, or runtime-inspected state.

## Code obligations

Trusted newtypes have private representations and complete smart constructors. Derived
deserialization and database decoding must not bypass constructors. External effects remain
fallible. Ambiguous distributed outcomes remain explicit and carry reconciliation identity.
Capability cloning, authority transfer, revocation, secret formatting, and interior
mutability require deliberate contracts.

Do not introduce unsafe code without full RUST-DOC-0007 compliance. Every unsafe block needs a
specific safety invariant, and every safe public API built above it must be sound for all safe
callers. Dependency unsafe code is part of review scope.

Do not add a dependency casually. State the capability gained, why standard-library or
existing dependencies are insufficient, MSRV and license compatibility, maintenance and
supply-chain cost, feature selection, and whether default features are necessary.

Compile-fail evidence is reviewed evidence. Do not silently overwrite generated `.stderr`
files. When the pinned compiler changes a diagnostic, inspect the semantic rejection, ensure
the intended failure still occurs at the intended boundary, and commit only the reviewed
diagnostic.

## Required validation

Before handing off a change, run the complete local validation sequence in the root
[`README.md`](README.md#local-validation). It is stated there once and nowhere else, so a
gate cannot be added in one document and missed in another; `doctrine-lint` rejects a second
copy.

Before bundle generation, run `npm run format:markdown` after editing canonical or governance
Markdown. Prettier deliberately excludes every generated file; never use a formatter to rewrite
bundles directly. The Markdown lint configuration uses narrow documented structural
exceptions and MUST NOT be weakened merely to make a change pass.

Run checks on both the pinned toolchain and MSRV when changing dependencies, public example
APIs, or compiler evidence. Confirm manifest schemas validate actual YAML. Inspect
`git status --short` and the repository inventory. Remove build output, temporary diagnostics,
archives, credentials, editor debris, and unrelated changes.

Reports must name commands actually run and their observed results. If a tool is unavailable,
state that limitation; do not claim its check. A change is not complete while generated drift,
compiler failure, Clippy warning, test failure, schema failure, link failure, or unexplained
working-tree change remains.
