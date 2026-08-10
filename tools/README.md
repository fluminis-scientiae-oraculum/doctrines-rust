# Repository tools

Three Rust crates make doctrine integrity executable: two binaries a contributor
runs, and the library both of them decode the manifests through.

## Package contents

| Crate                                                    | Kind    | Responsibility                                              |
| -------------------------------------------------------- | ------- | ----------------------------------------------------------- |
| [`doctrine-lint`](doctrine-lint/README.md)               | binary  | fails the build when the corpus disagrees with itself       |
| [`bundle-agent-context`](bundle-agent-context/README.md) | binary  | projects canonical sources into the generated distributions |
| [`doctrine-manifest`](doctrine-manifest/README.md)       | library | decodes the manifest vocabularies once, for both binaries   |

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

The same command carries the connectivity gates: every maintained Markdown file
has an inbound link, every backticked path that resolves on disk is also linked,
every workspace crate is linked from prose outside itself, every crate and every
directory holding maintained Markdown has an index, and every remaining file is
either named by some document or registered with a stated reason. Its own
[package README](doctrine-lint/README.md) lists each register, what it exempts,
and what the walk deliberately cannot see.

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

## `doctrine-manifest`

This crate holds no filesystem or link logic. It owns the vocabularies the
manifests are written in — doctrine status, agent role, verbosity ceiling,
decision-record status, evidence kind — as Rust types, together with
front-matter parsing, the verbosity annotation grammar, and the section filter
both binaries apply. Each closed vocabulary is asserted by test against the
artifact that owns it: the `enum` arrays in the JSON Schemas under
[`manifest/schema/`](../manifest/schema/README.md), and the state directories
under [`rfcs/`](../rfcs/README.md).

The two binaries began as siblings with no shared library, and every concept
both needed existed twice. Independently maintained parsers drift in their error
text before they drift in their behavior, which is the harder failure to notice.
A value added to a schema without a matching Rust variant now fails the build,
rather than failing to parse inside one tool and not the other.

Both binaries operate from the repository root and use stable ordering;
`doctrine-manifest` performs no filesystem access at all, so every path it
reasons about was handed to it by a caller. All three have unit tests. None
calls Git, accesses the network, or commits changes.
