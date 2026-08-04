# Review standard

Mark every gate **pass**, **fail**, **not applicable**, or with an approved **waiver
reference**. Blank status is not approval.

## Protocol discovery and stage identity

| Gate | Question                                               | Pass evidence      | Failure example                          | Severity | Remediation               |
| ---- | ------------------------------------------------------ | ------------------ | ---------------------------------------- | -------- | ------------------------- |
| S01  | Does a stage and edge inventory exist?                 | inventory document | types derived from existing functions    | high     | write the inventory first |
| S02  | Does each transition name the evidence it establishes? | evidence table     | transition described only as a step      | high     | name the proof            |
| S03  | Are failure classes listed per transition?             | failure inventory  | one shared failure list                  | high     | separate by stage         |
| S04  | Are external effects listed per transition?            | effect inventory   | effects discovered during implementation | high     | complete the inventory    |
| S05  | Is each stage named for a proven fact?                 | names and claims   | a stage named for its position           | high     | rename to the proof       |
| S06  | Can every stage name be tied to a ledger claim?        | guarantee ledger   | a stage with no stated claim             | critical | state or delete the stage |
| S07  | Is any stage a renamed processing step?                | boundary rationale | a stage per helper function              | medium   | merge into a proof        |

## Successor capability and bounds

| Gate | Question                                                 | Pass evidence         | Failure example                      | Severity | Remediation                 |
| ---- | -------------------------------------------------------- | --------------------- | ------------------------------------ | -------- | --------------------------- |
| S08  | Does each nonterminal capability name a successor type?  | trait definitions     | successor stated only in prose       | critical | add the associated type     |
| S09  | Is the successor bounded by the next capability?         | associated-type bound | unconstrained generic successor      | critical | add the bound               |
| S10  | Is the successor relationship free of type erasure?      | signatures            | successor returned as a trait object | critical | keep the concrete relation  |
| S11  | Do terminal stages avoid naming a successor?             | trait definitions     | terminal stage points at itself      | medium   | mark the stage terminal     |
| S12  | Does each bound reflect capability actually established? | evidence mapping      | bound widened to compile             | critical | fix stage or implementation |
| S13  | Was any bound relaxed, and was the relaxation reviewed?  | change record         | silent bound removal in a refactor   | critical | restore or record           |
| S14  | Can two implementations produce different successors?    | implementation list   | successor hardcoded where it varies  | medium   | abstract the successor      |

## Transition, evidence, and failure

| Gate | Question                                                   | Pass evidence       | Failure example                             | Severity | Remediation                |
| ---- | ---------------------------------------------------------- | ------------------- | ------------------------------------------- | -------- | -------------------------- |
| S15  | Does each transition consume its stage where reuse is bad? | method receivers    | transition advances an internal flag        | critical | consume the stage          |
| S16  | Is a borrowing transition justified?                       | read-only rationale | borrowing chosen for caller convenience     | high     | consume or justify         |
| S17  | Is the prior stage returned only on proven non-transition? | recovery shape      | prior stage restored after a partial effect | critical | return an explicit outcome |
| S18  | Does each stage carry what its successors need?            | field mapping       | later stage re-derives a checked fact       | medium   | move the evidence forward  |
| S19  | Are superseded raw representations removed?                | field audit         | raw input kept beside canonical value       | high     | drop or name separately    |
| S20  | Is retained original input separately named?               | field names         | one field holds raw or canonical            | high     | split the fields           |
| S21  | Does each failure identify its stage?                      | failure types       | one opaque protocol error                   | high     | separate by stage          |
| S22  | Is failure erasure deferred to the boundary?               | mapping location    | stages erase failure immediately            | high     | map at the boundary        |

## Branches, recovery, and granularity

| Gate | Question                                                | Pass evidence      | Failure example                            | Severity | Remediation               |
| ---- | ------------------------------------------------------- | ------------------ | ------------------------------------------ | -------- | ------------------------- |
| S23  | Is each material branch a named sum over successors?    | branch enum        | one successor with optional fields         | critical | model the branch          |
| S24  | Does each branch variant carry its own successor bound? | variant bounds     | both branches share one capability         | high     | bound per variant         |
| S25  | Is an undetermined outcome distinct from both branches? | failure or outcome | undetermined treated as rejection          | critical | represent the third case  |
| S26  | Is each retry or revision path a named edge?            | recovery stage     | retry left to caller control flow          | high     | name the edge             |
| S27  | Does a revision edge re-enter at the correct stage?     | successor bound    | revision skips canonicalization            | critical | bound the re-entry        |
| S28  | Is a terminal recovery stage genuinely terminal?        | stage definition   | abandoned stage still exposes transitions  | medium   | remove the operations     |
| S29  | Is the stage count justified against complexity?        | budget assessment  | twenty stages for one request              | medium   | merge to proof boundaries |
| S30  | Does any stage hide unrelated responsibilities?         | effect inventory   | one stage validates, writes, and publishes | high     | split the stage           |

## Construction, bypass, and erasure

| Gate | Question                                                  | Pass evidence       | Failure example                        | Severity | Remediation             |
| ---- | --------------------------------------------------------- | ------------------- | -------------------------------------- | -------- | ----------------------- |
| S31  | Are stage fields private?                                 | visibility audit    | public field on a later stage          | critical | restrict visibility     |
| S32  | Is there a conversion that produces a later stage?        | implementation list | a conversion into an approved stage    | critical | delete the conversion   |
| S33  | Does any derive construct a stage without its transition? | derive audit        | derived decoding of stage evidence     | critical | route through the stage |
| S34  | Are trusted construction paths visibility-restricted?     | visibility          | public test builder in the shipped API | critical | restrict the path       |
| S35  | Is every trusted path in the escape-hatch inventory?      | ledger              | an undocumented factory                | critical | inventory or remove     |
| S36  | Does each trusted path state its caller obligation?       | obligation record   | path documented only as convenience    | high     | state the obligation    |
| S37  | Does erasure occur only at a named boundary?              | boundary record     | a map passed between stages            | critical | keep the types          |
| S38  | Does dynamic selection preserve typed progression?        | dispatch design     | whole protocol erased for one choice   | high     | erase only the choice   |

## Effects, durability, and asynchrony

| Gate | Question                                                    | Pass evidence        | Failure example                           | Severity | Remediation                 |
| ---- | ----------------------------------------------------------- | -------------------- | ----------------------------------------- | -------- | --------------------------- |
| S39  | Does each transition disclose its durable effects?          | effect inventory     | a check stage writes a row                | critical | disclose or move the effect |
| S40  | Do check and preparation stages perform no durable write?   | code trace and tests | validation publishes a message            | critical | separate the stages         |
| S41  | Is a local transition kept distinct from a durable one?     | ledger rows          | consumed handle presented as commit proof | critical | narrow the claim            |
| S42  | Does authoritative advancement re-check identity and state? | query or procedure   | update by identity alone                  | critical | add the state predicate     |
| S43  | Does it carry a version, fence, or equivalent token?        | concurrency token    | blind overwrite of durable state          | critical | add concurrency control     |
| S44  | Is persisted lifecycle modeled at runtime?                  | storage model        | stage marker persisted as protocol truth  | critical | persist a runtime state     |
| S45  | Does restoration issue a typed stage through checked code?  | restoration service  | stored tag deserialized into a stage      | critical | validate before issuing     |
| S46  | Is each async transition's cancellation behavior stated?    | cancellation table   | interruption behavior unexamined          | high     | state per stage             |
| S47  | Is retry safety and its identity stated?                    | idempotency identity | retry without a deduplication identity    | critical | define the identity         |
| S48  | Is a durable acknowledgment required before the successor?  | ordering evidence    | successor built before acknowledgment     | critical | reorder or split the stage  |

## Evidence, honesty, and governance

| Gate | Question                                                    | Pass evidence           | Failure example                      | Severity | Remediation              |
| ---- | ----------------------------------------------------------- | ----------------------- | ------------------------------------ | -------- | ------------------------ |
| S49  | Does each claimed impossibility have compile-fail evidence? | compile-fail cases      | claim stated only in prose           | high     | add the case             |
| S50  | Was each diagnostic inspected for its semantic cause?       | reviewed diagnostic     | fixture accepted mechanically        | high     | inspect and re-record    |
| S51  | Do the cases reject at the intended boundary?               | diagnostic analysis     | case fails for an unrelated reason   | high     | rewrite the case         |
| S52  | Is the documented stage graph asserted executably?          | topology assertion      | graph checked only by reading        | high     | add the assertion        |
| S53  | Does the assertion cover every documented edge?             | coverage comparison     | branch edges unasserted              | medium   | extend the assertion     |
| S54  | Does the assertion fail when an edge changes?               | deliberate break        | assertion passes after a redirect    | high     | strengthen the assertion |
| S55  | Does every stage have a guarantee ledger row?               | completed ledger        | evidence absent from the ledger      | critical | complete the ledger      |
| S56  | Does each row state what the stage does not prove?          | ledger column           | stage claims durable completion      | critical | narrow the claim         |
| S57  | Is local vocabulary distinguished from standard terms?      | terminology definitions | a local coinage cited as established | medium   | attribute the family     |
| S58  | Is the governing decision record identified?                | decision reference      | code presented as the whole contract | high     | record the decision      |

## Outcome

Critical failures block merge. A valid waiver identifies the affected rule and protocol, the
owner accepting the risk, the consequence, the compensating control and its evidence, an expiry
or reconsideration trigger, and the removal condition. A waiver cannot make a bypassed protocol
sound, cannot convert a local move into a durable transition, and cannot make an inaccurate
external claim true. Remediation is verified by re-running the gate against the changed
artifact, not by asserting that the change was made.

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0010-R001`, `RUST-DOC-0010-R002`, `RUST-DOC-0010-R003`, `RUST-DOC-0010-R004`
- `RUST-DOC-0010-R005`, `RUST-DOC-0010-R006`, `RUST-DOC-0010-R007`, `RUST-DOC-0010-R008`
- `RUST-DOC-0010-R009`, `RUST-DOC-0010-R010`, `RUST-DOC-0010-R011`, `RUST-DOC-0010-R012`
- `RUST-DOC-0010-R013`, `RUST-DOC-0010-R014`, `RUST-DOC-0010-R015`, `RUST-DOC-0010-R016`
- `RUST-DOC-0010-R017`, `RUST-DOC-0010-R018`, `RUST-DOC-0010-R019`, `RUST-DOC-0010-R020`
- `RUST-DOC-0010-R021`, `RUST-DOC-0010-R022`

Gate groups map to rules as follows. S01 to S07 cover `RUST-DOC-0010-R001` and
`RUST-DOC-0010-R002`. S08 to S14 cover `RUST-DOC-0010-R003` and `RUST-DOC-0010-R004`. S15 to S22
cover `RUST-DOC-0010-R005`, `RUST-DOC-0010-R006`, and `RUST-DOC-0010-R007`. S23 to S30 cover
`RUST-DOC-0010-R008`, `RUST-DOC-0010-R009`, and `RUST-DOC-0010-R012`. S31 to S38 cover
`RUST-DOC-0010-R010`, `RUST-DOC-0010-R011`, and `RUST-DOC-0010-R017`. S39 to S48 cover
`RUST-DOC-0010-R013`, `RUST-DOC-0010-R014`, `RUST-DOC-0010-R015`, and `RUST-DOC-0010-R016`. S49
to S58 cover `RUST-DOC-0010-R018`, `RUST-DOC-0010-R019`, `RUST-DOC-0010-R020`,
`RUST-DOC-0010-R021`, and `RUST-DOC-0010-R022`.
