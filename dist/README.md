<!--
GENERATED FILE. DO NOT EDIT DIRECTLY.
Canonical sources live under /foundations, /doctrines, /patterns,
 /boundaries, /reviews, and /agents.
-->

# Generated doctrine distributions

---

## Source: `agents/distribution.md`

# Generated distributions

All files under `/dist` are deterministic outputs of
`bundle-agent-context`. Canonical sources live under `/foundations`,
`/doctrines`, `/patterns`, `/boundaries`, `/reviews`, and `/agents`.

Generate with:

```text
cargo run -p bundle-agent-context -- generate
```

Check for drift with:

```text
cargo run -p bundle-agent-context -- check
```

`full-doctrine.md` concatenates repository identity, foundations, active
doctrine packages, patterns, boundary guides, review procedures, and shared
obligations with source-path provenance. `compact-doctrine.md` combines the
compact core, normative rules, central decision material, and core audit gates.
Role packs combine manifest-declared shared/role sources, selected doctrine
rules, and relevant review procedures in stable order.

Do not edit generated files directly. A manual correction will be overwritten
and fails check mode. Change canonical source or manifest ordering, regenerate,
review the diff, and commit both source and generated output.

Generated packs are hydration and distribution artifacts, not a second
normative authority. Resolve ambiguity against canonical source and stable rule
IDs. Every generated file carries a warning banner and source headings.
