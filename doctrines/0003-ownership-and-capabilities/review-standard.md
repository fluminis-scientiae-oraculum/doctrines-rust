# Review standard

Record pass, fail, not applicable, or waiver.

| Gate                | Question                                              | Pass evidence               | Failure example                 | Severity | Remediation            |
| ------------------- | ----------------------------------------------------- | --------------------------- | ------------------------------- | -------- | ---------------------- |
| Authority map       | Is every resource and operation owner named?          | custody/lifecycle map       | ambient service access          | major    | assign ownership       |
| Exclusivity         | Does exclusive domain authority have one local owner? | non-cloneable value         | copied commit token             | critical | consume or coordinate  |
| Issuance            | Can unauthorized code construct capability?           | restricted constructor      | public capability field         | critical | restrict issuer        |
| Least privilege     | Does capability expose only scoped operations?        | narrow methods/scope        | admin service inside token      | critical | split capabilities     |
| Clone               | Is every authority clone deliberate?                  | clone contract/test         | derive for convenience          | critical | remove or redefine     |
| Transfer            | Is delegation explicit and auditable?                 | move/delegation record      | hidden clone across tasks       | major    | model transfer         |
| Revocation          | Can stale authority be used?                          | recheck or lease            | perpetual session capability    | critical | bound validity         |
| Borrow              | Does borrowed access grant only required rights?      | receiver/lifetime audit     | read view exposes mutation      | major    | narrow borrow          |
| RAII                | Is drop limited to local/best-effort cleanup?         | explicit fallible close     | drop claims rollback            | critical | expose completion      |
| Secret debug        | Are formatting paths redacted?                        | trait/redaction tests       | derived debug token             | critical | custom debug           |
| Secret copies       | Are clone and serialization minimized?                | call-site inventory         | secret freely cloneable         | critical | scope exposure         |
| Zeroization         | Is claim limited to controlled buffers?               | guarantee ledger            | "all traces removed"            | major    | narrow claim           |
| Shared state        | Was ownership designed before lock choice?            | alternatives/lock invariant | global `Arc<Mutex<_>>`          | major    | choose owner           |
| Lock scope          | Are external awaits/effects outside lock?             | code trace/test             | network call under lock         | critical | split critical section |
| Interior mutability | Is aliasing need explicit?                            | reentrancy/sync analysis    | `RefCell` to appease compiler   | major    | redesign ownership     |
| Lifetime            | Does each lifetime express a real referent relation?  | signature explanation       | ornamental generics             | minor    | simplify               |
| Task owner          | Who joins, cancels, and closes?                       | supervision tree            | detached resource task          | critical | structure tasks        |
| External truth      | Are local and external authority claims separated?    | non-guarantees              | local lease implies global lock | critical | revalidate/fence       |

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0003-R001`, `RUST-DOC-0003-R002`, `RUST-DOC-0003-R003`, `RUST-DOC-0003-R004`
- `RUST-DOC-0003-R005`, `RUST-DOC-0003-R006`, `RUST-DOC-0003-R007`, `RUST-DOC-0003-R008`
- `RUST-DOC-0003-R009`, `RUST-DOC-0003-R010`, `RUST-DOC-0003-R011`, `RUST-DOC-0003-R012`
- `RUST-DOC-0003-R013`, `RUST-DOC-0003-R014`
