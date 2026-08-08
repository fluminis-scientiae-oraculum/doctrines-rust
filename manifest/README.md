# Manifests

> [!IMPORTANT]
> These three files are the machine-readable authority for discovery. Prose elsewhere in the
> repository is a view of them, and `doctrine-lint` fails the build when a view disagrees.

| File                                             | Owns                                                                   | Read by                                 |
| ------------------------------------------------ | ---------------------------------------------------------------------- | --------------------------------------- |
| [`doctrines.yaml`](doctrines.yaml)               | which doctrines exist, their status, version, paths, and relationships | `doctrine-lint`, `bundle-agent-context` |
| [`agents.yaml`](agents.yaml)                     | what each role pack hydrates and the verbosity ceiling it applies      | `doctrine-lint`, `bundle-agent-context` |
| [`decision-records.yaml`](decision-records.yaml) | which decision records are active and which are archived               | `doctrine-lint`                         |

`repository_version` in `doctrines.yaml` is held equal to the workspace package version, so the
two cannot drift apart unnoticed.

Each file validates against a JSON Schema in [`schema/`](schema/README.md), and each constrained
vocabulary is decoded into a Rust type whose variants are asserted against that schema by test. A
value added to a schema without a matching variant fails the build rather than failing to parse
inside a tool.

> [!CAUTION]
> Editing a manifest changes what agents receive and what the linter enforces. Regenerate with
> `cargo run -p bundle-agent-context -- generate` afterwards, or the drift check fails.
