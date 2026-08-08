# Rationale

## Failure modes

**The successor that quietly stopped leading anywhere.** A protocol declares four stages. During
a refactor the second stage's return type is changed from the third stage to a general-purpose
context value, because one caller needed to branch. Every call site still compiles; the chain
still reads like the business process; the ordering guarantee is gone. Nothing in a
documentation-only design detects this. `RUST-DOC-0010-R003` puts the successor in the contract
and `RUST-DOC-0010-R019` makes its removal a build failure.

**The bound widened to make the build pass.** An implementation cannot satisfy
`type Next: AcceptPolicy`, so the bound is relaxed to `type Next: Sized`. The edit is one line,
looks like a generics fix, and converts a compile-time protocol into a naming convention.
`RUST-DOC-0010-R004` makes the bound the fixed point and the implementation the thing that must
change.

**The conversion that skipped four stages.** A `From<Submission> for ApprovedRegistration`
implementation is added for a test fixture and later used in production code because it is
convenient. The type name still asserts approval; no approval occurred. This is the bypass that
makes an otherwise sound stage graph decorative, and it is why `RUST-DOC-0010-R010` prohibits
the conversion path outright while `RUST-DOC-0010-R011` requires the remaining trusted paths to
be restricted, owned, and inventoried.

**The branch that became optional fields.** An availability check returns one successor carrying
`Option<ExistingAccount>`. Every later stage re-inspects it, one of them forgets, and a
registration completes against a taken identity. `RUST-DOC-0010-R008` requires distinct
successor types so the conflicting path cannot be reached with the available path's evidence.
A third outcome belongs in the failure type rather than in a third variant of the branch, and
`RUST-DOC-0010-R007` keeps an undetermined check distinguishable from both branches.

**The check stage that wrote a row.** A transition named `validate` acquires an identifier and
inserts a reservation, because the identifier was needed downstream. The collapsed chain still
reads as validation. A cancelled request now leaves durable state. `RUST-DOC-0010-R013` makes
the disclosure an obligation and forbids the naming mismatch.

**The move that was read as a commit.** The strongest local guarantee available in Rust is that
a consumed value cannot be used again. A design maps stages onto persisted lifecycle states and
concludes that because the Rust value was consumed, the durable advance happened once. Stored
facts do not work that way: they are read, copied into a value, and can be read again by another
worker, so no local move consumes them. Two workers can each hold a consumed local handle for
the same row. `RUST-DOC-0010-R014` separates the two claims and requires identity, stored state,
and a concurrency token to be re-checked where durable state advances.
`RUST-DOC-0010-R015` keeps the durable model at runtime.

**The consuming transition defeated by a derive.** Every transition takes `self`, the
compile-fail case for reuse-after-move passes, and the design is described as permitting single
progression. Then a stage derives `Clone`, because some test wanted a copy. A caller now clones
the stage and advances both copies; the consuming signatures are all still there and all still
satisfied. This defect was live in this package's own example until review caught it, and the
committed compiler diagnostic for the reuse case even suggested `.clone()` as the workaround.
`RUST-DOC-0010-R005` therefore makes non-duplicability part of the obligation rather than a
consequence of it, and `RUST-DOC-0010-R018` requires a compile-fail case for duplication
separately from one for reuse.

**The topology assertion that asserted itself.** A helper is written to pin the stage graph:

```rust
fn assert_edge<S, N>() where S: Canonicalize<Next = N>, N: CheckIdentity {}
```

It compiles, it names every edge, and it detects nothing. The `N: CheckIdentity` bound in the
helper supplies exactly the constraint the trait is supposed to declare, so deleting
`type Next: CheckIdentity` from `Canonicalize` leaves the assertion green. This too was live in
this package until review compiled the library with the bound removed and observed a passing
suite. A contract assertion has to derive the successor capability from the trait alone:

```rust
fn assert_contract<S: Canonicalize>() {
    fn requires<T: CheckIdentity>() {}
    requires::<S::Next>();
}
```

Nothing here supplies the bound, so it fails the moment the trait stops declaring it. Edge
assertions remain useful for pinning the concrete successor; they are not a substitute.
`RUST-DOC-0010-R019` requires the contract form.

**The protocol erased in the middle.** An orchestration layer converts stage three into a
dynamic map so a plugin can inspect it, then converts back. Static enforcement ends at that
point for every later stage. `RUST-DOC-0010-R017` confines erasure to a named boundary while
still permitting dynamic strategy selection, so long as each selected branch continues through
typed stages.

**The vocabulary that borrowed authority.** A design note names a local mechanism, and a later
reader cites the name as established practice, treating a project convention as external
consensus. `RUST-DOC-0010-R021` requires the family attribution to travel with the local name.

**The authority nobody partitioned.** A protocol enforces ordering, and the argument then runs in
one of two directions, both wrong. In one, the accurate observation that code enforces ordering
grows into the claim that code is the whole contract, so review evidence, guarantee ledgers, and
the decision process become optional; code enforces what it enforces, and records neither why the
ordering was chosen, nor what the stages deliberately do not prove, nor who accepted the residual
risk. In the other, the accurate observation that doctrine governs change grows into a precedence
of prose over the compiled graph, which would contradict `RUST-DOC-0010-R018` and
`RUST-DOC-0010-R019`, both of which exist because prose cannot detect a widened bound.
`RUST-DOC-0010-R022` partitions the claims instead of ranking the artifacts, and RUST-DOC-0011
governs that partition generally. The earlier text of this rule asserted the second direction; the
restatement is recorded in RFC-0003.

## Why weaker alternatives fail

**Prose ordering.** A design document stating "authorize before capture" is readable and cheap.
It is also unenforced, and it goes stale silently: the document and the code diverge without any
signal. It remains the right choice when the sequence is advisory or when the states are
externally determined.

**A concrete successor return type.** Returning `Authorized` directly from `authenticate` is
genuine typestate and satisfies most of this doctrine. What it cannot express is one capability
with several implementations producing different successor evidence. A password login and an
invitation-based signup both need to reach the authorization stage while carrying different
proofs. Without an associated successor type, that requires either one widened successor
carrying both proofs as options, which reintroduces the optional-field failure, or a duplicated
protocol. This is the specific gap `RUST-DOC-0010-R003` fills, and it is why the doctrine exists
separately from [`patterns/typestate.md`](../../patterns/typestate.md).

**A runtime state machine.** An enum with a `state` field and a `transition` method handles
dynamic, persisted, heterogeneous, and externally-determined state well, and it is the correct
choice for durable lifecycle. What it does not do is remove illegal calls from the API surface;
each method re-checks and each caller must handle a rejection that a typed protocol would have
made unrepresentable. `RUST-DOC-0010-R015` is the explicit instruction to use both: runtime for
the durable half, typed stages for the in-process pass.

**A middleware chain.** Ordering by registration position is flexible and composes well. It
proves nothing about what a downstream handler receives, and reordering two entries is a silent
behavioral change. It remains appropriate when the stages are genuinely independent and share no
evidence.

**Compile-fail tests alone.** Negative tests prove the specific programs written remain
rejected. They do not prove the graph is intact, because a redirected associated type can leave
every existing negative test passing while the edge it protected no longer exists. That
asymmetry is why `RUST-DOC-0010-R018` and `RUST-DOC-0010-R019` are separate obligations.

## Interaction with external reality

A stage type is local evidence with a timestamp. An availability observation records that no
conflicting account was visible to one reader at one moment; another writer can take the
identity immediately afterward. A consent proof records that an offered version matched the
version in force when the check ran; the policy can change before the record is written. A
prepared value records that the in-process protocol ran in order; it records nothing about
whether a durable write followed.

Asynchronous transitions add interruption. A transition cancelled after a remote effect was
accepted but before the successor was constructed leaves the external world advanced and the
local protocol not advanced. `RUST-DOC-0010-R016` requires that possibility to be stated per
stage rather than discovered, and where it matters the honest representation is an additional
stage for the interval whose outcome is unknown, which is the territory `RUST-DOC-0006` governs.

## Costs and overapplication

Capability traits with associated successor types make signatures longer and diagnostics
harder than a concrete return type; a mismatch is reported as an unsatisfied bound rather than a
plain type error. Generic stage types spread through helper functions, test harnesses, and mock
implementations. Each additional stage adds a type, a failure type, a ledger row, and a
topology assertion. Monomorphization grows with the product of stages and implementations.

The mechanism earns none of that when a protocol has two stages, when the sequence is advisory,
when states are chosen at runtime by external systems, when callers must hold heterogeneous
stages in one collection, or when the whole graph already fits in one function signature. A
three-line pipeline of ordinary functions is a better answer than seven traits, and
`RUST-DOC-0010-R012` exists to make that comparison mandatory rather than optional.

## Evidence limits

Compiler rejection proves that the specific programs written are rejected at the pinned
diagnostic boundary, and nothing about programs nobody wrote. A topology assertion proves the
edges it names still typecheck; it does not prove the graph matches the business process, which
remains a review judgment under `RUST-DOC-0010-R001`. Unit tests over an in-memory collaborator
prove the transitions behave as written on the inputs supplied, and prove nothing about a real
directory, database, or broker.

No evidence in this repository establishes the durable half. The example crate deliberately
stops at a persistable value, so `RUST-DOC-0010-R014` and `RUST-DOC-0010-R015` are supported by
argument and review gates rather than by an executed database test. A consuming system supplies
its own competing-writer and fault evidence; this package does not claim it.
