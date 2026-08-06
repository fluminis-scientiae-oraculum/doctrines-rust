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

#[cfg(test)]
mod tests {
    use super::{AgentRole, DoctrineStatus, FrontMatterError, Verbosity, front_matter};
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
}
