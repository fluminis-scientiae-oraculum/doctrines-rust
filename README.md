# doctrines-rust

`doctrines-rust` is the canonical Rust engineering doctrine corpus for AI agents and human
engineers. It emphasizes invariants, evidence, correctness boundaries, explicit uncertainty,
and proportionate use of Rust's type system. The corpus is designed to guide planning,
implementation, review, audit, and long-term maintenance without reducing engineering quality
to successful compilation, avoidance of `unwrap`, or a green test run.

Rust prevents broad classes of memory errors, but a memory-safe program can still encode an
impossible invoice, forge authority, skip validation while decoding a database row, retry an
ambiguous payment twice, deadlock under load, expose an unsound safe API, or make an
unmeasured performance claim. This repository treats those failures as design concerns with
named owners, explicit mechanisms, and reviewable evidence.

## Audience

The primary readers are AI planning, implementation, reviewing, auditing, and maintenance
agents; Rust architects and developers; code reviewers; and system designers. Documents use
stable identifiers, repeatable package anatomy, machine-readable manifests, and operational
checklists so an agent can load only the context required for its role. The same material
remains ordinary Markdown that a human can inspect, challenge, and revise through review.

Readers are expected to know basic Rust syntax. The corpus explains why a representation is
chosen, what it proves, how its construction is protected, and where runtime uncertainty
remains. It does not attempt to teach ownership syntax or replace the Rust language
documentation.

## Non-goals

This repository is not:

- a beginner Rust tutorial or a general-purpose Rust book;
- a crate recommendation catalogue;
- a formatting guide that duplicates `rustfmt`;
- a catalogue of Clippy lints;
- an unsafe-code cookbook;
- a prompt archive;
- a collection of clever type tricks;
- or a claim that compile-time representation can freeze mutable external reality.

Libraries appear when they support executable evidence or repository tooling. Their presence
is not a universal endorsement. Examples are deliberately bounded: an example email parser
establishes its documented syntax policy, not deliverability; integer money avoids binary
floating-point representation error, not tax, foreign-exchange, allocation, or rounding
policy.

## Core thesis

Rust engineering quality is not merely memory safety, successful compilation, avoidance of
`unwrap`, or passing tests. A design must discover important invariants, encode the invariants
that are structurally enforceable, constrain legal states and transitions, preserve validation
at trust boundaries, model external failure honestly, represent distributed uncertainty
explicitly, and keep type-system complexity proportional to the risk it removes.

The repository therefore starts before code. It asks which states are meaningful, who owns
each fact, where representations enter from less-trusted systems, which transitions are under
local control, and which observations can become stale. It then chooses among runtime enums,
opaque validated values, consuming transitions, typestate, capabilities, transactional
validation, ordinary runtime rules, and explicit reconciliation states. Typestate has no
privileged rank: persisted, heterogeneous, externally determined, or runtime-inspected state
usually belongs in an enum or a runtime state machine.

The reasoning lifecycle is:

```text
requirements
    ↓
invariant discovery
    ↓
invariant classification
    ↓
trust-boundary identification
    ↓
representation choice
    ↓
legal construction and transition design
    ↓
external failure and uncertainty modelling
    ↓
executable evidence
    ↓
review and guarantee audit
```

Skipping a stage creates predictable blind spots. Starting with structs can preserve
contradictory fields. Starting with typestate can encode a local protocol while ignoring
persistence. Starting with tests can exercise an API whose public constructors already
permit invalid values. Starting with retry policy can collapse an unknown external outcome
into a fictional failure.

## Repository architecture

Canonical content has distinct responsibilities:

- `foundations/` defines shared language for invariants, evidence, boundaries, guarantees,
  and complexity.
- [`doctrines/`](doctrines/README.md) contains versioned normative packages. Every normative rule
  has a stable `RUST-DOC-####-R###` identifier, intent, applicability, exception policy, and
  expected evidence.
- [`patterns/`](patterns/README.md) describes reusable representation choices, their exact
  guarantees, their limitations, and their costs.
- [`boundaries/`](boundaries/README.md) explains how untrusted HTTP, database, message,
  configuration, filesystem, Serde, and FFI representations become trusted domain evidence.
- [`reviews/`](reviews/README.md) contains pass/fail/not-applicable/waiver-oriented procedures.
- [`agents/`](agents/README.md) provides role overlays that select canonical doctrine instead of
  manually duplicating it.
- [`case-studies/`](case-studies/README.md) follows complete problems from weak representations
  through improved designs and residual uncertainty.
- [`examples/`](examples/README.md) provides positive tests, compiler-rejection evidence, and an
  [isolated unsafe abstraction](examples/unsafe-evidence/README.md) checked under Miri.
- [`sources/`](sources/README.md) records provenance, accepted ideas, refinements, and doctrine
  additions.
- [`decisions/`](decisions/README.md) holds the architecture decision records that survive the
  last-resort test in RUST-DOC-0011, with their template and worked examples. The active set is
  currently empty.
- [`templates/`](templates/README.md) and [`rfcs/`](rfcs/README.md) govern future doctrine work.
- [`manifest/`](manifest/README.md) exposes doctrine and agent-pack discovery through YAML
  validated against Draft 2020-12 JSON Schemas.
- [`tools/`](tools/README.md) contains the doctrine linter, the deterministic context bundler, and
  the `doctrine-manifest` crate both decode the manifests through, so one schema-owned
  vocabulary has one decoder rather than one per tool.

Definitions flow into doctrine; doctrine constrains patterns, boundary guides, reviews, and
agent work. A case study may demonstrate doctrine but cannot silently redefine it. Source
notes explain provenance but are non-normative. Executable examples provide evidence for
specific claims without becoming the only expression of a rule.

[`EVIDENCE.md`](EVIDENCE.md) inventories the repository's current executable
evidence by doctrine and names the material gaps that remain.

## Canonical and generated content

Canonical doctrine lives under `foundations/`, `doctrines/`, `patterns/`, `boundaries/`,
`reviews/`, `agents/`, `case-studies/`, `decisions/`, `templates/`, `rfcs/`, and `sources/`.

> [!TIP]
> To load this corpus into an agent, read
> [Loading the doctrine into an agent](dist/README.md#loading-the-doctrine-into-an-agent). It
> covers which bundle to pick, how to attach it, and what to tell the agent alongside it. That
> guidance travels with the bundles, so it is available to someone holding only `dist/`.

Three kinds of file are generated and are never edited by hand:

- everything under [`dist/`](dist/README.md), the hydration bundles projected from canonical
  sources;
- [`rfcs/accepted/README.md`](rfcs/accepted/README.md), the accepted-RFC index, built from
  `rfcs/accepted/overview.md` and the front matter of each accepted RFC;
- [`doctrines/map.md`](doctrines/map.md), the doctrine coverage map, built from
  `doctrines/map-overview.md` and the two manifests.

Every generated file carries a banner naming the sources it was built from. After a canonical
change, run:

```bash
cargo run -p bundle-agent-context -- generate
cargo run -p bundle-agent-context -- check
```

`check` reconstructs expected bytes in memory and fails if any generated file differs, whether
or not it lives under `dist/`. This separation permits compact agent loading, and a current
index, without creating a second manually maintained doctrine.

## Reading paths

The role packs under `agents/` state exact required inputs and outputs. The generated versions
under `dist/agents/` combine those overlays with selected canonical rules.

- A planner begins with [`foundations/invariants.md`](foundations/invariants.md),
  [`foundations/trust-boundaries.md`](foundations/trust-boundaries.md),
  [`foundations/complexity-budget.md`](foundations/complexity-budget.md),
  [`agents/planner.md`](agents/planner.md), and
  [`reviews/pre-implementation.md`](reviews/pre-implementation.md). Its output is an invariant
  inventory, boundary map, state graph, authority map, effect and uncertainty inventory,
  persistence choice, complexity decision, and evidence plan.
- An implementer reads the relevant doctrine package,
  [`agents/implementer.md`](agents/implementer.md), the applicable
  [boundary guide](boundaries/README.md), and the matching [example crate](examples/README.md).
  It protects constructors, retains external fallibility, and supplies positive, negative, and
  compiler-rejection evidence where useful.
- A reviewer reads [`foundations/guarantee-honesty.md`](foundations/guarantee-honesty.md),
  [`agents/reviewer.md`](agents/reviewer.md), the relevant doctrine review standard, and the
  [operational review documents](reviews/README.md). It inspects construction paths and bypasses
  rather than approving merely idiomatic syntax.
- An auditor loads [`agents/auditor.md`](agents/auditor.md), all applicable
  [boundary guides](boundaries/README.md), and
  [`reviews/final-correctness-audit.md`](reviews/final-correctness-audit.md). It searches
  adversarially for forged evidence, unchecked decoding, authority leakage, unsafe retry, and
  misleading claims.
- A maintainer reads [`foundations/normative-language.md`](foundations/normative-language.md),
  [`agents/maintainer.md`](agents/maintainer.md), [`rfcs/README.md`](rfcs/README.md), and the
  affected [source notes](sources/README.md) before changing meaning, versions, manifests, or
  generated outputs. It also revalidates or expires active decision records under
  [`decisions/`](decisions/README.md).
- Any role about to write a document first applies
  [RUST-DOC-0011](doctrines/0011-executable-narrative/README.md) and
  [`reviews/executable-narrative-review.md`](reviews/executable-narrative-review.md): classify
  the claim, name the single artifact authoritative for it, prefer moving an enforceable
  obligation into the mechanism that enforces it, and write a decision record only for a fact no
  artifact can carry. The most common correct outcome is that no document is added.
- A human architect normally reads all [foundations](foundations/README.md),
  [Doctrine 0001](doctrines/0001-invalid-states/README.md), the doctrines relevant to the
  system's risks, the decision frameworks, and one structurally similar
  [case study](case-studies/README.md).

> [!TIP]
> [`doctrines/map.md`](doctrines/map.md) shows which doctrine each role pack hydrates. A doctrine
> a pack omits is one that is not available from that pack alone and has to be loaded from its
> canonical source; the exclusions are the interesting cells.

For constrained context windows, `dist/compact-doctrine.md` contains the shared thesis,
classification and boundary pipelines, every active doctrine's normative rules, the central
decision tree, and the core audit gates. `dist/full-doctrine.md` retains the repository identity,
foundations, every file in each active doctrine package, patterns, boundary guides, operational
reviews, and shared agent obligations in stable order. Case studies, source notes, RFCs,
templates, and role-specific overlays remain canonical but are deliberately outside that
hydration bundle.

## Doctrine index

`manifest/doctrines.yaml` is the machine-readable discovery source, and
[`doctrines/README.md`](doctrines/README.md) is the reader-facing index derived from it. This
file deliberately does not repeat that table. It previously did, and the copy was wrong within
one release: two doctrines were added and the table still listed nine while claiming to be
synchronized with the manifest. `doctrine-lint` now checks the surviving index against the
manifest, so a doctrine cannot be added without the index following.

That is `RUST-DOC-0011-R004` and `RUST-DOC-0011-R017` applied to this repository's own front
page: one authority, one mechanically checked view, and no third copy to keep in step.
[`doctrines/map.md`](doctrines/map.md) is a second view of the same manifests, and it is
generated for the same reason rather than written.

The corpus does not claim `1.0` semantic stability, and doctrines version independently of the
repository and of each other. Patch releases clarify without changing normative meaning; minor
releases add normative requirements or substantial compatible material; major releases may make
normative contracts incompatible. The current repository version is `repository_version` in
`manifest/doctrines.yaml`, which `doctrine-lint` holds equal to the workspace package version;
per-doctrine versions and status live in the same manifest and in each package's front matter.
This file names no version number, for the same reason it no longer repeats the index.

## Local validation

The pinned development toolchain is Rust 1.97.1. The workspace MSRV is Rust 1.85.0, the first
stable release supporting Edition 2024; selected dependencies declare compatibility with that
floor. Markdown tooling uses the exact Node.js, Prettier, and markdownlint-cli2 versions pinned
by `.node-version` and `package-lock.json`. Run from the repository root:

```bash
npm ci --ignore-scripts --no-audit
npm audit --audit-level=high
npm run check:markdown-format
npm run lint:markdown
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p doctrine-lint -- check
cargo run -p bundle-agent-context -- generate
git diff --exit-code -- dist/ rfcs/accepted/README.md doctrines/map.md
cargo run -p bundle-agent-context -- check
cargo deny check
lychee --no-progress '**/*.md'
git diff --check
```

Run `npm run format:markdown` to format canonical and repository-governance Markdown before
regenerating bundles. Prettier deliberately ignores every generated file, `dist/` and the
accepted-RFC index alike; generated Markdown changes only through `bundle-agent-context`. The
workspace test command includes the `trybuild` compile-fail suite. CI exposes Markdown
dependency audit, format, and lint as a distinct pull-request gate,
then repeats Rust formatting, Clippy, tests, schema validation, bundle drift detection,
dependency policy, and link checks with read-only repository permissions. CI confirms locally
discovered behavior; it is not the first compiler, linter, or formatter.

## Contributing and evolution

Read [`CONTRIBUTING.md`](CONTRIBUTING.md) and repository [`AGENTS.md`](AGENTS.md) before changing
content. Corrections that preserve normative meaning may use an ordinary pull request. New
doctrines, normative rule additions or weakening, new escape hatches, supersession,
normative-term changes, significant bundle restructuring, license changes, and MSRV policy
changes require an RFC under [`rfcs/`](rfcs/README.md).

Every normative change identifies affected rule IDs, updates manifests and source provenance,
states compatibility and migration consequences, and regenerates `dist/`. A reviewer requires
a guarantee ledger separating what a mechanism establishes from what remains unproved. If a
claim is stronger than its constructors, boundary decoding, or external evidence, use the
guarantee-overclaim issue form.

Released changes are recorded in [`CHANGELOG.md`](CHANGELOG.md). Participation is governed by the
[`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md), and vulnerability reporting by
[`SECURITY.md`](SECURITY.md).

## License

Documentation — including Markdown doctrine, manifests, schemas, and generated doctrine
bundles — is licensed under Creative Commons Attribution 4.0 International; see
[`LICENSE-DOCS`](LICENSE-DOCS). Rust source, test fixtures, repository tools, and reusable
workflow or configuration code are available under MIT OR Apache-2.0 at the recipient's option;
see [`LICENSE-CODE`](LICENSE-CODE). External quotations and linked source material remain subject
to their respective rights.
