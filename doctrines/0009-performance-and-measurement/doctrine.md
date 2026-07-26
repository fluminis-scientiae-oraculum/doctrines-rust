# Normative doctrine

## RUST-DOC-0009-R001 — Define objective and workload

**Statement.** Optimization MUST begin with a quantified objective and a
workload representing the input distribution, concurrency, and system boundary
that matter.

**Intent.** Prevent work on irrelevant micro-costs.

**Applicability.** Performance changes, capacity plans, and regression gates.

**Allowed exceptions.** Removing an obviously unnecessary operation may proceed
as ordinary cleanup if no performance claim is made.

**Review evidence.** Metric, target, baseline, workload, and correctness
constraints.

## RUST-DOC-0009-R002 — Scope every performance claim

**Statement.** Performance claims MUST include environment, toolchain, build
profile, input distribution, concurrency, warmup/cache state, measurement
method, and comparison baseline sufficient for reproduction.

**Intent.** Make numbers interpretable and falsifiable.

**Applicability.** Documentation, pull requests, releases, and design decisions.

**Allowed exceptions.** A local exploratory note may be labeled preliminary and
must not support a merge claim.

**Review evidence.** Reproducible command, environment manifest, raw or
summarized samples, and commit identities.

## RUST-DOC-0009-R003 — Profile before optimizing

**Statement.** Profiling SHOULD precede nontrivial optimization and MUST precede
claims about a dominant bottleneck.

**Intent.** Direct effort to measured cost centers.

**Applicability.** Latency, CPU, allocation, contention, I/O, and size work.

**Allowed exceptions.** Algorithmic complexity defects apparent from complete
input bounds may be corrected without a profile, while still measuring outcome.

**Review evidence.** Flamegraph, trace, allocation profile, system metrics, or
equivalent relevant evidence.

## RUST-DOC-0009-R004 — Preserve correctness independently

**Statement.** A performance change MUST preserve domain invariants,
error/uncertainty semantics, security properties, and boundary validation, with
correctness evidence independent of the benchmark.

**Intent.** Reject faster incorrect behavior.

**Applicability.** All optimizations.

**Allowed exceptions.** An explicit product tradeoff may change semantics only
as a separately reviewed normative or API change, not as hidden optimization.

**Review evidence.** Invariant-linked tests and guarantee-ledger diff.

## RUST-DOC-0009-R005 — Defend benchmark execution

**Statement.** Benchmark code MUST prevent dead-code elimination, constant
folding, unintended setup measurement, and unrealistic reuse from invalidating
the intended workload.

**Intent.** Ensure measured work corresponds to the claim.

**Applicability.** Microbenchmarks and component benchmarks.

**Allowed exceptions.** None; framework facilities may provide the mechanism.

**Review evidence.** Input generation, black-boxing where appropriate,
setup/measurement separation, and result consumption.

## RUST-DOC-0009-R006 — Separate wall-clock and CPU claims

**Statement.** Measurements MUST distinguish wall-clock latency, CPU time, and
aggregate CPU consumption when their interpretations differ.

**Intent.** Prevent waiting and parallel work from being described as reduced
compute cost.

**Applicability.** Async, parallel, I/O-bound, and multi-process workloads.

**Allowed exceptions.** A single-threaded CPU-bound benchmark may report one
measure with its assumption stated.

**Review evidence.** Metric definition and collection method.

## RUST-DOC-0009-R007 — Report distributions

**Statement.** User-visible or service latency claims MUST report appropriate
distributions such as p50, p95, and p99 rather than only arithmetic averages.

**Intent.** Reveal tail behavior and multimodal workloads.

**Applicability.** Requests, queues, storage, and batch completion.

**Allowed exceptions.** Deterministic fixed-cost operations may use a narrow
summary after showing low variance.

**Review evidence.** Sample count, percentile method, confidence or variability,
and outlier policy.

## RUST-DOC-0009-R008 — Document warmup and cache state

**Statement.** Measurements MUST state process warmup, JIT or runtime
initialization where applicable, filesystem/page/cache state, connection reuse,
and dataset residency relevant to the claim.

**Intent.** Prevent cold and warm behavior from being mixed invisibly.

**Applicability.** Storage, network, serialization, and repeated services.

**Allowed exceptions.** A test may deliberately mix states only if the workload
distribution matches production and is documented.

**Review evidence.** Preparation sequence and separate cold/warm results where
both matter.

## RUST-DOC-0009-R009 — Measure allocation claims

**Statement.** Claims that code allocates less, performs no allocation, or
reduces memory MUST be supported by an allocator-aware measurement and MUST
identify retained as well as peak memory where relevant.

**Intent.** Avoid inferring allocation from syntax or clone count.

**Applicability.** Buffering, parsing, collections, async boxing, and caching.

**Allowed exceptions.** A direct removal of the only allocation call may be
noted structurally, but broader runtime claims still require measurement.

**Review evidence.** Allocation count/bytes, allocator, peak/resident set, and
workload.

## RUST-DOC-0009-R010 — Scope zero-copy claims

**Statement.** A zero-copy claim MUST identify every copy avoided within the
specified path and the lifetime, pinning, retention, fragmentation, API, and
ownership costs introduced.

**Intent.** Prevent one avoided copy from becoming a broad slogan.

**Applicability.** Parsers, networking, serialization, buffers, and FFI.

**Allowed exceptions.** None for the phrase "zero-copy."

**Review evidence.** Data-flow diagram, measured copy/allocation evidence, and
non-guarantees.

## RUST-DOC-0009-R011 — Do not equate async with speedup

**Statement.** Async concurrency MUST NOT be described as parallel CPU speedup
without evidence of parallel execution and a workload that benefits.

**Intent.** Distinguish overlap of waiting from reduced compute time.

**Applicability.** Runtime migrations, fan-out, and worker design.

**Allowed exceptions.** None for the claim; async may still improve resource
efficiency or concurrent latency.

**Review evidence.** Executor configuration, CPU utilization, throughput,
latency, and contention.

## RUST-DOC-0009-R012 — Make throughput/latency tradeoffs explicit

**Statement.** Batching, buffering, pipelining, and concurrency changes MUST
report both throughput and relevant latency/queue consequences.

**Intent.** Prevent aggregate gains from hiding worse tails or freshness.

**Applicability.** Brokers, databases, serializers, and service queues.

**Allowed exceptions.** Offline throughput-only jobs may state that latency has
no objective while still bounding resource use.

**Review evidence.** Batch/concurrency sweep and distribution results.

## RUST-DOC-0009-R013 — Measure contention and backpressure

**Statement.** Concurrent performance analysis MUST include queue depth, wait
time, saturation, lock or permit contention, rejection, and downstream load
where relevant.

**Intent.** Reveal whether local throughput shifts cost elsewhere.

**Applicability.** Shared state, pools, channels, and fan-out.

**Allowed exceptions.** Pure independent parallel work may document absence of
shared contention.

**Review evidence.** Contention profile, load curve, and overload behavior.

## RUST-DOC-0009-R014 — Count boundary costs

**Statement.** Performance investigations MUST consider serialization,
allocation, copies, syscalls, context switches, database queries, network
round-trips, and external rate limits before attributing cost solely to Rust
source constructs.

**Intent.** Optimize the actual end-to-end path.

**Applicability.** Integrated and service workloads.

**Allowed exceptions.** A deliberately isolated microbenchmark may narrow scope
and state that it excludes boundary cost.

**Review evidence.** Trace or component budget.

## RUST-DOC-0009-R015 — Review clone removal architecturally

**Statement.** Avoiding `clone` MUST NOT introduce worse algorithmic complexity,
excessive borrowing, global sharing, lock contention, or retention without
measurement and ownership analysis.

**Intent.** Prevent syntax-focused optimization from degrading architecture.

**Applicability.** Buffers, collections, async tasks, and shared state.

**Allowed exceptions.** Removal of a proven redundant clone with unchanged
ownership shape may be a local cleanup.

**Review evidence.** Data ownership, allocation profile, complexity, and
contention.

## RUST-DOC-0009-R016 — Govern unsafe optimization

**Statement.** Unsafe performance changes MUST satisfy RUST-DOC-0007 and MUST
show a material measured benefit under the target workload.

**Intent.** Charge proof risk to the benefit it buys.

**Applicability.** Unchecked indexing, custom allocation, SIMD, FFI, and
lock-free code.

**Allowed exceptions.** Unsafe may be necessary for an external API even when
performance is not its justification; that case is not an optimization claim.

**Review evidence.** Safe baseline, benchmark, profile, safety proof, and
specialized tests.

## RUST-DOC-0009-R017 — Automate stable regressions

**Statement.** Regression thresholds SHOULD be automated only for metrics whose
environmental variance is measured and whose threshold includes a justified
noise budget.

**Intent.** Catch real regressions without normalizing noisy gates.

**Applicability.** CI benchmarks, binary-size checks, allocations, and compile
time.

**Allowed exceptions.** Noisy metrics may run as trend reports or on controlled
dedicated hosts.

**Review evidence.** Baseline history, variance, threshold, hardware stability,
and rerun policy.

## RUST-DOC-0009-R018 — Do not generalize microbenchmarks

**Statement.** Microbenchmark results MUST NOT be generalized to end-to-end
performance without evidence connecting the measured operation to overall
workload contribution.

**Intent.** Prevent large local ratios from masking tiny system impact.

**Applicability.** Library and application optimization claims.

**Allowed exceptions.** A microbenchmark may establish the cost of the exact
isolated primitive it measures.

**Review evidence.** Profile share, integrated benchmark, or component budget.

## RUST-DOC-0009-R019 — Account for build and binary cost

**Statement.** Abstraction choices involving generics, code generation, feature
sets, or dependencies SHOULD assess compile time, monomorphization, binary size,
incremental behavior, and diagnostic cost when material.

**Intent.** Treat developer and deployment resources as performance dimensions.

**Applicability.** Public generic APIs, macro-heavy code, and constrained
artifacts.

**Allowed exceptions.** Small local code with immaterial measured impact may
document no concern.

**Review evidence.** Build timing, artifact sections, generic instantiations, or
dependency analysis.

## RUST-DOC-0009-R020 — Retain reproducible evidence

**Statement.** Accepted performance decisions MUST retain commands, commits,
configuration, result summaries, and raw-data location or format sufficient to
repeat or challenge the result.

**Intent.** Make optimization decisions durable and auditable.

**Applicability.** Merged performance changes and release claims.

**Allowed exceptions.** Sensitive production traces may be retained in
controlled storage with a sanitized reproducible summary.

**Review evidence.** Benchmark record and provenance.
