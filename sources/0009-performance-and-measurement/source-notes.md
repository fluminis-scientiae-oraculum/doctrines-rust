# RUST-DOC-0009 source notes

## Primary and authoritative tool sources

The [Criterion.rs book](https://bheisler.github.io/criterion.rs/book/) documents
statistical Rust benchmarking. Standard-library
[`black_box`](https://doc.rust-lang.org/std/hint/fn.black_box.html) documents an
optimization barrier and its limitations. The
[Cargo profiles reference](https://doc.rust-lang.org/cargo/reference/profiles.html)
defines build configuration affecting benchmark comparability.

The Rust project publishes compiler performance through
[perf.rust-lang.org](https://perf.rust-lang.org/) and the
[`rustc-perf`](https://github.com/rust-lang/rustc-perf) repository, illustrating
workload and environment-controlled regression tracking. Linux
[`perf` documentation](https://perf.wiki.kernel.org/index.php/Main_Page) and
[FlameGraph](https://github.com/brendangregg/FlameGraph) document widely used
profiling mechanics. Tokio
[`RuntimeMetrics`](https://docs.rs/tokio/latest/tokio/runtime/struct.RuntimeMetrics.html)
provides runtime-specific observations.

## Accepted ideas

The doctrine accepts defined workloads, baselines, repeated measurement,
statistical variability, profiling, and reproducible configuration as the basis
for performance claims. It accepts p50/p95/p99 for latency distributions,
allocation and memory profiling, concurrency/load sweeps, and stable automated
regressions where the environment supports them.

Compiler barriers and benchmark frameworks help prevent measurement distortion,
but the benchmark author remains responsible for setup, input, result
consumption, and workload relevance.

## Refined ideas

"Rust is fast" is refined to language capabilities plus a measured program and
workload. "No allocations" must be measured with an allocator-aware method.
"Zero-copy" names the exact data path and copies removed, together with
lifetime, retention, pinning, fragmentation, and API costs.

Async is refined to overlap of waiting work unless evidence establishes
parallel CPU execution. Throughput gains are paired with queueing and latency.
Removing `clone` is evaluated by actual copy/allocation and ownership effects,
not syntax.

Microbenchmarks establish the cost of the isolated primitive. They support
end-to-end claims only when profiles and integrated measurements connect that
primitive to system cost.

## Rejected ideas

The doctrine rejects single stopwatch runs, debug/release mismatches, averages
without relevant tails, benchmark output optimized away, optimization before a
defined objective, noisy thresholds that train reruns, and unsafe optimization
without material benefit and doctrine 0007 proof.

## Repository additions

The repository adds a workload/objective contract, guarantee-ledger format,
boundary-cost inventory, clone/zero-copy review, correctness-independent
evidence, stable/noisy regression policy, build and binary performance, and
sixty operational measurement gates.

## Source-to-rule application

Benchmark-defense rules use Criterion and `black_box` mechanics while keeping
the benchmark author's responsibility for realistic input and result use.
Profiling rules select CPU, allocation, async, syscall, database, or network
observation according to the suspected resource. Percentile, warmup, cache, and
environment requirements make results reproducible enough for their decision.

Concurrency and boundary-cost rules connect performance to doctrines 0004–0006:
removing backpressure or failure handling is not an optimization. Unsafe speed
work inherits doctrine 0007. Correctness tests remain independent under doctrine 0008.

## Maintenance triggers

Rebaseline after toolchain, target CPU, allocator, kernel, runtime, dependency,
feature, or benchmark-host changes. Reassess workloads when production input
size, concurrency, cache state, storage, or network distribution changes.
Automated timing thresholds require current variance history; a threshold that
only succeeds after favorable reruns is not valid evidence.
