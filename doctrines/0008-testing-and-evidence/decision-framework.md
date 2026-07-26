# Decision framework

## Map invariant to evidence

For every invariant record:

1. enforcement mechanism;
2. legal construction or transition;
3. prohibited path;
4. boundary where evidence enters;
5. external failure points;
6. concurrency or persistence risks;
7. primary test class;
8. independent supporting evidence;
9. residual risk.

## Select test classes

| Claim shape | Primary evidence |
|---|---|
| public API cannot express misuse | compile-fail test |
| constructor accepts/rejects specified values | unit/table tests |
| law holds across broad generated values | property test |
| parser and serializer agree with format | fixtures, properties, contract tests |
| database conversion preserves invariant | real integration test |
| consumer tolerates replay | duplicate/fault-injection test |
| small concurrent protocol preserves state | model checking plus unit tests |
| unsafe operation respects memory rules | proof plus Miri/sanitizers/fuzzing |
| end-to-end workflow recovers from crash | fault-injected system test |
| deployed workload meets expectation | telemetry plus performance evidence |

Use more than one layer when a claim crosses layers.

## Boundary-value design

Partition the input domain:

- minimum and maximum accepted;
- just below and above each bound;
- empty and zero;
- malformed structure;
- valid syntax but rejected policy;
- normalization collisions;
- Unicode and encoding cases;
- unknown versions/variants;
- oversized and deeply nested;
- duplicate and reordered;
- stale version;
- cancelled at each partial step.

Assert structured categories and retained evidence, not only `is_err()`.

## Property-test design

Define the property in domain language before generator code. Ensure generators
cover valid and invalid partitions, avoid excessive rejection, record failing
seeds, and shrink to interpretable cases. Compare against an independent model
or stable specification when possible. Bound sizes explicitly so execution
cost and unexplored regions are known.

## Compile-fail decision

Use a compile-fail test when:

- privacy prevents direct trusted construction;
- ownership consumption prevents handle reuse;
- typestate prevents an illegal operation;
- capability types restrict authority;
- trait bounds intentionally exclude a class.

Keep each failing source minimal. Verify the diagnostic points to the intended
rule. Update expected output only after semantic review on the pinned toolchain.

## Double fidelity decision

For each double, compare:

| Real behavior | Double behavior | Gap owner |
|---|---|---|
| latency/cancellation | controlled delay or instant | named integration suite |
| capacity/backpressure | bounded or unlimited | overload suite |
| transaction/isolation | real or simplified | database tests |
| duplicate/order | configurable or perfect | messaging fault tests |
| unknown outcome | representable or binary | distributed suite |
| schema/version | actual codec or hand-built values | contract suite |

If the double erases the very risk under test, replace it.

## Flaky-test procedure

1. retain the first failure signature and full reproducibility data;
2. classify shared state, time, schedule, randomness, resource, and external
   dependencies;
3. reproduce with fixed seed or controlled schedule;
4. determine whether product or harness owns the nondeterminism;
5. fix the cause;
6. use temporary quarantine only with owner and expiry;
7. remove retries that mask the resolved class.

## Stop conditions

Stop approval when:

- tests have no claim mapping;
- only positive construction is tested;
- a mock removes critical failure semantics;
- concurrency ordering depends on sleep;
- fault tests fail only before any effect;
- compiler output was bulk accepted;
- snapshot change lacks meaning analysis;
- flaky failure is resolved solely by retries;
- coverage percentage is the primary quality argument;
- production absence of incidents is described as proof.
