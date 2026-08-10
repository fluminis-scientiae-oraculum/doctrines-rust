//! Shared decoding of the repository's manifests and Markdown front matter.
//!
//! Both repository tools read `manifest/doctrines.yaml` and `manifest/agents.yaml`.
//! Each previously declared its own structs and compared status fields against string
//! literals, so one constrained vocabulary had three maintained representations: the
//! JSON Schema that owns it, and a `String` plus scattered comparisons in each tool.
//! `bundle-agent-context` never validated against the schema at all, so a misspelled
//! `status` silently excluded a doctrine from every generated bundle.
//!
//! The enums here decode that vocabulary once. An unknown value now fails at parse
//! time in every consumer, and the tests assert the variants against the schema files
//! themselves, so the type is checked against its authority rather than copied from it.

use serde::Deserialize;
use std::fmt;

/// Lifecycle of a doctrine package, owned by `manifest/schema/doctrine.schema.json`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum DoctrineStatus {
    Draft,
    Active,
    Deprecated,
    Superseded,
}

impl DoctrineStatus {
    /// Every status the schema permits, in schema order.
    pub const ALL: [Self; 4] = [
        Self::Draft,
        Self::Active,
        Self::Deprecated,
        Self::Superseded,
    ];

    /// The wire spelling of this status.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Active => "active",
            Self::Deprecated => "deprecated",
            Self::Superseded => "superseded",
        }
    }

    /// Whether this doctrine is in force. Bundles and counted claims about current
    /// rules include only these.
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }
}

impl fmt::Display for DoctrineStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Agent role, owned by `manifest/schema/agent-pack.schema.json`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum AgentRole {
    Shared,
    Planner,
    Implementer,
    Reviewer,
    Auditor,
    Maintainer,
}

impl AgentRole {
    /// Every role the schema permits. The manifest carries each exactly once.
    pub const ALL: [Self; 6] = [
        Self::Shared,
        Self::Planner,
        Self::Implementer,
        Self::Reviewer,
        Self::Auditor,
        Self::Maintainer,
    ];

    /// The wire spelling of this role.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shared => "shared",
            Self::Planner => "planner",
            Self::Implementer => "implementer",
            Self::Reviewer => "reviewer",
            Self::Auditor => "auditor",
            Self::Maintainer => "maintainer",
        }
    }
}

impl fmt::Display for AgentRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Verbosity ceiling for a generated role pack, owned by
/// `manifest/schema/agent-pack.schema.json`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum Verbosity {
    Focused,
    Operational,
    Detailed,
    Exhaustive,
}

impl Verbosity {
    /// Every verbosity the schema permits, in schema order.
    pub const ALL: [Self; 4] = [
        Self::Focused,
        Self::Operational,
        Self::Detailed,
        Self::Exhaustive,
    ];

    /// The wire spelling of this verbosity.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Focused => "focused",
            Self::Operational => "operational",
            Self::Detailed => "detailed",
            Self::Exhaustive => "exhaustive",
        }
    }
}

impl fmt::Display for Verbosity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Lifecycle a decision record declares in its own front matter.
///
/// `manifest/schema/decision-record.schema.json` governs registry membership only, so
/// no schema constrains this vocabulary. This type is therefore its sole authority
/// under `RUST-DOC-0011-R003`, which is why the states are named here rather than
/// left as literals at the comparison sites.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum RecordStatus {
    Active,
    Superseded,
    Expired,
    Archival,
}

impl RecordStatus {
    /// The wire spelling of this status.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Superseded => "superseded",
            Self::Expired => "expired",
            Self::Archival => "archival",
        }
    }
}

impl fmt::Display for RecordStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Lifecycle an RFC declares in its own front matter.
///
/// The authority for this vocabulary is the set of state directories under `rfcs/`,
/// which `rfcs/README.md` describes as the lifecycle an RFC moves through. The test
/// below checks the variants against those directories rather than restating them.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum RfcStatus {
    Accepted,
    Proposed,
    Rejected,
    Superseded,
}

impl RfcStatus {
    /// Every state an RFC can be in, in the order the directories sort.
    pub const ALL: [Self; 4] = [
        Self::Accepted,
        Self::Proposed,
        Self::Rejected,
        Self::Superseded,
    ];

    /// The wire spelling of this status.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Proposed => "proposed",
            Self::Rejected => "rejected",
            Self::Superseded => "superseded",
        }
    }
}

impl fmt::Display for RfcStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// `manifest/doctrines.yaml`.
#[derive(Debug, Deserialize)]
pub struct DoctrineManifest {
    pub schema_version: String,
    pub repository_version: String,
    pub doctrines: Vec<DoctrineEntry>,
}

impl DoctrineManifest {
    /// The doctrines currently in force, in manifest order.
    pub fn active(&self) -> impl Iterator<Item = &DoctrineEntry> {
        self.doctrines
            .iter()
            .filter(|entry| entry.status.is_active())
    }
}

/// One doctrine package's manifest entry.
#[derive(Debug, Deserialize)]
pub struct DoctrineEntry {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub status: DoctrineStatus,
    pub version: String,
    pub package_path: String,
    pub normative_path: String,
    pub applies_to: Vec<String>,
    pub risk_domains: Vec<String>,
    pub foundation_dependencies: Vec<String>,
    pub related_patterns: Vec<String>,
    pub related_boundaries: Vec<String>,
    pub related_case_studies: Vec<String>,
    pub supersedes: Vec<String>,
    pub superseded_by: Option<String>,
}

/// A doctrine package's `README.md` front matter.
#[derive(Debug, Deserialize)]
pub struct DoctrineMetadata {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub status: DoctrineStatus,
    pub version: String,
    pub normative: bool,
    pub applies_to: Vec<String>,
    pub risk_domains: Vec<String>,
    pub supersedes: Vec<String>,
    pub superseded_by: Option<String>,
}

/// `manifest/agents.yaml`.
#[derive(Debug, Deserialize)]
pub struct AgentManifest {
    pub schema_version: String,
    pub packs: Vec<AgentPack>,
}

/// One generated role pack's manifest entry.
#[derive(Debug, Deserialize)]
pub struct AgentPack {
    pub id: AgentRole,
    pub purpose: String,
    pub maximum_verbosity: Verbosity,
    pub ordering: u16,
    pub canonical_sources: Vec<String>,
    pub doctrine_selections: Vec<String>,
    pub review_checklists: Vec<String>,
    pub output_path: String,
}

/// `manifest/decision-records.yaml`. Membership only; each record's own front matter
/// is the authority for its metadata.
#[derive(Debug, Deserialize)]
pub struct DecisionRecordRegistry {
    pub schema_version: String,
    pub active_decision_records: Vec<String>,
    pub archived_decision_records: Vec<String>,
}

/// A decision record's own front matter.
#[derive(Debug, Deserialize)]
pub struct DecisionRecordMetadata {
    pub id: String,
    pub title: String,
    pub status: RecordStatus,
    pub owner: String,
    pub scope: String,
    #[serde(default)]
    pub executable_authority: Vec<String>,
    #[serde(default)]
    pub revalidate_on: Vec<String>,
    #[serde(default)]
    pub obsolete_when: Vec<String>,
    #[serde(default)]
    pub archived_reason: String,
}

/// Why a document's YAML front matter could not be located.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrontMatterError {
    /// The file does not open with a `---` fence.
    Missing,
    /// The opening fence is never closed.
    Unterminated,
}

impl fmt::Display for FrontMatterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Missing => "file must start with YAML front matter",
            Self::Unterminated => "front matter must end with ---",
        })
    }
}

impl std::error::Error for FrontMatterError {}

/// The YAML front matter of a Markdown document, without its fences.
///
/// # Errors
///
/// Returns [`FrontMatterError`] when the document does not open with a `---` fence or
/// never closes it.
pub fn front_matter(text: &str) -> Result<&str, FrontMatterError> {
    let body = text
        .strip_prefix("---\n")
        .ok_or(FrontMatterError::Missing)?;
    let end = body.find("\n---\n").ok_or(FrontMatterError::Unterminated)?;
    Ok(&body[..end])
}

/// The literal that opens any HTML comment, and therefore any candidate annotation.
///
/// The sentinel is the comment opener rather than the word `verbosity`, so a near miss
/// such as `<!-- verbosty: detailed -->` is reported instead of silently doing nothing.
const COMMENT_OPEN: &str = "<!--";

/// The exact opening of a well-formed annotation.
const MARKER_PREFIX: &str = "<!-- verbosity: ";

/// The exact closing of a well-formed annotation.
const MARKER_SUFFIX: &str = " -->";

/// How one source file is projected into one generated output.
///
/// [`SourcePolicy::Normative`] is deliberately distinct from
/// `SourcePolicy::Tiered(Verbosity::Exhaustive)` even though both emit every section. A
/// normative file carries no annotation and one in it is an error, so raising a pack's
/// ceiling can never legalize an annotation in a file that states obligations.
/// `RUST-DOC-0011-R018` requires generated agent context to be built from current
/// authority; a ceiling able to withhold a rule statement would remove an obligation from
/// an agent's view without either party observing it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourcePolicy {
    /// The file states obligations. Every section is emitted and an annotation is an error.
    Normative,
    /// The file is filtered at this ceiling. A section is emitted when its tier is at most
    /// the ceiling.
    Tiered(Verbosity),
}

/// Directories whose documents state obligations in lowercase prose.
///
/// `agents/` is here because an overlay is an obligation document, not commentary:
/// `agents/shared.md` is titled "Shared agent obligations" and carries sections named
/// "Boundary obligations", "Forbidden claims", and "Evidence obligations". Omitting it let
/// an annotation withhold those rules from every role pack while both tools exited zero,
/// which is the defect this constant exists to prevent.
const OBLIGATION_DIRECTORIES: &[&str] = &["foundations/", "agents/"];

/// Whether a repository-relative path states obligations, and is therefore projected under
/// [`SourcePolicy::Normative`] into every generated output.
///
/// Four classes qualify, and the wider three are why this is a manifest query rather than a
/// test on the file name. A doctrine's normative file states rules directly. `foundations/`
/// states them in lowercase prose: `foundations/guarantee-honesty.md` requires that every
/// type-level design answer nine named questions, which no scan for uppercase requirement
/// terms detects. `agents/` states the obligations of a role. Every review checklist states
/// the evidence a gate demands. Withholding any of the four would drop an obligation from
/// an agent's view while the receipt described it as detail.
///
/// The consequence is worth stating rather than discovering: every source any role pack
/// currently lists falls into one of these classes, so no section of any role pack is
/// eligible to be withheld. The ceiling still governs `dist/compact-doctrine.md` and any
/// future tierable source, and reader-only prose split out of an overlay into its own file
/// would become eligible. What it does not do is reduce what the role packs carry today.
pub fn states_obligations(
    relative: &str,
    doctrines: &DoctrineManifest,
    agents: &AgentManifest,
) -> bool {
    OBLIGATION_DIRECTORIES
        .iter()
        .any(|directory| relative.starts_with(directory))
        || doctrines
            .doctrines
            .iter()
            .any(|entry| entry.normative_path == relative)
        || agents
            .packs
            .iter()
            .any(|pack| pack.review_checklists.iter().any(|path| path == relative))
}

/// Why a document's verbosity annotations could not be accepted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MarkerError {
    /// An HTML comment that is not a well-formed annotation.
    Malformed { line: usize },
    /// A well-formed annotation naming a verbosity the schema does not declare.
    UnknownTier { line: usize, found: String },
    /// An annotation preceded by whitespace. Prettier preserves an indented comment
    /// inside a list, so this shape reaches the repository and must be rejected rather
    /// than skipped by a column-zero scan.
    Indented { line: usize },
    /// An annotation whose nearest preceding non-blank line is not a heading.
    Unanchored { line: usize },
    /// An annotation on a level-one heading. The document title is not a section.
    OnDocumentTitle { line: usize },
    /// An HTML comment inside YAML front matter, where the syntax has no meaning.
    InFrontMatter { line: usize },
    /// A nested annotation weaker than the section enclosing it. The effective tier would
    /// then depend on ancestry rather than on the annotation a reader can see.
    NotMonotone {
        line: usize,
        tier: Verbosity,
        enclosing: Verbosity,
    },
    /// An annotation in a file that states obligations.
    ForbiddenInNormative { line: usize },
}

impl MarkerError {
    /// The line the annotation was found on.
    pub const fn line(&self) -> usize {
        match self {
            Self::Malformed { line }
            | Self::UnknownTier { line, .. }
            | Self::Indented { line }
            | Self::Unanchored { line }
            | Self::OnDocumentTitle { line }
            | Self::InFrontMatter { line }
            | Self::NotMonotone { line, .. }
            | Self::ForbiddenInNormative { line } => *line,
        }
    }
}

impl fmt::Display for MarkerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Malformed { line } => write!(
                formatter,
                "HTML comment at line {line} is not a verbosity annotation"
            ),
            Self::UnknownTier { line, found } => write!(
                formatter,
                "verbosity annotation at line {line} names {found:?}, which the schema does not declare"
            ),
            Self::Indented { line } => write!(
                formatter,
                "verbosity annotation at line {line} must start at the beginning of the line"
            ),
            Self::Unanchored { line } => write!(
                formatter,
                "verbosity annotation at line {line} must follow a heading of level two to six"
            ),
            Self::OnDocumentTitle { line } => write!(
                formatter,
                "verbosity annotation at line {line} annotates the document title rather than a section"
            ),
            Self::InFrontMatter { line } => write!(
                formatter,
                "HTML comment at line {line} is inside YAML front matter"
            ),
            Self::NotMonotone {
                line,
                tier,
                enclosing,
            } => write!(
                formatter,
                "verbosity annotation at line {line} is {tier}, weaker than the enclosing {enclosing} section"
            ),
            Self::ForbiddenInNormative { line } => write!(
                formatter,
                "verbosity annotation at line {line} is in a file that states obligations"
            ),
        }
    }
}

impl std::error::Error for MarkerError {}

/// One accepted verbosity annotation and the section it governs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Marker {
    /// Line the annotation itself is on, counting from one.
    pub line: usize,
    /// Line of the heading it annotates, counting from one.
    pub heading_line: usize,
    /// Heading text, without its leading hashes.
    pub heading: String,
    /// Heading level, from two to six.
    pub level: usize,
    /// Verbosity a pack needs in order to receive this section.
    pub tier: Verbosity,
}

/// A section withheld from one generated output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Withheld {
    /// Heading text of the withheld section.
    pub heading: String,
    /// Tier that placed it above the ceiling.
    pub tier: Verbosity,
    /// Line of its heading in the canonical file, counting from one.
    pub line: usize,
}

/// The projection of one source file into one generated output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Filtered {
    /// Body text with annotations stripped and withheld sections replaced by receipts.
    pub text: String,
    /// Every section withheld, in document order.
    pub withheld: Vec<Withheld>,
}

/// An open code fence: its delimiter character and length.
type Fence = (char, usize);

/// The fence a line opens or closes, and whether it carries an info string.
///
/// Both tools must agree exactly on which lines are code, or one could treat an
/// annotation as real while the other treats it as an example. That divergence is the
/// only way this mechanism can lose content silently, so the rule lives here once.
fn fence_delimiter(line: &str) -> Option<(Fence, bool)> {
    let trimmed = line.trim_start_matches(' ');
    if line.len() - trimmed.len() > 3 {
        return None;
    }
    let character = trimmed.chars().next()?;
    if character != '`' && character != '~' {
        return None;
    }
    let length = trimmed
        .chars()
        .take_while(|value| *value == character)
        .count();
    if length < 3 {
        return None;
    }
    let info = trimmed[length..].trim();
    // A backtick fence may not carry a backtick in its info string, so an inline code
    // span is never mistaken for a fence opening.
    if character == '`' && info.contains('`') {
        return None;
    }
    Some(((character, length), info.is_empty()))
}

/// The heading level of an ATX heading line, or `None` for any other line.
fn heading_level(line: &str) -> Option<usize> {
    let trimmed = line.trim_start_matches(' ');
    if line.len() - trimmed.len() > 3 {
        return None;
    }
    let hashes = trimmed.chars().take_while(|value| *value == '#').count();
    if hashes == 0 || hashes > 6 {
        return None;
    }
    let rest = &trimmed[hashes..];
    if rest.is_empty() || rest.starts_with(' ') {
        Some(hashes)
    } else {
        None
    }
}

/// Heading text without its leading hashes or any closing sequence.
fn heading_text(line: &str) -> String {
    line.trim_start_matches(' ')
        .trim_start_matches('#')
        .trim()
        .trim_end_matches('#')
        .trim()
        .to_owned()
}

/// What a line carrying an HTML comment turned out to be.
enum Shape {
    Tier(Verbosity),
    Indented,
    Malformed,
    UnknownTier(String),
}

/// Classify a line already known to contain [`COMMENT_OPEN`].
fn comment_shape(line: &str) -> Shape {
    if line.trim_start() != line || line.trim_end() != line {
        return Shape::Indented;
    }
    let Some(rest) = line.strip_prefix(MARKER_PREFIX) else {
        return Shape::Malformed;
    };
    let Some(name) = rest.strip_suffix(MARKER_SUFFIX) else {
        return Shape::Malformed;
    };
    if name.is_empty() || name.contains(char::is_whitespace) {
        return Shape::Malformed;
    }
    match Verbosity::ALL
        .iter()
        .find(|verbosity| verbosity.as_str() == name)
    {
        Some(verbosity) => Shape::Tier(*verbosity),
        None => Shape::UnknownTier(name.to_owned()),
    }
}

/// Every verbosity annotation in a document, validated against `policy`.
///
/// An annotation is a line that is exactly `<!-- verbosity: T -->` for a verbosity the
/// schema declares, placed directly after the heading of level two to six that it
/// governs. Its scope runs to the next heading of the same or a higher level. A nested
/// annotation may not be weaker than the section enclosing it, so a section's effective
/// tier is the annotation a reader can see rather than a value assembled from ancestry.
///
/// Comments inside fenced code are ignored, so a document can show this syntax without
/// the example being read as an instruction. That is also how `doctrine-lint` treats
/// uppercase requirement terms, and it is what lets an RFC quote the grammar it proposes.
/// The failure direction is deliberate: an ignored annotation emits a section that might
/// have been withheld, never the reverse.
///
/// # Errors
///
/// Returns [`MarkerError`] for any HTML comment outside fenced code that is not a
/// well-formed, correctly anchored, monotone annotation, and for any annotation at all
/// when `policy` is [`SourcePolicy::Normative`].
pub fn markers(text: &str, policy: SourcePolicy) -> Result<Vec<Marker>, MarkerError> {
    let mut found = Vec::new();
    let mut enclosing: Vec<(usize, Verbosity)> = Vec::new();
    let mut anchor: Option<(usize, usize, String)> = None;
    let mut fence: Option<Fence> = None;
    let mut in_front_matter = text.starts_with("---\n");

    for (index, line) in text.lines().enumerate() {
        let number = index + 1;

        if in_front_matter {
            if line.contains(COMMENT_OPEN) {
                return Err(MarkerError::InFrontMatter { line: number });
            }
            if index > 0 && line.trim_end() == "---" {
                in_front_matter = false;
            }
            continue;
        }

        if let Some((delimiter, closing)) = fence_delimiter(line) {
            match fence {
                Some((character, length)) => {
                    if delimiter.0 == character && delimiter.1 >= length && closing {
                        fence = None;
                    }
                }
                None => fence = Some(delimiter),
            }
            anchor = None;
            continue;
        }
        if fence.is_some() {
            continue;
        }

        // The comment check precedes the heading check so that a heading line carrying a
        // trailing comment is examined rather than absorbed. Prettier preserves that
        // shape, so it reaches the repository.
        if line.contains(COMMENT_OPEN) {
            let tier = match comment_shape(line) {
                Shape::Tier(tier) => tier,
                Shape::Indented => return Err(MarkerError::Indented { line: number }),
                Shape::Malformed => return Err(MarkerError::Malformed { line: number }),
                Shape::UnknownTier(found) => {
                    return Err(MarkerError::UnknownTier {
                        line: number,
                        found,
                    });
                }
            };
            if policy == SourcePolicy::Normative {
                return Err(MarkerError::ForbiddenInNormative { line: number });
            }
            let Some((heading_line, level, heading)) = anchor.take() else {
                return Err(MarkerError::Unanchored { line: number });
            };
            if level < 2 {
                return Err(MarkerError::OnDocumentTitle { line: number });
            }
            if let Some((_, outer)) = enclosing.last() {
                if tier < *outer {
                    return Err(MarkerError::NotMonotone {
                        line: number,
                        tier,
                        enclosing: *outer,
                    });
                }
            }
            enclosing.push((level, tier));
            found.push(Marker {
                line: number,
                heading_line,
                heading,
                level,
                tier,
            });
            continue;
        }

        if let Some(level) = heading_level(line) {
            while enclosing.last().is_some_and(|(depth, _)| *depth >= level) {
                enclosing.pop();
            }
            anchor = Some((number, level, heading_text(line)));
            continue;
        }

        if !line.trim().is_empty() {
            anchor = None;
        }
    }

    Ok(found)
}

/// Project a document into one generated output, withholding sections above `ceiling`.
///
/// Annotation lines are removed at every ceiling, so no annotation reaches a generated
/// file. Each maximal run of withheld sections is replaced by a receipt naming the
/// headings, their tiers, and `source`. The receipt is what distinguishes a section that
/// was withheld from one that was never written: without it a reader who sees sections
/// one through eight and ten concludes that nine does not exist.
///
/// # Errors
///
/// Returns [`MarkerError`] when [`markers`] rejects the document.
pub fn filter_by_verbosity(
    text: &str,
    policy: SourcePolicy,
    source: &str,
) -> Result<Filtered, MarkerError> {
    let found = markers(text, policy)?;
    let SourcePolicy::Tiered(ceiling) = policy else {
        return Ok(Filtered {
            text: text.to_owned(),
            withheld: Vec::new(),
        });
    };

    let total = text.lines().count();
    let ends: Vec<usize> = found
        .iter()
        .map(|marker| section_end(text, marker, total))
        .collect();

    // A section above the ceiling is withheld with everything nested inside it. Nesting is
    // monotone, so a nested annotation is never weaker than its parent and can only be
    // withheld when its parent already is.
    let mut hidden = vec![false; total + 1];
    let mut withheld = Vec::new();
    for (marker, end) in found.iter().zip(&ends) {
        if marker.tier <= ceiling || hidden[marker.heading_line] {
            continue;
        }
        hidden[marker.heading_line..=*end].fill(true);
        withheld.push(Withheld {
            heading: marker.heading.clone(),
            tier: marker.tier,
            line: marker.heading_line,
        });
    }

    let annotations: Vec<usize> = found.iter().map(|marker| marker.line).collect();
    let mut output = String::with_capacity(text.len());
    let mut fence: Option<Fence> = None;
    let mut blank = false;
    let mut run: Vec<&Withheld> = Vec::new();

    for (index, line) in text.lines().enumerate() {
        let number = index + 1;
        if hidden[number] {
            if let Some(entry) = withheld.iter().find(|entry| entry.line == number) {
                run.push(entry);
            }
            continue;
        }
        if !run.is_empty() {
            push_receipt(&mut output, &run, ceiling, source, &mut blank);
            run.clear();
        }
        if annotations.contains(&number) {
            continue;
        }

        if let Some((delimiter, closing)) = fence_delimiter(line) {
            match fence {
                Some((character, length)) => {
                    if delimiter.0 == character && delimiter.1 >= length && closing {
                        fence = None;
                    }
                }
                None => fence = Some(delimiter),
            }
        }

        // Removing an annotation leaves the blank lines that surrounded it. Collapsing
        // them keeps the projection deterministic; `dist/` is excluded from Prettier, so
        // nothing downstream would repair it.
        let empty = fence.is_none() && line.trim().is_empty();
        if empty && blank {
            continue;
        }
        blank = empty;
        output.push_str(line);
        output.push('\n');
    }
    if !run.is_empty() {
        push_receipt(&mut output, &run, ceiling, source, &mut blank);
    }

    Ok(Filtered {
        text: output,
        withheld,
    })
}

/// Emit one receipt, keeping a blank line on each side of it so the surrounding document
/// still parses as separate blocks.
fn push_receipt(
    output: &mut String,
    run: &[&Withheld],
    ceiling: Verbosity,
    source: &str,
    blank: &mut bool,
) {
    if !*blank && !output.is_empty() {
        output.push('\n');
    }
    let names: Vec<String> = run
        .iter()
        .map(|entry| format!("{:?} ({})", entry.heading, entry.tier))
        .collect();
    output.push_str("> [!NOTE]\n> Withheld at the `");
    output.push_str(ceiling.as_str());
    output.push_str("` ceiling: ");
    output.push_str(&names.join(", "));
    output.push_str(".\n> Canonical text: `");
    output.push_str(source);
    output.push_str("`.\n\n");
    *blank = true;
}

/// The last line of a marker's section: the line before the next heading of the same or a
/// higher level, or the end of the document.
fn section_end(text: &str, marker: &Marker, total: usize) -> usize {
    let mut fence: Option<Fence> = None;
    for (index, line) in text.lines().enumerate() {
        let number = index + 1;
        if let Some((delimiter, closing)) = fence_delimiter(line) {
            match fence {
                Some((character, length)) => {
                    if delimiter.0 == character && delimiter.1 >= length && closing {
                        fence = None;
                    }
                }
                None => fence = Some(delimiter),
            }
            continue;
        }
        if fence.is_some() || number <= marker.heading_line {
            continue;
        }
        if heading_level(line).is_some_and(|level| level <= marker.level) {
            return number - 1;
        }
    }
    total
}

/// Directory names that are sanctioned scratch space at any depth.
///
/// A contributor's own working notes are not repository content, and a gate that reports
/// them fails on a correct repository inside a sequence the README calls mandatory before
/// every commit. That is the expensive direction: the only escapes are deleting the notes
/// or editing a tool.
///
/// This lives in the shared library because **four** walkers reach a scratch directory —
/// two in `doctrine-lint`, one behind its forbidden-marker scan, and one in
/// `bundle-agent-context`. The constant was first added to a single walker, and the other
/// three kept failing; a convention consulted through one predicate cannot drift the way
/// four copies of a name can.
///
/// It is a declaration, not a reimplementation. `.gitignore` states the same convention as
/// `**/wip/`, and a test in `doctrine-lint` holds the two in agreement. Reading
/// `.gitignore` itself would mean reproducing its pattern language — precedence, negation,
/// anchoring, directory-only matching — which is the unbounded correctness set that got the
/// file-coverage gate deleted in 0.9.0.
pub const SCRATCH_DIRECTORY_NAMES: &[&str] = &["wip"];

/// Whether a path's own final component names sanctioned scratch space.
///
/// Every walker asks this one question, so a name added above reaches all of them at once.
#[must_use]
pub fn is_scratch_directory(path: &std::path::Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| SCRATCH_DIRECTORY_NAMES.contains(&name))
}

#[cfg(test)]
mod tests {
    use super::{
        AgentRole, COMMENT_OPEN, DoctrineStatus, FrontMatterError, MarkerError, RfcStatus,
        SourcePolicy, Verbosity, filter_by_verbosity, front_matter, markers,
    };
    use serde_json::Value;
    use std::fs;
    use std::path::PathBuf;

    /// The schema files these types decode. The repository root is two levels above
    /// this crate.
    fn schema(name: &str) -> Value {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("manifest/schema")
            .join(name);
        let text = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        serde_json::from_str(&text)
            .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()))
    }

    /// The `enum` array a schema declares at a JSON Pointer.
    fn schema_enum(document: &Value, pointer: &str) -> Vec<String> {
        let node = document
            .pointer(pointer)
            .unwrap_or_else(|| panic!("schema has no node at {pointer}"));
        node.get("enum")
            .and_then(Value::as_array)
            .unwrap_or_else(|| panic!("{pointer} declares no enum"))
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .unwrap_or_else(|| panic!("{pointer} enum holds a non-string"))
                    .to_owned()
            })
            .collect()
    }

    /// The schema owns each constrained vocabulary; these types decode it. Asserting
    /// the variants against the schema is what makes the Rust copy a checked view
    /// rather than the second maintained source `RUST-DOC-0011-R004` prohibits. A
    /// value added to a schema without a matching variant fails here, rather than
    /// silently failing to parse in a tool.
    #[test]
    fn doctrine_status_variants_match_the_schema() {
        let declared = schema_enum(
            &schema("doctrine.schema.json"),
            "/$defs/doctrine/properties/status",
        );
        let modelled: Vec<String> = DoctrineStatus::ALL
            .iter()
            .map(|status| status.as_str().to_owned())
            .collect();
        assert_eq!(declared, modelled);
    }

    #[test]
    fn agent_role_variants_match_the_schema() {
        let declared = schema_enum(
            &schema("agent-pack.schema.json"),
            "/$defs/pack/properties/id",
        );
        let modelled: Vec<String> = AgentRole::ALL
            .iter()
            .map(|role| role.as_str().to_owned())
            .collect();
        assert_eq!(declared, modelled);
    }

    #[test]
    fn verbosity_variants_match_the_schema() {
        let declared = schema_enum(
            &schema("agent-pack.schema.json"),
            "/$defs/pack/properties/maximum_verbosity",
        );
        let modelled: Vec<String> = Verbosity::ALL
            .iter()
            .map(|verbosity| verbosity.as_str().to_owned())
            .collect();
        assert_eq!(declared, modelled);
    }

    /// The `rfcs/` state directories are the authority for the RFC lifecycle, so the
    /// variants are checked against them rather than restated from `rfcs/README.md`.
    /// A new state directory without a matching variant fails here.
    #[test]
    fn rfc_status_variants_match_the_state_directories() {
        let rfcs = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("rfcs");
        let mut declared: Vec<String> = fs::read_dir(&rfcs)
            .unwrap_or_else(|error| panic!("read {}: {error}", rfcs.display()))
            .map(|entry| entry.expect("read a directory entry").path())
            .filter(|path| path.is_dir())
            .map(|path| {
                path.file_name()
                    .expect("directory name")
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        declared.sort();

        let modelled: Vec<String> = RfcStatus::ALL
            .iter()
            .map(|status| status.as_str().to_owned())
            .collect();
        assert_eq!(declared, modelled);
    }

    /// The defect that motivated the shared crate: a misspelled status decoded into a
    /// `String` and was silently treated as not-active. It must now fail to parse.
    #[test]
    fn a_misspelled_doctrine_status_fails_to_parse() {
        assert!(serde_json::from_str::<DoctrineStatus>("\"activ\"").is_err());
        assert_eq!(
            serde_json::from_str::<DoctrineStatus>("\"active\"").expect("valid status"),
            DoctrineStatus::Active
        );
    }

    #[test]
    fn front_matter_is_extracted_and_its_failures_are_distinguished() {
        assert_eq!(
            front_matter("---\nid: X\n---\nbody\n").expect("valid front matter"),
            "id: X"
        );
        assert_eq!(front_matter("no fence\n"), Err(FrontMatterError::Missing));
        assert_eq!(
            front_matter("---\nid: X\n"),
            Err(FrontMatterError::Unterminated)
        );
    }

    /// The form Prettier produces. It inserts a blank line on each side of a comment that
    /// sits on its own line, so this is the shape the repository will actually carry and
    /// the one the parser must accept.
    const DOCUMENT: &str = "\
# Title

## Kept

Kept prose.

## Application

<!-- verbosity: detailed -->

Applied prose.

### Nested

<!-- verbosity: exhaustive -->

Nested prose.

## Also kept

Trailing prose.
";

    fn tiered(verbosity: Verbosity) -> SourcePolicy {
        SourcePolicy::Tiered(verbosity)
    }

    #[test]
    fn an_annotation_scopes_the_heading_it_follows_until_the_next_peer() {
        let found = markers(DOCUMENT, tiered(Verbosity::Exhaustive)).expect("valid document");
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].heading, "Application");
        assert_eq!(found[0].level, 2);
        assert_eq!(found[0].tier, Verbosity::Detailed);
        assert_eq!(found[1].heading, "Nested");
        assert_eq!(found[1].level, 3);
        assert_eq!(found[1].tier, Verbosity::Exhaustive);

        let filtered = filter_by_verbosity(DOCUMENT, tiered(Verbosity::Operational), "a.md")
            .expect("valid document");
        assert!(filtered.text.contains("## Kept"));
        assert!(filtered.text.contains("## Also kept"));
        assert!(!filtered.text.contains("## Application"));
        assert!(!filtered.text.contains("### Nested"));
        assert!(!filtered.text.contains("Applied prose."));
        assert!(!filtered.text.contains("Nested prose."));
        assert_eq!(
            filtered.withheld.len(),
            1,
            "the nested section is contained"
        );
        assert_eq!(filtered.withheld[0].heading, "Application");
    }

    /// Each of these survives Prettier unchanged, so each reaches the repository and each
    /// must be rejected rather than silently skipped by a scan anchored at column zero.
    #[test]
    fn every_shape_prettier_preserves_is_rejected() {
        let cases = [
            (
                "## A\n\n- item\n  <!-- verbosity: detailed -->\n",
                "indented",
            ),
            (
                "## A\n\n| <!-- verbosity: detailed --> | y |\n",
                "table cell",
            ),
            ("## A <!-- verbosity: detailed -->\n", "trailing on heading"),
            ("## A\n\n<!--verbosity:detailed-->\n", "no inner spaces"),
            (
                "## A\n\n<!-- verbosity: detailed -->\n<!-- verbosity: detailed -->\n",
                "two adjacent",
            ),
        ];
        for (text, description) in cases {
            assert!(
                markers(text, tiered(Verbosity::Exhaustive)).is_err(),
                "{description} must be rejected"
            );
        }
    }

    #[test]
    fn a_comment_that_is_not_an_annotation_is_reported_by_shape() {
        assert_eq!(
            markers(
                "## A\n\n<!-- editorial note -->\n",
                tiered(Verbosity::Focused)
            ),
            Err(MarkerError::Malformed { line: 3 })
        );
        assert_eq!(
            markers(
                "## A\n\n<!-- verbosity: verbose -->\n",
                tiered(Verbosity::Focused)
            ),
            Err(MarkerError::UnknownTier {
                line: 3,
                found: "verbose".to_owned()
            })
        );
        assert_eq!(
            markers(
                "Prose.\n\n<!-- verbosity: detailed -->\n",
                tiered(Verbosity::Focused)
            ),
            Err(MarkerError::Unanchored { line: 3 })
        );
        assert_eq!(
            markers(
                "# Title\n\n<!-- verbosity: detailed -->\n",
                tiered(Verbosity::Focused)
            ),
            Err(MarkerError::OnDocumentTitle { line: 3 })
        );
        assert_eq!(
            markers(
                "---\nid: X\n<!-- verbosity: detailed -->\n---\n",
                tiered(Verbosity::Focused)
            ),
            Err(MarkerError::InFrontMatter { line: 3 })
        );
        assert_eq!(
            markers(
                "## A\n\n<!-- verbosity: exhaustive -->\n\n### B\n\n<!-- verbosity: focused -->\n",
                tiered(Verbosity::Exhaustive)
            ),
            Err(MarkerError::NotMonotone {
                line: 7,
                tier: Verbosity::Focused,
                enclosing: Verbosity::Exhaustive
            })
        );
    }

    /// A file that states obligations carries no annotation, and raising a ceiling must
    /// never legalize one. `RUST-DOC-0011-R018` is the reason: a withheld rule statement
    /// removes an obligation from an agent's view without either party observing it.
    #[test]
    fn a_normative_file_rejects_every_annotation_including_the_strongest() {
        for verbosity in Verbosity::ALL {
            let text = format!("## A\n\n<!-- verbosity: {verbosity} -->\n\nProse.\n");
            assert_eq!(
                markers(&text, SourcePolicy::Normative),
                Err(MarkerError::ForbiddenInNormative { line: 3 }),
                "{verbosity} must be rejected in a normative file"
            );
        }
    }

    /// A document must be able to show this syntax without the example being read as an
    /// instruction, which is what lets an RFC quote the grammar it proposes. The failure
    /// direction is deliberate: an ignored annotation emits a section that might have been
    /// withheld, never the reverse.
    #[test]
    fn fenced_examples_are_not_instructions() {
        let text = "## A\n\n```text\n<!-- verbosity: exhaustive -->\n```\n\nProse.\n";
        assert!(
            markers(text, SourcePolicy::Normative)
                .expect("fenced examples are ignored")
                .is_empty()
        );
        let filtered =
            filter_by_verbosity(text, tiered(Verbosity::Focused), "a.md").expect("valid document");
        assert!(filtered.text.contains("<!-- verbosity: exhaustive -->"));
        assert!(filtered.withheld.is_empty());
    }

    /// A tilde fence closes only on tildes. A naive toggle shared between the two tools
    /// would end the block on the inner backticks and then read the annotation below as
    /// real, which is the one way this mechanism could withhold content silently.
    #[test]
    fn a_fence_closes_only_on_its_own_delimiter() {
        let text = "## A\n\n~~~text\n```\n<!-- verbosity: exhaustive -->\n~~~\n\nProse.\n";
        assert!(
            markers(text, SourcePolicy::Normative)
                .expect("the annotation stays inside the tilde fence")
                .is_empty()
        );
    }

    #[test]
    fn the_maximum_ceiling_withholds_nothing_and_every_ceiling_strips_annotations() {
        for verbosity in Verbosity::ALL {
            let filtered =
                filter_by_verbosity(DOCUMENT, tiered(verbosity), "a.md").expect("valid document");
            assert!(
                !filtered.text.contains(COMMENT_OPEN),
                "{verbosity} left an annotation in the projection"
            );
            assert!(
                !filtered.text.contains("\n\n\n"),
                "{verbosity} left a blank run behind a stripped annotation"
            );
        }

        let maximum = Verbosity::ALL[Verbosity::ALL.len() - 1];
        let filtered =
            filter_by_verbosity(DOCUMENT, tiered(maximum), "a.md").expect("valid document");
        assert!(filtered.withheld.is_empty());
        assert!(filtered.text.contains("Applied prose."));
        assert!(filtered.text.contains("Nested prose."));
    }

    #[test]
    fn a_receipt_names_each_withheld_section_its_tier_and_the_canonical_file() {
        let filtered = filter_by_verbosity(DOCUMENT, tiered(Verbosity::Focused), "patterns/x.md")
            .expect("valid document");
        assert!(filtered.text.contains("> [!NOTE]"));
        assert!(
            filtered
                .text
                .contains("Withheld at the `focused` ceiling: \"Application\" (detailed)")
        );
        assert!(filtered.text.contains("Canonical text: `patterns/x.md`."));
    }

    /// Consecutive withheld sections become one receipt, so the projection never places
    /// two blockquotes next to each other.
    #[test]
    fn adjacent_withheld_sections_share_one_receipt() {
        let text = "\
# Title

## A

<!-- verbosity: exhaustive -->

First.

## B

<!-- verbosity: exhaustive -->

Second.

## C

Kept.
";
        let filtered =
            filter_by_verbosity(text, tiered(Verbosity::Focused), "a.md").expect("valid document");
        assert_eq!(filtered.text.matches("> [!NOTE]").count(), 1);
        assert!(
            filtered
                .text
                .contains("\"A\" (exhaustive), \"B\" (exhaustive)")
        );
        assert_eq!(filtered.withheld.len(), 2);
    }

    /// Re-projecting a projection must not re-read its own receipts as content to strip.
    #[test]
    fn filtering_is_idempotent() {
        let once = filter_by_verbosity(DOCUMENT, tiered(Verbosity::Focused), "a.md")
            .expect("valid document");
        let twice = filter_by_verbosity(&once.text, tiered(Verbosity::Focused), "a.md")
            .expect("a projection is still a valid document");
        assert_eq!(once.text, twice.text);
        assert!(twice.withheld.is_empty());
    }

    /// `str::lines` strips a trailing carriage return, so a file that arrived with CRLF
    /// endings parses identically rather than failing to match the expected literal by one
    /// invisible byte. Asserted because the alternative — a silent non-match that emits a
    /// section nobody meant to publish — would be indistinguishable from an absent
    /// annotation.
    #[test]
    fn carriage_returns_parse_identically_to_line_feeds() {
        let unix = "## A\n\n<!-- verbosity: detailed -->\n\nProse.\n";
        let windows = "## A\r\n\r\n<!-- verbosity: detailed -->\r\n\r\nProse.\r\n";
        assert_eq!(
            markers(unix, tiered(Verbosity::Focused)).expect("LF parses"),
            markers(windows, tiered(Verbosity::Focused)).expect("CRLF parses")
        );
    }

    /// The schema owns the vocabulary and the parser must accept exactly it. A tier added
    /// to the schema without a variant fails in `verbosity_variants_match_the_schema`; a
    /// spelling the parser accepts but the schema does not fails here.
    #[test]
    fn the_annotation_parser_accepts_exactly_the_schema_vocabulary() {
        for verbosity in Verbosity::ALL {
            let text = format!("## A\n\n<!-- verbosity: {verbosity} -->\n");
            let found = markers(&text, tiered(Verbosity::Exhaustive))
                .unwrap_or_else(|error| panic!("{verbosity} must parse: {error}"));
            assert_eq!(found[0].tier, verbosity);
        }
    }
}
