# Decision framework

## Inputs

- the claim, stated precisely enough that its truth could be checked;
- the mechanisms available in the language, schema, build, and deployment configuration;
- the systems that own any durable or remote fact the claim depends on;
- the existing artifacts that already describe the claim, and who maintains each;
- the complexity budget assessment from `foundations/complexity-budget.md`;
- the audience that has to act on the claim, and what they consult today.

## Questions

1. Which class does the claim belong to: enforced local truth, external or durable fact,
   rationale, non-guarantee or accepted risk, or change authority?
2. Which available mechanism could enforce it, and what would that cost?
3. If a mechanism enforces part of it, which part is left unenforced?
4. Which artifacts already describe this claim, and how many of them are maintained by hand?
5. Can a described view be generated from the artifact that enforces the claim?
6. Would a generator need a hand-maintained input that describes the same claim?
7. If a record is proposed, which exact fact in it cannot be represented, enforced, generated, or
   recovered?
8. What event makes that fact stop mattering, and who notices?
9. Which artifact stays authoritative for current behavior after the record exists?
10. Is the proposal actually a change proposal, and therefore an RFC rather than a record?

## Decision table

| Situation                                                          | Placement                                                | Rules                                      |
| ------------------------------------------------------------------ | -------------------------------------------------------- | ------------------------------------------ |
| Ordering, invariant, or construction restriction, mechanism exists | the mechanism; prose is informative                      | `RUST-DOC-0011-R002`, `RUST-DOC-0011-R003` |
| Negative guarantee that can be demonstrated by rejection           | compile-fail fixture or rejected-case test               | `RUST-DOC-0011-R002`, `RUST-DOC-0011-R015` |
| Enforceable, but enforcement cost exceeds the budget               | prose, with the assessment and the five exception terms  | `RUST-DOC-0011-R002`, `RUST-DOC-0011-R020` |
| Human-readable view of an enforced claim                           | generated from the enforcing artifact, drift-checked     | `RUST-DOC-0011-R004`, `RUST-DOC-0011-R005` |
| View whose generator needs a hand-maintained description           | leave informative and owned; do not call it generated    | `RUST-DOC-0011-R005`                       |
| Durable, remote, or externally governed fact                       | the external system, named as the authority              | `RUST-DOC-0011-R014`                       |
| Rejected alternative whose rejection still governs                 | rationale, with the evidence the code does not carry     | `RUST-DOC-0011-R012`                       |
| Reason for an existing constraint is unavailable                   | record it as unknown, or label the inference             | `RUST-DOC-0011-R013`                       |
| External mandate, irreversible commitment, or accepted risk        | a decision record, with owner, triggers, and authorities | `RUST-DOC-0011-R006`, `RUST-DOC-0011-R007` |
| Proposal to change a normative contract                            | an RFC; retire it from authority once implemented        | `RUST-DOC-0011-R011`, `RUST-DOC-0011-R019` |
| Onboarding difficulty                                              | names, types, tests, generated views, examples           | `RUST-DOC-0011-R006`, `RUST-DOC-0011-R016` |

## Decision tree

This tree stays a text block while the corpus's other decision trees are drawn. Its four gates
continue in reading order rather than by a stated edge, so drawing them would require inventing
the edges between them and fixing one reading of a continuation the prose leaves open.

```text
Is the claim about a durable, remote, or externally governed fact?
  yes -> name the external authority and the check that consults it. R014. Stop.
  no  -> continue

Can an available mechanism enforce the claim, wholly or partly?
  no  -> is the reason cost, or is the fact simply not enforceable by anything?
           cost         -> prose, plus the budget assessment and the five terms. R002, R020.
           unenforceable -> continue to the record test.
  yes -> represent it in that mechanism. R002.
         Does the mechanism enforce all of it?
           yes -> the mechanism is the authority; any prose is informative. R003.
           no  -> state the unenforced part separately, and label it unenforced. R003, R015.

Does another maintained artifact also describe this claim?
  yes -> can it be generated from the enforcing artifact?
           yes -> generate it, declare the source, add the drift check. R005.
           no  -> would the generator need a hand-maintained description of the claim?
                    yes -> keep it informative and owned; do not call it generated. R005.
                    no  -> delete it, or confine it to rationale and non-guarantees. R004.
  no  -> continue

Record test. Which exact fact cannot be represented, enforced, generated, or recovered?
  none named        -> write no record. R006. Stop.
  the decision is a proposal to change a contract -> file an RFC instead. R011.
  a fact is named   -> is it an external mandate, an irreversible or externally expensive
                       commitment, a rejected alternative whose rejection depends on evidence the
                       implementation does not carry, a decision no single system owns, an
                       accepted residual risk, or a compatibility obligation from shipped
                       behavior?
                         no  -> write no record. R006. Stop.
                         yes -> write one narrow record. R008.
                                State the last-resort justification, the owner, the revalidation
                                trigger, the obsolescence condition, and the executable
                                authorities that govern current behavior. R007.
                                Register it in the active set so it can be audited and expired.
                                R009, R018.
```

## Complexity check

Count the representations of the claim after the decision, and compare with the count before it.
An acceptable outcome reduces the count or holds it at one authoritative representation plus
whatever is generated. An outcome that adds a maintained representation needs a reason stated in
the review record under `RUST-DOC-0011-R017`.

Moving an obligation into a mechanism has costs of its own. A type that enforces an ordering
lengthens signatures and worsens first-encounter diagnostics; a schema constraint moves a failure
from a readable message to a driver error; a generated view adds a generator and a drift check
that fails on unrelated changes until it is understood. Where the enforcement cost exceeds the
consequence of the obligation being violated, `RUST-DOC-0011-R002` permits the prose form with
the assessment recorded, and `RUST-DOC-0011-R020` requires the exception to name an owner and an
end condition. That is the honest exit, and it is preferable to a mechanism nobody can read,
which `RUST-DOC-0011-R016` treats as its own failure.

## Evidence selection

| Claim class                                  | Evidence that fits                                               | Evidence that does not                         |
| -------------------------------------------- | ---------------------------------------------------------------- | ---------------------------------------------- |
| Legal ordering or transition restriction     | types, compile-fail fixtures, contract assertions                | a document stating the order                   |
| Construction restriction                     | private representation, checked constructor, visibility audit    | a naming convention                            |
| Permitted conversion or cast                 | schema base types, explicit conversion functions, rejected cases | a comment naming the intended type             |
| Persistence invariant                        | schema constraint, checked decoding, transaction predicate       | an application-layer assertion alone           |
| Wire compatibility                           | canonical encoder and decoder, schema, compatibility suite       | a version note in a changelog                  |
| Negative guarantee                           | compile-fail fixture, rejected input case                        | prose asserting impossibility                  |
| Derived human-readable view                  | generator, declared source, drift check                          | a hand-updated diagram                         |
| Durable or remote fact                       | the external system's own check, with its identity and token     | a local type reached by a consuming transition |
| External mandate or accepted risk            | a registered decision record with owner and end condition        | a commit message or an issue thread            |
| Reason a rejected alternative stays rejected | rationale naming the evidence, dated                             | an inference from the current implementation   |

Choose the narrowest evidence class that matches the claim. A green suite, a passing build, or a
generated bundle is never itself evidence that an obligation is enforced; each proves only what
it exercised.
