# Decision framework

## Start with artifacts

Before selecting a primitive, produce:

1. an invariant inventory for shared and task-local state;
2. an ownership map naming every mutator;
3. a task tree with completion and shutdown owners;
4. an external-effect list with cancellation points;
5. a capacity table for queues, pools, permits, and downstream services;
6. a lock graph or a statement that no locks overlap;
7. a retry-and-timeout inventory;
8. an evidence plan covering adverse schedules and overload.

If these artifacts cannot be stated, implementation is premature.

## Choose the state model

| Need | Initial choice | Main checks |
|---|---|---|
| One sequential owner, many commands | actor or owner task | mailbox bound, response cancellation, supervision |
| Short shared reads and updates | lock-protected state | complete invariant, lock order, contention |
| Read-mostly immutable views | snapshot or copy-on-write | update cost, stale reads, retention |
| Fixed parallel pure work | bounded worker pool | input bound, result order, panic propagation |
| Narrow flag or counter protocol | atomic | happens-before argument, ordering, model evidence |
| External durable coordination | runtime protocol | lease, fencing, expiry, split brain, reconciliation |

Prefer one owner when shared mutation is complex. Prefer a lock when operations
are short and the protected invariant is clear. Prefer ordinary sequential code
when measured workload does not need overlap.

## Suspension-point decision

For each `.await`:

```text
Has local or external state changed before suspension?
├─ no → verify drop releases acquired resources
└─ yes
   Can drop leave the invariant valid and progress recoverable?
   ├─ yes → document recovery owner and resume identity
   └─ no
      Can the mutation move after the suspension?
      ├─ yes → reorder operation
      └─ no
         Can a guard/owner task finish it despite caller cancellation?
         ├─ yes → supervise bounded completion
         └─ no → add compensation or explicit reconciliation
```

A timeout around the future adds another cancellation edge. It does not prove a
remote request was not executed.

## Capacity decision

For each producer-consumer boundary, calculate:

- maximum admitted concurrent work;
- average and peak arrival rate assumptions;
- service-time distribution;
- per-item memory and resource cost;
- downstream capacity;
- wait deadline;
- overflow policy;
- fairness requirements;
- queue-age and rejection metrics.

Choose among:

| Policy | Use when | Cost |
|---|---|---|
| Wait | caller can absorb latency and pressure should propagate | deadline and head-of-line blocking |
| Reject | caller can retry or degrade safely | visible failure and retry coordination |
| Shed | work is explicitly disposable | information loss |
| Coalesce | newest or aggregate value is sufficient | intermediate history lost |
| Persist | work must survive process loss | storage, replay, deduplication |

An open-ended source with an unbounded in-memory queue fails the decision gate.

## Lock decision

Use a lock only after answering:

- Which invariant does it protect?
- Can code call unknown callbacks while holding it?
- Can it acquire another lock?
- Can it suspend or block?
- What is the acquisition order?
- What happens after panic?
- Can ownership transfer remove the shared mutation?
- Does measured contention require a different design?

Stop and redesign if the lock guards a broad component merely because ownership
was not decided.

## Task and shutdown decision

For every spawn, record:

| Field | Required content |
|---|---|
| Task name | stable operational identity |
| Owner | task or component that observes it |
| Completion | join, result channel, or supervised state |
| Failure | propagation, restart, degradation, or process stop |
| Cancellation | trigger and cleanup behavior |
| Capacity | maximum instances |
| Shutdown | admission stop, drain, deadline, abort |
| Observability | active count, failures, age, queue depth |

Detach only when loss is acceptable and the work remains bounded and observable.

## Retry composition

Compute the maximum attempt multiplication across all layers. Establish a
single end-to-end deadline and allocate sub-budgets. For each failure category,
choose:

- no retry;
- retry within remaining budget;
- reconcile before retry;
- terminal rejection;
- supervisor restart.

Use exponential backoff with jitter only where repeated attempts are safe.
Backoff is not a substitute for a hard attempt budget. If an operation may
already have taken effect, route to RUST-DOC-0006.

## Stop conditions

Choose a simpler or sequential design when:

- ownership is harder to explain than the workload benefit;
- the task tree has no clear failure owner;
- queue capacity has no defensible basis;
- lock ordering cannot be made acyclic;
- cancellation cannot preserve or reconcile partial work;
- an atomic ordering argument cannot be stated;
- benchmark evidence does not support the added parallelism;
- diagnostics and type complexity exceed the risk removed.

## Evidence selection

Use deterministic unit tests for closure and state transitions, controlled
timeouts for cancellation cleanup, stress tests for overload, and model checking
for small synchronization protocols. Measure queue depth, task count, lock wait,
latency distribution, rejection, restart, and shutdown duration in realistic
integration tests. State which schedules and failures remain untested.
