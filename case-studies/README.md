# Case studies

Case studies apply several doctrines to one coherent workflow. Each begins with
the problem and trust boundaries, shows a plausible compiling but weak design,
builds an improved representation and operation model, and finishes with facts
Rust cannot freeze or prove.

| Study | Central risks |
|---|---|
| [Invoice](invoice/) | contradictory state, money/email evidence, delivery ambiguity |
| [Payment lifecycle](payment-lifecycle/) | hybrid state, idempotency, capture uncertainty, compensation |
| [Database transaction](database-transaction/) | consumed handles, isolation, ambiguous commit |
| [Message delivery](message-delivery/) | at-least-once replay, inbox/outbox, ordering, poison data |
| [Authenticated session](authenticated-session/) | credential parsing, principal evidence, capability, revocation |
| [UI workflow](ui-workflow/) | local state, server authority, unknown submission, user-input preservation |

The code fragments emphasize representation and protocol. Executable mechanics
live under [`../examples/`](../examples/). Every improved design includes or
supports a guarantee ledger. A ledger row states exactly what establishes a
claim and which external or policy facts remain outside it.

Case studies are illustrative, not normative. Their rule references and linked
doctrine packages are normative. A real system must replace example policies,
capacities, provider contracts, and evidence with its own reviewed artifacts.
