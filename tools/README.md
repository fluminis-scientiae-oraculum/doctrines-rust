# Repository tools

Two Rust CLIs make doctrine integrity executable.

## `doctrine-lint`

```text
cargo run -p doctrine-lint -- check
```

The command parses both YAML manifests, validates them against Draft 2020-12
JSON Schemas, checks doctrine package anatomy and front matter, checks ID/folder
agreement and rule uniqueness, resolves related and agent paths, scans canonical
content for forbidden filler markers, and verifies generated-file banners. It
prints path-specific diagnostics and exits nonzero on any finding.

## `bundle-agent-context`

```text
cargo run -p bundle-agent-context -- generate
cargo run -p bundle-agent-context -- check
```

`generate` reads canonical sources and the agent manifest, then writes full,
compact, and role-specific Markdown distributions with a generated warning and
source provenance. `check` computes the same bytes without writing and fails on
missing, changed, or unexpected distribution files.

Both tools operate from the repository root, use stable ordering, and have unit
tests. They do not call Git, access the network, or commit changes.
