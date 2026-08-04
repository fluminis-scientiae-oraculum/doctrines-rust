# Anti-pattern catalogue

## Chain without state change

**Weak example.**

```rust
impl Registration {
    fn canonicalize(self) -> Result<Self, Error> { /* ... */ }
    fn check_identity(self) -> Result<Self, Error> { /* ... */ }
    fn accept_policy(self) -> Result<Self, Error> { /* ... */ }
}
```

**Why it fails.** The chain reads like a protocol and enforces nothing. Every method is available
at every point, so `accept_policy` can run first and the compiler is content. The fluency is
real; the ordering guarantee is imaginary.

**Risk.** A reviewer reads the call site, sees the business sequence, and approves an API that
permits every other sequence.

**Improved direction.** Give each stage a distinct type whose construction proves the preceding
transition ran, and expose the successor through the stage contract.

**When justified.** When the operations genuinely are reorderable, in which case the sequence
should not be presented as a protocol at all.

## Successor named only in prose

**Weak example.**

```rust
pub trait CheckIdentity {
    /// Returns a value that can then accept policy.
    fn check_identity(self) -> Result<Box<dyn Any>, Error>;
}
```

**Why it fails.** The successor relationship exists in a doc comment. Nothing checks that the
returned value can accept policy, and the first refactor that returns something else compiles
cleanly.

**Risk.** The protocol graph degrades silently, and the failure surfaces at a call site far from
the change.

**Improved direction.** Name the successor as an associated type bounded by the next capability,
and assert the resulting graph executably.

**When justified.** Never for a stage with a legal successor. A terminal stage names no successor
because it has none.

## Bound widened to satisfy an implementation

**Weak example.**

```rust
pub trait Canonicalize {
    type Next; // was: type Next: CheckIdentity
}
```

**Why it fails.** The bound was the entire protocol guarantee. Removing it is a one-line edit
that looks like a generics simplification and reads as noise in a diff.

**Risk.** A stage graph that still has all its types, all its names, and none of its edges.

**Improved direction.** Treat the bound as fixed and change the implementation or the stage
design. Where a bound genuinely must move, record the reason and re-check the topology
assertion.

**When justified.** Only when the protocol itself was wrong, and then the change is a doctrine
decision rather than a refactor.

## Conversion that manufactures a later stage

**Weak example.**

```rust
impl From<RawSubmission> for AcceptedRegistration {
    fn from(raw: RawSubmission) -> Self { /* ... */ }
}
```

**Why it fails.** `AcceptedRegistration` asserts that canonicalization, an availability check,
and a policy check all succeeded. The conversion asserts all three without performing any. The
same failure arrives through a public constructor, a public field, or a derived decoder.

**Risk.** Every guarantee downstream of the stage becomes false while the type names continue to
claim it.

**Improved direction.** Remove the conversion. Where a trusted path is genuinely required, make
it visibility-restricted, give it an owner, state the obligation its caller assumes, and list it
in the guarantee ledger.

**When justified.** Never as an ambient conversion. A restricted, inventoried, owned
construction path for checked restoration or migration is a different mechanism with different
obligations.

## Branch flattened into optional fields

**Weak example.**

```rust
pub struct CheckedRegistration {
    conflict: Option<AccountId>,
    uniqueness: Option<UniquenessObservation>,
}
```

**Why it fails.** The type admits all four combinations, two of which are meaningless. Every
later stage must re-inspect and re-decide, and the first stage that forgets proceeds on a
conflicting identity.

**Risk.** A registration completes against an identity another account already holds.

**Improved direction.** Return a named sum type over distinct successor stages, each bounded by
the capability that outcome legitimately leads to.

**When justified.** When the outcome changes no successor capability and no later obligation, in
which case it is ordinary data rather than a branch.

## Undetermined outcome folded into a branch

**Weak example.**

```rust
match directory.lookup(&address) {
    Ok(Some(holder)) => conflicting(holder),
    _ => available(),
}
```

**Why it fails.** A directory that could not be reached is treated as proof that the identity is
free. The most dangerous outcome is silently mapped onto the most permissive one.

**Risk.** A protocol advances on evidence that was never obtained.

**Improved direction.** Keep the undetermined case in the stage-identifying failure type, and
carry enough identity for an operator to look the attempt up.

**When justified.** Never. If the undetermined case is common, it deserves its own stage rather
than a quieter default.

## Check stage with a durable effect

**Weak example.**

```rust
fn validate(self) -> Result<Validated, Error> {
    self.repository.insert_reservation(&self.id)?;
    // ...
}
```

**Why it fails.** The name says the stage establishes a fact about the input; the body changes
the world. A cancelled or failed request now leaves durable state that nothing in the collapsed
chain suggests exists.

**Risk.** Orphaned reservations, duplicate side effects on retry, and a cleanup path nobody
wrote because nobody knew it was needed.

**Improved direction.** Split into a stage that checks and a stage that writes, and disclose the
effect on the stage that performs it.

**When justified.** When the domain genuinely defines one atomic operation, in which case the
stage name says so.

## Local move presented as durable proof

**Weak example.**

```rust
// The handle was consumed, so the row advanced exactly once.
let receipt = pending.mark_paid()?;
```

**Why it fails.** Consuming a Rust value proves the caller cannot use that value again. It
proves nothing about a stored row, because a stored fact can be read again into a second value
by a second worker. Two workers can each hold a consumed handle for the same row.

**Risk.** Lost updates, duplicate durable transitions, and a concurrency bug whose type
signatures look impeccable.

**Improved direction.** Re-check identity, stored state, and a version or fencing token in the
authoritative statement, and keep the durable model at runtime. State the local and durable
claims as separate ledger rows.

**When justified.** Never for the claim. The consuming transition remains correct and useful as
a statement about local handle lifecycle.

## Protocol erased between stages

**Weak example.**

```rust
let context: HashMap<String, Value> = stage_three.into_context();
let stage_four = StageFour::from_context(&context)?;
```

**Why it fails.** Static enforcement ends at the map. Every stage after it is checked by
convention, and the round trip re-admits exactly the states the protocol was built to exclude.

**Risk.** The remaining stages carry the appearance of enforcement without the substance.

**Improved direction.** Keep the types through the protocol and erase once, at a named
orchestration or persistence boundary, after the stages that matter have run.

**When justified.** At the named boundary itself, and for runtime selection among
implementations where each selected branch continues through typed stages.

## Stage per helper function

**Weak example.**

```rust
raw.trim()?.lowercase()?.split()?.reassemble()?.validate()?.normalize()?
```

**Why it fails.** None of these establish a fact a later stage depends on; they are steps in one
transformation. The reader now navigates six types to follow one canonicalization.

**Risk.** The protocol becomes unreadable, and the genuine proof boundaries are lost among the
mechanical ones.

**Improved direction.** Merge until each stage corresponds to a proof, and assess the resulting
count against the complexity budget.

**When justified.** When an external mandate requires a checkpoint per step even though the
engineering boundary is weaker.

## Local vocabulary cited as external authority

**Weak example.** A design note introduces a coined name for a mechanism, and a later document
cites the name as established practice without attribution.

**Why it fails.** The name is a useful local shorthand. Treating it as external consensus
borrows authority the mechanism has not earned and makes the claim hard for a reader to check.

**Risk.** Review deference to a term rather than to the argument behind it.

**Improved direction.** Keep the local name, and state the established family it refines and the
citation for that family.

**When justified.** When the term is genuinely defined by a cited specification or published
literature, in which case the citation travels with it.

## Code offered as its own governance

**Weak example.** A protocol enforces ordering, and the design note concludes that documentation
of the ordering is therefore unnecessary and no decision record is required.

**Why it fails.** The code records what is enforced. It does not record why this ordering was
chosen over alternatives, what the stages deliberately do not prove, which residual risks were
accepted, or who accepted them. A future maintainer reading only the code can reconstruct the
mechanism and none of the reasoning.

**Risk.** A guarantee is weakened in a refactor because the reason it existed was never written
down.

**Improved direction.** Let the code be authoritative for in-process ordering and keep the
decision record, the guarantee ledger, and the review evidence alongside it.

**When justified.** Never as stated. The underlying observation, that prose alone should not be
the only enforcement, is correct and is what the doctrine already requires.
