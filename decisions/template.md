---
id: ADR-NNNN
title: <the one decision question this record answers>
status: active
owner: <accountable role or team>
scope: <narrow subject, matching the registry entry>
created: <YYYY-MM-DD>
executable_authority:
  - <repository-relative path that governs current behavior>
revalidate_on:
  - <named event that prompts re-examination>
obsolete_when:
  - <condition under which this record stops applying>
---

# ADR-NNNN: <the decision question, repeated from the front matter>

## The one question

State the single decision question. A record that answers several questions expires in parts, so
it never expires at all; several decisions are several records.

## Why this cannot be executable

Name the exact fact that cannot be represented, enforced, generated, tested, or recovered from
the artifacts. If naming it is not possible, this record is not written.

Then say why a generated view is insufficient for that fact, since generation is the usual answer
when a fact exists in a machine-readable source but is hard to read.

## What this record does not govern

State the exclusions. Everything adjacent to the decision that the artifacts carry belongs to
them, and listing it here is what keeps this record from growing into an architecture
description.

## Current authority

Link the artifacts that are authoritative for current behavior: the types, schemas, manifests,
policy files, or tests that enforce whatever part of the decision is enforceable. A reader who
wants to know what the system does now reads those, not this file.

## Consequence and accepted risk

State what follows from the decision, what residual risk it leaves, and who accepted that risk.

## Revalidation and end

State the named event that prompts re-examination, the condition under which this record stops
applying, and who performs the deletion or the move to the archive. A record whose reason has
ended is expired or archived under `RUST-DOC-0011-R009`; it does not stay active because nobody
revisited it.

## Rejected alternatives

Record an alternative only where its rejection depends on evidence the implementation does not
carry, and where the alternative is expected to be proposed again. State why the rejection still
holds, not only that it happened.
