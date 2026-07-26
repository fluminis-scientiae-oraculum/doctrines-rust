# Pre-implementation review

## Record

Record feature/change identifier, planner, reviewer, date, affected doctrine
IDs, and status for every gate. Status is **pass**, **fail**, **not applicable**,
or **waiver reference**. Complete this review before public type, persistence,
or protocol choices become expensive to reverse.

## Domain and invariant inventory

| ID | Question | Pass evidence |
|---|---|---|
| PRE-01 | Is the domain objective stated without prescribing a Rust mechanism? | outcome and scope |
| PRE-02 | Is a shared vocabulary defined for values, actors, states, and effects? | vocabulary artifact |
| PRE-03 | Are ambiguous terms split by evidence level? | definitions such as parsed/verified |
| PRE-04 | Are non-goals and excluded systems explicit? | bounded scope |
| PRE-05 | Does every consequential invariant have a stable ID? | invariant inventory |
| PRE-06 | Is each invariant statement testable or reviewable? | precise predicate |
| PRE-07 | Is each invariant classified as value, state, transition, authority, lifecycle, boundary, cross-entity, temporal, environmental, or distributed? | classification field |
| PRE-08 | Is the invariant owner named? | component or role |
| PRE-09 | Is the enforcement mechanism proposed without claiming more than it proves? | mechanism column |
| PRE-10 | Is the trust boundary that establishes evidence named? | boundary column |
| PRE-11 | Is failure consequence recorded? | consequence/severity |
| PRE-12 | Is residual uncertainty recorded? | uncertainty column |
| PRE-13 | Are preconditions distinguished from invariants? | separate entries |
| PRE-14 | Are assumptions and observations distinguished from guarantees? | assumption ledger |
| PRE-15 | Are cross-entity rules excluded from pure scalar constructors? | enforcement placement |
| PRE-16 | Are external mutable facts identified as runtime evidence? | observation policy |

## State and authority

| ID | Question | Pass evidence |
|---|---|---|
| PRE-17 | Is a state graph provided for each meaningful lifecycle? | nodes and legal edges |
| PRE-18 | Does each state list required associated evidence? | state payload table |
| PRE-19 | Are mutually exclusive and independent dimensions distinguished? | representation rationale |
| PRE-20 | Does every transition identify actor and authority? | transition table |
| PRE-21 | Does every transition identify precondition and postcondition? | edge contract |
| PRE-22 | Are failure and cancellation edges present? | complete graph |
| PRE-23 | Are unknown or reconciliation states included where execution can be ambiguous? | explicit nodes |
| PRE-24 | Is an authority map provided for privileged actions? | principal/capability map |
| PRE-25 | Are capability construction, transfer, clone, expiry, and revocation defined? | authority lifecycle |
| PRE-26 | Are secret-bearing values and permitted readers identified? | data/authority map |

## Trust boundaries and external effects

| ID | Question | Pass evidence |
|---|---|---|
| PRE-27 | Is every ingress and egress boundary inventoried? | boundary map |
| PRE-28 | Does each ingress show raw, structural, and trusted representations? | conversion pipeline |
| PRE-29 | Are alternate writers and privileged bypass paths listed? | bypass inventory |
| PRE-30 | Are parsing, validation, authentication, and authorization separated? | layered design |
| PRE-31 | Are size, nesting, allocation, and concurrency limits proposed? | resource table |
| PRE-32 | Is version/unknown-value policy stated? | compatibility matrix |
| PRE-33 | Is every external side effect inventoried? | effect list |
| PRE-34 | Does each effect identify the point after which execution can be unknown? | protocol timeline |
| PRE-35 | Are idempotency and retry classifications stated per failure point? | failure matrix |
| PRE-36 | Is reconciliation evidence and owner identified? | reconciliation plan |
| PRE-37 | Are compensation actions treated as new fallible effects? | saga contract |
| PRE-38 | Are ordering claims scoped by key, producer, partition, and failover? | ordering contract |

## Persistence, complexity, and evidence

| ID | Question | Pass evidence |
|---|---|---|
| PRE-39 | Is the persistence representation distinct where its contract differs? | row/domain comparison |
| PRE-40 | Are transaction boundaries aligned with cross-entity invariants? | transaction map |
| PRE-41 | Is optimistic concurrency or another lost-update strategy selected? | conflict protocol |
| PRE-42 | Are migration and old-value compatibility needs identified? | version plan |
| PRE-43 | Is persistence plus messaging coordinated durably where loss matters? | outbox/inbox decision |
| PRE-44 | Is the concurrency ownership and synchronization model stated? | task/state ownership |
| PRE-45 | Are queue, pool, and retry capacities bounded? | capacity budget |
| PRE-46 | Is cancellation cleanup defined for partial operations? | cancellation table |
| PRE-47 | Is the simplest sufficient representation selected from enum, newtype, runtime validation, typestate, capability, or plain code? | decision record |
| PRE-48 | Does the complexity budget cover diagnostics, compile time, code size, migration, and team operation? | budget assessment |
| PRE-49 | Is unsafe code absent or separately justified under RUST-DOC-0007? | unsafe decision |
| PRE-50 | Does each invariant map to planned compiler, unit, property, compile-fail, integration, fault, model, or operational evidence? | evidence matrix |
| PRE-51 | Are negative and prohibited paths included? | rejection plan |
| PRE-52 | Are real boundaries exercised where consequential? | integration plan |
| PRE-53 | Are evidence limitations stated? | non-proof column |
| PRE-54 | Does the initial guarantee ledger state claim, establishment, protected construction, boundary preservation, escape hatches, non-proofs, and runtime risk? | ledger |

## Exit criteria

Implementation may start when every critical gate passes or has an approved
governance disposition, and the invariant inventory, boundary map, state graph,
effect inventory, authority map, persistence model, complexity budget, evidence
plan, and initial guarantee ledger are reviewable. New discoveries update these
artifacts rather than being buried only in code.
