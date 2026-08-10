# Executable doctrine evidence

The workspace examples demonstrate exact local guarantees and explicit limits.
The first eight are listed in their intended learning order, which is the order
[`src/lib.rs`](src/lib.rs) records as `EXAMPLE_PACKAGES`; the ninth is the
inventory crate that holds that list.

## Package contents

| Crate                                                    | Evidence                                                                                 |
| -------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| [`domain-modeling`](domain-modeling/README.md)           | positive currency-tagged money, checked addition, invoice sum type                       |
| [`validated-newtypes`](validated-newtypes/README.md)     | private checked values and verifier-owned email evidence                                 |
| [`typestate`](typestate/README.md)                       | fallible local connection and consuming transaction protocols                            |
| [`staged-protocol`](staged-protocol/README.md)           | staged advancement, successor capabilities, and recovery edges                           |
| [`boundary-validation`](boundary-validation/README.md)   | checked Serde and database-row conversions                                               |
| [`distributed-outcomes`](distributed-outcomes/README.md) | explicit outcomes and identity-preserving retry decisions                                |
| [`unsafe-evidence`](unsafe-evidence/README.md)           | panic-safe partial initialization and Miri evidence                                      |
| [`doctrine-compile-fail`](compile-fail/README.md)        | compiler rejection of forged values and illegal sequencing                               |
| [`doctrine-examples`](src/lib.rs)                        | the inventory that keeps this table, the package names, and workspace membership aligned |

`doctrine-examples` is the crate this directory itself compiles to. It holds no
domain model. It carries the inventory test that fails when an example crate is
added beside it without being named in `EXAMPLE_PACKAGES` and in the workspace
member list, and the evidence-of-absence trio named by
[RUST-DOC-0008](../doctrines/0008-testing-and-evidence/README.md), which shows a
vacuous assertion passing on data that violates the invariant it claims to
check.

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

The code is licensed under [`LICENSE-CODE`](../LICENSE-CODE). Explanatory prose
and case studies are licensed under [`LICENSE-DOCS`](../LICENSE-DOCS).
