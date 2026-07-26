# References

- [Criterion.rs book](https://bheisler.github.io/criterion.rs/book/) documents
  statistical benchmarking, measurement, comparison, and common methodology.
- [Rust `std::hint::black_box`](https://doc.rust-lang.org/std/hint/fn.black_box.html)
  documents an optimization barrier useful in benchmarks and its limitations.
- [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
  define optimization, debug information, code generation, and related build
  settings that must be recorded.
- [rustc performance data](https://perf.rust-lang.org/) and the
  [rustc-perf repository](https://github.com/rust-lang/rustc-perf) demonstrate
  controlled compiler performance tracking.
- [Linux `perf` documentation](https://perf.wiki.kernel.org/index.php/Main_Page)
  describes system profiling facilities commonly used for CPU and hardware
  events.
- [Brendan Gregg's FlameGraph repository](https://github.com/brendangregg/FlameGraph)
  documents the original stack-collapse and flamegraph tooling.
- [Tokio runtime metrics documentation](https://docs.rs/tokio/latest/tokio/runtime/struct.RuntimeMetrics.html)
  provides runtime-specific observations for async scheduling and queues.

Third-party tools establish their own measurement mechanics. This doctrine adds
workload, provenance, correctness, scope, regression, and guarantee-honesty
requirements.
