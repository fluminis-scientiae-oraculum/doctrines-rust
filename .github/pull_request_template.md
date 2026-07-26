## Purpose

<!-- Explain the problem, affected recipients, and why this change belongs in the doctrine corpus. -->

## Doctrine scope

- Doctrine IDs:
- Affected rule IDs:
- Change class: normative / non-normative / code evidence / tooling / generated output

## Change contract

- [ ] Normative meaning is unchanged, or the governing RFC is linked below.
- [ ] Any required RFC is accepted and identifies compatibility and migration effects.
- [ ] `manifest/doctrines.yaml` and `manifest/agents.yaml` reflect changed discovery or pack selection.
- [ ] Source notes and attribution identify new external inputs and repository refinements.
- [ ] Examples include positive and negative evidence proportionate to the claim.
- [ ] Compiler `.stderr` changes were inspected for semantic cause, not accepted mechanically.
- [ ] Canonical sources were changed before generated bundles.
- [ ] `cargo run -p bundle-agent-context -- generate` regenerated `dist/`.
- [ ] No file under `dist/` was edited directly.

## Guarantee ledger

| Claim | Established by | Protected construction | Boundary preservation | Escape hatches | Does not prove | Residual runtime risk |
|---|---|---|---|---|---|---|
|  |  |  |  |  |  |  |

## Local validation

Record exact commands and observed outcomes.

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p doctrine-lint -- check
cargo run -p bundle-agent-context -- generate
git diff --exit-code -- dist/
cargo run -p bundle-agent-context -- check
cargo deny check
lychee --no-progress '**/*.md'
git diff --check
```

## Review evidence

- [ ] Applicable review checklists record pass, fail, not applicable, or waiver references.
- [ ] Guarantees and non-guarantees are stated together.
- [ ] Serde, database, messaging, HTTP/RPC, configuration, filesystem, and FFI bypasses were considered where applicable.
- [ ] External timeout, retry, duplicate, and reconciliation semantics remain explicit where applicable.
- [ ] New dependencies have MSRV, license, source, feature, and duplication review.
- [ ] Unsafe code is absent, or doctrine RUST-DOC-0007 proof obligations are satisfied.

## Compatibility, migration, and limitations

<!-- Describe compatibility effects, migration steps, deliberate limits, and remaining uncertainty. -->
