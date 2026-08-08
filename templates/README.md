# Templates

> [!NOTE]
> Informative scaffolding. A template carries no obligation and is never cited as authority. It
> exists so a new package starts with the right shape rather than by copying a neighbour.

| Template                                       | Use when                                                              |
| ---------------------------------------------- | --------------------------------------------------------------------- |
| [`doctrine/`](doctrine/README.md)              | proposing a new doctrine package; it mirrors the eight required files |
| [`doctrine-proposal.md`](doctrine-proposal.md) | drafting the case for a doctrine before writing the package           |
| [`case-study.md`](case-study.md)               | adding a workflow study under [`case-studies/`](../case-studies/)     |

The doctrine template's files are [`README.md`](doctrine/README.md),
[`doctrine.md`](doctrine/doctrine.md), [`rationale.md`](doctrine/rationale.md),
[`decision-framework.md`](doctrine/decision-framework.md),
[`review-standard.md`](doctrine/review-standard.md),
[`anti-patterns.md`](doctrine/anti-patterns.md), [`glossary.md`](doctrine/glossary.md), and
[`references.md`](doctrine/references.md).

[`CONTRIBUTING.md`](../CONTRIBUTING.md) describes what each file of a doctrine package has to
contain. Start from [`doctrine/`](doctrine/README.md) rather than copying an existing package: a
copied package carries the original's rule identifiers, cross-links, and assumptions, and every
one of them has to be found and changed.

> [!TIP]
> This directory is excluded from the repository-wide marker scan precisely because scaffolding
> text is expected here. That exclusion is also why template prose never reaches a generated
> bundle: no manifest references it.
