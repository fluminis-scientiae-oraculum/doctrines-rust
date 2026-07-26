# Glossary

**Aliasing invariant**
: The set of permitted simultaneous references and mutation paths for one
  memory region.

**Fencing**
: Prevention of stale concurrent authority, commonly through a monotonically
  checked token.

**Foreign-function interface**
: A boundary where Rust calls or is called by code governed by another ABI,
  layout, allocation, error, or unwind model.

**Initialized**
: Containing a valid value for the relevant typed interpretation, not merely
  allocated bytes.

**Provenance**
: The allocation and authority history associated with a pointer, beyond its
  numeric address.

**Safe abstraction**
: An API whose safe callers cannot cause undefined behavior, even though its
  private implementation uses unsafe operations.

**Safety invariant**
: A condition that must hold whenever unsafe code relies on it to satisfy
  language or library preconditions.

**Soundness**
: Preservation of Rust's safety contract for all behavior available through a
  safe interface.

**Validity**
: Requirements a bit pattern must satisfy to be observed as a particular Rust
  type.

**Unwinding**
: Stack traversal caused by panic or a foreign exception, including destructor
  execution across frames.
