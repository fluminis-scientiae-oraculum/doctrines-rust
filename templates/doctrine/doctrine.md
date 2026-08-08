# Normative doctrine: <Replace with title>

Use uppercase normative terms according to
[`foundations/normative-language.md`](../../foundations/normative-language.md). Assign stable IDs sequentially; never reuse
an old ID for new meaning.

## RUST-DOC-NNNN-R001 — <Replace with concise rule title>

**Statement.** <Write one auditable MUST, MUST NOT, SHOULD, SHOULD NOT, or MAY
statement. Name the subject, action, and condition.>

**Intent.** <Explain the consequential failure prevented without adding an
unidentified requirement.>

**Applicability.** <Name domains, boundaries, and conditions under which the
rule applies.>

**Allowed exceptions.** <Define a narrow exception and required evidence, or
state that no exception is allowed. Do not use convenience as justification.>

**Review evidence.** <Name source paths, artifacts, tests, protocol evidence,
measurements, or guarantee-ledger fields a reviewer expects.>

**Enforcement.** <Name the artifact that enforces this rule as a backticked
repository path inside a Markdown link, followed by what it shows. If no
artifact can carry the rule, open with Unenforceable: and give the specific
reason. A waiver is the ordinary outcome for a design obligation, not a gap.>

## RUST-DOC-NNNN-R002 — <Replace with next rule title>

**Statement.** <Write the next normative statement.>

**Intent.** <State the risk reduction.>

**Applicability.** <State scope.>

**Allowed exceptions.** <State exact exception policy.>

**Review evidence.** <State required evidence.>

**Enforcement.** <Link the enforcing artifact, or state why none can carry it.>

## Guarantee and non-guarantee requirements

Include explicit rules requiring:

1. the claim established by each trusted representation;
2. construction protection;
3. boundary decoding preservation;
4. escape-hatch inventory;
5. external mutable facts;
6. runtime failures;
7. indeterminate outcomes;
8. proportionate executable evidence.

## Boundary requirements

<Add stable rule IDs for raw representations, parse/resource limits, validation,
authentication/authorization where relevant, persistence decoding, versioning,
sensitive-data handling, failure mapping, and remaining uncertainty.>

## Waiver requirements

<Specify which rules permit a waiver and require scope, owner, consequence,
compensating control, expiry, and removal condition. A waiver cannot make an
unsound safe API or false external claim true.>
