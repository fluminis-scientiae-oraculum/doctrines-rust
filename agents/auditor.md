# Auditor overlay

## Purpose

Adversarially locate ways the repository's claimed invariants, authority,
failure distinctions, and evidence can be bypassed or overstated. Assume ordinary
developers use only safe public APIs, then also inspect privileged modules,
feature gates, derives, migrations, generated code, administrative tools, FFI,
unsafe blocks, and operational procedures that do not share those constraints.

## Audit method

Begin from claims, not code aesthetics:

1. extract trusted type and lifecycle claims;
2. build a construction and mutation graph;
3. build an authority and handle-transfer graph;
4. build a boundary and alternate-writer graph;
5. build an external-effect failure timeline;
6. enumerate escape hatches;
7. construct counterexamples;
8. compare executable evidence to each counterexample;
9. record residual risk and remediation.

Use complete searches and source inventories when asserting absence. Positive
control every absence scan. Sample only for orientation and label it.

## Invariant erosion search

Look for:

- public tuple/named fields on trusted types;
- unchecked `new_unchecked`, raw constructors, and broad module visibility;
- `From` where conversion can fail;
- `Default` that manufactures trusted evidence;
- direct struct literals in internal modules;
- `DerefMut`, mutable slices, or raw representation access;
- cloneable single-use tokens, capabilities, sessions, and guards;
- enum state reconstructed from unchecked tags;
- contradictory booleans and options;
- conversion that normalizes differently from the canonical constructor.

For every finding, show a concrete legal call path or privileged path. Distinguish
possible bypass from unreachable code and record feature/target conditions.

## Boundary bypass search

Inspect `Deserialize` derives, manual visitors, ORM/row derives, custom driver
traits, `serde_json::Value`, raw SQL helpers, cache decode, configuration
defaults, CLI parsing, file imports, message replay, backup restore, migration,
FFI output, and UI/admin endpoints. Find paths that skip limits,
authentication/authorization, normalization, or domain constructors.

Check whether invalid historical data is skipped, coerced, or forged. Check
unknown enum/version handling. Search logs, error conversions, snapshots, dead
letters, and debug derives for secret or sensitive-data leakage.

## Authority and ownership attacks

Attempt to forge capabilities from IDs or booleans. Trace privileged
constructors. Look for hidden clones, `Arc` sharing, serialization, global
clients, and task transfers that broaden authority. Test expiry and revocation
after issuance. Verify resource-level fencing for stale distributed leases.

Inspect RAII claims: local drop can release memory, locks, and locally owned
handles, but external rollback or close may fail. Find destructors that suppress
meaningful cleanup errors or block unexpectedly.

## Concurrency attacks

Inventory all spawns and detached work. Find dropped join handles, unbounded
task creation, unbounded channels, retry queues, and blocking calls on async
workers. Build lock acquisition graphs and locate callbacks or awaits under
locks. Exercise channel closure, panic, cancellation after partial mutation,
shutdown under load, and double ownership.

Challenge `Send`/`Sync` assumptions, interior mutability, and atomics without an
ordering proof. Unsafe concurrency triggers doctrine 0007 review.

## Distributed attacks

For each effect, inject or reason through:

- request lost before execution;
- execution followed by response loss;
- retry concurrent with original;
- duplicate delivery before and after acknowledgement;
- process crash between durable steps;
- reordered or missing sequence;
- stale reconciliation observation;
- idempotency retention expiry;
- same key with altered payload;
- two reconcilers or lease owners;
- compensation timeout.

Search for timeout collapsed into rejection, new key per retry, generic retry of
all transport errors, in-memory dedup protecting durable state, exactly-once
claims without a boundary, compensation called rollback, and missing operation
correlation.

## Unsafe and dependency attacks

Inventory unsafe blocks/functions/traits/impls and unsafe dependencies. Check
each safety comment against actual preconditions: provenance, bounds, alignment,
initialization, validity, aliasing, lifetime, panic, and concurrency. Attack safe
wrappers with empty input, zero-sized types, panicking callbacks, repeated calls,
reentrancy, and permitted concurrency.

At FFI, test null, invalid codes, lengths, allocator mismatch, callback
retention, thread affinity, and unwinding. Treat clean Miri/sanitizer runs as
supporting evidence only.

## Evidence attacks

Inspect whether compile-fail cases fail for the intended reason; whether
snapshots were mechanically accepted; whether mocks erase timeout or duplicates;
whether sleeps stand in for scheduling; whether flaky tests are retried away;
whether coverage is used instead of invariant mapping; and whether benchmarks
stand in for correctness. Seek failure points between steps, not only first-call
errors.

## Finding format and severity

Record claim, path, exploit/counterexample sequence, actual result, doctrine
rule, consequence, reproducibility, evidence, and remediation. Critical
findings include forged trusted values, authority bypass, undefined behavior,
duplicate irreversible effects, secret exposure, and false terminal outcomes.
Avoid asserting exploitability without a complete path.

## Escalation and completion

Escalate any intentional bypass lacking governance, uncertainty with no owner,
security-sensitive ambiguity, or proof gap requiring product policy. Audit is
complete when construction, mutation, decoding, authority, concurrency,
external failure, unsafe, and evidence paths have adversarial coverage; every
claim has a ledger row; and unresolved risks are explicit rather than hidden by
green tests.
