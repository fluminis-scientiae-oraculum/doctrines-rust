# Anti-pattern catalogue

## `Arc<Mutex<T>>` as architecture

**Weak example.** Every service clones one `Arc<Mutex<State>>`, and each method
locks whichever fields it needs.

**Why it fails.** Shared access exists without a defined state owner, invariant
boundary, lock order, or contention model.

**Risk.** Deadlock, broad authority, torn logical transitions, and accidental
serialization.

**Improved direction.** Define ownership first. Use a dedicated owner task,
narrow capability, immutable snapshot, or a lock protecting a named invariant.

**When justified.** A small, short-lived component with one clear invariant and
measured low contention may use this representation.

## Locking across `.await`

**Weak example.** A task holds a synchronous mutex guard while awaiting a
network response.

**Why it fails.** The guard duration becomes externally controlled and may
block executor progress or create a resource cycle.

**Risk.** Deadlock, latency spikes, and throughput collapse.

**Improved direction.** Copy or move required state out, release the guard,
perform the request, then reacquire and validate. For mandatory serialization,
route work through an owner task.

**When justified.** An async-aware lock may span a deliberately serialized
operation only with a bounded external contract and a reviewed cancellation
path.

## Spawn per item

**Weak example.** A loop spawns one task for every element from an open-ended
stream.

**Why it fails.** Task count becomes the queue and has no admission policy.

**Risk.** Memory exhaustion, connection exhaustion, scheduler overhead, and
downstream overload.

**Improved direction.** Use bounded concurrency, a worker pool, or a stream
buffer with a reviewed limit and overload behavior.

**When justified.** The complete input is finite and statically proven below a
safe resource limit.

## Unbounded channel for convenience

**Weak example.** A producer uses an unbounded channel to avoid handling
capacity errors.

**Why it fails.** It turns overload into hidden memory consumption and latency.

**Risk.** Process termination and stale work processed after it has value.

**Improved direction.** Use a bounded channel and choose wait, reject, shed,
coalesce, or durable persistence.

**When justified.** A truly finite, tightly bounded event set may use one when
the bound is documented independently.

## Fire-and-forget side effect

**Weak example.** A request handler spawns an email or payment task, drops the
handle, and returns success.

**Why it fails.** Completion and failure have no owner; caller success describes
admission rather than effect.

**Risk.** Silent loss, duplicate retry, and dishonest API semantics.

**Improved direction.** Return an accepted operation identity, supervise the
task, persist durable intent, and expose confirmed or unknown outcomes.

**When justified.** A bounded best-effort telemetry action may detach when loss
is part of its documented contract.

## Timeout means cancellation succeeded

**Weak example.** Timing out a future is recorded as though no external effect
occurred.

**Why it fails.** Dropping local waiting does not revoke a transmitted request.

**Risk.** Duplicate payment, duplicate message, or false rejection.

**Improved direction.** Classify the operation as unknown when execution may
have occurred and preserve reconciliation identity.

**When justified.** Definitive failure is valid only when the protocol supplies
evidence that execution could not have occurred.

## Retry at every layer

**Weak example.** Client, proxy, service, database adapter, and supervisor each
retry independently.

**Why it fails.** Attempts multiply and common timing synchronizes load.

**Risk.** Retry storm, thundering herd, budget overrun, and duplicate effects.

**Improved direction.** Inventory all layers, allocate one deadline and attempt
budget, add jitter, and classify idempotency.

**When justified.** Nested retry scopes may address distinct failures when
their composed maximum is proven and observable.

## Closure as exceptional impossibility

**Weak example.** A receive loop unwraps channel reads because the sender
"always exists."

**Why it fails.** Shutdown, panic, and owner drop make closure an ordinary
lifecycle event.

**Risk.** panic loops, missing shutdown, or invisible owner failure.

**Improved direction.** Handle closure as stop, degrade, restart, or terminal
failure according to protocol.

**When justified.** A process-level invariant may deliberately terminate after
recording why closure proves internal corruption.

## Detached supervisor

**Weak example.** A background task restarts failed workers forever but its own
handle and failures are unobserved.

**Why it fails.** Moving failure into a supervisor does not establish custody of
the supervisor.

**Risk.** silent service loss or endless restart load.

**Improved direction.** Root the supervisor in the service task tree, bound
restarts, and expose terminal health.

**When justified.** Process-lifetime infrastructure can be top-level, but must
still be observed by the process owner.

## Atomics by folklore

**Weak example.** Code uses relaxed atomics because they are faster, or
sequential consistency because it feels safe.

**Why it fails.** Neither choice substitutes for a synchronization invariant;
stronger ordering can hide design confusion without establishing algorithmic
correctness.

**Risk.** stale reads, missing publication, rare protocol violation, or needless
cost.

**Improved direction.** State the happens-before relationship, model relevant
interleavings, or use a lock.

**When justified.** Independent telemetry counters can commonly use relaxed
ordering because they publish no other memory.

## Blocking pool as infinite escape

**Weak example.** Every slow or CPU-heavy action is moved to a blocking pool
with no admission limit.

**Why it fails.** Contention moves rather than disappears, and cancellation may
not stop running jobs.

**Risk.** thread growth, CPU saturation, and resource use after callers leave.

**Improved direction.** Bound admission, partition CPU-heavy work, account for
orphaned jobs, and monitor queue age.

**When justified.** Runtime-managed blocking facilities are appropriate when
their documented capacity and cancellation semantics satisfy the workload.

## Sleep-based concurrency test

**Weak example.** A test inserts sleeps and assumes a desired interleaving will
occur.

**Why it fails.** Timing varies by host and successful runs do not establish
the schedule was exercised.

**Risk.** flaky evidence or false confidence.

**Improved direction.** Use barriers, controlled clocks, injectable scheduling,
model checking, and explicit event observation.

**When justified.** Sleeps may bound an integration test deadline, but should
not be the synchronization mechanism that establishes its assertion.
