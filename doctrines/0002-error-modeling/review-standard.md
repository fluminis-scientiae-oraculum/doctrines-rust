# Review standard

Record pass, fail, not applicable, or waiver for each gate.

| Gate          | Question                                                                            | Pass evidence                  | Failure example                         | Severity | Remediation                       |
| ------------- | ----------------------------------------------------------------------------------- | ------------------------------ | --------------------------------------- | -------- | --------------------------------- |
| Inventory     | Are causes, post-error state, actions, recipients, and commitment semantics listed? | failure matrix                 | error enum copied from dependency       | major    | model domain failures first       |
| Structure     | Can callers distinguish actionable outcomes without parsing text?                   | typed matching tests           | `Result<T, String>`                     | major    | introduce stable categories       |
| Source        | Is causal detail preserved safely?                                                  | source-chain test              | formatted source discarded              | major    | wrap with `source`                |
| Context       | Does context identify operation without erasing category?                           | structured context             | all errors become one sentence          | major    | attach context separately         |
| Validation    | Is invalid input distinct from internal failure?                                    | boundary tests                 | malformed request returns 500           | major    | map validation explicitly         |
| Authorization | Is denial preserved and redacted?                                                   | security mapping               | denial becomes not-found internally too | critical | retain protected audit category   |
| Cancellation  | Is cancellation distinct and commitment-aware?                                      | cancellation tests             | cancelled task assumed rolled back      | critical | define post-cancel state          |
| Timeout       | Can timeout mean unknown execution?                                                 | commitment analysis            | timeout maps to rejection               | critical | add unknown outcome               |
| Retry         | Is retry typed by semantics, idempotency, budget, and backoff?                      | retry table and fault tests    | retry every transport error             | critical | add decision policy               |
| Recovery      | Does each variant state whether values or handles remain usable?                    | API docs and tests             | consumed guard silently lost            | major    | return recovery evidence          |
| Panic         | Are expected external failures returned?                                            | panic inventory                | panic on user JSON                      | critical | make boundary fallible            |
| Unwrap        | Does each production unwrap follow a local invariant?                               | reviewed search                | unwrap database result                  | major    | propagate structured error        |
| Secrets       | Are display, debug, source, response, and logs recipient-safe?                      | redaction tests                | token in provider error                 | critical | sanitize and correlate            |
| Conversion    | Do `From` and mapping preserve security/reconciliation data?                        | conversion tests               | provider reference dropped              | critical | retain fields or protected record |
| Compatibility | Is public evolution deliberate?                                                     | semver/non-exhaustive analysis | downstream exhaustive match breaks      | major    | stabilize or document migration   |
| Logging       | Is final handling logged once with correlation?                                     | trace of one failure           | same error logged four times            | minor    | assign log owner                  |
| Codes         | Are public codes stable and documented?                                             | code catalogue tests           | message text used as code               | major    | introduce semantic code           |
| Evidence      | Do tests cover action distinctions, not only display strings?                       | variant and fault tests        | snapshot-only evidence                  | major    | test semantics                    |

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
