# Repository automation

This directory holds what _runs_ the checks. What _defines_ them lives in
[`tools/`](../tools/README.md) and in the doctrine itself. Every workflow here
re-runs a command a contributor can run locally, and the root
[`README.md`](../README.md) carries that local sequence.

## Workflows

| Workflow                                       | Runs on                          | What it gates                                                                                                        |
| ---------------------------------------------- | -------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| [Doctrine CI](workflows/doctrine-ci.yml)       | pull request, push, manual       | Markdown format and lint, Rust format, Clippy, the full test suite, `doctrine-lint`, bundle drift, dependency policy |
| [Rust Examples](workflows/rust-examples.yml)   | pull request, push, manual       | every example crate at the workspace MSRV and at stable, plus Miri on pinned nightly                                 |
| [Links](workflows/links.yml)                   | pull request, push, cron, manual | every Markdown link resolves, including inside the generated bundles                                                 |
| [Release distributions](workflows/release.yml) | a `v*` tag, manual               | the tag agrees with `repository_version`, and the committed bundles still match source                               |

Every workflow also declares `workflow_dispatch`, so a maintainer with write
access can start any of them by hand from the Actions tab — `release.yml`
included, which takes a tag as a free-text input. That matters most for the one
job that escalates, so it is stated rather than left to be discovered in the
YAML.

Three of the four run with read-only permissions. `release.yml` is the
exception: its publish job needs `contents: write`, the grant sits on that job
rather than on the file, and it publishes nothing until the tag matches
`repository_version` and the bundle drift check passes. It builds no artifact of
its own — it packages the `dist/` files already committed at that tag.

`Links` also runs on a schedule, because an external URL can rot without anyone
touching the repository.

## Issue forms

| Form                                                          | Use when                                                        |
| ------------------------------------------------------------- | --------------------------------------------------------------- |
| [Doctrine correction](ISSUE_TEMPLATE/doctrine-correction.yml) | existing text is inaccurate, ambiguous, obsolete, or misleading |
| [Doctrine proposal](ISSUE_TEMPLATE/doctrine-proposal.yml)     | new or changed normative guidance is needed                     |
| [Guarantee overclaim](ISSUE_TEMPLATE/guarantee-overclaim.yml) | a claim is stronger than the evidence behind it                 |

A guarantee overclaim with security consequences goes to the private path in
[`SECURITY.md`](../SECURITY.md) instead.

## Pull requests

[`pull_request_template.md`](pull_request_template.md) asks for the affected
rule identifiers and the exact validation commands that were run. It is one of
four documents the linter counts when it checks that the local validation
sequence appears in exactly one place; the sequence itself is owned by the root
[`README.md`](../README.md), and every other document links to it.

## Markdown configuration

[`.markdownlint.jsonc`](.markdownlint.jsonc) relaxes MD041 for this directory,
because a pull-request body template opens with a body-level section rather than
a top-level heading. The repository-wide configuration is
[`.markdownlint-cli2.jsonc`](../.markdownlint-cli2.jsonc).
