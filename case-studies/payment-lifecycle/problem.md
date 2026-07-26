# Payment lifecycle: problem

## Domain

A payment advances through business and provider states:

```text
draft
validated
authorized
captured
settled
reversed
failed
outcome unknown
```

Draft and validated are local preparation. Authorization and capture cross a
payment-provider boundary. Settlement may arrive asynchronously. Reversal is a
later effect; it does not erase capture history. Some failures are confirmed
provider rejections, while timeouts can make authorization or capture outcome
unknown.

## Invariants

| ID     | Statement                                                                                     | Enforcement                               |
| ------ | --------------------------------------------------------------------------------------------- | ----------------------------------------- |
| PAY-01 | Amount is nonzero minor units in one supported currency.                                      | private value constructor                 |
| PAY-02 | Validation precedes authorization.                                                            | local workflow transition                 |
| PAY-03 | Capture requires authorization evidence scoped to payment and amount.                         | capability/transition                     |
| PAY-04 | One logical authorization or capture reuses one operation identity.                           | durable operation record                  |
| PAY-05 | Persisted status has one legal variant with required evidence.                                | runtime enum and checked row conversion   |
| PAY-06 | Concurrent workers cannot silently capture the same payment as separate intents.              | optimistic claim and provider idempotency |
| PAY-07 | Capture timeout can become unknown and blocks blind new capture.                              | explicit persisted state                  |
| PAY-08 | Settlement and reversal are observations/effects after capture, not local compile-time facts. | runtime events and services               |
| PAY-09 | Compensation is a fallible reversal with its own identity.                                    | separate operation                        |
| PAY-10 | Audit preserves request, actor, attempt, provider, and resolution causality.                  | durable event trail                       |

## Required architecture

The design needs a hybrid state machine. Locally owned operation handles prevent
obvious sequence misuse during one worker execution. A runtime enum remains the
durable source for recovery, querying, heterogeneous work, external events, and
unknown outcomes. A repository issues a typed work handle only after current
status, version, and worker authority are checked.

HTTP/RPC input is a raw command. Authentication yields a principal;
authorization yields account/payment capabilities. Money construction does not
prove balance or fraud policy. Provider authorization does not prove capture or
settlement.

## Failure boundaries

Failure can occur:

- before provider dispatch;
- after dispatch but before response;
- after provider success before local persistence;
- after local status change before outbox publication;
- during duplicate retry;
- while settlement events are delayed or reordered;
- during reversal after capture.

The model must preserve what is known at each boundary. It must give users a
safe action while capture is unknown and give operators a reconciliation
identity rather than only a log string.
