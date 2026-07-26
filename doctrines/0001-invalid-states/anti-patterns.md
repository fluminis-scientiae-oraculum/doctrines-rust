# Anti-patterns

## Boolean state soup

**Weak example.** A struct contains `is_paid`, `is_failed`, `is_sending`, optional receipt,
and optional failure reason.

**Why it fails.** Independent fields represent contradictory combinations and force repeated
checks.

**Risk.** Invalid persistence, unreachable UI branches, missing state-specific evidence.

**Improved direction.** Use an enum whose variants carry receipt, reason, form, or operation
identity.

**Justified appearance.** An untrusted legacy DTO may retain the shape solely for checked
conversion and migration.

## Public validated tuple field

**Weak example.** `pub struct PositiveMoney(pub u64);`

**Why it fails.** Any caller can construct zero; the wrapper proves only nominal distinction.

**Risk.** Business code omits checks because the name implies validation.

**Improved direction.** Private `NonZeroU64`, fallible constructor, currency-aware aggregate,
and checked arithmetic.

**Justified appearance.** A public wrapper can be appropriate when it deliberately asserts no
validation and the name does not imply one.

## Partial smart constructor

**Weak example.** `EmailAddress::new` checks only `contains('@')`.

**Why it fails.** The check does not establish a meaningful documented syntax policy, and
another constructor may be weaker still.

**Risk.** Invalid contact data, inconsistent boundary behavior, misleading verification
claims.

**Improved direction.** Define a bounded syntax policy, test it, and separate ownership
verification.

**Justified appearance.** A permissive raw input type is fine when it is named raw and all
delivery failure remains expected.

## Derive bypass

**Weak example.** A trusted private-field newtype derives `Deserialize` directly.

**Why it fails.** The decoder may populate representation without calling the constructor.

**Risk.** Network, cache, or file input forges trusted evidence.

**Improved direction.** Deserialize a raw representation and delegate through `TryFrom` or a
manual visitor.

**Justified appearance.** Direct derive is safe for a representation whose Rust-valid values
all satisfy the complete invariant.

## Trusted database myth

**Weak example.** ORM row mapping returns `VerifiedEmailAddress` from a text column without
verification metadata or validation.

**Why it fails.** Historical rows, other writers, manual repair, and policy change are outside
the current proof.

**Risk.** Forged evidence and irreparable migration ambiguity.

**Improved direction.** Decode a raw row, validate syntax and evidence fields, quarantine
invalid history, and version policy.

**Justified appearance.** Direct mapping can be valid for database-native values whose entire
claim is exactly enforced by the schema and decoder.

## Typestate everywhere

**Weak example.** Every persisted payment provider status becomes a generic marker type and
all combinations become separate concrete types.

**Why it fails.** External state is dynamic, persisted, heterogeneous, and mutable. Generic
states create explosion without external control.

**Risk.** Erasure at every boundary, stale proof, poor diagnostics, migration friction.

**Improved direction.** Runtime status enum plus transactional transition validation; add a
short-lived local capability where useful.

**Justified appearance.** A small local builder or connection protocol may benefit from
typestate.

## Infallible open connection

**Weak example.** `Connection<Open>::send` returns a receipt directly because the type is open.

**Why it fails.** The type records a local historical transition; the peer can close
immediately.

**Risk.** Panic, hidden retry, data loss, or false success.

**Improved direction.** Return structured `Result` and document remote-liveness
non-guarantee.

**Justified appearance.** A pure in-memory mock may be infallible if its type does not claim
network semantics and is not substituted where failure behavior matters.

## Timeout means failure

**Weak example.** Any payment-provider timeout transitions persisted state to `Failed`.

**Why it fails.** The request may have committed and only the response was lost.

**Risk.** Duplicate capture on retry and a false audit record.

**Improved direction.** Store `UnknownCapture` with operation and reconciliation identity.

**Justified appearance.** A protocol-defined pre-commit timeout may establish non-execution
when that guarantee is verified.

## Integer money solves rounding

**Weak example.** Documentation states that minor-unit integers eliminate rounding.

**Why it fails.** Tax, foreign exchange, discounts, allocation, and currencies with differing
scales still produce fractions and policy choices.

**Risk.** Silent bias, reconciliation differences, accounting defects.

**Improved direction.** State representation guarantee separately from calculation,
allocation, and rounding policy.

**Justified appearance.** An operation that only stores and adds same-scale whole minor units
may require no rounding at that step; document the narrow scope.

## Clone the capability

**Weak example.** An exclusive capture token derives `Clone` for convenience.

**Why it fails.** Local authority becomes duplicable and consuming one clone does not consume
the others.

**Risk.** Repeated irreversible effects or authority leakage.

**Improved direction.** Remove clone, issue operation-scoped identity, and make provider
idempotency a separate explicit control.

**Justified appearance.** Read-only shared capability can be cloneable when duplication is
part of the authority contract and revocation is handled.

## Unchecked constructor as ordinary API

**Weak example.** `pub fn new_unchecked(value: String) -> Self` is safe and used by all
adapters for speed.

**Why it fails.** Proof responsibility is invisible and boundaries bypass the canonical
policy.

**Risk.** Invariant erosion spreads through apparently valid types.

**Improved direction.** Remove it, restrict visibility, or make the exceptional precondition
and ownership explicit; measure before adding a fast path.

**Justified appearance.** A narrow unsafe constructor can support verified FFI or a measured
internal path under complete documented obligations and safe encapsulation.

## Type-level cross-entity truth

**Weak example.** `FundedAccount` is constructed once and later treated as proof that funds
remain sufficient.

**Why it fails.** Concurrent withdrawals, holds, expiry, or external updates change the fact.

**Risk.** Overspend or authorization bypass.

**Improved direction.** Treat the type as a timestamped observation or revalidate within the
transaction that spends funds.

**Justified appearance.** An immutable snapshot can carry historical evidence when its name
and API make staleness explicit.

## Test names as proof

**Weak example.** A test called `invalid_states_are_impossible` constructs only valid values.

**Why it fails.** The violation path and alternate constructors remain untested.

**Risk.** Review mistakes a label for evidence.

**Improved direction.** Add negative constructor tests, boundary tests, and compile-fail cases
for the precise prohibited programs.

**Justified appearance.** Broad scenario names are harmless when assertions and adjacent
documentation clearly state evidence scope.
