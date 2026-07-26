# Anti-pattern catalogue

## Timeout equals failure

**Weak example.** A payment timeout becomes `PaymentFailed`, enabling a new
capture immediately.

**Why it fails.** The provider may have executed before the response was lost.

**Risk.** duplicate charge and false customer status.

**Improved direction.** transition to `CaptureUnknown` with provider key and
reconcile before new action.

**When justified.** A protocol-specific cancellation acknowledgement may prove
non-execution.

## Retry with a fresh key

**Weak example.** The retry helper creates an idempotency key on every attempt.

**Why it fails.** The receiver sees each attempt as new intent.

**Risk.** duplicate irreversible effects.

**Improved direction.** create and persist identity once before dispatch.

**When justified.** A fresh key is correct only for a genuinely new requested
effect.

## Key-shaped decoration

**Weak example.** An API accepts `Idempotency-Key` but stores nothing, ignores
payload conflicts, or expires keys immediately.

**Why it fails.** Syntax is present without replay semantics.

**Risk.** callers retry under a false guarantee.

**Improved direction.** define and test scope, binding, atomic claim, replay,
retention, and expiry.

**When justified.** None if the interface claims idempotency.

## Retry every transport error

**Weak example.** All I/O errors trigger exponential retry.

**Why it fails.** Some occur after execution and some represent persistent
authority or validation failure.

**Risk.** duplicate effects and amplified load.

**Improved direction.** classify by failure point and choose safe retry,
reconcile, or terminal handling.

**When justified.** Pure reads may broadly retry within a deadline when load and
staleness are controlled.

## In-memory deduplication

**Weak example.** A consumer remembers message IDs in a process-local set while
performing durable writes.

**Why it fails.** restart forgets identities and cannot coordinate the claim
with the effect.

**Risk.** duplicate durable mutation.

**Improved direction.** persist an inbox identity atomically with the local
effect.

**When justified.** Ephemeral best-effort work may accept duplication.

## Acknowledge then act

**Weak example.** A consumer acknowledges a command before performing a
required effect.

**Why it fails.** crash after acknowledgement loses the obligation.

**Risk.** silent missing work.

**Improved direction.** persist durable intent or complete and record the effect
before acknowledgement.

**When justified.** Lossy telemetry may explicitly prefer at-most-once
processing.

## Act then acknowledge without deduplication

**Weak example.** A consumer completes an effect, then acknowledges, with no
stable identity.

**Why it fails.** acknowledgement loss produces redelivery.

**Risk.** duplicate effect.

**Improved direction.** use idempotent effect identity or durable deduplication.

**When justified.** Repeated effect must be harmless under its complete
semantics.

## Exactly-once by branding

**Weak example.** A broker feature is cited as proof every downstream effect
happens once.

**Why it fails.** feature boundaries may cover broker state but not external
systems.

**Risk.** consumers omit duplicate protection.

**Improved direction.** state exact identity, transaction, resource, retention,
and excluded effects.

**When justified.** Boundary-scoped exactly-once terminology is acceptable with
the complete mechanism.

## Compensation as rollback

**Weak example.** A saga diagram shows an external refund arrow labeled
rollback.

**Why it fails.** refund is later, fallible, and not equivalent to no charge.

**Risk.** unresolved compensation disappears from state.

**Improved direction.** model compensation command, outcome, retry, and unknown
state independently.

**When justified.** The term rollback is appropriate inside one actual local
transaction.

## Distributed lock without fencing

**Weak example.** A lease expiry lets a new worker proceed, but the protected
resource accepts writes from the paused old worker.

**Why it fails.** ownership service and effect resource disagree about current
authority.

**Risk.** concurrent stale mutation.

**Improved direction.** attach monotonic fencing tokens that the resource
rejects when stale, or make the effect idempotent and versioned.

**When justified.** A lock may reduce duplicate low-risk work when overlap is
harmless.

## Not found means never happened

**Weak example.** The first reconciliation query returns no record and the
system declares rejection.

**Why it fails.** provider indexing or processing can lag.

**Risk.** unsafe retry.

**Improved direction.** follow provider-defined finality and retention windows;
remain unknown until evidence is definitive.

**When justified.** The provider contract may make an authoritative not-found
response final for the operation identity.

## Endless reconciler

**Weak example.** Unknown operations retry rapidly forever without age metrics
or escalation.

**Why it fails.** ownership exists only in code, not operations.

**Risk.** permanent load, cost, and invisible customer impact.

**Improved direction.** bound concurrency and attempts per cycle, back off,
measure age, and escalate terminally.

**When justified.** A durable obligation may remain pending, but execution and
visibility still need bounds.
