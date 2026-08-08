# Rationale

## Evidence hierarchy

The following layers are complementary rather than a strict ranking:

| Evidence             | Supports                                                        | Does not establish                                 |
| -------------------- | --------------------------------------------------------------- | -------------------------------------------------- |
| compiler rejection   | a specific program cannot type-check under tested API/toolchain | runtime correctness or all prohibited programs     |
| type checking        | accepted code satisfies language and trait constraints          | domain truth or external behavior                  |
| unit test            | local behavior for selected inputs                              | boundary integration or full input space           |
| property test        | a property over generated cases                                 | mathematical universality outside generation/model |
| compile-fail test    | important misuse remains rejected                               | runtime failure handling                           |
| integration test     | behavior across instantiated components                         | every deployment or failure                        |
| contract test        | agreed protocol examples and compatibility                      | provider implementation correctness everywhere     |
| fault injection      | recovery at selected failure points                             | all timing and correlated failures                 |
| model checking       | modeled schedules within stated bounds                          | unmodeled code, inputs, or unbounded executions    |
| production telemetry | observed deployed behavior                                      | invisible failures or workloads not seen           |
| incident evidence    | a real failure mechanism and consequence                        | absence of other mechanisms                        |

Tests become persuasive when their scope matches the claim and independent
layers agree.

## Invariants make test selection concrete

An evidence plan therefore starts from the invariant inventory, not from test
framework preferences.

## Negative tests protect the boundary

Happy paths demonstrate admission, but trusted types are defined equally by
what they exclude. Boundary values, malformed encodings, contradictory row
states, unknown variants, oversized input, and unauthorized transitions should
produce structured rejection. Tests should assert the category callers use, not
fragile full wording unless wording is itself a contract.

Compile-fail tests are valuable when the claim is "this program cannot be
expressed through the public API." Their diagnostic snapshots are evidence that
must be interpreted. A changed compiler can alter wording while the prohibition
remains, or the program can still fail for an unrelated import error while the
actual protection vanished.

## Properties explore shape, not infinity

Property testing can generate many combinations and shrink failures to useful
examples. The generator defines the explored universe. If it never creates
Unicode edge cases, large lengths, or invalid state sequences, clean runs say
nothing about them. Properties also need an independent oracle; asserting that
encoding followed by the same flawed decoder returns something can preserve a
shared defect.

## Real boundaries matter

An in-memory repository may accept values a database rejects, ignore isolation,
or provide instantaneous consistency. A mock HTTP client may never delay after
remote execution. A fake broker may deliver each message once in order. These
doubles are useful for local logic only if broader tests cover the omitted
semantics.

Contract tests verify schemas and semantic categories across deployment
boundaries. Real integration tests reveal encoding, configuration, driver, and
transaction behavior. Neither guarantees the remote service will behave
forever, so monitoring and compatibility ownership remain.

## Concurrency needs controlled schedules

Sleeping creates a hope that another task progressed. Slow or fast CI hosts can
violate that timing. Barriers, channels, deterministic executors, paused clocks,
and observed events make the desired ordering explicit. Loom can enumerate
possible schedules for a small modeled protocol and detect invariant failures
that stress tests seldom encounter.

Models require scrutiny. Replacing a production primitive with a simplified one
can omit behavior. State bounds may exclude long histories. Still, a carefully
matched small model supplies stronger schedule evidence than thousands of
uncontrolled repetitions.

## Fault injection targets the spaces between steps

Many distributed defects occur between successful steps: after effect before
acknowledgement, after domain commit before publish, after partial file write
before rename, or after request dispatch before response. Returning an error
from the first call in a mock does not exercise these states.

A crash-point matrix names each durable boundary and expected recovery. Tests
then stop or fail at each point, restart the component, and verify invariant,
duplicate handling, and unknown outcome. Delay and reordering tests expose
timeouts and stale-observation assumptions.

## Snapshots require semantic ownership

Snapshots are useful for large structured outputs and compiler diagnostics.
Their danger is an easy update command that converts all current output into
expected output. Review should classify changes: intended semantic change,
stable formatting migration, environment noise, or unexpected regression.
Nondeterministic IDs, timestamps, and paths should be normalized at the source
or represented deliberately.

Compiler UI snapshots demand particular care. The test passes if output matches,
even when the failing cause no longer demonstrates the intended privacy or type
rule. Inspect the actual diagnostic.

## Flakiness is a system observation

Flaky tests can reveal races, leaked state, clock assumptions, resource
exhaustion, unstable external dependencies, and insufficient isolation.
Automatic retries improve short-term pipeline throughput but destroy frequency
and signature evidence if used alone. A temporary quarantine needs a visible
owner and deadline, with captured seeds, traces, environment, and timing.

If the root cause is genuinely external instability, the product's behavior
under that instability may also need design work.

## Coverage and mutation

Line and branch coverage reveal code that tests did not execute. They do not
show whether assertions would detect a defect, whether input partitions are
meaningful, or whether concurrency schedules occurred. Mutation testing can
provide additional evidence that assertions detect selected changes, but the
mutation set is also a model.

Use coverage to find gaps after mapping invariants, not as the definition of
quality.

## Production and incidents

Telemetry can validate workload distributions, error frequencies, latency,
queue saturation, and reconciliation age. It is strongest when detection
mechanisms are themselves tested. Silent data corruption or missing events may
produce no metric. An incident supplies high-authority evidence that one
failure mechanism is real; it should produce a regression test, fault scenario,
or doctrine correction where appropriate.

## Proportionality

Every test type has cost: environment maintenance, execution time, flaky
surface, and diagnostic work. A pure formatter does not need distributed fault
injection. A payment capture does. Select the smallest evidence portfolio that
addresses consequential failure risks, then state uncovered assumptions
honestly.
