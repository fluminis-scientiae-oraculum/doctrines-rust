# Executable doctrine evidence

The workspace examples demonstrate exact local guarantees and explicit limits:

| Crate                   | Evidence                                                           |
| ----------------------- | ------------------------------------------------------------------ |
| `domain-modeling`       | positive currency-tagged money, checked addition, invoice sum type |
| `validated-newtypes`    | private checked values and verifier-owned email evidence           |
| `typestate`             | fallible local connection and consuming transaction protocols      |
| `boundary-validation`   | checked Serde and database-row conversions                         |
| `distributed-outcomes`  | explicit outcomes and identity-preserving retry decisions          |
| `unsafe-evidence`       | panic-safe partial initialization and Miri evidence                |
| `doctrine-compile-fail` | compiler rejection of forged values and illegal sequencing         |

Run all examples from the repository root:

```text
cargo test --workspace --all-features
```

Examples deliberately avoid network and database dependencies so the complete
suite remains deterministic. They model boundary adapters and failure points
with explicit inputs. A real integration must replace those inputs with
protocol- and product-specific evidence and retain the same construction and
uncertainty discipline.

The workspace forbids unsafe code by default. `unsafe-evidence` is a
dependency-free, crate-local exception with a complete safety argument and
private unsafe surface. Run its specialized evidence with the pinned nightly:

```text
cargo +nightly-2026-07-13 miri test --locked -p unsafe-evidence
```

This Miri run covers only the example's exercised paths; it is not universal
soundness evidence.

The code is licensed under `LICENSE-CODE`. Explanatory prose and case studies
are licensed under `LICENSE-DOCS`.
