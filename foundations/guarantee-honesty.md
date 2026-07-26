# Guarantee honesty

A guarantee is a claim backed by an enforcement mechanism and evidence within a stated scope.
Guarantee honesty prevents type names, API documentation, reviews, and generated agent context
from becoming stronger than the implementation.

The discipline separates four things:

1. **Claim:** what the design says is true.
2. **Mechanism:** how the design attempts to establish or preserve it.
3. **Evidence:** what was observed about a specific revision, configuration, or runtime.
4. **Residual risk:** what can still fail, change, or remain unknown.

A private field is a mechanism. Compiler rejection of direct construction is evidence for one
class of program. Neither proves database decoding uses the constructor. A passing integration
test is evidence for tested behavior; it does not prove all schedules or external histories.

## Required questions

Every type-level design, capability, state machine, boundary conversion, and external-outcome
model must answer:

1. **What does the type prove?** State the narrow invariant, transition history, authority,
   or observation represented.
2. **How is the proof established?** Name constructor, parser, verifier, transaction,
   protocol response, reconciliation, or compiler rule.
3. **How is construction protected?** Enumerate visibility, private fields, sealed proof
   tokens, non-clonability, consuming APIs, and mutation controls.
4. **How does decoding preserve the proof?** Trace Serde, database, cache, migration, FFI, and
   versioned representation paths.
5. **Which escape hatches exist?** Name unchecked, unsafe, privileged, test-only, feature-gated,
   or migration paths and their review contracts.
6. **What does the type not prove?** List adjacent facts a reader may mistakenly infer.
7. **Which facts can change externally?** Include revocation, expiry, liveness, balance,
   policy, topology, or provider state.
8. **Which failures remain runtime failures?** Include I/O, resource exhaustion, rejection,
   cancellation, contention, and provider behavior.
9. **Which outcomes may be indeterminate?** Include transmitted requests without
   acknowledgement, ambiguous commit, lost messages, and stale observation.

If an answer is absent, narrow the claim or complete the design.

## Guarantee ledger

Use this ledger for major types, case studies, review, and pull requests:

| Claim | Established by | Protected construction | Boundary preservation | Escape hatches | Does not prove | Residual runtime risk |
|---|---|---|---|---|---|---|
| `PositiveMoney` is non-zero | `NonZeroU64` accepted by a fallible constructor | private field; no unchecked public constructor | DTO and row conversions call constructor | scoped migration conversion if reviewed | sufficient funds, correct FX, tax or allocation policy | overflow on later arithmetic, currency mismatch |
| `VerifiedEmailAddress` passed ownership verification | verifier-only proof token after completed challenge | private fields and restricted proof-token constructor | persisted issuer, scope, time, and address revalidated on load | administrative import with audit | future deliverability, continued control, RFC-complete validity | revocation, expiry, provider error |
| `Connection<Open>` completed local connection transition | consuming `connect` returned `Ok` | state marker and constructor visibility | not normally serialized; restoration requires a new connection | test transport factory | remote liveness at next send | immediate network failure, peer closure |
| `AuthorizedPayment` passed local authorization transition | accepted authorization response and identity/amount checks | consuming transition; capability not freely cloneable | row decode validates status and authorization reference | repair tool with scoped authorization | capture success, settlement, absence of provider reversal | timeout, expiry, provider rejection |
| `UnknownCapture` has reconciliation identity | explicit outcome constructor after ambiguous transport result | private operation and token fields | durable row stores operation identity and provider scope | manual reconciliation record with audit | whether capture succeeded or failed | delayed visibility, concurrent reconciliation |

Ledger rows should identify exact project types and methods during review. Generic examples
teach structure but are not evidence for an implementation.

## Construction audit

List every path that can create or change the trusted value:

- public and crate-visible constructors;
- struct literals and enum variants;
- `Default`, `From`, `TryFrom`, `FromStr`, builders, and macros;
- `Deserialize`, custom visitors, and remote adapters;
- database row mappings and ORM derives;
- migration and administrative repair code;
- cloning, copying, mutation, and collection insertion;
- test utilities and feature-gated APIs;
- FFI imports and raw-pointer wrappers;
- unsafe and unchecked functions.

The documented invariant is complete only if all paths establish or explicitly assume it. A
private field plus derived `Deserialize` can be dishonest. A complete `new` plus weaker
`From<String>` is dishonest. A capability that derives `Clone` may turn exclusive authority
into duplicable authority.

## Boundary preservation

Trusted memory does not make persisted or serialized bytes trusted. Decode into a raw
representation, then validate through the canonical constructor. If the wire format needs a
stable shape, use Serde's `try_from` or a manual implementation. Database adapters should use
`TryFrom<Row>`, return invalid historical data as a distinct failure, and provide quarantine
or repair policy.

Versioning is part of the proof. A value accepted under policy version 1 may not satisfy
version 2. Either retain evidence that v1 remains acceptable, migrate it, revalidate it, or
represent its legacy state honestly.

## Escape hatches

Some systems require a bypass for trusted constants, bulk migration, FFI, or measured hot
paths. The escape hatch must be:

- visibly named;
- narrower in visibility than ordinary construction;
- documented with complete preconditions;
- owned by a specific module or operational role;
- excluded from generic boundary adapters;
- covered by tests of the safe interface;
- discoverable by audit;
- and reviewed under the doctrine governing its risk.

`unsafe` means the compiler cannot verify the proof; it does not mean the invariant is
optional. A safe `from_raw_unchecked` is often worse because it looks ordinary while
transferring proof responsibility.

## External reality

Rust types describe local program evidence. They cannot freeze a network, user, database,
clock, credential issuer, remote service, or physical resource.

`Connection<Open>` records a local successful transition. The peer may close immediately.
`AuthenticatedPrincipal` records an authentication result; the session may expire or be
revoked. `AuthorizedCapability` records a decision under a policy and resource scope; policy
or ownership can change. `Persisted<T>` records a storage acknowledgement; a concurrent actor
may update the row. Documentation must state observation time, validity bounds, and required
rechecks.

## Failure and indeterminacy

External effects remain fallible after every compile-time sequencing check. Error categories
must preserve operational distinctions: rejection, validation failure, cancellation,
conflict, timeout, local resource failure, and unknown external outcome.

A timeout is not necessarily failure. If a request may have reached the remote system, the
result is unknown unless protocol guarantees otherwise. The type should carry operation
identity and reconciliation instructions. Automatically retrying may duplicate a payment,
message, or provisioning action.

Database commit can also be ambiguous when the connection fails around acknowledgement. The
application must use database-specific evidence, idempotent operation identity, or
reconciliation rather than report fictional rollback.

## Evidence quality

Evidence is bound to scope:

- the compiler rejects a selected forbidden program under a named API and toolchain;
- a unit test observes selected constructor behavior;
- a property test samples a generated input model;
- an integration test crosses a configured boundary;
- model checking explores a bounded state space;
- telemetry observes deployed histories;
- an incident falsifies an assumption.

More tests do not expand the claim automatically. Review asks what violation each test could
detect, which environment it ran against, and what it cannot observe. Updating snapshots or
compiler diagnostics without semantic inspection weakens evidence.

## Language discipline

Prefer “establishes,” “prevents through safe public construction,” “records that,” and “was
observed” over absolute terms such as “ensures forever.” Pair a guarantee with its
non-guarantee in the same section. If a type name repeatedly invites a stronger inference,
rename it rather than relying on distant caveats.

Honesty is not pessimism. Narrow guarantees compose. A type that accurately proves one fact is
more useful than a type that vaguely claims a whole business outcome. Explicit uncertainty
lets systems recover without corrupting their own account of reality.
