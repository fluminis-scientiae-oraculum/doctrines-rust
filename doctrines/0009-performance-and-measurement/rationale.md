# Rationale

## Performance has a workload

"Faster" is incomplete. An implementation can be faster for tiny ASCII inputs
and slower for large Unicode data; improve throughput at low concurrency and
collapse under contention; reduce average latency while worsening p99; save CPU
but increase memory retention. The objective selects the metric and the
workload establishes relevance.

A useful objective resembles: process the production-sized message distribution
at a stated concurrency with p99 latency below a target, while limiting peak
memory and preserving rejection and ordering semantics. It creates a decision
criterion rather than an aesthetic preference.

## Profile before explanation

Source inspection suggests hypotheses, not cost attribution. A clone may be
optimized away or insignificant next to a database round-trip. A parser may be
CPU-heavy only because decompression dominates. A mutex may show little
contention until a downstream service slows and the critical section expands.

CPU profiles, allocation traces, async spans, system-call traces, database query
plans, and network observations answer different questions. Choose the profiler
that can observe the suspected resource and verify that sampling overhead or
instrumentation does not change the conclusion materially.

## Benchmark discipline

Optimizing compilers can remove work whose result is unused, hoist constants,
or precompute predictable inputs. Benchmarks should generate or select inputs
outside the timed path, consume results, and prevent unrealistic knowledge
without hiding real optimization opportunities. Setup, teardown, allocation,
and cloning belong inside or outside measurement according to the claim.

Benchmark processes also share hosts with frequency scaling, thermal limits,
interrupts, background work, and virtual-machine noise. Record environment,
repeat samples, and compare like with like. A statistically significant small
difference may still be operationally irrelevant; a large difference under a
nonrepresentative input may also be irrelevant.

## Wall-clock, CPU, and throughput

Wall-clock latency includes waiting. CPU time measures compute consumed by one
or more threads. Parallel code can reduce wall time while consuming more total
CPU. Async code can improve concurrency by yielding during I/O without making
one operation's CPU work faster. Throughput can increase by batching while
individual items wait longer.

State the desired resource. A service constrained by CPU cost may reject a
change that improves latency through excessive parallelism. A latency-critical
batch may accept more CPU within capacity.

## Distributions and tails

Averages hide skew. A service with most requests at one millisecond and a small
group at one second can have an acceptable-looking mean but severe customer
impact. p50 describes central behavior; p95 and p99 expose progressively rarer
tails. Percentiles need sufficient samples and an explicit aggregation method,
especially across hosts or time windows.

Tail analysis should correlate with input size, tenant, cache state, queue wait,
retries, and downstream calls. Discarding outliers requires a methodological
reason; those observations may be the failure mode.

## Warm and cold systems

First-use costs include process initialization, page faults, DNS, connection
setup, TLS, allocator state, filesystem cache, and database plan/cache behavior.
Long-running steady-state services care about warm operation but also experience
deploy and failover cold starts. Command-line tools may be dominated by startup.

Measure the state the user experiences. If both matter, report both rather than
mixing an unspecified proportion.

## Allocation and retention

Source-level `clone` may increment a reference count, copy a small scalar, or
allocate and copy a large buffer. Conversely, eliminating a clone can extend a
borrow, retain a large backing buffer for one small slice, or introduce shared
locking. Allocation counts, allocated bytes, peak resident memory, and retained
memory answer different questions.

Arena and pool designs reduce allocation frequency but can increase peak memory
and latency spikes. Measure cleanup and long-lived fragmentation under realistic
lifetimes.

## Zero copy and lifetime cost

Data may be copied by kernel-to-user transfer, buffering, parsing, decoding,
normalization, ownership conversion, or serialization. A zero-copy parser might
avoid one application copy while requiring the entire input buffer to stay
alive. Scatter/gather I/O may avoid concatenation but complicate APIs and system
calls. DMA and kernel mechanisms have platform-specific boundaries.

Name the exact path and copy removed. Measure end-to-end effect and account for
retention, pinning, fragmentation, and caller ergonomics.

## Contention, queues, and backpressure

Increasing concurrency initially hides waiting, then saturates CPU, locks,
connections, database capacity, network, or a remote rate limit. Beyond that
point, queues increase latency and memory while throughput stays flat or falls.
A benchmark that measures only completed throughput and drops rejected work can
misrepresent service quality.

Sweep concurrency and batch size. Record queue depth, wait time, rejection,
timeouts, and downstream utilization. RUST-DOC-0004 requires bounded admission;
performance work must not remove the safety valve to inflate a benchmark.

## Boundary costs dominate often

Serialization can allocate, validate, and copy. Small database queries can incur
network round-trips and lock waits. Filesystem durability can require sync
operations. Logging can format and write synchronously. System calls and context
switches can dominate small computations.

Micro-optimizing an iterator has little effect if it represents one percent of
the profile. Batching queries or eliminating a round-trip may matter more, while
also changing consistency and error behavior that must be reviewed.

## Unsafe and correctness

Unchecked indexing can remove a branch in source yet produce no measurable gain
after compiler bounds-check elimination. Custom lock-free structures can
increase proof cost and regress under contention. SIMD may require alignment,
target detection, and fallback. Unsafe is justified only after a safe baseline,
material measured gain, and complete RUST-DOC-0007 evidence.

The benchmark itself cannot detect undefined behavior reliably and cannot prove
semantic equivalence. Correctness tests remain separate.

## Build-time performance

Heavy generics and macros can improve runtime but increase compile time,
monomorphized code, binary size, instruction-cache pressure, and diagnostics.
Feature unification can pull unused capabilities into tools. Dynamic dispatch
can reduce code size while adding an indirect call. Measure the dimension that
matters rather than applying slogans.

## Regression gates

Shared CI hosts are noisy. A strict one-percent wall-time gate may fail
randomly, training maintainers to rerun or ignore it. Stable metrics such as
binary bytes or allocation counts can support tight thresholds. Timing gates
need controlled hardware, historical variance, sufficient samples, and a
threshold above noise. Trend reports may be better on ordinary CI.

## Performance guarantee ledger

| Claim | Workload/environment | Established by | Does not prove | Residual risk |
|---|---|---|---|---|
| parser reduced allocation bytes | named corpus, allocator, release build | allocation profile and benchmark | lower end-to-end latency | corpus drift |
| batching raises throughput | concurrency sweep, real database | load test | improved p99 latency | production query mix |
| async version overlaps I/O | executor trace and utilization | integrated benchmark | parallel CPU speedup | runtime contention |
| binary shrank | identical features/toolchain/target | artifact measurement | faster startup | compression/deployment variance |
| unsafe path is materially faster | safe baseline and representative samples | profile plus benchmark | soundness | target and compiler changes |

## Proportionality

Measurement has cost. A low-risk readability change need not create a laboratory
benchmark. A claim affecting capacity, infrastructure spend, user latency, or
unsafe design deserves reproducible evidence. Preserve raw results only as long
as useful and keep sanitized summaries reviewable. The desired outcome is not
maximum benchmark sophistication; it is a trustworthy decision.
