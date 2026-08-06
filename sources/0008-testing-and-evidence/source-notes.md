# RUST-DOC-0008 source notes

## Primary tool and Rust sources

[The Rust Book testing chapter](https://doc.rust-lang.org/book/ch11-00-testing.html)
and [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
document built-in test organization and execution.
[trybuild](https://docs.rs/trybuild/latest/trybuild/) documents compile-fail/UI
tests and expected diagnostics.
[proptest](https://docs.rs/proptest/latest/proptest/) documents generators,
strategies, shrinking, and failure persistence.

[Loom](https://docs.rs/loom/latest/loom/) describes schedule permutation for
modeled concurrent Rust. [Miri](https://github.com/rust-lang/miri) and Rust
[sanitizer documentation](https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html)
provide unsafe-code evidence mechanisms.
[Criterion.rs](https://bheisler.github.io/criterion.rs/book/) is cited to keep
benchmarking method separate from correctness testing.

## Accepted ideas

The doctrine accepts unit, integration, property, compile-fail, contract,
fault-injection, and model tests as different evidence types. Compile-fail
testing is particularly useful when an API claim is that safe external code
cannot construct or call something. Property testing is useful for algebraic,
round-trip, parser, and collection invariants. Controlled concurrency tools can
exercise schedules ordinary tests miss.

## Refined ideas

"Tests prove correctness" is refined to observations under defined inputs,
environments, schedules, and models. Each test class names what it supports and
what it does not establish.

Property tests do not quantify over all values unless the domain is actually
exhaustive. Generator distribution, filtering, shrinking, oracle independence,
and retained seeds determine the evidence.

Compile-fail snapshots require semantic inspection. A program can still fail
for an unrelated import or syntax reason after the intended protection is
removed. Rewriting `.stderr` is not proof maintenance by itself.

Coverage is refined to gap discovery. Executed lines do not prove meaningful
assertions, adverse inputs, or schedules. Benchmarks sample performance and
remain separate from semantic correctness.

## Rejected ideas

The doctrine rejects happy-path-only evidence, mocks that erase material failure
semantics, sleep as the primary concurrency ordering mechanism, bulk snapshot
approval, permanent retry as the resolution to flakiness, and absence of
production incidents as proof. It rejects one test-class hierarchy as a
universal total ordering.

## Repository additions

The repository adds invariant-to-evidence mapping, boundary-value partitions,
double-fidelity tables, crash-point matrices, distributed duplicate/delay/order
scenarios, evidence ledgers, semantic UI-diagnostic review, incident feedback,
test-data sensitivity, and sixty operational gates.

## Source-to-rule application

The doctrine routes claims to tools rather than mandating all tools. trybuild
supports privacy, ownership, trait, and typestate prohibitions; it cannot test
remote effects. Proptest supports generator-defined properties; it cannot
establish values outside its domain. Loom supports modeled schedules; it cannot
automatically prove production code that differs from the model. Miri and
sanitizers support unsafe review under their execution limits.

Real-boundary and fault-injection rules are repository system requirements,
added because perfect doubles omit database isolation, acknowledgement loss,
timeouts after execution, and resource limits. Production telemetry and
incidents refine the evidence plan but do not establish universal absence.

## Maintenance triggers

Inspect compile-fail diagnostics after toolchain changes and preserve intended
failure reasons. Re-evaluate test-double fidelity when protocol or database
behavior changes. Tool upgrades can alter schedule models, shrink behavior, or
supported operations; record version and rerun relevant evidence rather than
assuming equivalence.

> [!TIP]
> [attribution](attribution.md) · **source notes**
> Index: [all source packages](../README.md).
> Doctrine: [`doctrines/0008-testing-and-evidence/`](../../doctrines/0008-testing-and-evidence/README.md).
