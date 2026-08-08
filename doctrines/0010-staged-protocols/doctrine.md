# Normative doctrine

## RUST-DOC-0010-R001 — Inventory the protocol before typing it

**Statement.** A staged protocol MUST have a written inventory of stages, edges, the evidence
each transition establishes, its failure classes, and its external effects before stage types
or capability traits are introduced.

**Intent.** Prevent a type graph from being derived mechanically from existing functions rather
than from the proof boundaries the domain actually has.

**Applicability.** Multi-stage command, request, submission, handshake, and workflow protocols
whose stage order carries consequence.

**Allowed exceptions.** A single-transition operation MAY record the inventory inline with its
design note.

**Review evidence.** Stage and edge inventory, evidence-per-transition table, and the design
note that preceded the types.

**Enforcement.** [`improved.md`](../../case-studies/registration-onboarding/improved.md) — stage
graph plus per-stage evidence rows

## RUST-DOC-0010-R002 — Name each stage by the fact it proves

**Statement.** A stage type MUST be named for the fact its construction establishes, and MUST
NOT be named for its position, its processing step, or a version counter.

**Intent.** Keep the stage graph readable as a sequence of proofs rather than an ordering of
implementation steps.

**Applicability.** Every named stage type and type-level state marker in a staged protocol.

**Allowed exceptions.** None. A stage whose proven fact cannot be named is evidence that the
boundary is not a real one.

**Review evidence.** Stage names, their documented guarantees, and the guarantee ledger.

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— stages named for proven facts, not positions

## RUST-DOC-0010-R003 — Expose the successor capability in the stage contract

**Statement.** A stage capability whose protocol has a legal successor MUST expose that
successor as an associated type bounded by the capability the successor is required to satisfy,
rather than returning an unconstrained generic, an erased type, or a value whose successor
relationship exists only in prose.

**Intent.** Make the protocol edge a checked part of the contract, so a stage that stops leading
anywhere legal fails to compile instead of failing in review.

**Applicability.** Capability traits for staged protocols with more than one transition.

**Allowed exceptions.** A terminal stage MUST NOT name a successor. A protocol with exactly one
transition MAY return a concrete successor type directly when no second implementation is
anticipated.

**Review evidence.** Trait definitions, associated-type bounds, and the topology assertion
required by RUST-DOC-0010-R019.

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— each capability declares its successor as an associated type

## RUST-DOC-0010-R004 — Bound the successor by capability actually established

**Statement.** A successor bound MUST name only capabilities the successor value genuinely
establishes, and MUST NOT be widened, relaxed, or removed in order to make an implementation
compile.

**Intent.** Prevent the protocol contract from being edited to match a convenient
implementation, which converts a compile-time guarantee into decoration.

**Applicability.** Every associated successor type and its bounds.

**Allowed exceptions.** None. A bound that cannot be satisfied indicates the stage graph or the
implementation is wrong, not the bound.

**Review evidence.** Bound change history, the reason each bound exists, and the review record
for any relaxation.

**Enforcement.** Unenforceable: No file records bound-change history; a relaxation is visible only
in review records

## RUST-DOC-0010-R005 — Consume the stage on transition

**Statement.** A stage transition MUST consume the stage value when reuse of the prior stage
would be invalid, and MUST NOT rely on an internal flag to mark the stage as advanced. A stage
whose protocol claims single progression MUST NOT be duplicable, so it MUST NOT implement or
derive `Clone` or `Copy` unless duplicate progression is deliberately permitted and documented.

**Intent.** Make the successor value the evidence that the transition ran, and make reuse of the
superseded stage a compiler error. Consumption alone is insufficient: a caller holding a
duplicable stage can copy it first and advance every copy, which satisfies the letter of a
consuming signature while defeating its purpose.

**Applicability.** Transitions between stages of a locally owned protocol, and the trait
implementations of every stage type and branch wrapper. RUST-DOC-0003 governs custody and
RUST-DOC-0001 governs legal transitions and the clone audit generally; this rule adds the
stage-to-stage obligation.

**Allowed exceptions.** A read-only inspection that establishes no new fact MAY borrow. A
failure proven to occur before any part of the transition MAY return the prior stage with its
error. A terminal stage with no successor MAY be duplicable, since duplicating it advances no
protocol.

**Review evidence.** Method receivers, recovery shapes, the derive and trait-implementation
audit for every stage type, and compile-fail cases for both consumed-stage reuse and stage
duplication.

**Enforcement.** [`reuse_consumed_stage.rs`](../../examples/compile-fail/ui/reuse_consumed_stage.rs)
— a consumed stage cannot be reused or cloned

## RUST-DOC-0010-R006 — Carry forward exactly the evidence successors need

**Statement.** A stage MUST carry the evidence its successors require, and MUST NOT retain a
superseded untrusted representation unless a named audit, diagnostic, or reconciliation
obligation requires it and the retained value is distinguishable from the canonical one.

**Intent.** Keep a later stage from re-deriving a fact, and keep a raw value from being mistaken
for a checked one after the stage that checked it.

**Applicability.** Stage payloads and the values transitions move between them.

**Allowed exceptions.** Audit, reconciliation, and error-reporting obligations MAY retain the
original input when it is separately named.

**Review evidence.** Stage fields, the field-provenance mapping, and tests that canonical values
survive every transition.

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— canonical values survive every transition

## RUST-DOC-0010-R007 — Keep stage failure distinguishable

**Statement.** Each **fallible** transition MUST expose a failure type that identifies the stage
that failed, and a protocol MUST NOT erase its stage failures into one opaque type before the
protocol completes. A transition that cannot fail MUST NOT declare a failure type it never
constructs.

**Intent.** Preserve which proof was not established, which is the information a caller needs to
choose between retry, revision, and abandonment, without spending that machinery on a state the
transition cannot reach.

**Applicability.** Failure types of stage transitions. RUST-DOC-0002 governs error taxonomy
design; this rule adds the stage-identity obligation inside a protocol. The second sentence
applies to any transition whose body has no failure path.

**Allowed exceptions.** A boundary adapter MAY map stage failures into one transport or
presentation error after the protocol completes. A transition that only rearranges evidence
already established, performs no I/O, and enforces no further condition MAY be infallible, as
RUST-DOC-0001-R013 permits for pure in-process operations; its signature then returns the
successor directly rather than a `Result`.

**Review evidence.** Per-stage failure types, the boundary mapping, tests asserting stage
identity is preserved, and, for each transition declared fallible, a test or code path that
constructs its failure.

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— the infallible signature cites this rule; four per-stage error types

## RUST-DOC-0010-R008 — Model material branches as named successor alternatives

**Statement.** A transition with materially different outcomes MUST return a named sum type over
distinct successor stages, and MUST NOT return one successor carrying optional fields that stand
in for a state that was never established.

**Intent.** Prevent a branch from degrading into a partially populated value that every later
stage must re-inspect.

**Applicability.** Approval, availability, eligibility, verification, and routing transitions.

**Allowed exceptions.** An outcome that changes no successor capability and no later obligation
MAY be represented as data on one successor.

**Review evidence.** Branch enum definitions, successor bounds per variant, and a test per
branch.

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— a branch outcome over distinct successor types

## RUST-DOC-0010-R009 — Name retry, revision, and recovery edges

**Statement.** A protocol that permits retry, revision, correction, or resumption MUST represent
each such path as a named stage and a named edge, and MUST NOT leave it implicit in caller
control flow.

**Intent.** Keep the recovery half of a protocol as visible and as reviewable as its success
path.

**Applicability.** Protocols with revisable input, contended identity, recoverable rejection, or
resumable interruption.

**Allowed exceptions.** A protocol whose only recovery is to restart from the initial stage MAY
state that explicitly instead of adding a stage.

**Review evidence.** Recovery stage types, the edges that reach them, and tests exercising each
recovery path.

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— revision re-enters at the first stage; abandonment is terminal

## RUST-DOC-0010-R010 — Prohibit conversion paths that skip stages

**Statement.** A protocol MUST NOT expose a `From`, `Into`, `Default`, public constructor,
public field, or derived decoding path that constructs a later stage without performing the
intervening transitions.

**Intent.** Close the bypass that makes an otherwise sound stage graph decorative, since a
conversion that produces a later stage asserts every proof that stage represents.

**Applicability.** Trait implementations, constructors, field visibility, and derived
deserialization on stage types and stage evidence.

**Allowed exceptions.** A restricted trusted-construction path MAY exist under
RUST-DOC-0010-R011.

**Review evidence.** Trait implementation inventory, field visibility audit, derive audit, and
the evidence-forgery compile-fail case.

**Enforcement.** [`forge_stage_evidence.rs`](../../examples/compile-fail/ui/forge_stage_evidence.rs)
— a private field blocks forged stage evidence

## RUST-DOC-0010-R011 — Restrict and inventory trusted stage construction

**Statement.** Any path that constructs a stage or its evidence without running the
corresponding transition MUST be visibility-restricted to a named owner, MUST be listed in the
guarantee ledger, and MUST state the obligation its caller assumes.

**Intent.** Keep necessary construction paths for testing, migration, and checked restoration
from becoming ambient protocol bypasses.

**Applicability.** Test builders, migration adapters, restoration services, and privileged
factories.

**Allowed exceptions.** None to omit the inventory. The path itself is permitted only with a
recorded owner and obligation.

**Review evidence.** Visibility, the escape-hatch inventory, and the caller obligation recorded
beside each path.

**Enforcement.** Unenforceable: Example crate has no trusted construction path; inventory
completeness has no checkable source

## RUST-DOC-0010-R012 — Keep stage granularity proportionate

**Statement.** A stage MUST correspond to a proof boundary rather than an implementation helper,
and the stage count SHOULD be justified against the complexity budget when the protocol exceeds
the size a reader can hold in one signature chain.

**Intent.** Prevent both directions of failure: one stage hiding several unrelated
responsibilities, and a stage per helper function.

**Applicability.** Protocol design and any change that adds or merges a stage.

**Allowed exceptions.** A regulated process MAY require a stage per externally mandated
checkpoint even when the engineering boundary is weaker.

**Review evidence.** Stage count, the proof each stage adds, the complexity-budget assessment,
and the rejected alternative granularity.

**Enforcement.** Unenforceable: Stage-count proportionality and budget justification appear in no
file for this protocol

## RUST-DOC-0010-R013 — Disclose durable and external effects per stage

**Statement.** A transition MUST disclose the durable writes, external calls, and messages it
performs, and a transition named for a check, validation, or preparation MUST NOT perform a
durable write or publish a message.

**Intent.** Keep the collapsed call chain an accurate summary of what the protocol does, not
only of what it proves.

**Applicability.** Every transition in a protocol that touches storage, a network, a broker, or
a filesystem.

**Allowed exceptions.** A domain that genuinely defines one atomic operation MAY combine effects
under a name that says so.

**Review evidence.** Per-stage effect inventory, the transition names, and tests asserting that
effect-free stages perform no effect.

**Enforcement.** [`improved.md`](../../case-studies/registration-onboarding/improved.md) — all four
transitions disclosed write-free

## RUST-DOC-0010-R014 — Do not present a local transition as a durable one

**Statement.** A consuming in-process transition MUST NOT be presented as evidence that a
durable or remote state change occurred, and a transition that advances authoritative state MUST
re-check the entity identity together with its stored state and a version, fence, or equivalent
concurrency token at the authoritative store.

**Intent.** Prevent the strongest available local guarantee from being read as a distributed
one. A move consumes a local value; stored facts are read, copied, and replayed, so no local
move can consume them.

**Applicability.** Protocols whose stages correspond to persisted lifecycle states, and any
mapping of a typed protocol onto database procedures or stored state.

**Allowed exceptions.** None for the claim. A protocol that never advances durable state states
that limit instead.

**Review evidence.** The authoritative-transition query or procedure, its concurrency token, the
guarantee ledger row separating local from durable proof, and competing-writer evidence.

**Enforcement.** Unenforceable: No durable store or competing-writer test exists; EVIDENCE.md
records this as unevidenced

## RUST-DOC-0010-R015 — Keep persisted or multi-actor lifecycle in a runtime model

**Statement.** Where protocol state is persisted, inspected heterogeneously, or advanced by more
than one actor, the durable model MUST be a runtime representation, and the typed stage protocol
MUST be scoped to one in-process pass that is issued by checked construction.

**Intent.** Keep a mechanism that is sound for a local sequence from being extended to a durable
lifecycle it cannot govern.

**Applicability.** Registration, onboarding, payment, approval, fulfillment, and any workflow
with durable status and several participants.

**Allowed exceptions.** A protocol that runs entirely within one process and stores nothing MAY
omit the runtime model.

**Review evidence.** The persisted representation, the restoration path that issues a typed
stage, and the conversion contract between the two.

**Enforcement.** Unenforceable: No persisted model or restoration path in repo; EVIDENCE.md records
this as unevidenced

## RUST-DOC-0010-R016 — State the async stage contract

**Statement.** An asynchronous transition MUST state its cancellation behavior, whether retry is
safe, the identity under which a retry is deduplicated, and whether the successor proof exists
only after a durable acknowledgment.

**Intent.** Keep an interrupted transition from silently producing a successor whose proof was
never completed.

**Applicability.** Transitions that await I/O, cross a process boundary, or can be cancelled.
RUST-DOC-0004 governs cancellation mechanics; this rule requires the contract per stage.

**Allowed exceptions.** A transition that performs no external effect and holds no resource MAY
state that cancellation is inconsequential.

**Review evidence.** Per-stage cancellation table, idempotency identity, retry policy, and fault
tests at each interruption point.

**Enforcement.** Unenforceable: Example has no async transition; no cancellation table, retry
identity, or fault test

## RUST-DOC-0010-R017 — Erase the protocol only at a named boundary

**Statement.** Type erasure of protocol state into trait objects, maps, dynamic contexts, or
serialized documents MUST occur at a named orchestration or persistence boundary, and MUST NOT
occur between stages.

**Intent.** Keep the stage graph checkable for its whole length, since an erased intermediate
value ends static enforcement for every stage after it.

**Applicability.** Orchestration layers, dynamic strategy selection, and persistence adapters.

**Allowed exceptions.** Runtime selection among protocol implementations MAY be dynamic while
each selected branch continues to advance through typed stages.

**Review evidence.** The named boundary, what is erased there, and the reason earlier erasure is
unnecessary.

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— origin erasure happens once, at the persistence boundary

## RUST-DOC-0010-R018 — Prove the prohibited orderings

**Statement.** Illegal stage orderings, reuse of a consumed stage, and construction of stage
evidence outside its transition MUST have compile-fail evidence when the protocol claims those
programs are unrepresentable.

**Intent.** Keep a claimed impossibility from silently becoming possible during refactoring.

**Applicability.** Every negative guarantee a staged protocol states.

**Allowed exceptions.** A prohibition enforced only at runtime MUST be stated as a runtime check
rather than given compile-fail evidence it does not have.

**Review evidence.** Compile-fail cases, their reviewed diagnostics, and confirmation that each
rejection occurs at the intended boundary.

**Enforcement.** [`examples/compile-fail/tests/ui.rs`](../../examples/compile-fail/tests/ui.rs) —
trybuild runs the four staged-protocol rejections

## RUST-DOC-0010-R019 — Assert the stage graph executably

**Statement.** The stage and successor graph a protocol documents MUST be asserted executably,
so that a redirected associated type, a widened bound, or a removed implementation is detected
by the build rather than by reading. At least one assertion per capability MUST derive the
successor's required capability from the stage capability alone, and MUST NOT restate that
requirement as its own bound.

**Intent.** Keep the documented topology and the compiled topology from diverging, which is the
failure that prose review is least able to catch.

**Applicability.** Protocols with more than two stages or more than one branch.

**Allowed exceptions.** A protocol whose complete graph is visible in one function signature MAY
rely on that signature.

**Review evidence.** The contract assertions, the edge assertions, their coverage of every
documented edge, and an observed compiler failure when a successor bound is deleted from a
capability. An assertion whose own bounds restate the trait's obligation is not evidence for
this rule.

**Enforcement.** [`examples/staged-protocol/src/lib.rs`](../../examples/staged-protocol/src/lib.rs)
— contract assertions derive successors from the trait alone

## RUST-DOC-0010-R020 — Record a guarantee ledger row per stage

**Statement.** Each stage MUST have a guarantee ledger row stating the claim it establishes, the
transition that establishes it, how its construction is protected, how boundary decoding
preserves it, its escape hatches, what it does not prove, and the residual runtime risk.

**Intent.** Keep the protocol's honesty auditable at the granularity at which its claims are
made.

**Applicability.** Every stage type and every piece of stage evidence.

**Allowed exceptions.** None.

**Review evidence.** The completed ledger and its agreement with the stage definitions.

**Enforcement.** [`improved.md`](../../case-studies/registration-onboarding/improved.md) — a
guarantee ledger row per stage

## RUST-DOC-0010-R021 — Keep protocol terminology honest

**Statement.** Documentation for a staged protocol MUST NOT present project vocabulary as
standardized external terminology, and MUST identify the established family a mechanism belongs
to when it uses a local name for it.

**Intent.** Keep a useful local vocabulary from being cited as external authority it does not
have.

**Applicability.** Doctrine text, design notes, API documentation, and agent instructions that
name a protocol mechanism.

**Allowed exceptions.** Terms defined by a cited specification or published literature MAY be
used as standard when the citation is given.

**Review evidence.** Terminology definitions, their family attribution, and the source notes
recording which vocabulary is local.

**Enforcement.** [`doctrines/0010-staged-protocols/glossary.md`](../../doctrines/0010-staged-protocols/glossary.md)
— the local term is marked local and attributed

## RUST-DOC-0010-R022 — Partition protocol authority explicitly

**Statement.** Each claim a staged protocol makes MUST be classified as an in-process claim the
executable protocol mechanically enforces, a durable or remote claim an external system owns, or
a rationale, non-guarantee, waiver, or change-authority claim its governing records own. The
executable protocol MUST be treated as authoritative for the ordering, successor constraints,
construction restrictions, and negative capabilities it mechanically enforces. An artifact
governing one of these classes MUST NOT be maintained as a second, independently edited source
for another class.

**Intent.** Replace a precedence contest with a partition. The accurate observation that code
enforces ordering does not make code the source of rationale, accepted risk, or change authority;
the accurate observation that doctrine governs change does not make doctrine a second description
of what the program currently permits. `RUST-DOC-0010-R018` and `RUST-DOC-0010-R019` exist
because prose cannot detect a widened bound or a redirected successor, and a rule subordinating
the executable protocol to prose would contradict them.

**Applicability.** Design notes, doctrine text, decision records, review records, and agent
instructions that state which artifact settles a question about a staged protocol. RUST-DOC-0011
governs the partition generally, including the decision-record obligations; this rule applies it
to stages, edges, and stage evidence.

**Allowed exceptions.** A generated or mechanically checked view of the executable protocol MAY
restate its topology, because such a view cannot drift from the artifact it is derived from.

**Review evidence.** The claim classification, the executable artifact cited for each in-process
claim, the external check cited for each durable claim, and the governing record cited for each
rationale, non-guarantee, waiver, or change-authority claim.

**Enforcement.** Unenforceable: No per-claim authority classification exists for this protocol; only
gates state it

## Guarantee and non-guarantee requirements

A staged protocol states, for each stage and each piece of stage evidence: the claim its
construction establishes under RUST-DOC-0010-R002; how construction is protected under
RUST-DOC-0010-R010 and RUST-DOC-0010-R011; how decoding and restoration preserve or re-establish
it under RUST-DOC-0010-R015; its escape hatches under RUST-DOC-0010-R011; the external facts
that remain mutable under RUST-DOC-0010-R014; the failures that remain runtime failures under
RUST-DOC-0010-R007; the outcomes that can remain indeterminate under RUST-DOC-0010-R016; and the
executable evidence supporting the claim under RUST-DOC-0010-R018 and RUST-DOC-0010-R019.

## Boundary requirements

Untrusted input enters at the initial stage and is canonicalized under RUST-DOC-0010-R006 before
any stage claims a checked value. Persistence and wire boundaries follow RUST-DOC-0010-R015 and
RUST-DOC-0010-R017: durable state is a runtime model, erasure is named, and a typed stage is
issued only by checked construction. Durable advancement follows RUST-DOC-0010-R014 and re-checks
identity, stored state, and a concurrency token. Sensitive values carried as stage evidence
remain subject to RUST-DOC-0003 secret handling, and failure mapping at the outer boundary
follows RUST-DOC-0010-R007.

## Waiver requirements

RUST-DOC-0010-R012, RUST-DOC-0010-R016, and RUST-DOC-0010-R019 MAY be waived for a protocol
whose scope, lifetime, or effect makes the obligation disproportionate. A waiver records the
affected rule and protocol, the owner accepting the risk, the consequence, the compensating
control, an expiry or reconsideration trigger, and the removal condition.

RUST-DOC-0010-R003, RUST-DOC-0010-R004, RUST-DOC-0010-R010, RUST-DOC-0010-R011,
RUST-DOC-0010-R014, RUST-DOC-0010-R020, RUST-DOC-0010-R021, and RUST-DOC-0010-R022 MUST NOT be
waived. A waiver cannot make a bypassed protocol sound, cannot convert a local move into a
durable transition, and cannot make an inaccurate external claim true.
