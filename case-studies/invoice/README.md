# Invoice

> [!NOTE]
> Informative. This study applies doctrine to one workflow and decides nothing. The obligations it
> demonstrates are stated in [`doctrines/`](../../doctrines/README.md) with stable rule identifiers.

One invoice, one account, one recipient, and a positive amount in one explicit currency. The
lifecycle is pending, paid with a receipt, or failed with a structured reason, and the study
follows what happens when those three are represented as flags that can all be true at once.

[Problem](problem.md) →
[naive design](naive.md) →
[improved design](improved.md) →
[remaining uncertainty](remaining-uncertainty.md)

The role each of those four files plays is described once, in
[the case-study index](../README.md), rather than repeated per study.

> [!TIP]
> Executable mechanics live under [`examples/`](../../examples/README.md), and every improved
> design states a guarantee ledger rather than a summary claim.
