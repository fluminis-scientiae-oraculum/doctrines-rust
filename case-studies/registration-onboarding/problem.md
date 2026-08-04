# Registration onboarding: problem

## The workflow

An applicant submits an address and a display name. Two entry paths exist: a self-service signup
carrying a bot-challenge identifier, and an invited signup carrying an invitation code and the
account that issued it. Both converge on the same question: is this identity available, and may
this applicant proceed under the policy version currently in force?

A successful attempt ends with an account row, and separately with a welcome notification. A
blocked attempt ends either with a revised submission or with an abandoned one.

## What has to be true, and when

Ordering here is not stylistic. Each step establishes a fact the next one depends on:

- the address must be canonical before it is used as a lookup key, or the availability check asks
  the wrong question;
- availability must be observed before an account identity is allocated, or identities are
  allocated for attempts that cannot complete;
- the policy version in force must be checked against what the applicant accepted, or consent is
  recorded against a version nobody agreed to;
- the durable write must happen before the notification, or an applicant is welcomed to an
  account that does not exist.

Reordering any pair produces a defect that compiles, passes a happy-path test, and appears in
production as a duplicate account, an orphaned identifier, or an unenforceable consent record.

## Trust boundaries

The submission is untrusted input arriving over HTTP. The challenge identifier and invitation
code are untrusted claims until checked. The identity directory is an external read whose answer
is a snapshot, not a lock. The policy version is mutable external state that can change during
the attempt. The account store is authoritative, transactional, and reachable by other writers
including administrative tooling. The notification path is an external effect that cannot be
recalled.

## What makes this a hard case

Three properties interact badly.

**Two entry paths carrying different evidence.** A self-service registration proves a challenge
was answered; an invited registration proves an invitation existed and names its issuer. Both
must reach the same availability check. A single successor type serving both ends up carrying
each proof as an optional field, and every later stage inherits the obligation to re-inspect
them.

**An observation that expires.** Availability is a read. Between the read and the write, another
worker can take the identity. No amount of local type safety changes this, and a design that
presents the checked stage as proof of a reserved identity is making a claim the read cannot
support.

**A genuine third outcome.** The directory can be unreachable. That is neither availability nor
conflict, and folding it into either one is the more dangerous kind of defect: mapping it to
availability advances the protocol on evidence never obtained.

## Doctrines engaged

RUST-DOC-0010 governs the stage graph, the successor capabilities, the branch and recovery edges,
and the point at which the local protocol stops being durable evidence. RUST-DOC-0001 governs the
canonical values and the removal of contradictory representations. RUST-DOC-0002 governs the
failure taxonomy. RUST-DOC-0005 governs the durable write and its concurrency control.
RUST-DOC-0006 governs the notification effect and the outcomes that stay ambiguous.

## What this study does not cover

It does not design the challenge or invitation verification protocols, the notification
transport, or the account schema. It does not claim a specific database product's semantics. The
executable fragment under [`../../examples/staged-protocol`](../../examples/staged-protocol/src/lib.rs)
stops at a persistable value and ships no store, so the durable half of this study is argued and
reviewed rather than executed.
