# Representation patterns

Patterns are reusable design mechanisms, not universal prescriptions. Each
pattern begins with a problem and forces, compares a weak and improved
representation, and states both guarantees gained and guarantees not gained.
Boundary, persistence, testing, and complexity sections prevent a local type
shape from being mistaken for a complete system proof.

| Pattern | Primary fit | Common overapplication |
|---|---|---|
| [Sum types](sum-types.md) | mutually exclusive runtime states | variant explosion for independent dimensions |
| [Opaque newtypes](opaque-newtypes.md) | one value with a stable local invariant | names stronger than construction evidence |
| [Smart constructors](smart-constructors.md) | checked establishment and normalization | incomplete checks split across callers |
| [Typestate](typestate.md) | small, locally controlled protocol sequence | persisted or externally determined state |
| [Capability types](capability-types.md) | possession represents authority | cloneable handles with undefined revocation |
| [Consuming transitions](consuming-transitions.md) | prevent reuse of prior lifecycle state | losing recovery evidence on fallible transition |
| [Validated collections](validated-collections.md) | non-empty, bounded, sorted, or unique sets | mutation paths that invalidate the wrapper |
| [Hybrid state machines](hybrid-state-machines.md) | local typed workflow plus dynamic persistence | duplicated state without conversion contract |
| [Explicit uncertainty](explicit-uncertainty.md) | external effect may have indeterminate outcome | treating unknown as generic error |

## Selection rule

Choose the simplest mechanism that directly protects the consequential
invariant:

- mutually exclusive states: sum type;
- refined scalar or identifier: opaque newtype and smart constructor;
- collection invariant: validated wrapper;
- locally controlled sequence with few states: typestate or consuming
  transition;
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
