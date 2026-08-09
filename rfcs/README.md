# Doctrine RFC governance

RFCs are durable decision records for changes that alter doctrine contracts or
repository-wide governance. They permit review before normative text, agents,
examples, and generated distributions move together.

## RFC required

An RFC is mandatory for:

- a new doctrine;
- addition of a normative rule;
- normative weakening or removal;
- a new trusted-construction or other escape hatch;
- doctrine deprecation or supersession;
- a change to the meaning of MUST, MUST NOT, SHOULD, SHOULD NOT, or MAY;
- significant generated-pack architecture or selection restructuring;
- documentation or code license change;
- MSRV policy change.

An RFC may also record disputed broad architecture or security choices.

## RFC usually not required

Typographical correction, broken link, source citation improvement,
non-normative example correction, or wording clarification may proceed directly
when reviewers establish that normative meaning is unchanged. If reasonable
readers would change behavior, enforcement, exception, or expected evidence,
the change is normative and requires an RFC.

## Lifecycle

1. Copy [`template.md`](template.md) into [`proposed/`](proposed/) with a unique
   numeric ID and descriptive slug.
2. Complete evidence, compatibility, migration, and impact sections.
3. Open review and link affected doctrine/issue/PR.
4. Record objections and revisions without erasing decision history.
5. Move accepted, rejected, or superseded RFCs into their state directory —
   [`accepted/`](accepted/README.md), [`rejected/`](rejected/), or
   [`superseded/`](superseded/) — with a final decision record.
6. Apply accepted changes in a separate or clearly reviewable implementation,
   updating versions, manifests, source notes, examples, agents, bundles, and
   CHANGELOG.

## Identity and immutability

RFC IDs are never reused. State transitions preserve the same filename and
history. Accepted and rejected RFCs may receive factual annotations or link
repairs, but their decided proposal is not rewritten. A new RFC supersedes an
old decision.

## Decision authority

Repository maintainers identify decision owners according to repository policy.
Acceptance requires review of normative wording, guarantee ledger, complexity,
security, compatibility, evidence, and source provenance. Lack of objection is
not automatic acceptance.

## Relationship to doctrine status

An accepted RFC authorizes implementation; it does not itself activate a
doctrine. Status changes occur when canonical doctrine, manifest, examples,
generated bundles, and validation land. A supersession RFC names replacement
and migration. Rejected RFCs do not change doctrine.

## Markdown configuration

[`.markdownlint.jsonc`](.markdownlint.jsonc) relaxes the single-title rule for
this directory, because [`template.md`](template.md) carries an instructional
title above the replacement title an author fills in.
