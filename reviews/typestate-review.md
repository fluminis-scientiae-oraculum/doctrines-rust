# Typestate review

## Record

Use whenever a state parameter, marker type, consuming state-specific handle, or
type-level transition is proposed. Record **pass**, **fail**, **not applicable**,
or **waiver reference**. The review must compare a runtime enum and plain
runtime validation rather than presuming typestate wins.

## Fit and scope

| ID | Question | Pass evidence |
|---|---|---|
| TSR-01 | Is the protected problem operation sequencing rather than value validation? | invariant classification |
| TSR-02 | Is the sequence locally controlled by one handle owner? | ownership map |
| TSR-03 | Is state not primarily externally determined? | boundary analysis |
| TSR-04 | Is the state graph small and stable enough for static APIs? | node/edge count |
| TSR-05 | Are illegal calls consequential and frequent enough to justify types? | risk assessment |
| TSR-06 | Can marker construction be restricted? | visibility |
| TSR-07 | Does each marker represent evidence actually established? | guarantee mapping |
| TSR-08 | Are independent dimensions kept out of a type cross-product? | decomposition |
| TSR-09 | Is the workflow unsuitable for a simpler consuming non-generic handle? | alternative comparison |
| TSR-10 | Is a runtime enum explicitly evaluated? | decision record |

## Transition design

| ID | Question | Pass evidence |
|---|---|---|
| TSR-11 | Does each legal edge have one clear method? | state API graph |
| TSR-12 | Are illegal state-specific methods absent from the type? | impl inspection |
| TSR-13 | Do transitions consume the prior state when reuse is invalid? | signatures |
| TSR-14 | Are infallible transitions truly local and infallible? | operation analysis |
| TSR-15 | Do fallible transitions return structured errors? | error types |
| TSR-16 | Is the prior handle returned only when non-transition is proven? | error/recovery shape |
| TSR-17 | Is external ambiguity represented instead of restoring old state? | unknown outcome |
| TSR-18 | Are transition payloads carried in the successor type? | successor fields |
| TSR-19 | Are authorization and capability requirements explicit? | method arguments |
| TSR-20 | Can parallel or duplicate transition attempts occur through another handle? | resource identity analysis |

## Async and external reality

| ID | Question | Pass evidence |
|---|---|---|
| TSR-21 | Is every `.await` in a transition cancellation-reviewed? | cancellation table |
| TSR-22 | Does cancellation release or reconcile consumed resources? | cleanup evidence |
| TSR-23 | Are blocking operations isolated appropriately? | async design |
| TSR-24 | Does an open/connected marker mean only local transition success? | documentation |
| TSR-25 | Do send/capture/commit operations remain fallible? | method results |
| TSR-26 | Can timeout produce explicit unknown outcome? | outcome type |
| TSR-27 | Are external facts re-observed when needed? | validation policy |
| TSR-28 | Does typestate avoid claiming current remote liveness? | guarantee ledger |
| TSR-29 | Does a lease/capability marker account for expiry/revocation? | runtime check |
| TSR-30 | Is compensation modeled as a later fallible transition? | state graph |

## Persistence and ergonomics

| ID | Question | Pass evidence |
|---|---|---|
| TSR-31 | Must states be stored heterogeneously or inspected dynamically? | usage inventory |
| TSR-32 | Is a stable runtime persisted enum defined where needed? | storage model |
| TSR-33 | Does rehydration validate before issuing a typed handle? | restoration service |
| TSR-34 | Is marker spelling excluded from durable protocol evidence? | encoding policy |
| TSR-35 | Are optimistic version or claim semantics present for multiple workers? | persistence concurrency |
| TSR-36 | Can trait objects or plugin interfaces use the design clearly? | dispatch design |
| TSR-37 | Are mocks and test harnesses comprehensible? | test API review |
| TSR-38 | Are compiler diagnostics useful at misuse sites? | compile-fail output |
| TSR-39 | Is generic propagation limited at public boundaries? | API signatures |
| TSR-40 | Are transition errors smaller/clearer than equivalent runtime checks? | caller comparison |
| TSR-41 | Has monomorphization/code-size impact been considered? | size analysis |
| TSR-42 | Has compile-time and IDE diagnostic impact been considered? | complexity budget |

## Evidence and decision

| ID | Question | Pass evidence |
|---|---|---|
| TSR-43 | Do compile-fail tests prove important prohibited calls? | UI tests |
| TSR-44 | Do they fail for the intended reason? | diagnostic inspection |
| TSR-45 | Are every successful and failed transition tested? | unit suite |
| TSR-46 | Are cancellation and external failure tested? | fault suite |
| TSR-47 | Is persisted/runtime conversion tested? | integration suite |
| TSR-48 | Are unknown outcomes and reconciliation tested? | distributed tests |
| TSR-49 | Does documentation state exact guarantees and non-guarantees? | ledger/docs |
| TSR-50 | Does benefit exceed type/API complexity? | signed decision |

## Exit criteria

Approve typestate only when local sequencing is the real risk, construction is
protected, external effects remain fallible, persistence has a runtime model,
diagnostics are acceptable, and the complexity comparison favors it. Otherwise
select a runtime enum, consuming transition, capability, or ordinary validation.
