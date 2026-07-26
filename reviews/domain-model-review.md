# Domain model review

## Record

Apply to every trusted value, aggregate, state machine, and authority-bearing
handle. Record **pass**, **fail**, **not applicable**, or **waiver reference**,
with source paths and doctrine rule IDs.

## Values and names

| ID | Question | Pass evidence |
|---|---|---|
| DMR-01 | Does each trusted type name only evidence its constructors establish? | constructor/name comparison |
| DMR-02 | Are raw, parsed, policy-accepted, verified, authorized, and reconciled values distinct where operationally different? | evidence ladder |
| DMR-03 | Are primitive aliases replaced where unit or invariant mixing is consequential? | opaque type |
| DMR-04 | Are zero and empty states evaluated explicitly? | boundary table |
| DMR-05 | Does integer money include explicit currency? | money representation |
| DMR-06 | Does arithmetic reject currency mismatch and overflow? | checked operations |
| DMR-07 | Are tax, FX, scale, allocation, and rounding outside the scalar guarantee? | non-guarantees |
| DMR-08 | Does email syntax avoid ownership/deliverability claims? | evidence-accurate types |
| DMR-09 | Are identifiers nonempty/bounded/normalized according to one policy? | constructor |
| DMR-10 | Are secrets deliberately non-formatting and minimally cloneable? | trait/API audit |

## Construction and mutation

| ID | Question | Pass evidence |
|---|---|---|
| DMR-11 | Are trusted representation fields private? | visibility inspection |
| DMR-12 | Does every public constructor enforce the complete documented invariant? | constructor trace |
| DMR-13 | Is fallible construction visibly fallible? | `Result`/`TryFrom` API |
| DMR-14 | Is normalization centralized and ordered before dependent checks? | construction pipeline |
| DMR-15 | Are errors structured and actionable? | error enum/categories |
| DMR-16 | Do mutation methods preserve the complete invariant? | mutation proof/tests |
| DMR-17 | Are mutable representation escapes absent? | no `DerefMut`/raw field |
| DMR-18 | Do conversion impls preserve evidence direction? | `From` versus `TryFrom` audit |
| DMR-19 | Are unchecked constructors private, narrow, and obligation-documented? | escape-hatch inventory |
| DMR-20 | Are unsafe constructors reviewed under doctrine 0007? | safety proof |

## States and transitions

| ID | Question | Pass evidence |
|---|---|---|
| DMR-21 | Are mutually exclusive states represented by a sum type? | state shape |
| DMR-22 | Are contradictory boolean/optional combinations absent? | truth-table review |
| DMR-23 | Does associated data live only in meaningful variants? | enum payloads |
| DMR-24 | Are independent dimensions kept separate? | state decomposition |
| DMR-25 | Is variant evolution and unknown persistence planned? | encoding/version policy |
| DMR-26 | Are legal transition edges explicit? | state graph |
| DMR-27 | Are illegal transitions structurally blocked or explicitly rejected? | API/runtime checks |
| DMR-28 | Do consuming transitions prevent invalid prior-state reuse where useful? | ownership API |
| DMR-29 | Do fallible transitions preserve or consume prior authority honestly? | error shape |
| DMR-30 | Do async transitions handle cancellation and partial effects? | cancellation matrix |
| DMR-31 | Does local typestate avoid remote-liveness claims? | guarantee ledger |
| DMR-32 | Are persisted/dynamic states represented at runtime? | hybrid/runtime model |
| DMR-33 | Are unknown distributed outcomes explicit? | outcome enum |
| DMR-34 | Does reconciliation require new evidence rather than arbitrary assignment? | transition service |

## Authority, aggregates, and external rules

| ID | Question | Pass evidence |
|---|---|---|
| DMR-35 | Are privileged constructors restricted to the authority owner? | module visibility |
| DMR-36 | Does each capability expose least-privilege operations? | API surface |
| DMR-37 | Is capability cloning justified? | clone/transfer policy |
| DMR-38 | Are expiry and revocation runtime semantics explicit? | authority lifecycle |
| DMR-39 | Are single-use tokens consumed or transactionally claimed? | use protocol |
| DMR-40 | Are collection invariants protected after mutation? | wrapper API |
| DMR-41 | Are completeness claims absent from paginated subsets? | type naming |
| DMR-42 | Are cross-entity rules enforced in a domain service/transaction? | service boundary |
| DMR-43 | Are environmental assumptions represented as checks/observations? | runtime validation |
| DMR-44 | Do external effects remain fallible? | `Result`/outcome API |
| DMR-45 | Does timeout preserve unknown execution where needed? | outcome handling |
| DMR-46 | Are public escape hatches enumerated in the guarantee ledger? | ledger |
| DMR-47 | Does each type list what it proves? | documentation |
| DMR-48 | Does each type list what it does not prove? | documentation |
| DMR-49 | Does executable evidence cover acceptance and rejection? | tests |
| DMR-50 | Is type-system complexity proportional to misuse impact? | complexity decision |

## Exit criteria

Approval requires protected construction, evidence-accurate names, complete
state truth tables, legal transition handling, honest external fallibility, and
an updated guarantee ledger. Idiomatic syntax is not sufficient.
