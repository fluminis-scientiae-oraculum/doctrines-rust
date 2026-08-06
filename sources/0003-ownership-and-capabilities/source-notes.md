# RUST-DOC-0003 source notes

## Primary Rust mechanics

[The Rust Book: ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html),
[references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html),
and [lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
describe moves, borrowing, and lifetime relationships. Standard-library
documentation for [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html),
[`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html), and
[`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) establishes
mechanics relevant to custody and cleanup.

The [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) informs
trait and wrapper design. Capability-security terminology is used as a design
model; repository rules remain scoped to Rust API authority, not a claim that
the whole process is a capability-secure operating system.

## Accepted ideas

Ownership can express more than allocation safety. A moved transaction or
single-use token can prevent local reuse. A non-`Clone` capability can represent
single custody. Borrowing can grant temporary observation without ownership.
RAII aligns local resource lifetime with scope and is appropriate for memory,
locks, file descriptors, and other locally controlled handles.

Private capability construction and narrow methods reduce ambient authority.
Secrets benefit from deliberate formatting, cloning, and serialization rather
than broad derives.

## Refined ideas

Ownership evidence is local. Moving a transaction handle prevents the caller
from using that value; it does not prove a remote commit or rollback.
Dropping a guard can initiate cleanup but cannot report an externally fallible
compensation through `Drop`. A file-lock guard proves local handle custody, not
that every process cooperates.

Non-`Clone` improves local exclusivity but does not prevent another handle from
being issued for the same external resource. Revocation and expiry require
runtime enforcement. Lifetime parameters should encode a real relationship,
not be introduced as decoration.

`Arc<Mutex<T>>` is a valid primitive, not an ownership architecture by itself.
The protected invariant, mutation authority, lock scope, poisoning, and task
shutdown remain design obligations.

## Rejected ideas

The doctrine rejects globally shared clients as automatic authority,
clone-derived tokens without semantic review, hidden interior mutability, and
claims that zeroization removes all copies. It rejects destructors that are
described as guaranteed external rollback and lifetime extension used only to
silence design errors.

## Repository additions

The repository adds authority maps, capability constructor audits, clone and
transfer reviews, session/token revocation semantics, secret leakage checks,
lease and fencing qualifications, shutdown permits, transaction-guard failure
models, and task custody. It connects ownership with RUST-DOC-0004 supervision,
RUST-DOC-0005 transaction persistence, and RUST-DOC-0006 distributed authority.

## Source-to-rule application

The doctrine's ownership rules apply move and borrow mechanics to domain
custody. Capability rules go beyond the language source: Rust privacy and
non-`Clone` construction prevent ordinary local forgery and duplication, while
runtime policy handles revocation and external copies. RAII rules separate
infallible local resource release from effects whose failure must remain
observable.

Secret-wrapper guidance derives from trait behavior — `Debug`, `Display`,
`Serialize`, and `Clone` are capabilities — but its privacy and retention policy
is repository governance. No type-level decision can guarantee removal from
allocator history, crash dumps, or foreign code.

## Maintenance triggers

Revisit the proof when ownership crosses new tasks, processes, FFI, or
persistence; when a capability becomes serializable or cloneable; or when
resource cleanup semantics change. A new session store, lease service, or
downstream enforcement point can invalidate prior revocation assumptions even
if Rust signatures remain unchanged.

> [!TIP]
> [attribution](attribution.md) · **source notes**
> Index: [all source packages](../README.md).
> Doctrine: [`doctrines/0003-ownership-and-capabilities/`](../../doctrines/0003-ownership-and-capabilities/README.md).
