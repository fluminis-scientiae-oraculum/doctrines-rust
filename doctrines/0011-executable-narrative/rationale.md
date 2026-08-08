# Rationale

## Failure modes

**The obligation that only prose held.** A design states that authentication precedes
authorization. Nothing enforces it. A refactor reorders two calls, every test passes because
every test exercises the ordinary path, and the document still says the right thing while the
program does the wrong one. The document's correctness is what makes this hard to find: a
reviewer reads the sentence, believes the system, and never opens the call site.
`RUST-DOC-0011-R002` moves an enforceable obligation into the mechanism that enforces it.

**The second copy that drifted.** A protocol's stage graph is enforced by types and also
described by a table in an architecture document. One of them is updated. Nothing announces which
is now wrong, and a reader who consults the table forms a false model of the running system with
no signal that anything is amiss. Two editable descriptions of one obligation are two obligations.
`RUST-DOC-0011-R004` prohibits the second, and `RUST-DOC-0011-R005` prefers generation so the
view cannot drift at all.

**The record that outlived its reason.** A record states that a component was chosen over an
alternative because of a constraint that held at the time. The constraint disappears; the record
does not. It stays discoverable, it stays plausible, and a reviewer or an agent cites it against
a change that is now clearly better. The improvement then requires arguing against a document
rather than against a constraint, and the person best placed to know the constraint is gone has
no standing that the document lacks. `RUST-DOC-0011-R009` requires an end condition and
`RUST-DOC-0011-R010` requires applicability to be confirmed before the record is cited.

**The record that was written because the decision felt large.** A team debates a choice at
length, the discussion is valuable, and a record is written to preserve it. The record describes
the module layout, the interfaces, and the ordering, all of which the code carries and enforces.
Nothing in it is irrecoverable. What it produces is a second source that a future maintainer must
reconcile against current behavior, and a permanent artifact whose only stated justification is
that the decision mattered. `RUST-DOC-0011-R006` requires the residue to be named, and no rule in
this package permits a record on the grounds that a decision was important, debated, complex, or
memorable. Those are arguments for better names, types, tests, and examples.

**The rationale that was inferred.** A constraint exists in code with no recorded reason. A
reviewer reconstructs a plausible motive from the surrounding implementation and writes it down.
The inference is now indistinguishable from a recorded decision at the point of use, and it will
be cited to defend a constraint whose actual reason may have been different or may have expired.
`RUST-DOC-0011-R013` requires the unknown to stay unknown, or the inference to be labelled as
one.

**The local guarantee read as an external fact.** A type proves that a local sequence ran. A
reader concludes that a row was written, a message was delivered, or a provider agreed. The
inference is natural because the local guarantee is the strongest one visibly available, and it
is wrong because the external system was never consulted. `RUST-DOC-0011-R014` requires each
external fact to name the system authoritative for it, and this doctrine's partition keeps the
two classes apart by construction rather than by care.

**The precedence rule that answered the wrong claim.** This corpus recorded a source claim as
"code is a sufficient contract", rejected it, and encoded the rejection as a blanket precedence
of governance over the executable protocol. The source had not made that claim. What resulted was
a rule stating that the executable protocol does not settle what the system is obliged to do,
sitting in a package whose `RUST-DOC-0010-R018` and `RUST-DOC-0010-R019` exist precisely because
prose cannot detect a widened bound or a redirected successor. The package argued the partition
and then denied it. The defect was live in this corpus until the restatement recorded in RFC-0003.

**The index nobody regenerated.** [`rfcs/accepted/README.md`](../../rfcs/accepted/README.md) listed RFC-0001 and omitted
RFC-0002 within a single release, because the index of accepted proposals is maintained by hand beside a directory that
already contains the answer. The cost was small and the mechanism is the general one: a hand-maintained view of a
machine-readable fact is wrong as soon as attention lapses. `RUST-DOC-0011-R005` prefers generation, and
`RUST-DOC-0011-R017` makes the count of maintained representations something a review states rather than estimates.

## Why weaker alternatives fail

**"Documentation is important, and so is code."** True and useless. It resolves no question,
because every disputed claim has two artifacts describing it and this position tells a reviewer
nothing about which to trust. The partition is what makes the sentiment actionable: for an
enforced claim the artifact wins, for an external fact the external system wins, and for a
rejected alternative only the record has anything to say.

**Blanket precedence in either direction.** "Code is the contract" cannot express an external
mandate, an accepted residual risk, or who may change an interface. "Documentation governs" turns
a stale sentence into an authority over a running mechanism. Both are attempts to answer a
question about five classes of claim with one rule.

**Diligence.** Requiring people to keep two representations synchronized assigns an obligation
that has no failure signal. Nothing breaks when the copy goes stale, so nothing surfaces it, and
the discovery happens later at a reader who trusted the wrong one.
`RUST-DOC-0011-R005` removes the obligation rather than assigning it.

**A record for every architectural change.** This is the practice the doctrine restricts, and its
cost is not the writing. It is the accumulating set of plausible, discoverable, unowned documents
that a future reader cannot distinguish from current constraints, and that an agent will hydrate
as context. The set grows monotonically unless expiry is built in, which is why
`RUST-DOC-0011-R007` and `RUST-DOC-0011-R009` are separate obligations from
`RUST-DOC-0011-R006`.

**Prohibiting records entirely.** An external mandate, an irreversible commitment, and an
accepted residual risk are real facts that no artifact carries. Prohibiting the record does not
remove the fact; it relocates it to a commit message or an issue thread, where it has no owner,
no expiry, and no place in review.

## Interaction with external reality

Rationale is also subject to external change. A regulatory interpretation, a contractual
obligation, and a vendor restriction can all lapse without anything in the repository moving,
which is why `RUST-DOC-0011-R009` attaches an obsolescence condition to the record rather than to
a review calendar.

## Costs and overapplication

Classifying a claim before citing an authority for it is a real step, and on a small change it
can cost more than the confusion it prevents. Counting maintained representations is a review
obligation that adds time to design review. Generation adds a build step, a generator to
maintain, and a drift check that fails on unrelated changes until it is understood.

The doctrine is overapplied when it becomes an argument for deleting rationale. The rules that
prohibit a competing copy are about copies of _enforced_ claims; the constraint that shaped a
design, the alternative that was rejected, and the risk somebody accepted are not enforced by
anything and have no other home. `RUST-DOC-0011-R012` and `RUST-DOC-0011-R013` are the guards,
and a reviewer who cites this doctrine to remove an irrecoverable reason has inverted it.

It is also overapplied when generation is demanded for a view whose generator would need a
hand-maintained input describing the same claim. That input is the competing copy wearing the
word "generated", and `RUST-DOC-0011-R005` names the case explicitly so it does not have to be
rediscovered.

## Evidence limits

Most of this doctrine is judgment. The linter opens each registered record and validates, from
that record's own front matter, that it has a well-formed unique identifier, a title, an owner, a
scope, and for an active record a revalidation trigger, an obsolescence condition, and executable
authorities that resolve to real files. It cannot decide whether the record should exist, whether
the stated justification is honest, or whether an obligation carried by prose could have been
carried by a type. Those are
`RUST-DOC-0011-R002`, `RUST-DOC-0011-R003`, `RUST-DOC-0011-R006`, `RUST-DOC-0011-R012`,
`RUST-DOC-0011-R013`, `RUST-DOC-0011-R016`, and `RUST-DOC-0011-R017`, and they are supported by
review gates rather than by executed evidence.

Deterministic bundle generation with drift detection is real evidence for the mechanism
`RUST-DOC-0011-R005` prefers, in this repository, for these bundles. It is not evidence that any
view elsewhere is generated, and it is not evidence that the canonical sources feeding it are
themselves free of duplication.

The registry ships with an empty active set. That is the state this doctrine predicts for a
repository whose obligations are carried by doctrine, manifests, schemas, and a linter, and the
linter validates the registry as such. An empty set is evidence about the record set and not
about the constraints this repository is under; an unrecorded constraint leaves no trace in a
registry designed to hold only what somebody chose to record.
