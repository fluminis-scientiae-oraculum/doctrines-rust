# Glossary

Terms whose meaning here is narrower than ordinary architecture usage. Shared vocabulary lives in
the foundations.

**Executable narrative**
: An architectural obligation carried by the mechanism that enforces it, in a form a reader can
follow as the domain's own account of what the system does. Narrower than "self-documenting
code": the obligation is enforced, not merely described, and legibility is a requirement of the
enforcing artifact rather than a hoped-for property of it.

**Authority partition**
: The assignment of each class of architectural claim to exactly one kind of authority. Narrower
than "separation of concerns": the partition is about which artifact settles a disputed claim, not
about how responsibilities are divided.

**Operational authority**
: The artifact that decides what a system currently does, for the claims it mechanically
enforces. Distinct from governance authority, which decides who may change the contract, and from
external authority, which owns durable and remote facts.

**Enforced claim**
: A claim a mechanism rejects violations of. A claim that a mechanism merely describes is not
enforced, however precisely it is written.

**Maintained representation**
: A description of a claim that a person has to update when the claim changes. A generated view
is not a maintained representation; a hand-updated table is, whether or not anyone updates it.

**Derived view**
: A human-readable projection of a machine-readable source, produced by a generator and checked
for drift. A view produced from a hand-written description of the same claim is not derived; the
description is a maintained representation with a generator attached.

**Competing copy**
: A manually maintained artifact that restates an enforced claim as an independently editable
normative source. It is the object of `RUST-DOC-0011-R004`, and it is defined by editability
rather than by format.

**Irrecoverable rationale**
: A reason that cannot be reconstructed safely from the artifacts: an external constraint, a
rejected alternative and the evidence for rejecting it, an irreversible commitment, an accepted
risk, and who accepted it. Narrower than "why the code is like this", most of which the code and
its tests answer.

**Decision record**
: A durable artifact recording irrecoverable rationale for one decision, with an owner, a
revalidation trigger, an obsolescence condition, and links to the artifacts authoritative for
current behavior. Narrower than the general architecture-decision-record practice this doctrine
restricts: here a record is the residue left after the executability test, not the default
artifact of a decision.

**Active set**
: The decision records currently claimed as authority, enumerated in a machine-readable registry.
Membership is what makes a record citable; a file in the archive is not in the active set.

**Archival record**
: A record retained for a stated compatibility or audit obligation, marked as not current
operational authority and excluded from generated agent context.

**Revalidation trigger**
: A named event on whose occurrence a record's constraint is re-examined. A calendar date is one
form; a dependency major version, a regulatory renewal, an architecture-boundary change, or a
migration completion are others.

**Obsolescence condition**
: The stated condition under which a record stops applying and is expired or archived. Distinct
from a revalidation trigger: the trigger prompts a check, the condition ends the record.

**Historical veto**
: The authority a discoverable but unconfirmed record acquires when it is cited against a change.
`RUST-DOC-0011-R010` removes it by requiring current applicability to be confirmed first.

**Executability test**
: The assessment required before a record is created, asking which part of the decision can be
represented, enforced, generated, or recovered from the artifacts, and which part cannot.

**Representation count**
: The number of maintained representations of one claim. It is the number of places a future
change has to be made correctly, and `RUST-DOC-0011-R017` makes it a stated review quantity.

**Labelled inference**
: A reconstructed reason recorded as an inference, with its evidence, alongside a statement that
the governing rationale is unknown. The only permitted form of an unrecorded reason under
`RUST-DOC-0011-R013`.

## Glossary review

- every normative term is defined in the foundations or here;
- no definition implies a stronger guarantee than the artifact establishes;
- the difference between enforced and described is preserved in every entry;
- local vocabulary is marked as local and attributed to its established family;
- abbreviations expand on first use;
- links point to the authoritative rule or foundation.
