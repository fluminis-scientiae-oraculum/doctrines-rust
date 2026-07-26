# Agent hydration

Agent documents are role overlays for planning, implementation, review, audit,
and maintenance. They point to canonical foundations, doctrine rule IDs,
patterns, boundaries, and review procedures. They do not restate the full corpus.
Deterministic generated packs combine these overlays with selected canonical
sources according to [`../manifest/agents.yaml`](../manifest/agents.yaml).

## Roles

| Role                          | Primary output                                                       |
| ----------------------------- | -------------------------------------------------------------------- |
| [Shared](shared.md)           | common invariant, boundary, evidence, and honesty obligations        |
| [Planner](planner.md)         | invariant inventory, maps, state graph, complexity and evidence plan |
| [Implementer](implementer.md) | protected construction, fallible effects, executable evidence        |
| [Reviewer](reviewer.md)       | claim-versus-evidence decision and remediation                       |
| [Auditor](auditor.md)         | adversarial bypass and overclaim findings                            |
| [Maintainer](maintainer.md)   | governed doctrine evolution and reproducible generation              |

## Hydration model

Every role begins with `shared.md`, then reads its role overlay, declared
foundations, selected doctrine packages, and review procedures in manifest
order. An agent may need additional doctrine when the work crosses another risk
domain. The selection is a minimum, not a prohibition on relevant reading.

Role packs contain source-path headings so agents and humans can trace a
statement back to canonical content. The compact core in
[`compact-core.md`](compact-core.md) supplies deterministic operational
orientation. [`distribution.md`](distribution.md) supplies the generated
distribution README. Neither changes normative rule meaning.

## Agent-output contract

Artifacts should cite doctrine rule IDs when practical and distinguish:

- discovered invariant from selected representation;
- compile-time guarantee from runtime validation;
- local state from external mutable fact;
- confirmed outcome from rejection and unknown outcome;
- evidence observed from assumption;
- normative change from wording correction;
- canonical source from generated bundle.

When evidence is incomplete, agents state the exact limitation and escalation
needed. They do not invent verification, mark uncertainty as failure, or approve
because code compiles.

## Generated content

Never edit `/dist` directly. Change canonical sources or manifest selection,
run:

```text
cargo run -p bundle-agent-context -- generate
cargo run -p bundle-agent-context -- check
```

Review the generated diff for source ordering and meaning. A bundle check must
reproduce exact bytes. Generated packs are distribution conveniences; source
reviews and doctrine changes occur in canonical directories.
