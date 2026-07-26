# UI workflow: problem

## Domain

A single-page application lets a user edit a form and submit a consequential
server command. Local workflow states are:

```text
draft
validated
submitting
submitted
rejected
outcome unknown
```

Validation improves feedback but the server remains authoritative for domain
invariants and authorization. Network timeout can occur after the server
executes. Retrying can duplicate the action. Navigation, refresh, multiple tabs,
and hash-routed deployment must preserve or deliberately discard local input.

## Invariants

| ID | Statement | Classification |
|---|---|---|
| UI-01 | User input remains available after local/server rejection unless policy requires removal. | lifecycle |
| UI-02 | Local validation never replaces server validation. | boundary |
| UI-03 | UI state never grants backend authorization. | authority |
| UI-04 | One submission intent has one stable operation/idempotency identity. | distributed |
| UI-05 | Double-click and concurrent tab submissions are coordinated. | concurrency/distributed |
| UI-06 | Timeout after dispatch becomes unknown, not automatic rejection. | distributed |
| UI-07 | Submitted means confirmed server evidence, not merely request dispatch. | evidence |
| UI-08 | Browser persistence and URLs do not expose secrets. | security |
| UI-09 | Route restoration works under the deployed hash/path routing contract. | environmental |
| UI-10 | Server conflicts preserve current version and safe next action. | persistence |

## Boundaries

Browser fields, local storage, URL fragments, history state, and client-side
types are untrusted at the server. The frontend parses and validates for
interaction. The HTTP/RPC adapter repeats bounded parsing and domain validation,
authenticates the session, authorizes the action/resource, and performs
transactional checks.

The browser receives server outcome evidence. A disconnected browser may not
receive a confirmed result. A status endpoint keyed by operation ID can
reconcile. Multiple tabs share or conflict over stored drafts and operation
identities.

## UX obligations

The interface should say what is known: local issues, submitting, server
rejection, confirmed acceptance, or confirmation pending. It should disable
unsafe blind repeats but provide status refresh, explicit new intent, or support
escalation. Accessibility must not depend solely on color or transient toasts.

Routing must match hosting. A hash-routed SPA keeps client path after `#`, so a
static server can return one entry file. A history-routed SPA needs server
fallbacks. Neither routing choice proves authentication or protects sensitive
state in URLs.
