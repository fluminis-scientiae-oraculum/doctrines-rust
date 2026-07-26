# Review standard

Record **pass**, **fail**, **not applicable**, or an approved **waiver
reference** for every gate.

| Gate | Question                                            | Pass evidence               | Failure example                    | Severity | Remediation           |
| ---- | --------------------------------------------------- | --------------------------- | ---------------------------------- | -------- | --------------------- |
| T01  | Does each test map to a claim?                      | invariant/risk reference    | incidental method call test        | high     | name purpose          |
| T02  | Does every critical invariant have evidence?        | evidence matrix             | authority invariant untested       | critical | add layer             |
| T03  | Are evidence limits stated?                         | scope/non-proof column      | passing tests called proof         | high     | qualify               |
| T04  | Are valid constructor cases tested?                 | representative table        | no acceptance test                 | medium   | add                   |
| T05  | Are invalid constructor cases tested?               | boundary rejection          | only happy path                    | high     | add negatives         |
| T06  | Are exact bounds tested?                            | below/at/above cases        | only middle value                  | high     | add boundaries        |
| T07  | Are structured errors asserted?                     | category assertions         | only `is_err()`                    | medium   | inspect category      |
| T08  | Is normalization tested?                            | idempotence/collision cases | silent data change                 | high     | add properties        |
| T09  | Are Unicode/encoding risks represented?             | fixtures                    | ASCII-only parser tests            | high     | broaden domain        |
| T10  | Are size/resource limits tested?                    | oversized input             | decoder limit unexercised          | high     | add adversarial case  |
| T11  | Is property appropriate?                            | domain-level statement      | implementation restatement         | high     | define independently  |
| T12  | Does generator cover relevant partitions?           | distribution analysis       | invalid cases filtered out         | high     | improve generator     |
| T13  | Are failing seeds reproducible?                     | seed capture                | random CI failure irreproducible   | high     | persist seed          |
| T14  | Is shrink result interpretable?                     | minimal case                | huge opaque failure                | medium   | tune strategy         |
| T15  | Is oracle independent enough?                       | model/spec comparison       | encoder tests itself               | high     | add oracle            |
| T16  | Are prohibited APIs compile-tested?                 | UI cases                    | privacy only assumed               | high     | add compile-fail      |
| T17  | Is each compile-fail source minimal?                | one prohibition             | unrelated errors                   | high     | simplify              |
| T18  | Does diagnostic fail for intended reason?           | semantic inspection         | missing import causes pass         | critical | repair fixture        |
| T19  | Was `.stderr` change reviewed?                      | focused diff rationale      | overwrite accepted blindly         | critical | inspect               |
| T20  | Is pinned compiler used for UI evidence?            | toolchain config            | diagnostics vary unnoticed         | high     | pin                   |
| T21  | Are real codecs exercised?                          | serialization integration   | hand-built domain only             | high     | cross boundary        |
| T22  | Is real database behavior exercised where needed?   | integration setup           | in-memory map stands for isolation | critical | add DB test           |
| T23  | Are migrations tested from old fixtures?            | version fixtures            | fresh schema only                  | high     | migrate               |
| T24  | Are protocol contracts versioned?                   | compatibility tests         | current pair only                  | high     | add matrix            |
| T25  | Are unknown fields/variants tested?                 | forward cases               | decoder panics                     | high     | add                   |
| T26  | Is authentication/authorization boundary tested?    | separate outcomes           | mock principal injected            | critical | cross real adapter    |
| T27  | Do doubles preserve relevant failures?              | fidelity table              | remote never times out             | critical | improve double        |
| T28  | Are double gaps covered elsewhere?                  | suite reference             | undocumented omission              | high     | assign owner          |
| T29  | Does concurrency test use explicit synchronization? | barrier/event/model         | sleeps establish order             | critical | control schedule      |
| T30  | Is cancellation tested at partial steps?            | cancellation matrix         | only before start                  | critical | inject at awaits      |
| T31  | Is cleanup asserted after cancellation?             | resource counts/state       | task drop assumed enough           | high     | inspect postcondition |
| T32  | Are lock/channel closures tested?                   | owner-drop cases            | unwrap closure                     | high     | add                   |
| T33  | Is shutdown tested while loaded?                    | drain/deadline case         | idle-only shutdown                 | high     | add outstanding work  |
| T34  | Is model abstraction documented?                    | production/model map        | Loom model differs silently        | high     | explain gaps          |
| T35  | Are model bounds sufficient for claim?              | bound rationale             | one trivial step                   | high     | expand/narrow claim   |
| T36  | Are partial durable failures injected?              | crash-point matrix          | error only before write            | critical | inject between steps  |
| T37  | Are duplicate deliveries tested?                    | repeated identity case      | broker double once-only            | critical | add replay            |
| T38  | Are delayed acknowledgements tested?                | effect-before-loss case     | timeout only pre-dispatch          | critical | inject loss           |
| T39  | Are reorderings tested?                             | version/out-of-order cases  | global FIFO assumed                | high     | add sequences         |
| T40  | Does unknown remain unknown?                        | outcome assertion           | timeout collapsed                  | critical | preserve state        |
| T41  | Is reconciliation tested repeatedly?                | still-unknown then terminal | one query only                     | high     | model lifecycle       |
| T42  | Are retries bounded in tests?                       | virtual-time budget         | test can loop forever              | high     | cap                   |
| T43  | Do snapshots exclude nondeterministic noise?        | normalization policy        | changing timestamps                | medium   | stabilize             |
| T44  | Are snapshot changes semantically explained?        | review note                 | bulk approval                      | critical | classify diffs        |
| T45  | Are golden fixtures sourced and versioned?          | provenance                  | unexplained blob                   | medium   | document              |
| T46  | Are flaky signatures retained?                      | issue/log evidence          | rerun erases failure               | high     | capture first         |
| T47  | Is retry temporary and visible?                     | owner/expiry                | permanent CI reruns                | high     | fix cause             |
| T48  | Are tests isolated in parallel?                     | unique resources            | shared fixed port/file             | high     | allocate uniquely     |
| T49  | Are environment mutations restored safely?          | scoped guard/process        | global env races                   | high     | isolate process       |
| T50  | Are clocks controlled?                              | injected/paused clock       | wall-clock sleep                   | high     | abstract time         |
| T51  | Is randomness seeded?                               | recorded seed               | irreproducible fuzz failure        | high     | capture               |
| T52  | Is coverage supplemental?                           | invariant matrix            | percentage alone                   | high     | add semantic evidence |
| T53  | Are benchmark assertions separately tested?         | correctness suite           | benchmark only                     | high     | extract tests         |
| T54  | Does unsafe code have specialized evidence?         | Miri/sanitizer results      | ordinary tests only                | critical | run tools             |
| T55  | Are tool blind spots stated?                        | limitations                 | Miri called complete proof         | high     | qualify               |
| T56  | Does telemetry detect claimed outcomes?             | tested metrics              | silent failure not instrumented    | high     | add observability     |
| T57  | Did incidents create regressions?                   | linked test                 | fix has no reproduction            | high     | encode mechanism      |
| T58  | Is test-data sensitivity controlled?                | synthetic/redacted fixtures | production secret copied           | critical | scrub                 |
| T59  | Are cleanup failures visible?                       | teardown result             | errors ignored                     | high     | report                |
| T60  | Is total suite cost proportionate?                  | layer rationale             | redundant slow tests               | medium   | rebalance             |

Critical gaps block merge. Waivers identify the uncovered invariant, alternative
evidence, consequence, owner, expiry, and removal condition.

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0008-R001`, `RUST-DOC-0008-R002`, `RUST-DOC-0008-R003`, `RUST-DOC-0008-R004`
- `RUST-DOC-0008-R005`, `RUST-DOC-0008-R006`, `RUST-DOC-0008-R007`, `RUST-DOC-0008-R008`
- `RUST-DOC-0008-R009`, `RUST-DOC-0008-R010`, `RUST-DOC-0008-R011`, `RUST-DOC-0008-R012`
- `RUST-DOC-0008-R013`, `RUST-DOC-0008-R014`, `RUST-DOC-0008-R015`, `RUST-DOC-0008-R016`
- `RUST-DOC-0008-R017`, `RUST-DOC-0008-R018`, `RUST-DOC-0008-R019`, `RUST-DOC-0008-R020`
- `RUST-DOC-0008-R021`
