# Case study template: <Replace with case title>

When materializing a case, split this structure into `problem.md`, `naive.md`,
`improved.md`, and `remaining-uncertainty.md`.

## Problem

Define domain vocabulary, actors, lifecycle, trust boundaries, persistence,
external effects, and at least one invariant table:

| ID | Statement | Classification | Enforcement candidate | Failure consequence |
|---|---|---|---|---|
| <CASE-01> | <Exact invariant> | <value/state/etc.> | <mechanism> | <consequence> |

List failure points before, between, and after durable/external steps.

## Naive representation

Show realistic compiling Rust or data shape. Explain contradictory states,
unchecked values, authority forgery, boundary bypass, error collapse,
concurrency, persistence, retry, and evidence weaknesses. Do not construct a
straw man that no engineer would write.

## Improved representation

Provide:

- protected domain values and state;
- boundary DTO/row conversion;
- authorization/capability flow;
- legal transitions;
- database/version/transaction behavior;
- external operation identity and outcome model;
- cancellation, retry, duplicate, and reconciliation behavior;
- positive, negative, compile-fail, integration, and fault evidence;
- complexity tradeoffs.

## Guarantee ledger

| Claim | Established by | Protected construction | Boundary preservation | Escape hatches | Does not prove | Residual runtime risk |
|---|---|---|---|---|---|---|
| <Claim> | <Evidence> | <Protection> | <Boundary> | <Escape> | <Non-proof> | <Risk> |

## Remaining uncertainty

State external mutable facts, time scope, provider/database/runtime assumptions,
policy evolution, permanent ambiguity, operational capacity, privacy, and
human decisions. End with an exact statement of improvement and limits.
