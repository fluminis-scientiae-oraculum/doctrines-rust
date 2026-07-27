# Repository evidence map

This inventory describes executable evidence shipped with repository version
0.2.0. It is a claim ledger, not a coverage percentage. The 165 normative rules
define review obligations; no test count implies one test per rule or universal
proof.

## Executable baseline

- 28 ordinary unit tests exercise the doctrine example crates.
- Five `trybuild` UI cases preserve selected compiler rejections.
- One inventory test keeps example directories, package names, and workspace
  membership aligned.
- Thirteen tooling tests exercise doctrine linting and deterministic bundle
  generation.
- A dedicated CI job reruns the five `unsafe-evidence` unit tests under Miri on
  pinned nightly `nightly-2026-07-13`.

The ordinary workspace suite runs on pinned Rust 1.97.1 and MSRV 1.85.0. Miri
requires nightly and supplements rather than replaces the safety argument.

## Doctrine-to-evidence map

| Doctrine                                                              | Rules | Current executable evidence                                                                                                                                                   | Strongest evidence class | Material gap                                                                                                             |
| --------------------------------------------------------------------- | ----: | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------ | ------------------------------------------------------------------------------------------------------------------------ |
| [RUST-DOC-0001](doctrines/0001-invalid-states/README.md)              |    22 | `domain-modeling`, `validated-newtypes`, typestate cases, and five compiler-rejection cases exercise private construction, checked values, sum states, and illegal sequencing | unit plus compile-fail   | no exhaustive constructor-path proof, deployed boundary, or mutable-external-fact test                                   |
| [RUST-DOC-0002](doctrines/0002-error-modeling/README.md)              |    14 | example errors preserve invalid input, transition failure, ambiguity, and caller retry decisions                                                                              | deterministic unit       | no real transport, panic-boundary integration, error-redaction audit, or operator workflow                               |
| [RUST-DOC-0003](doctrines/0003-ownership-and-capabilities/README.md)  |    14 | consuming transaction and connection examples plus compiler rejection exercise selected custody and sequencing claims                                                         | unit plus compile-fail   | no revocation backend, secret-handling target, FFI authority transfer, or cross-process capability test                  |
| [RUST-DOC-0004](doctrines/0004-concurrency-and-async/README.md)       |    20 | local lifecycle examples provide only indirect evidence for consuming operations and failures                                                                                 | deterministic unit       | no async runtime, cancellation injection, schedule exploration, lock model, backpressure stress, or shutdown integration |
| [RUST-DOC-0005](doctrines/0005-persistence-boundaries/README.md)      |    17 | `boundary-validation` exercises raw-row conversion, rejection, and trusted constructor reuse                                                                                  | deterministic unit       | no real database, isolation anomaly, migration, constraint, restore, outbox, or ambiguous-commit integration             |
| [RUST-DOC-0006](doctrines/0006-distributed-uncertainty/README.md)     |    19 | `distributed-outcomes` exercises explicit outcome states, stable identity, and identity-preserving retry decisions                                                            | deterministic unit       | no network, broker, lease/fencing backend, clock-fault injection, crash matrix, or live reconciliation                   |
| [RUST-DOC-0007](doctrines/0007-unsafe-rust/README.md)                 |    18 | `unsafe-evidence` exercises a documented `MaybeUninit` abstraction on success, error, panic, zero length, and zero-sized elements; CI also runs those cases under Miri        | unit plus interpreter    | no sanitizer, fuzzing, FFI target, unsafe concurrency, allocator boundary, or architecture matrix                        |
| [RUST-DOC-0008](doctrines/0008-testing-and-evidence/README.md)        |    21 | the full example suite supplies positive, negative, deterministic, compile-fail, boundary, and Miri evidence classes                                                          | layered local suite      | no property testing, fault injection, model checking, deployed contract test, mutation testing, or production telemetry  |
| [RUST-DOC-0009](doctrines/0009-performance-and-measurement/README.md) |    20 | no benchmark or profiling target is shipped; ordinary tests only protect functional prerequisites                                                                             | functional unit only     | no workload, benchmark, profiler capture, allocation evidence, statistical comparison, or regression budget              |

## Interpretation limits

Example crates deliberately avoid live network and database dependencies, so
their results are deterministic and narrow. A unit test proves only the
behavior exercised under its inputs. A compiler-rejection fixture proves the
selected program remains rejected at the pinned diagnostic boundary. Miri
interprets the exercised unsafe paths under one toolchain and configuration; it
does not prove soundness for all executions.

When evidence changes, update this file from observed test inventory and tool
results. Add the narrowest evidence class that matches the claim, retain known
gaps, and never promote prose, generated bundles, or green compilation into
executable evidence.
