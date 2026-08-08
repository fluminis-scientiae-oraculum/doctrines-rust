# Normative doctrine

## RUST-DOC-0011-R001 — Classify a claim before assigning its authority

**Statement.** An architectural claim MUST be classified, before any artifact is cited as its
authority, as an in-process claim that executable structures enforce, a durable or remote claim
an external system owns, rationale or historical context, a stated non-guarantee or accepted
residual risk, or a governance claim about who may change a contract. One artifact MUST NOT be
cited as the authority for every class.

**Intent.** Replace precedence arguments between code and documents with a partition, so that a
question about what a program currently permits and a question about who accepted a residual risk
are answered by different artifacts rather than by whichever artifact is nearer to hand.

**Applicability.** Design notes, doctrine text, decision records, review records, and agent
instructions that state what settles a question about a system's architecture.

**Allowed exceptions.** None. A claim whose class cannot be named is evidence that the claim is
not yet stated precisely enough to review.

**Review evidence.** The claim classification and the single artifact cited as authority for each
classified claim.

**Enforcement.** Unenforceable: No check classifies claims; classification exists only in review
prose

## RUST-DOC-0011-R002 — Represent an enforceable obligation in the mechanism that enforces it

**Statement.** An ordering, invariant, construction restriction, capability boundary, transition
restriction, or negative guarantee that an available mechanism can enforce mechanically MUST be
represented in that mechanism, and MUST NOT be carried by prose alone.

**Intent.** Keep an obligation that a type, schema, constraint, manifest, or test could enforce
from surviving only as a description that nothing contradicts when it is violated.

**Applicability.** Architectural obligations in systems governed by this corpus, where a
mechanism is available in the language, the schema, the build, or the deployment configuration.
RUST-DOC-0001 governs which invariants are representable; this rule governs whether a
representable obligation was in fact represented.

**Allowed exceptions.** An obligation whose enforcement cost exceeds the assessment required by
[`foundations/complexity-budget.md`](../../foundations/complexity-budget.md) MAY remain prose-carried when the
assessment, its owner, and the residual risk are recorded on the terms of RUST-DOC-0011-R020.

**Review evidence.** The enforcing artifact, or the recorded assessment showing that no available
mechanism enforces the obligation proportionately.

**Enforcement.** [`tools/doctrine-lint/src/main.rs`](../../tools/doctrine-lint/src/main.rs) —
check_rule_enforcement

## RUST-DOC-0011-R003 — Treat the enforcing artifact as the operational authority

**Statement.** Where an executable or machine-checked artifact completely enforces a claim, that
artifact MUST be treated as authoritative for the claim's current operational truth, and any
prose description of the same claim MUST be treated as informative.

**Intent.** Name the artifact a reader, reviewer, or agent should consult for what the system
currently does, so that a stale description cannot be cited against a mechanism that is running.

**Applicability.** Claims about legal ordering, available operations, construction restrictions,
permitted conversions, schema constraints, canonical encodings, visibility boundaries, and
negative guarantees.

**Allowed exceptions.** Where an artifact enforces only part of a claim, prose remains
authoritative for the unenforced part, which MUST be stated separately rather than left implied
by the enforced part.

**Review evidence.** The artifact cited for the claim, and the statement of any part of the claim
it does not enforce.

**Enforcement.** Unenforceable: Nothing detects prose being cited as authority over the enforcing
artifact

## RUST-DOC-0011-R004 — Keep no competing manually maintained copy of an enforced claim

**Statement.** A manually maintained artifact MUST NOT restate an enforced topology, invariant,
interface, or constraint as an independently editable normative source. A human-readable view of
an enforced claim MAY exist only when it is generated from the enforcing artifact, mechanically
checked against it, explicitly marked informative, or confined to rationale and non-guarantees.

**Intent.** Remove the second source that drifts. Two editable descriptions of one obligation
produce two obligations, one of which is wrong and neither of which announces which.

**Applicability.** Protocol tables, state diagrams, interface listings, dependency descriptions,
schema documentation, and architecture overviews that describe an enforced claim.

**Allowed exceptions.** A dated, informative illustration that is not cited as authority MAY be
hand-written. An excerpt quoted for explanation MAY appear in rationale when the enforcing
artifact is named at the point of quotation.

**Review evidence.** The generation command or drift check for each derived view, or the
informative marking and the authority it points to.

**Enforcement.** [`tools/doctrine-lint/src/main.rs`](../../tools/doctrine-lint/src/main.rs) —
check_doctrine_index

## RUST-DOC-0011-R005 — Generate a derived view and declare its source

**Statement.** A derived view of a machine-readable source SHOULD be generated from that source
and checked for drift rather than synchronized by hand, and a generated artifact MUST declare the
source it was generated from and MUST NOT be edited in place.

**Intent.** Convert a recurring synchronization obligation into a build step, so that a view is
current because it was produced rather than because someone remembered.

**Applicability.** Diagrams, tables, interface listings, dependency graphs, distribution bundles,
and agent context packs derived from code, schemas, or manifests.

**Allowed exceptions.** A view whose generator would itself require a hand-maintained input
describing the same claim MUST NOT be generated, because that input is the competing copy
RUST-DOC-0011-R004 prohibits. Such a view stays informative, or the claim is derived from the
enforcing artifact directly.

**Review evidence.** The generator, its declared source, the drift check, and the reason for any
view left hand-written.

**Enforcement.** [`tools/bundle-agent-context/src/main.rs`](../../tools/bundle-agent-context/src/main.rs)
— the drift check, with check_generated_files

## RUST-DOC-0011-R006 — Create a decision record only for what cannot live elsewhere

**Statement.** A decision record MUST NOT be created when the decision can be represented,
enforced, generated, tested, or recovered from executable and machine-readable artifacts. A
record MAY be created only for the part that cannot be: an external mandate, an irreversible or
externally expensive commitment, a rejected alternative whose rejection depends on evidence the
implementation does not carry, a decision no single system owns, an accepted residual risk or
waiver, or a compatibility obligation created by previously shipped behavior.

**Intent.** Make the record the exception. A record duplicates the system, drifts independently
of it, and outlives the constraint that produced it, so its permanent cost is only justified by a
fact the system genuinely cannot carry.

**Applicability.** Every proposal to create an architecture decision record, design note, or
equivalent durable rationale artifact.

**Allowed exceptions.** None. That a decision is large, was debated, is hard to understand, or
may be forgotten is not a fact the executable artifacts cannot carry; those are arguments for
better names, types, tests, generated views, and examples.

**Review evidence.** The executability assessment, the artifact each recoverable part of the
decision now lives in, and the justification required by RUST-DOC-0011-R007 for whatever remains.

**Enforcement.** Unenforceable: Registry stores membership only; no check judges whether a record
should exist

## RUST-DOC-0011-R007 — State the last-resort justification in the record

**Statement.** An active decision record MUST state which fact cannot be represented executably
and why, why a generated view is insufficient, the future decision the record protects, a named
owner, a revalidation trigger, an obsolescence condition, and the executable artifacts that
remain authoritative for current behavior.

**Intent.** Make an active record auditable and removable. A record without an owner and an end
condition cannot be retired, and a record that does not name the current authority invites a
reader to treat it as one.

**Applicability.** Every decision record in the active set, and every record proposed for it.

**Allowed exceptions.** None. A record whose justification cannot be stated in these terms fails
RUST-DOC-0011-R006 and is not created.

**Review evidence.** The record's own metadata and the registry entry that makes it discoverable.

**Enforcement.** [`tools/doctrine-lint/src/main.rs`](../../tools/doctrine-lint/src/main.rs) —
check_active_record

## RUST-DOC-0011-R008 — Keep a decision record narrow

**Statement.** A decision record MUST answer one decision question, MUST state what it does not
govern, and MUST NOT be used as a general description of a system's architecture or as a home for
decisions adjacent to the one it records.

**Intent.** Keep the record's scope small enough that its obsolescence condition can be evaluated.
A record covering several decisions expires in parts, so it never expires at all.

**Applicability.** Every active decision record.

**Allowed exceptions.** None. Several related decisions are several records, each with its own
owner and expiry, or one record and an executable representation of the rest.

**Review evidence.** The record's stated question, its stated exclusions, and the review record
confirming no adjacent decision was folded in.

**Enforcement.** Unenforceable: Only a non-empty scope field is checked; narrowness and exclusions
are not

## RUST-DOC-0011-R009 — Expire a record whose reason has ended

**Statement.** A decision record whose external constraint, commitment, or accepted risk no
longer applies MUST be marked expired or superseded and removed from active discovery, and MUST
NOT remain in the active set because no one revisited it.

**Intent.** A record's danger begins when its reason ends and its text does not. Survival by
inattention is the mechanism by which a correct record becomes a false one.

**Applicability.** Every active decision record, at each of its revalidation triggers and at any
review that observes its obsolescence condition satisfied.

**Allowed exceptions.** A record retained for a stated compatibility or audit obligation MAY
remain discoverable when it is marked as archival and is excluded from the active set.

**Review evidence.** The registry status, the archival marking, and the trigger or condition that
was observed.

**Enforcement.** [`tools/doctrine-lint/src/main.rs`](../../tools/doctrine-lint/src/main.rs) —
check_archived_record and status agreement

## RUST-DOC-0011-R010 — Confirm applicability before citing a record as a constraint

**Statement.** A decision record MUST NOT be cited to block or restrict a change unless its
governing constraint is confirmed still applicable, its revalidation condition is satisfied, and
the current implementation still depends on it.

**Intent.** Remove the veto a historical choice otherwise acquires. A record states what was
decided under conditions that held at the time; discoverability is not authority, and age is not
consent.

**Applicability.** Review comments, planning documents, and agent reasoning that cite a decision
record as a reason a change cannot proceed.

**Allowed exceptions.** None. A citation whose applicability cannot be confirmed is recorded as
an open question rather than as a constraint.

**Review evidence.** The confirmation of current applicability, its date, and the person or role
that made it.

**Enforcement.** Unenforceable: No mechanism observes record citations in review comments or agent
reasoning

## RUST-DOC-0011-R011 — Retire an implemented proposal from operational authority

**Statement.** A proposal governs review and acceptance before implementation. After
implementation the accepted proposal MUST be treated as decision history, and MUST NOT be
maintained or cited as a current specification of behavior that canonical doctrine and executable
artifacts now carry.

**Intent.** Keep an accepted RFC from becoming a competing specification that future maintainers
must reconcile against current behavior.

**Applicability.** Accepted RFCs and equivalent proposal documents after their implementation has
landed. This rule does not weaken the RFC obligations stated in [`AGENTS.md`](../../AGENTS.md) and
[`rfcs/README.md`](../../rfcs/README.md), which govern the change process rather than the resulting contract.

**Allowed exceptions.** A proposal MAY remain cited for its decision, its date, its owners, its
accepted conditions, and its recorded alternatives, which are rationale rather than
specification.

**Review evidence.** The canonical doctrine and executable artifacts the proposal points to, and
the absence of a normative obligation stated only in the proposal.

**Enforcement.** Unenforceable: No check detects an accepted RFC being cited as current
specification

## RUST-DOC-0011-R012 — Record only rationale that cannot be recovered

**Statement.** Rationale MUST be recorded when it cannot be reconstructed safely from executable
artifacts and remains material to a future decision, and MUST NOT restate the operational
topology, interface, or invariant set as an independent contract.

**Intent.** Confine prose to what only prose carries: the constraint that shaped a design, the
alternative that was rejected and why the rejection still holds, and the risk somebody accepted.

**Applicability.** Rationale sections, design notes, decision records, and source-provenance
files.

**Allowed exceptions.** Rationale MAY quote or reference an enforced artifact for explanation
when the artifact is named as the authority at the point of reference.

**Review evidence.** The rationale text, the artifact it points to, and the statement of why the
recorded reason is not recoverable from that artifact.

**Enforcement.** Unenforceable: No check separates irrecoverable rationale from restatement of
enforced topology

## RUST-DOC-0011-R013 — Do not invent rationale for an existing constraint

**Statement.** Where the governing rationale for an enforced constraint is absent, a reviewer,
author, or agent MUST record it as unknown, and MUST NOT infer a reason from the implementation
and present that inference as the governing rationale.

**Intent.** An inferred reason presented as governing is a fabricated authority. It is
indistinguishable from a recorded one at the point of use, and it will be cited to block or
justify a change that the absent original reason may not have supported.

**Applicability.** Review records, documentation of existing systems, migration analyses, and
agent-generated summaries of code whose history is unavailable.

**Allowed exceptions.** An inference MAY be recorded when it is labelled as an inference, names
its evidence, and states that the governing rationale is unknown.

**Review evidence.** The unknown-rationale record, or the labelled inference with its evidence.

**Enforcement.** Unenforceable: No mechanism distinguishes a recorded reason from an inferred one

## RUST-DOC-0011-R014 — Keep an external claim outside the executable authority

**Statement.** A local executable guarantee MUST NOT be presented as evidence of a current
durable, remote, or externally governed fact, and each such fact MUST name the external system
that is authoritative for it.

**Intent.** State the partition's external leg. A type proves what its construction established
inside one process; committed state, remote acknowledgment, provider status, policy currency,
lock ownership, and settlement are facts other systems own.

**Applicability.** Claims about persisted state, remote effects, external identity, current
policy, distributed locks, fencing tokens, delivery, and settlement. RUST-DOC-0006 governs
ambiguity and reconciliation and RUST-DOC-0010-R014 governs durable advancement in a staged
protocol; this rule adds the obligation to name the authoritative external system for each claim.

**Allowed exceptions.** None. An external fact with no named authority is an unowned claim.

**Review evidence.** The claim, the named external authority, and the check that consults it.

**Enforcement.** Unenforceable: No check identifies external facts or verifies the named
authoritative system

## RUST-DOC-0011-R015 — Make a compatibility or migration promise executable

**Statement.** A compatibility promise, migration obligation, or negative guarantee SHOULD be
carried by a test, schema check, compile-fail fixture, or migration code, and where it is carried
by prose alone the artifact stating it MUST record that no mechanism enforces it.

**Intent.** Keep a promise from being read as a guarantee. A published compatibility statement
with no check behind it is a claim about intent, and a reader is entitled to know which it is.

**Applicability.** Published interfaces, wire formats, schemas, persisted representations, and
documented negative guarantees.

**Allowed exceptions.** A promise whose enforcement requires a system unavailable to the
repository stating it MAY remain prose-carried when the gap is recorded on the terms of
RUST-DOC-0011-R020.

**Review evidence.** The enforcing test, check, fixture, or migration, or the recorded statement
that the promise is unenforced.

**Enforcement.** Unenforceable: No check links a compatibility promise to a test, schema, or fixture

## RUST-DOC-0011-R016 — Keep the enforced structure readable as its domain story

**Statement.** An executable structure relied on as the authority for an architectural claim MUST
use domain names, MUST name its states for the facts they establish, MUST disclose its effects,
MUST keep capabilities narrow, and MUST delay type erasure, so that the enforced obligation is
legible without a parallel prose description.

**Intent.** An authority nobody can read produces the duplicate this doctrine exists to remove.
Legibility is not decoration here; it is the condition under which the executable artifact can
actually serve as the shared account of what the system does.

**Applicability.** Types, traits, schemas, manifests, and configuration relied on as the
authority for an architectural claim. RUST-DOC-0010-R002 governs stage naming within a staged
protocol; this rule generalizes the obligation to any artifact carrying architectural authority.

**Allowed exceptions.** An internal artifact with a documented, narrow audience MAY use local
abbreviations when they are defined at the artifact's entry point.

**Review evidence.** Names, state definitions, effect disclosure, capability scope, and the
location of any erasure boundary.

**Enforcement.** Unenforceable: No lint judges domain naming, effect disclosure, capability width,
or erasure timing

## RUST-DOC-0011-R017 — Count and reduce the maintained representations of a claim

**Statement.** A design review MUST identify every maintained representation of an architectural
claim, and MUST remove those that are neither authoritative, generated, mechanically checked, nor
required for irrecoverable rationale.

**Intent.** Make duplication a reviewable quantity rather than a matter of taste. The count is the
number of places a future change has to be made correctly, and it is the honest measure of the
cost of an architectural decision.

**Applicability.** Design reviews, doctrine changes, and any change that adds a description of an
existing obligation.

**Allowed exceptions.** A representation retained for a stated audience obligation MAY remain
when it is generated, mechanically checked, or marked informative and owned.

**Review evidence.** The representation inventory for the claim, and the disposition recorded for
each entry.

**Enforcement.** [`tools/doctrine-lint/src/main.rs`](../../tools/doctrine-lint/src/main.rs) —
check_validation_sequence_copies

## RUST-DOC-0011-R018 — Hydrate agents from current authority

**Statement.** Generated agent context MUST be built from current canonical and executable
authority, and MUST NOT include expired, superseded, or archived decision records by default.

**Intent.** Keep obsolete decisions out of the context an agent reasons from. An agent cannot
apply RUST-DOC-0011-R010 to a record it was handed as background rather than as a citation.

**Applicability.** Agent manifests, generated hydration packs, and any automated assembly of
context for planning, implementation, review, audit, or maintenance.

**Allowed exceptions.** An archived record MAY be included for a task whose scope is that record,
when the inclusion is explicit and the archival status travels with it.

**Review evidence.** The agent manifest, the generated pack contents, and the drift check.

**Enforcement.** [`tools/doctrine-lint/src/main.rs`](../../tools/doctrine-lint/src/main.rs) —
check_agent_packs_exclude_archive

## RUST-DOC-0011-R019 — Govern a change without duplicating what it changes

**Statement.** A normative change remains subject to the RFC, review, versioning, and migration
obligations of this corpus, and a governance artifact MUST NOT thereby become a second
operational specification of the contract it governs.

**Intent.** Preserve the change process without letting it accumulate a parallel description of
the system. Governance decides who may change a contract and on what evidence; it does not
restate the contract.

**Applicability.** RFCs, manifests, review records, waivers, and release notes.

**Allowed exceptions.** A governance artifact MAY state the contract as it stands at the moment
of decision, as the record of what was decided, when it is dated and is not maintained afterwards.

**Review evidence.** The governance artifact, the canonical contract it governs, and the absence
of a maintained restatement.

**Enforcement.** [`tools/doctrine-lint/src/main.rs`](../../tools/doctrine-lint/src/main.rs) —
check_normative_scope

## RUST-DOC-0011-R020 — Record the terms of a prose-only obligation

**Statement.** An exception that leaves an obligation carried by prose alone, or that keeps a
decision record in the active set, MUST name the owner, the consequence if the obligation is not
met, the compensating control, the reconsideration trigger, and the removal condition.

**Intent.** Give every unenforced obligation an end condition and somebody who owns it, so that
the exception is a decision with a lifetime rather than an omission with a description.

**Applicability.** Every exception claimed under RUST-DOC-0011-R002, RUST-DOC-0011-R005,
RUST-DOC-0011-R009, RUST-DOC-0011-R015, and RUST-DOC-0011-R017.

**Allowed exceptions.** None.

**Review evidence.** The recorded exception with all five terms, and the review that confirmed the
trigger has not fired.

**Enforcement.** Unenforceable: No schema or check requires the five exception terms anywhere

## Authority partition

Every claim this doctrine governs belongs to exactly one of five classes, and each class has one
kind of authority.

Executable and machine-checked artifacts are authoritative for the in-process operational truths
they enforce: legal ordering, available operations, successor relationships, construction
restrictions, permitted conversions and casts, schema constraints, canonical encodings,
visibility boundaries, runtime transition predicates, generated interface surfaces, and negative
guarantees demonstrated by rejection. For these claims, prose is informative under
RUST-DOC-0011-R003.

External and durable systems are authoritative for facts outside local execution: committed
state, remote acknowledgment, broker acceptance, provider identity status, current policy, the
current time, distributed lock ownership, fencing-token validity, delivery, and settlement.
RUST-DOC-0011-R014 keeps a local guarantee from standing in for any of them.

Rationale artifacts are authoritative only for what cannot be recovered from the artifacts: the
external constraint that shaped a design, a rejected alternative whose rejection remains
material, an irreversible commitment, a regulatory interpretation, a contractual obligation, an
accepted trade-off, and migration history that affects compatibility. RUST-DOC-0011-R012 keeps
them from restating the topology.

Non-guarantee and residual-risk statements are authoritative for what a design deliberately does not prove and who
accepted the remainder, on the terms [`foundations/guarantee-honesty.md`](../../foundations/guarantee-honesty.md)
states.

Governance artifacts are authoritative for who may change a normative contract, the required
review, waiver ownership, versioning policy, migration obligations, release gates, and legal or
regulatory approval. RUST-DOC-0011-R019 keeps them from becoming a second specification.

## Decision-record requirements

A decision record is created only for the residue identified by RUST-DOC-0011-R006, carries the
justification required by RUST-DOC-0011-R007, answers one question under RUST-DOC-0011-R008, and
ends under RUST-DOC-0011-R009. The active set is enumerated in a machine-readable registry so
that it can be audited, and so that RUST-DOC-0011-R018 can exclude what is no longer current.

A record is not a substitute for an RFC. An RFC proposes a change to a normative contract and is
governed by RUST-DOC-0011-R011 and by `rfcs/README.md`; a decision record captures a fact that
outlives the change and that no artifact carries.

## Guarantee and non-guarantee requirements

This doctrine states, for each claim it governs: the class the claim belongs to under
RUST-DOC-0011-R001; the artifact authoritative for it under RUST-DOC-0011-R003 or
RUST-DOC-0011-R014; the part of the claim no artifact enforces, stated separately under
RUST-DOC-0011-R003 and RUST-DOC-0011-R015; the maintained representations that remain and why,
under RUST-DOC-0011-R017; and the owner, trigger, and removal condition of every exception, under
RUST-DOC-0011-R020.

What this doctrine does not establish: that an obligation moved into a mechanism is thereby
correct; that a generated view is correct because it is current; that a record with a stated
justification has a good one; or that a system with no decision records has no unrecorded
constraints. Absence of a record is evidence about the record set, not about the constraints.

## Boundary requirements

Where an obligation crosses a boundary, the enforcing mechanism changes and the authority moves with it. A wire contract
is enforced by its canonical encoder, decoder, schema, and compatibility suite under
[`boundaries/serde.md`](../../boundaries/serde.md) and [`boundaries/http-and-rpc.md`](../../boundaries/http-and-rpc.md).
A persistence invariant is enforced by schema constraints, checked decoding, and transaction predicates under
RUST-DOC-0005 and [`boundaries/database-decoding.md`](../../boundaries/database-decoding.md). An operational policy is
enforced by deployable configuration and machine-checked manifests under
[`boundaries/configuration.md`](../../boundaries/configuration.md). A claim that crosses into another system's ownership
becomes an external claim governed by RUST-DOC-0011-R014.

## Waiver requirements

RUST-DOC-0011-R002, RUST-DOC-0011-R005, RUST-DOC-0011-R015, RUST-DOC-0011-R016, and
RUST-DOC-0011-R017 MAY be waived for an obligation whose enforcement or review cost is
disproportionate to its consequence. A waiver records the affected rule and claim, the owner
accepting the risk, the consequence, the compensating control, an expiry or reconsideration
trigger, and the removal condition, which are the same terms RUST-DOC-0011-R020 requires.

RUST-DOC-0011-R001, RUST-DOC-0011-R003, RUST-DOC-0011-R004, RUST-DOC-0011-R006,
RUST-DOC-0011-R007, RUST-DOC-0011-R008, RUST-DOC-0011-R009, RUST-DOC-0011-R010,
RUST-DOC-0011-R011, RUST-DOC-0011-R012, RUST-DOC-0011-R013, RUST-DOC-0011-R014,
RUST-DOC-0011-R018, RUST-DOC-0011-R019, and RUST-DOC-0011-R020 MUST NOT be waived. A waiver
cannot make an obsolete record current, cannot make an inferred rationale a governing one, cannot
make a local guarantee external evidence, and cannot authorize a second maintained source for a
claim an artifact already enforces.
