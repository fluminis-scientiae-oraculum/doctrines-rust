# Database transaction

> [!NOTE]
> Informative. This study applies doctrine to one workflow and decides nothing. The obligations it
> demonstrates are stated in [`doctrines/`](../../doctrines/README.md) with stable rule identifiers.

One transaction that mutates an account and writes an audit record, then either commits or rolls
back. The study is about the handle: what may be done with it in each state, and what a commit
whose acknowledgement never arrived actually establishes.

[Problem](problem.md) →
[naive design](naive.md) →
[improved design](improved.md) →
[remaining uncertainty](remaining-uncertainty.md)

The role each of those four files plays is described once, in
[the case-study index](../README.md), rather than repeated per study.

> [!TIP]
> Executable mechanics live under [`examples/`](../../examples/README.md), and every improved
> design states a guarantee ledger rather than a summary claim.
