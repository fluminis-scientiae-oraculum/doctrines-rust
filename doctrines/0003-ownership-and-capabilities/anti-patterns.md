# Anti-patterns

## Cloneable single-use token

**Weak example.** A capture token derives `Clone`. **Why it fails.** Consumption of one copy
does not consume authority. **Risk.** duplicate effect. **Improved direction.** Remove clone
and add idempotency separately. **Justified appearance.** None for truly single-use authority.

## Public capability constructor

**Weak example.** Any caller can create `AdminCapability`. **Why it fails.** The type is
forgeable. **Risk.** authorization bypass. **Improved direction.** Restrict issuance to policy
owner and embed scope. **Justified appearance.** A nominal marker with no authority claim.

## RAII equals rollback

**Weak example.** Transaction drop is documented as guaranteeing rollback. **Why it fails.**
External rollback can fail and `Drop` cannot report it. **Risk.** false state and lost
diagnostics. **Improved direction.** explicit fallible rollback; drop as observed fallback.
**Justified appearance.** Infallible in-memory restoration.

## Secret derives everything

**Weak example.** Token derives `Debug`, `Clone`, `Serialize`. **Why it fails.** Ordinary
tooling leaks and copies it. **Risk.** credential exposure. **Improved direction.** redacted
debug, no display, scoped exposure, protected store adapter. **Justified appearance.**
Serialization only through a separate encrypted-store contract.

## Universal zeroization

**Weak example.** Clearing one `Vec<u8>` is claimed to remove the secret. **Why it fails.**
Copies and external traces remain. **Risk.** false security assurance. **Improved direction.**
state exact cleared buffer and uncovered paths. **Justified appearance.** Narrow buffer-level
claim.

## `Arc<Mutex<T>>` architecture

**Weak example.** Every service shares one mutable application state. **Why it fails.** No
progress owner, broad lock scope, hidden authority. **Risk.** deadlock and contention.
**Improved direction.** task ownership, bounded commands, partitioning, or narrow lock.
**Justified appearance.** Small shared cache with documented invariant and measured contention.

## `RefCell` to silence borrowing

**Weak example.** Interior mutability replaces a difficult ownership choice. **Why it fails.**
Borrow failure moves to runtime and reentrancy may panic. **Risk.** latent failure.
**Improved direction.** decide owner or document aliasing requirement. **Justified appearance.**
single-threaded cache with controlled reentrancy.

## Lifetime as liveness

**Weak example.** A borrow lifetime is described as proving a remote session stays active.
**Why it fails.** Lifetimes govern references, not external systems. **Risk.** skipped
revalidation. **Improved direction.** explicit lease/expiry and fallible use. **Justified
appearance.** Lifetime can tie a local guard to its resource.

## Detached custody

**Weak example.** A handle moves into a spawned task and its join handle is dropped. **Why it
fails.** No owner observes failure or shutdown. **Risk.** leaked resource and hidden effects.
**Improved direction.** supervisor owns join and cancellation. **Justified appearance.**
Process-lifetime telemetry task with explicit supervisor and loss policy.

## Broad borrowed service

**Weak example.** A function receives `&mut AppContext` to perform one authorized action.
**Why it fails.** It can mutate unrelated state and exercise ambient authority. **Risk.**
confused-deputy behavior. **Improved direction.** pass narrow capability and required data.
**Justified appearance.** tightly scoped internal orchestration where context is the explicit
transaction owner.
