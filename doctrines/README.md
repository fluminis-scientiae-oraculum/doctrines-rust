# Doctrine index

Doctrine packages define normative engineering obligations. Each package has a
metadata-bearing orientation file, stable rule identifiers, rationale, a
decision framework, an auditable review standard, an anti-pattern catalogue, a
focused glossary, and source references. Foundations define shared vocabulary;
patterns offer reusable mechanisms; boundary guides describe where evidence is
established; review procedures operationalize checks.

Every doctrine started at version `0.1.0`; repository and doctrine versions have
since diverged, and each doctrine's current version lives in
[`manifest/doctrines.yaml`](../manifest/doctrines.yaml) and in that package's front matter. A patch release
clarifies without changing normative meaning. A minor release may add normative
rules or substantial compatible guidance. A major release denotes normative
incompatibility, removal, or contract change. Status and supersession are
governed by the manifest and RFC process.

This table is the reader-facing view of `manifest/doctrines.yaml`, and
`doctrine-lint` checks it against the manifest: every active doctrine's
identifier and exact title has to appear here. The root
[`README.md`](../README.md) deliberately links to this index rather than
repeating it.

> [!TIP]
> [`map.md`](map.md) answers the other question this table cannot: which agent
> role packs hydrate each doctrine. It is generated from both manifests, so a
> selection change reaches it without anyone editing it. An unselected doctrine
> is one a pack does not carry, not one its role may disregard; canonical
> sources remain available to load separately.

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

## Choosing a doctrine by situation

The index above is organized by doctrine. This table is organized by the work in
front of you: find the row matching the task at hand, start with its primary
doctrine, and read the last column before assuming that doctrine covers the
whole problem.

> [!NOTE]
> This table is informative and hand-maintained, in the sense
> [`RUST-DOC-0011-R005`](0011-executable-narrative/doctrine.md) permits: a generator would need
> a hand-written description of each package's scope, so it is owned rather than generated.
> Every last-column cell paraphrases or quotes the scope statement in the named package's own
> `README.md`, which remains authoritative. Two packages name no successor doctrine at all, and
> their rows say so instead of inventing one.

| When the work is…                                                                         | Start with                                         | Also load                                                                                                                                                                                                                            | Where that doctrine stops                                                                                                                                                                              |
| ----------------------------------------------------------------------------------------- | -------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| representing a domain invariant: states, values, collections, transitions                 | [RUST-DOC-0001](0001-invalid-states/)              | chosen by risk; this package names companions by domain rather than by identifier                                                                                                                                                    | It names no successor. Its scope note defers to "specialist doctrines for errors, concurrency, persistence, distributed systems, unsafe code, testing, and performance" — pick the matching row below. |
| designing an API's failure vocabulary and its retry guidance                              | [RUST-DOC-0002](0002-error-modeling/)              | [RUST-DOC-0001](0001-invalid-states/), [RUST-DOC-0004](0004-concurrency-and-async/), [RUST-DOC-0005](0005-persistence-boundaries/), [RUST-DOC-0006](0006-distributed-uncertainty/), [RUST-DOC-0008](0008-testing-and-evidence/)      | Not at an ambiguous outcome: `RUST-DOC-0002-R008` keeps that here and forbids collapsing it into confirmed rejection. It defers full distributed outcome modelling to RUST-DOC-0006.                   |
| deciding whether a value should carry authority, custody, or lifecycle completion         | [RUST-DOC-0003](0003-ownership-and-capabilities/)  | [RUST-DOC-0004](0004-concurrency-and-async/)                                                                                                                                                                                         | It names no successor. Its own test for dropping a capability needs both halves: every use performs the same mutable external authorization **and** local possession adds no stable evidence.          |
| work that overlaps in time: tasks, cancellation, capacity, locks, shutdown                | [RUST-DOC-0004](0004-concurrency-and-async/)       | [RUST-DOC-0005](0005-persistence-boundaries/), [RUST-DOC-0006](0006-distributed-uncertainty/), [RUST-DOC-0007](0007-unsafe-rust/), [RUST-DOC-0009](0009-performance-and-measurement/)                                                | "If an operation may already have taken effect, route to RUST-DOC-0006." Distributed idempotency, boundary validation, unsafe review, and measurement belong to 0006, 0005, 0007, and 0009.            |
| data crossing between stored bytes and trusted domain types                               | [RUST-DOC-0005](0005-persistence-boundaries/)      | [RUST-DOC-0006](0006-distributed-uncertainty/)                                                                                                                                                                                       | Partially: distributed delivery and unknown external outcomes are "governed more fully by RUST-DOC-0006", while outbox, inbox, and compensation stay here.                                             |
| an effect crossing a boundary where communication can fail separately from execution      | [RUST-DOC-0006](0006-distributed-uncertainty/)     | [RUST-DOC-0002](0002-error-modeling/), [RUST-DOC-0004](0004-concurrency-and-async/), [RUST-DOC-0005](0005-persistence-boundaries/)                                                                                                   | "Local concurrency rules are in RUST-DOC-0004; persistence coordination is in RUST-DOC-0005; error categories are in RUST-DOC-0002." A request proven never dispatched stays here, as `LocalFailure`.  |
| an unsafe block, raw pointer, FFI surface, or a safe API built over one                   | [RUST-DOC-0007](0007-unsafe-rust/)                 | [RUST-DOC-0003](0003-ownership-and-capabilities/), [RUST-DOC-0004](0004-concurrency-and-async/), [RUST-DOC-0008](0008-testing-and-evidence/), [RUST-DOC-0009](0009-performance-and-measurement/)                                     | A performance justification moves to RUST-DOC-0009, "benchmark and profiler". If the unsafe only silences a borrow error, "redesign ownership first".                                                  |
| choosing which observation supports a claim, and naming the blind spot it leaves          | [RUST-DOC-0008](0008-testing-and-evidence/)        | [RUST-DOC-0007](0007-unsafe-rust/), [RUST-DOC-0009](0009-performance-and-measurement/)                                                                                                                                               | "Performance measurement belongs to RUST-DOC-0009, while unsafe-specific proof obligations belong to RUST-DOC-0007."                                                                                   |
| making or defending a performance claim                                                   | [RUST-DOC-0009](0009-performance-and-measurement/) | [RUST-DOC-0004](0004-concurrency-and-async/), [RUST-DOC-0007](0007-unsafe-rust/), [RUST-DOC-0008](0008-testing-and-evidence/)                                                                                                        | Before it begins: "If no decision changes with the result, do not optimize yet." Correctness stays in scope here; 0008 owns the benchmark-versus-correctness distinction.                              |
| an in-process ordered sequence where each stage establishes a fact a later stage consumes | [RUST-DOC-0010](0010-staged-protocols/)            | [RUST-DOC-0001](0001-invalid-states/), [RUST-DOC-0003](0003-ownership-and-capabilities/), [RUST-DOC-0005](0005-persistence-boundaries/), [RUST-DOC-0006](0006-distributed-uncertainty/), [RUST-DOC-0011](0011-executable-narrative/) | When several actors advance it, the "runtime state model is authoritative (RUST-DOC-0005, RUST-DOC-0006)" — but a local pass still worth enforcing keeps typed stages issued by checked restoration.   |
| deciding where an obligation lives, and whether a decision record is justified            | [RUST-DOC-0011](0011-executable-narrative/)        | [`foundations/complexity-budget.md`](../foundations/complexity-budget.md), which its decision framework requires as an input                                                                                                         | At its first gate: a durable, remote, or externally governed fact leaves to a named external authority (`R014`), and a proposal to change a contract becomes an RFC (`R011`).                          |

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
