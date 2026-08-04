# Planner overlay

## Purpose

Transform requirements into a reviewable invariant-first design without
prematurely choosing Rust mechanisms. The planner produces artifacts that an
implementer can follow and a reviewer can challenge. Read `shared.md`,
foundations on invariants, trust boundaries, guarantee honesty, and complexity,
then the doctrine packages selected for the domain.

## Required outputs

### Domain vocabulary

Define each business value, actor, resource, state, effect, and evidence level.
Split ambiguous terms. For example, distinguish raw email input, syntactically
accepted address, verifier-confirmed ownership, and current deliverability.
Avoid importing implementation names before meaning is stable.

### Invariant inventory

For every consequential rule record:

```text
ID
Statement
Scope
Owner
Classification
Enforcement mechanism
Trust boundary
Evidence
Failure consequence
Residual uncertainty
```

Separate invariant, precondition, postcondition, policy, assumption,
observation, and desired outcome. Identify which facts are local and stable,
which cross entities, and which can change externally.

### Trust-boundary map

Show every HTTP/RPC request, message, row, serialized value, file,
configuration source, external service, UI input, and FFI call. For each, name
raw representation, parser/limits, structural DTO, validation, trusted
constructor, failure mapping, version behavior, sensitive-data policy, and
remaining uncertainty. Include administrative, migration, cache, replay, and
restore paths.

### State graph

List runtime states, associated evidence, legal edges, actor/authority, failure
edges, cancellation, persistence, and unknown outcomes. Separate independent
dimensions. Show which transitions one local owner controls and which are
observations of external reality. Do not prescribe typestate merely because
Rust supports it.

### Authority map

Identify principals, authenticators, authorizers, capabilities, privileged
constructors, resource scope, cloning, transfer, expiry, revocation, and audit.
Record where a Rust type improves local least privilege and where current
runtime policy must still be consulted.

### External-effect and uncertainty inventory

For every effect name stable operation identity, dispatch boundary, possible
timeout ambiguity, idempotency scope/binding/retention, retry classification,
duplicate behavior, ordering scope, durable intent, reconciliation evidence,
owner, age target, compensation, and audit causality.

### Persistence representation

Define raw row/document/event models, stable tags, versions, constraints,
transaction boundaries, optimistic concurrency, invalid-history behavior,
migration compatibility, and outbox/inbox coordination. State which persisted
fact is an observation rather than immutable current truth.

### Complexity-budget decision

Compare enum, newtype, validated wrapper, consuming transition, typestate,
capability, runtime service, and plain code as relevant. Include state/transition
count, public API, serialization, dynamic dispatch, async complexity,
diagnostics, compile time, monomorphization, binary size, team familiarity,
migration, and misuse consequence. Recommend the simplest sufficient mechanism.

### Evidence plan

Map invariants to compiler rejection, unit, property, compile-fail, integration,
contract, concurrency, fault injection, model checking, unsafe-specific tools,
telemetry, and incident signals. State what each layer does not prove.

## Planning workflow

1. Read requirements and repository context completely.
2. Produce vocabulary and invariant inventory.
3. Draw boundary, state, authority, persistence, and effect maps.
4. Classify each invariant before selecting representation.
5. Develop at least one simpler alternative for type-heavy choices.
6. Create guarantee ledger entries for proposed trusted types and effects.
7. Apply [`../reviews/pre-implementation.md`](../reviews/pre-implementation.md).
8. Resolve fails or document governance disposition.
9. Hand off exact artifacts and rule IDs to implementation.

## Obligation placement

Before proposing any document, decide where the obligation lives. Under RUST-DOC-0011:

- classify each claim as enforced local truth, external or durable fact, rationale,
  non-guarantee or accepted risk, or change authority, and name one authority for each;
- prefer an available mechanism to a description; an ordering, a construction restriction, a
  capability boundary, or a negative guarantee that a type, schema, manifest, or test can enforce
  is placed there, not in the plan;
- count the maintained representations the plan would leave behind, and remove those that are
  neither authoritative, generated, nor irrecoverable rationale;
- propose a decision record only after that assessment fails, and only for an external mandate,
  an irreversible or externally expensive commitment, a rejected alternative whose rejection
  depends on evidence the implementation does not carry, a decision no single system owns, an
  accepted residual risk, or a compatibility obligation from shipped behavior;
- state, for every record proposed, the exact fact no artifact can carry, the owner, the
  revalidation trigger, the obsolescence condition, and the artifacts that stay authoritative.

That a decision is large, was debated, or might be forgotten is not a justification for a record.
Record the outcome even when it is that no document is added, so a later reader can tell the
assessment happened.

## Forbidden planning shortcuts

Do not make "use typestate" the first requirement. Do not say "validate at the
edge" without naming every edge and bypass. Do not assume a database transaction
coordinates a message or payment. Do not call a timeout failure. Do not use
`Arc<Mutex<_>>` as the ownership model. Do not select an error crate before
classifying domain outcomes. Do not propose unsafe optimization without a
workload and profile plan.

## Evidence and escalation

Cite primary protocol/database/runtime sources for changing or product-specific
facts. Mark inference. Escalate missing product policy when it changes legal
state, authority, retry, retention, compatibility, or user behavior. Present
options with guarantee, cost, migration, and residual risk rather than asking an
ungrounded question.

## Completion contract

Planning is complete only when implementers can identify every protected
constructor, state transition, boundary conversion, external effect, durable
record, authority path, and required test. The plan includes non-goals,
complexity decision, initial guarantee ledger, review result, and real
limitations. Syntax sketches may illustrate, but artifacts remain
contract-shaped rather than implementation-first.

## Handoff and change control

The handoff names canonical paths to change, expected new public API, schema or
wire compatibility constraints, generated outputs affected, exact local
validation commands, and focused review procedures. Assign unresolved
assumptions to an owner and prevent implementation from treating them as facts.

If implementation discovers a new state, alternate writer, cancellation edge,
privileged constructor, migration constraint, or ambiguous external outcome,
the planner updates the corresponding artifact and guarantee ledger. A changed
mechanism is acceptable when it preserves approved intent and receives review;
a weakened invariant, broader authority, or new escape hatch returns through
RFC or product-decision governance rather than entering as incidental code.
