# Authenticated session

> [!NOTE]
> Informative. This study applies doctrine to one workflow and decides nothing. The obligations it
> demonstrates are stated in [`doctrines/`](../../doctrines/README.md) with stable rule identifiers.

A credential verified into an authenticated principal, then used to authorize specific actions.
The study is about what that verification proves at the instant it happened, and about expiry,
revocation, key rotation, and cached policy afterwards.

[Problem](problem.md) →
[naive design](naive.md) →
[improved design](improved.md) →
[remaining uncertainty](remaining-uncertainty.md)

The role each of those four files plays is described once, in
[the case-study index](../README.md), rather than repeated per study.

> [!TIP]
> Executable mechanics live under [`examples/`](../../examples/README.md), and every improved
> design states a guarantee ledger rather than a summary claim.
