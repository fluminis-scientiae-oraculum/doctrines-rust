# Manifest schemas

> [!IMPORTANT]
> Each schema is the sole authority for the vocabulary it declares. A Rust enum that decodes one of
> these vocabularies is a checked view of it, not a second source: tests assert the variants
> against the `enum` arrays here.

| Schema                        | Validates                  | Notable closed vocabularies                                              |
| ----------------------------- | -------------------------- | ------------------------------------------------------------------------ |
| `doctrine.schema.json`        | `../doctrines.yaml`        | doctrine `status`                                                        |
| `agent-pack.schema.json`      | `../agents.yaml`           | pack `id` and `maximum_verbosity`                                        |
| `decision-record.schema.json` | `../decision-records.yaml` | registry membership only; a record's own front matter governs its status |

Validation runs as part of `cargo run -p doctrine-lint -- check`, before any other check, so a
malformed manifest is reported rather than half-interpreted.

> [!CAUTION]
> Adding a value to a closed vocabulary here without adding the matching Rust variant fails a test
> in `doctrine-manifest`. That is deliberate: the alternative is a value that silently fails to
> match at every comparison site, which once removed an entire doctrine from every generated
> bundle while the tool exited zero.
