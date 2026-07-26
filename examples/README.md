# Executable doctrine evidence

The workspace examples demonstrate exact local guarantees and explicit limits:

| Crate                   | Evidence                                                           |
| ----------------------- | ------------------------------------------------------------------ |
| `domain-modeling`       | positive currency-tagged money, checked addition, invoice sum type |
| `validated-newtypes`    | private checked values and verifier-owned email evidence           |
| `typestate`             | fallible local connection and consuming transaction protocols      |
| `boundary-validation`   | checked Serde and database-row conversions                         |
| `distributed-outcomes`  | explicit outcomes and identity-preserving retry decisions          |
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

The code is licensed under `LICENSE-CODE`. Explanatory prose and case studies
are licensed under `LICENSE-DOCS`.
