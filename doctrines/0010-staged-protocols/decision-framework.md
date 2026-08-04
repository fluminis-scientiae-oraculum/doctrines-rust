# Decision framework

## Inputs

Bring the stage and edge inventory, the evidence each transition establishes, the failure
classes per transition, the external-effect inventory, the ownership map for the values being
advanced, the persistence model where any stage state is durable, the complexity budget, and the
evidence plan. A protocol cannot be assessed from its happy path alone.

## Questions

1. What consequential ordering is being protected, and what does the system do today when the
   order is violated?
2. Does each proposed stage establish a fact a later stage depends on, or is it a processing
   step that was convenient to name?
3. Is the sequence controlled by one owner within one process, or advanced by several actors
   against durable state?
4. Will one capability have several implementations producing different successor evidence?
5. Which transitions branch materially, which permit revision or retry, and which can end the
   protocol?
6. Which transitions perform durable writes, external calls, or message publication?
7. Where does untrusted input enter, and where must the protocol be erased for storage or
   dispatch?
8. What evidence would show the design is wrong, and what would show the graph has drifted?

## Decision table

| Situation                                                        | Preferred mechanism                                                 | Conditions                                                          | Stop condition                                                          |
| ---------------------------------------------------------------- | ------------------------------------------------------------------- | ------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| Advisory ordering with no shared evidence                        | ordinary functions in sequence                                      | violation is inconvenient rather than consequential                 | when a violation becomes a security, financial, or integrity fault      |
| Two-stage local sequence, one implementation                     | consuming transition returning a concrete successor                 | the successor never varies                                          | when a second implementation needs different successor evidence         |
| Multi-stage local sequence, one implementation per stage         | typestate with consuming transitions                                | the graph fits in signatures a reader can follow                    | when the successor relationship must be abstracted over implementations |
| Multi-stage sequence, several implementations per capability     | capability traits with bounded associated successor types           | successors differ in evidence but agree on the next capability      | when stages must be stored heterogeneously or inspected dynamically     |
| Materially different outcomes from one transition                | named sum type over distinct successor stages                       | each outcome changes the successor capability or a later obligation | when the outcome changes nothing downstream and is ordinary data        |
| Outcome that is neither success nor a modeled branch             | stage-identifying failure type                                      | availability, eligibility, or authority could not be determined     | when the third case is common enough to deserve its own stage           |
| Durable lifecycle advanced by several actors                     | runtime state model plus a typed pass issued by checked restoration | storage is authoritative and the typed protocol covers one pass     | when the typed protocol starts being treated as the durable record      |
| Runtime choice among protocol implementations                    | enum or dispatch at the selection point only                        | each branch continues through typed stages afterwards               | when the whole protocol is erased to accommodate one choice             |
| Stage state must be persisted, listed, or inspected across kinds | runtime enum with explicit operations                               | callers hold heterogeneous states together                          | when static enforcement is being simulated with runtime checks anyway   |

## Decision tree

```text
Is the ordering consequential when violated?
├─ no  → ordinary functions; record the sequence in the design note and stop
└─ yes → Does each stage establish a fact a later stage consumes?
   ├─ no  → merge the steps until they do; re-enter this tree
   └─ yes → Is the sequence advanced by one owner within one process?
      ├─ no  → runtime state model is authoritative (RUST-DOC-0005, RUST-DOC-0006)
      │        └─ is there also a local pass worth enforcing?
      │           ├─ no  → stop; runtime model only
      │           └─ yes → typed stages for the pass, issued by checked restoration
      └─ yes → How many transitions?
         ├─ one   → consuming transition with a concrete successor; stop
         ├─ two   → typestate with concrete successors unless a second
         │          implementation is already known
         └─ three or more → Will one capability have several implementations
                            producing different successor evidence?
            ├─ no  → typestate with concrete successors; revisit if that changes
            └─ yes → capability traits with bounded associated successors
                     ├─ add named sum types for material branches
                     ├─ add named stages for retry, revision, and recovery
                     ├─ add the topology assertion
                     └─ is the stage count still justifiable against the budget?
                        ├─ no  → merge to proof boundaries and re-enter
                        └─ yes → proceed to the evidence plan
```

The tree has two deliberate exits into simpler designs. A protocol that cannot answer the second
question is not a protocol, and a protocol whose stage count fails the budget check is expressing
implementation structure rather than proof structure.

## Complexity check

Count the stages, the capabilities, the implementations per capability, and the resulting
monomorphized combinations. Read one full transition signature aloud; if its bounds cannot be
followed, callers and mock authors will not follow them either. Check how far generic stage
parameters travel into helper functions, test harnesses, and public API boundaries, and whether
they can be stopped at an internal boundary.

Compare against the runtime alternative honestly: the same protocol as an enum with explicit
operations, and the same protocol as ordinary sequenced functions. Record what each alternative
would fail to prevent. If the answer is "nothing consequential," the simpler design wins.

Then check diagnostics. An unsatisfied successor bound is a worse first-encounter error message
than a plain type mismatch. If the protocol will be used mainly by people who did not write it,
that cost is real and belongs in the assessment.

## Evidence selection

| Decision                                 | Evidence class                                                        |
| ---------------------------------------- | --------------------------------------------------------------------- |
| Stage graph matches the documented graph | executable topology assertion over every edge                         |
| Illegal ordering is unrepresentable      | compile-fail case per claimed impossibility                           |
| Consumed stages cannot be reused         | compile-fail case on reuse after a consuming transition               |
| Stage evidence cannot be forged          | compile-fail case on literal construction of private evidence         |
| Each transition builds correct evidence  | unit test per transition, positive and negative                       |
| Branches produce the right successor     | unit test per branch variant                                          |
| Recovery edges re-enter correctly        | unit test per recovery path, including the terminal one               |
| Canonical values survive transitions     | unit test comparing first-stage input with terminal-stage output      |
| Effect-free stages perform no effect     | collaborator observation or fault injection asserting no write        |
| Cancellation behavior is as stated       | fault test interrupting each async transition                         |
| Durable advancement is exactly once      | competing-writer test against the real store, in the consuming system |
| Restoration issues a valid typed stage   | integration test over stored state, in the consuming system           |

The last two rows are deliberately assigned to the consuming system. This repository ships no
database or broker, so the doctrine states those obligations and the review gates check them,
but the executable evidence for them is not claimed here.
