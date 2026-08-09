# Repository evidence map

This inventory describes executable evidence shipped with repository version
0.9.0. It is a claim ledger, not a coverage percentage. The 208 normative rules
define review obligations; no test count implies one test per rule or universal
proof.

## Executable baseline

- 40 ordinary unit tests exercise the doctrine example crates.
- Nine `trybuild` UI cases preserve selected compiler rejections.
- One inventory test keeps example directories, package names, and workspace
  membership aligned.
- Ninety-three tooling tests across `doctrine-lint`, `doctrine-manifest`, and
  `bundle-agent-context` exercise doctrine linting, decision-record validation,
  doctrine-index agreement with the manifest, counted-claim, rule-citation and
  duplicated-validation-sequence drift detection, per-rule enforcement and
  per-gate check declarations, and deterministic bundle generation.
- Twelve of those hold the repository's own connectivity. Four seed a violation
  and require the matching diagnostic: a workspace crate no document links, a
  crate or prose directory with no index, a non-Markdown file no document names,
  and drift in either direction between the workspace members and the workflow
  that tests them. Four pin the parsers those checks depend on: that a mention
  names a whole file rather than the tail of a longer one, that the walk honours
  the repository's own ignore patterns, that the membership parser stops at the
  array and skips comments, and that the workflow package list is read from the
  test step alone. Four control the checks themselves rather than their subject:
  that the walk reaches [`.github/`](.github/README.md), that every register
  entry states a reason, that every registered path still exists, and that the
  membership parser reads either array shape. A register entry for a deleted
  file silences nothing and conceals that the register is stale.
- Twelve of those govern the verbosity annotation that decides which sections a
  generated pack receives: each malformation Prettier preserves unchanged, a
  tilde fence closed by inner backticks, non-monotone nesting, idempotence, and
  the rejection of every tier in a file that states obligations. The other six
  tests in that crate predate the annotation and check decoded vocabularies
  against the artifacts that own them. One bundler test asserts that link
  targets are still validated inside a section the ceiling withholds, so link
  checking does not depend on what any pack receives, and one linter test
  asserts against the real manifests that no source a role pack lists can be
  withheld from it.
- Four of those assert the vocabularies decoded by `doctrine-manifest` against
  the artifacts that own them: the `enum` arrays in the JSON Schemas, and the
  [`rfcs/`](rfcs/) state directories. A value added to one without a matching Rust
  variant fails the build rather than failing to parse in a tool.
- Five more assert that a directory or file which exists and cannot be read is
  reported rather than skipped, and that an absent directory and content that is
  not UTF-8 are not reported, so the checks cannot be satisfied by reporting
  every path that fails to open.
- Two more assert that a symbolic link is reported and not followed, in both the
  linter's walk and the bundler's, so an entry naming a target outside the
  repository cannot reach a scan or a generated bundle.
- A dedicated CI job reruns the five `unsafe-evidence` unit tests under Miri on
  pinned nightly `nightly-2026-07-13`.

The ordinary workspace suite runs on pinned Rust 1.97.1 and MSRV 1.85.0. Miri
requires nightly and supplements rather than replaces the safety argument.

## Doctrine-to-evidence map

| Doctrine                                                              | Rules | Current executable evidence                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Strongest evidence class                       | Material gap                                                                                                                                                              |
| --------------------------------------------------------------------- | ----: | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [RUST-DOC-0001](doctrines/0001-invalid-states/README.md)              |    22 | `domain-modeling`, `validated-newtypes`, typestate cases, and five compiler-rejection cases exercise private construction, checked values, sum states, and illegal sequencing                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | unit plus compile-fail                         | no exhaustive constructor-path proof, deployed boundary, or mutable-external-fact test                                                                                    |
| [RUST-DOC-0002](doctrines/0002-error-modeling/README.md)              |    14 | example errors preserve invalid input, transition failure, ambiguity, and caller retry decisions                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | deterministic unit                             | no real transport, panic-boundary integration, error-redaction audit, or operator workflow                                                                                |
| [RUST-DOC-0003](doctrines/0003-ownership-and-capabilities/README.md)  |    14 | consuming transaction and connection examples plus compiler rejection exercise selected custody and sequencing claims                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | unit plus compile-fail                         | no revocation backend, secret-handling target, FFI authority transfer, or cross-process capability test                                                                   |
| [RUST-DOC-0004](doctrines/0004-concurrency-and-async/README.md)       |    20 | local lifecycle examples provide only indirect evidence for consuming operations and failures                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | deterministic unit                             | no async runtime, cancellation injection, schedule exploration, lock model, backpressure stress, or shutdown integration                                                  |
| [RUST-DOC-0005](doctrines/0005-persistence-boundaries/README.md)      |    17 | `boundary-validation` exercises raw-row conversion, rejection, and trusted constructor reuse                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | deterministic unit                             | no real database, isolation anomaly, migration, constraint, restore, outbox, or ambiguous-commit integration                                                              |
| [RUST-DOC-0006](doctrines/0006-distributed-uncertainty/README.md)     |    19 | `distributed-outcomes` exercises explicit outcome states, stable identity, and identity-preserving retry decisions                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | deterministic unit                             | no network, broker, lease/fencing backend, clock-fault injection, crash matrix, or live reconciliation                                                                    |
| [RUST-DOC-0007](doctrines/0007-unsafe-rust/README.md)                 |    18 | `unsafe-evidence` exercises a documented `MaybeUninit` abstraction on success, error, panic, zero length, and zero-sized elements; CI also runs those cases under Miri                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | unit plus interpreter                          | no sanitizer, fuzzing, FFI target, unsafe concurrency, allocator boundary, or architecture matrix                                                                         |
| [RUST-DOC-0008](doctrines/0008-testing-and-evidence/README.md)        |    22 | the full example suite supplies positive, negative, deterministic, compile-fail, boundary, and Miri evidence classes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | layered local suite                            | no property testing, fault injection, model checking, deployed contract test, mutation testing, or production telemetry                                                   |
| [RUST-DOC-0009](doctrines/0009-performance-and-measurement/README.md) |    20 | no benchmark or profiling target is shipped; ordinary tests only protect functional prerequisites                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | functional unit only                           | no workload, benchmark, profiler capture, allocation evidence, statistical comparison, or regression budget                                                               |
| [RUST-DOC-0010](doctrines/0010-staged-protocols/README.md)            |    22 | `staged-protocol` exercises two entry paths, both branches, both recovery edges, an invited revision carried through to the terminal stage, undetermined availability, and stale consent; contract assertions derive each successor capability from its trait alone and edge assertions pin every concrete edge; four compiler-rejection cases cover stage skipping, consumed-stage reuse, stage duplication, and evidence forgery                                                                                                                                                                                                                                                                                | unit plus compile-fail plus contract assertion | no database, competing writer, restoration path, async cancellation, or durable-advancement test; R014, R015, and R016 have no executable evidence here                   |
| [RUST-DOC-0011](doctrines/0011-executable-narrative/README.md)        |    20 | six `doctrine-lint` tests validate the decision-record registry: owner, revalidation trigger, obsolescence condition, and executable authorities on an active record, the directory each status is filed under, identifier uniqueness, the archival marking, and the prohibition on an agent pack hydrating an archived record; the repository's own generated bundles and their drift check demonstrate the generation mechanism; four `doctrine-manifest` tests check each closed vocabulary against the artifact that owns it, three against JSON Schema `enum` arrays and one against the `rfcs/` state directories, which is the mechanical check R004 requires of a view derived from an enforcing artifact | tool unit plus generated-drift detection       | most rules are judgment obligations no linter can decide; R002, R003, R012, R013, R016, and R017 have no executable evidence here, and the registry's active set is empty |

## Interpretation limits

Example crates deliberately avoid live network and database dependencies, so
their results are deterministic and narrow. A unit test proves only the
behavior exercised under its inputs. A compiler-rejection fixture proves the
selected program remains rejected at the pinned diagnostic boundary. A contract
assertion proves a capability still declares its successor bound; an edge
assertion proves the concrete successor is unchanged. Neither proves the graph is
the right graph for a domain, which remains a review judgment. An edge assertion
alone proves less than it appears to: if its own bounds restate the trait's
obligation, it stays green after that obligation is deleted. Miri
interprets the exercised unsafe paths under one toolchain and configuration; it
does not prove soundness for all executions.

The decision-record checks were observed failing on seeded violations before they
were accepted as evidence, and observed passing on the corrected forms. They
prove that a malformed or misfiled record fails the build; they cannot decide
whether a record should exist, whether its stated justification is honest, or
whether an obligation left in prose could have been carried by a type. The
registry's active set is empty, which is a fact about the record set and not
about the constraints this repository is under. No protocol-graph generator is
shipped: a generator fed by a hand-written edge list would be the second
maintained representation RUST-DOC-0011 prohibits, so the graph obligation is
carried by the contract and edge assertions instead.

When evidence changes, update this file from observed test inventory and tool
results. Add the narrowest evidence class that matches the claim, retain known
gaps, and never promote prose, generated bundles, or green compilation into
executable evidence.
