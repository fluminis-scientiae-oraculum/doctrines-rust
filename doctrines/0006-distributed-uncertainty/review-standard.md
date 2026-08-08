# Review standard

Each gate receives **pass**, **fail**, **not applicable**, or an approved
**waiver reference**.

| Gate | Question                                          | Check    | Pass evidence                       | Failure example                         | Severity | Remediation                   |
| ---- | ------------------------------------------------- | -------- | ----------------------------------- | --------------------------------------- | -------- | ----------------------------- |
| D01  | Are all external effects inventoried?             | judgment | effect map                          | notification hidden in callback         | critical | enumerate side effects        |
| D02  | Is logical operation identity stable?             | judgment | persisted ID                        | ID generated per retry                  | critical | generate once                 |
| D03  | Are transport attempts separately identified?     | judgment | attempt log                         | retries indistinguishable               | medium   | add attempt identity          |
| D04  | Is request fingerprint retained safely?           | judgment | canonical fingerprint               | same key accepts changed amount         | critical | bind payload                  |
| D05  | Does timeout avoid definitive failure?            | judgment | unknown branch                      | timeout becomes declined                | critical | preserve uncertainty          |
| D06  | Is pre-dispatch failure actually proven?          | judgment | transport evidence                  | connection error guessed early          | high     | narrow classification         |
| D07  | Are confirmed rejections authenticated?           | judgment | protocol evidence                   | proxy text treated as provider decision | high     | validate source               |
| D08  | Are outcomes operationally distinct?              | judgment | outcome table                       | one generic error                       | critical | structure outcomes            |
| D09  | Does unknown carry reconciliation identity?       | judgment | durable token                       | only error string remains               | critical | persist evidence              |
| D10  | Is reconciliation owner named?                    | judgment | service/runbook owner               | unknown state abandoned                 | critical | assign custody                |
| D11  | Is reconciliation source authoritative?           | judgment | provider query/event contract       | local cache decides                     | critical | use authoritative observation |
| D12  | Is observation freshness stated?                  | judgment | timestamp/version                   | stale read called current               | high     | record and revalidate         |
| D13  | Can still-unknown remain explicit?                | judgment | state transition                    | absence converted to failure too early  | critical | retain state                  |
| D14  | Is escalation bounded by age/attempt?             | judgment | policy                              | retries continue silently forever       | high     | escalate visibly              |
| D15  | Is idempotency scope defined?                     | judgment | account/resource/endpoint rule      | key meaning global by assumption        | critical | define namespace              |
| D16  | Is key uniqueness defined?                        | judgment | generator and collision analysis    | timestamp-only key                      | high     | use robust identity           |
| D17  | Is payload bound to key?                          | judgment | conflict test                       | same key changes request                | critical | store fingerprint             |
| D18  | Are concurrent same-key calls handled?            | judgment | atomic claim                        | both execute before record              | critical | serialize/constraint          |
| D19  | Is response replay behavior defined?              | judgment | stored terminal response            | duplicate gets unrelated response       | high     | define replay                 |
| D20  | Is key retention sufficient?                      | judgment | horizon calculation                 | key pruned before broker replay         | critical | extend or constrain replay    |
| D21  | Is post-expiry behavior documented?               | judgment | contract                            | old key silently executes again         | high     | reject or identify new intent |
| D22  | Are naturally idempotent claims scoped?           | judgment | effect set                          | repeated email called idempotent        | critical | analyze all effects           |
| D23  | Does every retry reuse identity?                  | judgment | attempt trace                       | retry loop regenerates key              | critical | move ID outside loop          |
| D24  | Is retry classification per failure point?        | judgment | matrix                              | retry every I/O error                   | critical | classify                      |
| D25  | Is total attempt budget bounded?                  | judgment | equation                            | layers multiply without cap             | high     | coordinate                    |
| D26  | Is one deadline propagated?                       | judgment | remaining-time budget               | each layer restarts timeout             | high     | propagate deadline            |
| D27  | Are backoff and jitter appropriate?               | judgment | policy                              | synchronized fixed retry                | high     | desynchronize                 |
| D28  | Is downstream overload honored?                   | judgment | rate-limit handling                 | immediate repeated retry                | critical | wait/shed                     |
| D29  | Are duplicates expected by consumers?             | judgment | duplicate test                      | duplicate panics or repeats charge      | critical | deduplicate/idempotent effect |
| D30  | Is dedup state durable when needed?               | judgment | inbox/store                         | in-memory set                           | critical | persist                       |
| D31  | Is claim atomic with local effect?                | judgment | transaction proof                   | inbox records before effect             | critical | coordinate                    |
| D32  | Is acknowledgement order explicit?                | judgment | crash matrix                        | ack timing incidental                   | critical | define protocol               |
| D33  | Are crash points before/after effect tested?      | judgment | fault tests                         | only happy path                         | high     | inject crashes                |
| D34  | Is poison-message handling defined?               | judgment | quarantine/dead-letter              | endless hot loop                        | high     | isolate and escalate          |
| D35  | Is replay policy explicit?                        | judgment | operator procedure                  | replay duplicates unknown effects       | high     | preserve identities           |
| D36  | Is ordering scope named?                          | judgment | key/partition contract              | global FIFO claim                       | critical | narrow                        |
| D37  | Are gaps handled?                                 | judgment | version policy                      | missing predecessor ignored             | high     | wait/reconcile/reject         |
| D38  | Are out-of-order events tested?                   | judgment | sequence fixtures                   | assumed broker order                    | high     | add versions                  |
| D39  | Are exactly-once claims bounded?                  | judgment | guarantee ledger                    | broad slogan                            | critical | specify mechanism             |
| D40  | Are excluded external effects named?              | judgment | boundary diagram                    | DB transaction includes email claim     | critical | list exclusions               |
| D41  | Is compensation called a new effect?              | judgment | saga state model                    | refund called rollback                  | high     | model separately              |
| D42  | Is compensation failure handled?                  | judgment | outcome and reconcile path          | compensation assumed successful         | critical | retain uncertainty            |
| D43  | Is compensation idempotency analyzed?             | judgment | repeat test                         | duplicate reversal                      | critical | stable identity               |
| D44  | Are concurrent coordinators controlled?           | judgment | lease/CAS protocol                  | two reconcilers both act                | critical | claim and fence               |
| D45  | Do leases use fencing where needed?               | judgment | monotonic token at resource         | expired owner still accepted            | critical | add fencing                   |
| D46  | Is the time-authority contract complete?          | judgment | source, clock kind, bounds, failure | wall clocks assumed identical           | critical | define bounds and failure     |
| D47  | Is audit causality preserved?                     | judgment | parent/trigger IDs                  | attempts cannot be reconstructed        | high     | enrich audit schema           |
| D48  | Are audit secrets minimized?                      | judgment | field classification                | raw credential logged                   | critical | redact/minimize               |
| D49  | Are retry/reconcile queues bounded?               | judgment | capacity and age metrics            | backlog consumes memory                 | critical | persist and bound workers     |
| D50  | Are fault tests representative?                   | judgment | loss/delay/duplicate/reorder matrix | mock returns only error                 | high     | inject protocol failures      |
| D51  | Does guarantee ledger state residual uncertainty? | judgment | completed ledger                    | types imply remote permanence           | critical | narrow claims                 |
| D52  | Can users act safely while outcome is unknown?    | judgment | UI/API contract                     | retry button duplicates effect          | critical | gate action or reconcile      |

Critical failures block merge. Waivers need a named owner, affected operations,
accepted consequence, compensating control, monitoring, expiry, and resolution
condition.

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0006-R001`, `RUST-DOC-0006-R002`, `RUST-DOC-0006-R003`, `RUST-DOC-0006-R004`
- `RUST-DOC-0006-R005`, `RUST-DOC-0006-R006`, `RUST-DOC-0006-R007`, `RUST-DOC-0006-R008`
- `RUST-DOC-0006-R009`, `RUST-DOC-0006-R010`, `RUST-DOC-0006-R011`, `RUST-DOC-0006-R012`
- `RUST-DOC-0006-R013`, `RUST-DOC-0006-R014`, `RUST-DOC-0006-R015`, `RUST-DOC-0006-R016`
- `RUST-DOC-0006-R017`, `RUST-DOC-0006-R018`, `RUST-DOC-0006-R019`
