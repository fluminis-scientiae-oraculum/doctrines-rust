# Distributed-effects review

## Record

Apply to every network, broker, database-commit, payment, email, provisioning,
or other externally executed effect. Record **pass**, **fail**, **not
applicable**, or **waiver reference**.

## Effect and identity

| ID     | Question                                                   | Pass evidence         |
| ------ | ---------------------------------------------------------- | --------------------- |
| DER-01 | Is each external effect listed separately?                 | effect inventory      |
| DER-02 | Is one logical operation distinct from transport attempts? | identity model        |
| DER-03 | Is operation identity generated before first dispatch?     | lifecycle trace       |
| DER-04 | Do retries reuse the logical identity?                     | attempt tests         |
| DER-05 | Is the target/resource included in identity scope?         | key contract          |
| DER-06 | Is request intent fingerprinted canonically?               | fingerprint design    |
| DER-07 | Is identity collision risk proportionate?                  | generator analysis    |
| DER-08 | Is same identity with different payload rejected?          | conflict behavior     |
| DER-09 | Are concurrent same-identity attempts coordinated?         | atomic claim          |
| DER-10 | Is identity retained for the full replay horizon?          | retention calculation |

## Timeout, outcome, and retry

| ID     | Question                                                                         | Pass evidence         |
| ------ | -------------------------------------------------------------------------------- | --------------------- |
| DER-11 | Is the point after which execution may have occurred identified?                 | protocol timeline     |
| DER-12 | Does timeout avoid implying non-execution?                                       | outcome mapping       |
| DER-13 | Is local pre-dispatch failure supported by actual protocol evidence?             | transport contract    |
| DER-14 | Are confirmed success and confirmed rejection authenticated?                     | response verification |
| DER-15 | Are confirmed, rejected, local-failure, and unknown outcomes distinct as needed? | outcome type          |
| DER-16 | Does unknown carry reconciliation evidence?                                      | stored token          |
| DER-17 | Is retry classified at every failure point?                                      | decision matrix       |
| DER-18 | Are unsafe retries prohibited?                                                   | retry policy          |
| DER-19 | Does reconcile-before-retry exist for ambiguity?                                 | transition path       |
| DER-20 | Is one end-to-end deadline propagated?                                           | deadline budget       |
| DER-21 | Is maximum retry multiplication across layers calculated?                        | attempt equation      |
| DER-22 | Are backoff, jitter, and server guidance applied?                                | policy                |
| DER-23 | Are retry concurrency and queues bounded?                                        | capacity              |
| DER-24 | Are overload and rate-limit responses preserved?                                 | error/retry handling  |

## Delivery, order, and coordination

| ID     | Question                                                    | Pass evidence         |
| ------ | ----------------------------------------------------------- | --------------------- |
| DER-25 | Are duplicates expected for at-least-once delivery?         | consumer contract     |
| DER-26 | Is deduplication durable when protecting durable effects?   | inbox/store           |
| DER-27 | Is dedup claim atomic with the local effect?                | transaction           |
| DER-28 | Is dedup retention sufficient and expiry behavior explicit? | retention/replay plan |
| DER-29 | Is acknowledgement order documented?                        | crash-point matrix    |
| DER-30 | Is acknowledgement loss handled?                            | redelivery test       |
| DER-31 | Are poison messages isolated without a hot retry loop?      | dead-letter policy    |
| DER-32 | Is administrative replay identity-preserving and audited?   | replay runbook        |
| DER-33 | Is ordering scoped to key/partition/producer/consumer?      | ordering contract     |
| DER-34 | Are gaps and out-of-order versions handled?                 | state/version policy  |
| DER-35 | Are failover and retry effects on order stated?             | scenario tests        |
| DER-36 | Is every exactly-once claim boundary-specific?              | guarantee ledger      |
| DER-37 | Are external effects outside the claimed transaction named? | boundary diagram      |
| DER-38 | Is persistence plus publication coordinated durably?        | outbox/event log      |

## Reconciliation, compensation, and authority

| ID     | Question                                                  | Pass evidence             |
| ------ | --------------------------------------------------------- | ------------------------- |
| DER-39 | Is every unknown state durable when process loss matters? | persistence model         |
| DER-40 | Is a reconciliation owner named?                          | service/runbook ownership |
| DER-41 | Is the observation source authoritative?                  | provider contract         |
| DER-42 | Are observation freshness and finality defined?           | timestamp/version/window  |
| DER-43 | Can reconciliation remain unknown?                        | repeated state path       |
| DER-44 | Are reconciliation attempts bounded and observable?       | age/attempt metrics       |
| DER-45 | Is terminal human escalation defined?                     | operations procedure      |
| DER-46 | Are operator overrides audited as decisions, not proof?   | audit event               |
| DER-47 | Is compensation modeled as a new effect?                  | saga states               |
| DER-48 | Does compensation have idempotency and unknown handling?  | effect contract           |
| DER-49 | Are concurrent coordinators claimed atomically?           | lease/CAS                 |
| DER-50 | Are stale lease owners fenced at the effect resource?     | fencing token             |
| DER-51 | Are clock and process-pause assumptions documented?       | lease analysis            |
| DER-52 | Can users safely act while state is unknown?              | API/UI behavior           |

## Audit, secrecy, and evidence

| ID     | Question                                                             | Pass evidence        |
| ------ | -------------------------------------------------------------------- | -------------------- |
| DER-53 | Does audit preserve operation, attempt, parent, trigger, and target? | event schema         |
| DER-54 | Are outcome observations and decisions reconstructible?              | incident query       |
| DER-55 | Are credentials and unnecessary personal data excluded?              | field classification |
| DER-56 | Is correlation retained without uncontrolled tracking?               | privacy policy       |
| DER-57 | Do tests inject loss before and after dispatch?                      | fault suite          |
| DER-58 | Do tests inject duplicate, delay, reordering, and crash?             | scenario matrix      |
| DER-59 | Do tests cover concurrent identity and reconciler claims?            | concurrency suite    |
| DER-60 | Does the ledger state residual unknowns and non-guarantees?          | completed ledger     |

## Exit criteria

Approval requires stable identity, exact outcome semantics, bounded safe retry,
durable reconciliation, duplicate/order handling, honest transaction scope,
auditable compensation, sensitive-data minimization, and failure-point evidence.
