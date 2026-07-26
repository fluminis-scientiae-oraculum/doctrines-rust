# Review standard

Record each gate as **pass**, **fail**, **not applicable**, or an approved
**waiver reference**. A critical failure blocks merge.

| Gate | Question                                                | Pass evidence                        | Failure example                                  | Severity | Remediation                            |
| ---- | ------------------------------------------------------- | ------------------------------------ | ------------------------------------------------ | -------- | -------------------------------------- |
| C01  | Is mutable-state ownership explicit?                    | ownership map                        | several tasks mutate shared state by convention  | critical | assign owner and synchronization       |
| C02  | Does each synchronization primitive name its invariant? | invariant-to-primitive mapping       | mutex exists only because sharing was convenient | high     | define protected relationship          |
| C03  | Are related fields updated under one protocol?          | atomic grouped transition            | status and handle use separate locks             | critical | combine state or coordinate transition |
| C04  | Is aliasing authority minimal?                          | narrow borrowed or message interface | broad mutable handle escapes                     | high     | restrict interface                     |
| C05  | Is every task represented in a task tree?               | spawn inventory                      | untracked spawn                                  | high     | add owner and completion path          |
| C06  | Are task failures observed?                             | join or supervisor result handling   | join handle dropped                              | critical | propagate or supervise failure         |
| C07  | Are panics handled according to policy?                 | panic branch and telemetry           | worker silently disappears                       | high     | define fail, restart, or degrade       |
| C08  | Are restart attempts bounded?                           | restart budget                       | permanent failure loops forever                  | high     | cap and expose terminal state          |
| C09  | Does restart use backoff and jitter where needed?       | policy and tests                     | all workers restart simultaneously               | high     | stagger bounded retries                |
| C10  | Is concurrency bounded?                                 | semaphore, pool, or finite proof     | spawn per unbounded input                        | critical | impose reviewed limit                  |
| C11  | Is queue capacity justified?                            | resource calculation                 | arbitrary enormous buffer                        | high     | derive from budget                     |
| C12  | Is overload behavior explicit?                          | wait/reject/shed/persist contract    | memory grows until failure                       | critical | add backpressure                       |
| C13  | Is overload visible?                                    | queue age/depth/rejection metrics    | saturation is silent                             | medium   | instrument capacity signals            |
| C14  | Is fairness considered?                                 | scheduling policy                    | one tenant monopolizes permits                   | medium   | partition or schedule fairly           |
| C15  | Is each channel closure branch handled?                 | closure tests                        | receive loop spins after closure                 | high     | terminate or transition                |
| C16  | Does receiver loss reach senders?                       | send error policy                    | sends reported successful after owner exit       | critical | preserve closure failure               |
| C17  | Is drain behavior defined?                              | shutdown contract                    | queue is silently discarded                      | high     | drain, persist, or document loss       |
| C18  | Is lock scope bounded?                                  | visible narrow guard lifetime        | guard spans whole request                        | high     | extract and release                    |
| C19  | Is `.await` absent under synchronous locks?             | code inspection                      | network call while mutex held                    | critical | split phase or use owner task          |
| C20  | Are blocking calls classified?                          | blocking-work inventory              | synchronous filesystem call on executor          | high     | isolate deliberately                   |
| C21  | Is blocking-pool capacity bounded?                      | pool configuration                   | isolation pool grows without limit               | high     | add capacity and admission             |
| C22  | Can cancelled blocking work continue?                   | explicit accounting                  | request cancellation assumed to stop syscall     | high     | supervise remaining work               |
| C23  | Is a lock acquisition order documented?                 | acyclic lock graph                   | paths acquire A/B and B/A                        | critical | establish order or redesign            |
| C24  | Are callbacks excluded from lock scope?                 | call graph evidence                  | arbitrary callback under lock                    | high     | release before callback                |
| C25  | Is poisoning policy explicit?                           | recovery or fail-stop rationale      | poisoned lock blindly unwrapped                  | high     | validate or terminate                  |
| C26  | Is every suspension point cancellation-reviewed?        | cancellation table                   | timeout drops half-finished operation            | critical | reorder, guard, or reconcile           |
| C27  | Does cancellation release local resources?              | RAII or explicit cleanup test        | permit leak                                      | high     | add owned guard                        |
| C28  | Are external effects before cancellation recorded?      | outcome state                        | timeout becomes definitive rejection             | critical | represent uncertainty                  |
| C29  | Can partial progress resume safely?                     | cursor/idempotency evidence          | restart repeats unsafe effect                    | critical | persist progress or reconcile          |
| C30  | Are timeout budgets end-to-end?                         | deadline allocation                  | nested layers each use full timeout              | high     | propagate remaining budget             |
| C31  | Is retry multiplication calculated?                     | attempt equation                     | client, proxy, worker each retry blindly         | critical | centralize budgets                     |
| C32  | Are retries idempotency-classified?                     | operation table                      | mutation retried after lost response             | critical | reconcile or use idempotency           |
| C33  | Are retry waves desynchronized?                         | jitter policy                        | thundering herd at fixed intervals               | high     | add jitter and admission               |
| C34  | Is ordering scope precise?                              | key/producer/partition contract      | FIFO called global order                         | high     | narrow claim                           |
| C35  | Are retry and failover effects on order stated?         | scenario tests                       | redelivery reorders unnoticed                    | high     | version or tolerate reorder            |
| C36  | Does shutdown stop admission first?                     | ordered procedure                    | new work enters while draining                   | high     | close ingress                          |
| C37  | Does shutdown have a deadline?                          | time budget                          | waits forever                                    | high     | define forced termination              |
| C38  | Are in-flight effects classified at shutdown?           | work accounting                      | process exit loses ambiguous request             | critical | persist or reconcile                   |
| C39  | Are resources released on all exits?                    | tests for normal/error/cancel        | permits survive task failure                     | high     | use guards and cleanup                 |
| C40  | Are detached tasks exceptional and named?               | approved inventory                   | anonymous fire-and-forget                        | high     | supervise or document exception        |
| C41  | Are detached resources bounded?                         | count and queue limits               | cleanup task accumulation                        | critical | bound and shed                         |
| C42  | Are detached failures observable?                       | metrics and logs                     | background failure invisible                     | high     | report health                          |
| C43  | Does each atomic name its invariant?                    | code comment and design note         | atomic chosen for speed                          | critical | define protocol                        |
| C44  | Is memory ordering justified?                           | happens-before argument              | copied `Relaxed` ordering                        | critical | prove or use lock                      |
| C45  | Is unsafe concurrency separately reviewed?              | RUST-DOC-0007 evidence               | manual `Send` with no proof                      | critical | perform unsafe audit                   |
| C46  | Are small protocols model-tested where valuable?        | Loom or equivalent result            | rare schedule only stress-tested                 | medium   | add controlled schedule tests          |
| C47  | Are async trait costs understood?                       | dispatch/allocation analysis         | boxed future on hot path by accident             | medium   | simplify or measure                    |
| C48  | Are latency claims distributions, not anecdotes?        | p50/p95/p99 under load               | one local timing                                 | medium   | benchmark correctly                    |
| C49  | Does local state avoid remote-liveness claims?          | guarantee ledger                     | connected state promises next send               | critical | narrow guarantee                       |
| C50  | Are evidence limits documented?                         | residual-risk section                | tests presented as universal proof               | high     | state untested schedules               |

## Review outcome

Approval requires all critical gates to pass or carry a time-bounded waiver with
owner, scope, compensating evidence, and removal condition. High-severity
failures require remediation or explicit risk acceptance. Medium findings may
be scheduled only when they cannot mask correctness or overload failures.

The reviewer MUST attach or reference the ownership map, task tree, capacity
table, cancellation table, retry inventory, shutdown procedure, and guarantee
ledger. Code style alone is not sufficient evidence.
