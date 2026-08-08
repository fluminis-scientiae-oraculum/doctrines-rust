# Review standard

Record pass, fail, not applicable, or waiver.

| Gate | Question                                                               | Check    | Pass evidence               | Failure example                 | Severity | Remediation            |
| ---- | ---------------------------------------------------------------------- | -------- | --------------------------- | ------------------------------- | -------- | ---------------------- |
| O01  | **Authority map.** Is every resource and operation owner named?        | judgment | custody/lifecycle map       | ambient service access          | major    | assign ownership       |
| O02  | **Exclusivity.** Does exclusive domain authority have one local owner? | judgment | non-cloneable value         | copied commit token             | critical | consume or coordinate  |
| O03  | **Issuance.** Can unauthorized code construct capability?              | judgment | restricted constructor      | public capability field         | critical | restrict issuer        |
| O04  | **Least privilege.** Does capability expose only scoped operations?    | judgment | narrow methods/scope        | admin service inside token      | critical | split capabilities     |
| O05  | **Clone.** Is every authority clone deliberate?                        | judgment | clone contract/test         | derive for convenience          | critical | remove or redefine     |
| O06  | **Transfer.** Is delegation explicit and auditable?                    | judgment | move/delegation record      | hidden clone across tasks       | major    | model transfer         |
| O07  | **Revocation.** Can stale authority be used?                           | judgment | recheck or lease            | perpetual session capability    | critical | bound validity         |
| O08  | **Borrow.** Does borrowed access grant only required rights?           | judgment | receiver/lifetime audit     | read view exposes mutation      | major    | narrow borrow          |
| O09  | **RAII.** Is drop limited to local/best-effort cleanup?                | judgment | explicit fallible close     | drop claims rollback            | critical | expose completion      |
| O10  | **Secret debug.** Are formatting paths redacted?                       | judgment | trait/redaction tests       | derived debug token             | critical | custom debug           |
| O11  | **Secret copies.** Are clone and serialization minimized?              | judgment | call-site inventory         | secret freely cloneable         | critical | scope exposure         |
| O12  | **Zeroization.** Is claim limited to controlled buffers?               | judgment | guarantee ledger            | "all traces removed"            | major    | narrow claim           |
| O13  | **Shared state.** Was ownership designed before lock choice?           | judgment | alternatives/lock invariant | global `Arc<Mutex<_>>`          | major    | choose owner           |
| O14  | **Lock scope.** Are external awaits/effects outside lock?              | judgment | code trace/test             | network call under lock         | critical | split critical section |
| O15  | **Interior mutability.** Is aliasing need explicit?                    | judgment | reentrancy/sync analysis    | `RefCell` to appease compiler   | major    | redesign ownership     |
| O16  | **Lifetime.** Does each lifetime express a real referent relation?     | judgment | signature explanation       | ornamental generics             | minor    | simplify               |
| O17  | **Task owner.** Who joins, cancels, and closes?                        | judgment | supervision tree            | detached resource task          | critical | structure tasks        |
| O18  | **External truth.** Are local and external authority claims separated? | judgment | non-guarantees              | local lease implies global lock | critical | revalidate/fence       |

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0003-R001`, `RUST-DOC-0003-R002`, `RUST-DOC-0003-R003`, `RUST-DOC-0003-R004`
- `RUST-DOC-0003-R005`, `RUST-DOC-0003-R006`, `RUST-DOC-0003-R007`, `RUST-DOC-0003-R008`
- `RUST-DOC-0003-R009`, `RUST-DOC-0003-R010`, `RUST-DOC-0003-R011`, `RUST-DOC-0003-R012`
- `RUST-DOC-0003-R013`, `RUST-DOC-0003-R014`
