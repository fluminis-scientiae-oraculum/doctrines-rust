<!--
GENERATED FILE. DO NOT EDIT DIRECTLY.
Canonical sources live under /foundations, /doctrines, /patterns,
 /boundaries, /reviews, and /agents.
-->

# Generated doctrine distributions

---

## Source: `agents/distribution.md`

# Generated distributions

Everything under `/dist` is a deterministic output of `bundle-agent-context`. Canonical
sources live under `/foundations`, `/doctrines`, `/patterns`, `/boundaries`, `/reviews`,
and `/agents`.

## Getting the bundles

Clone the repository, or download them without cloning:

```bash
curl -fsSLO https://github.com/fluminis-scientiae-oraculum/doctrines-rust/releases/latest/download/doctrines-rust-dist.tar.gz
curl -fsSLO https://github.com/fluminis-scientiae-oraculum/doctrines-rust/releases/latest/download/doctrines-rust-dist.sha256
sha256sum --check --ignore-missing doctrines-rust-dist.sha256
tar --extract --gzip --file doctrines-rust-dist.tar.gz
```

Each release is published from a `v*` tag, and the workflow refuses to publish unless the
tag matches `repository_version` and the committed bundles still match canonical source.
`full-doctrine.md` and `compact-doctrine.md` are also attached individually, so a single
bundle can be fetched without the archive.

## Loading the doctrine into an agent

Pick one file. Do not concatenate several: they overlap heavily, and the overlap is
duplicated rules rather than added coverage.

| You want                                          | Load                                                      |
| ------------------------------------------------- | --------------------------------------------------------- |
| One agent doing one job, and you know which job   | `dist/agents/<role>.md`                                   |
| A general assistant with limited context          | [`dist/compact-doctrine.md`](compact-doctrine.md) |
| Everything, for search, audit, or offline reading | [`dist/full-doctrine.md`](full-doctrine.md)       |

The roles are `planner`, `implementer`, `reviewer`, `auditor`, `maintainer`, and `shared`.
[`shared.md`](../agents/shared.md) is the common obligations every other role already includes; load it alone only
when the agent's job does not match any single role. [`doctrines/map.md`](../doctrines/map.md) in the repository
shows which doctrine each role pack carries.

Sizes are listed at the end of this file. Check them against your context window before
choosing: the full corpus does not fit in most, and a truncated bundle is worse than a
smaller one because the agent cannot tell what it lost.

### Attaching it

The bundles are plain Markdown with no front matter, so anything that accepts a file or a
system prompt accepts them.

- **A project instruction file** — copy the bundle to whatever your tool reads on every
  turn, or reference it from there. Claude Code reads `CLAUDE.md`, Cursor reads
  `.cursor/rules/`, Windsurf reads `.windsurfrules`, Copilot reads
  `.github/copilot-instructions.md`, Codex reads [`AGENTS.md`](../AGENTS.md).
- **A system prompt** — paste the bundle ahead of your own instructions. Put the doctrine
  first so your task-specific text is the more recent context.
- **A retrieval index** — split on the `## Source:` headings. Each carries the canonical
  path it came from, so a retrieved chunk stays attributable.

### Worked example: Claude Code

Claude Code reads `CLAUDE.md` from the project root on every turn. An implementer agent
working in a Rust project:

```bash
mkdir -p .claude/doctrine
cp dist/agents/implementer.md .claude/doctrine/
```

Then put the framing in `CLAUDE.md` and reference the pack rather than pasting it, so the
file stays readable and the pack stays replaceable. Fill in the version you downloaded; it
is the release tag, and also `repository_version` in [`manifest/doctrines.yaml`](../manifest/doctrines.yaml):

```markdown
## Rust engineering doctrine

`.claude/doctrine/implementer.md` is the implementer pack of doctrines-rust <version>. Apply
it to Rust work in this repository.

It is hydration, not authority. Cite rules by their `RUST-DOC-####-R###` identifier. If a
rule seems to forbid something this project needs, quote the rule's allowed exceptions
before concluding it does; do not work around it silently. A doctrine the pack does not
carry is not out of force, it is one this pack omits.
```

### Worked example: Codex

Codex reads `AGENTS.md`. It has no per-file include, so the pack goes in the file:

```bash
{
  echo "# Agent instructions"
  echo
  echo "The doctrine below is the doctrines-rust implementer pack. It is hydration, not"
  echo "authority: cite rules by their RUST-DOC-####-R### identifier, and quote a rule's"
  echo "allowed exceptions before concluding it forbids something."
  echo
  cat dist/agents/implementer.md
} > AGENTS.md
```

Put the framing above the pack, not below it. A reader that truncates keeps the beginning.

### Tell the agent three things alongside it

Without these the agent treats the bundle as complete and current, and it is neither.

1. **The bundle is hydration, not authority.** Where the bundle and the repository
   disagree, the repository wins. Cite rules by their stable `RUST-DOC-####-R###`
   identifier so a claim can be checked against the canonical source.
2. **A role pack is a subset.** A doctrine absent from the pack is not out of force; it is
   a doctrine the pack does not carry, and it can be read from [`doctrines/`](../doctrines/). Each pack
   states its own ceiling and what it withheld in its `## Assembly` section.
3. **The corpus is versioned and moves.** Record which version you loaded. It is in the
   `repository_version` field of `manifest/doctrines.yaml`, and in the [`CHANGELOG.md`](../CHANGELOG.md)
   heading for that release.

### What not to do

Do not edit a bundle before feeding it to an agent. If a rule does not fit your project,
that is a doctrine question with an RFC process behind it, and an edited copy is a second
normative source that no longer matches the identifiers it still uses.

Do not paste a bundle into a chat and then argue with it. The rules carry allowed
exceptions and review evidence; an agent that has the whole rule can apply the exception,
and one working from a summary cannot.

## Regenerating

Generate:

```bash
cargo run -p bundle-agent-context -- generate
```

Check for drift:

```bash
cargo run -p bundle-agent-context -- check
```

`full-doctrine.md` concatenates repository identity, foundations, active doctrine packages,
patterns, boundary guides, review procedures, and shared obligations with source-path
provenance. `compact-doctrine.md` combines the compact core, normative rules, central
decision material, and core audit gates. Role packs combine manifest-declared shared and
role sources, selected doctrine rules, and relevant review procedures in stable order.

Do not edit generated files directly. A manual correction is overwritten and fails check
mode. Change canonical source or manifest ordering, regenerate, review the diff, and commit
both source and generated output.

Generated packs are hydration and distribution artifacts, not a second normative authority.
Resolve ambiguity against canonical source and stable rule IDs. Every generated file
carries a warning banner and source headings.

---

## Sizes

| Distribution | Bytes | Approximate tokens |
| --- | ---: | ---: |
| `dist/agents/shared.md` | 161801 | 40450 |
| `dist/agents/maintainer.md` | 162861 | 40715 |
| `dist/agents/planner.md` | 217475 | 54368 |
| `dist/compact-doctrine.md` | 239726 | 59931 |
| `dist/agents/implementer.md` | 249098 | 62274 |
| `dist/agents/auditor.md` | 257587 | 64396 |
| `dist/agents/reviewer.md` | 275372 | 68843 |
| `dist/full-doctrine.md` | 970170 | 242542 |

The token column divides bytes by a fixed estimate of four. It is a planning figure for choosing a bundle against a context window, not a measurement: a real count depends on the tokenizer.
