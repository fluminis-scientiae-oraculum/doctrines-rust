# `bundle-agent-context`

The deterministic projector. It reads canonical sources plus both manifests and
writes every generated file in the repository. Nothing it emits is edited by
hand.

```text
cargo run -p bundle-agent-context -- generate
cargo run -p bundle-agent-context -- check
```

`generate` writes. `check` computes the same bytes in memory and writes nothing,
failing on any generated file that is missing, changed, or unexpected — of any
extension, not only Markdown. Both modes share one code path, so `check` cannot
pass on output `generate` would not produce.

## What it emits

- The full and compact doctrine bundles, and one pack per agent role, under
  [`dist/`](../../dist/README.md).
- Two generated files that live outside `dist/`: the accepted-RFC index
  [`rfcs/accepted/README.md`](../../rfcs/accepted/README.md), built from
  [`rfcs/accepted/overview.md`](../../rfcs/accepted/overview.md) and the front
  matter of each accepted RFC; and the doctrine coverage map
  [`doctrines/map.md`](../../doctrines/map.md), built from
  [`doctrines/map-overview.md`](../../doctrines/map-overview.md) and the two
  manifests.

Every output carries a banner naming the sources it was built from. A generated
file outside `dist/` also needs an entry in
[`.prettierignore`](../../.prettierignore) and an exclusion in the
`lint:markdown` script of [`package.json`](../../package.json), because the
formatters would otherwise rewrite bytes the drift check then rejects.

## What it refuses

- A manifest path that escapes the repository root.
- A curated source inventory that differs from disk. Four directories are
  enumerated in hardcoded const arrays —
  [`foundations/`](../../foundations/README.md),
  [`patterns/`](../../patterns/README.md),
  [`boundaries/`](../../boundaries/README.md), and
  [`reviews/`](../../reviews/README.md) — and generation fails unless the on-disk set of
  `.md` files equals the const exactly, naming both what is missing and what is
  unbundled. Adding a file to one of those directories without editing the const
  is a build failure, by design: an unlisted file would otherwise ship to no
  reader.
- A verbosity annotation in a file that states obligations. Such a file is
  projected whole or not at all, never partially.

## Link rewriting

A relative link is resolved from its canonical source, checked against a real
repository path, and re-emitted relative to each output location. That last step
is why a link to a _directory_ under `dist/` cannot appear anywhere: the bundler
rewrites the output's own directory to an empty relative path, which renders as
`(/)` and fails the link check. Link to files inside it instead.

## Evidence

Unit tests cover ordering stability, link rewriting per output, the curated
inventory equality, the withheld-section case where link targets are still
validated, and rejection of an unexpected file under `dist/`.

```text
cargo test --locked -p bundle-agent-context
```
