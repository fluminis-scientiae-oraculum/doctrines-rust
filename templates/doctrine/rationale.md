# Rationale: <Replace with doctrine title>

## Failure modes

<Describe concrete legal programs, boundary inputs, interleavings, migrations,
or external failures that motivate the doctrine. Trace each to normative rule
IDs.>

## Why weaker alternatives fail

<Compare plausible alternatives fairly. Explain exactly which invariant,
evidence, recovery, or review property they omit. Do not reject a mechanism
because it is unfashionable.>

## Interaction with external reality

<Separate local type evidence from mutable external facts. Explain timeouts,
staleness, cancellation, persistence, distributed execution, and reconciliation
where relevant.>

## Costs and overapplication

<Cover public API size, generics, diagnostics, serialization, persistence,
dynamic dispatch, compile time, monomorphization, binary size, migration,
runtime cost, and team familiarity as applicable. Give examples where plain code
or a runtime enum is better.>

## Guarantee ledger

| Claim         | Established by                                   | Protected construction | Boundary preservation     | Escape hatches    | Does not prove   | Residual runtime risk |
| ------------- | ------------------------------------------------ | ---------------------- | ------------------------- | ----------------- | ---------------- | --------------------- |
| <Exact claim> | <Constructor, transition, protocol, or evidence> | <Visibility/authority> | <Decode/persistence path> | <Privileged path> | <Excluded facts> | <Failure/uncertainty> |

Add enough rows to cover each major type, transition, authority, persisted fact,
and external outcome.

## Evidence limits

<State what compiler rejection, tests, model checking, measurements, telemetry,
and incidents do and do not establish for this doctrine.>
