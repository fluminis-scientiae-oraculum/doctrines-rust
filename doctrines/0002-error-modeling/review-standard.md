# Review standard

Record pass, fail, not applicable, or waiver for each gate.

| Gate | Question                                                                                           | Check    | Pass evidence                  | Failure example                         | Severity | Remediation                       |
| ---- | -------------------------------------------------------------------------------------------------- | -------- | ------------------------------ | --------------------------------------- | -------- | --------------------------------- |
| F01  | **Inventory.** Are causes, post-error state, actions, recipients, and commitment semantics listed? | judgment | failure matrix                 | error enum copied from dependency       | major    | model domain failures first       |
| F02  | **Structure.** Can callers distinguish actionable outcomes without parsing text?                   | judgment | typed matching tests           | `Result<T, String>`                     | major    | introduce stable categories       |
| F03  | **Source.** Is causal detail preserved safely?                                                     | judgment | source-chain test              | formatted source discarded              | major    | wrap with `source`                |
| F04  | **Context.** Does context identify operation without erasing category?                             | judgment | structured context             | all errors become one sentence          | major    | attach context separately         |
| F05  | **Validation.** Is invalid input distinct from internal failure?                                   | judgment | boundary tests                 | malformed request returns 500           | major    | map validation explicitly         |
| F06  | **Authorization.** Is denial preserved and redacted?                                               | judgment | security mapping               | denial becomes not-found internally too | critical | retain protected audit category   |
| F07  | **Cancellation.** Is cancellation distinct and commitment-aware?                                   | judgment | cancellation tests             | cancelled task assumed rolled back      | critical | define post-cancel state          |
| F08  | **Timeout.** Can timeout mean unknown execution?                                                   | judgment | commitment analysis            | timeout maps to rejection               | critical | add unknown outcome               |
| F09  | **Retry.** Is retry typed by semantics, idempotency, budget, and backoff?                          | judgment | retry table and fault tests    | retry every transport error             | critical | add decision policy               |
| F10  | **Recovery.** Does each variant state whether values or handles remain usable?                     | judgment | API docs and tests             | consumed guard silently lost            | major    | return recovery evidence          |
| F11  | **Panic.** Are expected external failures returned?                                                | judgment | panic inventory                | panic on user JSON                      | critical | make boundary fallible            |
| F12  | **Unwrap.** Does each production unwrap follow a local invariant?                                  | judgment | reviewed search                | unwrap database result                  | major    | propagate structured error        |
| F13  | **Secrets.** Are display, debug, source, response, and logs recipient-safe?                        | judgment | redaction tests                | token in provider error                 | critical | sanitize and correlate            |
| F14  | **Conversion.** Do `From` and mapping preserve security/reconciliation data?                       | judgment | conversion tests               | provider reference dropped              | critical | retain fields or protected record |
| F15  | **Compatibility.** Is public evolution deliberate?                                                 | judgment | semver/non-exhaustive analysis | downstream exhaustive match breaks      | major    | stabilize or document migration   |
| F16  | **Logging.** Is final handling logged once with correlation?                                       | judgment | trace of one failure           | same error logged four times            | minor    | assign log owner                  |
| F17  | **Codes.** Are public codes stable and documented?                                                 | judgment | code catalogue tests           | message text used as code               | major    | introduce semantic code           |
| F18  | **Evidence.** Do tests cover action distinctions, not only display strings?                        | judgment | variant and fault tests        | snapshot-only evidence                  | major    | test semantics                    |

A critical failure blocks approval unless an explicit doctrine waiver permits it. Redacting a
public message does not justify erasing protected internal evidence.

## Normative rule traceability

The review record cites each applicable rule ID beside its gate result. Gate questions
operationalize the rules; they do not replace a rule's statement, applicability, or allowed
exceptions. Complete package coverage is:

- `RUST-DOC-0002-R001`, `RUST-DOC-0002-R002`, `RUST-DOC-0002-R003`, `RUST-DOC-0002-R004`
- `RUST-DOC-0002-R005`, `RUST-DOC-0002-R006`, `RUST-DOC-0002-R007`, `RUST-DOC-0002-R008`
- `RUST-DOC-0002-R009`, `RUST-DOC-0002-R010`, `RUST-DOC-0002-R011`, `RUST-DOC-0002-R012`
- `RUST-DOC-0002-R013`, `RUST-DOC-0002-R014`
