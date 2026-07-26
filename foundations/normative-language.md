# Normative language

This corpus uses five uppercase requirement terms deliberately: **MUST**, **MUST NOT**,
**SHOULD**, **SHOULD NOT**, and **MAY**. Their purpose is to make obligations, recommended
engineering judgment, and permitted choices reviewable. Casual emphasis uses ordinary
lowercase language.

The terms follow the general interpretation established by
[RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and clarified by
[RFC 8174](https://www.rfc-editor.org/rfc/rfc8174), with repository-specific governance
defined here.

## Requirement levels

**MUST** states an absolute requirement within the rule's applicability. A conforming design
satisfies it or carries an approved, explicit waiver where the rule permits waiver. Difficulty,
legacy cost, or compiler inconvenience does not silently lower the requirement.

**MUST NOT** states an absolute prohibition within applicability. An apparent workaround that
recreates the prohibited risk under another name is still nonconforming. For example, a rule
against public trusted-value construction is not satisfied by a public `from_raw` method that
performs no validation.

**SHOULD** states the recommended choice when its applicability holds. A different choice can
be conforming only when the work records a concrete reason, analyzes the resulting risk,
identifies compensating evidence where needed, and survives review. “Preference” alone is
insufficient.

**SHOULD NOT** marks a normally prohibited choice whose exceptional use requires the same
documented judgment. The exception must explain why the usual failure mode does not apply or
how another control contains it.

**MAY** grants permission or identifies a genuinely optional mechanism. It does not remove
obligations from other rules. A design MAY use typestate in a suitable local protocol, but it
must still model transition failure honestly and preserve persistence boundaries.

## Vocabulary calibration

Normative vocabulary is selected by meaning, not by a target distribution. A doctrine does
not need to contain every term. `SHOULD NOT` is appropriate only for a normally prohibited
choice that can remain conforming after an explicit risk argument; a strict prohibition with
bounded applicability instead uses `MUST NOT` and its allowed-exceptions field. `MAY` marks a
permission that would otherwise be unclear. Lowercase “may” can still describe uncertainty or
possibility without granting a new permission.

Reviewers examine whether each chosen force matches consequence and available exceptions.
They do not rebalance counts mechanically. Replacing one normative term with another can
change the set of conforming systems and therefore follows the doctrine-change process unless
the edit demonstrably preserves meaning.

## Scope and applicability

Every doctrine rule states applicability. The normative term governs only within that scope,
but applicability is evaluated by system behavior rather than file layout or labels. A
database adapter that constructs a domain value is a trusted-construction path even if it is
placed in an infrastructure crate. A background task that captures payment is an external
effect even if called from a method named `advance`.

“Not applicable” is a review result, not an omission. The reviewer records why the triggering
conditions do not exist. If the system later changes, the applicability decision must be
revisited.

The strongest applicable rule controls when rules overlap. If one doctrine recommends
structured errors and another requires an explicit unknown outcome for an ambiguous effect,
an opaque report alone cannot erase the unknown state. Conflicts between normative rules are
reported as corpus defects and resolved through governance rather than private
interpretation.

## Normative and informative material

Uppercase requirement terms in `doctrine.md` are normative. Stable rule IDs are the citation
unit. Package metadata records whether the doctrine is normative and its lifecycle state.
Repository governance contracts such as `AGENTS.md`, `CONTRIBUTING.md`, and `rfcs/README.md`
may use the same vocabulary for repository operations; those obligations are governance, not
unnumbered doctrine rules. Definition documents may mention the uppercase terms as terms.
Other informative material uses ordinary lowercase language or cites the governing doctrine
rule instead of creating a hidden obligation.

Rationale, glossary entries, source notes, anti-pattern explanation, and ordinary examples are
informative unless a normative rule explicitly incorporates them. Informative material can
clarify intent and expected evidence but cannot create a hidden obligation. Conversely, a
normative rule cannot be evaded by reading its example as the only permitted syntax.

Examples are illustrative unless marked as required evidence. An enum example teaches
mutually exclusive state; it does not require the same variant names in every domain. A
`TryFrom` example teaches checked conversion; a complete smart constructor with equivalent
protection can comply.

Compliance requires satisfying a rule's intent, not merely copying syntax. A private field
does not protect an invariant if public deserialization bypasses the constructor. A consuming
method does not prevent duplicate effects if callers can clone the authority-bearing value.
A test named after a rule is not evidence when it never exercises the violation path.

## Exceptions

A rule's “allowed exceptions” section defines conditions under which its default statement
does not apply or a reviewed deviation may be accepted. Exceptions must be narrow enough to
test. They should name the changed threat or domain assumption, not merely state that the
implementation is special.

If a rule lists no exception, ordinary review cannot invent one. A new escape hatch or
normative weakening requires an RFC. If an emergency requires a temporary deviation, the
waiver records the breach and remediation rather than pretending compliance.

## Waivers

A waiver is explicit, reviewed, scoped, time-aware, and documented. It includes:

- affected doctrine and rule IDs;
- exact code, component, boundary, or deployment scope;
- owner authorized to accept the risk;
- reason compliance is currently impracticable;
- failure consequence and affected users or systems;
- compensating controls and their evidence;
- expiration or reconsideration trigger;
- remediation or removal plan;
- reviewer and approval reference.

Silence, an inline allow attribute, a generic “legacy” label, or a passing CI job is not a
waiver. A waiver does not change the doctrine for other work. Repeated waivers can reveal that
a rule is wrong or adoption is blocked; that observation should trigger doctrine review, not
automatic normalization.

## Rule writing

A normative rule uses one stable ID such as `RUST-DOC-0001-R004` and includes:

- **Statement:** one testable obligation or tightly related contract.
- **Intent:** the failure mode or invariant protected.
- **Applicability:** the systems, paths, or conditions that trigger it.
- **Allowed exceptions:** bounded conditions or “none.”
- **Review evidence:** artifacts and observations sufficient to assess it.

The applicability and review-evidence fields use capitalized noun-phrase lists consistently.
This register keeps machine extraction predictable while the statement, intent, and exception
fields carry complete propositions.

Avoid combining unrelated requirements merely to reduce rule count. Avoid vague verbs such as
“handle appropriately” without defining outcomes. Name owners and failure semantics. A rule
about timeouts should say whether the result is confirmed failure, cancellation, or
indeterminate effect; a rule about validation should name construction and decoding paths.

## Versioning consequences

A patch doctrine version may correct grammar or clarify meaning without changing the set of
conforming systems. A minor version may add normative rules or compatible obligations. A
major version may weaken, remove, or incompatibly reinterpret obligations. While the
repository is pre-1.0, changes still record their semantic category and migration impact;
pre-1.0 is not permission for silent contract change.

Normative language makes review sharper, not automatic. Judgment remains necessary for
classification, applicability, consequence, proportionality, and evidence quality. The terms
ensure that judgment is visible and accountable.
