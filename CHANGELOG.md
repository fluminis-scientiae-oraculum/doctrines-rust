# Changelog

All notable changes are documented here. Repository releases follow semantic versioning while
the corpus is pre-1.0: patch releases preserve normative meaning, minor releases may add
compatible normative requirements, and major releases may change doctrine contracts.

## [0.9.0] — 2026-08-10

Connects every file, directory, and crate the corpus had left unreachable, and gates the
connection so the next one cannot land unnoticed. A repository-wide audit before release then
held the corpus to its own doctrine and fixed what it found. No normative meaning changes; the
version is minor because the new gates add requirements a future change has to satisfy.

### Audited before release

- The pinned development toolchain, the workspace MSRV, and the Miri nightly were each
  restated by hand in several places with nothing comparing them — the clippy MSRV, the
  examples matrix and its display names, the CI install step, the Miri job's own name and
  install step, and every Miri invocation the corpus tells a reader to run. That is
  `RUST-DOC-0011-R004` in this repository's own tree. `check_pinned_versions` now holds each
  restatement equal to the file that declares it, reading only artifacts the repository owns.
  Both prose copies were removed rather than gated, since the remedy R004 prefers is to delete
  the second source.
- `RUST-DOC-0002-R010` waived itself with "No production unwrap or expect exists to justify".
  Two do, in `tools/bundle-agent-context/src/main.rs`, and they are model compliance with the
  rule: each states the invariant that makes failure a defect. The waiver is now the artifact.
- `RUST-DOC-0008-R020` said the inventory test "scopes its reads to the manifest directory".
  It reads the workspace manifest one level above it. What is true, and what the rule is about,
  is that every read derives from `CARGO_MANIFEST_DIR` rather than the working directory.
- Gate `T20` declared `grep -q 'channel = "1.97.1"' rust-toolchain.toml`, which decided only
  that a file contains a string, embedded a fourth copy of the toolchain version in a doctrine
  package, and could pass while `RUSTUP_TOOLCHAIN` overrode the pin. It now declares the
  command that does decide it: `trybuild` fails when recorded diagnostics disagree with the
  resolved compiler.
- Gate `T48` declared the workspace test command for "Are tests isolated in parallel?". A green
  run is equally consistent with isolated and non-isolated tests; one that shares a fixed file
  passes or fails by scheduling. The gate is judgment, which is what its own pass evidence
  ("unique resources") always described.
- `EVIDENCE.md` omitted the three `evidence_of_absence` tests for the whole release that
  introduced the rule they enforce, so its counts summed to three fewer tests than the
  workspace runs. It also said six `doctrine-manifest` tests check a decoded vocabulary while
  the next paragraph said four — four is right — and opened its connectivity bullet with a
  count of eleven against nine described items, two of which were one test counted twice.
- The root `README.md` said "This file names no version number" four lines above naming two.

### Fixed in 0.9.0

- `tools/README.md` described "two Rust CLIs" and never named `doctrine-manifest`, the
  library both binaries decode the manifests through. It was the one workspace member no
  Markdown file had ever linked — mentioned six times in backticks, reachable from nowhere.
- `examples/README.md` inventoried seven crates while the workspace held nine. The missing
  two were `staged-protocol` and `doctrine-examples`, the latter being the crate the
  directory itself compiles to and the named enforcement artifact for three rules of
  `RUST-DOC-0008`. Crate names in both indexes were backticks rather than links.
- `.github/pull_request_template.md` had no inbound link from anywhere in the repository,
  and named four manifest and generated paths in backticks that navigated nowhere. Neither
  defect was visible, because no scan reached `.github/`.
- `CONTRIBUTING.md` and the root `README.md` referred to "the guarantee-overclaim issue
  form" in prose a reader could not click. All three issue forms, the pull-request template,
  and the release workflow are now links.
- `EVIDENCE.md` claimed eighty tooling tests when the workspace held eighty-one.
- A fourth review round found fifteen defects in the gates this release adds, and all
  fifteen are fixed here. Three made a gate silently wrong: crate coverage credited a link
  whose target does not exist, so a typo satisfied the very check written to catch a crate
  linked from nowhere; deleting the file-coverage gate silently removed a second obligation
  nobody reasoned about, leaving Markdown outside every declared root subject to no rule;
  and the dated-record filter let a directory of archived records escape the index
  requirement. Five made the gates fail on correct repositories inside the sequence the
  README calls mandatory — a trailing-slash or globbed `[workspace] members` entry, a
  Cargo-excluded crate, a gitignored `wip/` scratch directory, a Markdown issue template,
  and a directory name that does not round-trip through `to_string_lossy`, which blamed a
  correct `Cargo.toml` with a diagnostic no edit to it could clear.
- The file-coverage gate was deleted in this release because a check needing another tool's
  ignore semantics cannot be made correct, but `check_directory_indexes` was left walking
  the same working tree with the same dependency, and reproduced the same false failure.
  The index requirement is now derived from declared artifacts only — `[workspace] members`
  plus the root lists — never from a filesystem walk.
- A fifth review round found fifteen more, all fixed here. `doctrine-lint` no longer
  compiled on the workspace MSRV of 1.85.0 — a Rust 1.88 let-chain — and nothing caught it
  because the same release had excluded the tool crates from the only MSRV job; that job
  now checks the whole workspace. `.github/README.md` would have replaced the repository's
  front page on GitHub, which resolves it before the root `README.md`; the automation index
  is now `.github/AUTOMATION.md` and the directory is registered as index-free.
- The scratch-directory skip reached one of four walkers. It is now a single predicate in
  `doctrine-manifest` that `doctrine-lint`'s two walks, its forbidden-marker scan, and
  `bundle-agent-context` all consult, so a sanctioned `wip/` no longer fails the mandatory
  sequence three commands after passing the first.
- Narrowing the index obligation to declared directories had dropped it for roughly two
  dozen nested indexes. It is restored from the link graph — a directory something links to
  owes the reader an index — which is committed content rather than a filesystem walk.
- `check_workflow_index` resolved links by raw text prefix, rejecting the `./` form GitHub
  follows and passing a dangling target; `check_lint_parity` guarded one hardcoded crate
  rather than every crate that cannot inherit workspace lints; and `[workspace] exclude` was
  subtracted from explicitly listed members, which Cargo does not do and which silently
  removed the crate from every membership gate.
- A sixth review round found fifteen more, all fixed here. The worst was a test this
  repository's own `RUST-DOC-0008-R022` forbids: the lint-parity test asserted an empty
  diagnostic list guarded only by a members-list length, so renaming the single
  non-inheriting crate would have made it pass having compared nothing. It now counts the
  comparisons it performed and fails if that count is zero. The only test of the index
  gate contained no Markdown links at all, leaving the branch it was updated to cover dead
  during its own run; its fixture now links a directory, generated output, and scratch
  space, and the branch is mutation-controlled.
- The link-derived index obligation shipped without the two carve-outs its four sibling
  consumers already carried, so a link to a directory under `dist/` demanded an index the
  bundler never emits, and a link to scratch space demanded one inside a gitignored tree.
  It also concluded absence from a partial read; it now reports the shortfall instead.
- `check_lint_parity` had been moved behind `check_connectivity`'s four early returns, so a
  malformed unrelated manifest key silently stopped it running; it reads its own membership
  and is dispatched independently. A member with no `Cargo.toml` was skipped in silence and
  is now reported, because no other gate examined it either.
- The scratch predicate was applied at different points in the two crates — before the
  symlink guard in one, after it in the other — so a committed symlink named `wip` made the
  two tools contradict each other one command apart. They now agree.
- Eight doc comments and README rows described code the same commits had changed: the index
  gate's own comment still declared its required set was never discovered, the membership
  reader's rustdoc argued for the exclude behaviour that was reversed, and the workflow
  index's comment still named the file the rename exists to abolish. A register was spliced
  between a neighbouring register's doc comment and its declaration, leaving each described
  by the other's.
- `PATH_REFERENCE_EXEMPTIONS` suspended the whole check rather than the linking rule, which
  removed the last verification that the pull-request template names real files. The
  exemption is now narrow: those paths must still exist.
- Three claims that had stopped being true: `doctrine-lint`'s README advertised a control
  test deleted with that gate, the root `README.md` said every invocation resolves to the
  pinned toolchain when the MSRV matrix and the Miri job deliberately override it, and
  `examples/boundary-validation` promised it reused the shared bound while re-implementing
  it, duplicate constant included. The example now uses `BoundedName`.
- The workspace clippy denies added in this release never reached
  `examples/unsafe-evidence`, the one crate holding `unsafe`, because Cargo cannot inherit
  lints into a crate that declares any lint table of its own. The copy is now compared
  mechanically. A comment claiming two lints rejected a collected `Vec<&String>` was false
  in both directions and is replaced by a statement of what nothing enforces.

### Added in 0.9.0

- `check_declared_top_level`, `check_workflow_index`, and `check_lint_parity`: every
  top-level directory is a declared root or is registered with a reason; the workflow index
  names exactly the workflows on disk; and the one crate that cannot inherit workspace
  clippy lints restates them without drift.
- Eleven package indexes: one per `tools/` crate, one per `examples/` crate that lacked one,
  and `.github/AUTOMATION.md`, which describes what each workflow gates and which issue form
  answers which question.
- A `Repository configuration` section in the root `README.md` linking every root
  configuration file and naming what each governs, and a paragraph in `rfcs/README.md` and
  `templates/README.md` explaining the nested Markdown-lint override each directory carries.
- Two checks in `doctrine-lint`. A workspace crate has to be linked from prose outside
  itself, so a crate whose own README is its only inbound link is reported as the island it
  is; and a crate, or any directory holding maintained Markdown, has to carry an index.
- `rust-examples.yml` now runs `cargo test --workspace --exclude` the three tools instead of
  naming each example with `-p`. The `-p` list was a second copy of `[workspace] members`,
  and the exclusion list is the half that does not grow, so a new example crate is tested
  the day it is added and there is no longer a duplicate to keep in sync.
- `.github` joins the reachability scope, which is what made the pull-request template's
  defects visible at all.
- One register, `INDEXLESS_DIRECTORIES`, recording directories that deliberately carry no
  index, in the shape `REACHABILITY_EXEMPTIONS` already used. It is empty, which is the
  healthy state. Every entry states a reason, and a test asserts each reason is long enough
  to be one and that each named path still exists.
- Selected `clippy::nursery` lints in `[workspace.lints.clippy]`, named individually rather
  than enabled as a group so a toolchain bump cannot turn unrelated new lints into build
  failures. `missing_const_for_fn` found nine functions across the examples and both
  binaries that could be `const` and were not. `option_if_let_else` is allowed with a
  stated reason: it rewrites a two-armed `match` into `map_or_else`, whose default argument
  comes first, and this corpus optimises for a reader following prose and code together.
- Six tests. Each new check is exercised on a seeded violation rather than on the passing
  corpus, one pins the workspace-membership parser, and two control the checks themselves.
  Every check was also positive-controlled live: seeded in the working tree, observed
  failing with its own diagnostic, and restored. The island test is mutation-controlled —
  deleting the guard it is named for makes it fail.
- A `toml` dependency, so the workspace manifest is parsed rather than pattern-matched.

### Fixed before release in 0.9.0

Two adversarial review rounds found twenty-one defects in the new checks. Every one was
reproduced and fixed here rather than shipped. The first round found six:

- The new walk read the working tree, not the repository, so a contributor with an editor
  open (`*.swp`) or who had run the link checker (`.lycheecache`) failed the mandatory local
  validation sequence. It now honours `.gitignore`, and reports a pattern it cannot express
  instead of silently misreading it.
- The walk pruned skipped directories by first path component only, so a nested
  `node_modules/` or `target/` was descended into and every file under it reported.
- The mention test was an unanchored substring search: `ci.yml` counted as named because a
  document mentioned `doctrine-ci.yml`. Anchoring it then over-corrected — treating `/` as
  part of a name un-named six files their indexes do link — so a separator is a boundary.
- The workspace-member parser left the array only on a line beginning `]`, so an array
  closing as `"last"]` swept the rest of the manifest in as members, and a commented-out
  member counted as live.
- The workflow package list was harvested from the whole file, so a crate tested only by the
  Miri job, or merely named in a YAML comment, satisfied the gate that exists to prove the
  matrix builds it. The `-p` scanner also truncated any package name at an underscore.
- Three checks each re-walked the Markdown scope, so one unreadable entry or symbolic link
  was reported once per check.

A second round, run against those repairs, found fifteen more — most of them in the repairs
themselves. The three worst were a crash and two silent false passes:

- `names_whole`, written to fix the first round's anchoring bug, advanced a byte index by
  hand after a failed match. On a multibyte filename that index could land inside a
  character, and slicing there aborted the whole lint with a backtrace instead of a
  diagnostic — no result at all for any check, rather than a wrong one.
- The same function anchored only the character _before_ a match, so a filename that was a
  prefix of a mentioned one was credited: `deny.toml` passed because a document named
  `deny.toml.license`. The test written to pin that function asserted only the suffix
  direction.
- The workflow gate armed on any line containing `cargo test`, so a package named in a log
  message satisfied it while CI silently stopped building that crate.

The rest were of a kind: hand-rolled parsers for three specified formats. The response was to
stop hand-rolling them rather than patch them again — a real TOML reader for the manifest,
and deletion of the YAML gate in favour of removing the duplicate list it guarded.

A third round, run against those repairs in turn, found fifteen more, and nine of them were
in one gate: the check requiring every remaining file to be named by some document, together
with the ignore matcher it needed. Six of the nine were the mandatory gate rejecting a
correct repository — a filename ending a sentence, a `git worktree` checkout, a developer's
own exclude file, a gitignored scratch directory, ordinary editor state, a file documented
in the accepted RFC that introduced it.

That gate is now deleted, and the deletion is the finding. Answering "is every file named
somewhere?" needs a walk of the working tree, and a working tree is the repository plus
whatever a contributor's tools left in it, so correctness meant reimplementing `.gitignore`.
Each round made the matcher more faithful and each round it was still not Git, with the
failures landing in a pre-commit sequence the repository calls mandatory. The question was
worth asking once — it found twenty-six files named by nothing, and they are connected now —
but the answer did not need a permanent gate, and the gate cost more than it returned.

What survived is the two checks that caught the defects this release exists to fix: the one
that found the shared manifest library linked from nowhere, and the one that found this very
crate without an index. Both answer a question about the corpus using only the corpus.

## [0.8.1] — 2026-08-08

Repairs prose the `0.8.0` rationale audit damaged, and closes the schema gap that audit
opened between the corpus and the documents a new package is written from. No normative
meaning changes.

### Fixed in 0.8.1

- Four rationale sections left incoherent by the `R012` audit. `RUST-DOC-0009`'s regression
  gates ended mid-sentence; `RUST-DOC-0008`'s evidence-plan conclusion kept a "therefore"
  whose premise was removed; `RUST-DOC-0006`'s idempotency section opened on a "none of
  these" whose antecedent was gone; and `RUST-DOC-0007`'s partial-initialization paragraph
  lost the subject of "leaking values". Each is repaired without restoring the restatement
  `R012` removed. The repairs move `rationale.md` from the 1,383 lines `0.8.0` and RFC-0007
  record to 1,386; both remain accurate as statements about `0.8.0`.
- `foundations/normative-language.md` listed five rule fields while the corpus required six,
  understating its own contract.
- `templates/doctrine/doctrine.md` had no `**Enforcement.**` field and
  `templates/doctrine/review-standard.md` had no `Check` column and used gate identifiers
  (`NNNN-01`) that the coded-identifier convention does not recognise. A package written
  from the template would have failed both checks `0.8.0` added, with nothing in the
  template to say why.
- `templates/doctrine-proposal.md` gains an `Enforcement` column, so a proposal states the
  enforcing artifact when the rule is proposed rather than when it is merged.

### Added in 0.8.1

- A test asserting the template satisfies what every package must: each rule in the
  template carries an enforcement field, the gate table carries a `Check` column with coded
  identifiers, and the documented field list names the enforcement field.
  Positive-controlled against all three omissions.

## [0.8.0] — 2026-08-08

Adds no normative rule and changes no rule statement, applicability, exception, or
identifier. Every rule now names what enforces it, and every review gate declares whether
a command decides it. The measured result is the point: 80 of 208 rules name an
enforcement artifact and 128 state a waiver, while 32 of 486 gates are command-decidable
and 454 are judgment. Gate identifiers are unified, which changes review-evidence
citations in three packages and is why those take a minor version. RFC-0007, closing
issues #12, #13, and #14.

### Added in 0.8.0

- An `**Enforcement.**` field on all 208 rules, naming a linked repository artifact that
  must exist, or opening with `Unenforceable:` and a stated reason. The field lives on the
  rule rather than in the manifest, because a per-rule manifest entry would copy 208 rule
  identifiers into a second maintained file — the competing copy `RUST-DOC-0011-R004`
  prohibits.
- A `Check` column on all 486 review gates, valued `judgment` or `mechanical(<command>)`,
  so "how much of review is mechanical" is a property of the table rather than an estimate.
- Two `doctrine-lint` checks enforcing both, each positive-controlled against a violation
  seeded in the real corpus.
- A typed `CommandFailure` in `bundle-agent-context`, separating stale bundles from a run
  that could not complete, with distinct exit codes so CI need not parse the message.

### Changed in 0.8.0

- `RUST-DOC-0001` gates move from `## Gate N` headings to the shared table as `I01`–`I18`;
  `RUST-DOC-0002` prose names become `F01`–`F18` and `RUST-DOC-0003` become `O01`–`O18`.
  Each gate keeps its title as the lead-in of its question, in its original order. All 486
  gates are now citable by a stable identifier.
- `rationale.md` falls from 1,787 to 1,383 lines. Forty sections restating rules, types, or
  manifests are removed under `RUST-DOC-0011-R012`; rejected alternatives, accepted risks,
  and constraints no artifact holds are kept, and the per-file outcome is recorded in
  RFC-0007.
- `check_rule_enforcement` parses rule sections rather than tracking an open field with a
  boolean, which had made two contradictory states representable in the tool that rejects
  exactly that shape.
- Two corpus-wide tests that asserted absence now assert a non-zero observation first, so
  an empty file list can no longer satisfy them.
- `EVIDENCE.md` reports eighty tooling tests and names the three crates they span; the
  previous count was stale.

## [0.7.0] — 2026-08-08

Adds one normative rule. An assertion expecting nothing — an empty collection, a zero
count, an uncalled double — passes identically whether the condition was searched for
and absent or the search matched nothing. The corpus already governed the compile-fail
instance of that principle through `RUST-DOC-0008-R005` and gate `T18`; it governed the
runtime instance nowhere. Four recorded incidents, three of them in this repository,
show the runtime form passing while examining nothing. The corpus grows from 207
normative rules to 208, so this is a minor release for an added obligation. RFC-0006.

### Added in 0.7.0

- `RUST-DOC-0008-R022`, requiring a runtime absence assertion to establish that its
  predicate can observe the condition — through a self-validating predicate that fails
  when its subject is missing, a positive control asserted alongside it, or a paired
  assertion whose expected count is non-zero. Applicability is bounded to runtime
  assertions whose expected result is empty, so the rule cannot itself be applicable and
  vacuous; compile-fail evidence stays with `R005` and `T18`, and production telemetry
  stays with the existing anti-pattern.
- Gate `T61` in the RUST-DOC-0008 review standard, and a matching anti-pattern entry
  naming the shape: a zero count produced by a predicate that selected nothing.
- Three tests in the examples inventory crate, run against one registry that violates the
  invariant: the vacuous form passing on it, a positive control separating a blind
  predicate from a seeing one, and a non-zero pair naming the violation the vacuous form
  missed.

### Changed in 0.7.0

- RUST-DOC-0008 moves to `0.2.0`; no other doctrine version moves.

## [0.6.1] — 2026-08-08

Navigation became enforceable. Of 245 canonical Markdown files, 104 had no inbound link
from anywhere, because package indexes named their siblings in backticks rather than
links: prose that reads like a reference and navigates nowhere. Two remain, both
generator inputs exempted by name, and a `doctrine-lint` gate now fails the build if the
count rises. No normative rule changes, so this is a patch release. The generated
distributions change content rather than shape, and every file under `dist/agents/` is
byte-identical, because the material added lives in files no role pack carries.

### Added in 0.6.1

- A package-contents table in every doctrine package README, linking all seven sibling
  files. A table rather than links threaded through existing prose, because several
  packages never mention some siblings at all — RUST-DOC-0003 mentions none of its six
  non-normative siblings — so linkifying prose could not have reached them without
  inventing sentences.
- A situation-keyed selector in `doctrines/README.md`, mapping the work in front of a
  reader to a primary doctrine, its companions, and where that doctrine stops applying.
  It is marked informative and owned under `RUST-DOC-0011-R005` rather than called
  generated, because a generator would need a hand-written description of each package's
  scope. Its boundaries come from each package's own scope statement, and the two
  packages that name no successor say so instead of carrying an invented one.
- A reachability gate in `doctrine-lint`: maintained Markdown that nothing links to fails
  the build, with three exemptions that each state a reason. It walks the canonical roots
  plus `templates/`, `manifest/`, `tools/`, and `examples/`, assembled separately from
  `CANONICAL_ROOTS` so the normative-term scan and the drift checks keep their narrower
  set. It checks inbound links rather than reachability from the root, and says so.
- Mermaid rendering for branching structures: the decision trees in six doctrine
  packages, the registration-onboarding stage graph, and two case-study blocks. The
  doctrine template states the rule, which is that a branching structure renders and a
  linear sequence stays a text block, with `foundations/` and `agents/` excepted so
  generated packs stay byte-stable.

### Fixed in 0.6.1

- Root governance documents, directory indexes, per-doctrine source notes, RFC state
  directories, and the template files were unreachable by clicking. Each is now linked
  from the index that owns it.
- Five cells of the situation selector described boundaries their packages do not state,
  two of them contradicting the doctrine they cited: the error-modeling row implied
  ambiguous outcomes were out of scope, when `RUST-DOC-0002-R008` keeps them, and the
  distributed-uncertainty row treated a locally failed request as a handoff when it is
  one of that doctrine's four outcome variants. The underlying error was reading a
  package's stop conditions as handoffs; in this corpus they resolve inward, and the
  outward routing lives in each package's scope statement.

## [0.6.0] — 2026-08-06

A declared field that enforced nothing became real, and the corpus became navigable.
`manifest/agents.yaml` has always declared a `maximum_verbosity` per agent pack; the
bundler had never read it, and the measured pack sizes ran contrary to the order it
declared, with the pack labelled most focused thirty-five percent larger than one
labelled less focused. RFC-0005 gives the field an effect and reserves its widest
tier, which is what lets the corpus gain reading aids without any generated pack
growing. No normative rule changes, so this is a minor release for distribution
restructuring rather than for added obligations.

### Added in 0.6.0

- A verbosity annotation for a Markdown section, parsed once in `doctrine-manifest`
  and used by both tools. A generated output receives a section only when its ceiling
  is at least the annotation's tier, and every annotation is stripped from generated
  output. Comments inside fenced code are ignored, so a document can show the syntax
  without the example being read as an instruction.
- An `## Assembly` note in every filtered distribution, naming the ceiling applied,
  where it is declared, and how much it withheld. It is emitted even when nothing was
  withheld, so a reader can distinguish that from a forgotten disclosure. Each
  withheld run is replaced in place by a receipt naming the sections and their tiers.
- `doctrines/map.md`, a generated coverage map showing which doctrine each role pack
  hydrates. It is the transpose of six `doctrine_selections` lists, which the corpus
  carried but no document presented.
- Breadcrumb navigation on the fifty case-study and source-note leaves, which
  previously had no path to their own index, and live links throughout the root
  README reading paths and the package `Related material` sections.
- Instructions for loading the corpus into an agent, in `agents/distribution.md` and
  therefore in the generated `dist/README.md`, so the guidance travels with the
  bundles rather than living only in the repository. It covers which bundle to
  pick, how to attach it to a project instruction file, a system prompt or a
  retrieval index, and the three things an agent has to be told alongside it: the
  bundle is hydration and not authority, a role pack is a subset, and the corpus
  is versioned.
- A release workflow. A `v*` tag publishes the committed `dist/` bundles as a
  tarball, a zip, a checksum file, and the two whole-corpus bundles individually.
  It refuses to publish unless the tag matches `repository_version` and
  `bundle-agent-context -- check` confirms the bundles still match canonical
  source, so a stale commit cannot ship as though it were current. Asset names
  carry no version, because `releases/latest/download/<asset>` matches an exact
  filename and a versioned name would leave every documented download command
  pointing at a missing file one release later. This is the only workflow that is
  not read-only: `contents: write` is granted on the publishing job rather than
  the file, and `CONTRIBUTING.md` now says so instead of claiming CI is read-only
  throughout.
- A generated size table in `dist/README.md`, so choosing a bundle against a
  context window uses a current number rather than a hand-copied one. The index
  is built last and excludes itself from its own table, which is what makes the
  table a fixed point instead of a value that changes each time it is written.
- Orientation pages for twenty-one directories that held content and no index:
  each case study, each source package, `templates/`, `manifest/`, and
  `manifest/schema/`. They carry only what is not stated elsewhere — the
  subject, an ordered path through the directory, and a pointer to the single
  description of what each file holds — because the four-file arc and the
  provenance file roles are each already described once, and repeating them per
  directory is the duplication `RUST-DOC-0011-R017` asks a review to remove.
  None reaches a generated pack: no manifest references these directories, so
  they are reader-facing by location rather than by annotation.
- A closed callout vocabulary. An alert marks a distinction the corpus already draws
  rather than restating a claim, and `doctrine-lint` rejects any other alert name.

### Changed in 0.6.0

- The `auditor` pack moves from `exhaustive` to `detailed`. The widest tier is now
  reserved from every pack: a pack at that ceiling withholds nothing by definition, so
  it would absorb the whole of any canonical growth and get strictly larger.
- Double quotes are normalized to the corpus majority and eighteen em dashes gain the
  spacing the other two hundred and seventy-eight already use. Shell blocks under
  `agents/` are fenced as `bash` rather than `text`.

### Fixed in 0.6.0

- The release workflow interpolated its tag input directly into a shell block,
  in the one job that holds a write token. A `workflow_dispatch` input is free
  text, so its value could have closed the quoting and executed. The tag now
  reaches the shell through the environment, and a check rejects any character
  outside `v0-9.` before the shape is examined. Found by reviewing the change
  after writing it: the step immediately below already passed its values through
  `env:`, so the defect was an inconsistent application of the same fix. The
  first tightening was itself decorative and a control caught it — a `case`
  glob's `*` matches quotes and semicolons, so the shape pattern accepted a
  payload it appeared to reject until the alphabet check was added ahead of it.
- `EVIDENCE.md` and RFC-0005 both claimed eighteen tests govern the verbosity
  annotation. Twelve do; the other six predate it and check decoded vocabularies
  against the artifacts that own them. The tooling total was also one release
  behind at sixty-six against sixty-seven. Neither could be caught mechanically:
  `check_stated_counts` matches only a digit before three literal phrases, and
  both numbers are spelled out beside a phrase it does not track. That hole is
  recorded rather than closed.
- Agent overlays are obligation documents and are now untierable. An earlier
  classifier covered only doctrine normative files, the foundations, and the
  review checklists, so an annotation under `## Boundary obligations` in
  `agents/shared.md` was accepted by both tools and replaced those rules with a
  withholding receipt in every role pack. Because the widest tier is reserved
  from every pack, the obligation then reached none of them. Found in review.
  The consequence is stated rather than absorbed: every source any role pack
  lists now falls into an untierable class, so the ceiling removes nothing from
  a role pack today, and a test asserts that against the real manifests.
- The coverage map listed only active doctrines while `build_role_pack` hydrates
  any selected doctrine regardless of status, so a deprecated but selected
  doctrine would vanish from the map while the pack carrying it was unchanged.
  The map now covers every doctrine that is active or selected, and states each
  one's status.

- The per-doctrine rule counts in `EVIDENCE.md` were eleven machine-derivable integers
  maintained by hand with nothing checking them. `check_stated_counts` could not see
  them, because it only matches an integer immediately before one of three literal
  phrases and these sit alone in a table cell. They are recomputed and compared now.

## [0.5.0] — 2026-08-06

The corpus's own Rust code was audited against the corpus. The defects below are
the divergences that reproduced. Writing up the first of them exposed a defect in
the doctrine rather than the code: no rule could be cited against it. RFC-0004
amends `RUST-DOC-0001-R002` accordingly, which moves that doctrine to `0.2.0` and
makes this a minor release rather than a patch.

### Changed in 0.5.0

- `RUST-DOC-0001-R002` gains a second obligation. Its applicability named string
  discriminants while its statement reached only contradictory field combinations
  carrying state-specific data, so a lone `status` field over four unit states
  satisfied the applicability and the statement asked nothing of it. The rule was
  applicable and vacuous, which is why the manifest defect below could not be
  reported against it. A single field whose value selects among a closed, known
  set of mutually exclusive alternatives must now be decoded into a type that
  cannot hold a value outside that set. A new allowed exception routes a
  vocabulary too large or too volatile to enumerate to a validated newtype whose
  rejection of an unknown value is tested, so the rule does not require
  enumerating every constrained string.

  The rule keeps its identifier, title, and position, so existing citations
  resolve. No rule is added, removed, or weakened, and the corpus keeps 207
  normative rules. A system whose closed vocabulary reaches the domain as an
  unconstrained scalar was conforming before and is not conforming now, which is
  the minor version change. See `rfcs/accepted/RFC-0004-closed-vocabulary-discriminants.md`.

### Fixed in 0.5.0

- `bundle-agent-context` decoded the manifest `status` field as a `String` and
  never validated the manifest against `manifest/schema/doctrine.schema.json`,
  which constrains it to four values. A misspelled status therefore matched no
  filter and silently excluded that doctrine from `dist/full-doctrine.md`,
  `dist/compact-doctrine.md`, and every agent pack, while `generate` reported
  success and exited zero. Seeding `status: activ` on the first entry removed
  1901 lines from the bundles without a diagnostic. The vocabulary is now an
  enum decoded once, so the same input fails to parse in every consumer. This
  is `RUST-DOC-0001-R006`: deserialization must not bypass documented
  validation.
- `doctrine-lint` restated two schema-owned vocabularies in Rust: the six agent
  role identifiers and the four `maximum_verbosity` values, each a hand-written
  copy of an `enum` array in `manifest/schema/agent-pack.schema.json` that the
  same run had already validated. Both copies are removed; the values decode
  into types, and three tests assert those types against the schema files, so a
  vocabulary change fails the build instead of drifting. This is
  `RUST-DOC-0011-R017` applied to the tool that enforces it elsewhere.
- The manifest types, the status vocabularies, and the front-matter parser were
  each declared twice, once per tool, because the two binaries shared no
  library. The two `front_matter` implementations had already diverged, and the
  copy specialized to say "README" was being used to report malformed decision
  records. All of it now lives in a new `doctrine-manifest` crate.
- `doctrine-lint` treated a directory it could not read as an empty directory.
  An unreadable repository root would have scanned no root documents and let
  every root-document check pass while still reporting the repository valid.
  Directory walks now report a directory that exists and cannot be
  enumerated; an absent directory stays silent, because absence is genuinely
  empty and is already reported against its manifest entry.
- `bundle-agent-context` validated its command argument and then matched the
  raw string a second time, needing an `unreachable!` arm to restate a
  guarantee the first check had established. The argument is parsed into a
  `Command` and the dispatch is exhaustive.
- `unsafe-evidence` declared its escape hatch from the workspace's
  `unsafe_code = "forbid"` twice, in `Cargo.toml` and as an inner attribute.
  The manifest declaration is kept and named as the scoped exception under
  `RUST-DOC-0001-R016`.
- `RfcMetadata.status` was the one closed vocabulary still decoded as a string
  after the sweep above. It is now a type, checked by test against the `rfcs/`
  state directories that own the lifecycle. This is the amended
  `RUST-DOC-0001-R002` applied to the corpus's own code, so the repository
  conforms to the rule it publishes in the same release that publishes it.
- `doctrine-lint` skipped a file it could not read during the repository-wide
  forbidden-marker scan. Only directory-enumeration failures were reported, so a
  single unreadable file left `check` exiting zero and calling the repository
  valid. The read is now reported. Content that is not UTF-8 stays silent, since
  the markers are Markdown filler and a binary file carries none.
- `bundle-agent-context` turned a directory it could not enumerate into an empty
  list, and discarded per-entry read failures. With `rfcs/accepted` unlistable,
  `generate` rewrote the accepted-RFC index from its three rows to "no accepted
  RFCs" and exited zero. The walk is now fallible and aborts generation. This is
  the same class as the `doctrine-lint` fix above, which had been applied to only
  one of the two tools.
- Both tools classified directory entries with `Path::is_dir` and `Path::is_file`,
  which follow symbolic links and report `false` for a metadata error. A link was
  therefore treated as whatever it pointed at, and a classification failure was
  indistinguishable from an entry that was neither. A symbolic link added under
  `rfcs/accepted` made `generate` publish a second index row for a target the
  repository does not contain, and exit zero. Entries are now classified with
  `DirEntry::file_type`, a link is reported and not followed, and a classification
  failure is reported. The linter also reported no per-entry read failure, which
  `filter_map(Result::ok)` discarded; those are reported too.
- `doctrines/0001-invalid-states/README.md` stated "All rules are version `0.1.0`
  and active", which the `R002` amendment made false and which bundle generation
  propagated into `dist/full-doctrine.md`. The sentence restated a version the
  file's own front matter and the manifest already carry, so it is replaced with
  the stable-identifier wording the other doctrine READMEs use, which names no
  version and cannot go stale. That is `RUST-DOC-0011-R004` applied to a package
  README.
- `rfcs/accepted/RFC-0004-closed-vocabulary-discriminants.md` carried a prose
  paragraph where `rfcs/template.md` requires a decision record with decision,
  date, decision owners, rationale, conditions, and supersession. The record now
  states all six, so the acceptance authority for a normative change is on the
  record rather than implied.

## [0.4.1] — 2026-08-05

Documentation coherency only. No normative rule, statement, allowed exception,
or doctrine version changes.

### Fixed in 0.4.1

- The root `README.md` doctrine index listed nine doctrines while claiming to be
  synchronized with `manifest/doctrines.yaml`, which by then held eleven. That
  table was a third representation of a fact the manifest owns and
  `doctrines/README.md` already presented, and it also propagated into
  `dist/full-doctrine.md`. The duplicate is removed; the root README now links to
  the doctrine index instead of repeating it.
- `doctrine-lint` now checks the surviving doctrine index against the manifest:
  every active doctrine needs a row naming its identifier and its exact manifest
  title on one line, and the index may not list a doctrine the manifest does not
  carry as active. A prose cross-reference does not satisfy the row requirement,
  which is how the shipped index concealed a missing row.
- The root `README.md` claimed repository version `0.2.0`. It now names no
  version number at all and points at the manifest, so the same defect cannot
  recur there.
- `doctrines/README.md` said the repository release and every doctrine were at
  `0.1.0`; repository and doctrine versions have since diverged.
- The generated-file set was described as `dist/` alone in `README.md`,
  `CONTRIBUTING.md`, `agents/shared.md`, and the pull-request template, although
  `rfcs/accepted/README.md` became generated in 0.4.0. All four now name both,
  and the documented drift-gate command covers both paths.
- `decisions/` was absent from the canonical-root list and the architecture
  section of `README.md`.
- The reading paths in `README.md` did not mention RUST-DOC-0011, the executable
  narrative review, or the maintainer's decision-record obligations.
- `doctrines/0004-concurrency-and-async/README.md` referred to "the 0.1.0
  workspace".
- `EVIDENCE.md` was absent from the linter's root-document list, so it had never
  been scanned for normative-term scope, and the new counted-claim check silently
  skipped the corpus document carrying the most derived numbers. Every Markdown
  file at the repository root is now scanned.

### Added in 0.4.1

- A counted-claim check. Prose stating a number of normative rules, doctrine
  packages, or active doctrines is compared against the recomputed corpus, so a
  hand-maintained number cannot silently disagree with the manifest.
- A rule-citation check. Every `RUST-DOC-NNNN-RNNN` cited in maintained canonical
  Markdown must resolve to a rule a doctrine defines. The corpus already required
  each rule to appear in its own review standard; nothing checked the reverse, so
  a renamed or removed rule left dangling citations that read as authoritative.
- Both checks exempt dated records, which state the contract as it stood when a
  decision was taken and are not maintained afterwards, as
  `RUST-DOC-0011-R011` and `RUST-DOC-0011-R019` permit. Rewriting them to satisfy
  a linter would destroy the record. The exemption follows artifact lifecycle
  rather than directory membership: the RFC documents themselves in any lifecycle
  state, archived decision records, and `CHANGELOG.md`. Maintained governance
  around them stays scanned, including `rfcs/README.md`, the state-directory
  READMEs, and `rfcs/accepted/overview.md`, which cites live rule identifiers.
- A duplicated-validation-sequence check. The local validation sequence was
  carried in full by `README.md`, `AGENTS.md`, and the pull-request template at
  once, so a change to any gate had to be made in three places correctly with
  nothing announcing a miss. `README.md` now owns it, the other two link to it,
  and `doctrine-lint` counts fenced blocks carrying three or more validation
  commands and requires exactly one, in the owner.
- Root Markdown files are discovered from the directory rather than listed in a
  constant, so a future root document is covered on the day it is added rather
  than when someone remembers the inventory.

## [0.4.0] — 2026-08-04

### Changed in 0.4.0

- Restated RUST-DOC-0010-R022. It keeps its identifier and its non-waivable
  status, and now states an authority partition rather than a blanket
  governance precedence: the executable protocol is authoritative for the
  ordering, successor constraints, construction restrictions, and negative
  capabilities it mechanically enforces; external systems are authoritative for
  durable and remote facts; governing records are authoritative for rationale,
  non-guarantees, waivers, and change authority; and no artifact is maintained
  as a competing source for another's class of claim. The previous text
  contradicted RUST-DOC-0010-R018 and RUST-DOC-0010-R019 in the same package.
  RUST-DOC-0010 advances to 0.2.0.
- Corrected the source provenance for RUST-DOC-0010. The earlier notes recorded
  the originating document as claiming "code as sufficient contract" and
  rejected it; the document had made no such claim. The corrected
  classification records executable authority as accepted and refined, records
  the source's stronger last-resort stance on decision records, and states the
  earlier error in place rather than deleting it.
- Made CT³ visible where the mechanism is taught. `patterns/successor-capabilities.md`
  now states the local name and explains what each of its words carries,
  including the containment that "telescopic" names; the package glossary entry
  carries the same explanation. The term remains local vocabulary with its
  established families attributed, per RUST-DOC-0010-R021.
- Added the authority-partition gates S58, S61, and S62 to the RUST-DOC-0010
  review standard, and a second failure direction to its rationale and
  anti-pattern catalogue.
- Selected RUST-DOC-0011 for every agent pack, added the new review procedure to
  five of them, and gave each role file its obligation-placement section.

### Added in 0.4.0

- RUST-DOC-0011 "Executable Narrative and Minimal Decision Records", an
  eleventh doctrine governing where an architectural obligation lives, which
  artifact settles which class of claim, how a derived view is kept from
  drifting, and when a manually maintained decision record earns its permanent
  cost. It adds twenty rules covering claim classification, executable
  authority, the prohibition on a competing manually maintained copy,
  generation over synchronization, the decision-record last-resort test and
  lifecycle, the historical-veto prohibition, rationale honesty, external
  authority, representation counting, and agent hydration.
- A forty-gate review standard for the new doctrine, and the
  `reviews/executable-narrative-review.md` procedure.
- The `patterns/executable-narrative.md` pattern guide.
- A `decisions/` tree holding the record template and two worked examples: one
  decision whose residue justifies a narrow record, and one that should not be
  written at all. Both are illustrations and are absent from the registry.
- `manifest/decision-records.yaml` and its Draft 2020-12 schema, enumerating the
  active and archived record sets. The active set is empty, which is the outcome
  RUST-DOC-0011-R006 predicts for this repository.
- Six `doctrine-lint` checks for the registry, each with a unit test: owner,
  revalidation trigger, obsolescence condition and resolvable executable
  authorities on an active record; the directory each status is filed under;
  identifier uniqueness; the archival marking; and the prohibition on an agent
  pack hydrating an archived record. Each check was observed failing on a seeded
  violation before being accepted as evidence.
- RFC-0003 recording the accepted decision, including the previous position, the
  corrected position, and the reason the earlier wording was wrong.
- A generated accepted-RFC index. `rfcs/accepted/README.md` is now produced by
  `bundle-agent-context` from `rfcs/accepted/overview.md` and each RFC's own
  front matter, and is drift-checked; it had omitted RFC-0002 while
  hand-maintained.

### Fixed before release in 0.4.0

Review of the initial revision found three defects. All were corrected before
merge:

- The decision-record registry duplicated the metadata each record already
  carries in its front matter, and the linter validated only the registry row.
  A record could omit or contradict its own owner, scope, authorities, or
  triggers while the build stayed green, which both overstated the guarantee and
  created the competing copy RUST-DOC-0011-R004 prohibits, inside the
  enforcement for that rule. The registry now enumerates membership only; the
  linter opens each listed record and validates RUST-DOC-0011-R007 from its
  front matter. The one overlapping fact, list membership versus declared
  `status`, is compared rather than trusted, and a mismatch test covers it.
- The accepted-RFC index was left hand-maintained on the stated ground that a
  generator would need another hand-written list. That reason was wrong: the
  directory supplies the file set and each RFC's front matter supplies its
  identifier, title, and status, so the view is generated with no competing
  source.
- RFC-0003 said ten existing rule statements were preserved. RUST-DOC-0010 has
  twenty-two rules and only R022 changes, so twenty-one are preserved in that
  package and 165 in the nine others.

## [0.3.0] — 2026-08-04

### Added in 0.3.0

- RUST-DOC-0010 "Staged Protocols and Successor Capabilities", a tenth doctrine
  governing in-process multi-stage protocols whose capabilities expose their
  legal successor as an associated type bounded by the next capability. It adds
  twenty-two rules covering successor bounds, branch and recovery edges,
  per-stage failure identity, construction bypass, effect disclosure, stage
  granularity, erasure boundaries, and the limit at which a local transition
  stops being evidence of a durable one.
- A fifty-eight gate review standard for the new doctrine, and a
  staged-protocol gate group in the typestate review procedure.
- The `patterns/successor-capabilities.md` pattern guide.
- A `registration-onboarding` case study covering two entry paths, an expiring
  availability observation, branch and recovery edges, and the durable limit.
- The `staged-protocol` example crate with ten unit tests and an executable
  topology assertion that pins every documented protocol edge, plus three
  compiler-rejection cases for stage skipping, consumed-stage reuse, and stage
  evidence forgery.
- RFC-0002 recording the accepted decision to add the doctrine.

### Changed in 0.3.0

- Selected RUST-DOC-0010 for the planner, implementer, reviewer, and auditor
  agent packs, and cross-referenced the new pattern from RUST-DOC-0001,
  RUST-DOC-0003, RUST-DOC-0004, RUST-DOC-0005, and RUST-DOC-0006. Their
  normative text is unchanged.
- Updated the repository evidence map for 187 normative rules, 40 unit tests,
  and nine compiler-rejection cases, and recorded which of the new rules have
  no executable evidence in this repository.

### Fixed before release in 0.3.0

Review of the initial RUST-DOC-0010 revision found five defects in the shipped
package. All were corrected before merge:

- `RUST-DOC-0010-R005` now makes non-duplicability part of the consumption
  obligation. Nonterminal stages previously derived `Clone`, so a caller could
  copy a stage and advance every copy while every consuming signature remained
  satisfied. Added a compiler-rejection case for duplication and gate S59.
- `RUST-DOC-0010-R019` now requires a contract assertion that derives the
  successor capability from the stage capability alone. The original assertions
  restated the bound themselves, so deleting `type Next: CheckIdentity` from a
  capability left the suite green.
- `RUST-DOC-0010-R007` now applies to fallible transitions only, and forbids
  declaring a failure type that is never constructed. `prepare_persistence` is
  infallible and returns its successor directly, consistent with
  RUST-DOC-0001-R013. Added gate S60.
- The conflict-resolution edge preserves origin evidence. A revised submission
  is now parameterized by the original origin, so an invited attempt stays
  invited instead of being reported as self-service under a fabricated
  challenge identifier.
- The registration case study carries a guarantee-ledger row per stage, as
  `RUST-DOC-0010-R020` and gate S55 require, including duplicability and the
  recovery stages.

## [0.2.0] — 2026-07-27

### Changed in 0.2.0

- Strengthened RUST-DOC-0005-R009 so concurrency designs name the anomaly class
  they prevent and every residual anomaly class they permit; added isolation
  terminology, a mechanism map, and a critical review gate.
- Strengthened RUST-DOC-0006-R014 so time-based authority defines its clock
  source, clock kind, timing bounds, and behavior when assumptions fail.
- Clarified RUST-DOC-0004-R004 coverage of poisoning and non-poisoning lock
  APIs, including the pinned toolchain's experimental `std::sync::nonpoison`
  module.
- Documented the deliberate expanded-versus-tabular review procedure policy.

### Added in 0.2.0

- A repository-level evidence map relating all 165 normative rules to current
  executable evidence classes and known gaps.
- An isolated, panic-safe `MaybeUninit` array initializer with a local unsafe
  lint exception, five unit tests, a safety argument, and pinned-nightly Miri
  CI.
- RFC-0001 recording the accepted normative isolation and time-assumption
  changes.

## [0.1.0] — Initial doctrine corpus

### Added

- Shared foundations for normative language, invariant classification, evidence levels, trust
  boundaries, guarantee honesty, and type-system complexity budgeting.
- Nine active doctrine packages: invalid states, error modeling, ownership and capabilities,
  concurrency and async, persistence boundaries, distributed uncertainty, unsafe Rust,
  testing and evidence, and performance measurement.
- Pattern guides for sum types, opaque newtypes, smart constructors, typestate, capabilities,
  consuming transitions, validated collections, hybrid state machines, and explicit
  uncertainty.
- Boundary guides for Serde, database decoding, HTTP/RPC, messaging, configuration,
  filesystems, and FFI.
- Operational review procedures covering design preparation, domain models, boundaries,
  typestate, distributed effects, and final correctness audits.
- Shared, planner, implementer, reviewer, auditor, and maintainer agent role contracts with
  deterministic generated hydration packs.
- End-to-end invoice, payment, transaction, message delivery, authenticated session, and UI
  workflow case studies.
- Stable Rust examples for domain modeling, validated newtypes, typestate, boundary
  validation, and distributed outcomes, plus compiler-rejection tests.
- Machine-readable doctrine and agent manifests with Draft 2020-12 schemas.
- A substantive doctrine linter, deterministic bundle generator, RFC governance, reusable
  authoring templates, and source-provenance packages.
- Generated full, compact, and role-specific distributions with drift detection.
- Read-only GitHub workflows for doctrine validation, stable/MSRV example testing, and
  Markdown formatting, linting, and link checking; structured issue and pull-request forms.
- Reproducible Markdown quality tooling with pinned Node.js, Prettier, and markdownlint-cli2
  versions, a patched transitive override, deterministic formatting policy, advisory audit,
  and a distinct pull-request gate.
- Dual licensing: CC BY 4.0 for documentation and MIT OR Apache-2.0 for code.
