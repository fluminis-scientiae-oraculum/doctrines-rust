# Architecture decision records

Decision records are governed by [RUST-DOC-0011](../doctrines/0011-executable-narrative/) and are
a last resort. A record is created only for the part of a decision that cannot be represented,
enforced, generated, tested, or recovered from executable and machine-readable artifacts.

## Current state

**This repository holds no active decision records.**

Every architectural obligation it carries is enforced somewhere: rule identifiers and package
structure by `doctrine-lint`, manifest shape by the JSON Schemas, generated distributions by
`bundle-agent-context` and its drift check, protocol claims by the example crates, their contract
assertions, and their committed compiler rejections. The executability test left no residue, so
nothing was written.

That is an ordinary outcome rather than an omission, and it is the outcome
`RUST-DOC-0011-R006` predicts for a repository of this shape. It is a statement about the record
set, not a claim that this repository is under no unrecorded constraint.

## The active set

[`manifest/decision-records.yaml`](../manifest/decision-records.yaml) enumerates the active set
and the archive, and it enumerates **membership only**. Each record's own front matter is the
authority for its identifier, title, owner, scope, executable authorities, revalidation trigger,
and obsolescence condition. The registry states none of those, so the two cannot disagree about a
field only one of them carries.

Membership in the active list is what makes a record citable as authority; a file that is not
listed is not in the active set, whatever it contains.

`doctrine-lint` validates the registry against
[its schema](../manifest/schema/decision-record.schema.json), then opens each listed record and
validates `RUST-DOC-0011-R007` from that record's front matter: a well-formed `ADR-NNNN`
identifier, unique across both sets; a non-empty title, owner, and scope; and for an active
record at least one executable authority that resolves to a real file, at least one revalidation
trigger, and at least one obsolescence condition. An archived record needs a reason in its own
front matter, needs to live under `decisions/archive/`, needs to carry the archival marking in its
own text, and cannot appear in any agent pack.

The one fact both artifacts express is which set a record belongs to: the list it appears in, and
the `status` it declares. That pair is compared mechanically rather than trusted, which is the
checked-view exception `RUST-DOC-0011-R004` allows. A record declaring `status: expired` while
listed as active fails the build.

## Before writing a record

Run the executability test in
[the decision framework](../doctrines/0011-executable-narrative/decision-framework.md), then
answer these. If any answer is unavailable, the record is not written.

1. Which exact fact cannot live in code, schema, tests, manifests, or generated output?
2. Why would a generated view be insufficient?
3. Which future decision becomes unsafe without this record?
4. What event makes this record obsolete?
5. Who owns its deletion or revalidation?
6. Which executable artifact stays authoritative for current behavior?
7. How is accidental duplication of that artifact avoided?

A record is justified only by an external mandate, an irreversible or externally expensive
commitment, a rejected alternative whose rejection depends on evidence the implementation does not
carry, a decision no single system owns, an accepted residual risk or waiver, or a compatibility
obligation created by previously shipped behavior. That a decision is important, was debated, is
complex, or might be forgotten is not a justification.

If the proposal is to change a normative contract rather than to record a fact about one, it is an
RFC and belongs under [`rfcs/`](../rfcs/README.md).

## Files here

| Path                              | Purpose                                       |
| --------------------------------- | --------------------------------------------- |
| [`template.md`](template.md)      | the required shape of a record                |
| [`examples/`](examples/README.md) | one justified and one rejected worked example |
| `active/`                         | active records, when any exist                |
| `archive/`                        | superseded, expired, and archival records     |

The `active/` and `archive/` directories are created when their first record is filed. Examples
are illustrations and are deliberately absent from the registry.

## Citing a record

Under `RUST-DOC-0011-R010`, a record cannot be cited to block or restrict a change until its
governing constraint is confirmed still applicable, its revalidation condition is satisfied, and
the current implementation still depends on it. Discoverability is not authority, and age is not
consent. A citation whose applicability cannot be confirmed is recorded as an open question.
