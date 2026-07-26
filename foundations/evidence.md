# Evidence

Trusted domain values are evidence-carrying values. Their representation and construction
record that a particular check, transition, observation, or authority grant occurred. The
strength of the type's name and documentation must not exceed that evidence.

Evidence is not metaphysical proof. It is a scoped claim established by a mechanism under
assumptions. A private `NonZeroU64` field can make zero unconstructible through safe public
paths. It cannot establish that the amount is affordable, belongs to a particular currency,
or follows a correct tax calculation. An ownership-verification response can justify
`VerifiedEmailAddress`; it cannot guarantee continued mailbox control or future
deliverability.

## Evidence levels

The following progression is common but not universal:

```text
raw input
    ↓
parsed value
    ↓
syntactically valid value
    ↓
policy-accepted value
    ↓
externally verified value
    ↓
authorized capability
    ↓
persisted fact
    ↓
reconciled external outcome
```

Each arrow is a fallible evidence-producing operation. Systems may branch or omit levels, but
they must not silently rename a lower level as a higher one.

### Raw input

Raw input is bytes, text, loosely typed JSON, a database row, an environment variable, an FFI
pointer, or another representation not yet interpreted by the domain. Size limits and
resource controls may be required before parsing. Raw input is not “bad”; it is simply
untrusted for domain use.

### Parsed value

Parsing establishes structural interpretation: text became an integer, JSON became a request
DTO, or bytes became a protocol frame. Parsing may reject malformed representation but does
not necessarily enforce domain policy. A parsed integer can still be zero or outside an
account limit.

### Syntactically valid value

Syntax validation establishes a documented grammar or local shape. An `EmailAddress` example
might require one `@`, non-empty local and domain parts, bounded length, and a dotted domain.
That is not RFC-complete validity, deliverability, or ownership. The validation policy must
be named and tested.

### Policy-accepted value

Policy acceptance applies current domain rules: allowed country, permitted currency, password
strength, configured amount bounds, or product availability. Policy can change and may depend
on configuration. Persisted evidence should record policy version or be revalidated when
current acceptance matters.

### Externally verified value

External verification relies on another authority or observation: a mailbox challenge was
completed, an identity provider authenticated a principal, or a bank confirmed an account.
The value should carry verification identity, time, issuer, scope, or expiry where those
affect use. Network and provider failures remain runtime failures.

### Authorized capability

A capability indicates local possession of authority to request an operation. Constructor
visibility, unforgeable tokens, limited methods, and non-clonability can strengthen it.
Revocation, expiry, leakage, serialization, and external enforcement remain part of the
contract. A capability is not the result of exercising authority.

### Persisted fact

Persistence establishes that a representation was accepted by a specific storage operation
under a schema and transaction. It may provide version or commit identity. It does not make
data forever current, preserve new invariants automatically, or include external effects
outside the transaction.

### Reconciled external outcome

Reconciliation establishes a later observation about an effect whose immediate result was
unknown. It ties an operation identifier or provider reference to a confirmed outcome.
Reconciliation evidence should record authority, observation time, and causality. Even then,
the claim has a boundary: a confirmed capture does not establish later settlement.

## Evidence-accurate naming

Names are public claims. A useful progression is:

```text
EmailInput
EmailAddress
DeliverableEmailAddress
VerifiedEmailAddress
```

`EmailInput` says only where the representation came from. `EmailAddress` should document its
syntax policy. `DeliverableEmailAddress` would require evidence that a delivery route accepted
or is expected to accept the address; the exact mechanism and time matter.
`VerifiedEmailAddress` requires an ownership-verification process and protected construction.
These types are not interchangeable.

Avoid aspirational names such as `SafePath`, `AuthorizedUser`, or `CommittedTransaction`
unless constructors and boundaries establish the stated evidence. Prefer narrower names:
`NormalizedRelativePath`, `AuthenticatedPrincipal`, `CaptureCapability`, or
`CommitAcknowledged`, as appropriate.

## Establishing evidence

For each evidence-carrying type, record:

- claim established;
- input and preconditions;
- producer or authority;
- validation or transition algorithm;
- policy or protocol version;
- time and expiry when relevant;
- protected constructor location;
- persistence and deserialization path;
- error and indeterminate outcomes;
- revocation or invalidation;
- evidence tests;
- non-guarantees.

The producer matters. A public `VerifiedEmailAddress::new(String)` cannot establish external
verification because any caller can invoke it without proof. A verifier-owned proof token
whose field is private and whose constructor is restricted can make the transition harder to
forge. If the token is `Clone`, serializable, or valid forever, those semantics need explicit
justification.

## Preserving evidence

Private fields protect only ordinary struct construction. Evidence can be lost or forged
through:

- derived `Deserialize` that writes fields directly;
- unchecked `From<String>`;
- database `FromRow` construction without validation;
- public `from_raw` or `new_unchecked`;
- broad `unsafe` constructors;
- mutation methods that no longer validate;
- public enum variants carrying trusted inner data;
- `Default` values that do not satisfy the claim;
- cloning or serialization of authority;
- migration scripts that write impossible historical data;
- FFI values accepted without layout or semantic checks;
- stale cache reloads under newer policy.

Every boundary uses the protected constructor or an explicitly reviewed equivalent. Where a
bypass is necessary for trusted internal performance, scope it narrowly, state preconditions,
make misuse visible, and test the safe façade.

## Evidence composition

Evidence can be composed only when scopes align. `PositiveMoney<USD>` plus a verified account
does not prove sufficient funds. An authenticated principal plus an authorization policy
decision can produce a scoped capability, but only for the resource, operation, tenant, and
time covered. A persisted authorization plus a current capture request must still compare
payment identity and amount.

Composition often belongs in a domain service or transactional operation. Encoding every
cross-entity fact in generic parameters can create stale evidence and state explosion.
Structural types should carry stable local facts; runtime services should establish temporal
and relational facts.

## Evidence decay and revocation

Some evidence is immutable history: a compiler accepted a revision, a challenge completed, or
a database acknowledged a commit. The implications can still decay. Authorization may be
revoked, a policy version superseded, a certificate expire, or an external status change.

Types cannot freeze mutable external reality. Designs use expiry, version fields, observation
timestamps, revocation checks, leases, or forced revalidation. The type name should distinguish
historical evidence from current authority when that difference matters.

## Evidence in failures

Errors and unknown outcomes also carry evidence. A rejection can include a provider decision
code; a validation error identifies which policy failed; an unknown capture carries operation
and reconciliation identifiers. Collapsing all failures to text loses machine-actionable
evidence. Collapsing timeout to rejection invents evidence the system does not have.

## Review

Review follows the evidence chain from every producer to every consumer. It attempts direct
construction, alternate deserialization, invalid historical rows, clones, expired tokens, and
wrong-entity composition. Tests should demonstrate accepted and rejected values, while
compile-fail tests demonstrate important prohibited programs.

Evidence supports precise confidence. Passing tests are evidence for selected behavior on the
tested revision and environment. They do not prove universal correctness. Honest naming and
boundary preservation keep limited evidence useful rather than turning it into false
certainty.
