# Message delivery

> [!NOTE]
> Informative. This study applies doctrine to one workflow and decides nothing. The obligations it
> demonstrates are stated in [`doctrines/`](../../doctrines/README.md) with stable rule identifiers.

An at-least-once broker that redelivers after a crash or a lost acknowledgement, across producers
and partitions that can reorder. The study covers duplicate suppression, ordering scope, and
keeping one malformed message from blocking a hot partition.

[Problem](problem.md) →
[naive design](naive.md) →
[improved design](improved.md) →
[remaining uncertainty](remaining-uncertainty.md)

The role each of those four files plays is described once, in
[the case-study index](../README.md), rather than repeated per study.

> [!TIP]
> Executable mechanics live under [`examples/`](../../examples/README.md), and every improved
> design states a guarantee ledger rather than a summary claim.
