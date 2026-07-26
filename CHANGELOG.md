# Changelog

All notable changes are documented here. Repository releases follow semantic versioning while
the corpus is pre-1.0: patch releases preserve normative meaning, minor releases may add
compatible normative requirements, and major releases may change doctrine contracts.

## [0.1.0] — Initial doctrine corpus

### Added

- Shared foundations for normative language, invariant classification, evidence levels, trust
  boundaries, guarantee honesty, and type-system complexity budgeting.
- Nine active doctrine packages: invalid states, error modeling, ownership and capabilities,
  concurrency and async, persistence boundaries, distributed uncertainty, unsafe Rust,
  testing and evidence, and performance measurement.
- Pattern guides for sum types, opaque newtypes, smart constructors, typestate, capabilities,
  consuming transitions, validated collections, hybrid state machines, and explicit
  uncertainty.
- Boundary guides for Serde, database decoding, HTTP/RPC, messaging, configuration,
  filesystems, and FFI.
- Operational review procedures covering design preparation, domain models, boundaries,
  typestate, distributed effects, and final correctness audits.
- Shared, planner, implementer, reviewer, auditor, and maintainer agent role contracts with
  deterministic generated hydration packs.
- End-to-end invoice, payment, transaction, message delivery, authenticated session, and UI
  workflow case studies.
- Stable Rust examples for domain modeling, validated newtypes, typestate, boundary
  validation, and distributed outcomes, plus compiler-rejection tests.
- Machine-readable doctrine and agent manifests with Draft 2020-12 schemas.
- A substantive doctrine linter, deterministic bundle generator, RFC governance, reusable
  authoring templates, and source-provenance packages.
- Generated full, compact, and role-specific distributions with drift detection.
- Read-only GitHub workflows for doctrine validation, stable/MSRV example testing, and
  Markdown formatting, linting, and link checking; structured issue and pull-request forms.
- Reproducible Markdown quality tooling with pinned Node.js, Prettier, and markdownlint-cli2
  versions, a patched transitive override, deterministic formatting policy, advisory audit,
  and a distinct pull-request gate.
- Dual licensing: CC BY 4.0 for documentation and MIT OR Apache-2.0 for code.
