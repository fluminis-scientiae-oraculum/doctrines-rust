# Glossary

**Capability type**

A value whose possession grants a bounded operation. Its authority depends on protected
issuance, scope, clone and transfer semantics, expiry, and revocation.

**Consuming transition**

A method taking ownership of the prior state or authority so ordinary safe code cannot reuse
it after transition.

**Contradictory state**

A representable combination that the domain declares impossible, such as paid and failed
simultaneously.

**Evidence-accurate name**

A name whose implied claim does not exceed what constructors, transitions, or observations
establish.

**Escape hatch**

A construction or mutation path that assumes rather than checks the ordinary invariant,
including unchecked, unsafe, administrative, migration, or privileged paths.

**Hybrid state machine**

A design using runtime state for persistence or external observation and compile-time state or
capabilities for a bounded local operation.

**Indeterminate outcome**

A state in which the system lacks evidence to classify an external effect as confirmed
success or confirmed non-execution/rejection.

**Opaque newtype**

A wrapper with representation hidden from ordinary callers so validated construction gives
the type evidentiary meaning.

**Protected construction**

Visibility, proof tokens, fallible constructors, and controlled mutation that prevent
untrusted code from forging the claimed evidence.

**Reconciliation identity**

Durable operation, provider, correlation, or token data sufficient to observe and resolve an
indeterminate external outcome.

**Runtime state machine**

An explicit state representation and validated transition function evaluated at runtime,
often required for persistence, dynamic inspection, concurrency, or external state.

**Smart constructor**

A fallible constructor that parses, normalizes, validates, or requires proof before producing
a trusted type.

**Sum type**

An enum or equivalent representation where a value is exactly one of several variants, each
with its own data.

**Typestate**

Compile-time representation of a local object's protocol phase through distinct concrete
types, generic marker states, or state-specific implementations.

**Trusted domain representation**

A value whose documented invariant has been established through reviewed construction. Trust
is scoped to that invariant and does not imply external certainty.

**Validated collection**

A wrapper that establishes and preserves a whole-collection property such as non-empty,
bounded, sorted, unique, or compatible members.
