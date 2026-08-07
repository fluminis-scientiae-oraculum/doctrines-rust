# RUST-DOC-0002 source notes

## Primary Rust mechanics

[The Rust Book: recoverable errors with `Result`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
and the standard-library
[`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) documentation
establish Rust's ordinary success/error representation and propagation
mechanics. [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)
defines source chaining. The
[Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) inform public
API naming and interoperability.

The [`thiserror`](https://docs.rs/thiserror/latest/thiserror/) documentation
shows a derive-based way to implement structured library errors without making
that crate part of a public semantic contract. The
[`anyhow`](https://docs.rs/anyhow/latest/anyhow/) documentation shows contextual
application reports. These crate documents establish capabilities; they do not
make one crate mandatory.

## Accepted ideas

The doctrine accepts `Result` for expected fallibility, error source
preservation, and structured enums when callers choose different behavior.
Validation, conflict, rejection, timeout, cancellation, and unknown execution
are different when they produce different recovery. Panics are reserved for
violated internal assumptions or programmer errors, not malformed external
input.

Application boundaries often benefit from opaque reports and context, while
library/domain boundaries benefit from stable categories. These approaches can
compose: a structured error remains in the source chain of an application
report.

## Refined ideas

"Use custom error types" is refined into an operational test: keep distinctions
only when callers, security, compatibility, retry, reconciliation, or audit act
differently. One enum per function is unnecessary; one giant `AppError` often
couples unrelated layers.

Retryability is not a property inferred solely from an I/O variant. It depends
on operation semantics and failure point. A timeout after possible dispatch is
not automatically a retryable failure. RUST-DOC-0006 adds explicit unknown
outcomes and reconciliation.

`expect` can be useful when an internal invariant has already been established
and the message states why impossibility follows. Its presence is not the only
quality signal; a hand-written panic on user input is equally incorrect.

## Rejected ideas

The doctrine rejects `Result<T, String>` as a primary library/domain contract,
logging and discarding an error, retrying every error, converting timeout to
rejection, and exposing sensitive internal details to clients. It also rejects
the absolute claim that every public error enum must expose every internal
variant forever. Public compatibility and non-exhaustive evolution require
deliberate design.

## Repository additions

The doctrine adds review gates for error stability, sensitive-data redaction,
double logging, cancellation, security and reconciliation information,
machine-actionable codes, and fatal process policy. It requires an error
decision table connected to domain actions and a guarantee ledger that keeps
failure evidence separate from external certainty.

## Source-to-rule application

RUST-DOC-0002-R001 through R004 apply the `Result` and source-chain mechanics to
stable domain contracts. The retry, timeout, and indeterminate-outcome rules are
repository refinements informed by boundary semantics rather than features of
an error derive. Panic and `unwrap` rules follow the Book's recoverable versus
unrecoverable distinction but add a production-path justification and review
record.

Public error compatibility is assessed alongside enum evolution and
non-exhaustive patterns; changing display wording is not automatically a
breaking semantic change, while collapsing machine categories can be.

## Maintenance triggers

Recheck this provenance when Rust changes `Error` capabilities, selected crates
change MSRV or public behavior, or a protocol supplies new retry/finality
evidence. A new convenience error crate does not change doctrine unless it
changes the semantics available to callers. Security response mapping should be
reviewed against the actual application threat model rather than copied from a
generic example.

> [!TIP]
> [attribution](attribution.md) · **source notes**
> Index: [all source packages](../README.md).
> Doctrine: [`doctrines/0002-error-modeling/`](../../doctrines/0002-error-modeling/README.md).
