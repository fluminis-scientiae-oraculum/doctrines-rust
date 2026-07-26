# Review standard

Record each gate as pass, fail, not applicable, or a waiver reference. “Looks idiomatic” is
not evidence.

## Gate 1 — Invariant inventory

**Question.** Are consequential value, state, transition, authority, boundary, cross-entity,
temporal, and distributed invariants identified with owners?

**Pass evidence.** Inventory links each statement to enforcement, boundary, consequence, and
residual uncertainty.

**Failure examples.** Types were selected first; ownership or timeout semantics are absent.

**Severity.** Critical when effects or authority are involved; otherwise major.

**Remediation.** Complete discovery and reconsider representation.

## Gate 2 — Mutually exclusive states

**Question.** Can booleans, options, or discriminants express contradictory states?

**Pass evidence.** Enum variants carry only relevant data; external DTO contradiction is
rejected during conversion.

**Failure examples.** Paid without receipt; failed and submitted simultaneously.

**Severity.** Major.

**Remediation.** Introduce a sum type and migration plan.

## Gate 3 — Construction protection

**Question.** Can untrusted callers construct a trusted value without complete validation?

**Pass evidence.** Private fields, complete fallible constructors, restricted proof issuance,
and no weaker `From`, builder, default, macro, or re-export.

**Failure examples.** Public tuple field; public `from_raw`; builder skips one rule.

**Severity.** Critical for authority or safety; major otherwise.

**Remediation.** Restrict representation and consolidate construction.

## Gate 4 — Evidence-accurate names

**Question.** Does every type and state name match the evidence actually established?

**Pass evidence.** Guarantee ledger maps names to producers and non-guarantees.

**Failure examples.** `VerifiedEmail` from syntax parser; `Open` documented as remote liveness.

**Severity.** Major; critical when it drives security or external retry.

**Remediation.** Narrow name or strengthen evidence and protected construction.

## Gate 5 — Serde and format decoding

**Question.** Does every deserializer preserve the canonical invariant?

**Pass evidence.** Raw DTO plus `TryFrom`, Serde `try_from`, or equivalent manual validation;
invalid and oversized input tests.

**Failure examples.** Derived `Deserialize` writes private field; unknown variant maps to
default.

**Severity.** Critical at untrusted boundaries.

**Remediation.** Decode structurally, validate canonically, define version behavior.

## Gate 6 — Database decoding

**Question.** Can historical or alternate-writer rows forge domain values?

**Pass evidence.** Checked row conversion, schema constraints, invalid-history quarantine,
migration and version tests.

**Failure examples.** ORM derive directly constructs trusted type; invalid row is coerced.

**Severity.** Critical for financial, authority, or safety data; otherwise major.

**Remediation.** Separate persistence representation and validate.

## Gate 7 — Mutation preservation

**Question.** Can mutation, dereferencing, iteration, cloning, or collection conversion erode
the invariant or authority?

**Pass evidence.** Controlled methods, read-only borrowing, clone rationale, negative tests.

**Failure examples.** `NonEmptyVec` exposes `clear`; capability derives `Clone` without scope.

**Severity.** Major or critical by consequence.

**Remediation.** Narrow API or move validation to every mutation.

## Gate 8 — Transition legality

**Question.** Do APIs prevent significant wrong-order or repeated local operations?

**Pass evidence.** Consuming transition or transactional runtime validator; compile-fail or
concurrency tests.

**Failure examples.** Capture accepts draft payment; transaction can commit twice.

**Severity.** Critical for irreversible effects; major otherwise.

**Remediation.** Encode prior evidence or validate atomically.

## Gate 9 — Typestate proportionality

**Question.** Is typestate locally controlled, small, static, recoverable, and cheaper than
runtime alternatives?

**Pass evidence.** Complexity record, state graph, diagnostics, persistence decision, async
failure semantics.

**Failure examples.** Generic state mirrors dozens of persisted provider statuses.

**Severity.** Major maintainability concern; critical if it creates false certainty.

**Remediation.** Simplify to runtime enum, consuming method, or hybrid.

## Gate 10 — Authority

**Question.** Are issuance, scope, clone, transfer, serialization, expiry, revocation, and use
count defined?

**Pass evidence.** Capability construction and call-site audit plus misuse tests.

**Failure examples.** Public capability constructor; serializable admin token; stale grant.

**Severity.** Critical.

**Remediation.** Restrict issuance and define mutable authority checks.

## Gate 11 — External fallibility

**Question.** Do local state proofs leave every external effect fallible?

**Pass evidence.** Structured results distinguish expected categories and preserve sources.

**Failure examples.** `send` returns receipt without error; destructor claims external rollback.

**Severity.** Critical when failure would corrupt state; otherwise major.

**Remediation.** Restore fallible API and recovery semantics.

## Gate 12 — Commitment and timeout

**Question.** Can failure of acknowledgement occur after external commitment?

**Pass evidence.** Protocol analysis identifies commitment point and maps timeout to explicit
unknown where necessary.

**Failure examples.** Timeout becomes `Failed`; automatic retry has no idempotency analysis.

**Severity.** Critical.

**Remediation.** Add unknown outcome, identity, and reconciliation.

## Gate 13 — Unknown outcome durability

**Question.** Can uncertainty survive process restart and be resolved without losing causality?

**Pass evidence.** Durable operation ID, idempotency key, provider scope, reconciliation token,
audit correlation, and worker tests.

**Failure examples.** Unknown exists only in memory; restart retries blindly.

**Severity.** Critical.

**Remediation.** Persist uncertainty before or atomically with operation progress.

## Gate 14 — Money contract

**Question.** Are amount, currency, overflow, arithmetic compatibility, and rounding/allocation
policy represented honestly?

**Pass evidence.** Non-zero or bounded constructor, same-currency tests, policy owner and
non-guarantees.

**Failure examples.** `u64` called positive; integers described as eliminating rounding.

**Severity.** Critical for movement of funds; major for display-only calculations.

**Remediation.** Refine amount and currency; document policy boundaries.

## Gate 15 — Email evidence

**Question.** Are syntax, policy acceptance, ownership verification, and deliverability kept
distinct?

**Pass evidence.** Documented parser, verifier-owned transition, expiry or revocation policy,
and delivery failure handling.

**Failure examples.** `contains('@')`; syntax result named verified.

**Severity.** Critical for authentication or recovery; major otherwise.

**Remediation.** Separate evidence levels and constructors.

## Gate 16 — Escape hatches

**Question.** Are all unchecked, unsafe, administrative, migration, test, and feature-gated
bypasses explicit and scoped?

**Pass evidence.** Search inventory and reviewed call sites with documented preconditions.

**Failure examples.** Ordinary decoder calls `new_unchecked`; test helper is enabled in
production.

**Severity.** Critical.

**Remediation.** Remove, restrict, or govern under RUST-DOC-0007.

## Gate 17 — Compiler evidence

**Question.** Are important prohibited programs tested and their diagnostics semantically
reviewed?

**Pass evidence.** Minimal compile-fail cases plus positive tests on pinned stable.

**Failure examples.** UI test fails for unused import; diagnostics overwritten after upgrade.

**Severity.** Major.

**Remediation.** Target the intended rejection and inspect `.stderr`.

## Gate 18 — Guarantee ledger

**Question.** Are guarantees, construction, decoding, escape hatches, non-guarantees, and
residual risk complete?

**Pass evidence.** Ledger entries trace to exact code and tests.

**Failure examples.** Documentation states “impossible” while public bypass or external
mutation exists.

**Severity.** Critical for misleading safety/security claims; otherwise major.

**Remediation.** Narrow claim or strengthen mechanism and evidence.
