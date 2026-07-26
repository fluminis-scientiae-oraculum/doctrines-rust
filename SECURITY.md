# Security policy

`doctrines-rust` primarily contains engineering doctrine, examples, validation tools, and CI.
Errors can still create real security risk when downstream agents or humans treat an unsound
example, unsafe proof, boundary claim, or error mapping as authoritative.

## Private reporting

Use the repository's GitHub Security Advisory interface for a vulnerability that should not
be public before remediation. Include the affected path and revision, a minimal reproduction,
the violated invariant, expected and actual behavior, likely impact, and any known mitigation.
Do not include live credentials, production data, or unrelated personal information.

Ordinary wording errors and non-sensitive guarantee overclaims can use the public issue forms.
When uncertain whether disclosure creates risk, use private reporting.

## In scope

Reports are especially useful for:

- vulnerable or misleading example code;
- an unsafe block, unsafe function contract, or safe wrapper that fails its safety invariant;
- a trusted type that can be forged through public fields, unchecked construction,
  deserialization, database decoding, FFI, or feature combinations;
- dependency vulnerabilities that affect executable tools or CI;
- command injection, path traversal, symlink, archive, or unbounded-allocation risk in local
  tooling;
- CI supply-chain weaknesses, excessive workflow permissions, mutable third-party action
  risk, or untrusted-code execution with credentials;
- accidental committed secrets or sensitive infrastructure identifiers;
- error text or diagnostics that expose secrets;
- and security-sensitive guarantee overclaims that could cause unsafe downstream decisions.

A claim that “authorization is encoded in a type” is security-sensitive if the capability can
be constructed, cloned, serialized, or retained after revocation contrary to the claim. A
claim that FFI is safe is security-sensitive if unwinding, ownership, layout, nullability, or
thread rules are incomplete.

## Response

Maintainers should acknowledge a private report promptly, reproduce it against an identified
revision, assess affected releases and generated bundles, and agree on disclosure timing.
Remediation may require code, doctrine, examples, manifests, generated distributions, and an
advisory. A narrow wording change is insufficient when the executable construction path
remains unsafe.

No fixed response deadline is promised by this initial release. Reporters should receive
honest status and should be told when a report cannot be reproduced or is outside project
scope.

## Dependency and CI posture

The workspace commits `Cargo.lock`, runs `cargo deny check`, restricts dependency sources, and
uses read-only workflow permissions. These are controls, not proof that dependencies or the
build environment are harmless. Dependency additions require feature, license, MSRV,
maintenance, and transitive-risk review. Action versions are updated deliberately and
workflow changes are reviewed as code.

## Secret response

If a real secret appears in the repository, treat it as compromised. Revoke or rotate it at
the issuing system before relying on history rewriting. Remove the value from the current
tree, assess logs and forks, and coordinate any history repair with maintainers. Deleting a
branch or release does not guarantee that distributed copies disappear.

## Supported versions

Security corrections target the latest repository release and active `main` branch while the
project remains pre-1.0. Older generated bundles may contain superseded claims; consumers
should pin a revision and monitor the changelog and advisories.
