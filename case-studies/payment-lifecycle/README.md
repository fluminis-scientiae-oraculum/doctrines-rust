# Payment lifecycle

> [!NOTE]
> Informative. This study applies doctrine to one workflow and decides nothing. The obligations it
> demonstrates are stated in [`doctrines/`](../../doctrines/README.md) with stable rule identifiers.

A payment advancing through business and provider states that do not agree with each other. The
study separates what the application decided from what the provider acknowledged, and keeps the
gap between them representable.

[Problem](problem.md) →
[naive design](naive.md) →
[improved design](improved.md) →
[remaining uncertainty](remaining-uncertainty.md)

The role each of those four files plays is described once, in
[the case-study index](../README.md), rather than repeated per study.

> [!TIP]
> Executable mechanics live under [`examples/`](../../examples/README.md), and every improved
> design states a guarantee ledger rather than a summary claim.
