# Doctrine index

Doctrine packages define normative engineering obligations. Each package has a
metadata-bearing orientation file, stable rule identifiers, rationale, a
decision framework, an auditable review standard, an anti-pattern catalogue, a
focused glossary, and source references. Foundations define shared vocabulary;
patterns offer reusable mechanisms; boundary guides describe where evidence is
established; review procedures operationalize checks.

Every doctrine started at version `0.1.0`; repository and doctrine versions have
since diverged, and each doctrine's current version lives in
`manifest/doctrines.yaml` and in that package's front matter. A patch release
clarifies without changing normative meaning. A minor release may add normative
rules or substantial compatible guidance. A major release denotes normative
incompatibility, removal, or contract change. Status and supersession are
governed by the manifest and RFC process.

This table is the reader-facing view of `manifest/doctrines.yaml`, and
`doctrine-lint` checks it against the manifest: every active doctrine's
identifier and exact title has to appear here. The root `README.md` deliberately
links to this index rather than repeating it.

| ID            | Doctrine                                                                              | Primary concern                                                 |
| ------------- | ------------------------------------------------------------------------------------- | --------------------------------------------------------------- |
| RUST-DOC-0001 | [Making Invalid States Unrepresentable](0001-invalid-states/)                         | invariant discovery, representation, construction, transitions  |
| RUST-DOC-0002 | [Error Modeling as Domain Design](0002-error-modeling/)                               | actionable failures, recovery, source preservation              |
| RUST-DOC-0003 | [Ownership as Authority and Lifecycle](0003-ownership-and-capabilities/)              | custody, exclusivity, capability, resource lifecycle            |
| RUST-DOC-0004 | [Concurrency and Async Correctness](0004-concurrency-and-async/)                      | task ownership, cancellation, backpressure, synchronization     |
| RUST-DOC-0005 | [Persistence Boundaries and Domain Integrity](0005-persistence-boundaries/)           | checked decoding, migration, transactions, durable intent       |
| RUST-DOC-0006 | [Distributed Effects, Uncertainty, and Reconciliation](0006-distributed-uncertainty/) | ambiguous outcomes, retries, duplicates, reconciliation         |
| RUST-DOC-0007 | [Unsafe Rust as a Proof Obligation](0007-unsafe-rust/)                                | soundness, FFI, initialization, aliasing, dynamic evidence      |
| RUST-DOC-0008 | [Testing as Layered Evidence](0008-testing-and-evidence/)                             | claim-linked evidence and proof limits                          |
| RUST-DOC-0009 | [Performance Claims Require Measurement](0009-performance-and-measurement/)           | workloads, profiles, benchmarks, scoped claims                  |
| RUST-DOC-0010 | [Staged Protocols and Successor Capabilities](0010-staged-protocols/)                 | stage evidence, successor capability, branch and recovery edges |
| RUST-DOC-0011 | [Executable Narrative and Minimal Decision Records](0011-executable-narrative/)       | authority partition, duplication, generated views, records      |

## Dependency direction

Doctrines depend on foundations and may reference earlier or adjacent doctrine
rules. Patterns and boundary guides apply doctrines; they do not silently alter
normative meaning. Reviews aggregate rule checks. Case studies demonstrate
combinations and preserve remaining uncertainty.

Normative changes require manifest updates and, where governance requires, an
RFC. Wording corrections must not change rule meaning under the guise of
editing. Generated distributions under `/dist` are derived artifacts and never
the source of a doctrine change.

## Reading strategy

Start with RUST-DOC-0001 for the core reasoning pipeline. Add RUST-DOC-0002 and
RUST-DOC-0003 for failure and authority. Select boundary-specific doctrines
from 0004–0007. Add RUST-DOC-0010 when operation order carries consequence and
one stage's result is another stage's precondition; it depends on 0001 and 0003
and defers its durable half to 0005 and 0006. Read RUST-DOC-0011 before deciding
where an obligation lives or whether a decision record is justified; it governs
the authority partition the rest of the corpus relies on and is the doctrine most
often reached for outside Rust-specific work. Use 0008 for the evidence plan and
0009 whenever performance affects design. A reviewer should read the complete
package for every doctrine that governs a consequential decision, not only its
compact summary.
