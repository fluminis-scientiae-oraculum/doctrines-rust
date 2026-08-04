# Review standard

Mark every gate **pass**, **fail**, **not applicable**, or with an approved **waiver
reference**. Blank status is not approval. There is no score: a total would let a strong result
in a cheap category offset a critical failure in an expensive one.

## Claim classification and authority

| Gate | Question                                                      | Pass evidence         | Failure example                          | Severity | Remediation             |
| ---- | ------------------------------------------------------------- | --------------------- | ---------------------------------------- | -------- | ----------------------- |
| E01  | Is each architectural claim classified before it is cited?    | claim classification  | claim reviewed with no class named       | high     | classify first          |
| E02  | Does each classified claim name one authority?                | authority mapping     | two artifacts cited for one claim        | high     | pick the authority      |
| E03  | Is any one artifact cited as authority for every class?       | authority mapping     | doctrine cited for current behavior      | critical | partition the claims    |
| E04  | Is the enforcing artifact cited for an enforced claim?        | source or schema path | prose cited for legal ordering           | critical | cite the mechanism      |
| E05  | Is the unenforced part of a claim stated separately?          | scope statement       | partial enforcement read as complete     | critical | state the remainder     |
| E06  | Does each external fact name its authoritative system?        | external check        | remote status inferred from a local type | critical | name the external owner |
| E07  | Is a local guarantee presented as durable or remote evidence? | ledger rows           | consumed handle read as commit proof     | critical | narrow the claim        |

## Executable representation

| Gate | Question                                                          | Pass evidence         | Failure example                                 | Severity | Remediation             |
| ---- | ----------------------------------------------------------------- | --------------------- | ----------------------------------------------- | -------- | ----------------------- |
| E08  | Could this obligation be a type, constructor, or visibility rule? | executability test    | ordering enforced by convention                 | high     | move it into the type   |
| E09  | Could it be a schema constraint, cast rule, or procedure?         | schema or migration   | identifier species mixed without a cast         | high     | constrain in the schema |
| E10  | Could it be a test, fixture, or manifest entry?                   | test or manifest      | negative guarantee asserted in prose            | high     | add the check           |
| E11  | Is a prose-only obligation recorded with its assessment?          | complexity assessment | prose obligation with no reason recorded        | high     | record or enforce       |
| E12  | Does a compatibility promise have a mechanism behind it?          | test or schema check  | published promise with no check                 | high     | enforce or label        |
| E13  | Is an unenforced promise labelled as unenforced?                  | explicit statement    | intent stated as a guarantee                    | critical | label the claim         |
| E14  | Is the authoritative structure legible in domain terms?           | names and states      | positional names, hidden effects, early erasure | medium   | rename and disclose     |

## Duplication and generated views

| Gate | Question                                                    | Pass evidence            | Failure example                          | Severity | Remediation                |
| ---- | ----------------------------------------------------------- | ------------------------ | ---------------------------------------- | -------- | -------------------------- |
| E15  | Does a manually maintained copy of an enforced claim exist? | representation inventory | hand-written stage table beside the code | high     | generate or delete         |
| E16  | Is each derived view generated or drift-checked?            | generator and check      | diagram updated by hand after a change   | high     | generate the view          |
| E17  | Does each generated artifact declare its source?            | banner or header         | generated file with no provenance        | medium   | declare the source         |
| E18  | Was any generated artifact edited in place?                 | drift check              | manual fix applied to generated output   | high     | fix the source             |
| E19  | Is a hand-written view marked informative and owned?        | marking and owner        | informal diagram cited as authority      | medium   | mark or remove             |
| E20  | Would the generator need a hand-maintained input?           | generator input          | edge list retyped to feed a generator    | high     | derive or stay informative |
| E21  | Is the representation count for the claim recorded?         | inventory disposition    | duplication assessed by impression       | medium   | count the representations  |

## Decision-record necessity

| Gate | Question                                                    | Pass evidence       | Failure example                      | Severity | Remediation                |
| ---- | ----------------------------------------------------------- | ------------------- | ------------------------------------ | -------- | -------------------------- |
| E22  | Which exact fact cannot live in an executable artifact?     | named fact          | record justified as "important"      | critical | name the fact or drop it   |
| E23  | Why is a generated view insufficient for it?                | stated reason       | generation never considered          | high     | assess generation          |
| E24  | Which future decision does the record protect?              | stated risk         | record protects nothing identifiable | high     | state the risk or drop it  |
| E25  | Is the record a restatement of a decision the code carries? | comparison          | record describes the module layout   | critical | delete the record          |
| E26  | Is the record actually a proposal, so an RFC instead?       | governance route    | change proposal filed as a record    | medium   | route to the RFC process   |
| E27  | Is the record onboarding prose in decision form?            | audience check      | record explains how the system works | high     | improve names and examples |
| E28  | Does the record answer exactly one decision question?       | stated question     | one record covering four decisions   | high     | split the record           |
| E29  | Does the record state what it does not govern?              | exclusion statement | scope left to the reader             | high     | state the exclusions       |

## Record lifecycle and historical veto

| Gate | Question                                                           | Pass evidence       | Failure example                            | Severity | Remediation            |
| ---- | ------------------------------------------------------------------ | ------------------- | ------------------------------------------ | -------- | ---------------------- |
| E30  | Does each active record name an owner?                             | registry entry      | record with no accountable role            | critical | assign an owner        |
| E31  | Does it name a revalidation trigger and an obsolescence condition? | registry entry      | record active with no end condition        | critical | state both             |
| E32  | Does it link the executable authorities for current behavior?      | linked paths        | record silent on what governs behavior now | high     | link the authorities   |
| E33  | Has a record whose reason ended been expired or archived?          | status change       | obsolete record still active               | critical | expire the record      |
| E34  | Was a record cited as a constraint without confirming it?          | confirmation record | old record cited to block a change         | critical | confirm or withdraw    |
| E35  | Is an implemented proposal still cited as a specification?         | citation audit      | accepted RFC treated as current contract   | high     | cite doctrine and code |

## Rationale honesty, agents, and governance

| Gate | Question                                                      | Pass evidence        | Failure example                            | Severity | Remediation            |
| ---- | ------------------------------------------------------------- | -------------------- | ------------------------------------------ | -------- | ---------------------- |
| E36  | Is recorded rationale genuinely irrecoverable from artifacts? | recoverability check | rationale restates the type signatures     | medium   | trim to what is unique |
| E37  | Is an absent rationale recorded as unknown?                   | unknown record       | a reason inferred from the implementation  | critical | record unknown         |
| E38  | Is any inference labelled as an inference with its evidence?  | labelled inference   | inference presented as governing rationale | critical | label or remove        |
| E39  | Do generated agent packs exclude archived records?            | pack contents        | expired record hydrated as context         | high     | exclude from the packs |
| E40  | Does every exception carry all five recorded terms?           | exception record     | exception with no removal condition        | high     | complete the terms     |

## Outcome

Critical failures block merge. A valid waiver identifies the affected rule and claim, the owner
accepting the risk, the consequence, the compensating control and its evidence, an expiry or
reconsideration trigger, and the removal condition. A waiver cannot make an obsolete record
current, cannot make an inferred rationale a governing one, cannot make a local guarantee
external evidence, and cannot authorize a second maintained source for a claim an artifact
already enforces. Remediation is verified by re-running the gate against the changed artifact,
not by asserting that the change was made.

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0011-R001`, `RUST-DOC-0011-R002`, `RUST-DOC-0011-R003`, `RUST-DOC-0011-R004`
- `RUST-DOC-0011-R005`, `RUST-DOC-0011-R006`, `RUST-DOC-0011-R007`, `RUST-DOC-0011-R008`
- `RUST-DOC-0011-R009`, `RUST-DOC-0011-R010`, `RUST-DOC-0011-R011`, `RUST-DOC-0011-R012`
- `RUST-DOC-0011-R013`, `RUST-DOC-0011-R014`, `RUST-DOC-0011-R015`, `RUST-DOC-0011-R016`
- `RUST-DOC-0011-R017`, `RUST-DOC-0011-R018`, `RUST-DOC-0011-R019`, `RUST-DOC-0011-R020`

Gate groups map to rules as follows. E01 to E07 cover `RUST-DOC-0011-R001`,
`RUST-DOC-0011-R003`, and `RUST-DOC-0011-R014`. E08 to E14 cover `RUST-DOC-0011-R002`,
`RUST-DOC-0011-R015`, and `RUST-DOC-0011-R016`. E15 to E21 cover `RUST-DOC-0011-R004`,
`RUST-DOC-0011-R005`, and `RUST-DOC-0011-R017`. E22 to E29 cover `RUST-DOC-0011-R006`,
`RUST-DOC-0011-R007`, and `RUST-DOC-0011-R008`. E30 to E35 cover `RUST-DOC-0011-R009`,
`RUST-DOC-0011-R010`, and `RUST-DOC-0011-R011`. E36 to E40 cover `RUST-DOC-0011-R012`,
`RUST-DOC-0011-R013`, `RUST-DOC-0011-R018`, `RUST-DOC-0011-R019`, and `RUST-DOC-0011-R020`.
