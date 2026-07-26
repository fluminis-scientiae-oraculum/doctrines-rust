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

**Review evidence.** trait implementation audit, redaction tests, exposure call sites, and
storage contract.

## RUST-DOC-0003-R009 — Limit zeroization claims

**Statement.** Zeroization claims MUST state the exact owned buffer cleared and MUST NOT imply
removal of compiler-created copies, allocator remnants, swap, logs, external stores, or prior
serialization unless those paths are controlled and evidenced.

**Intent.** Prevent a local overwrite mechanism from becoming a universal secrecy guarantee.

**Applicability.** Secret memory and cryptographic material.

**Allowed exceptions.** None to claim accuracy.

**Review evidence.** ownership and copy analysis, drop path, memory-locking policy where used,
and explicit non-guarantees.

## RUST-DOC-0003-R010 — Design before `Arc<Mutex<T>>`

**Statement.** `Arc<Mutex<T>>` MUST NOT be the default substitute for identifying ownership,
task responsibility, mutation protocol, lock scope, and shutdown.

**Intent.** Avoid shared mutable bags that compile but hide contention, deadlock, and authority.

**Applicability.** Concurrent shared state and service handles.

**Allowed exceptions.** It MAY be the simplest correct mechanism after the ownership and
synchronization contract is documented.

**Review evidence.** owner, lock invariant, contention and poisoning policy, alternatives, and
tests.

## RUST-DOC-0003-R011 — Justify interior mutability

**Statement.** Interior mutability MUST be justified by a required aliasing contract and MUST
preserve the domain's synchronization and authority invariants.

**Intent.** Prevent `Cell`, `RefCell`, locks, or atomics from bypassing a better ownership
design.

**Applicability.** Mutation through shared references.

**Allowed exceptions.** Local caching or instrumentation MAY use it when invisible to domain
semantics and reentrancy is safe.

**Review evidence.** aliasing rationale, borrow/panic behavior, synchronization, and reentrancy
tests.

## RUST-DOC-0003-R012 — Use lifetimes for real relationships

**Statement.** Lifetime parameters SHOULD express actual borrowing or validity relationships,
not ornamental complexity or an inaccurate claim that an external resource remains valid.

**Intent.** Keep APIs readable and prevent local borrow duration from implying remote
liveness.

**Applicability.** Borrowed views, guards, transactions, callbacks, and FFI.

**Allowed exceptions.** Internal generic abstraction MAY carry a lifetime required by a
dependency, with its relationship documented.

**Review evidence.** referent and duration explanation, escape analysis, and simpler owned
alternative.

## RUST-DOC-0003-R013 — Define cross-task ownership

**Statement.** Transfer of authority or resources across tasks MUST identify the new owner,
completion signal, cancellation behavior, shutdown responsibility, and behavior if the task
is dropped or panics.

**Intent.** Prevent detached custody and resources with no accountable closer.

**Applicability.** Spawned tasks, worker actors, channels carrying handles, and supervisors.

**Allowed exceptions.** Truly process-lifetime services MAY be owned by the process supervisor.

**Review evidence.** task tree, join/abort contract, channel closure, and shutdown tests.

## RUST-DOC-0003-R014 — Keep external authority revalidation explicit

**Statement.** A local capability MUST NOT claim current external authority when revocation,
expiry, tenant membership, or resource ownership can change without local control; current
use MUST revalidate or carry a bounded lease.

**Intent.** Prevent stale authorization.

**Applicability.** Sessions, identity-provider grants, distributed locks, and policy decisions.

**Allowed exceptions.** Immutable operation-scoped grants MAY remain valid for their defined
commit window.

**Review evidence.** lease or recheck boundary, stale-state handling, and revocation race
tests.
