# Executable narrative

## 1. Problem

An architectural obligation has to be somewhere. A team decides that authentication precedes
authorization, that a transaction identifier is never compared with a wallet identifier, or that a
capture cannot be attempted twice. Each of these is a sentence someone can write down, and each is
also something a mechanism could reject.

Written down, the sentence is cheap and immediately readable, and nothing contradicts it when the
code stops matching. Enforced, it changes when the system changes and fails loudly when it is
violated, but it is only useful if a reader can still follow it. The problem is not choosing
between the two. It is deciding which artifact settles which claim, and then not maintaining a
second editable copy of the answer.

## 2. Forces

Obligations differ in whether a mechanism exists for them. Some are enforceable by types, some
only by a schema, some only by a deployment check, and some by nothing available. Readers differ
too: an implementer wants the mechanism, a reviewer wants the reason, an auditor wants the owner.
Records outlive the constraints that produced them, and discoverability turns into authority
without anybody deciding that it should. Generation removes a synchronization obligation but adds
a generator, and a generator fed by a hand-written description has removed nothing. Agents hydrate
whatever context is offered and cannot distinguish a current constraint from an expired one unless
the artifacts do.

## 3. Weak representation

A design document, maintained by hand, states the obligation and describes the enforced structure
beside it:

```text
## Registration protocol

Stages run in this order:

  1. Canonicalize    -> produces CanonicalRegistration
  2. Check identity  -> produces AvailableRegistration or ConflictingRegistration
  3. Accept policy   -> produces ConsentedRegistration
  4. Prepare         -> produces PersistableRegistration

Authentication must precede authorization. Transaction identifiers must never be
compared with wallet identifiers.
```

Three separate failures live in that block. The stage list duplicates a graph the compiler already
enforces, so the two can diverge with no signal. The ordering sentence is enforceable and is not
enforced, so it is a description of an intention. The identifier sentence is enforceable in the
type system and in the schema, and is enforced in neither.

## 4. Improved representation

Move each obligation into the mechanism that rejects violations of it, and keep the document for
what only it can carry.

The ordering becomes the successor bound, so a stage that stops leading anywhere legal fails to
compile:

```rust
pub trait Authenticate: Sized {
    type Next: Authorize;
    type Error;

    fn authenticate(self) -> Result<Self::Next, Self::Error>;
}
```

The negative guarantee becomes a rejected program rather than a claim of impossibility:

```rust
// examples/compile-fail/ui/skip_protocol_stage.rs
// Reaching authorization without authenticating is a compiler error, and the
// committed diagnostic is the evidence that it still is.
```

The identifier distinction becomes nominal in both type systems, so mixing two species requires a
conversion somebody wrote on purpose:

```rust
pub struct TransactionId(Uuid); // private field, checked constructor
pub struct WalletId(Uuid);
```

```sql
-- The same distinction, carried into the schema. A comparison across species
-- needs an explicit cast, so it is visible in review rather than silent.
CREATE DOMAIN transaction_id AS uuid;
CREATE DOMAIN wallet_id      AS uuid;
```

What the document keeps is the part no mechanism carries: why this ordering was chosen, what the
stages deliberately do not prove, which residual risks were accepted, and who accepted them.

## 5. Exact guarantee gained

For each obligation moved into a mechanism, a violation now fails: at compile time for a bound or
a visibility restriction, at test time for a rejected program or input, at deploy time for a
schema constraint or a machine-checked manifest. The claim and its enforcement change together
because they are the same artifact, so the class of defect where a correct document describes an
incorrect system is removed for that obligation.

For each derived view that is generated and drift-checked, divergence from its source fails the
build. That is a guarantee about currency, and it holds without anybody remembering.

## 6. Guarantees not gained

An enforced obligation is not thereby a correct obligation. The compiler agrees that the graph is
the graph; it has no opinion about whether that graph matches the business process, which stays a
review judgment.

A generated view is current, not correct. It faithfully reflects a source that may itself be
wrong, and generating it removes drift without removing error.

An empty decision-record set is evidence about the record set. A constraint nobody recorded leaves
no trace in a registry that holds only what somebody chose to record, so absence of records is not
absence of constraints.

Legibility is not proved by anything mechanical. `RUST-DOC-0011-R016` states the obligation, and
review is the only check on it.

## 7. Boundary considerations

The enforcing mechanism changes at each boundary, and the authority moves with it. Inside the
process, types and visibility carry the obligation. At the wire, the canonical encoder, the
decoder, the schema, and the compatibility suite carry it. At the database, schema constraints,
checked decoding, and transaction predicates carry it. At deployment, machine-checked
configuration carries it.

A claim that crosses into another system's ownership stops being an in-process claim at that
point. Committed state, remote acknowledgment, provider status, current policy, lock ownership,
and settlement are owned by systems that have to be asked, and no local artifact can be cited for
them.

## 8. Persistence considerations

Persist the fact, not the enforcement artifact. A stage marker, a type name, and a trait bound are
compile-time artifacts whose spelling in a column establishes nothing. The durable representation
is a runtime model, and the obligation at the persistence boundary is carried by constraints,
checked decoding, and predicates.

The schema is a mechanism in its own right, and often the strongest one available for an
obligation that several services share. A nominal distinction expressed only in one service's
types is enforced for that service; expressed as a base type or domain in the schema, it is
enforced for every writer.

## 9. Testing evidence

Test what the mechanism rejects, not only what it accepts. A compile-fail fixture for a claimed
impossibility, a rejected-input case for a checked constructor, and a rejected-cast case for a
schema distinction are each the evidence that the negative guarantee is real. A green suite of
positive tests demonstrates that the ordinary path works and says nothing about the obligation.

Add a drift check for each generated view. Generation without a check is a convention; generation
with a check is a guarantee that the view matches its source at build time.

An assertion or gate that has never been observed failing is not evidence. Delete the thing it
protects, confirm the build breaks, and restore it. That step is what distinguishes a real check
from a decoration, and it is cheap.

## 10. Costs

Enforcement costs are real and specific. A bound lengthens signatures and reports a mismatch as an
unsatisfied bound rather than as a plain type error. A schema constraint moves a failure from a
readable domain message into a driver error. A generated view adds a generator to maintain and a
drift check that fails on unrelated changes until its cause is understood. Classification adds a
step to every review, and counting representations adds another.

The offsetting saving is that the common outcome of the executability test is that no record is
written at all, and every record not written is a permanent artifact nobody has to revalidate,
expire, or reconcile.

## 11. When not to use it

Do not force enforcement where the cost exceeds the consequence. An advisory ordering, a
convention with no failure mode, and a preference that has never been violated are all reasonable
to leave in prose with the assessment recorded.

Do not generate a view whose generator would need a hand-maintained description of the same
claim. That input is the competing copy under another name, and an informative hand-written
diagram with a named owner is the more honest artifact.

Do not use this pattern as an argument for deleting rationale. The prohibition covers copies of
enforced claims. A rejected alternative, an external constraint, and an accepted risk have no
other home, and removing them destroys the only record of them.

Do not read it as a prohibition on decision records. The residue is real, and pushing it into a
commit message or an issue thread relocates the fact to somewhere with no owner and no expiry.

## 12. Related doctrines

RUST-DOC-0011 governs this pattern, the authority partition, the prohibition on competing copies,
and the decision-record policy. RUST-DOC-0010 applies the partition to staged protocols in
`RUST-DOC-0010-R022`, and its compile-fail and topology-assertion rules are worked instances of an
obligation moved into an artifact because prose could not detect its violation. RUST-DOC-0001
governs which invariants are representable at all. RUST-DOC-0005 governs the persistence
mechanisms an obligation moves into at the database boundary. RUST-DOC-0006 governs the ambiguity
that remains once a claim becomes external. RUST-DOC-0008 governs which evidence class supports
which claim.

## 13. Executable example

This repository is its own worked instance of the generated-view half.
[`tools/bundle-agent-context`](../tools/bundle-agent-context/) builds every [generated distribution](../dist/README.md)
from the canonical sources named in [`../manifest/doctrines.yaml`](../manifest/doctrines.yaml) and
[`../manifest/agents.yaml`](../manifest/agents.yaml), stamps each output with a banner naming the canonical roots, and
its `check` mode fails on drift. No file under `dist/` is edited by hand, and the drift check is part of the ordinary
validation set.

[`tools/doctrine-lint`](../tools/doctrine-lint/) is the enforcement half. It validates the decision-record registry at
[`../manifest/decision-records.yaml`](../manifest/decision-records.yaml) against its schema and
against the obligations `RUST-DOC-0011-R007` states, so a record without an owner, a revalidation
trigger, an obsolescence condition, or resolvable executable authorities fails the build rather
than a review.

No protocol-graph generator is shipped. A generator that derived the stage graph from the trait
definitions would be a further instance of this pattern; one fed by a hand-written edge list would
be the competing copy this pattern prohibits, wearing the word "generated". The graph obligation
is carried instead by the contract and edge assertions in
[`../examples/staged-protocol/src/lib.rs`](../examples/staged-protocol/src/lib.rs), which fail the
build when an edge changes.

## 14. Worked application

**A decision that needs no record.** A team decides that authentication precedes authorization.
The executability test finds a mechanism for every part of it: the ordering becomes a successor
bound, the impossibility of skipping becomes a committed compile-fail diagnostic, and the graph
becomes a contract assertion that fails if the bound is deleted. Nothing remains that an artifact
cannot carry, so no record is written. Someone who later asks why the order is what it is finds a
short comment and a doctrine cross-reference; someone who asks whether the order holds reads the
bound.

**A decision that needs a narrow record.** A system is required to keep a class of data inside one
jurisdiction because of an obligation a regulator and a customer contract impose. Deployment
regions, storage endpoints, and replica placement are enforceable, and they become
policy-as-code with a machine-checked manifest. What no artifact carries is the interpretation of
the obligation, the accountable owner, and the condition under which it lapses.

That residue is one narrow record. It states the single question it answers, states that it does
not govern the deployment topology, links the policy-as-code that is authoritative for current
behavior, names the owner, and names a revalidation trigger and an obsolescence condition. It is
registered in the active set so it can be audited and expired. The worked form is in
[`../decisions/examples/justified-data-residency.md`](../decisions/examples/justified-data-residency.md).

**A record that should not exist.** A proposed record titled "Authentication must happen before
authorization" restates an obligation the successor bound enforces. It names no fact an artifact
cannot carry, so it fails the test at the first question and is not written. The rejected form,
with the reasoning, is in
[`../decisions/examples/rejected-authentication-order.md`](../decisions/examples/rejected-authentication-order.md).

## 15. Review prompts

- Which class does this claim belong to, and which single artifact is authoritative for it?
- Is there a mechanism that could reject a violation, and was it used?
- If the mechanism enforces part of the claim, is the remainder stated separately?
- How many maintained representations of this claim exist, and which are neither authoritative,
  generated, nor irrecoverable rationale?
- Is any derived view synchronized by hand, and could it be generated instead?
- Would the generator need a hand-maintained description of the same claim?
- Does every generated artifact declare its source, and does a drift check cover it?
- Which fact in this proposed record cannot be represented, enforced, generated, or recovered?
- Who owns the record, what event revalidates it, and what condition ends it?
- Is this record actually a change proposal, and therefore an RFC?
- Was any record cited against a change without confirming its constraint still applies?
- Is any recorded reason an inference from the implementation rather than a governing rationale?
- Does any local guarantee stand in for a durable, remote, or externally governed fact?
