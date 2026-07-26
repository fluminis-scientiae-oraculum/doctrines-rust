# Boundary review

## Record

Apply separately to each HTTP/RPC, message, database, Serde, configuration,
filesystem, and FFI ingress/egress. Record **pass**, **fail**, **not
applicable**, or **waiver reference**.

## Inventory and layering

| ID | Question | Pass evidence |
|---|---|---|
| BR-01 | Is the boundary owner and threat/error model named? | boundary record |
| BR-02 | Are all raw bytes, metadata, and alternate sources inventoried? | input list |
| BR-03 | Is transport/physical parsing separate from domain validation? | layered conversion |
| BR-04 | Is a raw DTO/row/foreign type used where contracts differ? | representation map |
| BR-05 | Are trusted types constructed only after complete validation? | call trace |
| BR-06 | Are authentication and authorization separate transitions? | evidence path |
| BR-07 | Are cross-entity checks placed transactionally or in domain services? | enforcement location |
| BR-08 | Are egress DTOs deliberate rather than broad domain serialization? | output types |
| BR-09 | Are privileged administrative paths included in the inventory? | bypass list |
| BR-10 | Are cache, replay, restore, and migration paths included? | complete map |

## Parsing and resource limits

| ID | Question | Pass evidence |
|---|---|---|
| BR-11 | Is maximum raw input size enforced before large allocation? | limit/config test |
| BR-12 | Are decompression ratios and expanded size bounded? | decompression policy |
| BR-13 | Are nesting, field, element, and batch counts bounded? | parser limits |
| BR-14 | Are numeric conversions checked for range and units? | conversion code |
| BR-15 | Are text encoding and Unicode policies explicit? | parser policy |
| BR-16 | Are duplicate fields/headers/keys handled deliberately? | fixtures |
| BR-17 | Are paths protected from traversal and lossy conversion? | path policy |
| BR-18 | Are pointer null, length, alignment, and ownership checked at FFI? | FFI contract |
| BR-19 | Are time and size values represented with typed units? | DTO/domain types |
| BR-20 | Does malformed input return structured failure without panic? | negative tests |

## Construction and bypasses

| ID | Question | Pass evidence |
|---|---|---|
| BR-21 | Does Serde delegate to checked `TryFrom` or manual validation? | implementation |
| BR-22 | Does every database read validate trusted newtypes? | row conversions |
| BR-23 | Are derived decoders prevented from assigning trusted fields unchecked? | derive audit |
| BR-24 | Are unchecked `From` conversions absent for fallible evidence? | impl search |
| BR-25 | Are defaults prevented from inventing historical or verified facts? | default audit |
| BR-26 | Are partial projections named as partial types? | query/type review |
| BR-27 | Can test-only or feature-gated constructors ship? | feature matrix |
| BR-28 | Are unsafe layout/construction shortcuts absent or fully audited? | unsafe inventory |
| BR-29 | Are UI/client claims excluded from backend authority? | authorization trace |
| BR-30 | Does every bypass have a scoped, reviewed obligation? | escape-hatch ledger |

## Evolution, errors, and secrecy

| ID | Question | Pass evidence |
|---|---|---|
| BR-31 | Is long-lived representation versioned? | envelope/schema version |
| BR-32 | Is unknown-field policy deliberate? | reject/ignore/retain rationale |
| BR-33 | Is unknown enum/version behavior explicit? | compatibility tests |
| BR-34 | Are stable external tags independent of source rename? | encoding table |
| BR-35 | Is rolling old/new compatibility tested? | version matrix |
| BR-36 | Are syntax, validation, authority, conflict, availability, and unknown outcomes distinguishable as needed? | error model |
| BR-37 | Are source errors retained internally? | error chain |
| BR-38 | Are public diagnostics redacted and stable? | error mapping tests |
| BR-39 | Are secrets absent from logs, debug, metrics, and snapshots? | redaction audit |
| BR-40 | Are quarantine/dead-letter records access-controlled and retained safely? | operations policy |
| BR-41 | Are correlation IDs bounded and sensitivity-classified? | telemetry schema |
| BR-42 | Are credentials minimized across parsing copies? | secret data flow |

## Evidence and non-guarantees

| ID | Question | Pass evidence |
|---|---|---|
| BR-43 | Do tests cover valid and invalid boundary values? | fixtures |
| BR-44 | Do tests cover oversized and resource-hostile input? | adversarial cases |
| BR-45 | Do tests cross the real codec/driver/router/ABI where consequential? | integration suite |
| BR-46 | Do tests cover old and future/unknown values? | compatibility fixtures |
| BR-47 | Do fault tests cover partial effects and acknowledgement loss? | fault matrix |
| BR-48 | Are invalid historical records rejected or quarantined? | database evidence |
| BR-49 | Is fuzz/property evidence used where input space warrants it? | test record |
| BR-50 | Does the boundary ledger state what parsing proves? | guarantee entry |
| BR-51 | Does it state mutable external facts not proved? | non-guarantees |
| BR-52 | Are observation time, freshness, and reconciliation recorded? | evidence lifecycle |

## Exit criteria

Approval requires complete construction-path coverage, bounded resource use,
intentional evolution, redacted failures, real-boundary evidence proportional
to risk, and explicit non-guarantees.
