# Registration onboarding: remaining uncertainty

The improved design removes a specific set of illegal programs. It does not remove the following
facts, and a review that treats the typed protocol as covering them is reading a stronger claim
than the types support.

## The availability window is still open

`AvailableRegistration` records that no conflicting account was visible to one reader at one
moment. Another worker can take the identity immediately afterward. The typed protocol cannot
narrow this window; it only keeps the observation honestly named.

The unique constraint at write time is the actual guarantee. That means the conflicting path is
reachable twice, once at check time and once at write time, and both must route to the same
recovery stage. A design that handles only the first is incomplete in exactly the way its type
signatures will not reveal.

## Consent can go stale between check and write

The policy stage compares an offered version against the version in force when the stage runs. If
policy is updated during the attempt, the stored consent record refers to a superseded version.
Whether that is acceptable is a legal and product question, not a type-system one. The options
are to re-check at write time inside the transaction, to pin the version for the duration of the
attempt, or to accept the drift and record the check time alongside the version.

This package does not choose. It requires the choice to be recorded.

## The durable write outcome can be ambiguous

A connection lost after the insert is sent and before the response arrives leaves the outcome
unknown. The row may exist, or may not. Retrying without an idempotency identity can create a
second account when the constraint permits it, or return a conflict that is actually the
attempt's own earlier success.

This is RUST-DOC-0006 territory. The registration needs a stable attempt identity carried into
the write so a retry can distinguish "someone else took it" from "I already did this." The typed
protocol does not supply that identity and does not claim to.

## The notification is at-least-once, and cannot be recalled

A welcome message sent after a successful write may be delivered more than once if the publisher
retries, or not at all if the process dies between commit and publish. Coupling the intent to the
transaction through an outbox makes the loss case recoverable; it does not make delivery exactly
once. An applicant may receive two welcomes.

## Upstream verification is assumed, not proved

`SelfServiceOrigin` carries a challenge identifier and `InvitedOrigin` carries an invitation code.
Constructing these stages proves the values were present and canonical, not that the challenge was
solved or the invitation was genuine. If those verifications are weak, every stage downstream
inherits the weakness while looking rigorous.

## Address normalization is a local policy

`EmailAddress` establishes this repository's documented policy: trim, and lowercase the domain.
Providers differ on local-part case sensitivity, plus-addressing, and dot handling. Two systems
with different policies will disagree about whether two addresses are the same identity, which
matters precisely because the address is the uniqueness key. Syntax is not ownership, and a
canonical form is not a deliverable mailbox.

## The stage graph is asserted, not validated

The topology assertion proves the documented edges still typecheck. It does not prove the graph
is the right graph. If the business requires a fraud check between availability and policy, no
assertion in this package will notice its absence. That remains a review judgment under
`RUST-DOC-0010-R001`.

## Evidence limits

The example crate uses an in-memory directory and ships no database, broker, or network. Its
tests prove the transitions behave as written on the inputs supplied. The compiler-rejection
cases prove three specific programs are rejected at the pinned diagnostic boundary on both the
pinned toolchain and the minimum supported version.

Nothing here executes a competing writer, a constraint violation, a lost connection, a policy
change mid-attempt, or a duplicate notification. Those obligations are stated in the doctrine and
checked by review gates, and their executable evidence belongs to the consuming system.
