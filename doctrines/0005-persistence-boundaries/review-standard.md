# Review standard

Mark every gate **pass**, **fail**, **not applicable**, or with an approved
**waiver reference**.

| Gate | Question                                              | Pass evidence            | Failure example                         | Severity | Remediation               |
| ---- | ----------------------------------------------------- | ------------------------ | --------------------------------------- | -------- | ------------------------- |
| P01  | Are all durable representations inventoried?          | storage map              | cache snapshot omitted                  | high     | enumerate sources         |
| P02  | Are alternate writers known?                          | writer list              | admin tool bypasses service             | high     | constrain or validate     |
| P03  | Is persisted data treated as boundary input?          | fallible conversion      | row directly becomes trusted type       | critical | add raw model             |
| P04  | Are private newtype constructors preserved?           | checked constructor call | ORM writes field internally             | critical | route through `TryFrom`   |
| P05  | Do invalid rows fail explicitly?                      | structured error         | invalid value normalized silently       | critical | reject or quarantine      |
| P06  | Are conversion diagnostics actionable?                | record ID and category   | opaque decode error                     | medium   | add safe context          |
| P07  | Are sensitive values absent from diagnostics?         | redaction tests          | token printed with row error            | critical | redact                    |
| P08  | Are storage and domain models separated where needed? | contract comparison      | nullable row leaks into domain          | high     | split models              |
| P09  | Are null combinations validated?                      | truth table              | paid row has no receipt                 | critical | sum conversion and checks |
| P10  | Are defaults evidence-honest?                         | provenance               | migration invents verification time     | critical | derive or keep unknown    |
| P11  | Does schema reinforce stable invariants?              | constraint map           | alternate writer can store zero         | high     | add constraint            |
| P12  | Are constraint failures structured?                   | conflict mapping         | all become internal error               | high     | preserve category         |
| P13  | Are cross-row rules transactionally protected?        | isolation proof          | check then update races                 | critical | lock/constraint/protocol  |
| P14  | Is isolation tied to the anomaly?                     | documented analysis      | transaction assumed sufficient          | critical | choose mechanism          |
| P15  | Are concurrent writers tested?                        | competing-operation test | only sequential test                    | high     | add concurrency evidence  |
| P16  | Are lost updates prevented?                           | version or atomic update | blind overwrite                         | critical | add concurrency control   |
| P17  | Is version conflict visible?                          | typed conflict           | zero rows treated as success            | critical | return conflict           |
| P18  | Is last-write-wins explicit if used?                  | policy approval          | accidental overwrite                    | high     | document or prevent       |
| P19  | Is enum encoding stable?                              | encoding table           | source variant name persisted casually  | high     | define tags               |
| P20  | Are unknown enum values handled?                      | explicit branch          | decoder panics                          | high     | reject/retain/unknown     |
| P21  | Is downgrade behavior considered?                     | compatibility matrix     | old reader misinterprets new state      | high     | stage rollout             |
| P22  | Are durable formats versioned?                        | version strategy         | old snapshot silently decoded anew      | critical | add version/migration     |
| P23  | Are unknown versions rejected safely?                 | fixture test             | version ignored                         | high     | preserve incompatibility  |
| P24  | Does migration name invariant effects?                | migration contract       | shape-only description                  | high     | add domain analysis       |
| P25  | Does strengthening scan old rows?                     | precondition query       | constraint fails mid-rollout            | critical | scan and repair           |
| P26  | Is postcondition verified completely?                 | authoritative query      | sampled rows only                       | critical | query full affected set   |
| P27  | Is rollback semantically safe?                        | compatibility reasoning  | old binary corrupts new meaning         | high     | prefer forward repair     |
| P28  | Is rollout order explicit?                            | expand/contract sequence | writer deploys before readers           | critical | stage compatibility       |
| P29  | Are decoding resource limits set?                     | limits and tests         | huge blob allocated blindly             | high     | bound or stream           |
| P30  | Are batches bounded?                                  | pagination policy        | full table loaded                       | high     | page and cap              |
| P31  | Is transaction handle lifecycle guarded?              | consuming/runtime state  | reused after commit                     | high     | consume or reject         |
| P32  | Is commit ambiguity considered?                       | driver/protocol analysis | any error means rollback                | critical | reconcile unknown         |
| P33  | Are rollback errors preserved?                        | cleanup result           | rollback failure discarded              | high     | report residual state     |
| P34  | Is connection loss behavior documented?               | failure matrix           | retry assumes no commit                 | critical | identify outcome          |
| P35  | Are external effects outside DB atomicity?            | boundary diagram         | email called transactionally            | critical | add durable protocol      |
| P36  | Is durable intent used when loss matters?             | outbox/inbox design      | commit then best-effort publish         | critical | couple intent             |
| P37  | Is outbox write in the same transaction?              | query evidence           | separate connection writes              | critical | make atomic               |
| P38  | Is publisher retry idempotent?                        | operation identity       | duplicate external effect               | critical | deduplicate/reconcile     |
| P39  | Is outbox lag observable?                             | metrics/alerts           | stuck events invisible                  | high     | instrument                |
| P40  | Is retention defined?                                 | cleanup policy           | dedup/outbox grows forever              | medium   | bound safely              |
| P41  | Is ordering scope documented?                         | aggregate/partition rule | insertion order called global           | high     | narrow claim              |
| P42  | Is invalid history quarantined?                       | explicit path            | unchecked constructor used              | critical | contain invalid evidence  |
| P43  | Is repair audited?                                    | before/after evidence    | manual edit unrecorded                  | high     | record repair             |
| P44  | Are backup/restores validated?                        | restore test             | stale schema restored blindly           | high     | migrate and check         |
| P45  | Are replicas/freshness claims accurate?               | read-routing contract    | replica read called current             | high     | state staleness           |
| P46  | Are durability settings identified?                   | configuration evidence   | product default assumed                 | critical | document and monitor      |
| P47  | Are schema and domain tests linked?                   | invariant matrix         | tests cover only model                  | medium   | add boundary cases        |
| P48  | Are administrative escape paths reviewed?             | access and audit policy  | direct SQL silently allowed             | high     | restrict and validate     |
| P49  | Does guarantee ledger state non-guarantees?           | completed ledger         | persisted implies externally complete   | critical | narrow claim              |
| P50  | Are evidence limits stated?                           | residual risks           | tests presented as proof of all history | high     | document limits           |

Critical failures block merge. A waiver must identify owner, affected paths,
compensating controls, monitoring, expiry, and a condition for removal.
