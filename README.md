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
- `doctrines/` contains versioned normative packages. Every normative rule has a stable
  `RUST-DOC-####-R###` identifier, intent, applicability, exception policy, and expected
  evidence.
- `patterns/` describes reusable representation choices, their exact guarantees, their
  limitations, and their costs.
- `boundaries/` explains how untrusted HTTP, database, message, configuration, filesystem,
  Serde, and FFI representations become trusted domain evidence.
- `reviews/` contains pass/fail/not-applicable/waiver-oriented procedures.
- `agents/` provides role overlays that select canonical doctrine instead of manually
  duplicating it.
- `case-studies/` follows complete problems from weak representations through improved
  designs and residual uncertainty.
- `examples/` provides positive tests and compiler-rejection evidence.
- `sources/` records provenance, accepted ideas, refinements, and doctrine additions.
- `templates/` and `rfcs/` govern future doctrine work.
- `manifest/` exposes doctrine and agent-pack discovery through YAML validated against Draft
  2020-12 JSON Schemas.
- `tools/` contains the doctrine linter and deterministic context bundler.

Definitions flow into doctrine; doctrine constrains patterns, boundary guides, reviews, and
agent work. A case study may demonstrate doctrine but cannot silently redefine it. Source
notes explain provenance but are non-normative. Executable examples provide evidence for
specific claims without becoming the only expression of a rule.

## Canonical and generated content

Canonical doctrine lives under `foundations/`, `doctrines/`, `patterns/`, `boundaries/`,
`reviews/`, `agents/`, `case-studies/`, `templates/`, `rfcs/`, and `sources/`. Generated
hydration bundles live under `dist/`.

Files in `dist/` are deterministic projections of canonical sources. Every generated file
contains a warning banner and source-path provenance headings. Contributors MUST NOT edit
those files directly. After a canonical change, run:

```bash
cargo run -p bundle-agent-context -- generate
cargo run -p bundle-agent-context -- check
```

`check` reconstructs expected bytes in memory and fails if a tracked distribution differs.
This separation permits compact agent loading without creating a second manually maintained
doctrine.

## Reading paths

The role packs under `agents/` state exact required inputs and outputs. The generated versions
under `dist/agents/` combine those overlays with selected canonical rules.

- A planner begins with `foundations/invariants.md`, `foundations/trust-boundaries.md`,
  `foundations/complexity-budget.md`, `agents/planner.md`, and
  `reviews/pre-implementation.md`. Its output is an invariant inventory, boundary map, state
  graph, authority map, effect and uncertainty inventory, persistence choice, complexity
  decision, and evidence plan.
- An implementer reads the relevant doctrine package, `agents/implementer.md`, the applicable
  boundary guide, and the matching example crate. It protects constructors, retains external
  fallibility, and supplies positive, negative, and compiler-rejection evidence where useful.
- A reviewer reads `foundations/guarantee-honesty.md`, `agents/reviewer.md`, the relevant
  doctrine review standard, and the operational review documents. It inspects construction
  paths and bypasses rather than approving merely idiomatic syntax.
- An auditor loads `agents/auditor.md`, all applicable boundary guides, and
  `reviews/final-correctness-audit.md`. It searches adversarially for forged evidence,
  unchecked decoding, authority leakage, unsafe retry, and misleading claims.
- A maintainer reads `foundations/normative-language.md`, `agents/maintainer.md`,
  `rfcs/README.md`, and the affected source notes before changing meaning, versions,
  manifests, or generated outputs.
- A human architect normally reads all foundations, Doctrine 0001, the doctrines relevant to
  the system's risks, the decision frameworks, and one structurally similar case study.

For constrained context windows, `dist/compact-doctrine.md` contains the shared thesis,
classification and boundary pipelines, every active doctrine's normative rules, the central
decision tree, and the core audit gates. `dist/full-doctrine.md` retains complete canonical
material in stable order.

## Doctrine index

This index is synchronized with `manifest/doctrines.yaml`, which is the machine-readable
discovery source.

| ID | Active doctrine | Principal concern |
|---|---|---|
| RUST-DOC-0001 | [Making Invalid States Unrepresentable](doctrines/0001-invalid-states/README.md) | Invariant discovery, representation, construction, transitions, honest uncertainty |
| RUST-DOC-0002 | [Error Modeling as Domain Design](doctrines/0002-error-modeling/README.md) | Structured failure, recoverability, retryability, panic boundaries |
| RUST-DOC-0003 | [Ownership as Authority and Lifecycle](doctrines/0003-ownership-and-capabilities/README.md) | Exclusive custody, capabilities, transfer, revocation, secrets |
| RUST-DOC-0004 | [Concurrency and Async Correctness](doctrines/0004-concurrency-and-async/README.md) | Cancellation, backpressure, task ownership, synchronization |
| RUST-DOC-0005 | [Persistence Boundaries and Domain Integrity](doctrines/0005-persistence-boundaries/README.md) | Decoding, migrations, transactions, historical data |
| RUST-DOC-0006 | [Distributed Effects, Uncertainty, and Reconciliation](doctrines/0006-distributed-uncertainty/README.md) | Idempotency, ambiguity, duplicates, reconciliation |
| RUST-DOC-0007 | [Unsafe Rust as a Proof Obligation](doctrines/0007-unsafe-rust/README.md) | Safety invariants, encapsulation, FFI, validation tooling |
| RUST-DOC-0008 | [Testing as Layered Evidence](doctrines/0008-testing-and-evidence/README.md) | Evidence scope, forbidden programs, faults, model checking |
| RUST-DOC-0009 | [Performance Claims Require Measurement](doctrines/0009-performance-and-measurement/README.md) | Workloads, profiling, distributions, regressions |

All doctrines begin at version `0.1.0`. Repository `0.1.0` establishes the initial corpus but
does not claim `1.0` semantic stability. Patch releases clarify without changing normative
meaning; minor releases add normative requirements or substantial compatible material; major
releases may make normative contracts incompatible. Individual doctrines may later version
independently.

## Local validation

The pinned development toolchain is Rust 1.97.1. The workspace MSRV is Rust 1.85.0, the first
stable release supporting Edition 2024; selected dependencies declare compatibility with that
floor. Run from the repository root:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p doctrine-lint -- check
cargo run -p bundle-agent-context -- generate
git diff --exit-code -- dist/
cargo run -p bundle-agent-context -- check
cargo deny check
lychee --no-progress '**/*.md'
git diff --check
```

The workspace test command includes the `trybuild` compile-fail suite. CI repeats formatting,
linting, tests, schema validation, bundle drift detection, and link checks with read-only
repository permissions. CI confirms locally discovered behavior; it is not the first compiler
or formatter.

## Contributing and evolution

Read `CONTRIBUTING.md` and repository `AGENTS.md` before changing content. Corrections that
preserve normative meaning may use an ordinary pull request. New doctrines, normative rule
additions or weakening, new escape hatches, supersession, normative-term changes, significant
bundle restructuring, license changes, and MSRV policy changes require an RFC under `rfcs/`.

Every normative change identifies affected rule IDs, updates manifests and source provenance,
states compatibility and migration consequences, and regenerates `dist/`. A reviewer requires
a guarantee ledger separating what a mechanism establishes from what remains unproved. If a
claim is stronger than its constructors, boundary decoding, or external evidence, use the
guarantee-overclaim issue form.

## License

Documentation—including Markdown doctrine, manifests, schemas, and generated doctrine
bundles—is licensed under Creative Commons Attribution 4.0 International; see
`LICENSE-DOCS`. Rust source, test fixtures, repository tools, and reusable workflow or
configuration code are available under MIT OR Apache-2.0 at the recipient's option; see
`LICENSE-CODE`. External quotations and linked source material remain subject to their
respective rights.
