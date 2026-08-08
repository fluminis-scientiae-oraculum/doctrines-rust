# Repository tools

Two Rust CLIs make doctrine integrity executable.

## `doctrine-lint`

```text
cargo run -p doctrine-lint -- check
```

The command parses both YAML manifests, validates them against Draft 2020-12
JSON Schemas, checks doctrine package anatomy and front matter, checks ID/folder
agreement, rule heading depth, uniqueness, and review-standard citation
coverage. It also compares the repository and Cargo workspace versions,
rejects unsafe manifest paths, enforces the informative/normative language
boundary and structured-field register, scans repository text for forbidden
filler markers, resolves related and agent paths, and verifies generated-file
banners. It prints path-specific diagnostics and exits nonzero on any finding.

## `bundle-agent-context`

```text
cargo run -p bundle-agent-context -- generate
cargo run -p bundle-agent-context -- check
```

`generate` reads canonical sources and both manifests, then writes full,
compact, and role-specific Markdown distributions with a generated warning and
source provenance. It also writes two generated files that live outside `dist/`:
the accepted-RFC index [`rfcs/accepted/README.md`](../rfcs/accepted/README.md), and the doctrine coverage map
[`doctrines/map.md`](../doctrines/map.md), which transposes `doctrine_selections` so a reader can see
which packs carry a given doctrine. Relative canonical links are rewritten for
each output location, and each source is projected at the destination's
verbosity ceiling, except where it states obligations and is projected whole.
Generation rejects unsafe manifest paths, a curated source inventory that
differs from disk, and a verbosity annotation in a file that states obligations.
`check` computes the same bytes without writing and fails on missing, changed,
or unexpected distribution files of any type.

Both tools operate from the repository root, use stable ordering, and have unit
tests. They do not call Git, access the network, or commit changes.
