# Maintainer overlay

## Purpose

Evolve doctrine, examples, tooling, manifests, dependencies, and generated
artifacts without eroding normative meaning, provenance, reproducibility, or
MSRV compatibility. Preserve stable doctrine and rule identity so agents and
human reviews can cite durable contracts.

## Change classification

Classify before editing:

- **wording correction** — grammar or clarity with identical normative meaning;
- **non-normative clarification** — rationale/example improvement without new
  obligation;
- **patch doctrine change** — clarification with no normative meaning change;
- **minor doctrine change** — additive normative rule or substantial compatible
  expansion;
- **major doctrine change** — incompatible meaning, removal, or contract
  change;
- **supersession/deprecation** — governed lifecycle transition.

A small diff can be normative. A large generated diff may contain no normative
change. Classification follows meaning, not line count.

## RFC gate

Require an RFC for a new doctrine, normative rule, normative weakening, new
escape hatch, supersession, change to normative term meaning, significant pack
restructuring, license change, or MSRV policy change. The RFC identifies affected
rule IDs, migration, compatibility, complexity, security, evidence, and source
provenance.

Wording corrections and non-normative clarification can proceed directly only
after reviewers confirm meaning is unchanged.

## Doctrine package maintenance

Preserve the eight-file package contract. Keep README metadata and
[`manifest/doctrines.yaml`](../manifest/doctrines.yaml) synchronized: ID, slug, title, status, version, path,
applicability, risks, foundations, relations, and supersession. New rules use
the doctrine's stable prefix and never reuse removed IDs for different meaning.
Deprecated rules retain traceable history.

Update rationale, decision framework, review standard, anti-patterns, glossary,
references, source notes, related patterns/boundaries/case studies, agent
selection, examples, and CHANGELOG according to impact. Do not edit only the
normative paragraph while leaving operational material contradictory.

## Provenance

Source notes are non-normative and distinguish accepted, refined, rejected, and
added ideas. Prefer primary Rust, protocol, database, standards, and foundational
sources. Verify changing facts such as stable toolchain, action versions, and
dependency MSRV. Use short quotations only when necessary; summarize and link.
Never mirror external media or transcripts.

## Generated bundles

Never edit `/dist` directly. After canonical or manifest changes:

```bash
cargo run -p bundle-agent-context -- generate
cargo run -p bundle-agent-context -- check
```

Inspect ordering, banners, source-path headings, role relevance, and compact
operational completeness. Generation must be stable across repeated runs and
independent of map iteration. A role pack should contain shared obligations,
selected canonical rules, its workflow, and applicable reviews without manually
duplicating the corpus.

## Examples and compiler evidence

Keep examples compatible with pinned MSRV and stable toolchain. Every example
has meaningful tests and honest comments. Run format, Clippy with warnings
denied, all features, and compile-fail tests. When compiler diagnostics change,
run the UI suite in overwrite/update mode only to collect candidate output;
inspect every `.stderr` to confirm the intended prohibited program still fails,
then commit.

Do not loosen privacy, trait bounds, or ownership merely to stabilize a
diagnostic. Adjust minimal fixtures if compiler wording changes without
semantic impact.

## Dependencies and tools

Add dependencies only for clear capability. Verify current release, MSRV,
license, source, advisories, feature surface, and duplicate risk. Keep
`Cargo.lock` committed. Update [`deny.toml`](../deny.toml) narrowly when policy changes, never to
silence an unexplained result. Tooling CLIs must perform real validation and
carry unit tests; a success printer is not acceptable.

Toolchain/MSRV changes require compatibility evidence and governance. Edition
and resolver policy remain explicit.

## Schema and manifest maintenance

Validate both YAML manifests against Draft 2020-12 schemas. Ensure related paths
exist, IDs match folders, package files exist, statuses are allowed, and
supersession is coherent. Agent outputs are unique and under `dist/agents`.
Schema changes include fixtures or tool tests proving current documents remain
valid and invalid shapes fail.

## CI and repository hygiene

Workflows remain read-only, least privilege, and never commit or rewrite source.
Pin supported action majors or immutable SHAs according to policy. CI confirms
the complete local suite. Maintain narrow link exclusions, no broad warning
suppression, no hidden credentials, no repository payload indirection, and no
tracked build output.

Before release or push, run the documented commands, regenerate, inspect
`git diff --check`, full file inventory, forbidden-marker scan, secret/PII scan,
and clean status. Claims in a PR body must match commands actually run.

## Decision-record lifecycle

Under RUST-DOC-0011, the record set is maintained, not accumulated:

- revalidate each active record at its stated trigger, and record the confirmation and its date;
- expire or archive a record whose external constraint, commitment, or accepted risk no longer
  applies; a record does not stay active because nobody revisited it;
- move an archived record under `decisions/archive/`, mark it as not current operational
  authority in its own text, and update the registry entry with the reason;
- keep the active set narrow, and keep archived records out of generated agent context;
- regenerate the bundles after any change to the registry or the canonical sources, and inspect
  the generated difference rather than trusting it;
- when retiring a record, check whether the obligation it carried can now be enforced by an
  artifact instead, and prefer that to a replacement record.

Do not let the archive become the default destination. A record whose reason has ended and that
carries no compatibility or audit obligation is deleted.

## Escalation and completion

Escalate uncertain normative meaning, incompatible migration, licensing,
security-sensitive guarantee change, unsafe proof, or policy choice. Present
source evidence and a recommended RFC path.

Maintenance completes when canonical and generated content agree, versions and
provenance are synchronized, examples/MSRV pass, manifests validate, links and
dependency policy pass, the final audit is recorded, and release notes state
real limitations. Do not declare completion while any required check fails.
