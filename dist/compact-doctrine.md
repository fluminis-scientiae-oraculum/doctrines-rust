<!--
GENERATED FILE. DO NOT EDIT DIRECTLY.
Canonical sources live under /foundations, /doctrines, /patterns,
 /boundaries, /reviews, and /agents.
-->

# Compact Rust doctrine hydration

---

## Source: `agents/compact-core.md`

# Compact doctrine core

## Thesis

Rust quality requires discovering important invariants, encoding those that are
structurally enforceable, constraining legal states and transitions, preserving
validation at trust boundaries, modeling external failure honestly,
representing distributed uncertainty explicitly, and keeping type complexity
proportional to risk removed.

## Invariant classification

Classify value, state, transition, authority, lifecycle, boundary, cross-entity,
temporal, environmental, and distributed invariants. Record statement, scope,
owner, enforcement, boundary, evidence, consequence, and residual uncertainty.
Distinguish invariant, precondition, postcondition, policy, assumption,
observation, and desired outcome.

## Boundary pipeline

```text
raw input → parse → structural value → validate → trusted domain value
          → execute fallible effect → observe/reconcile
          → confirmed evidence or explicit uncertainty
```

Validation moves to protected construction and boundaries; it never disappears.

## Mechanism selection

Use an enum for mutually exclusive state, opaque newtype for one stable local
invariant, validated wrapper for a collection invariant, consuming transition
or typestate for a small locally controlled sequence, capability for authority,
runtime enum/service for dynamic or persisted state, and runtime validation for
external/cross-entity facts. Use plain code when machinery costs more than the
risk it removes.

## Guarantee honesty

For each claim record establishment, construction protection, boundary
preservation, escape hatches, non-proofs, mutable external facts, runtime
failures, and indeterminate outcomes. Type names describe evidence, not
aspiration.

## Core audit

- Can any constructor, decoder, row mapper, migration, or feature forge trusted
  state?
- Can mutation invalidate a wrapper?
- Can authority be cloned, serialized, or used after revocation?
- Does local typestate claim remote liveness?
- Does timeout become rejection?
- Does retry reuse one operation identity and satisfy idempotency?
- Are duplicates, order scope, acknowledgement loss, and reconciliation
  explicit?
- Are unsafe obligations complete?
- Do tests cover prohibited and partial-failure paths?
- Are performance claims measured under a defined workload?

---

## Source: `foundations/README.md`

# Foundations

Foundations define the vocabulary and reasoning contracts used by every doctrine in this
repository. They are separate because a doctrine should not privately redefine “invariant,”
“evidence,” “trusted,” “guarantee,” or the force of `MUST`. A shared definition lets rule IDs
compose across domain modeling, errors, persistence, distributed effects, unsafe code,
testing, and performance.

## Dependency direction

The dependency direction is intentionally one-way:

```text
foundations
    ↓
normative doctrines
    ↓
patterns and boundary guides
    ↓
review procedures and agent workflows
    ↓
case studies and executable evidence
```

A foundation describes concepts and interpretation. A doctrine imposes requirements using
those concepts. A pattern presents a reusable mechanism that may satisfy one or more
requirements. A boundary guide specializes doctrine for a place where representations or
authority cross systems. A review procedure asks for evidence. A case study demonstrates a
coherent application and records what remains uncertain.

This direction matters during disagreement. A case study cannot establish that typestate is
universally preferred merely because it uses typestate. A pattern cannot weaken a doctrine's
constructor rule for convenience. A source note cannot become normative through repetition.
If a shared definition must change, the change is governed explicitly because every
downstream document may be affected.

## Reading order

Read the foundation documents in this order:

1. `normative-language.md` defines the force and scope of requirements, informative text,
   examples, exceptions, and waivers.
2. `invariants.md` distinguishes invariants from preconditions, policies, assumptions,
   observations, and desired outcomes. It supplies the inventory that precedes representation
   choice.
3. `evidence.md` describes the evidence carried by a value or capability and requires names
   to match what construction actually established.
4. `trust-boundaries.md` maps untrusted representations through parse and validation into
   trusted domain values, then through effects into observations or explicit uncertainty.
5. `guarantee-honesty.md` requires a ledger for proofs, protected construction, boundary
   preservation, escape hatches, non-guarantees, and residual risk.
6. `complexity-budget.md` keeps structural enforcement proportional to consequence and
   prevents type machinery from obscuring the system.

The order is a reasoning sequence, not a ranking. Complexity does not override a safety
invariant, and an important invariant does not authorize an inaccurate guarantee. The design
must make a risk-owned tradeoff and preserve evidence at every boundary.

## Document classes

### Definition

A definition assigns a stable meaning to a term. Definitions are shared interpretation, not
an implementation command by themselves. For example, a transition invariant is a condition
that must hold across a permitted state change; this does not yet choose an enum, transaction,
or consuming method.

### Doctrine

A doctrine is a versioned normative contract. It contains identified requirements, intent,
applicability, permitted exceptions, and review evidence. Compliance is evaluated against
rule intent and system behavior, not a copied syntax fragment.

### Pattern

A pattern relates a recurring problem to a mechanism under stated forces. It identifies the
exact guarantee gained and not gained, boundary and persistence implications, evidence, cost,
and conditions where it should not be used. Patterns are selections, not mandates unless a
doctrine rule requires one in a defined case.

### Boundary guide

A boundary guide starts with less-trusted data, authority, or effects. It asks what is parsed,
what is validated, how trusted construction is protected, how failures and unknown values are
represented, and what remains mutable or uncertain. Boundaries include more than network
inputs: database rows, cached bytes, environment variables, filesystem state, and FFI values
are representations from outside the current domain proof.

### Review procedure

A review procedure is operational. Each item must be recorded as pass, fail, not applicable,
or a waiver reference. A prose assertion such as “validation looks adequate” is weaker than
evidence naming every constructor and decoding route. Review does not manufacture proof; it
checks whether the mechanism and evidence support the claim.

### Case study

A case study follows one domain through problem, weak model, improved model, and residual
uncertainty. It makes tradeoffs concrete without turning an example domain into a universal
rule. Its guarantee ledger must say what the design cannot establish.

## Using foundations during work

A planner begins by writing an invariant inventory and trust-boundary map. Each inventory row
names the owner, enforcement classification, failure consequence, evidence, and residual
uncertainty. The planner then chooses representation and records why a simpler or more complex
mechanism is proportionate.

An implementer traces every public construction path, decoding path, mutation path, and state
transition to the inventory. A reviewer attempts bypasses and distinguishes compiler-enforced
facts from runtime observations. An auditor checks whether type names or documentation have
silently grown stronger than their proofs. A maintainer rechecks the dependency direction
before changing a definition.

The foundations share a single discipline: a claim is not its mechanism, and a mechanism is
not evidence that it ran. `NonZeroU64` is a mechanism for excluding zero; the private field and
compiler rejection protect ordinary construction; tests demonstrate selected constructor
behavior; none of these establishes tax correctness. An external verification response may
establish evidence at one time, but it cannot guarantee that external reality remains
unchanged.

## Change policy

Changes that clarify wording without changing meaning use normal review. A change to
normative-term meaning, invariant classification, evidence interpretation, boundary trust
model, guarantee ledger obligations, or complexity policy can alter every doctrine and
requires the RFC process. The proposal must enumerate downstream contracts, compatibility,
migration, and evidence.

Foundations should remain domain-neutral but not vague. They use examples to expose
distinctions, name failure semantics, and state limitations. They avoid universal mechanisms:
structural enforcement is preferred when it removes consequential invalid programs, while
runtime validation remains necessary for external, temporal, cross-entity, and distributed
facts.

---

## Source: `foundations/guarantee-honesty.md`

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

| Claim                                                     | Established by                                                | Protected construction                                | Boundary preservation                                          | Escape hatches                          | Does not prove                                                  | Residual runtime risk                           |
| --------------------------------------------------------- | ------------------------------------------------------------- | ----------------------------------------------------- | -------------------------------------------------------------- | --------------------------------------- | --------------------------------------------------------------- | ----------------------------------------------- |
| `PositiveMoney` is non-zero                               | `NonZeroU64` accepted by a fallible constructor               | private field; no unchecked public constructor        | DTO and row conversions call constructor                       | scoped migration conversion if reviewed | sufficient funds, correct FX, tax or allocation policy          | overflow on later arithmetic, currency mismatch |
| `VerifiedEmailAddress` passed ownership verification      | verifier-only proof token after completed challenge           | private fields and restricted proof-token constructor | persisted issuer, scope, time, and address revalidated on load | administrative import with audit        | future deliverability, continued control, RFC-complete validity | revocation, expiry, provider error              |
| `Connection<Open>` completed local connection transition  | consuming `connect` returned `Ok`                             | state marker and constructor visibility               | not normally serialized; restoration requires a new connection | test transport factory                  | remote liveness at next send                                    | immediate network failure, peer closure         |
| `AuthorizedPayment` passed local authorization transition | accepted authorization response and identity/amount checks    | consuming transition; capability not freely cloneable | row decode validates status and authorization reference        | repair tool with scoped authorization   | capture success, settlement, absence of provider reversal       | timeout, expiry, provider rejection             |
| `UnknownCapture` has reconciliation identity              | explicit outcome constructor after ambiguous transport result | private operation and token fields                    | durable row stores operation identity and provider scope       | manual reconciliation record with audit | whether capture succeeded or failed                             | delayed visibility, concurrent reconciliation   |

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

---

## Source: `doctrines/0001-invalid-states/decision-framework.md`

# Decision framework

## Start with the invariant

Do not begin with `struct`, `enum`, `PhantomData`, or a library. Write a falsifiable invariant,
its owner, classification, boundary, consequence, and residual uncertainty. Then ask whether
the fact is stable and local, dynamic and persisted, relational, authoritative, or external.

The first selection table is:

| Problem                               | Preferred mechanism                                |
| ------------------------------------- | -------------------------------------------------- |
| Mutually exclusive state              | `enum`                                             |
| Validated scalar or identifier        | opaque newtype                                     |
| Non-empty or bounded collection       | validated wrapper                                  |
| Locally controlled operation sequence | typestate or consuming transition                  |
| Authority to perform an operation     | capability type                                    |
| Dynamic or persisted state            | runtime enum/state machine                         |
| External input                        | runtime parse and validation                       |
| Cross-entity business rule            | domain service or transactional runtime validation |
| External success/failure              | `Result`                                           |
| Indeterminate distributed outcome     | explicit unknown/reconciliation state              |

“Preferred” means first candidate, not automatic answer. Multiple mechanisms often compose.

## Operational decision tree

```text
Is the problem mutually exclusive states?
├─ yes → enum / sum type
└─ no
   Is it a single value with a stable local invariant?
   ├─ yes → opaque validated newtype
   └─ no
      Is it a collection invariant?
      ├─ yes → validated collection wrapper
      └─ no
         Is it locally controlled operation sequencing?
         ├─ yes
         │  Is state count small and API static?
         │  ├─ yes → typestate or consuming transitions
         │  └─ no → runtime state machine
         └─ no
            Is it authority?
            ├─ yes → capability type
            └─ no
               Is it external or mutable reality?
               ├─ yes → runtime observation + Result
               │        + explicit uncertainty where needed
               └─ no → ordinary runtime rule or domain service
```

Before accepting the leaf, apply complexity and honesty checks:

- Can the mechanism's proof become stale?
- Does persistence need a runtime discriminant?
- Can an external effect occur without acknowledgement?
- Does an enum already remove the contradiction more simply?
- Does a consuming method prevent the actual misuse without generic state?
- Can callers understand compiler diagnostics?
- Are all constructors and boundary decoders protected?
- What does the mechanism not prove?

## State decision

Use an enum when cases are mutually exclusive, data differs by case, state is inspected at
runtime, or persistence matters. Put only state-relevant data in each variant. Decide unknown
and future variant behavior at external boundaries.

Use a runtime transition service when legality depends on current database facts,
authorization, concurrent version, or external state. Validate expected prior status and
cross-entity invariants transactionally.

Consider typestate when all answers are favorable:

| Question                          | Favorable evidence                                               |
| --------------------------------- | ---------------------------------------------------------------- |
| Is sequencing locally controlled? | Current owner chooses every transition                           |
| Is the graph small?               | Few states and stable transitions                                |
| Is ownership natural?             | Prior state can be consumed without harming recovery             |
| Is storage unnecessary?           | Value is short-lived or runtime conversion is explicit           |
| Are callers static?               | No routine heterogeneous collection or dynamic dispatch          |
| Is failure designed?              | Transition returns prior state, error evidence, or unknown state |
| Are diagnostics usable?           | Compile-fail examples point to the domain mistake                |
| Is cost proportionate?            | Consequence exceeds API/build/maintenance cost                   |

If several answers are unfavorable, stop and use a runtime enum or consuming method.

## Value decision

Use an opaque newtype when the invariant:

- concerns one value;
- is stable enough to name;
- can be checked without mutable external state;
- is valuable after construction;
- and can be preserved through all mutations and boundaries.

Choose the evidence level before the name. For email, decide whether the type means parsed
input, a documented syntax subset, policy acceptance, ownership verification, or delivery
observation. Do not compress those levels into one `ValidEmail`.

For money, decide:

- signed, non-negative, positive, or bounded amount;
- minor-unit scale;
- currency representation;
- overflow policy;
- same-currency arithmetic;
- rounding and allocation owner;
- foreign-exchange boundary.

`u64` includes zero. `NonZeroU64` excludes zero but does not encode policy maximum or currency.

## Collection decision

Identify whether the invariant is non-empty, bounded, sorted, unique, capacity-limited, or
relational among members. A wrapper is only valid if it controls:

- vector or set creation;
- extension and insertion;
- removal and clearing;
- mutable slice or inner-container access;
- `FromIterator`;
- deserialization and database decoding.

If mutation is broad and invariant checks are cheap, an ordinary collection plus an
operation-level validator may be clearer.

## Authority decision

Use a capability when local possession should enable a narrow operation and forgery must be
hard. Record:

- issuer and protected constructor;
- resource, tenant, operation, and amount scope;
- whether cloning is valid;
- transfer and task ownership;
- serialization policy;
- expiry and revocation;
- use count;
- external recheck;
- secret handling.

If authority changes frequently and every use must query a policy engine, model the runtime
authorization decision explicitly rather than claiming durable authority.

## Boundary decision

For each HTTP DTO, message, row, file, configuration, or FFI value:

1. bound size and resource use;
2. parse into a structural representation;
3. normalize according to explicit policy;
4. validate using canonical constructors;
5. authenticate and authorize separately;
6. preserve unknown versions safely;
7. convert errors without erasing category;
8. retain operation identity for effects;
9. test invalid and historical values;
10. state what remains uncertain.

If a derive constructs private fields directly, it is a bypass unless the derive delegates to
checked conversion.

## Effect and outcome decision

Mark the external commitment point. Then classify outcomes:

| Observation                              | Domain meaning         | Retry posture                        |
| ---------------------------------------- | ---------------------- | ------------------------------------ |
| Request definitely not sent              | local failure          | retry may be safe after policy check |
| Definitive accepted response             | confirmed success      | do not repeat effect                 |
| Definitive rejected response             | confirmed rejection    | retry only if rejection is retriable |
| Sent, response lost or timed out         | unknown                | reconcile or use proven idempotency  |
| Cancellation before commitment confirmed | cancelled/non-executed | retry may be safe                    |
| Cancellation acknowledgement absent      | unknown                | reconcile                            |

Define idempotency key scope, uniqueness, retention, and replay response. If those properties
are unknown, do not label retry safe.

## Choosing a simpler mechanism

Choose ordinary runtime validation instead of a new type when:

- the fact is used once at the boundary;
- it depends on mutable external or cross-entity state;
- the invalid state has low consequence and an immediate structured error;
- a wrapper would expose unrestricted mutation;
- persistence or dynamic inspection erases the type on every operation;
- or the team cannot maintain the abstraction safely.

Choose a consuming method instead of full typestate when only double use matters. Choose an
enum instead of boolean combinations. Choose a capability instead of a broad service object
when authority surface is the concern.

## Stop conditions

Stop adding type machinery when it no longer removes a named consequential invalid program,
when state combinations multiply faster than the domain, when compiler errors cease to express
the mistake, when serialization requires pervasive erasure, or when external reality makes the
proof stale immediately.

Stop simplifying when a public bypass remains, the same invariant is scattered across
callers, wrong-state effects are plausible and severe, or an unknown outcome is still forced
into success/failure.

The final decision includes a guarantee ledger, boundary map, evidence plan, and a trigger for
revisiting the representation.

---

## Source: `doctrines/0001-invalid-states/doctrine.md`

# Normative doctrine

## RUST-DOC-0001-R001 — Inventory invariants before representation

**Statement.** A design MUST identify consequential invariants, their owners,
classifications, trust boundaries, enforcement mechanisms, evidence, failure consequences,
and residual uncertainty before selecting domain representations.

**Intent.** Prevent a favorite mechanism or an initial struct shape from deciding the domain
before contradictory states, authority, temporal facts, and external ambiguity are known.

**Applicability.** New domain models, substantial lifecycle changes, new external effects,
boundary integrations, and repairs caused by invariant failure.

**Allowed exceptions.** Pure mechanical refactoring whose behavior and construction surface
are demonstrably unchanged.

**Review evidence.** An invariant inventory using the foundation format, plus a state and
boundary map appropriate to the risk.

## RUST-DOC-0001-R002 — Represent mutually exclusive state as a sum type

**Statement.** Contradictory field combinations MUST be replaced by an enum or equivalent sum
type when domain states are mutually exclusive and carry state-specific data.

**Intent.** Remove combinations such as `is_paid = true` with no receipt or simultaneous paid
and failed flags from ordinary construction.

**Applicability.** Booleans, nullable fields, option groups, string discriminants, or structs
whose validity depends on exclusive combinations.

**Allowed exceptions.** A foreign persistence or wire DTO may retain its external shape if it
is untrusted and converted into a validated domain enum before use.

**Review evidence.** State table, exhaustive matching, invalid-combination rejection at the
boundary, and persistence evolution policy.

## RUST-DOC-0001-R003 — Protect trusted newtype representation

**Statement.** A trusted validated newtype MUST keep its representation private from callers
that are not authorized to assume or establish its invariant.

**Intent.** Make possession of the type meaningful evidence rather than an advisory wrapper.

**Applicability.** Scalars, identifiers, names, money amounts, tokens, and other values whose
type name asserts validation or authority.

**Allowed exceptions.** Transparent public wrappers whose documented purpose is nominal
distinction only and whose name does not assert validation.

**Review evidence.** Visibility audit covering fields, constructors, macros, derives,
features, tests, and re-exports.

## RUST-DOC-0001-R004 — Enforce the complete documented invariant

**Statement.** Every safe constructor for a trusted type MUST enforce the complete invariant
documented for that type, or require an evidence object that establishes the missing part.

**Intent.** Prevent a strong type name from being backed by one partial check or by different
policies across constructors.

**Applicability.** `new`, `parse`, `FromStr`, `TryFrom`, builders, collection constructors,
verifier transitions, and safe boundary conversions.

**Allowed exceptions.** A constructor may establish a deliberately narrower type whose name
and documentation reflect that evidence level.

**Review evidence.** Constructor matrix, positive and negative tests, policy version where
relevant, and proof-token construction audit.

## RUST-DOC-0001-R005 — Name the evidence accurately

**Statement.** A type, variant, method, or field name MUST NOT imply stronger evidence than its
construction establishes.

**Intent.** Prevent syntax validation from being mistaken for ownership, local transition from
external liveness, persistence acknowledgement from durable business completion, or timeout
from rejection.

**Applicability.** All evidence-carrying types and lifecycle variants.

**Allowed exceptions.** None for public claims. Domain-standard abbreviations MAY be used when
their exact repository meaning is documented.

**Review evidence.** Guarantee ledger linking each name to producer, scope, time, and
non-guarantees.

## RUST-DOC-0001-R006 — Preserve invariants through deserialization

**Statement.** Deserialization MUST NOT write a trusted representation in a way that bypasses
its documented validation.

**Intent.** Treat serialized bytes as untrusted regardless of whether they came from an
internal service or cache.

**Applicability.** Serde, custom formats, caches, files, message payloads, and RPC adapters.

**Allowed exceptions.** An explicitly versioned, cryptographically authenticated internal
format may use a privileged decoder only when its authenticity, invariant version, and bypass
preconditions are reviewed and tested.

**Review evidence.** `try_from` or manual decoding path, malformed and policy-invalid cases,
size limits, and unknown-version behavior.

## RUST-DOC-0001-R007 — Validate database decoding

**Statement.** Database decoding MUST NOT silently forge trusted domain values. Raw rows MUST
be checked against current or explicitly versioned invariants before trusted use.

**Intent.** Account for historical data, migrations, manual repair, schema drift, and writes
from other applications.

**Applicability.** ORM derives, row decoders, repositories, event stores, snapshot loaders, and
migrations.

**Allowed exceptions.** A database-native scalar whose complete invariant is enforced by the
database and whose decoder cannot represent an invalid value MAY map directly, provided that
the equivalence is documented.

**Review evidence.** Raw-row/domain separation, checked conversion, invalid-history test,
constraint inspection, quarantine or repair policy, and migration compatibility.

## RUST-DOC-0001-R008 — Preserve collection invariants after construction

**Statement.** A validated collection wrapper MUST control every mutation and construction
route that could violate non-empty, bounded, sorted, unique, capacity, or member-compatibility
invariants.

**Intent.** Prevent a valid initial wrapper from becoming invalid through unrestricted inner
access, iterator collection, clearing, or replacement.

**Applicability.** Domain collections whose whole-value property carries evidence.

**Allowed exceptions.** Immutable wrappers MAY expose read-only slices, iterators, and
borrowing that cannot violate the invariant.

**Review evidence.** Mutation API audit, boundary conversion tests, empty and overflow tests,
and iterator construction behavior.

## RUST-DOC-0001-R009 — Consume prior state when reuse is invalid

**Statement.** State-transition APIs SHOULD consume the prior state, token, transaction, or
capability when its reuse would violate a lifecycle or authority invariant.

**Intent.** Make local double commit, double use, wrong-order capture, or continued use after
close unavailable through ordinary safe code.

**Applicability.** Single-use tokens, transaction completion, shutdown permits, local protocol
states, and authority consumed by an operation.

**Allowed exceptions.** Runtime state guarded by durable concurrency control, externally
shared state, or transitions requiring retry from the same handle may use mutable/runtime
validation when consuming ownership would make recovery less correct.

**Review evidence.** Transition signatures, clone audit, compile-fail test for significant
reuse, and failure return semantics.

## RUST-DOC-0001-R010 — Use typestate proportionately

**Statement.** Typestate MUST be reserved for locally controlled operation sequencing where
state count, ownership, API shape, diagnostics, persistence, and evolution costs are justified
by the invalid programs prevented.

**Intent.** Avoid state explosion and false claims that compile-time local state describes
external or persisted reality.

**Applicability.** Generic marker states, `PhantomData`, state-specific impl blocks, and
builders that move through compile-time phases.

**Allowed exceptions.** None to the proportionality analysis; a small internal experiment MAY
be used to gather diagnostic and complexity evidence.

**Review evidence.** State graph, local-control argument, runtime-enum comparison, persistence
plan, async failure design, compile diagnostics, and complexity budget.

## RUST-DOC-0001-R011 — Use runtime state for dynamic reality

**Statement.** Dynamic, persisted, heterogeneous, externally determined, runtime-inspected, or
frequently evolving state SHOULD use an enum or explicit runtime state machine.

**Intent.** Preserve honest inspection, serialization, migration, and unknown-value handling
without encoding mutable external facts in static type parameters.

**Applicability.** Database status, UI state, message workflow, external provider lifecycle,
mixed-state collections, and replay.

**Allowed exceptions.** A hybrid design MAY convert a validated runtime state into a local
typestate operation when construction and staleness are controlled.

**Review evidence.** Persistence schema, transition validator, concurrency policy, unknown
variant plan, and hybrid conversion contract if used.

## RUST-DOC-0001-R012 — Represent authority as restricted capability

**Statement.** When possession should authorize an operation, a capability type MUST protect
issuance and expose no broader authority than intended; cloning, transfer, serialization,
expiry, and revocation MUST be specified.

**Intent.** Prevent forgery or accidental amplification of authority.

**Applicability.** Authorization grants, transaction rights, shutdown permits, verifier proof
tokens, secret access, and single-use operations.

**Allowed exceptions.** A centralized runtime authorization check MAY be clearer when
authority is mutable and must be revalidated on every use.

**Review evidence.** Issuer visibility, operation surface, clone/serialize audit, scope fields,
revocation and expiry behavior, and misuse tests.

## RUST-DOC-0001-R013 — Keep external effects fallible

**Statement.** Network, database, filesystem, process, device, and other external effects MUST
remain fallible even when local types prove legal sequencing and input invariants.

**Intent.** Prevent compile-time state from being misrepresented as control over independent
systems or resources.

**Applicability.** Connect, send, close, commit, capture, persist, publish, delete, and similar
operations.

**Allowed exceptions.** A pure in-memory transition with no observable external dependency MAY
be infallible if allocation and panic behavior are outside the API's promised failure model.

**Review evidence.** Structured result types, error categories, cancellation behavior,
resource-failure tests, and stated non-guarantees.

## RUST-DOC-0001-R014 — Do not collapse ambiguous timeout into failure

**Statement.** A timeout, disconnect, cancellation, or acknowledgement loss MUST NOT be
reported as confirmed non-execution when the external effect may have occurred.

**Intent.** Avoid duplicate payments, messages, commits, or provisioning caused by invented
failure evidence.

**Applicability.** Any request that may cross an external commitment point before local
certainty is lost.

**Allowed exceptions.** A protocol can establish non-execution when it specifies and
implements a verifiable pre-commit cancellation or rejection boundary.

**Review evidence.** Protocol commitment analysis, fault injection around send and
acknowledgement, outcome type, and retry decision table.

## RUST-DOC-0001-R015 — Model distributed uncertainty explicitly

**Statement.** When an external outcome can be uncertain, the domain MUST include an explicit
`Unknown`, `Indeterminate`, or reconciliation state carrying enough identity and evidence to
resolve or safely manage the outcome.

**Intent.** Preserve truth during partial failure rather than force every history into success
or failure.

**Applicability.** Payment capture, message acknowledgement, ambiguous commit, remote
provisioning, email submission, and similar distributed effects.

**Allowed exceptions.** None when ambiguity is possible and consequential.

**Review evidence.** Outcome variants, operation and idempotency identity, durable storage,
reconciliation procedure, audit trail, and tests that unknown never becomes confirmed failure
without new evidence.

## RUST-DOC-0001-R016 — Make escape hatches explicit

**Statement.** Every public or privileged construction bypass MUST be visibly named,
documented, scoped, owned, and reviewed; ordinary boundary adapters MUST NOT use it.

**Intent.** Keep migrations, trusted constants, or performance paths from silently becoming
general invariant-forging APIs.

**Applicability.** `unchecked`, raw, privileged, feature-gated, administrative, test, and
migration constructors.

**Allowed exceptions.** Test-only constructors MAY have broader convenience when confined to
non-production builds and incapable of leaking into public APIs.

**Review evidence.** Search inventory, visibility and feature analysis, precondition
documentation, call-site list, and safe-interface tests.

## RUST-DOC-0001-R017 — Scope unsafe constructors narrowly

**Statement.** An unsafe constructor MUST state the complete caller proof obligation and MUST
be no broader than the invariant that safe code cannot verify.

**Intent.** Treat unsafe construction as transferred proof responsibility, not permission to
skip validation.

**Applicability.** Raw pointers, FFI wrappers, unchecked UTF or layout conversion, and
performance-sensitive trusted construction.

**Allowed exceptions.** None to documentation or soundness. Avoid unsafe when a checked safe
constructor is practical.

**Review evidence.** RUST-DOC-0007 review, safety section, encapsulation, invalid-input
analysis, Miri or sanitizer evidence where applicable, and all call sites.

## RUST-DOC-0001-R018 — Prove important prohibited programs

**Statement.** Compile-fail tests SHOULD demonstrate compiler rejection of important direct
construction, wrong-state operations, forged authority, or reuse after consumption.

**Intent.** Bind a type-level claim to executable evidence and detect accidental public API
weakening.

**Applicability.** Public or reusable APIs whose primary benefit is compiler prevention.

**Allowed exceptions.** Runtime-only invariants or unstable diagnostics may use API compile
tests plus other structural evidence when a compile-fail harness would be brittle without
adding meaningful confidence.

**Review evidence.** Minimal UI case, reviewed diagnostic, pinned toolchain, and positive
counterpart test.

## RUST-DOC-0001-R019 — Publish guarantees and non-guarantees

**Statement.** Every major trusted type and state transition MUST document its exact guarantee
beside its non-guarantees, escape hatches, boundary preservation, and residual runtime risk.

**Intent.** Stop local evidence from expanding into claims about external liveness, business
policy, distributed certainty, or universal correctness.

**Applicability.** Public domain types, capabilities, typestate APIs, persisted states, and
case-study designs.

**Allowed exceptions.** Trivial private wrappers MAY rely on a nearby module-level guarantee
ledger if every constructor and use is covered.

**Review evidence.** Completed guarantee ledger traced to code, tests, boundaries, and effect
outcomes.

## RUST-DOC-0001-R020 — Keep cross-entity and temporal facts at runtime

**Statement.** Cross-entity, temporal, policy-dependent, and externally mutable invariants MUST
be revalidated by the owning runtime service or transaction when current truth is required.

**Intent.** Avoid stale types that claim balance, authorization, uniqueness, liveness, or
policy acceptance after the underlying fact may change.

**Applicability.** Account funds, inventory, tenant membership, session revocation, uniqueness,
foreign exchange, and multi-record totals.

**Allowed exceptions.** Immutable snapshots MAY carry historical evidence when the name and
API make the observation time and scope explicit.

**Review evidence.** Owner, transaction or observation boundary, concurrency controls,
staleness policy, failure type, and race tests.

## RUST-DOC-0001-R021 — Model money without false arithmetic guarantees

**Statement.** Monetary types MUST carry currency and enforce the documented amount invariant;
arithmetic MUST check currency compatibility and MUST NOT claim that integer representation
eliminates tax, foreign-exchange, allocation, or rounding policy.

**Intent.** Prevent zero/negative amounts where prohibited, accidental currency mixing, binary
floating-point representation error, and overstatement of what minor units solve.

**Applicability.** Prices, invoices, payments, fees, balances, allocations, and settlement.

**Allowed exceptions.** A domain with exactly one fixed currency MAY bind currency at the
aggregate or module level if accidental mixing is structurally impossible and documented.

**Review evidence.** `u64`/`NonZeroU64` semantics, overflow behavior, same-currency tests,
rounding and allocation policy location, and non-guarantee statement.

## RUST-DOC-0001-R022 — Separate email syntax from ownership

**Statement.** An email-address type MUST document its actual syntax policy; mailbox ownership
or external verification MUST require separate verifier-produced evidence.

**Intent.** Prevent checks such as `contains('@')` from being represented as meaningful
deliverability or ownership proof.

**Applicability.** User contact, authentication, notification, and account-recovery addresses.

**Allowed exceptions.** A raw contact string MAY remain unrefined when the system does not
claim email semantics and safely treats delivery failure.

**Review evidence.** Syntax policy tests, private representation, verifier-only proof path,
expiry or revocation considerations, and deliverability non-guarantee.

---

## Source: `doctrines/0002-error-modeling/doctrine.md`

# Normative doctrine

## RUST-DOC-0002-R001 — Define a failure inventory

**Statement.** APIs with consequential failure MUST identify failure categories, caller
actions, commitment semantics, recipients, and evidence before selecting an error type.

**Intent.** Prevent implementation details or string messages from becoming the accidental
contract.

**Applicability.** Public libraries, service operations, external effects, persistence, and
security-sensitive flows.

**Allowed exceptions.** Trivial private helpers MAY reuse the enclosing operation's inventory.

**Review evidence.** Failure table mapping causes to variants, recovery, retry, logging,
protocol status, and uncertainty.

## RUST-DOC-0002-R002 — Use structured library errors

**Statement.** Library APIs MUST NOT use opaque string errors as their primary public contract
when callers can respond differently to failure categories.

**Intent.** Preserve machine-actionable meaning independently of human wording.

**Applicability.** Reusable crates and module boundaries with multiple operational outcomes.

**Allowed exceptions.** An opaque non-exhaustive error object MAY be used when no stable
category can be promised, provided callers have documented inspection or reporting semantics.

**Review evidence.** Public enum or equivalent typed interface, match examples, and stability
policy.

## RUST-DOC-0002-R003 — Distinguish actionable categories

**Statement.** Validation failure, policy rejection, authorization denial, conflict,
cancellation, timeout, resource exhaustion, local I/O failure, and indeterminate outcome MUST
remain distinguishable when they require different caller or operator action.

**Intent.** Prevent unsafe retry, misleading user messages, and loss of reconciliation.

**Applicability.** Any operation where at least two listed outcomes differ operationally.

**Allowed exceptions.** Categories MAY be coarsened at an outer recipient boundary when the
recipient cannot act differently and observability retains safe internal detail.

**Review evidence.** Outcome-to-action matrix and conversion tests.

## RUST-DOC-0002-R004 — Preserve sources

**Statement.** Error wrapping and conversion SHOULD preserve the originating error through a
source chain when doing so is safe and useful for diagnosis.

**Intent.** Retain causal evidence while adding domain context.

**Applicability.** I/O, parsing, serialization, database, protocol, and dependency errors.

**Allowed exceptions.** Security, privacy, compatibility, or cross-process boundaries MAY
replace the exposed source with a sanitized internal correlation record.

**Review evidence.** `source()` chain tests or report inspection, plus redaction review.

## RUST-DOC-0002-R005 — Add context without erasing category

**Statement.** Application context SHOULD identify the failed operation and relevant
non-sensitive identity without replacing machine-actionable categories with formatted text.

**Intent.** Make diagnosis specific while retaining programmatic action.

**Applicability.** Layered application operations, job processing, and boundary adapters.

**Allowed exceptions.** A terminal application boundary MAY use an opaque report after all
control decisions have been made.

**Review evidence.** Context chain, correlation ID, structured fields, and user-facing
redaction.

## RUST-DOC-0002-R006 — State recoverability

**Statement.** Recoverability MUST be explicit at the decision point; callers MUST NOT infer
that every `Err` leaves state unchanged or reusable.

**Intent.** Account for partial mutation, consumed authority, cancellation, ambiguous commit,
and external side effects.

**Applicability.** Stateful, consuming, transactional, asynchronous, and external operations.

**Allowed exceptions.** Pure functions MAY document the conventional no-side-effect error
contract once at module level.

**Review evidence.** Post-error state contract, returned recovery value or token, and tests.

## RUST-DOC-0002-R007 — Type retry guidance

**Statement.** Retryability MUST NOT be inferred solely from a generic transport class,
status family, or error string. Retry policy MUST account for operation semantics,
idempotency, attempt budget, backoff, and external commitment.

**Intent.** Prevent duplicates, retry storms, and repeated permanent rejection.

**Applicability.** Network, database, broker, and other transient-looking errors.

**Allowed exceptions.** None where the operation can cause a consequential effect.

**Review evidence.** Typed retry decision, idempotency analysis, budget, jitter, and fault
tests.

## RUST-DOC-0002-R008 — Preserve indeterminate outcomes

**Statement.** Error conversion MUST NOT convert an indeterminate external effect into
confirmed rejection or non-execution.

**Intent.** Keep the system's account of reality honest and enable reconciliation.

**Applicability.** Timeout, acknowledgement loss, ambiguous commit, cancellation race, or
connection loss after possible send.

**Allowed exceptions.** A protocol-proven pre-commit failure MAY be classified as
non-execution.

**Review evidence.** Commitment analysis, explicit unknown type, reconciliation identity, and
conversion tests.

## RUST-DOC-0002-R009 — Bound panic to programmer faults

**Statement.** Panics MUST be reserved for violated internal invariants or unrecoverable
programmer errors, not expected external, user, configuration, or data failure.

**Intent.** Keep expected failure in the declared control-flow and cleanup model.

**Applicability.** Production library and application paths.

**Allowed exceptions.** Process startup MAY deliberately abort on invalid required
configuration after producing a clear sanitized diagnostic, when continued operation is
unsafe and no caller can recover.

**Review evidence.** Panic-site inventory, unwind/abort policy, and boundary failure tests.

## RUST-DOC-0002-R010 — Justify `unwrap` and `expect`

**Statement.** `unwrap` and `expect` in production paths MUST have a locally evident invariant
or explicit justification showing why failure is a programmer defect rather than expected
input or environment.

**Intent.** Prevent hidden panic contracts.

**Applicability.** Non-test Rust code.

**Allowed exceptions.** Tests and examples MAY use them when the panic is not the behavior
being taught and failure location remains clear.

**Review evidence.** Search results, invariant comments where not obvious, and negative tests
for external input.

## RUST-DOC-0002-R011 — Preserve security and reconciliation evidence

**Statement.** Error conversion MUST NOT erase security-relevant denial, authentication
failure, operation correlation, provider reference, or reconciliation identity needed for
safe action and audit.

**Intent.** Avoid turning an authorization event or ambiguous effect into an undifferentiated
internal error.

**Applicability.** Security, financial, distributed, and regulated workflows.

**Allowed exceptions.** Details MAY be withheld from an untrusted recipient while retained in
a protected correlated record.

**Review evidence.** Internal/external mapping, audit fields, access control, and redaction.

## RUST-DOC-0002-R012 — Prevent secret disclosure

**Statement.** Error display, debug, source chains, protocol responses, logs, and telemetry
MUST NOT expose secrets or sensitive internal data to unauthorized recipients.

**Intent.** Ensure diagnosis does not create a confidentiality breach.

**Applicability.** Credentials, tokens, personal data, SQL, paths, provider payloads, and
security decisions.

**Allowed exceptions.** Restricted forensic storage MAY retain necessary evidence under
explicit access and retention policy.

**Review evidence.** Recipient map, redaction tests, debug implementations, and sample logs.

## RUST-DOC-0002-R013 — Govern public error compatibility

**Statement.** Public error categories and inspection behavior MUST be treated as API
compatibility surface; evolution MUST account for exhaustive matching, non-exhaustive design,
error codes, and downstream recovery behavior.

**Intent.** Avoid breaking callers or forcing unstable implementation details into permanent
variants.

**Applicability.** Published crates, versioned protocols, and stable internal platform APIs.

**Allowed exceptions.** Private application errors MAY evolve with coordinated callers.

**Review evidence.** Semver analysis, non-exhaustive strategy, code stability, and migration
notes.

## RUST-DOC-0002-R014 — Log once at an ownership boundary

**Statement.** Errors SHOULD be logged by the layer that owns the final handling decision,
rather than at every propagation layer.

**Intent.** Prevent duplicate events, contradictory severity, and noisy alerts.

**Applicability.** Layered services, jobs, and request handlers.

**Allowed exceptions.** A lower layer MAY emit a distinct metric or trace event when it adds
unique timing or state evidence and correlation prevents double counting.

**Review evidence.** Error path trace, log ownership, event IDs, and alert mapping.

---

## Source: `doctrines/0003-ownership-and-capabilities/doctrine.md`

# Normative doctrine

## RUST-DOC-0003-R001 — Map authority and custody

**Statement.** A design MUST identify who owns each resource, who may borrow it, which
operations possession authorizes, how custody transfers, and how authority ends.

**Intent.** Prevent memory ownership from being confused with business permission or lifecycle
completion.

**Applicability.** Resources, tokens, sessions, transactions, locks, permits, secrets, and task
handoffs.

**Allowed exceptions.** Pure immutable data without authority or lifecycle meaning.

**Review evidence.** Authority map, lifecycle diagram, and ownership signatures.

## RUST-DOC-0003-R002 — Encode exclusive authority with ownership

**Statement.** Ownership SHOULD express exclusive authority when only one actor may legally
exercise or complete an operation.

**Intent.** Prevent duplicated commit, shutdown, claim, or single-use token consumption.

**Applicability.** Exclusive domain actions with natural transfer or consumption.

**Allowed exceptions.** Durable external coordination MAY require runtime exclusivity when
multiple processes or persisted actors participate.

**Review evidence.** Non-cloneable type, consuming operation, and concurrency or compile-fail
tests.

## RUST-DOC-0003-R003 — Bound borrowed authority

**Statement.** A borrowed reference MUST NOT accidentally grant mutation, ownership transfer,
serialization, or authority beyond the documented borrow scope.

**Intent.** Keep read access from becoming lasting or privileged access.

**Applicability.** References, guards, views, callbacks, and borrowed service handles.

**Allowed exceptions.** Interior mutability MAY permit mutation when that aliasing contract is
the explicit design and synchronization is correct.

**Review evidence.** Method receiver audit, returned-lifetime analysis, and mutation tests.

## RUST-DOC-0003-R004 — Restrict capability issuance and surface

**Statement.** Capability constructors MUST be restricted to authorized issuers, and a
capability MUST expose only the operations and scope it grants.

**Intent.** Make capabilities hard to forge and consistent with least privilege.

**Applicability.** Authorization, verification proof, shutdown, transaction, secret, and
resource capabilities.

**Allowed exceptions.** None for security-relevant authority.

**Review evidence.** Visibility, fields, re-exports, operation methods, and issuer tests.

## RUST-DOC-0003-R005 — Justify cloning authority

**Statement.** Cloning or copying an authority-bearing value MUST require explicit
justification consistent with exclusivity, use count, scope, and revocation.

**Intent.** Prevent convenience derives from amplifying authority.

**Applicability.** Capabilities, tokens, guards, handles, and credentials.

**Allowed exceptions.** A shareable read capability MAY be cloneable when duplication is part
of the documented authority model.

**Review evidence.** `Clone`/`Copy` audit, clone semantics, and duplicate-use tests.

## RUST-DOC-0003-R006 — Define transfer and revocation

**Statement.** Tokens, sessions, transaction guards, leases, and resource handles MUST define
transfer, expiry, revocation, and post-revocation behavior when those concepts apply.

**Intent.** Prevent local possession from being treated as perpetual external permission.

**Applicability.** Mutable authority, leased resources, sessions, and cross-task custody.

**Allowed exceptions.** Irrevocable process-local values MAY state that revocation is not part
of their contract.

**Review evidence.** State transitions, clocks or versions, revocation check, and stale-use
tests.

## RUST-DOC-0003-R007 — Treat RAII as local cleanup

**Statement.** RAII SHOULD release locally owned resources, but destruction MUST NOT be
described as proving fallible external rollback, commit, compensation, or durable cleanup.

**Intent.** Distinguish deterministic local drop from effects whose failure cannot be returned
by `Drop`.

**Applicability.** Transactions, locks, temporary files, sockets, remote leases, and sessions.

**Allowed exceptions.** Infallible local memory bookkeeping MAY be completed entirely in
`Drop`.

**Review evidence.** Explicit completion methods, drop fallback, error observability, and
failure tests.

## RUST-DOC-0003-R008 — Protect secret-bearing types

**Statement.** Secret-bearing types MUST minimize accidental `Debug`, `Display`, cloning,
serialization, logging, and long-lived borrowing; exposure MUST be explicit and scoped.

**Intent.** Reduce unintended copies and recipient leakage.

**Applicability.** Passwords, tokens, private keys, session secrets, and decrypted material.

**Allowed exceptions.** None for ordinary formatting. Controlled serialization MAY be
required for a protected secret store under a distinct API.

**Review evidence.** Trait implementation audit, redaction tests, exposure call sites, and
storage contract.

## RUST-DOC-0003-R009 — Limit zeroization claims

**Statement.** Zeroization claims MUST state the exact owned buffer cleared and MUST NOT imply
removal of compiler-created copies, allocator remnants, swap, logs, external stores, or prior
serialization unless those paths are controlled and evidenced.

**Intent.** Prevent a local overwrite mechanism from becoming a universal secrecy guarantee.

**Applicability.** Secret memory and cryptographic material.

**Allowed exceptions.** None to claim accuracy.

**Review evidence.** Ownership and copy analysis, drop path, memory-locking policy where used,
and explicit non-guarantees.

## RUST-DOC-0003-R010 — Design before `Arc<Mutex<T>>`

**Statement.** `Arc<Mutex<T>>` MUST NOT be the default substitute for identifying ownership,
task responsibility, mutation protocol, lock scope, and shutdown.

**Intent.** Avoid shared mutable bags that compile but hide contention, deadlock, and authority.

**Applicability.** Concurrent shared state and service handles.

**Allowed exceptions.** It MAY be the simplest correct mechanism after the ownership and
synchronization contract is documented.

**Review evidence.** Owner, lock invariant, contention and poisoning policy, alternatives, and
tests.

## RUST-DOC-0003-R011 — Justify interior mutability

**Statement.** Interior mutability MUST be justified by a required aliasing contract and MUST
preserve the domain's synchronization and authority invariants.

**Intent.** Prevent `Cell`, `RefCell`, locks, or atomics from bypassing a better ownership
design.

**Applicability.** Mutation through shared references.

**Allowed exceptions.** Local caching or instrumentation MAY use it when invisible to domain
semantics and reentrancy is safe.

**Review evidence.** Aliasing rationale, borrow/panic behavior, synchronization, and reentrancy
tests.

## RUST-DOC-0003-R012 — Use lifetimes for real relationships

**Statement.** Lifetime parameters SHOULD express actual borrowing or validity relationships,
not ornamental complexity or an inaccurate claim that an external resource remains valid.

**Intent.** Keep APIs readable and prevent local borrow duration from implying remote
liveness.

**Applicability.** Borrowed views, guards, transactions, callbacks, and FFI.

**Allowed exceptions.** Internal generic abstraction MAY carry a lifetime required by a
dependency, with its relationship documented.

**Review evidence.** Referent and duration explanation, escape analysis, and simpler owned
alternative.

## RUST-DOC-0003-R013 — Define cross-task ownership

**Statement.** Transfer of authority or resources across tasks MUST identify the new owner,
completion signal, cancellation behavior, shutdown responsibility, and behavior if the task
is dropped or panics.

**Intent.** Prevent detached custody and resources with no accountable closer.

**Applicability.** Spawned tasks, worker actors, channels carrying handles, and supervisors.

**Allowed exceptions.** Truly process-lifetime services MAY be owned by the process supervisor.

**Review evidence.** Task tree, join/abort contract, channel closure, and shutdown tests.

## RUST-DOC-0003-R014 — Keep external authority revalidation explicit

**Statement.** A local capability MUST NOT claim current external authority when revocation,
expiry, tenant membership, or resource ownership can change without local control; current
use MUST revalidate or carry a bounded lease.

**Intent.** Prevent stale authorization.

**Applicability.** Sessions, identity-provider grants, distributed locks, and policy decisions.

**Allowed exceptions.** Immutable operation-scoped grants MAY remain valid for their defined
commit window.

**Review evidence.** Lease or recheck boundary, stale-state handling, and revocation race
tests.

---

## Source: `doctrines/0004-concurrency-and-async/doctrine.md`

# Normative doctrine

## RUST-DOC-0004-R001 — Define the concurrency model

**Statement.** Every component with shared mutable state or overlapping work
MUST document its ownership and synchronization model.

**Intent.** Make custody, mutation authority, and coordination visible before
interleavings obscure them.

**Applicability.** Threads, async tasks, callbacks, actors, shared caches,
worker pools, and foreign callbacks.

**Allowed exceptions.** A leaf function using no shared state may rely on the
surrounding component contract.

**Review evidence.** An ownership map identifies state owner, permitted
mutators, synchronization primitive, task owner, and shutdown authority.

## RUST-DOC-0004-R002 — Protect invariants, not fields

**Statement.** Synchronization boundaries MUST cover the complete invariant
they protect. Related fields MUST NOT be independently locked when an operation
requires their values to change atomically as a group.

**Intent.** Prevent logically torn state even when every individual memory
access is race-free.

**Applicability.** Multi-field state, counters paired with collections, status
paired with resources, and cross-index data.

**Allowed exceptions.** Independent locks are permitted when the invariant
inventory proves the fields are independent or a documented reconciliation
protocol tolerates temporary divergence.

**Review evidence.** Invariant-to-lock mapping plus tests or model evidence for
multi-step updates.

## RUST-DOC-0004-R003 — Bound lock scope

**Statement.** Lock scope MUST be minimized to the protected operation and MUST
be documented on latency-sensitive or contention-sensitive paths. A blocking
call or `.await` MUST NOT occur while holding a synchronous lock unless a
specific correctness argument and bounded behavior justify it.

**Intent.** Reduce deadlock, convoying, executor blockage, and accidental
serialization.

**Applicability.** Mutexes, read-write locks, semaphore permits, and
transaction-like guards.

**Allowed exceptions.** A deliberately serialized critical section may include
a bounded non-suspending operation when splitting it would violate an
invariant.

**Review evidence.** Critical-section boundaries, timing assumptions, and
contention measurements where performance matters.

## RUST-DOC-0004-R004 — Review lock order and poisoning

**Statement.** Components that may acquire more than one lock MUST define a
global acquisition order or another deadlock-avoidance protocol. Lock poisoning
behavior MUST be chosen consciously rather than inherited without review.

**Intent.** Make cyclic waits and post-panic state handling explicit.

**Applicability.** Nested locks, callbacks under locks, standard-library
poisoning locks, version-appropriate `std::sync::nonpoison` APIs when available,
and libraries with different poisoning semantics.

**Allowed exceptions.** A proof that locks cannot overlap may replace a global
order.

**Review evidence.** Lock graph, callback analysis, and documented recovery,
fail-stop, or invariant-rebuild policy after panic.

## RUST-DOC-0004-R005 — Isolate blocking work

**Statement.** Potentially blocking filesystem, process, DNS, compression,
cryptographic, database, or CPU-intensive work MUST NOT run on async executor
worker threads without deliberate isolation.

**Intent.** Preserve progress for unrelated tasks and keep runtime scheduling
assumptions honest.

**Applicability.** Async services and libraries running work whose latency is
not cooperatively yielded.

**Allowed exceptions.** Operations proven bounded below the component's
documented scheduling budget may remain inline.

**Review evidence.** Classification of blocking calls, isolation mechanism,
pool capacity, cancellation behavior, and overload limits.

## RUST-DOC-0004-R006 — Analyze cancellation at every suspension point

**Statement.** Every `.await` or equivalent suspension point inside a partial
operation MUST be reviewed for cancellation safety.

**Intent.** Prevent abandoned state mutations, lost data, leaked authority, and
unobserved external effects when a future is dropped.

**Applicability.** Select loops, timeouts, request handlers, retries, and
multi-stage operations.

**Allowed exceptions.** A suspension point may be classified cancellation-safe
when dropping the future cannot lose progress or violate an invariant; the
classification still requires evidence.

**Review evidence.** A cancellation table showing state before suspension,
drop effect, cleanup owner, resumability, and external uncertainty.

## RUST-DOC-0004-R007 — Define cancellation cleanup

**Statement.** Resources and partial state created before a cancellable
suspension MUST have a defined cleanup, commit, compensation, or reconciliation
path.

**Intent.** Ensure that future destruction is part of the protocol instead of
an invisible control-flow edge.

**Applicability.** Locks, permits, temporary files, transactions, leases,
messages, and external requests.

**Allowed exceptions.** Abandonment is permitted only when the resource is
designed to expire or be collected and the delay and capacity consequences are
acceptable.

**Review evidence.** Drop behavior, explicit cleanup calls, expiry bounds,
reconciliation identifiers, and cancellation tests.

## RUST-DOC-0004-R008 — Bound concurrency

**Statement.** Concurrency SHOULD be bounded by a reviewed resource limit.
Unbounded task spawning requires explicit justification and an overload
analysis.

**Intent.** Prevent memory growth, connection exhaustion, scheduler collapse,
and downstream overload.

**Applicability.** Per-request tasks, batch fan-out, consumer loops, and retry
workers.

**Allowed exceptions.** A finite, statically bounded input set may establish a
safe upper bound without a runtime semaphore.

**Review evidence.** Capacity source, queue bound, rejection or waiting policy,
and stress evidence at and above capacity.

## RUST-DOC-0004-R009 — Make backpressure explicit

**Statement.** Producers and consumers MUST define what happens when demand
exceeds service capacity: wait, reject, shed, coalesce, persist, or degrade.

**Intent.** Replace accidental memory buffering with an operational contract.

**Applicability.** Channels, queues, streams, batching, connection pools, and
API ingress.

**Allowed exceptions.** None for an open-ended producer. A fixed small batch
may document its finite bound.

**Review evidence.** Capacity values, overflow behavior, fairness, metrics, and
caller-visible failure semantics.

## RUST-DOC-0004-R010 — Handle channel closure

**Statement.** Send and receive paths MUST handle channel closure as a normal
protocol event. Closure MUST NOT be converted silently into an endless retry,
busy loop, or fabricated success.

**Intent.** Make owner departure, shutdown, and worker failure observable.

**Applicability.** Bounded and unbounded channels, watch streams, broadcast
channels, and actor mailboxes.

**Allowed exceptions.** A process-terminating invariant breach may escalate
closure after recording adequate context.

**Review evidence.** Closure branches, sender/receiver ownership, drain policy,
and tests for last-sender and receiver-drop behavior.

## RUST-DOC-0004-R011 — Structure task ownership

**Statement.** Every spawned task MUST have an owner responsible for observing
completion, failure, cancellation, and shutdown.

**Intent.** Prevent invisible task failure and work that outlives its authority
or dependencies.

**Applicability.** Runtime tasks, threads, worker pools, and background
maintenance loops.

**Allowed exceptions.** Process-lifetime infrastructure may be supervised by a
top-level owner rather than joined by the immediate caller.

**Review evidence.** Task tree, join or supervision strategy, failure
propagation, restart limits, and shutdown trigger.

## RUST-DOC-0004-R012 — Restrict detached tasks

**Statement.** Detached tasks MUST be exceptional, named, observable, bounded,
and documented with their process-lifetime contract.

**Intent.** Avoid fire-and-forget work whose success, failure, or resource use
cannot be accounted for.

**Applicability.** Telemetry flushers, cache refreshers, cleanup work, and
best-effort notifications.

**Allowed exceptions.** A deliberately lossy best-effort action may detach if
loss is acceptable, resource use is bounded, and failures are measured.

**Review evidence.** Owner rationale, task name, metrics, capacity, panic
handling, and termination behavior.

## RUST-DOC-0004-R013 — Define graceful shutdown

**Statement.** Concurrent services MUST define admission stop, cancellation,
queue drain, resource release, deadline, forced termination, and observability
semantics for shutdown.

**Intent.** Turn shutdown into a tested lifecycle transition.

**Applicability.** Services, consumers, worker pools, and long-running tools.

**Allowed exceptions.** Short-lived pure computations may rely on process
completion when they own no persistent or external effect.

**Review evidence.** Ordered shutdown procedure, time budget, outstanding-work
accounting, and tests for idle and loaded shutdown.

## RUST-DOC-0004-R014 — State ordering guarantees precisely

**Statement.** Ordering claims MUST identify their scope, key, producer set,
buffering boundary, and behavior during retry or failover.

**Intent.** Prevent local FIFO behavior from being described as global order.

**Applicability.** Channels, brokers, actor mailboxes, logs, and concurrent
state updates.

**Allowed exceptions.** None when callers depend on order.

**Review evidence.** Ordering contract plus tests that include multiple
producers, retries, and closure where relevant.

## RUST-DOC-0004-R015 — Justify atomic ordering

**Statement.** Every nontrivial atomic operation MUST document why its memory
ordering is sufficient for the associated synchronization invariant.

**Intent.** Prevent atomics from becoming unexplained race-free but incorrect
protocols.

**Applicability.** `Atomic*`, fences, lock-free structures, and unsafe
concurrency.

**Allowed exceptions.** A simple standalone statistics counter may use relaxed
ordering when no other memory is synchronized through it.

**Review evidence.** Happens-before argument, invariant, permitted
interleavings, Loom or equivalent model evidence where tractable, and
RUST-DOC-0007 review if unsafe code is present.

## RUST-DOC-0004-R016 — Preserve failure and ordering through supervision

**Statement.** Task supervision MUST distinguish normal completion,
cancellation, panic, retryable failure, permanent failure, and exhausted
restart policy when those outcomes require different action.

**Intent.** Prevent restart loops and silent partial service.

**Applicability.** Actors, consumers, background workers, and service task
trees.

**Allowed exceptions.** Outcomes may be combined only when no caller or
operator acts differently and diagnostic evidence remains adequate.

**Review evidence.** Supervision decision table, restart budget, backoff,
jitter, terminal-state reporting, and panic policy.

## RUST-DOC-0004-R017 — Review async abstraction costs

**Statement.** Async traits, boxed futures, dynamic dispatch, and generated
state machines MUST be evaluated for allocation, object-safety, API stability,
diagnostic, and monomorphization tradeoffs.

**Intent.** Keep async abstraction proportional to actual polymorphism needs.

**Applicability.** Public traits, plugin boundaries, high-volume paths, and
generic middleware.

**Allowed exceptions.** Local low-volume code may choose the clearest interface
without benchmark evidence when its cost is immaterial.

**Review evidence.** Required dispatch mode, allocation expectations, public
API consequences, and measurements for performance claims.

## RUST-DOC-0004-R018 — Coordinate timeouts and retries

**Statement.** Timeout and retry layers MUST be inventoried end to end.
Independent layers MUST NOT multiply attempts or synchronize retries without a
documented load and idempotency analysis.

**Intent.** Prevent retry storms, thundering herds, duplicated effects, and
latency that exceeds caller budgets.

**Applicability.** Clients, middleware, proxies, services, databases, brokers,
and supervisors.

**Allowed exceptions.** Nested retries may exist when attempt budgets compose
within one deadline and each layer has distinct, proven safe semantics.

**Review evidence.** Attempt equation, total deadline, backoff and jitter,
idempotency classification, downstream capacity, and unknown-outcome handling.

## RUST-DOC-0004-R019 — Separate concurrency safety from external correctness

**Statement.** A race-free local transition MUST NOT be claimed to establish
remote liveness, durable completion, unique execution, or current external
state.

**Intent.** Keep local synchronization evidence distinct from mutable external
reality.

**Applicability.** Network connections, acknowledgements, leases, distributed
locks, and database commits.

**Allowed exceptions.** None.

**Review evidence.** Guarantee ledger identifying local proof, observation
time, runtime failures, timeout ambiguity, and reconciliation path.

## RUST-DOC-0004-R020 — Test adverse schedules and overload

**Statement.** Evidence for a consequential concurrent protocol MUST include
adverse scheduling, closure, cancellation, overload, and shutdown behavior,
using model checking or fault control when ordinary tests cannot reliably
exercise the interleavings.

**Intent.** Test the protocol edges that happy-path scheduling conceals.

**Applicability.** Shared-state protocols, supervisors, queues, and
cancellation-sensitive operations.

**Allowed exceptions.** Trivial immutable parallel computation may document why
these hazards are absent.

**Review evidence.** Invariant-linked tests, stress or model results, failure
injection, and known evidence limits.

---

## Source: `doctrines/0005-persistence-boundaries/doctrine.md`

# Normative doctrine

## RUST-DOC-0005-R001 — Treat persisted data as boundary input

**Statement.** Data read from persistence MUST be treated as an untrusted
representation until it has been decoded and validated against current domain
invariants.

**Intent.** Prevent storage provenance from forging domain evidence.

**Applicability.** Rows, documents, snapshots, cached values, event payloads,
and restored backups.

**Allowed exceptions.** None for a type whose name carries a validated
invariant. Trusted storage infrastructure may reduce threat likelihood but not
remove the construction obligation.

**Review evidence.** A complete read-path inventory and conversions that call
the trusted constructor.

## RUST-DOC-0005-R002 — Separate models when contracts differ

**Statement.** Persistence models and domain models SHOULD be separated when
their nullability, versioning, normalization, compatibility, or invariant
contracts differ.

**Intent.** Prevent storage evolution concerns from weakening the domain model.

**Applicability.** Most durable business entities and versioned records.

**Allowed exceptions.** One representation may serve both roles when field
contracts are demonstrably identical and decoding still preserves invariants.

**Review evidence.** Field mapping, rationale for shared or separate models,
and tests for invalid stored representations.

## RUST-DOC-0005-R003 — Validate trusted newtypes during decoding

**Statement.** Database and serialized decoding MUST construct trusted newtypes
through their validated public path. A driver mapping MUST NOT write private
representation bytes through an unchecked or unsafe path merely to satisfy an
interface.

**Intent.** Preserve one invariant gate across every construction source.

**Applicability.** SQL decoding traits, ORM hooks, Serde adapters, event
deserializers, and cache loaders.

**Allowed exceptions.** A narrowly scoped internal constructor may accept
evidence already validated in the same operation, with the proof documented and
tested.

**Review evidence.** `TryFrom`, parser, or smart-constructor calls and negative
decoding tests.

## RUST-DOC-0005-R004 — Reinforce invariants in the schema

**Statement.** Schema constraints SHOULD reinforce stable value and
cross-column invariants that the database can enforce without duplicating
volatile business policy.

**Intent.** Defend against alternate writers and narrow invalid-data ingress.

**Applicability.** Nullability, ranges, uniqueness, referential integrity,
discriminators, and state-related column combinations.

**Allowed exceptions.** A constraint may remain application-only when the
database cannot express it reliably, enforcement would create unacceptable
coupling, or rollout cannot yet guarantee compatibility.

**Review evidence.** Invariant mapping to domain constructor, schema constraint,
transactional validation, or explicit residual gap.

## RUST-DOC-0005-R005 — Avoid contradictory nullable records

**Statement.** Partial records, boolean flags, and nullable associated fields
MUST NOT represent mutually exclusive domain states without a checked
discriminator and a validation rule that rejects contradictory combinations.

**Intent.** Prevent rows such as "paid without receipt" or "failed with settled
timestamp."

**Applicability.** Lifecycle tables, optional payload columns, and soft-state
flags.

**Allowed exceptions.** A deliberately incomplete staging record may exist in a
separate type and table whose lifecycle never exposes it as the completed
domain entity.

**Review evidence.** Row-state truth table, schema checks where feasible, and
conversion tests for every invalid combination.

## RUST-DOC-0005-R006 — Make migrations invariant-aware

**Statement.** Every migration MUST state which invariants it preserves,
strengthens, weakens, or transforms, and MUST define handling for rows that do
not satisfy the target invariant.

**Intent.** Treat migration as a domain transition rather than only a shape
change.

**Applicability.** Schema, data, index, encoding, and enum migrations.

**Allowed exceptions.** A metadata-only operation may state that domain
invariants are unaffected, with evidence.

**Review evidence.** Precondition query, transformation, postcondition query,
rollback or forward-repair strategy, and representative migration test.

## RUST-DOC-0005-R007 — Version durable representations

**Statement.** Persisted formats that can outlive one release MUST be versioned
or have an explicit compatibility and migration strategy.

**Intent.** Keep old values decodable without silently assigning new meaning.

**Applicability.** JSON blobs, snapshots, event payloads, files, cache entries
that survive deployment, and database schemas.

**Allowed exceptions.** Ephemeral caches may be invalidated atomically when
version changes, if stale values cannot be interpreted.

**Review evidence.** Version field or schema version, supported-reader matrix,
unknown-version behavior, and fixture tests.

## RUST-DOC-0005-R008 — Plan enum evolution

**Statement.** Persistence of enums MUST define storage encoding, unknown or
future value behavior, rename policy, and downgrade compatibility.

**Intent.** Avoid making source-level variant spelling an accidental permanent
wire contract.

**Applicability.** SQL enums, text discriminators, integer tags, and serialized
sum types.

**Allowed exceptions.** A closed, disposable dataset may reject unknown values
and rebuild from canonical input.

**Review evidence.** Stable encoding table, unknown-value path, migration plan,
and old/new reader tests.

## RUST-DOC-0005-R009 — Align transactions with cross-entity invariants

**Statement.** A cross-entity invariant that requires atomic observation and
mutation MUST be enforced within a transaction boundary and isolation mechanism
capable of protecting that invariant, or through an explicit alternative
coordination protocol. The design MUST name the concurrency anomaly being
controlled and the residual anomaly set permitted by the selected mechanism,
database, and configuration.

**Intent.** Prevent application prechecks from racing concurrent writers.

**Applicability.** Balances, uniqueness, inventory, state transitions,
aggregate versions, and paired records.

**Allowed exceptions.** Eventual convergence is permitted when temporary
violation is a documented domain state with bounded detection and repair.

**Review evidence.** Transaction scope, isolation analysis against the package
taxonomy, locking or constraint mechanism, concurrent test, and named residual
anomaly set.

## RUST-DOC-0005-R010 — Prevent lost updates

**Statement.** Read-modify-write operations subject to concurrent writers MUST
use optimistic version checks, locking, commutative updates, or another explicit
lost-update prevention strategy.

**Intent.** Stop later writes from silently erasing changes based on stale
state.

**Applicability.** Mutable entities, counters with derived fields, and
administrative edits.

**Allowed exceptions.** Last-write-wins is allowed only when it is the explicit
business policy and discarded updates are acceptable and observable where
needed.

**Review evidence.** Version predicate or locking query, conflict error,
concurrency test, and caller conflict policy.

## RUST-DOC-0005-R011 — Preserve transaction-handle lifecycle

**Statement.** Transaction APIs SHOULD prevent use after commit or rollback
through consuming methods or an equivalent runtime lifecycle guard. Commit
failure MUST preserve the distinction between confirmed rollback, confirmed
commit, and ambiguous outcome when the driver or protocol permits ambiguity.

**Intent.** Prevent stale transaction reuse and dishonest commit status.

**Applicability.** Database clients, unit-of-work abstractions, and transactional
repositories.

**Allowed exceptions.** A library-owned mutable transaction handle may enforce
the same lifecycle at runtime when consuming APIs are incompatible with the
driver.

**Review evidence.** Handle transition tests, compile-fail evidence where
useful, and connection-loss behavior.

## RUST-DOC-0005-R012 — Do not extend database atomicity to external effects

**Statement.** Database transaction success MUST NOT be claimed to include a
message, payment, file, or network effect outside the transaction's actual
resource boundary.

**Intent.** Prevent fictional atomicity across independent systems.

**Applicability.** State changes coupled to publishing or external calls.

**Allowed exceptions.** A documented distributed transaction mechanism may
state only the boundary and failure model it actually provides.

**Review evidence.** Effect inventory, atomic boundary diagram, failure matrix,
and reconciliation path.

## RUST-DOC-0005-R013 — Coordinate persistence and messaging durably

**Statement.** When a domain transition and message publication must not be
silently separated, the design SHOULD use a transactional outbox, inbox, event
log, or equivalent durable coordination protocol.

**Intent.** Make retry and recovery possible after process or network failure.

**Applicability.** Event publication, job enqueueing, and integration messages.

**Allowed exceptions.** A best-effort notification may remain outside durable
coordination when loss is an accepted, documented outcome.

**Review evidence.** Atomic write, publisher retry, deduplication identity,
retention, ordering scope, and operational lag metrics.

## RUST-DOC-0005-R014 — Quarantine invalid historical data

**Statement.** A stored representation that fails current domain validation
MUST be rejected, quarantined, repaired through an audited migration, or exposed
as an explicit invalid-record type. It MUST NOT be forged into the trusted type.

**Intent.** Preserve the meaning of trusted domain values while allowing
operational recovery.

**Applicability.** Production reads, imports, restores, and migration scans.

**Allowed exceptions.** None for trusted construction.

**Review evidence.** Diagnostic classification, record identity, sensitive-data
handling, repair workflow, and metrics.

## RUST-DOC-0005-R015 — Preserve unknown fields and values deliberately

**Statement.** Readers MUST choose and document whether unknown fields or values
are rejected, ignored, retained, or mapped to an explicit unknown variant.

**Intent.** Make forward compatibility and security posture deliberate.

**Applicability.** Flexible records, events, snapshots, and rolling upgrades.

**Allowed exceptions.** None; the chosen policy may be implicit in a format only
if documented and tested.

**Review evidence.** Compatibility matrix and tests for extra fields, missing
fields, and unknown discriminators.

## RUST-DOC-0005-R016 — Bound stored-input resource use

**Statement.** Decoding durable values MUST enforce appropriate limits on
length, nesting, allocation, decompression, and batch size before constructing
trusted in-memory state.

**Intent.** Prevent validly encoded but hostile or corrupted records from
exhausting resources.

**Applicability.** Blobs, arrays, compressed payloads, large text, and batch
queries.

**Allowed exceptions.** A format with a proven small physical bound may rely on
that bound and document it.

**Review evidence.** Limits, streaming behavior, oversized fixtures, and failure
mapping.

## RUST-DOC-0005-R017 — Record persistence guarantees and non-guarantees

**Statement.** Persistence designs MUST document the exact durability,
consistency, isolation, freshness, and external-effect claims they rely on,
including configuration assumptions.

**Intent.** Prevent product names or successful calls from implying stronger
guarantees than deployed behavior.

**Applicability.** Every durable domain component.

**Allowed exceptions.** None.

**Review evidence.** Guarantee ledger linked to database documentation,
configuration, tests, monitoring, and residual failure modes.

---

## Source: `doctrines/0006-distributed-uncertainty/doctrine.md`

# Normative doctrine

## RUST-DOC-0006-R001 — Do not equate timeout with non-execution

**Statement.** A timeout MUST NOT be represented as confirmed failure when the
remote operation may have executed.

**Intent.** Preserve the distinction between stopping local waiting and learning
remote outcome.

**Applicability.** Network requests, database commit, broker acknowledgement,
filesystem operations over remote mounts, and subprocess protocols.

**Allowed exceptions.** A timeout may be definitive only when protocol evidence
establishes that execution could not have begun or was atomically cancelled.

**Review evidence.** Protocol timeline, cancellation semantics, and explicit
unknown-outcome path.

## RUST-DOC-0006-R002 — Model operationally distinct outcomes

**Statement.** Outcome types MUST distinguish confirmed success, confirmed
rejection, local failure before dispatch, and unknown outcome when callers
require different recovery.

**Intent.** Prevent transport symptoms from erasing domain knowledge.

**Applicability.** Consequential external operations.

**Allowed exceptions.** Categories may combine when no caller action, audit
meaning, security consequence, or reconciliation path differs.

**Review evidence.** Outcome decision table and exhaustive caller handling.

## RUST-DOC-0006-R003 — Carry reconciliation evidence

**Statement.** An unknown outcome MUST carry or reference sufficient evidence
to reconcile it, including stable operation identity and the external target.

**Intent.** Make uncertainty actionable and auditable.

**Applicability.** Payments, messages, provisioning, commits, and any effect that
cannot safely be repeated blindly.

**Allowed exceptions.** An explicitly irreconcilable best-effort action may
retain only audit evidence if business policy accepts permanent uncertainty.

**Review evidence.** Reconciliation token, operation ID, request fingerprint,
target, attempt history, and observation method.

## RUST-DOC-0006-R004 — Analyze before retry

**Statement.** Every retry policy MUST classify the operation as safe to retry,
unsafe to retry, or reconcile-before-retry for each relevant failure point.

**Intent.** Prevent duplicate effects and unsafe assumptions.

**Applicability.** Clients, consumers, publishers, schedulers, and operator
runbooks.

**Allowed exceptions.** Pure reads may use a simpler safe-retry classification
when staleness and load remain documented.

**Review evidence.** Failure-point matrix, idempotency mechanism, deadline, and
attempt budget.

## RUST-DOC-0006-R005 — Define idempotency-key semantics

**Statement.** An idempotency key MUST have defined uniqueness, caller and
resource scope, payload binding, retention, concurrency, replay, and conflict
semantics.

**Intent.** Prevent a string field from being mistaken for idempotent behavior.

**Applicability.** Mutable external APIs and durable commands.

**Allowed exceptions.** Naturally idempotent operations may omit keys when their
semantic identity and repeated-result behavior are established independently.

**Review evidence.** Key contract, storage constraint, same-key/same-payload and
same-key/different-payload tests, and expiry policy.

## RUST-DOC-0006-R006 — Reuse operation identity across attempts

**Statement.** Retries of one logical operation MUST reuse its operation and
idempotency identity. A new identity MUST mean a new requested effect.

**Intent.** Allow receivers and reconcilers to distinguish replay from new
intent.

**Applicability.** External API requests, published commands, and repair tools.

**Allowed exceptions.** A protocol-mandated new transport attempt identifier may
be added, but it MUST remain correlated to the stable logical operation.

**Review evidence.** Identity lifecycle and attempt log.

## RUST-DOC-0006-R007 — Expect duplicate delivery

**Statement.** Consumers in at-least-once systems MUST expect duplicate
delivery and MUST define whether repeated processing is deduplicated,
idempotent, commutative, or safely rejected.

**Intent.** Make acknowledgement loss and redelivery ordinary protocol paths.

**Applicability.** Brokers, job queues, webhook delivery, change feeds, and
replayed logs.

**Allowed exceptions.** A verified at-most-once boundary may accept loss instead
of duplicates, with that loss documented.

**Review evidence.** Duplicate test, stable message identity, and effect-level
handling.

## RUST-DOC-0006-R008 — Persist deduplication durably

**Statement.** Deduplication that protects a durable effect MUST itself use
durable state with atomic relationship to that effect, and MUST define
retention.

**Intent.** Prevent process restart or pruning from reopening duplicate effects.

**Applicability.** Consumer inboxes, payment commands, and webhook handlers.

**Allowed exceptions.** In-memory deduplication may protect only ephemeral
best-effort work whose duplicate cost is accepted.

**Review evidence.** Unique key, transaction boundary, retention calculation,
and replay-after-restart test.

## RUST-DOC-0006-R009 — State ordering scope

**Statement.** Ordering claims MUST identify key or partition, producer set,
consumer concurrency, retry behavior, failover behavior, and observation point.

**Intent.** Prevent partition-local or producer-local order from becoming a
false global guarantee.

**Applicability.** Brokers, streams, event logs, RPC sequencing, and replication.

**Allowed exceptions.** None when business behavior relies on order.

**Review evidence.** Ordering contract and tests for retries, multiple
producers, and failover.

## RUST-DOC-0006-R010 — Qualify exactly-once claims

**Statement.** Any "exactly once" claim MUST identify the precise boundary,
identity, transactional mechanism, failure assumptions, retention, and effects
included. It MUST NOT imply exactly-once behavior beyond that boundary.

**Intent.** Replace a broad slogan with an auditable scoped guarantee.

**Applicability.** Messaging, stream processing, payments, jobs, and APIs.

**Allowed exceptions.** None.

**Review evidence.** Guarantee ledger, protocol documentation, duplicate tests,
and excluded effects.

## RUST-DOC-0006-R011 — Coordinate acknowledgement with effect

**Statement.** A consumer MUST define the order and atomic relationship among
effect execution, durable progress, and acknowledgement.

**Intent.** Make the duplicate-versus-loss tradeoff visible.

**Applicability.** Message and job consumers.

**Allowed exceptions.** Best-effort consumers may acknowledge early only when
loss is accepted and measured.

**Review evidence.** Crash-point matrix and tests before and after each durable
step.

## RUST-DOC-0006-R012 — Treat compensation as a new effect

**Statement.** Sagas and compensating operations MUST NOT be described as
rollback. Each compensation MUST remain fallible, idempotency-analyzed, and
capable of an unknown outcome.

**Intent.** Preserve real-world irreversibility and changed conditions.

**Applicability.** Distributed workflows, reservations, payments, and
provisioning.

**Allowed exceptions.** A local database rollback may be called rollback within
its actual transaction boundary.

**Review evidence.** Forward/compensation pairs, business non-equivalence,
failure handling, and reconciliation.

## RUST-DOC-0006-R013 — Treat observations as time-scoped evidence

**Statement.** External observations MUST record or imply their observation
time and MUST NOT be presented as immutable current truth when the external
state can change.

**Intent.** Prevent stale reads from becoming permanent authority.

**Applicability.** Status queries, authorization, inventory, leases, and
reconciliation.

**Allowed exceptions.** Immutable append-only facts may remain stable when the
source contract establishes immutability.

**Review evidence.** Freshness policy, version or timestamp, cache behavior, and
revalidation trigger.

## RUST-DOC-0006-R014 — Address concurrent execution and split brain

**Statement.** Where multiple workers or coordinators can act on one logical
operation, the design MUST address concurrent execution using ownership,
leases with fencing, compare-and-set state, consensus-backed leadership, or an
effect-level idempotency mechanism. When a lease, expiry, or deadline
contributes to that authority, the design MUST define the clock source, whether
elapsed or wall time is used, accepted clock-skew, process-pause, and
renewal-delay bounds, and behavior when any timing assumption fails.

**Intent.** Prevent stale owners and duplicate coordinators from acting with
equal authority, including after a timing assumption ceases to hold.

**Applicability.** Reconciliation workers, schedulers, failover, distributed
locks, leases, and other time-based authority.

**Allowed exceptions.** Concurrent execution is allowed for commutative,
duplicate-safe operations with evidence.

**Review evidence.** Authority protocol, expiry, fencing token use, clock source
and kind, quantified timing bounds, assumption-failure behavior, and overlap
test.

## RUST-DOC-0006-R015 — Bound retries and reconciliation

**Statement.** Retry and reconciliation loops MUST have bounded concurrency,
attempt or time budgets, backoff where appropriate, terminal escalation, and
observability.

**Intent.** Prevent uncertainty from turning into permanent load or hidden
backlog.

**Applicability.** Retry queues, reconcilers, publishers, and operator repair.

**Allowed exceptions.** A durable obligation may remain pending indefinitely,
but each execution cycle still requires bounded work and visible age.

**Review evidence.** Queue capacity, schedule, age metrics, dead-letter or
manual escalation, and overload test.

## RUST-DOC-0006-R016 — Preserve correlation and causality

**Statement.** Audit trails MUST preserve stable operation identity, attempt
identity, triggering event, parent correlation, request fingerprint, outcome
observations, and reconciliation decisions where these affect accountability.

**Intent.** Reconstruct what was requested, attempted, observed, and resolved.

**Applicability.** Consequential distributed effects.

**Allowed exceptions.** Low-risk telemetry may use aggregated correlation when
individual reconstruction is unnecessary.

**Review evidence.** Event schema, trace propagation, redaction, and end-to-end
incident query.

## RUST-DOC-0006-R017 — Protect sensitive reconciliation data

**Statement.** Reconciliation and audit evidence MUST contain enough identity
to act without unnecessarily storing credentials, secret payloads, or sensitive
personal data.

**Intent.** Avoid turning operational evidence into a second secret database.

**Applicability.** Operation logs, dead-letter records, tracing, and support
tools.

**Allowed exceptions.** Required regulated evidence may be retained with
documented access, encryption, minimization, and deletion policy.

**Review evidence.** Field classification, redaction tests, access policy, and
retention.

## RUST-DOC-0006-R018 — Test failure points, not only final errors

**Statement.** Distributed-effect tests MUST inject loss, delay, duplication,
reordering, concurrent execution, and crash points between durable steps in
proportion to risk.

**Intent.** Exercise ambiguity and replay paths hidden by happy-path mocks.

**Applicability.** Integrations, consumers, publishers, and reconcilers.

**Allowed exceptions.** A low-risk pure read may narrow the matrix and state
why.

**Review evidence.** Fault matrix linked to invariants, test results, and
unexercised assumptions.

## RUST-DOC-0006-R019 — State residual uncertainty

**Statement.** Public and internal contracts MUST state which outcomes can
remain unknown, how long, who owns reconciliation, and what users or operators
may safely do meanwhile.

**Intent.** Make uncertainty an owned lifecycle state rather than an error
message.

**Applicability.** Every consequential effect with ambiguous execution.

**Allowed exceptions.** None.

**Review evidence.** State machine, service-level target, escalation path, and
guarantee ledger.

---

## Source: `doctrines/0007-unsafe-rust/doctrine.md`

# Normative doctrine

## RUST-DOC-0007-R001 — Justify the need for unsafe

**Statement.** Introduction or expansion of unsafe code MUST document the
required capability, safe alternatives considered, and why their cost or
limitations are unacceptable for the stated risk domain.

**Intent.** Prevent unsafe from becoming a convenience escape from design or
borrowing work.

**Applicability.** Every new unsafe block, function, trait implementation, or
FFI boundary.

**Allowed exceptions.** Mechanically generated binding declarations may share
one reviewed justification for a generated unit.

**Review evidence.** Required capability, safe alternatives, explicit scope,
and benchmark evidence when performance justifies the risk.

## RUST-DOC-0007-R002 — State the safety invariant

**Statement.** Every unsafe block MUST be associated with a `SAFETY:` argument
that states the relevant invariant and explains why each unsafe operation's
preconditions hold at that point.

**Intent.** Make transferred proof obligations inspectable beside the code.

**Applicability.** Explicit and compiler-required unsafe operations.

**Allowed exceptions.** Repeated operations inside one tightly bounded block may
share one complete argument when their obligations are identical.

**Review evidence.** The `SAFETY:` comment names the applicable aliasing,
validity, lifetime, alignment, provenance, initialization, concurrency, and
panic considerations.

## RUST-DOC-0007-R003 — Minimize and encapsulate unsafe

**Statement.** Unsafe operations MUST be kept in the smallest practical lexical
and API scope and encapsulated behind a safe abstraction whenever safe callers
can use the capability.

**Intent.** Reduce proof surface and prevent invariant-dependent values from
escaping unchecked.

**Applicability.** Low-level modules, FFI wrappers, containers, and optimized
algorithms.

**Allowed exceptions.** A public unsafe primitive may be appropriate when
callers must supply obligations that cannot be checked.

**Review evidence.** Unsafe inventory, module visibility, private fields, and
safe wrapper tests.

## RUST-DOC-0007-R004 — Make safe APIs sound for every safe caller

**Statement.** A safe public API implemented with unsafe code MUST uphold
memory-safety requirements for all values and call sequences constructible in
safe Rust, including reentrancy, panic, cancellation, and concurrent use allowed
by its traits.

**Intent.** Prevent hidden caller obligations from leaking through a safe
signature.

**Applicability.** All safe wrappers over unsafe internals.

**Allowed exceptions.** None.

**Review evidence.** Adversarial safe-call analysis, invariant ownership,
panic/drop paths, and executable evidence.

## RUST-DOC-0007-R005 — Document unsafe caller obligations

**Statement.** Every public or cross-module `unsafe fn` and unsafe trait MUST
have a `# Safety` section specifying complete caller obligations in testable,
non-circular terms.

**Intent.** Define exactly what the compiler no longer checks for the caller.

**Applicability.** Unsafe functions, methods, traits, and constructors.

**Allowed exceptions.** Private functions used once may state obligations at
the function or call site, but the proof chain MUST remain explicit.

**Review evidence.** Caller obligations name valid ranges, lifetime, ownership,
aliasing, initialization, thread, and provenance constraints as relevant.

## RUST-DOC-0007-R006 — Protect representation validity

**Statement.** Unsafe code MUST preserve Rust validity requirements for every
value that becomes observable as a typed value. It MUST NOT create invalid enum
discriminants, references, booleans, characters, nonzero values, or other
restricted representations.

**Intent.** Avoid undefined behavior before ordinary code can validate.

**Applicability.** Casts, reads, transmutation, FFI, serialization shortcuts,
and uninitialized memory.

**Allowed exceptions.** Bytes may remain untyped storage until validity is
established; they MUST NOT be observed through an invalid typed value.

**Review evidence.** Representation source, validation, layout reference, and
invalid-input tests.

## RUST-DOC-0007-R007 — Prove aliasing and lifetime

**Statement.** Creation or use of references from raw pointers MUST establish
non-nullness, alignment, dereferenceability, initialization, permitted aliasing,
and a lifetime no longer than the backing allocation and authority.

**Intent.** Prevent references from asserting guarantees the pointer does not
provide.

**Applicability.** Raw-pointer dereference, slices from raw parts, FFI pointers,
and self-referential structures.

**Allowed exceptions.** None; only the proof mechanism varies.

**Review evidence.** Allocation owner, mutation paths, reallocation analysis,
and borrow duration.

## RUST-DOC-0007-R008 — Respect provenance and bounds

**Statement.** Raw-pointer arithmetic and integer-pointer conversions MUST have
a documented provenance, allocation, element-bound, alignment, and one-past-end
argument consistent with the supported Rust model and target APIs.

**Intent.** Prevent address arithmetic from being treated as sufficient pointer
authority.

**Applicability.** Allocators, buffers, intrusive structures, memory maps, and
FFI.

**Allowed exceptions.** None.

**Review evidence.** Originating allocation, range proof, zero-sized-type
behavior, overflow handling, and Miri coverage where supported.

## RUST-DOC-0007-R009 — Handle partial initialization and drop

**Statement.** `MaybeUninit` and manual initialization MUST track exactly which
elements are initialized and MUST drop each initialized value exactly once on
success, error, and panic paths.

**Intent.** Prevent reads of uninitialized memory, leaks of owned resources, and
double drop.

**Applicability.** Arrays, FFI output buffers, custom collections, and
performance-sensitive construction.

**Allowed exceptions.** Trivially non-dropping byte storage still requires proof
against uninitialized typed reads.

**Review evidence.** Initialization counter or state, guard behavior, panic
injection, and destructor tests.

## RUST-DOC-0007-R010 — Require exceptional justification for transmute

**Statement.** `transmute` MUST require stronger justification than convenience:
source and destination size, alignment, validity, lifetime, ownership, and
layout compatibility MUST be established from authoritative contracts.

**Intent.** Expose the many simultaneous obligations hidden by one operation.

**Applicability.** Every transmute or equivalent bit reinterpretation.

**Allowed exceptions.** None; a narrower cast or conversion SHOULD be used when
it expresses fewer obligations.

**Review evidence.** Primary layout citation, static assertions where possible,
and tests across supported targets.

## RUST-DOC-0007-R011 — Define FFI representation and ABI

**Statement.** FFI declarations MUST specify the correct ABI and use
representations whose layout is defined for that boundary. Rust-native layout
MUST NOT be assumed stable without an applicable representation contract.

**Intent.** Prevent caller/callee disagreement about call convention and data
layout.

**Applicability.** Foreign functions, callbacks, shared structs, unions, and
opaque handles.

**Allowed exceptions.** Bindings generated from an authoritative interface may
derive declarations, but generated output and generator version remain reviewed
inputs.

**Review evidence.** Header/specification match, `repr` choice, target matrix,
and ABI tests.

## RUST-DOC-0007-R012 — Define FFI ownership and allocation

**Statement.** Every pointer crossing FFI MUST define nullability, length,
ownership transfer, lifetime, mutability, thread access, allocator of origin,
and the matching release operation.

**Intent.** Prevent double frees, leaks, allocator mismatch, and dangling
access.

**Applicability.** Buffers, strings, handles, callbacks, and allocated objects.

**Allowed exceptions.** None; an opaque handle still requires a lifecycle
contract.

**Review evidence.** Boundary table, constructor/destructor pairs, null and
length tests, and foreign-side documentation.

## RUST-DOC-0007-R013 — Control unwinding across FFI

**Statement.** Panic or foreign exception unwinding across an ABI boundary MUST
be prevented or handled according to an explicitly selected ABI and supported
runtime contract.

**Intent.** Avoid undefined behavior and uncontrolled process state.

**Applicability.** Exported Rust functions, imported callbacks, and foreign
exceptions.

**Allowed exceptions.** An unwind-capable ABI may be used only with documented
cross-language behavior and target support.

**Review evidence.** Catch/abort policy, destructor implications, and panic-path
test.

## RUST-DOC-0007-R014 — Prove unsafe `Send` and `Sync`

**Statement.** Every unsafe implementation of `Send` or `Sync` MUST state a
concurrency proof covering all contained state, aliasing, mutation,
destruction, callbacks, and foreign-library thread guarantees.

**Intent.** Ensure marker traits do not grant unsupported cross-thread
authority.

**Applicability.** Custom containers, raw handles, FFI wrappers, and
self-referential values.

**Allowed exceptions.** None.

**Review evidence.** Trait invariant, synchronization model, adverse schedule
tests, and upstream thread-safety contract.

## RUST-DOC-0007-R015 — Preserve panic safety

**Statement.** Unsafe abstractions MUST remain memory-safe if safe callbacks,
allocation, cloning, comparison, formatting, or destruction panic at any
permitted point.

**Intent.** Prevent partial mutation from violating assumptions later consumed
by unsafe code.

**Applicability.** Collections, sorting, initialization, callback-based APIs,
and guards.

**Allowed exceptions.** Logical corruption after panic may be allowed only if
memory safety remains intact and the object cannot be used as though valid.

**Review evidence.** Unwind-state analysis, guards, injected panics, and drop
accounting.

## RUST-DOC-0007-R016 — Use complementary dynamic evidence

**Statement.** Unsafe code SHOULD be exercised with Miri and relevant
sanitizers, fuzzing, model checking, or target-specific integration tests where
the tools support its behavior.

**Intent.** Detect violations that code review and ordinary tests can miss.

**Applicability.** Pointer, initialization, FFI, and concurrency code.

**Allowed exceptions.** Unsupported operations or targets may use alternative
evidence, with the limitation documented.

**Review evidence.** Exact commands, supported targets, findings resolved, and
known blind spots.

## RUST-DOC-0007-R017 — Review unsafe dependencies

**Statement.** Dependencies containing material unsafe code MUST be identified
and reviewed proportionally to reachability, privilege, input exposure,
maintenance, advisories, and substitutability.

**Intent.** Include transitive proof trust in the system risk model.

**Applicability.** FFI bindings, parsers, runtimes, allocators, cryptography, and
highly privileged libraries.

**Allowed exceptions.** Low-risk unreachable target-specific code may receive a
documented reduced review.

**Review evidence.** Dependency inventory, versions, advisory status, unsafe
surface, upstream audit evidence, and update policy.

## RUST-DOC-0007-R018 — Re-audit when assumptions change

**Statement.** Unsafe code MUST be re-reviewed when compiler behavior, target,
ABI, dependency, layout, allocation, synchronization, or surrounding safe API
assumptions change.

**Intent.** Keep proof obligations synchronized with their premises.

**Applicability.** Upgrades, ports, refactors, and feature changes.

**Allowed exceptions.** A change proven outside the unsafe dependency cone may
document that conclusion.

**Review evidence.** Assumption inventory, changed-premise analysis, repeated
dynamic evidence, and reviewer approval.

---

## Source: `doctrines/0008-testing-and-evidence/doctrine.md`

# Normative doctrine

## RUST-DOC-0008-R001 — Trace tests to invariants and risks

**Statement.** Tests MUST identify the invariant, contract, failure mode, or
regression risk they support.

**Intent.** Make suites evidence-oriented rather than collections of incidental
examples.

**Applicability.** All canonical tests and verification jobs.

**Allowed exceptions.** A compact regression test may reference an issue,
incident, or neighboring test module rather than repeat the full invariant.

**Review evidence.** Names, documentation, or manifest mapping from claim to
test.

## RUST-DOC-0008-R002 — Test constructor acceptance and rejection

**Statement.** Validated constructors MUST have positive and negative tests at
meaningful boundaries, including normalization and error categories.

**Intent.** Demonstrate both admitted and excluded value sets.

**Applicability.** Parsers, smart constructors, newtypes, collections, and
configuration.

**Allowed exceptions.** A constructor delegated entirely to a separately tested
primitive may cite that evidence and test its integration.

**Review evidence.** Boundary-value table and assertions on structured errors.

## RUST-DOC-0008-R003 — Use properties for generative invariants

**Statement.** Property-based tests SHOULD cover algebraic, round-trip,
ordering, normalization, parser, and collection invariants when a small list of
examples leaves substantial input space.

**Intent.** Explore classes of inputs and produce minimized counterexamples.

**Applicability.** Serialization, arithmetic, state-machine commands, parsers,
and collection operations.

**Allowed exceptions.** Exhaustive finite domains or directly proven simple
functions may use table tests.

**Review evidence.** Generator domain, shrinking behavior, seed retention, and
property statement.

## RUST-DOC-0008-R004 — Prove prohibited programs where valuable

**Statement.** Compile-fail tests SHOULD preserve important API prohibitions
whose guarantee depends on privacy, ownership, traits, or typestate.

**Intent.** Detect accidental widening of legal programs.

**Applicability.** Trusted construction, capability forgery, consumed handles,
state-specific operations, and trait bounds.

**Allowed exceptions.** Fragile diagnostics may be avoided when a stable API
surface check or compile test provides clearer evidence.

**Review evidence.** Minimal failing programs and reviewed compiler diagnostics.

## RUST-DOC-0008-R005 — Inspect compiler-diagnostic changes

**Statement.** Committed compile-fail `.stderr` or equivalent evidence MUST NOT
be rewritten mechanically without reviewing whether the prohibited program
still fails for the intended reason.

**Intent.** Prevent snapshot acceptance from hiding weakened construction or
transition rules.

**Applicability.** UI test suites implemented with `trybuild` or equivalent harnesses.

**Allowed exceptions.** Pure path, line, or diagnostic wording changes may be
accepted after semantic inspection.

**Review evidence.** Diff review and assertion that the intended error remains.

## RUST-DOC-0008-R006 — Cross real boundaries

**Statement.** Integration tests SHOULD cross the real parser, protocol,
database, filesystem, or process boundary when practical and consequential.

**Intent.** Exercise adapters and assumptions that unit tests omit.

**Applicability.** Boundary conversions and external integrations.

**Allowed exceptions.** Unavailable or costly systems may use faithful
emulators plus scheduled real-system evidence, with gaps documented.

**Review evidence.** Environment description, real components, setup isolation,
and cleanup.

## RUST-DOC-0008-R007 — Protect protocol contracts

**Statement.** Contract tests SHOULD verify request and response schemas,
semantic categories, compatibility, idempotency, versioning, and unknown-value
behavior relied on across independently deployed components.

**Intent.** Detect integration drift before deployment.

**Applicability.** HTTP/RPC, messages, FFI, durable events, and public libraries.

**Allowed exceptions.** One jointly released private component may rely on
end-to-end integration evidence when independent compatibility is irrelevant.

**Review evidence.** Provider/consumer contract, version matrix, and failure
fixtures.

## RUST-DOC-0008-R008 — Control concurrency evidence

**Statement.** Concurrency tests MUST use explicit synchronization, schedule
control, model checking, or observable events rather than sleeps as the primary
means of establishing an interleaving.

**Intent.** Avoid flaky timing guesses and unexercised schedules.

**Applicability.** Locks, channels, atomics, cancellation, and shutdown.

**Allowed exceptions.** A sleep may enforce an outer deadline but MUST NOT be
the evidence that an ordering occurred.

**Review evidence.** Barriers, controlled clock, Loom model, event trace, or
equivalent mechanism.

## RUST-DOC-0008-R009 — Test cancellation and cleanup

**Statement.** Async and concurrent operations MUST test cancellation at
consequential suspension points and verify resource, partial-state, and
external-outcome handling.

**Intent.** Exercise future-drop control flow.

**Applicability.** Partial writes, permits, transactions, external calls, and
task supervision.

**Allowed exceptions.** Pure cancellation-safe reads may share representative
evidence when the reasoning applies identically.

**Review evidence.** Controlled cancellation and postcondition assertions.

## RUST-DOC-0008-R010 — Inject partial failure

**Statement.** Fault-injection tests SHOULD exercise failures before, during,
and after durable or external steps in proportion to consequence.

**Intent.** Verify recovery rather than only returned errors.

**Applicability.** Persistence, messaging, payments, filesystems, and
multi-stage operations.

**Allowed exceptions.** Low-risk pure transformations may not need fault
injection.

**Review evidence.** Crash-point matrix, injected faults, resulting state, and
recovery.

## RUST-DOC-0008-R011 — Exercise distributed uncertainty

**Statement.** Distributed tests MUST exercise duplicate, delay, reordering,
lost acknowledgement, retry, and unknown outcomes when the production protocol
permits them.

**Intent.** Prevent perfect-network doubles from defining false behavior.

**Applicability.** Brokers, remote APIs, reconcilers, and distributed workflows.

**Allowed exceptions.** A protocol may exclude a scenario only with
authoritative evidence.

**Review evidence.** Scenario matrix and explicit terminal or unknown states.

## RUST-DOC-0008-R012 — Preserve failure modes in test doubles

**Statement.** Test doubles MUST NOT erase failure categories, cancellation,
latency, capacity, ordering, duplicate, or uncertainty behavior that is
material to the tested claim.

**Intent.** Keep tests faithful to the risk being evaluated.

**Applicability.** Mocks, fakes, emulators, in-memory repositories, and clocks.

**Allowed exceptions.** A narrow unit test may use a simpler double when the
omitted behavior is outside its claim and covered elsewhere.

**Review evidence.** Double-to-real contract comparison and gap ownership.

## RUST-DOC-0008-R013 — Review snapshots semantically

**Statement.** Snapshot changes MUST be reviewed as semantic output changes.
Bulk acceptance MUST NOT replace explanation of why each affected behavior is
correct.

**Intent.** Prevent expected-output updates from blessing regressions.

**Applicability.** Serialized output, diagnostics, UI, plans, and compiler UI
tests.

**Allowed exceptions.** Deterministic formatting-only migrations may group
equivalent changes with one documented rationale.

**Review evidence.** Focused diff, invariant impact, and reviewer sign-off.

## RUST-DOC-0008-R014 — Treat flakiness as evidence

**Statement.** A flaky test MUST be investigated as evidence of uncontrolled
time, state, environment, scheduling, isolation, or product behavior. Retries
MUST NOT be the sole resolution.

**Intent.** Prevent nondeterminism from being normalized.

**Applicability.** All test and benchmark automation.

**Allowed exceptions.** A temporary bounded retry may gather diagnostics while
the issue is owned and visible.

**Review evidence.** Failure signatures, root cause, deterministic fix, or
time-bounded quarantine with owner.

## RUST-DOC-0008-R015 — Do not substitute coverage for invariant evidence

**Statement.** Coverage percentages MUST NOT be used as the sole claim that
behavior or invariants are adequately tested.

**Intent.** Distinguish executed lines from asserted semantics and input space.

**Applicability.** Coverage gates and quality reports.

**Allowed exceptions.** Coverage may serve as a supplemental regression and gap
discovery metric.

**Review evidence.** Invariant-to-evidence matrix in addition to coverage.

## RUST-DOC-0008-R016 — Separate benchmarks from correctness

**Statement.** Benchmarks MUST NOT substitute for correctness tests, and
correctness assertions inside benchmark setup MUST remain independently
executable where feasible.

**Intent.** Prevent performance samples from becoming weak semantic evidence.

**Applicability.** Microbenchmarks, load tests, and profiling harnesses.

**Allowed exceptions.** A benchmark may validate setup defensively, but the
invariant still needs appropriate tests.

**Review evidence.** Corresponding correctness suite and benchmark methodology.

## RUST-DOC-0008-R017 — Use model checking proportionally

**Statement.** Small consequential concurrent protocols SHOULD be considered
for Loom or equivalent model checking, with the model's abstraction and bounds
documented.

**Intent.** Explore scheduler interleavings ordinary runs rarely reach.

**Applicability.** Atomics, locks, channels, once initialization, and ownership
handoff.

**Allowed exceptions.** Unsupported primitives or state explosion may use a
simplified model plus stress and reasoning.

**Review evidence.** Modeled invariant, bounds, results, and mismatch from
production code.

## RUST-DOC-0008-R018 — Exercise unsafe code with specialized tools

**Statement.** Unsafe code SHOULD run under Miri and relevant sanitizers,
fuzzing, or target-specific tests as required by RUST-DOC-0007.

**Intent.** Add dynamic evidence for memory-model and boundary violations.

**Applicability.** Unsafe internals and FFI wrappers.

**Allowed exceptions.** Tool incompatibility must be documented with
alternative evidence.

**Review evidence.** Commands, results, supported targets, and blind spots.

## RUST-DOC-0008-R019 — Use production evidence carefully

**Statement.** Production telemetry and incidents SHOULD refine tests and risk
models, but MUST NOT be treated as proof that unobserved failures cannot occur.

**Intent.** Learn from real workloads without confusing absence of observation
with absence of defects.

**Applicability.** Operational services and libraries with field data.

**Allowed exceptions.** None for universal claims.

**Review evidence.** Telemetry coverage, detection limits, incident-derived
regressions, and residual uncertainty.

## RUST-DOC-0008-R020 — Keep tests deterministic and isolated

**Statement.** Tests MUST control or uniquely scope mutable external state,
clocks, randomness, ports, files, and environment variables required for their
claim.

**Intent.** Make failures reproducible and parallel execution safe.

**Applicability.** Workspace tests and CI.

**Allowed exceptions.** Deliberate randomized or stress tests may vary inputs
but MUST record reproducible seeds and isolate effects.

**Review evidence.** Temporary resource strategy, seed capture, controlled
clock, and parallel-run results.

## RUST-DOC-0008-R021 — State evidence limits

**Statement.** Every consequential evidence plan MUST state what each selected
test class proves, what it does not prove, and which risks remain observed only
in production or external systems.

**Intent.** Preserve guarantee honesty.

**Applicability.** Feature plans, reviews, and release audits.

**Allowed exceptions.** Trivial local changes may reference an existing suite
contract.

**Review evidence.** Evidence ledger tied to invariant inventory.

---

## Source: `doctrines/0009-performance-and-measurement/doctrine.md`

# Normative doctrine

## RUST-DOC-0009-R001 — Define objective and workload

**Statement.** Optimization MUST begin with a quantified objective and a
workload representing the input distribution, concurrency, and system boundary
that matter.

**Intent.** Prevent work on irrelevant micro-costs.

**Applicability.** Performance changes, capacity plans, and regression gates.

**Allowed exceptions.** Removing an obviously unnecessary operation may proceed
as ordinary cleanup if no performance claim is made.

**Review evidence.** Metric, target, baseline, workload, and correctness
constraints.

## RUST-DOC-0009-R002 — Scope every performance claim

**Statement.** Performance claims MUST include environment, toolchain, build
profile, input distribution, concurrency, warmup/cache state, measurement
method, and comparison baseline sufficient for reproduction.

**Intent.** Make numbers interpretable and falsifiable.

**Applicability.** Documentation, pull requests, releases, and design decisions.

**Allowed exceptions.** A local exploratory note may be labeled preliminary and
must not support a merge claim.

**Review evidence.** Reproducible command, environment manifest, raw or
summarized samples, and commit identities.

## RUST-DOC-0009-R003 — Profile before optimizing

**Statement.** Profiling SHOULD precede nontrivial optimization and MUST precede
claims about a dominant bottleneck.

**Intent.** Direct effort to measured cost centers.

**Applicability.** Latency, CPU, allocation, contention, I/O, and size work.

**Allowed exceptions.** Algorithmic complexity defects apparent from complete
input bounds may be corrected without a profile, while still measuring outcome.

**Review evidence.** Flamegraph, trace, allocation profile, system metrics, or
equivalent relevant evidence.

## RUST-DOC-0009-R004 — Preserve correctness independently

**Statement.** A performance change MUST preserve domain invariants,
error/uncertainty semantics, security properties, and boundary validation, with
correctness evidence independent of the benchmark.

**Intent.** Reject faster incorrect behavior.

**Applicability.** All optimizations.

**Allowed exceptions.** An explicit product tradeoff may change semantics only
as a separately reviewed normative or API change, not as hidden optimization.

**Review evidence.** Invariant-linked tests and guarantee-ledger diff.

## RUST-DOC-0009-R005 — Defend benchmark execution

**Statement.** Benchmark code MUST prevent dead-code elimination, constant
folding, unintended setup measurement, and unrealistic reuse from invalidating
the intended workload.

**Intent.** Ensure measured work corresponds to the claim.

**Applicability.** Microbenchmarks and component benchmarks.

**Allowed exceptions.** None; framework facilities may provide the mechanism.

**Review evidence.** Input generation, black-boxing where appropriate,
setup/measurement separation, and result consumption.

## RUST-DOC-0009-R006 — Separate wall-clock and CPU claims

**Statement.** Measurements MUST distinguish wall-clock latency, CPU time, and
aggregate CPU consumption when their interpretations differ.

**Intent.** Prevent waiting and parallel work from being described as reduced
compute cost.

**Applicability.** Async, parallel, I/O-bound, and multi-process workloads.

**Allowed exceptions.** A single-threaded CPU-bound benchmark may report one
measure with its assumption stated.

**Review evidence.** Metric definition and collection method.

## RUST-DOC-0009-R007 — Report distributions

**Statement.** User-visible or service latency claims MUST report appropriate
distributions such as p50, p95, and p99 rather than only arithmetic averages.

**Intent.** Reveal tail behavior and multimodal workloads.

**Applicability.** Requests, queues, storage, and batch completion.

**Allowed exceptions.** Deterministic fixed-cost operations may use a narrow
summary after showing low variance.

**Review evidence.** Sample count, percentile method, confidence or variability,
and outlier policy.

## RUST-DOC-0009-R008 — Document warmup and cache state

**Statement.** Measurements MUST state process warmup, JIT or runtime
initialization where applicable, filesystem/page/cache state, connection reuse,
and dataset residency relevant to the claim.

**Intent.** Prevent cold and warm behavior from being mixed invisibly.

**Applicability.** Storage, network, serialization, and repeated services.

**Allowed exceptions.** A test may deliberately mix states only if the workload
distribution matches production and is documented.

**Review evidence.** Preparation sequence and separate cold/warm results where
both matter.

## RUST-DOC-0009-R009 — Measure allocation claims

**Statement.** Claims that code allocates less, performs no allocation, or
reduces memory MUST be supported by an allocator-aware measurement and MUST
identify retained as well as peak memory where relevant.

**Intent.** Avoid inferring allocation from syntax or clone count.

**Applicability.** Buffering, parsing, collections, async boxing, and caching.

**Allowed exceptions.** A direct removal of the only allocation call may be
noted structurally, but broader runtime claims still require measurement.

**Review evidence.** Allocation count/bytes, allocator, peak/resident set, and
workload.

## RUST-DOC-0009-R010 — Scope zero-copy claims

**Statement.** A zero-copy claim MUST identify every copy avoided within the
specified path and the lifetime, pinning, retention, fragmentation, API, and
ownership costs introduced.

**Intent.** Prevent one avoided copy from becoming a broad slogan.

**Applicability.** Parsers, networking, serialization, buffers, and FFI.

**Allowed exceptions.** None for the phrase "zero-copy."

**Review evidence.** Data-flow diagram, measured copy/allocation evidence, and
non-guarantees.

## RUST-DOC-0009-R011 — Do not equate async with speedup

**Statement.** Async concurrency MUST NOT be described as parallel CPU speedup
without evidence of parallel execution and a workload that benefits.

**Intent.** Distinguish overlap of waiting from reduced compute time.

**Applicability.** Runtime migrations, fan-out, and worker design.

**Allowed exceptions.** None for the claim; async may still improve resource
efficiency or concurrent latency.

**Review evidence.** Executor configuration, CPU utilization, throughput,
latency, and contention.

## RUST-DOC-0009-R012 — Make throughput/latency tradeoffs explicit

**Statement.** Batching, buffering, pipelining, and concurrency changes MUST
report both throughput and relevant latency/queue consequences.

**Intent.** Prevent aggregate gains from hiding worse tails or freshness.

**Applicability.** Brokers, databases, serializers, and service queues.

**Allowed exceptions.** Offline throughput-only jobs may state that latency has
no objective while still bounding resource use.

**Review evidence.** Batch/concurrency sweep and distribution results.

## RUST-DOC-0009-R013 — Measure contention and backpressure

**Statement.** Concurrent performance analysis MUST include queue depth, wait
time, saturation, lock or permit contention, rejection, and downstream load
where relevant.

**Intent.** Reveal whether local throughput shifts cost elsewhere.

**Applicability.** Shared state, pools, channels, and fan-out.

**Allowed exceptions.** Pure independent parallel work may document absence of
shared contention.

**Review evidence.** Contention profile, load curve, and overload behavior.

## RUST-DOC-0009-R014 — Count boundary costs

**Statement.** Performance investigations MUST consider serialization,
allocation, copies, syscalls, context switches, database queries, network
round-trips, and external rate limits before attributing cost solely to Rust
source constructs.

**Intent.** Optimize the actual end-to-end path.

**Applicability.** Integrated and service workloads.

**Allowed exceptions.** A deliberately isolated microbenchmark may narrow scope
and state that it excludes boundary cost.

**Review evidence.** Trace or component budget.

## RUST-DOC-0009-R015 — Review clone removal architecturally

**Statement.** Avoiding `clone` MUST NOT introduce worse algorithmic complexity,
excessive borrowing, global sharing, lock contention, or retention without
measurement and ownership analysis.

**Intent.** Prevent syntax-focused optimization from degrading architecture.

**Applicability.** Buffers, collections, async tasks, and shared state.

**Allowed exceptions.** Removal of a proven redundant clone with unchanged
ownership shape may be a local cleanup.

**Review evidence.** Data ownership, allocation profile, complexity, and
contention.

## RUST-DOC-0009-R016 — Govern unsafe optimization

**Statement.** Unsafe performance changes MUST satisfy RUST-DOC-0007 and MUST
show a material measured benefit under the target workload.

**Intent.** Charge proof risk to the benefit it buys.

**Applicability.** Unchecked indexing, custom allocation, SIMD, FFI, and
lock-free code.

**Allowed exceptions.** Unsafe may be necessary for an external API even when
performance is not its justification; that case is not an optimization claim.

**Review evidence.** Safe baseline, benchmark, profile, safety proof, and
specialized tests.

## RUST-DOC-0009-R017 — Automate stable regressions

**Statement.** Regression thresholds SHOULD be automated only for metrics whose
environmental variance is measured and whose threshold includes a justified
noise budget.

**Intent.** Catch real regressions without normalizing noisy gates.

**Applicability.** CI benchmarks, binary-size checks, allocations, and compile
time.

**Allowed exceptions.** Noisy metrics may run as trend reports or on controlled
dedicated hosts.

**Review evidence.** Baseline history, variance, threshold, hardware stability,
and rerun policy.

## RUST-DOC-0009-R018 — Do not generalize microbenchmarks

**Statement.** Microbenchmark results MUST NOT be generalized to end-to-end
performance without evidence connecting the measured operation to overall
workload contribution.

**Intent.** Prevent large local ratios from masking tiny system impact.

**Applicability.** Library and application optimization claims.

**Allowed exceptions.** A microbenchmark may establish the cost of the exact
isolated primitive it measures.

**Review evidence.** Profile share, integrated benchmark, or component budget.

## RUST-DOC-0009-R019 — Account for build and binary cost

**Statement.** Abstraction choices involving generics, code generation, feature
sets, or dependencies SHOULD assess compile time, monomorphization, binary size,
incremental behavior, and diagnostic cost when material.

**Intent.** Treat developer and deployment resources as performance dimensions.

**Applicability.** Public generic APIs, macro-heavy code, and constrained
artifacts.

**Allowed exceptions.** Small local code with immaterial measured impact may
document no concern.

**Review evidence.** Build timing, artifact sections, generic instantiations, or
dependency analysis.

## RUST-DOC-0009-R020 — Retain reproducible evidence

**Statement.** Accepted performance decisions MUST retain commands, commits,
configuration, result summaries, and raw-data location or format sufficient to
repeat or challenge the result.

**Intent.** Make optimization decisions durable and auditable.

**Applicability.** Merged performance changes and release claims.

**Allowed exceptions.** Sensitive production traces may be retained in
controlled storage with a sanitized reproducible summary.

**Review evidence.** Benchmark record and provenance.

---

## Source: `doctrines/0010-staged-protocols/doctrine.md`

# Normative doctrine

## RUST-DOC-0010-R001 — Inventory the protocol before typing it

**Statement.** A staged protocol MUST have a written inventory of stages, edges, the evidence
each transition establishes, its failure classes, and its external effects before stage types
or capability traits are introduced.

**Intent.** Prevent a type graph from being derived mechanically from existing functions rather
than from the proof boundaries the domain actually has.

**Applicability.** Multi-stage command, request, submission, handshake, and workflow protocols
whose stage order carries consequence.

**Allowed exceptions.** A single-transition operation MAY record the inventory inline with its
design note.

**Review evidence.** Stage and edge inventory, evidence-per-transition table, and the design
note that preceded the types.

## RUST-DOC-0010-R002 — Name each stage by the fact it proves

**Statement.** A stage type MUST be named for the fact its construction establishes, and MUST
NOT be named for its position, its processing step, or a version counter.

**Intent.** Keep the stage graph readable as a sequence of proofs rather than an ordering of
implementation steps.

**Applicability.** Every named stage type and type-level state marker in a staged protocol.

**Allowed exceptions.** None. A stage whose proven fact cannot be named is evidence that the
boundary is not a real one.

**Review evidence.** Stage names, their documented guarantees, and the guarantee ledger.

## RUST-DOC-0010-R003 — Expose the successor capability in the stage contract

**Statement.** A stage capability whose protocol has a legal successor MUST expose that
successor as an associated type bounded by the capability the successor is required to satisfy,
rather than returning an unconstrained generic, an erased type, or a value whose successor
relationship exists only in prose.

**Intent.** Make the protocol edge a checked part of the contract, so a stage that stops leading
anywhere legal fails to compile instead of failing in review.

**Applicability.** Capability traits for staged protocols with more than one transition.

**Allowed exceptions.** A terminal stage MUST NOT name a successor. A protocol with exactly one
transition MAY return a concrete successor type directly when no second implementation is
anticipated.

**Review evidence.** Trait definitions, associated-type bounds, and the topology assertion
required by RUST-DOC-0010-R019.

## RUST-DOC-0010-R004 — Bound the successor by capability actually established

**Statement.** A successor bound MUST name only capabilities the successor value genuinely
establishes, and MUST NOT be widened, relaxed, or removed in order to make an implementation
compile.

**Intent.** Prevent the protocol contract from being edited to match a convenient
implementation, which converts a compile-time guarantee into decoration.

**Applicability.** Every associated successor type and its bounds.

**Allowed exceptions.** None. A bound that cannot be satisfied indicates the stage graph or the
implementation is wrong, not the bound.

**Review evidence.** Bound change history, the reason each bound exists, and the review record
for any relaxation.

## RUST-DOC-0010-R005 — Consume the stage on transition

**Statement.** A stage transition MUST consume the stage value when reuse of the prior stage
would be invalid, and MUST NOT rely on an internal flag to mark the stage as advanced.

**Intent.** Make the successor value the evidence that the transition ran, and make reuse of the
superseded stage a compiler error.

**Applicability.** Transitions between stages of a locally owned protocol. RUST-DOC-0003 governs
custody and RUST-DOC-0001 governs legal transitions generally; this rule adds the
stage-to-stage obligation.

**Allowed exceptions.** A read-only inspection that establishes no new fact MAY borrow. A
failure proven to occur before any part of the transition MAY return the prior stage with its
error.

**Review evidence.** Method receivers, recovery shapes, and the consumed-reuse compile-fail case.

## RUST-DOC-0010-R006 — Carry forward exactly the evidence successors need

**Statement.** A stage MUST carry the evidence its successors require, and MUST NOT retain a
superseded untrusted representation unless a named audit, diagnostic, or reconciliation
obligation requires it and the retained value is distinguishable from the canonical one.

**Intent.** Keep a later stage from re-deriving a fact, and keep a raw value from being mistaken
for a checked one after the stage that checked it.

**Applicability.** Stage payloads and the values transitions move between them.

**Allowed exceptions.** Audit, reconciliation, and error-reporting obligations MAY retain the
original input when it is separately named.

**Review evidence.** Stage fields, the field-provenance mapping, and tests that canonical values
survive every transition.

## RUST-DOC-0010-R007 — Keep stage failure distinguishable

**Statement.** Each transition MUST expose a failure type that identifies the stage that failed,
and a protocol MUST NOT erase its stage failures into one opaque type before the protocol
completes.

**Intent.** Preserve which proof was not established, which is the information a caller needs to
choose between retry, revision, and abandonment.

**Applicability.** Failure types of stage transitions. RUST-DOC-0002 governs error taxonomy
design; this rule adds the stage-identity obligation inside a protocol.

**Allowed exceptions.** A boundary adapter MAY map stage failures into one transport or
presentation error after the protocol completes.

**Review evidence.** Per-stage failure types, the boundary mapping, and tests asserting stage
identity is preserved.

## RUST-DOC-0010-R008 — Model material branches as named successor alternatives

**Statement.** A transition with materially different outcomes MUST return a named sum type over
distinct successor stages, and MUST NOT return one successor carrying optional fields that stand
in for a state that was never established.

**Intent.** Prevent a branch from degrading into a partially populated value that every later
stage must re-inspect.

**Applicability.** Approval, availability, eligibility, verification, and routing transitions.

**Allowed exceptions.** An outcome that changes no successor capability and no later obligation
MAY be represented as data on one successor.

**Review evidence.** Branch enum definitions, successor bounds per variant, and a test per
branch.

## RUST-DOC-0010-R009 — Name retry, revision, and recovery edges

**Statement.** A protocol that permits retry, revision, correction, or resumption MUST represent
each such path as a named stage and a named edge, and MUST NOT leave it implicit in caller
control flow.

**Intent.** Keep the recovery half of a protocol as visible and as reviewable as its success
path.

**Applicability.** Protocols with revisable input, contended identity, recoverable rejection, or
resumable interruption.

**Allowed exceptions.** A protocol whose only recovery is to restart from the initial stage MAY
state that explicitly instead of adding a stage.

**Review evidence.** Recovery stage types, the edges that reach them, and tests exercising each
recovery path.

## RUST-DOC-0010-R010 — Prohibit conversion paths that skip stages

**Statement.** A protocol MUST NOT expose a `From`, `Into`, `Default`, public constructor,
public field, or derived decoding path that constructs a later stage without performing the
intervening transitions.

**Intent.** Close the bypass that makes an otherwise sound stage graph decorative, since a
conversion that produces a later stage asserts every proof that stage represents.

**Applicability.** Trait implementations, constructors, field visibility, and derived
deserialization on stage types and stage evidence.

**Allowed exceptions.** A restricted trusted-construction path MAY exist under
RUST-DOC-0010-R011.

**Review evidence.** Trait implementation inventory, field visibility audit, derive audit, and
the evidence-forgery compile-fail case.

## RUST-DOC-0010-R011 — Restrict and inventory trusted stage construction

**Statement.** Any path that constructs a stage or its evidence without running the
corresponding transition MUST be visibility-restricted to a named owner, MUST be listed in the
guarantee ledger, and MUST state the obligation its caller assumes.

**Intent.** Keep necessary construction paths for testing, migration, and checked restoration
from becoming ambient protocol bypasses.

**Applicability.** Test builders, migration adapters, restoration services, and privileged
factories.

**Allowed exceptions.** None to omit the inventory. The path itself is permitted only with a
recorded owner and obligation.

**Review evidence.** Visibility, the escape-hatch inventory, and the caller obligation recorded
beside each path.

## RUST-DOC-0010-R012 — Keep stage granularity proportionate

**Statement.** A stage MUST correspond to a proof boundary rather than an implementation helper,
and the stage count SHOULD be justified against the complexity budget when the protocol exceeds
the size a reader can hold in one signature chain.

**Intent.** Prevent both directions of failure: one stage hiding several unrelated
responsibilities, and a stage per helper function.

**Applicability.** Protocol design and any change that adds or merges a stage.

**Allowed exceptions.** A regulated process MAY require a stage per externally mandated
checkpoint even when the engineering boundary is weaker.

**Review evidence.** Stage count, the proof each stage adds, the complexity-budget assessment,
and the rejected alternative granularity.

## RUST-DOC-0010-R013 — Disclose durable and external effects per stage

**Statement.** A transition MUST disclose the durable writes, external calls, and messages it
performs, and a transition named for a check, validation, or preparation MUST NOT perform a
durable write or publish a message.

**Intent.** Keep the collapsed call chain an accurate summary of what the protocol does, not
only of what it proves.

**Applicability.** Every transition in a protocol that touches storage, a network, a broker, or
a filesystem.

**Allowed exceptions.** A domain that genuinely defines one atomic operation MAY combine effects
under a name that says so.

**Review evidence.** Per-stage effect inventory, the transition names, and tests asserting that
effect-free stages perform no effect.

## RUST-DOC-0010-R014 — Do not present a local transition as a durable one

**Statement.** A consuming in-process transition MUST NOT be presented as evidence that a
durable or remote state change occurred, and a transition that advances authoritative state MUST
re-check the entity identity together with its stored state and a version, fence, or equivalent
concurrency token at the authoritative store.

**Intent.** Prevent the strongest available local guarantee from being read as a distributed
one. A move consumes a local value; stored facts are read, copied, and replayed, so no local
move can consume them.

**Applicability.** Protocols whose stages correspond to persisted lifecycle states, and any
mapping of a typed protocol onto database procedures or stored state.

**Allowed exceptions.** None for the claim. A protocol that never advances durable state states
that limit instead.

**Review evidence.** The authoritative-transition query or procedure, its concurrency token, the
guarantee ledger row separating local from durable proof, and competing-writer evidence.

## RUST-DOC-0010-R015 — Keep persisted or multi-actor lifecycle in a runtime model

**Statement.** Where protocol state is persisted, inspected heterogeneously, or advanced by more
than one actor, the durable model MUST be a runtime representation, and the typed stage protocol
MUST be scoped to one in-process pass that is issued by checked construction.

**Intent.** Keep a mechanism that is sound for a local sequence from being extended to a durable
lifecycle it cannot govern.

**Applicability.** Registration, onboarding, payment, approval, fulfillment, and any workflow
with durable status and several participants.

**Allowed exceptions.** A protocol that runs entirely within one process and stores nothing MAY
omit the runtime model.

**Review evidence.** The persisted representation, the restoration path that issues a typed
stage, and the conversion contract between the two.

## RUST-DOC-0010-R016 — State the async stage contract

**Statement.** An asynchronous transition MUST state its cancellation behavior, whether retry is
safe, the identity under which a retry is deduplicated, and whether the successor proof exists
only after a durable acknowledgment.

**Intent.** Keep an interrupted transition from silently producing a successor whose proof was
never completed.

**Applicability.** Transitions that await I/O, cross a process boundary, or can be cancelled.
RUST-DOC-0004 governs cancellation mechanics; this rule requires the contract per stage.

**Allowed exceptions.** A transition that performs no external effect and holds no resource MAY
state that cancellation is inconsequential.

**Review evidence.** Per-stage cancellation table, idempotency identity, retry policy, and fault
tests at each interruption point.

## RUST-DOC-0010-R017 — Erase the protocol only at a named boundary

**Statement.** Type erasure of protocol state into trait objects, maps, dynamic contexts, or
serialized documents MUST occur at a named orchestration or persistence boundary, and MUST NOT
occur between stages.

**Intent.** Keep the stage graph checkable for its whole length, since an erased intermediate
value ends static enforcement for every stage after it.

**Applicability.** Orchestration layers, dynamic strategy selection, and persistence adapters.

**Allowed exceptions.** Runtime selection among protocol implementations MAY be dynamic while
each selected branch continues to advance through typed stages.

**Review evidence.** The named boundary, what is erased there, and the reason earlier erasure is
unnecessary.

## RUST-DOC-0010-R018 — Prove the prohibited orderings

**Statement.** Illegal stage orderings, reuse of a consumed stage, and construction of stage
evidence outside its transition MUST have compile-fail evidence when the protocol claims those
programs are unrepresentable.

**Intent.** Keep a claimed impossibility from silently becoming possible during refactoring.

**Applicability.** Every negative guarantee a staged protocol states.

**Allowed exceptions.** A prohibition enforced only at runtime MUST be stated as a runtime check
rather than given compile-fail evidence it does not have.

**Review evidence.** Compile-fail cases, their reviewed diagnostics, and confirmation that each
rejection occurs at the intended boundary.

## RUST-DOC-0010-R019 — Assert the stage graph executably

**Statement.** The stage and successor graph a protocol documents MUST be asserted executably,
so that a redirected associated type, a widened bound, or a removed implementation is detected
by the build rather than by reading.

**Intent.** Keep the documented topology and the compiled topology from diverging, which is the
failure that prose review is least able to catch.

**Applicability.** Protocols with more than two stages or more than one branch.

**Allowed exceptions.** A protocol whose complete graph is visible in one function signature MAY
rely on that signature.

**Review evidence.** The topology assertion, its coverage of every documented edge, and its
failure when an edge is changed.

## RUST-DOC-0010-R020 — Record a guarantee ledger row per stage

**Statement.** Each stage MUST have a guarantee ledger row stating the claim it establishes, the
transition that establishes it, how its construction is protected, how boundary decoding
preserves it, its escape hatches, what it does not prove, and the residual runtime risk.

**Intent.** Keep the protocol's honesty auditable at the granularity at which its claims are
made.

**Applicability.** Every stage type and every piece of stage evidence.

**Allowed exceptions.** None.

**Review evidence.** The completed ledger and its agreement with the stage definitions.

## RUST-DOC-0010-R021 — Keep protocol terminology honest

**Statement.** Documentation for a staged protocol MUST NOT present project vocabulary as
standardized external terminology, and MUST identify the established family a mechanism belongs
to when it uses a local name for it.

**Intent.** Keep a useful local vocabulary from being cited as external authority it does not
have.

**Applicability.** Doctrine text, design notes, API documentation, and agent instructions that
name a protocol mechanism.

**Allowed exceptions.** Terms defined by a cited specification or published literature MAY be
used as standard when the citation is given.

**Review evidence.** Terminology definitions, their family attribution, and the source notes
recording which vocabulary is local.

## RUST-DOC-0010-R022 — Keep governance precedence explicit

**Statement.** An executable protocol is authoritative for the in-process ordering it enforces,
and MUST NOT be treated as replacing doctrine obligations, recorded review evidence, or the
decision process required to change a normative contract.

**Intent.** Keep the accurate observation that code enforces ordering from becoming the claim
that code alone settles what a system is obliged to do.

**Applicability.** Design notes, doctrine proposals, and agent instructions that describe a
protocol as a living or self-documenting contract.

**Allowed exceptions.** None.

**Review evidence.** The governing decision record, the review evidence for the protocol, and
the guarantee ledger the code does not itself supply.

## Guarantee and non-guarantee requirements

A staged protocol states, for each stage and each piece of stage evidence: the claim its
construction establishes under RUST-DOC-0010-R002; how construction is protected under
RUST-DOC-0010-R010 and RUST-DOC-0010-R011; how decoding and restoration preserve or re-establish
it under RUST-DOC-0010-R015; its escape hatches under RUST-DOC-0010-R011; the external facts
that remain mutable under RUST-DOC-0010-R014; the failures that remain runtime failures under
RUST-DOC-0010-R007; the outcomes that can remain indeterminate under RUST-DOC-0010-R016; and the
executable evidence supporting the claim under RUST-DOC-0010-R018 and RUST-DOC-0010-R019.

## Boundary requirements

Untrusted input enters at the initial stage and is canonicalized under RUST-DOC-0010-R006 before
any stage claims a checked value. Persistence and wire boundaries follow RUST-DOC-0010-R015 and
RUST-DOC-0010-R017: durable state is a runtime model, erasure is named, and a typed stage is
issued only by checked construction. Durable advancement follows RUST-DOC-0010-R014 and re-checks
identity, stored state, and a concurrency token. Sensitive values carried as stage evidence
remain subject to RUST-DOC-0003 secret handling, and failure mapping at the outer boundary
follows RUST-DOC-0010-R007.

## Waiver requirements

RUST-DOC-0010-R012, RUST-DOC-0010-R016, and RUST-DOC-0010-R019 MAY be waived for a protocol
whose scope, lifetime, or effect makes the obligation disproportionate. A waiver records the
affected rule and protocol, the owner accepting the risk, the consequence, the compensating
control, an expiry or reconsideration trigger, and the removal condition.

RUST-DOC-0010-R003, RUST-DOC-0010-R004, RUST-DOC-0010-R010, RUST-DOC-0010-R011,
RUST-DOC-0010-R014, RUST-DOC-0010-R020, RUST-DOC-0010-R021, and RUST-DOC-0010-R022 MUST NOT be
waived. A waiver cannot make a bypassed protocol sound, cannot convert a local move into a
durable transition, and cannot make an inaccurate external claim true.

---

## Source: `patterns/README.md`

# Representation patterns

Patterns are reusable design mechanisms, not universal prescriptions. Each
pattern begins with a problem and forces, compares a weak and improved
representation, and states both guarantees gained and guarantees not gained.
Boundary, persistence, testing, and complexity sections prevent a local type
shape from being mistaken for a complete system proof.

| Pattern                                             | Primary fit                                                               | Common overapplication                               |
| --------------------------------------------------- | ------------------------------------------------------------------------- | ---------------------------------------------------- |
| [Sum types](../patterns/sum-types.md)                           | mutually exclusive runtime states                                         | variant explosion for independent dimensions         |
| [Opaque newtypes](../patterns/opaque-newtypes.md)               | one value with a stable local invariant                                   | names stronger than construction evidence            |
| [Smart constructors](../patterns/smart-constructors.md)         | checked establishment and normalization                                   | incomplete checks split across callers               |
| [Typestate](../patterns/typestate.md)                           | small, locally controlled protocol sequence                               | persisted or externally determined state             |
| [Capability types](../patterns/capability-types.md)             | possession represents authority                                           | cloneable handles with undefined revocation          |
| [Consuming transitions](../patterns/consuming-transitions.md)   | prevent reuse of prior lifecycle state                                    | losing recovery evidence on fallible transition      |
| [Validated collections](../patterns/validated-collections.md)   | non-empty, bounded, sorted, or unique sets                                | mutation paths that invalidate the wrapper           |
| [Hybrid state machines](../patterns/hybrid-state-machines.md)   | local typed workflow plus dynamic persistence                             | duplicated state without conversion contract         |
| [Explicit uncertainty](../patterns/explicit-uncertainty.md)     | external effect may have indeterminate outcome                            | treating unknown as generic error                    |
| [Successor capabilities](../patterns/successor-capabilities.md) | one capability, several implementations with differing successor evidence | bounds widened until the protocol edge is decorative |

## Selection rule

Choose the simplest mechanism that directly protects the consequential
invariant:

- mutually exclusive states: sum type;
- refined scalar or identifier: opaque newtype and smart constructor;
- collection invariant: validated wrapper;
- locally controlled sequence with few states: typestate or consuming
  transition;
- multi-stage sequence whose capabilities have several implementations with
  differing successor evidence: successor capabilities;
- authority: capability;
- dynamic, heterogeneous, persisted, or externally observed state: runtime
  enum/state machine;
- external effect: `Result` plus explicit unknown/reconciliation state where
  execution can be ambiguous.

Patterns can combine. A payment workflow may use an opaque operation ID, a
capability for capture authority, consuming local transitions, a persisted
runtime status, and explicit unknown capture outcome. Each layer must name its
own evidence and avoid claiming the others' guarantees.

## Pattern review

Reviewers should ask:

1. Which invariant is protected?
2. Can every construction and mutation path preserve it?
3. What boundary re-establishes the evidence?
4. How is the value persisted and evolved?
5. Which external facts remain mutable?
6. Does the mechanism improve diagnostics?
7. Is the type/API complexity proportional to misuse frequency and impact?
8. Which executable evidence demonstrates admitted and prohibited behavior?

Executable examples live under [`../examples/`](../examples/). They illustrate
mechanics and limitations; they are not a substitute for a domain's own
invariant inventory.

---

## Source: `reviews/final-correctness-audit.md`

# Final correctness audit

## Record

Run before merge or release for material changes. Record change/release,
commit, auditor, date, applicable doctrines, focused-review references, and
**pass**, **fail**, **not applicable**, or **waiver reference** for every gate.
This audit checks evidence; it does not infer completion from CI color.

## Repository and scope integrity

| ID     | Question                                                                                  | Pass evidence                       |
| ------ | ----------------------------------------------------------------------------------------- | ----------------------------------- |
| FCA-01 | Does the diff match the approved scope?                                                   | complete diff review                |
| FCA-02 | Are unrelated user changes preserved?                                                     | status/diff provenance              |
| FCA-03 | Are all new files intentional and reviewable?                                             | full file inventory                 |
| FCA-04 | Are archives, encoded payloads, generated source commits, and transient artifacts absent? | inventory/scan                      |
| FCA-05 | Are secrets, credentials, personal paths, and internal identifiers absent?                | positive-controlled secret/PII scan |
| FCA-06 | Are canonical and generated paths separated?                                              | architecture check                  |
| FCA-07 | Are generated files derived only by the declared tool?                                    | clean regeneration                  |
| FCA-08 | Are dependency additions justified and licensed?                                          | dependency review                   |
| FCA-09 | Is MSRV/toolchain policy preserved?                                                       | toolchain matrix                    |
| FCA-10 | Is repository version/change log accurate?                                                | metadata comparison                 |

## Invariants, construction, and authority

| ID     | Question                                                                   | Pass evidence                 |
| ------ | -------------------------------------------------------------------------- | ----------------------------- |
| FCA-11 | Is the invariant inventory current?                                        | reviewed artifact             |
| FCA-12 | Does every changed trusted type have exact proof and non-proof statements? | documentation/ledger          |
| FCA-13 | Are trusted fields and constructors protected?                             | visibility/construction audit |
| FCA-14 | Do all decoders preserve construction evidence?                            | Serde/DB/boundary trace       |
| FCA-15 | Are contradictory states structurally absent or explicitly rejected?       | state truth table             |
| FCA-16 | Are legal transitions and authority explicit?                              | state/authority graph         |
| FCA-17 | Are capability cloning, transfer, expiry, and revocation honest?           | lifecycle contract            |
| FCA-18 | Are secret types protected from formatting and serialization?              | trait audit                   |
| FCA-19 | Are cross-entity invariants enforced transactionally/runtime?              | service/query evidence        |
| FCA-20 | Are escape hatches enumerated, scoped, and reviewed?                       | ledger                        |

## Boundaries, persistence, and evolution

| ID     | Question                                                   | Pass evidence         |
| ------ | ---------------------------------------------------------- | --------------------- |
| FCA-21 | Is every ingress represented raw → structural → trusted?   | boundary map          |
| FCA-22 | Are resource limits enforced before expensive processing?  | limits/tests          |
| FCA-23 | Are authentication and authorization distinct?             | request flow          |
| FCA-24 | Are unknown fields/versions/variants handled deliberately? | compatibility policy  |
| FCA-25 | Are durable formats and enum tags stable/versioned?        | schema/encoding       |
| FCA-26 | Do migrations state and verify invariant transformations?  | migration evidence    |
| FCA-27 | Are invalid historical values rejected or quarantined?     | tests/operations      |
| FCA-28 | Are lost updates and conflicts explicit?                   | version/lock protocol |
| FCA-29 | Are transaction isolation claims mechanism-specific?       | database analysis     |
| FCA-30 | Are public errors structured and redacted?                 | error tests           |

## Concurrency, effects, and uncertainty

| ID     | Question                                                                 | Pass evidence           |
| ------ | ------------------------------------------------------------------------ | ----------------------- |
| FCA-31 | Is shared mutable state ownership explicit?                              | ownership map           |
| FCA-32 | Are locks scoped and ordered?                                            | lock graph              |
| FCA-33 | Is async blocking work isolated and bounded?                             | pool/capacity design    |
| FCA-34 | Are cancellation points and cleanup reviewed?                            | cancellation matrix     |
| FCA-35 | Are tasks supervised and shutdown bounded?                               | task tree/tests         |
| FCA-36 | Are queues and concurrency bounded with backpressure?                    | capacity/overload tests |
| FCA-37 | Does every external effect remain fallible?                              | APIs                    |
| FCA-38 | Does timeout preserve unknown execution?                                 | outcome states          |
| FCA-39 | Are idempotency scope, binding, retention, and replay defined?           | key contract            |
| FCA-40 | Are duplicates and acknowledgement loss expected?                        | consumer evidence       |
| FCA-41 | Are ordering and exactly-once claims scoped?                             | guarantee ledger        |
| FCA-42 | Is persistence plus side effect coordinated without fictional atomicity? | outbox/reconciliation   |
| FCA-43 | Are compensations fallible new effects?                                  | saga model              |
| FCA-44 | Are unknown outcomes durable, owned, and reconcilable?                   | operations plan         |

## Unsafe, evidence, and performance

| ID     | Question                                                                                    | Pass evidence          |
| ------ | ------------------------------------------------------------------------------------------- | ---------------------- |
| FCA-45 | Is unsafe code absent or fully reviewed under doctrine 0007?                                | unsafe inventory/proof |
| FCA-46 | Does each unsafe block state complete safety premises?                                      | local comments         |
| FCA-47 | Are FFI ABI, ownership, unwind, and threading explicit?                                     | boundary contract      |
| FCA-48 | Are unsafe dependencies proportionally reviewed?                                            | dependency audit       |
| FCA-49 | Do tests trace to invariants and failure risks?                                             | evidence matrix        |
| FCA-50 | Are positive, negative, and prohibited programs covered?                                    | test suite             |
| FCA-51 | Are real boundaries exercised where consequential?                                          | integration evidence   |
| FCA-52 | Are cancellation, duplicate, reordering, and partial failures injected?                     | fault matrix           |
| FCA-53 | Were compile-fail diagnostics inspected semantically?                                       | reviewed stderr diff   |
| FCA-54 | Are snapshots reviewed rather than bulk accepted?                                           | focused rationale      |
| FCA-55 | Is flakiness resolved rather than retried away?                                             | failure records        |
| FCA-56 | Are model/Miri/sanitizer limits stated?                                                     | evidence limits        |
| FCA-57 | Are performance claims workload- and environment-scoped?                                    | benchmark record       |
| FCA-58 | Does profiling support optimization?                                                        | profile                |
| FCA-59 | Are latency distributions, allocation, contention, and boundary costs measured as relevant? | results                |
| FCA-60 | Is correctness evidence independent from benchmarks?                                        | suite linkage          |

## Governance and reproducibility

| ID     | Question                                                            | Pass evidence                |
| ------ | ------------------------------------------------------------------- | ---------------------------- |
| FCA-61 | Are normative changes identified rather than called wording edits?  | doctrine diff classification |
| FCA-62 | Does every required normative change have an accepted RFC?          | RFC link                     |
| FCA-63 | Are doctrine IDs and versions preserved or changed by policy?       | manifest comparison          |
| FCA-64 | Are source notes and attribution current?                           | provenance review            |
| FCA-65 | Do manifests and JSON Schemas agree?                                | lint/schema result           |
| FCA-66 | Does doctrine lint pass on the complete tree?                       | exact command/result         |
| FCA-67 | Does deterministic bundle generation produce no diff?               | generate/check result        |
| FCA-68 | Do format, Clippy, tests, compile-fail, and dependency policy pass? | exact commands/results       |
| FCA-69 | Do Markdown links pass with only narrow documented exclusions?      | link-check result            |
| FCA-70 | Is the working tree clean after regeneration and validation?        | `git status --short`         |

## Required guarantee ledger

Every major domain or case-study claim uses:

| Claim       | Established by                                 | Protected construction      | Boundary preservation         | Escape hatches   | Does not prove | Residual runtime risk |
| ----------- | ---------------------------------------------- | --------------------------- | ----------------------------- | ---------------- | -------------- | --------------------- |
| exact claim | constructor, transition, protocol, or evidence | privacy/authority mechanism | decoding and persistence path | privileged paths | excluded facts | failure/uncertainty   |

The auditor rejects rows whose claim is broader than establishment evidence.
External mutable facts state observation time and reconciliation. Passing tests
appear under evidence, never as universal proof.

## Exit criteria

Release or merge approval requires every critical item to pass, all focused
reviews to be referenced, the guarantee ledger to be complete, generation and
validation to reproduce cleanly, and residual limitations to be written in the
change record. CI confirms locally discovered results; it does not replace this
audit.

---

## Source: `agents/shared.md`

# Shared agent obligations

## Mission

Produce Rust systems whose important guarantees are discoverable, accurately
named, protected at construction and transition, preserved at boundaries, and
supported by proportionate evidence. Compilation and test success are evidence
layers, not the definition of correctness. Follow repository `AGENTS.md` and
read applicable canonical doctrine before changing code or doctrine.

## Required reasoning order

1. State domain vocabulary and desired outcome.
2. Inventory invariants using
   [`../foundations/invariants.md`](../foundations/invariants.md).
3. Classify values, states, transitions, authority, boundaries, cross-entity
   rules, temporal assumptions, and distributed facts.
4. Map every ingress, durable representation, external effect, and observation.
5. Select the simplest mechanism that directly protects the consequential
   invariant.
6. Protect construction and mutation.
7. Keep external effects and cleanup fallible.
8. Represent indeterminate execution explicitly.
9. Map claims to executable and operational evidence.
10. Complete a guarantee ledger and relevant review.

Do not begin with typestate, an error crate, `Arc<Mutex<_>>`, or an ORM model.
Begin with the invariant and trust boundary.

## Representation obligations

Use an enum for mutually exclusive runtime state. Use an opaque newtype for a
stable local value invariant. Use a validated wrapper for aggregate collection
rules. Consider a consuming transition or typestate only for a small,
locally controlled sequence. Use a capability when possession should represent
authority. Use a runtime service or transaction for cross-entity facts. Use
ordinary code when added type structure removes little consequential risk.

RUST-DOC-0001 is central. Apply evidence-accurate names: `EmailAddress` cannot
mean mailbox ownership unless verification evidence is required; `Open` cannot
mean future remote liveness. `NonZeroU64` cannot mean complete money policy.

## Boundary obligations

Model:

```text
raw input → structural value → validated domain value
          → effect attempt → observed/reconciled outcome
```

Validation is centralized, not eliminated. Audit Serde, database, file,
message, HTTP/RPC, configuration, and FFI paths for bypass. A trusted domain
type must not expose a public construction path weaker than its claim.
Authentication and authorization are separate. Persistence is historical
evidence and must be decoded against current invariants.

Apply limits before large allocations. Preserve version and unknown-value
policy. Avoid logs and diagnostics that expose credentials, secrets, or
sensitive domain values.

## Failure and uncertainty obligations

Keep expected external failure out of panics. Preserve structured categories
when callers act differently: rejection, validation, conflict, cancellation,
timeout, unavailable, and unknown execution. Do not retry by transport class
alone. A timeout after possible dispatch requires an explicit unknown state,
stable operation identity, and reconciliation plan when the effect matters.

Idempotency is a receiver protocol, not a header name. Define scope, payload
binding, concurrent attempt behavior, response replay, retention, and expiry.
Compensation is a new fallible action, not rollback.

## Evidence obligations

For each material claim identify:

- enforcement mechanism;
- construction protection;
- boundary preservation;
- escape hatches;
- positive evidence;
- negative/prohibited evidence;
- non-guarantees;
- residual runtime risk.

Use unit tests for local behavior, property tests for generative invariants,
compile-fail tests for important prohibited programs, real integration tests
for boundary behavior, fault injection for partial/distributed failures, and
model or unsafe-specific tools where warranted. Inspect compile-fail diagnostics
before updating committed expected output. Treat flaky tests as system evidence.

## Forbidden claims

Never claim:

- compilation proves domain correctness;
- passing tests prove universal correctness;
- integer money removes all rounding policy;
- parsed email proves ownership or deliverability;
- a connected typestate guarantees next network success;
- a database transaction includes unrelated external effects;
- timeout proves non-execution;
- an outbox makes end-to-end delivery exactly once;
- a lease prevents stale owners without effect-level fencing;
- async automatically makes CPU work faster;
- unsafe is sound because Miri passed.

## Canonical and generated sources

Never edit `dist/` manually. Change canonical material, update manifests where
selection changes, regenerate, and check deterministic output. Generated text
must retain its banner and source provenance. A bundle mismatch is a failed
repository state.

## Escalation

Escalate when intent materially changes representation, authorization,
persistence, external-effect semantics, public compatibility, unsafe proof,
licensing, or normative doctrine. Before escalating, read relevant sources and
present the exact unresolved decision, consequences, evidence, and recommended
option. Do not guess through irreversible or security-sensitive ambiguity.

Normative weakening, a new escape hatch, supersession, or new normative rule
requires RFC governance. A wording edit that changes meaning is normative even
if its diff is small.

## Completion

Completion means canonical files and code are consistent; the guarantee ledger
is honest; required tests and focused reviews pass; generated output reproduces;
format, Clippy, tests, lint, schemas, dependency policy, and links pass; and the
working tree contains no accidental artifact or secret. Report failed or
unperformed checks exactly.
