# Review standard

Mark each gate **pass**, **fail**, **not applicable**, or with an approved
**waiver reference**. Safety-contract failures cannot be waived into soundness.

| Gate | Question                                        | Pass evidence                       | Failure example                         | Severity | Remediation               |
| ---- | ----------------------------------------------- | ----------------------------------- | --------------------------------------- | -------- | ------------------------- |
| U01  | Is unsafe necessary?                            | safe alternatives and measured need | borrow checker bypass                   | critical | redesign safely           |
| U02  | Is unsafe inventory complete?                   | tool/search inventory               | macro-generated unsafe missed           | critical | enumerate                 |
| U03  | Is lexical scope minimal?                       | small block                         | whole function marked unsafe            | high     | narrow block              |
| U04  | Is API visibility minimal?                      | private module/helper               | raw constructor public                  | critical | encapsulate               |
| U05  | Does every block state invariant?               | `SAFETY:` argument                  | “pointer seems valid”                   | critical | write proof               |
| U06  | Does comment cover each operation?              | operation-to-premise mapping        | one generic comment                     | critical | split or expand           |
| U07  | Are safe callers adversarially considered?      | call-sequence analysis              | intended use only                       | critical | test full safe surface    |
| U08  | Are hidden caller obligations absent?           | signature enforces rules            | safe method says “must not call twice”  | critical | encode/check/mark unsafe  |
| U09  | Does unsafe API have `# Safety` docs?           | complete section                    | caller obligations omitted              | critical | document                  |
| U10  | Are obligations non-circular?                   | concrete predicates                 | “call only when safe”                   | critical | specify facts             |
| U11  | Is pointer origin known?                        | allocation/foreign provenance       | integer address guessed                 | critical | trace origin              |
| U12  | Is nullability checked?                         | check or non-null contract          | dereference nullable result             | critical | validate                  |
| U13  | Is alignment established?                       | layout or runtime check             | byte offset cast blindly                | critical | align/copy                |
| U14  | Are bounds and overflow checked?                | checked arithmetic                  | length multiplication wraps             | critical | checked operations        |
| U15  | Is dereferenceability established?              | live allocation range               | pointer only numerically in range       | critical | prove allocation          |
| U16  | Is initialization tracked?                      | progress state                      | assume-init before complete             | critical | guard                     |
| U17  | Is typed validity established?                  | bit-pattern validation              | arbitrary byte as bool                  | critical | remain untyped            |
| U18  | Are enum discriminants valid?                   | stable conversion                   | transmute integer to enum               | critical | checked match             |
| U19  | Is aliasing permitted?                          | reference graph                     | mutable and shared references overlap   | critical | shorten/restructure       |
| U20  | Is lifetime bounded by owner?                   | custody proof                       | forged static reference                 | critical | return owned/short borrow |
| U21  | Is reallocation considered?                     | capacity/pinning proof              | reference held across vector push       | critical | avoid movement            |
| U22  | Are zero-sized types handled?                   | explicit case                       | pointer increment assumes size          | high     | account for ZST           |
| U23  | Is one-past-end use valid?                      | arithmetic proof                    | dereference end pointer                 | critical | correct bounds            |
| U24  | Are integer-pointer conversions justified?      | supported provenance API            | round-trip assumed universally valid    | critical | use supported operations  |
| U25  | Is every initialized value dropped once?        | guard and counters                  | panic leaks/double drops                | critical | track prefix              |
| U26  | Is error cleanup sound?                         | failure tests                       | partial FFI output leaked               | critical | cleanup guard             |
| U27  | Is panic cleanup sound?                         | injected panic                      | callback panic corrupts collection      | critical | use repair guard          |
| U28  | Is transmute unavoidable?                       | narrower alternatives rejected      | convenience cast                        | critical | replace                   |
| U29  | Are transmute sizes/layouts proven?             | primary contract/assertion          | current compiler observation            | critical | establish or remove       |
| U30  | Is ownership preserved across bit cast?         | drop analysis                       | duplicated owned pointer                | critical | use safe conversion       |
| U31  | Is FFI ABI exact?                               | header/spec match                   | default ABI assumed                     | critical | correct declaration       |
| U32  | Is representation stable?                       | applicable `repr` contract          | Rust layout exported                    | critical | boundary type             |
| U33  | Are foreign lengths in correct units?           | bytes/elements contract             | byte length used as elements            | critical | convert checked           |
| U34  | Is FFI ownership explicit?                      | boundary table                      | unclear who frees                       | critical | define lifecycle          |
| U35  | Is allocator pairing correct?                   | matching free function              | Rust frees C allocation                 | critical | return to origin          |
| U36  | Is string encoding explicit?                    | conversion policy                   | UTF-8 assumed from C                    | high     | validate                  |
| U37  | Are callbacks lifetime-safe?                    | registration/unregistration proof   | stack context retained                  | critical | own context               |
| U38  | Are callback threads known?                     | foreign contract                    | thread-affine state accessed anywhere   | critical | marshal/synchronize       |
| U39  | Is reentrancy handled?                          | state-machine analysis              | callback reenters mutable borrow        | critical | guard/design              |
| U40  | Is unwind policy explicit?                      | catch/abort/ABI contract            | panic crosses C ABI                     | critical | contain unwind            |
| U41  | Is foreign exception behavior known?            | source contract                     | exception crosses Rust unknowingly      | critical | wrapper boundary          |
| U42  | Does unsafe `Send` cover all fields?            | concurrency proof                   | raw pointer ignored                     | critical | prove or remove           |
| U43  | Does unsafe `Sync` cover shared methods?        | alias/mutation proof                | foreign handle not thread-safe          | critical | restrict                  |
| U44  | Is drop thread behavior valid?                  | destruction contract                | must free on creator thread             | critical | enforce affinity          |
| U45  | Are atomics ordered by invariant?               | happens-before proof                | folklore ordering                       | critical | prove/use lock            |
| U46  | Does safe abstraction remain sound after panic? | state and drop evidence             | poison ignored                          | critical | isolate invalid state     |
| U47  | Has Miri run where supported?                   | command/result                      | no dynamic UB evidence                  | high     | run or explain            |
| U48  | Have relevant sanitizers run?                   | target results                      | concurrency code untested               | high     | add evidence              |
| U49  | Is fuzzing aimed at boundary invariants?        | corpus/property                     | only fixed examples                     | medium   | fuzz                      |
| U50  | Are tool blind spots recorded?                  | evidence limits                     | clean run called proof                  | high     | qualify                   |
| U51  | Are unsafe dependencies inventoried?            | dependency audit                    | transitive FFI crate ignored            | high     | review                    |
| U52  | Are advisories and maintenance current?         | audit evidence                      | abandoned critical crate                | high     | update/replace            |
| U53  | Are target assumptions tested?                  | target matrix                       | only developer architecture             | high     | cross-test                |
| U54  | Is performance justification measured?          | benchmark/profile                   | “faster” assertion                      | high     | measure                   |
| U55  | Is re-audit trigger documented?                 | assumption list                     | compiler upgrade ignored                | high     | define trigger            |
| U56  | Does guarantee ledger state non-guarantees?     | completed ledger                    | safe wrapper claims foreign correctness | critical | narrow claim              |

Approval requires a reviewer competent in the relevant unsafe domain. Tool
success cannot compensate for an incomplete safety argument.

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0007-R001`, `RUST-DOC-0007-R002`, `RUST-DOC-0007-R003`, `RUST-DOC-0007-R004`
- `RUST-DOC-0007-R005`, `RUST-DOC-0007-R006`, `RUST-DOC-0007-R007`, `RUST-DOC-0007-R008`
- `RUST-DOC-0007-R009`, `RUST-DOC-0007-R010`, `RUST-DOC-0007-R011`, `RUST-DOC-0007-R012`
- `RUST-DOC-0007-R013`, `RUST-DOC-0007-R014`, `RUST-DOC-0007-R015`, `RUST-DOC-0007-R016`
- `RUST-DOC-0007-R017`, `RUST-DOC-0007-R018`
