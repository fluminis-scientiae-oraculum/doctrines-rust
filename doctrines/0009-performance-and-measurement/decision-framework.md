# Decision framework

## Define the objective

Record:

- user or system outcome;
- metric and unit;
- baseline commit;
- target or regression budget;
- representative input distribution;
- concurrency and batch range;
- correctness and resource constraints;
- deployment environment;
- decision deadline.

If no decision changes with the result, do not optimize yet.

## Choose observation

| Question | Observation |
|---|---|
| Where is CPU spent? | sampled/instrumented CPU profile |
| What waits? | async trace, span timing, syscall trace |
| What allocates? | allocation count/bytes and heap profile |
| Why are tails slow? | percentile trace correlated with inputs/queues |
| Is a lock saturated? | lock wait/hold profile and concurrency sweep |
| Is storage dominant? | query plan, round-trips, I/O and durability timing |
| Is network dominant? | request trace, payload size, retransmit/rate limits |
| What grows the binary? | section/symbol/generic analysis |
| What slows builds? | clean/incremental timing and compiler timings |

Collect a baseline before modifying code.

## Design the benchmark

1. Build with the intended profile and features.
2. Fix or record toolchain and target.
3. Select representative and adversarial inputs.
4. Decide whether setup and allocation belong in the measured operation.
5. make input unavailable for unintended constant folding;
6. consume output;
7. define warmup and cache state;
8. collect enough samples for expected variance;
9. record environment noise and thermal/frequency policy;
10. retain commands and summarized data.

Validate benchmark outputs against correctness tests.

## Interpret changes

Ask:

- Is the difference larger than measured noise?
- Is it large enough to matter to the objective?
- Does the profile show the changed code contributes enough?
- Did total CPU, memory, or downstream load worsen?
- Did p95/p99 change differently from the mean?
- Did input distribution or cache state shift?
- Did compiler output or features differ?
- Did correctness, failure, or backpressure behavior change?

If the benchmark improves but the integrated workload does not, narrow the
claim to the primitive or reject the complexity.

## Concurrency sweep

Measure at minimum:

```text
concurrency: 1 → nominal → saturation → overload
```

At each point capture throughput, p50/p95/p99, queue depth, wait time,
rejection, timeouts, CPU, memory, and downstream utilization. Sweep batch size
where batching changes. Choose capacity before the collapse region with
operational margin.

## Optimization choice

| Measured bottleneck | Candidate direction | Correctness check |
|---|---|---|
| algorithmic complexity | better data structure/algorithm | ordering, limits, worst case |
| allocation churn | reuse, ownership change, compact representation | retention, aliasing |
| serialization | format/configuration/buffering | compatibility, validation |
| syscall/round-trip | batching or pipelining | partial failure, latency |
| lock contention | ownership partitioning or shorter scope | invariant atomicity |
| cache misses | layout/locality change | representation validity |
| monomorphization | dispatch/API simplification | behavior and object safety |
| compile time | dependency/features/generic reduction | runtime and diagnostics |

## Unsafe gate

Proceed toward unsafe only if:

1. safe baseline is correct and profiled;
2. bottleneck is material;
3. safe alternatives were measured or rejected;
4. expected gain changes the objective;
5. safety invariant is reviewable;
6. Miri/sanitizer/target evidence is feasible;
7. fallback and re-audit triggers exist.

Otherwise keep the safe implementation.

## Regression strategy

Use a blocking threshold for stable allocation counts, artifact sizes, or
dedicated-host timings with measured variance. Use trend reporting for noisy
shared-host timings. Always retain correctness gates. A rerun policy must not
select only favorable samples.

## Stop conditions

Stop when:

- objective or workload is undefined;
- only debug-build or single-run numbers exist;
- profile contradicts the proposed bottleneck;
- benchmark result is unused or constant-folded;
- average hides material tail regression;
- local improvement increases downstream load;
- clone removal creates global sharing without evidence;
- zero-copy scope is unspecified;
- unsafe gain is immaterial;
- result depends on unrecorded environment differences;
- microbenchmark ratio is claimed end to end without contribution analysis.
