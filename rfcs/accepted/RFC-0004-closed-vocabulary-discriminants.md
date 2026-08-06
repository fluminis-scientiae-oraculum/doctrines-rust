---
id: RFC-0004
title: Make R002 reach the lone string discriminant its applicability already names
author: doctrines-rust maintainers
status: accepted
created: 2026-08-06
affected_doctrines:
  - RUST-DOC-0001
---

# RFC-0004: Make R002 reach the lone string discriminant its applicability already names

## Summary

`RUST-DOC-0001-R002` will gain a second obligation covering a single field that carries a closed,
known vocabulary. Its applicability already names string discriminants, but its statement governs
only contradictory field combinations that carry state-specific data, so a lone `status` field
over four unit states satisfies the applicability while the statement asks nothing of it. The rule
a reviewer would reach for cannot be cited against the most common stringly-typed-state defect in
Rust.

The rule keeps its identifier, its title, and its position. `RUST-DOC-0001` moves to `0.2.0` and
the repository to `0.5.0`, because the set of conforming systems shrinks.

## Motivation

The defect was found by auditing this repository's own tooling against this corpus. Both tools
decoded a manifest field constrained by JSON Schema to four values as a `String`, then compared it
against string literals at eight sites. A misspelled value matched no comparison, so the bundler
silently excluded a whole doctrine from every hydration bundle and exited zero. Seeding
`status: activ` removed 1901 lines from the generated corpus without a diagnostic.

When that finding was written up, the rule to cite could not be found.

`R002` is titled "Represent mutually exclusive state as a sum type" and names string discriminants
in its applicability, which is exactly the shape of the defect. Its statement, however, opens with
"Contradictory field combinations", and a lone discriminant is not a combination of fields. It
then narrows further to states that "carry state-specific data", and the four manifest statuses
carry none. The rule is applicable and simultaneously vacuous.

`RUST-DOC-0001-R011` covers the case, so nothing in the corpus is unsound. But it covers it as a
recommendation, its applicability enumerates runtime and persistence settings rather than the
general shape, and it is titled for dynamic reality rather than for representation. A reviewer
looking for the obligation will not find it where the corpus has trained them to look.

`foundations/normative-language.md` states that applicability narrows a rule rather than extending
it. An applicability list that names a class the statement cannot reach is therefore a promise the
rule does not keep, and it is the kind of defect that only appears when somebody tries to cite the
rule against real code.

## Proposed normative changes

### Amended rule

| Doctrine/rule      | Current meaning                                                                                                              | Proposed meaning                                                                                                                                                       | Reason                                                                                                                  |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| RUST-DOC-0001-R002 | Contradictory field combinations are replaced by a sum type when states are mutually exclusive and carry state-specific data | The same, plus: a single field selecting among a closed, known set of mutually exclusive alternatives is decoded into a type that cannot hold a value outside that set | The applicability names string discriminants; the statement could not reach one, so the rule was applicable and vacuous |

The statement will read:

```text
Contradictory field combinations MUST be replaced by an enum or equivalent sum type
when domain states are mutually exclusive and carry state-specific data. A single
field whose value selects among a closed, known set of mutually exclusive
alternatives MUST likewise be decoded into a type that cannot hold a value outside
that set, rather than retained as an unconstrained string or integer.
```

The allowed exceptions will read:

```text
A foreign persistence or wire DTO may retain its external shape if it is untrusted
and converted into a validated domain enum before use. A vocabulary too large or too
volatile to enumerate may use a validated newtype that rejects an unknown value at
construction, provided the rejection is tested.
```

Intent and review evidence gain the corresponding clauses. Applicability is unchanged, because it
already named the class; that is the defect being repaired.

### What the amended rule does not require

The obligation is scoped to a discriminant, meaning a field whose value selects which of several
mutually exclusive alternatives applies. It does not reach:

- a format-constrained scalar that selects nothing, such as a semantic version, a path, a title,
  or a schema-version gate with one legal value. There are no alternatives to discriminate
  between, and a pattern or a comparison is the proportionate check;
- an open vocabulary, such as a user-supplied label or tag, where the legal set is not known;
- a closed vocabulary too large or too volatile to enumerate, such as a currency or country code
  list, which the new allowed exception routes to a validated newtype. The obligation is that an
  unknown value is rejected at construction, not that the mechanism is specifically an enum.

This scoping is stated because the rule would otherwise read as a requirement to enumerate every
constrained string in a system, which `foundations/complexity-budget.md` would not support.

## Affected doctrine IDs and artifacts

`RUST-DOC-0001-R002` in `doctrines/0001-invalid-states/doctrine.md`, its gate in the package
review standard, the package front matter and manifest version, `sources/0001-invalid-states/`,
`CHANGELOG.md`, and the regenerated bundles.

No other rule changes. No rule is added, removed, renumbered, or weakened. The corpus keeps 207
normative rules across eleven packages.

## Guarantee ledger impact

| Claim                                                                                  | Before                                                                | After                                                     |
| -------------------------------------------------------------------------------------- | --------------------------------------------------------------------- | --------------------------------------------------------- |
| A closed vocabulary reaching the domain as an unconstrained scalar is a rule violation | not established; `R002` was applicable but vacuous, `R011` advised it | established as an obligation under `R002`                 |
| An unknown value in a closed vocabulary is rejected before use                         | not required                                                          | required, by a type or by a tested validating constructor |
| Every constrained string is an enum                                                    | not claimed                                                           | still not claimed, and explicitly excluded                |

The amendment establishes an obligation. It does not establish that any particular system meets
it, and it makes no claim about vocabularies that are open, large, or volatile.

## Compatibility

Breaking for conformance review, not for code. A system whose closed vocabulary reaches the domain
as an unconstrained scalar was conforming under the old `R002` and is not conforming under the
amended one. That is a minor version change under `foundations/normative-language.md`, which
reserves minor releases for added normative requirements.

No identifier changes, so every existing citation of `RUST-DOC-0001-R002` continues to resolve.
No exception is removed; one is added.

## Migration

An affected system decodes the vocabulary into a sum type at its boundary, or into a validated
newtype where the exception applies, and replaces literal comparisons with matching. Where the
vocabulary is owned by an external schema, the type is checked against that schema rather than
copied from it, on the terms of `RUST-DOC-0011-R004`.

This repository migrated its own tooling in the same change set, which is the worked example.
`RfcMetadata.status` was the one remaining closed vocabulary decoded as a string after the initial
sweep; it is now a type checked against the `rfcs/` state directories that own it.

## Alternatives

**Narrow the applicability instead.** Remove "string discriminants" from `R002` and let `R011`
carry the case as a recommendation. This resolves the inconsistency with a smaller edit and no
version consequence. Rejected because it resolves it in the wrong direction: the empirical
evidence in this repository is a silent corpus deletion caused by exactly this shape, which argues
for a stronger obligation rather than a quieter one.

**Add a new rule.** Introduce `RUST-DOC-0001-R023` for closed-vocabulary discriminants and leave
`R002` alone. Rejected because two rules would then govern one representation question, and a
reviewer would have to know which applies to a discriminant that is also part of a contradictory
combination. The rule count is not the cost; the ambiguity is.

**Raise `R011` to an absolute requirement.** Rejected because `R011` governs where dynamic state
lives, not how a discriminant is represented, and its applicability list is a set of runtime
settings. Raising its force would over-reach into cases where a runtime state machine is genuinely
the right answer.

**Leave it.** Rejected. An applicability list that names a class the statement cannot reach will
send the next reviewer looking for an obligation that is not there, and this one already did.

## Security impact

Indirect and favourable. An unconstrained discriminant that silently fails to match is a
fail-open shape: the manifest defect it was found through caused a document set to be dropped
rather than an error to be raised. Requiring rejection at the boundary converts that class into a
decode failure. The amendment adds no new trust boundary and no new escape hatch beyond the
validated-newtype exception, which requires tested rejection.

## Complexity impact

Low and bounded. The obligation applies where a vocabulary is already closed and known, so the
type enumerates a set the system had to know anyway. The exception exists to keep the rule from
forcing enumeration of large or volatile vocabularies, which is where the cost would stop being
proportionate.

In this repository the change removed code: four vocabularies became types, and eight literal
comparisons and two hand-written validity checks were deleted.

## Evidence plan

The amendment is a review obligation, and no linter decides whether a vocabulary is closed. The
executable evidence is the worked migration:

- `doctrine-manifest` decodes the repository vocabularies as types, so an unknown value fails at
  parse time in every consumer;
- three tests assert the Rust variants against the JSON Schema `enum` arrays that own them, and a
  fourth asserts the RFC status variants against the `rfcs/` state directories;
- one test asserts that a misspelled status fails to parse, which is the defect that motivated
  this proposal;
- `doctrine-lint` continues to require that `RUST-DOC-0001-R002` appear in its package review
  standard, so the amended gate cannot go missing.

`EVIDENCE.md` records that most `RUST-DOC-0001` rules remain judgment obligations. This amendment
does not change that.

## Source provenance

No external material. The defect was found by auditing this repository against its own corpus, and
the reasoning is recorded in `sources/0001-invalid-states/source-notes.md` as an internally
derived refinement. The parse-versus-validate framing is long-standing community practice rather
than a citable source for this specific rule, and is not claimed as one.

## Decision record

Accepted. The amendment is implemented in the same change set as the tooling migration that
exposed the defect, so the corpus and its own code move together rather than leaving the corpus
stating an obligation its tooling does not meet.
