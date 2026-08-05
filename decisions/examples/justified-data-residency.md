---
id: ADR-EXAMPLE-0001
title: Example of a justified record — subscriber data stays inside one jurisdiction
status: example
owner: example-platform-governance
scope: example-data-residency
created: 2026-08-04
executable_authority:
  - deploy/policy/storage-regions.yaml
  - deploy/policy/replica-placement.yaml
revalidate_on:
  - regulator-guidance-change
  - customer-contract-renewal
obsolete_when:
  - the residency obligation is withdrawn by both the regulator and the contract
---

# Example: subscriber data stays inside one jurisdiction

**This is an illustration, not a record.** It records no obligation of this repository, describes
a system that does not exist here, and names no real organization. It is absent from
`manifest/decision-records.yaml` and its identifier does not match the pattern the registry
requires. It exists to show what surviving the test in
[RUST-DOC-0011](../../doctrines/0011-executable-narrative/) looks like.

## The one question

May subscriber records be stored or replicated outside Indonesia?

## Why this cannot be executable

Three parts of this decision are enforceable and were made so. Which regions may hold data,
which storage endpoints are reachable, and where replicas may be placed are all expressible as
machine-checked deployment policy, and the deployment fails when the policy is violated.

One part is not. The obligation arises from a national data-protection requirement as interpreted
by counsel, reinforced by a term in one customer's contract. The interpretation is what makes the
policy the shape it is, and it lives in neither the policy file nor the code. A future engineer
reading `storage-regions.yaml` learns which regions are permitted; nothing in it says whether the
restriction is a legal obligation, a performance choice, or a preference somebody once expressed,
and those three lead to very different decisions when a fourth region is proposed.

A generated view is insufficient for the same reason. Generation could render the permitted region
set in any format; it cannot generate an interpretation of a regulation, because no machine-
readable source in this system contains one.

## What this record does not govern

It does not govern the deployment topology, the choice of storage technology, replica counts,
backup schedules, latency targets, or which regions are permitted at any given moment. Those are
carried by the policy files linked below and change without touching this record.

It does not govern data classification. Which records count as subscriber data is enforced by the
type of the values at the persistence boundary, and a change there is a code change, not a
decision-record change.

It does not extend to other jurisdictions, other customers, or other data classes. A comparable
obligation elsewhere is a separate record with its own owner and its own end condition.

## Current authority

`deploy/policy/storage-regions.yaml` and `deploy/policy/replica-placement.yaml` are authoritative
for what the system currently does. This record is authoritative for nothing operational; it
explains why those files have the contents they have, and who is accountable if the obligation
they encode turns out to be misread.

## Consequence and accepted risk

Cross-region failover into a permitted region only, which raises the recovery time objective for a
whole-region outage. The residual risk is a longer outage in exchange for the obligation being
met, and it was accepted by the named owner.

## Revalidation and end

Revalidated when the regulator publishes changed guidance, and at each renewal of the customer
contract. It becomes obsolete when both the regulatory requirement and the contractual term are
withdrawn, at which point the owner deletes it or moves it to the archive with a reason. It does
not stay active because nobody revisited it.

## Rejected alternatives

Storing encrypted subscriber data outside the jurisdiction with keys held inside it was proposed
and rejected. The rejection depends on counsel's reading of whether encrypted storage constitutes
processing under the requirement, which is exactly the kind of evidence no artifact carries, and
the alternative is expected to be proposed again whenever a cheaper region appears. That is why
this alternative is recorded and the others discussed at the time are not.
