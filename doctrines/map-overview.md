# Doctrine coverage map

> [!NOTE]
> This document is informative. It describes which doctrine each generated agent pack
> hydrates, and it decides nothing. `manifest/agents.yaml` is the authority for pack
> composition, and `doctrines/README.md` is the reader-facing index of the corpus.

A doctrine a pack does not select is one that is not available from that hydration pack alone, and
must be loaded separately from its canonical source. That is a fact about the pack, not about the
role: `agents/shared.md` directs an agent to read the applicable canonical doctrine, and an agent
working inside this repository can open it directly.

The selections live in `manifest/agents.yaml` as six separate `doctrine_selections` lists, which
answer "what does the planner pack carry" readily and "which packs carry RUST-DOC-0009" only by
reading all six. The table below is the transpose, so the second question is as cheap as the
first.

The interesting cells are the excluded ones. An exclusion is not a defect: a pack selects the
doctrine its role acts on, and breadth is a cost paid in every hydration. It is a question worth
asking when a role's routine work would turn on a doctrine it has to fetch separately.

> [!TIP]
> To change what a pack hydrates, edit `doctrine_selections` in `manifest/agents.yaml` and
> regenerate. Editing the generated map directly has no effect and is rejected by the drift check.
