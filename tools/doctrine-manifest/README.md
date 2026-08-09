# `doctrine-manifest`

The library both binaries decode the manifests through. It is the only crate in
[`tools/`](../README.md) that produces no executable, and it deliberately holds
no filesystem, link, or process logic — every path in the repository is walked
by a caller, never by this crate.

## What it owns

- **The manifest types.** `DoctrineManifest`, `DoctrineEntry`, `AgentManifest`,
  `AgentPack`, and the decision-record registry, as the shapes the YAML under
  [`manifest/`](../../manifest/README.md) deserializes into.
- **The closed vocabularies.** Doctrine status, agent role, verbosity ceiling,
  decision-record status, and evidence kind are Rust enums, not strings. A value
  the schema permits but the enum does not fails to decode.
- **Front-matter parsing.** One implementation, so the linter and the bundler
  cannot disagree about what a package header says.
- **The verbosity annotation grammar and section filter**, which decide what
  each generated pack receives from a source document.

## Why it exists

The two binaries began as siblings with no shared library, and every concept
both needed existed twice. The duplicate parsers had already begun to diverge:
their error text differed before their behavior did, which is the harder failure
to notice, because a green test suite says nothing about the message a
contributor will read.

The vocabularies matter more than the parsing. A manifest `status` decoded as
`String` once let a typo silently drop an entire doctrine from every hydration
bundle with a zero exit code. Decoding it as an enum turns that into a decode
failure at the first read.

## Evidence

Each closed vocabulary is asserted by test against the artifact that owns it —
the `enum` arrays in the JSON Schemas under
[`manifest/schema/`](../../manifest/schema/README.md), and the state directories
under [`rfcs/`](../../rfcs/README.md). A value added to a schema without a
matching Rust variant fails the build, rather than failing to parse inside one
tool and not the other.

```text
cargo test --locked -p doctrine-manifest
```

## Doctrine

The closed-vocabulary decision is recorded in
[RFC-0004](../../rfcs/accepted/RFC-0004-closed-vocabulary-discriminants.md), and
the verbosity ceiling it decodes in
[RFC-0005](../../rfcs/accepted/RFC-0005-verbosity-ceilings-and-reader-facing-material.md).
