# Doctrine coverage map

> [!NOTE]
> This document is informative. It describes which doctrine each generated agent pack
> hydrates, and it decides nothing. `manifest/agents.yaml` is the authority for pack
> composition, and `doctrines/README.md` is the reader-facing index of the corpus.

A doctrine an agent never receives is a doctrine that agent cannot apply. That fact lives in
`manifest/agents.yaml` as six separate `doctrine_selections` lists, which answer "what does the
planner get" readily and "who carries RUST-DOC-0009" only by reading all six. The table below is
the transpose, so the second question is as cheap as the first.

The interesting cells are the empty ones. A blank is not a defect: a pack selects the doctrine its
role acts on, and breadth is a cost paid in every hydration. A blank is a question worth asking
when a role's work would turn on the doctrine it does not carry.

> [!TIP]
> To change what a pack hydrates, edit `doctrine_selections` in `manifest/agents.yaml` and
> regenerate. Editing the generated map directly has no effect and is rejected by the drift check.
