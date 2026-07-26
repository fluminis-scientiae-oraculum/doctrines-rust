# Complexity budget

Rust can encode rich protocols and refined values, but every type parameter, wrapper, trait
bound, macro, state marker, and conversion consumes a complexity budget. The goal is not
maximum type cleverness. The goal is the simplest representation that prevents consequential
invalid programs without obscuring the system.

Complexity is justified by risk removed, not by elegance in isolation. A two-state local
builder used by many callers may benefit from typestate. A persisted workflow with dozens of
states, external transitions, dynamic inspection, and evolving schema is usually clearer as
a runtime enum plus validated transitions. A bounded amount can be an opaque newtype; a
cross-account balance rule belongs in transactional runtime logic.

## Budget inputs

Assess the following before selecting additional type machinery.

### State and transition shape

Count meaningful states, legal transitions, conditional transitions, and state-specific data.
Estimate growth. Typestate works best when the graph is small, locally controlled, and static.
Generic states can explode public APIs when transitions depend on runtime policy or external
responses.

### Misuse frequency and impact

Ask how often the invalid action is plausible and what happens if it occurs. A wrong-state
payment capture deserves stronger prevention than a harmless display preference. Repeated
production incidents, security consequences, financial loss, or irreversible external effects
increase the budget.

### Public API surface

Structural enforcement is more valuable at a widely reused library boundary because callers
cannot share all local context. It also increases compatibility obligations. Public marker
types, error enums, and generics become part of the API. Consider evolution, downstream
diagnostics, semver, and language bindings.

### Serialization and persistence

Generic typestate does not naturally serialize a heterogeneous runtime state. Persisted
records need stable discriminants, unknown-variant policy, migration, and runtime decoding.
A hybrid can use a runtime enum for storage and consuming transitions for one local operation.
Duplicated representations require explicit conversion and evidence.

### Dynamic dispatch and heterogeneity

Collections of mixed states, plugins, trait objects, user-driven workflows, and runtime
inspection often favor enums or trait objects. Encoding every state in a distinct concrete
type can force boxing, erasure, or large match layers that remove the expected benefit.

### Async and external effects

An async transition can fail, be cancelled, or become ambiguous. A consuming method that
loses the prior value on failure may make recovery awkward. The API may need to return the
previous state with the error, store durable intent before awaiting, or use a runtime state
machine. Types cannot turn a timeout into certainty.

### Trait-bound readability

Complex generic constraints can hide the business rule and produce diagnostics remote from
the caller's mistake. Measure whether a new team member can understand construction,
transition, and recovery without reverse-engineering type algebra. Good compiler rejection is
part of usability evidence.

### Compile time and monomorphization

Generic states and combinatorial trait implementations can increase compilation, incremental
rebuild time, generated code, and binary size. These are measurement questions. Do not claim
zero-cost solely because the state markers are zero-sized; monomorphization and API
duplication can still cost.

### Team familiarity and maintenance

A mechanism that only one maintainer understands has operational risk. Documentation and
examples can reduce that cost, but should not be used to justify needless abstraction.
Migration, debugging, on-call diagnosis, and incident repair count alongside authoring.

### Interoperation

FFI, databases, Serde, RPC schemas, and other languages often need runtime representations.
If every boundary erases and reconstructs type state, verify that the proof is real and the
conversion cost is worthwhile.

## Mechanism ladder

Choose the lowest-cost mechanism that adequately contains the failure:

1. clear ordinary code and a named runtime check;
2. enum for mutually exclusive runtime state;
3. opaque newtype for a stable local value invariant;
4. validated collection wrapper;
5. consuming method to prevent immediate local reuse;
6. capability type for authority;
7. typestate for small locally controlled sequencing;
8. hybrid runtime and compile-time state model;
9. more elaborate type-level proof only for exceptional, evidenced risk.

This is not a universal ranking. A capability may be simpler than a complex runtime
permission object. An enum may be both cheaper and stronger for contradictory state. The
ladder prompts an explanation for skipping simpler options.

## Complexity decision record

For a representation choice, record:

| Dimension | Observation | Cost or risk | Evidence |
|---|---|---|---|
| Invalid action | Which misuse is prevented | Consequence and frequency | incidents, threat model, review |
| State graph | State and transition count | explosion or clarity | state diagram |
| Control | local, external, or shared | stale proof and runtime need | boundary map |
| Persistence | format and migration | conversion and compatibility | schema tests |
| API | caller count and stability | semver and diagnostics | compile-fail tests |
| Runtime | dispatch, allocation, synchronization | latency and contention | benchmarks or profiles |
| Build | generics and macros | compile time and binary size | measured builds |
| Team | familiarity and support | maintenance and incident cost | review exercise |
| Alternative | simpler mechanism | residual invalidity | comparative prototype |

The decision also states a removal trigger. If state count grows beyond the usable limit,
persistence becomes necessary, diagnostics degrade, or measurement shows material cost, the
design should be reconsidered.

## Typestate budget

Typestate is proportionate when:

- sequencing is locally controlled;
- state count and transition graph are small;
- ownership can naturally consume prior state;
- callers benefit from compiler rejection;
- state-specific methods are stable;
- objects do not require heterogeneous storage or routine serialization;
- async failure can return usable recovery state or is otherwise designed;
- and compile diagnostics are understandable.

Prefer a runtime enum when state is externally chosen, persisted, replayed, discovered at
runtime, held in mixed collections, inspected by UI or operations, or expected to evolve
frequently. Prefer a consuming transition without a generic state parameter when only reuse
prevention matters.

Do not use a phantom marker to suggest external liveness or authorization that can be revoked.
The complexity buys local protocol evidence only.

## Newtype budget

Opaque newtypes are usually low-cost for stable scalar invariants. Costs still include
conversion, formatting, borrowing, Serde adapters, database decoding, error types, and policy
versioning. Avoid creating a distinct wrapper for every conceptual label when values have no
different invariant or accidental interchange consequence.

A wrapper must control mutation. A `NonEmptyVec` that exposes unrestricted `Vec::clear` is
dishonest. An email wrapper with a public tuple field buys only naming.

## Capability budget

Capability types are valuable when possession should grant a narrow operation. Review issuance,
clone, transfer, serialization, expiry, and revocation. If authority is checked centrally on
every operation and local possession offers no stable right, a runtime authorization decision
may be clearer. If revocation is essential, a capability may carry an identifier and require
online validation rather than claim perpetual authority.

## Runtime simplicity is legitimate

Plain runtime validation is not a design failure. External input, mutable reality,
cross-entity facts, configurable policy, concurrency, and distributed outcomes require
runtime checks. An explicit function with a structured error can be more honest than a type
whose proof becomes stale.

The important questions are whether validation is centralized, every boundary uses it,
failure is represented, and evidence covers violation. “Compile time” is not automatically
stronger when the fact exists only at runtime.

## Measuring complexity

Do not replace one unmeasured claim with another. Measure compile time, binary size,
allocations, latency, or diagnostic quality when those costs drive the decision. Use a
representative workload and compare against a simpler design. Record toolchain, features,
target, and inputs. A zero-sized marker does not prove zero system cost.

Qualitative costs can also be tested: ask a reviewer to trace a failed transition, have a new
maintainer add a state, inspect compiler messages from incorrect calls, and simulate migration
from a stored record. These exercises produce evidence about operability.

## Review questions

Review asks:

- Which consequential invalid program becomes impossible?
- Could an enum, newtype, runtime check, or consuming method achieve the same result more
  clearly?
- Does the mechanism preserve evidence across every boundary?
- Is the proof local while the name implies an external fact?
- What state or policy growth breaks the design?
- How does failure, cancellation, or uncertainty return usable information?
- Are compiler diagnostics better than runtime errors for intended callers?
- What compatibility surface and build cost are added?
- Is the mechanism understood and testable by the maintenance team?
- Which observation would trigger simplification?

Complexity budgeting is not permission to leave severe risk uncontained. It is the discipline
that makes protection sustainable. Strong design spends complexity where invalidity is
consequential and keeps the rest legible.
