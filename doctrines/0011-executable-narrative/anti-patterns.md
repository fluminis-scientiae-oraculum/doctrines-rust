# Anti-pattern catalogue

## Ordering held only by a document

**Weak example.** A design document states that authentication precedes authorization, and the
call sites depend on developers reading it.

**Why it fails.** Nothing contradicts the document when the code stops matching it. The sequence
is enforceable by types, so the obligation had a mechanism available and was left in the one
place that cannot fail.

**Risk.** A reordering passes review because the reviewer read the sentence and believed the
system.

**Improved direction.** Make the successor of authentication something only authorization
accepts, and let the document explain why the order exists rather than assert that it holds.

**When justified.** When the ordering is advisory, or when the stages are chosen externally at
runtime, in which case the runtime model owns it and the document says so.

## Architecture document that restates the enforced graph

**Weak example.** A protocol's stages are enforced by types and also tabulated in an architecture
overview, kept current by whoever remembers.

**Why it fails.** Two editable descriptions of one obligation are two obligations. When they
diverge, neither announces it, and the reader who consults the table forms a false model with no
signal.

**Risk.** A design decision made against a stale table, then defended by citing it.

**Improved direction.** Generate the view from the enforcing artifact and check it for drift, or
mark it informative, name its owner, and point it at the authority.

**When justified.** Never as an independently maintained normative source. A generated view, a
drift-checked view, or a dated informative illustration is the acceptable form.

## Generator fed by a hand-maintained description

**Weak example.** A protocol diagram is generated from an edge list that a developer types in and
updates by hand after changing the traits.

**Why it fails.** The edge list is the competing copy the doctrine prohibits, and calling the
output generated conceals that. The generator proves the diagram matches the list, not that the
list matches the code.

**Risk.** Confidence in a view whose only guarantee is internal consistency with a second
manually maintained artifact.

**Improved direction.** Derive the view from the enforcing artifact, or leave it informative and
owned and stop describing it as generated.

**When justified.** When the input is itself the authority, as a manifest is, and the code is
validated against it rather than the other way round.

## Decision record for a decision the code carries

**Weak example.** A record titled "we will validate input at the boundary" describes the module
layout, the interfaces, and the ordering, all of which the types enforce.

**Why it fails.** Nothing in the record is irrecoverable. It creates a second source, drifts
independently, and gives a future maintainer something to reconcile against current behavior.

**Risk.** A record cited years later as the reason a boundary cannot move, when the code has long
since moved it.

**Improved direction.** Delete the record. If the design is hard to follow, improve the names, the
types, the tests, and the worked examples.

**When justified.** Never for the recoverable part. If some fact in the same decision is genuinely
external, that fact becomes a narrow record and the rest does not.

## Record justified by the importance of the decision

**Weak example.** "This was a major architectural decision, so it deserves a record."

**Why it fails.** Importance is not a property that makes a fact irrecoverable. The justification
names no fact that cannot live elsewhere, so it cannot be evaluated, and no future reader can
tell when the record stops applying.

**Risk.** A record set that grows monotonically, each entry plausible and none removable.

**Improved direction.** Name the exact fact that no artifact can carry. If naming it is not
possible, that is the answer.

**When justified.** Never. The valid categories are external mandate, irreversible or externally
expensive commitment, a rejected alternative whose rejection depends on evidence the code does
not carry, a decision no single system owns, accepted residual risk, and a compatibility
obligation from shipped behavior.

## Active record with no owner and no end

**Weak example.** A record states a constraint, names no accountable role, and has no condition
under which it stops applying.

**Why it fails.** Nobody can retire it, so it survives every review that does not specifically
attack it. Its authority grows with age because nothing about it changes while everything around
it does.

**Risk.** An obsolete constraint enforced by deference.

**Improved direction.** Give every active record an owner, a revalidation trigger, an obsolescence
condition, and links to the artifacts that are authoritative for current behavior.

**When justified.** Never for an active record. A record with no owner belongs in the archive,
marked as not current authority.

## Historical record used to block an improvement

**Weak example.** A reviewer rejects a change by citing a record from three years ago, without
checking whether its constraint still holds.

**Why it fails.** The record states what was decided under conditions that held then.
Discoverability is not authority, and the person proposing the improvement is asked to argue
against a document that may describe nothing current.

**Risk.** Improvement blocked by an expired constraint, and the reviewer's citation treated as
governance because it is written down.

**Improved direction.** Confirm the constraint is still applicable, the revalidation condition is
satisfied, and the implementation still depends on it. If confirmation is not available, record
the citation as an open question rather than as a constraint.

**When justified.** When the constraint is confirmed current, in which case the confirmation and
its date belong in the review record.

## Accepted proposal kept as a specification

**Weak example.** An accepted RFC continues to be cited for what the system does, after doctrine
and code have carried the contract for several releases.

**Why it fails.** The proposal described an intended change at a point in time. Maintaining it as
a description of current behavior creates the reconciliation obligation the doctrine exists to
remove.

**Risk.** A maintainer implementing to the proposal rather than to the contract.

**Improved direction.** Cite doctrine and the executable artifacts for current behavior, and cite
the proposal for its decision, date, owners, conditions, and rejected alternatives.

**When justified.** For the decision history itself, which is rationale and has no other home.

## Rationale inferred from the implementation

**Weak example.** A reviewer cannot find why a constraint exists, reconstructs a plausible motive
from the surrounding code, and writes it into the documentation as the reason.

**Why it fails.** The inference is indistinguishable from a recorded decision at the point of use.
It will be cited to defend the constraint, and the actual reason may have been different or may
have expired years ago.

**Risk.** A fabricated authority, created in good faith, that outlives every person who could
correct it.

**Improved direction.** Record the rationale as unknown. If the inference is useful, label it as
an inference, name its evidence, and state that the governing reason is unavailable.

**When justified.** Never unlabelled. A labelled inference with its evidence is honest and useful.

## Local guarantee presented as an external fact

**Weak example.** A design states that a record was persisted because a local transition consumed
its handle and produced a persistable value.

**Why it fails.** The local guarantee describes one process. The durable fact is owned by a store
that was never consulted, and no local mechanism can establish it.

**Risk.** A system that reports success for an effect that did not occur, with a type signature
that appears to justify the report.

**Improved direction.** Name the external system authoritative for the fact and the check that
consults it, and keep the local claim scoped to the sequence it actually proves.

**When justified.** Never. The two claims are different classes and stay separate.

## Doctrine cited for what the program currently permits

**Weak example.** A reviewer answers "can this call happen before that one" by quoting a doctrine
package rather than by reading the trait bounds that decide it.

**Why it fails.** Doctrine states obligations. What a program currently permits is decided by the
program, and a doctrine sentence can be correct about the obligation while the code has drifted
from it, or stale while the code is right.

**Risk.** A review that certifies the obligation and misses the violation.

**Improved direction.** Cite the enforcing artifact for what the program permits, and cite
doctrine for whether that is what it ought to permit. Both answers are needed; they are answers
to different questions.

**When justified.** Never as a substitute. Doctrine remains the authority for the obligation, for
the review process, and for who may change the contract.

## Deleting rationale in the name of executable architecture

**Weak example.** A cleanup removes a design note explaining why an alternative was rejected,
citing this doctrine's preference for executable artifacts.

**Why it fails.** The prohibition covers manually maintained copies of _enforced_ claims. A
rejected alternative, an external constraint, and an accepted risk are enforced by nothing and
have no other home, so removing them destroys the only record of them.

**Risk.** A future team re-adopting a rejected alternative because the reason for rejecting it was
deleted as duplication.

**Improved direction.** Trim the parts that restate the enforced topology, keep the parts that
cannot be recovered, and name the enforcing artifact where the two meet.

**When justified.** Never for irrecoverable rationale. Removing a restatement of the enforced
graph from the same document is the correct edit.
