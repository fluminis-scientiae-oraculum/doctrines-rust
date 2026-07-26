# UI workflow: improved design

## Explicit local state

```rust
pub enum WorkflowState {
    Draft { form: FormDraft },
    Validated { form: ValidatedForm },
    Submitting {
        form: ValidatedForm,
        operation: OperationId,
    },
    Submitted {
        receipt: SubmissionReceipt,
    },
    Rejected {
        form: FormDraft,
        rejection: SubmissionRejection,
    },
    Unknown {
        form: FormDraft,
        reconciliation: SubmissionReconciliation,
    },
}
```

The enum prevents contradictory local combinations and retains input in
rejected and unknown states. Editing a validated form transitions back to draft
or revalidates, so stale validation is not represented. `Submitted` requires a
server receipt, not dispatch.

This client representation improves UI control flow. It is not a security
boundary and is not deserialized by the backend as domain evidence.

## Validation layers

Frontend validation checks immediate format, required fields, and bounded local
rules for responsive feedback. It produces `ValidatedForm` only inside the
client application. On the server:

1. body and decompression limits apply;
2. request DTO parses;
3. domain smart constructors validate values again;
4. session authenticates;
5. action/resource authorization executes under current policy;
6. entity version and cross-entity rules execute transactionally;
7. operation intent and result are persisted.

Server errors map to field-level validation, authorization-safe response,
version conflict, confirmed domain rejection, unavailable, or unknown outcome.
The client never sends a trusted principal or permit.

## Submission identity

When moving `Validated → Submitting`, the client creates one operation ID and
idempotency key, stores them with a canonical request fingerprint, and disables
ordinary repeat for that state. Re-render and button double-click reuse or
observe the same in-flight operation. The server binds key to principal,
endpoint, target, version, and payload for a defined retention period.

If failure is proven before dispatch, the client can retry the same operation.
If request may have executed and response is lost, state becomes `Unknown` with
operation ID and status endpoint. The form remains visible but a new submission
is gated until reconciliation or explicit new-intent confirmation.

## Server outcomes

| Server evidence | Client transition |
|---|---|
| structured validation rejection | `Rejected` with original draft and field mapping |
| authorization denial | safe rejection without leaking protected resource detail |
| optimistic version conflict | `Rejected` with current version/refetch action |
| confirmed command rejection | `Rejected` with domain-safe reason |
| confirmed completion | `Submitted` with receipt |
| accepted asynchronous processing | `Submitting` with operation status, not submitted |
| timeout after possible dispatch | `Unknown` |

The status endpoint authenticates and authorizes access to the operation. It
returns confirmed terminal, still processing, or still unknown. It does not
guess failure from elapsed time.

## Persistence and multiple tabs

A draft store keys data by application, tenant, principal/session scope, form
kind, and resource. It uses a version and expiry, minimizes sensitive fields,
and can encrypt according to threat model. Logout or principal change removes
or isolates drafts. Secret credentials never enter the draft.

Tabs coordinate through a browser-supported channel or storage version. The
server remains the final concurrency arbiter with entity version and idempotency
identity. A tab observing another operation can follow its status rather than
create a duplicate. When merge is appropriate, the UI shows field conflicts;
it does not silently last-write-wins.

## Routing and hosting

The deployment deliberately chooses hash routing for a static host:

```text
https://example.invalid/app/#/payments/new
```

The server receives `/app/` and returns the SPA entry file; the client router
interprets the fragment. Refresh and deep links work without a catch-all server
route. If history routing is selected later, hosting configuration adds a safe
fallback that does not mask real asset/API 404s.

No secrets or sensitive form values go in either path or fragment. Hash
fragments are not sent in HTTP requests, but scripts, browser history, screen
capture, extensions, and local device access can still observe them.

## Evidence and accessibility

State-machine tests cover every legal transition and ensure edit invalidates
validation. Integration tests use the real router and server adapter for
validation, authorization, version conflict, and idempotency. Controlled network
tests inject pre-dispatch failure, response loss after execution, delayed 202
completion, and status reconciliation. Browser tests cover double-click,
multiple tabs, reload, expiry, logout, hash deep-link refresh, and offline
recovery.

Accessibility tests verify focus reaches field errors, submitting and unknown
states use semantic status announcements, controls remain keyboard-operable,
and no outcome relies only on color. Snapshots support appearance but assertions
check state meaning.

## Guarantee ledger

| Claim | Established by | Protected construction | Boundary preservation | Escape hatches | Does not prove | Residual runtime risk |
|---|---|---|---|---|---|---|
| local workflow has one represented state | client enum | state update module | persisted draft decodes raw then validates | browser devtools can alter client | server domain validity/authority | client bug/tampering |
| validated form passed client policy | client constructor | private local type | server ignores claim and revalidates | direct JS mutation | server acceptance | policy drift |
| one submission intent has stable identity | ID created once entering submitting | state transition | server binds fingerprint | expiry/manual new intent | effect executed once everywhere | retention/provider behavior |
| submitted has confirmed server receipt | response/status evidence | no dispatch-only constructor | receipt decoded/authorized | mocked/dev build path | irreversible external finality | later reversal |
| unknown retains safe reconciliation | explicit state and operation ID | required payload | status endpoint | user authorizes new intent | success/failure | status outage |
| draft survives rejection | state carries `FormDraft` | rejection transition | scoped local storage | browser data clearing | durable server storage | quota/device loss |
| hash route deep-link works on configured static host | client route plus entry-file hosting | deployment config | browser integration test | host rewrite change | authorization/security | host/cache drift |
