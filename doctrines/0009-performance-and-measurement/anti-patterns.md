# Anti-pattern catalogue

## Rust is fast

**Weak example.** A design claims acceptable performance because it is written
in Rust.

**Why it fails.** language capability does not define algorithm, workload,
allocation, I/O, or contention.

**Risk.** untested capacity and latency.

**Improved direction.** define objective, profile, and measure representative
work.

**When justified.** Rust features can explain mechanisms after measurement, not
replace it.

## Optimize by inspection

**Weak example.** Code with clones or iterator adapters is rewritten without a
profile.

**Why it fails.** compiler optimization or dominant I/O can make the change
irrelevant.

**Risk.** complexity with no material benefit.

**Improved direction.** profile cost and retain a clear baseline.

**When justified.** A clear algorithmic complexity defect may be corrected and
then measured.

## One stopwatch run

**Weak example.** A command is timed once before and after on a busy laptop.

**Why it fails.** noise, cache, frequency, and background work dominate.

**Risk.** random variation becomes architecture.

**Improved direction.** use repeated controlled measurements and report
variability.

**When justified.** A single run can orient exploration but cannot support a
claim.

## Debug-versus-release comparison

**Weak example.** old code runs in debug and new code in release.

**Why it fails.** configuration, not implementation, explains the result.

**Risk.** invalid decision.

**Improved direction.** compare identical profiles, features, toolchains, and
targets.

**When justified.** Comparing build profiles is valid when profile choice is the
actual subject.

## Average latency only

**Weak example.** Mean request time improves while tail samples are omitted.

**Why it fails.** queueing or rare inputs can harm users severely.

**Risk.** hidden p99 regression.

**Improved direction.** report distributions correlated with workload.

**When justified.** Near-deterministic operations may summarize narrowly after
variance evidence.

## Throughput at any cost

**Weak example.** Batch size increases until maximum throughput, ignoring item
wait and memory.

**Why it fails.** queueing shifts cost into latency and resource retention.

**Risk.** deadline misses and overload collapse.

**Improved direction.** sweep batch/concurrency and record latency, queue,
memory, and rejection.

**When justified.** Offline throughput-only jobs can prioritize aggregate
completion while bounding resources.

## Async means parallel

**Weak example.** An async rewrite is expected to speed CPU-heavy work.

**Why it fails.** cooperative concurrency may remain on one worker and adds
scheduling overhead.

**Risk.** worse latency and executor starvation.

**Improved direction.** isolate CPU work, measure parallel execution, and bound
concurrency.

**When justified.** Async can improve overlap of waiting operations.

## Clone-count optimization

**Weak example.** Removing `.clone()` introduces shared ownership and locking
without measuring the clone.

**Why it fails.** clone cost varies, and longer sharing can be worse.

**Risk.** contention, retention, and complex lifetimes.

**Improved direction.** measure allocations/bytes and compare ownership
architectures.

**When justified.** A proven large copy on the hot path may deserve redesign.

## Zero-copy slogan

**Weak example.** A borrowed parser is marketed as zero-copy though
normalization and output serialization still copy.

**Why it fails.** scope and retained buffers are hidden.

**Risk.** misleading API and higher peak memory.

**Improved direction.** enumerate avoided copies and lifetime costs.

**When justified.** Use the term only for a precisely defined path with
evidence.

## Microbenchmark victory

**Weak example.** A primitive becomes twice as fast but occupied 0.2 percent of
request time.

**Why it fails.** end-to-end impact is below relevance.

**Risk.** maintenance cost without user benefit.

**Improved direction.** connect profile share to integrated measurement.

**When justified.** Library users may value the exact primitive claim; keep it
narrow.

## Unsafe for theoretical speed

**Weak example.** Checked indexing is replaced by raw pointers without showing
a measurable bottleneck.

**Why it fails.** compiler may remove checks; proof risk remains.

**Risk.** undefined behavior for no gain.

**Improved direction.** measure safe baseline, try safe structure changes, then
apply RUST-DOC-0007 if material.

**When justified.** Material target-workload gain plus complete safety evidence.

## Noisy hard gate

**Weak example.** Shared CI fails on a tiny timing regression, so maintainers
rerun until green.

**Why it fails.** gate measures host noise and selects favorable samples.

**Risk.** distrust and missed real regressions.

**Improved direction.** calibrate variance, use a dedicated host, raise
threshold, or report trends.

**When justified.** Stable metrics with justified thresholds make good gates.
