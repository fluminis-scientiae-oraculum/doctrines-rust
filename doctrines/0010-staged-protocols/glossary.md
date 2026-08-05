# Glossary

Terms whose meaning here is narrower than ordinary Rust or architecture usage. Shared vocabulary
lives in the foundations.

**Staged protocol**
: An in-process sequence in which each stage establishes a fact later stages depend on. Narrower
than "workflow": a workflow may be durable, distributed, and multi-actor, while a staged
protocol is one owner's pass through a sequence within one process.

**Stage**
: A distinct type whose construction proves that its preceding transition completed. Narrower
than "state": a stage carries the evidence of the transition that produced it, and a state may
be a mere marker.

**Transition**
: The operation that consumes one stage and produces its successor. A transition is a protocol
edge, not any method that happens to return a new value.

**Capability**
: A trait describing the operation legal at a stage, together with the successor that operation
produces. Distinguish from a capability in the authority sense governed by RUST-DOC-0003, where
possession conveys permission; a stage capability conveys position in a protocol, not authority.

**Successor capability**
: The bound placed on a capability's associated output type, naming what the next stage is
required to satisfy. This is the protocol edge in checkable form.

**Protocol edge**
: One legal transition from one stage to one successor. Edges include branch alternatives and
recovery paths, not only the success path.

**Stage evidence**
: A value constructible only by a specific transition, whose possession is proof that the
transition ran. Narrower than "data carried by a stage": ordinary payload is not evidence unless
its construction is protected.

**Protocol topology**
: The complete graph of stages and edges a protocol documents. The topology is asserted
executably so that documentation and compilation cannot diverge.

**Topology assertion**
: An executable check that each documented edge still typechecks. It proves the edges named
still exist; it does not prove the graph is right for the domain.

**Collapsed view**
: The call site read as a sequence of transitions. It summarizes the protocol and is not itself
evidence of anything.

**Expanded view**
: The stages, evidence, failures, branches, and effects the collapsed view abbreviates.

**Terminal stage**
: A stage with no legal successor, including a recovery stage that ends an attempt. A terminal
stage names no successor capability.

**Undetermined outcome**
: A transition result in which the fact could not be observed. It is distinct from both a
success branch and a modeled rejection, and it belongs in the stage-identifying failure type.

**Durable advancement**
: A change to authoritative stored state. Distinct from a local transition: a local transition
consumes a value, while durable advancement requires re-checking identity, stored state, and a
concurrency token. No local move consumes a stored fact.

**Trusted construction path**
: A restricted way to build a stage or its evidence without running the transition, used for
checked restoration, migration, or testing. It carries an owner, a stated caller obligation, and
a guarantee-ledger entry.

**Chainable Telescopic Typestate Traits (CT³)**
: Local project vocabulary for the mechanism this doctrine governs, recorded here so the term is
recognizable in older internal documents. It is not standardized external terminology. The
established families it refines are typestate-oriented programming, behavioral types, and object
protocols; the specific mechanism is a consuming transition with an associated successor type
bounded by the next capability. Each word carries part of the mechanism. _Typestate_: the current
type proves the current state. _Trait_: the stage exposes the behavioral capability legal at that
state. _Chainable_: the legal happy path composes into a readable sequence, the collapsed view.
_Telescopic_: a chain gives order, `A → B → C`, while a telescope gives containment — A holds the
controlled opening into B, and B holds the controlled opening into C. The associated successor
type is that opening, so a stage carries both proof of completed history and permission for a
constrained future. Prefer the descriptive terms above in new material, per
`RUST-DOC-0010-R021`.

**Authority partition**
: The assignment of each protocol claim to exactly one authority: the executable protocol for what
it mechanically enforces, an external system for a durable or remote fact, and a governing record
for rationale, non-guarantees, waivers, and change authority. Narrower than "precedence": the
partition does not rank artifacts, it assigns claims. `RUST-DOC-0010-R022` states it for staged
protocols and RUST-DOC-0011 governs it generally.

## Glossary review

- every normative term is defined in the foundations or here;
- no definition implies a stronger guarantee than construction establishes;
- local vocabulary is marked as local and attributed to its established family;
- observations record their scope and the moment they were taken;
- abbreviations expand on first use;
- links point to the authoritative rule or foundation.
