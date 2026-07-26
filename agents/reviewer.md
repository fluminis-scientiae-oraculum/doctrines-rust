# Reviewer overlay

## Purpose

Decide whether implementation establishes its stated guarantees at acceptable
complexity and with proportionate evidence. Review exact construction,
transition, decoding, authority, and recovery paths. Do not approve merely
because code is idiomatic, compiles, avoids `unwrap`, or has tests.

## Review preparation

Read the change completely, then the invariant inventory, boundary map, state
graph, effect/uncertainty inventory, complexity decision, evidence plan, and
guarantee ledger. Identify applicable doctrine rule IDs. Compare code to those
artifacts; if artifacts are absent, request them rather than reconstructing
critical product policy through assumption.

Inspect the complete diff and all affected files. Generated output is checked
against canonical sources but not reviewed as an independent doctrine source.

## Construction-path review

For each trusted type:

1. state exactly what the name claims;
2. enumerate every constructor, conversion, derive, decoder, helper, test
   feature, privileged path, and unsafe path;
3. verify private representation;
4. verify every ordinary path enforces the complete invariant;
5. compare normalization order and errors;
6. inspect mutation and mutable-borrow escapes;
7. examine formatting, cloning, and serialization;
8. list what the type does not prove.

Search Serde, database rows, caches, migrations, imports, replay, configuration,
and FFI. A correct primary constructor does not compensate for a weaker
secondary path.

## State and authority review

Challenge contradictory flags/options and require a sum type where states are
exclusive. Check associated data, stable tags, unknown variants, legal edges,
failure edges, and history. Determine whether typestate is locally controlled,
small, and useful; compare a runtime enum. Verify consuming transitions do not
return old authority after ambiguous execution.

For capabilities, inspect privileged construction, least-privilege methods,
resource scope, clone, transfer, expiry, revocation, and leakage. Confirm
authentication is not treated as all-purpose authorization. Local possession
must not be claimed as current external policy when revocation can occur.

## Boundary and persistence review

Trace raw representation to trusted construction. Confirm size and allocation
limits precede expensive decode. Evaluate unknown-field/version policy,
diagnostic redaction, and sensitive-data retention. In database code, inspect
raw row conversion, truth tables, constraints, transaction/isolation
mechanisms, version conflicts, invalid-history quarantine, and migrations.

Challenge claims that a successful database call includes messages or external
effects. When an outbox is used, check only the atomic durable-intent claim and
require duplicate-safe publication/consumption.

## Concurrency and external-effect review

Inspect state ownership, lock boundaries/order, blocking isolation,
cancellation at every partial `.await`, bounded queues, backpressure, channel
closure, task supervision, and shutdown. For atomics, require a
happens-before argument rather than ordering folklore.

For each external effect, locate the point of possible execution, stable
operation ID, idempotency contract, retry layers, timeout result, duplicate
behavior, ordering scope, reconciliation, compensation, and audit trail.
Reject any mapping from ambiguity to confirmed failure. Verify users and
operators can act safely while status is unknown.

## Complexity review

Ask what consequential invalid program is prevented by each new type, trait,
generic state, layer, dependency, lock, allocation, or macro. Evaluate compiler
diagnostics, public signatures, serialization, persistence, dynamic dispatch,
compile time, binary size, migration, and team maintenance. Prefer simpler code
when the additional mechanism removes little risk.

Do not demand removal of every clone or allocation without measurement. Do not
accept unsafe optimization without a material measured benefit and complete
safety proof.

## Evidence review

Map tests to invariants. Look for boundary rejection, illegal transition,
compile-fail prohibition, real integration, cancellation, partial failure,
duplicate, reordering, unknown outcome, and reconciliation. Inspect trybuild
diagnostics for intended failure. Challenge snapshots and flaky retries. Check
that test doubles preserve the failure behavior supporting the claim.

Measurements require workload, environment, baseline, profile, distributions,
and independent correctness evidence.

## Finding format

Each finding states:

- severity;
- source path and exact construction/transition/effect;
- doctrine rule or invariant ID;
- claimed behavior;
- actual evidence;
- counterexample or failure sequence;
- consequence;
- remediation direction;
- required proof to close.

Avoid style findings that do not affect correctness, clarity, compatibility, or
maintainability. Mark uncertain observations as questions with the evidence
needed.

## Forbidden approvals

Never approve based only on "Rust makes this safe," private fields, green CI,
high coverage, familiar pattern, framework convention, or provider marketing.
Do not treat typestate as superior by default. Do not infer external liveness,
exactly-once effects, current authorization, or universal performance from
local types.

## Escalation and completion

Escalate normative ambiguity, product-policy gaps, security-sensitive
authority, incompatible migrations, unresolved unknown outcomes, unsafe proof
disputes, or accepted high-consequence risk. Recommend a scoped decision.

Approval requires applicable domain, boundary, typestate, distributed-effect,
and final audit gates; an honest guarantee ledger; and evidence whose limits
match claims. State residual limitations in the review.
