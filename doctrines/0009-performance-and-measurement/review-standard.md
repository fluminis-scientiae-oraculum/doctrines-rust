# Review standard

Mark every gate **pass**, **fail**, **not applicable**, or an approved **waiver
reference**.

| Gate | Question                                      | Pass evidence             | Failure example                         | Severity | Remediation            |
| ---- | --------------------------------------------- | ------------------------- | --------------------------------------- | -------- | ---------------------- |
| M01  | Is objective quantified?                      | metric and target         | “make faster”                           | critical | define outcome         |
| M02  | Is workload representative?                   | input distribution        | tiny synthetic only                     | critical | sample/model reality   |
| M03  | Is concurrency specified?                     | range and nominal load    | single-thread claim generalized         | high     | sweep                  |
| M04  | Are correctness constraints fixed?            | invariant list            | errors dropped for speed                | critical | restore semantics      |
| M05  | Is baseline commit identified?                | SHA/config                | vague prior version                     | high     | record                 |
| M06  | Is toolchain recorded?                        | version/profile/target    | debug vs release comparison             | critical | rebuild comparably     |
| M07  | Are features identical?                       | feature manifest          | dependency feature changed              | high     | normalize              |
| M08  | Is hardware/OS recorded?                      | environment summary       | different machines                      | high     | control or qualify     |
| M09  | Is frequency/thermal state considered?        | policy/monitoring         | throttled run                           | high     | stabilize              |
| M10  | Is environment noise measured?                | repeated baseline         | one shared-host sample                  | high     | repeat/control         |
| M11  | Was profiling performed?                      | relevant profile          | bottleneck guessed                      | high     | profile                |
| M12  | Does profile support target?                  | cost attribution          | optimized cold code                     | critical | redirect               |
| M13  | Is profiler overhead considered?              | comparison                | tracing dominates                       | medium   | sample/qualify         |
| M14  | Is benchmark work retained?                   | result consumption        | computation optimized away              | critical | black-box/consume      |
| M15  | Are constants controlled?                     | dynamic inputs            | compiler precomputes                    | critical | vary input             |
| M16  | Is setup located correctly?                   | methodology               | input allocation accidentally timed     | high     | separate/define        |
| M17  | Is teardown excluded/included deliberately?   | scope                     | destructor cost omitted unintentionally | high     | align claim            |
| M18  | Is warmup documented?                         | preparation               | first and steady mixed                  | high     | separate               |
| M19  | Is cache state documented?                    | cold/warm method          | filesystem cache unknown                | high     | control                |
| M20  | Are connections reused as intended?           | setup trace               | handshake accidentally excluded         | high     | match workload         |
| M21  | Is sample count sufficient?                   | framework/statistics      | one timing                              | critical | collect samples        |
| M22  | Is variability reported?                      | CI/error/dispersion       | point estimate only                     | high     | report                 |
| M23  | Is practical significance assessed?           | objective delta           | tiny statistical win                    | medium   | simplify               |
| M24  | Are p50/p95/p99 present for latency?          | distribution              | average only                            | critical | measure tails          |
| M25  | Are outliers explained rather than discarded? | policy                    | slow samples deleted                    | high     | analyze                |
| M26  | Is wall-clock distinguished from CPU?         | named metrics             | parallel run called cheaper             | high     | measure both           |
| M27  | Is aggregate CPU captured for parallel work?  | process/thread CPU        | only elapsed time                       | high     | record resource        |
| M28  | Is throughput paired with latency?            | load curve                | batching throughput only                | high     | report distribution    |
| M29  | Is saturation point identified?               | sweep                     | nominal point only                      | high     | load to overload       |
| M30  | Is overload behavior preserved?               | rejection/queue data      | unbounded queue inflates throughput     | critical | restore backpressure   |
| M31  | Is queue wait included?                       | end-to-end latency        | service time only                       | high     | include ingress        |
| M32  | Is downstream load measured?                  | DB/API metrics            | local speed overloads dependency        | critical | coordinate             |
| M33  | Are lock wait and hold measured?              | contention profile        | mutex blamed by count                   | high     | instrument             |
| M34  | Are allocations counted?                      | count/bytes               | clone syntax used as proof              | high     | measure                |
| M35  | Is peak and retained memory considered?       | heap/RSS profile          | fewer allocs retain huge buffer         | high     | measure lifetimes      |
| M36  | Is allocator identified?                      | environment               | cross-allocator comparison              | medium   | record                 |
| M37  | Is copy claim scoped?                         | data-flow                 | “zero-copy” broad claim                 | critical | enumerate copies       |
| M38  | Are lifetime/retention costs assessed?        | ownership analysis        | slice pins large buffer                 | high     | compare total          |
| M39  | Are serialization costs profiled?             | component trace           | iterator optimized instead              | high     | target boundary        |
| M40  | Are syscalls/round-trips counted?             | trace                     | source CPU blamed                       | high     | measure system path    |
| M41  | Are database plans and locks considered?      | query evidence            | local benchmark excludes DB             | high     | integrate              |
| M42  | Are network limits/rate behavior included?    | load trace                | unlimited fake server                   | high     | use realistic boundary |
| M43  | Is async described accurately?                | overlap/CPU data          | async means parallel                    | critical | narrow claim           |
| M44  | Is clone removal architecturally safe?        | ownership/contention      | introduces global mutex                 | critical | redesign/measure       |
| M45  | Does algorithmic complexity improve?          | input-size curve          | lower constant, worse growth            | high     | test sizes             |
| M46  | Are worst-case inputs represented?            | adversarial corpus        | average-only parser                     | high     | add                    |
| M47  | Does unsafe satisfy doctrine 0007?            | proof and tools           | unchecked indexing for tiny gain        | critical | remove/review          |
| M48  | Is unsafe benefit material?                   | safe baseline comparison  | no measurable difference                | critical | keep safe              |
| M49  | Are platform fallbacks measured?              | target matrix             | SIMD only one CPU                       | high     | test dispatch          |
| M50  | Is binary size measured where affected?       | artifact data             | generics assumed free                   | medium   | inspect                |
| M51  | Is compile time measured where affected?      | clean/incremental data    | macro cost ignored                      | medium   | time builds            |
| M52  | Is monomorphization assessed?                 | symbol/code-size evidence | generic explosion                       | medium   | simplify               |
| M53  | Is benchmark separate from correctness tests? | suite link                | benchmark is sole check                 | critical | add tests              |
| M54  | Are fault/error paths still tested?           | negative suite            | fast path bypasses validation           | critical | restore coverage       |
| M55  | Is regression metric stable enough?           | history/variance          | noisy shared runner hard gate           | high     | trend/dedicated host   |
| M56  | Is threshold above noise and meaningful?      | rationale                 | arbitrary one percent                   | high     | calibrate              |
| M57  | Does rerun policy avoid cherry-picking?       | aggregate policy          | keep fastest rerun                      | critical | predefine method       |
| M58  | Are commands reproducible?                    | checked-in harness/docs   | manual GUI steps only                   | high     | script                 |
| M59  | Are results retained with provenance?         | record/raw format         | PR says “much faster”                   | high     | attach evidence        |
| M60  | Is claim no broader than evidence?            | guarantee ledger          | microbench generalized                  | critical | narrow                 |

Critical failures block the performance claim and any complexity justified by
it. Correctness failures block the change itself.

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0009-R001`, `RUST-DOC-0009-R002`, `RUST-DOC-0009-R003`, `RUST-DOC-0009-R004`
- `RUST-DOC-0009-R005`, `RUST-DOC-0009-R006`, `RUST-DOC-0009-R007`, `RUST-DOC-0009-R008`
- `RUST-DOC-0009-R009`, `RUST-DOC-0009-R010`, `RUST-DOC-0009-R011`, `RUST-DOC-0009-R012`
- `RUST-DOC-0009-R013`, `RUST-DOC-0009-R014`, `RUST-DOC-0009-R015`, `RUST-DOC-0009-R016`
- `RUST-DOC-0009-R017`, `RUST-DOC-0009-R018`, `RUST-DOC-0009-R019`, `RUST-DOC-0009-R020`
