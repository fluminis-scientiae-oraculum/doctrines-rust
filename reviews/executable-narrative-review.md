# Executable narrative review

## Record

Use whenever a change adds a description of an architectural obligation, proposes a decision
record, adds or edits a derived view, or cites an existing record as a reason a change cannot
proceed. Record **pass**, **fail**, **not applicable**, or **waiver reference**. There is no
score: a total would let a strong result in a cheap category offset a critical failure in an
expensive one.

The review answers a question that precedes every gate below: which claim is under review, and
which single artifact is authoritative for it. A review that cannot state the claim precisely has
nothing to check.

## Source-of-truth inventory

| ID     | Question                                                              | Pass evidence            |
| ------ | --------------------------------------------------------------------- | ------------------------ |
| ENR-01 | Is the claim stated precisely enough that its truth could be checked? | claim statement          |
| ENR-02 | Which class does the claim belong to?                                 | classification           |
| ENR-03 | Which single artifact is authoritative for it?                        | authority mapping        |
| ENR-04 | Which other artifacts describe the same claim?                        | representation inventory |
| ENR-05 | Which of those are maintained by hand?                                | maintenance owner list   |
| ENR-06 | Can any of them be generated, or deleted outright?                    | disposition per entry    |
| ENR-07 | Is the representation count after this change recorded?               | review record            |

## Executability test

| ID     | Question                                                           | Pass evidence           |
| ------ | ------------------------------------------------------------------ | ----------------------- |
| ENR-08 | Can the claim become a type, a bound, or a visibility restriction? | signature or module     |
| ENR-09 | Can it become a checked constructor or a private representation?   | constructor audit       |
| ENR-10 | Can it become a schema constraint, a domain, or a cast rule?       | schema or migration     |
| ENR-11 | Can it become a test, a fixture, or a rejected-input case?         | test path               |
| ENR-12 | Can it become a manifest entry or machine-checked configuration?   | manifest or policy file |
| ENR-13 | Can the human-readable view of it be generated and drift-checked?  | generator and check     |
| ENR-14 | Can it become an executable topology or contract assertion?        | assertion path          |
| ENR-15 | If a mechanism enforces only part of it, is the rest stated?       | scope statement         |
| ENR-16 | If it stays prose, is the budget assessment recorded?              | complexity assessment   |

## Decision-record necessity test

| ID     | Question                                                        | Pass evidence          |
| ------ | --------------------------------------------------------------- | ---------------------- |
| ENR-17 | Which exact fact cannot be executable, generated, or recovered? | named fact             |
| ENR-18 | Why is that fact material to a future decision?                 | stated risk            |
| ENR-19 | Which future mistake does the record prevent?                   | failure scenario       |
| ENR-20 | Could a short comment, a manifest field, or an example suffice? | alternative comparison |
| ENR-21 | Is this a proposal to change a contract, and therefore an RFC?  | governance route       |
| ENR-22 | Is this onboarding prose in decision form?                      | audience check         |
| ENR-23 | Does the record answer one question and state its exclusions?   | scope statement        |
| ENR-24 | Does it link the artifacts authoritative for current behavior?  | linked paths           |

## Improvement-friction test

| ID     | Question                                                              | Pass evidence          |
| ------ | --------------------------------------------------------------------- | ---------------------- |
| ENR-25 | Does this artifact make a future improvement need permission from it? | dependency reading     |
| ENR-26 | Could a future reader or agent mistake it for permanent authority?    | status marking         |
| ENR-27 | Does it preserve a constraint that may disappear?                     | obsolescence condition |
| ENR-28 | Who revalidates it, and on what trigger?                              | owner and trigger      |
| ENR-29 | Is active discovery limited to currently valid records?               | registry contents      |
| ENR-30 | Was a record cited against a change without confirming it applies?    | confirmation record    |
| ENR-31 | Is an implemented proposal still cited as a current specification?    | citation audit         |

## Durable-truth test

| ID     | Question                                                           | Pass evidence          |
| ------ | ------------------------------------------------------------------ | ---------------------- |
| ENR-32 | Is a local guarantee being read as durable or remote evidence?     | ledger rows            |
| ENR-33 | Does each external fact name the system authoritative for it?      | external authority map |
| ENR-34 | Is the check that consults that system named?                      | query or call site     |
| ENR-35 | Are concurrency, fencing, and identity explicit where state moves? | token and predicate    |
| ENR-36 | Is a wire or database scalar type being read as lifecycle state?   | schema and model       |

## Narrative test

| ID     | Question                                                         | Pass evidence     |
| ------ | ---------------------------------------------------------------- | ----------------- |
| ENR-37 | Do the enforcing artifacts read as the domain's own account?     | names and states  |
| ENR-38 | Are states named for the facts they establish?                   | state definitions |
| ENR-39 | Are effects disclosed where they occur?                          | effect inventory  |
| ENR-40 | Are branches explicit rather than implied by optional fields?    | branch types      |
| ENR-41 | Is type erasure delayed to a named boundary?                     | erasure boundary  |
| ENR-42 | Does generated documentation agree with the enforcing artifacts? | drift check       |

## Rationale honesty

| ID     | Question                                                                                | Pass evidence        |
| ------ | --------------------------------------------------------------------------------------- | -------------------- |
| ENR-43 | Is recorded rationale genuinely irrecoverable from the artifacts?                       | recoverability check |
| ENR-44 | Where a reason is unavailable, is it recorded as unknown?                               | unknown record       |
| ENR-45 | Is any inference labelled as an inference, with its evidence?                           | labelled inference   |
| ENR-46 | Does every exception carry owner, consequence, control, trigger, and removal condition? | exception record     |

## Severity guidance

Treat as **critical**: one artifact cited as authority for every class; a local guarantee
presented as an external fact; an inferred rationale presented as governing; an obsolete record
still in the active set; a record cited against a change without confirming applicability; an
unenforced part of a claim left implied by the enforced part.

Treat as **high**: an enforceable obligation left in prose with no recorded assessment; a
manually maintained copy of an enforced claim; a derived view synchronized by hand; a record whose
irrecoverable fact is not named; an implemented proposal cited as a current specification; an
archived record hydrated into agent context.

Treat as **medium**: a hand-written view that is unmarked or unowned; a generated artifact with no
declared source; a representation count assessed by impression rather than stated; rationale that
restates the enforced structure without contradicting it.

## Outcome

Critical failures block merge. A valid waiver identifies the affected rule and claim, the owner
accepting the risk, the consequence, the compensating control and its evidence, an expiry or
reconsideration trigger, and the removal condition. A waiver cannot make an obsolete record
current, cannot make an inferred rationale a governing one, cannot make a local guarantee
external evidence, and cannot authorize a second maintained source for a claim an artifact
already enforces.

The most common correct outcome of this review is that no artifact is added: the obligation moves
into a mechanism, the derived view is generated, and the proposed record is not written. Record
that outcome explicitly, because a review that produces no document is easily mistaken for a
review that did not happen.

Rules exercised: `RUST-DOC-0011-R001` through `RUST-DOC-0011-R020`, with
`RUST-DOC-0010-R022` where the claim concerns a staged protocol.
