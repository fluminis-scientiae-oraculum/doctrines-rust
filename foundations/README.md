# Foundations

Foundations define the vocabulary and reasoning contracts used by every doctrine in this
repository. They are separate because a doctrine should not privately redefine "invariant,"
"evidence," "trusted," "guarantee," or the force of `MUST`. A shared definition lets rule IDs
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
or a waiver reference. A prose assertion such as "validation looks adequate" is weaker than
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
