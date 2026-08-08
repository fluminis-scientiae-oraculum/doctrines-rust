# Review standard: <Replace with doctrine title>

Record every gate as **pass**, **fail**, **not applicable**, or **waiver
reference**.

Every gate needs a stable coded identifier: one to three uppercase letters
naming the package, then exactly two digits. Choose a prefix letter no active
package already uses. A gate that cannot be named cannot be cited from a
waiver, a review record, or a CI job.

The `Check` column says who decides the gate: `judgment`, or
`mechanical(...)` naming the command that settles it. Judgment is the ordinary
answer, and stating it is what makes the mechanical share countable.

| Gate | Question             | Check                  | Pass evidence       | Failure example           | Severity               | Remediation                    |
| ---- | -------------------- | ---------------------- | ------------------- | ------------------------- | ---------------------- | ------------------------------ |
| X01  | <Auditable question> | judgment               | <Concrete artifact> | <Specific counterexample> | <critical/high/medium> | <Direction, not vague request> |
| X02  | <Next question>      | mechanical(cargo test) | <Evidence>          | <Failure>                 | <Severity>             | <Remediation>                  |

## Required gate groups

Add domain-specific gates for:

- invariant discovery and classification;
- representation and protected construction;
- transition and authority;
- every decoding/persistence bypass;
- external failure and uncertainty;
- versioning and compatibility;
- sensitive-data handling;
- positive, negative, compile-fail, integration, fault, and operational evidence;
- guarantee and non-guarantee accuracy;
- complexity proportionality.

Provide at least forty substantive gates or an equally rigorous structured
standard. Do not game the count with paraphrases.

## Outcome

<Define which severities block merge, what a valid waiver contains, required
attachments, and how remediation is verified.>
