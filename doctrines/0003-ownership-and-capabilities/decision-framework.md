# Decision framework

## Determine the authority shape

Ask:

1. Is authority exclusive, shareable, or consumable?
2. Is it local or enforced externally?
3. Who issues it and can callers forge it?
4. Can it be cloned, delegated, serialized, or moved across tasks?
5. Does it expire or revoke?
6. What happens on cancellation, drop, panic, or process loss?
7. Does completion have a fallible external effect?

| Shape                                    | First mechanism                  |
| ---------------------------------------- | -------------------------------- |
| Exclusive local custody                  | owned non-cloneable value        |
| One-time operation                       | consuming token or capability    |
| Temporary read access                    | immutable borrow                 |
| Temporary exclusive mutation             | mutable borrow or scoped guard   |
| Shareable immutable authority            | cloneable scoped capability      |
| Mutable external permission              | runtime recheck or bounded lease |
| Single task owns mutable state           | actor/task ownership             |
| Shared state with short critical section | documented lock                  |

## Capability design

Record issuer, scope fields, operations, transfer, clone, serialization, expiry, revocation,
use count, and audit identity. Prefer separate capability types for materially different
authority instead of a boolean `is_admin` inside a broad context object. Avoid encoding claims
the enforcing service will not honor.

## RAII decision

Use drop for infallible local bookkeeping and best-effort cleanup. Add explicit completion when
failure matters. If forgetting explicit completion is dangerous, mark returned guards
`#[must_use]`, provide consuming methods, test drop fallback, and supervise leaked external
state.

## Shared state decision

Before `Arc<Mutex<T>>`, compare:

- move ownership into one worker;
- partition state by key;
- send commands through a bounded channel;
- use immutable snapshots;
- use a lock with explicit invariant and ordering;
- use atomics for one measured simple state.

Choose the simplest mechanism whose failure, shutdown, and contention behavior is clear.

## Secret decision

Decide who may expose the secret, in what representation, for how long, and to which API. Audit
every trait derive and serialization path. If zeroization is used, enumerate all known copies
and state the uncovered channels.

## Stop conditions

Do not add a capability when every use must perform the same mutable external authorization and
local possession adds no stable evidence. Do not use ownership to imply distributed
exclusivity. Do not add lifetimes that only make signatures harder. Do not share state merely
to avoid choosing an owner.
