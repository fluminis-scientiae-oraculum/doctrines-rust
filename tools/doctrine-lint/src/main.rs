use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

const PACKAGE_FILES: &[&str] = &[
    "README.md",
    "doctrine.md",
    "rationale.md",
    "decision-framework.md",
    "review-standard.md",
    "anti-patterns.md",
    "glossary.md",
    "references.md",
];

const GENERATED_BANNER: &str = "<!--\nGENERATED FILE. DO NOT EDIT DIRECTLY.\nCanonical sources \
live under /foundations, /doctrines, /patterns,\n /boundaries, /reviews, and /agents.\n-->\n";

const CANONICAL_ROOTS: &[&str] = &[
    "foundations",
    "doctrines",
    "patterns",
    "boundaries",
    "reviews",
    "agents",
    "case-studies",
    "decisions",
    "rfcs",
    "sources",
];

const DOCTRINE_INDEX: &str = "doctrines/README.md";

const ACTIVE_RECORD_DIRECTORY: &str = "decisions/active/";
const ARCHIVED_RECORD_DIRECTORY: &str = "decisions/archive/";
const ARCHIVAL_MARKER: &str = "NOT CURRENT OPERATIONAL AUTHORITY";

const ROOT_DOCUMENTS: &[&str] = &[
    "README.md",
    "AGENTS.md",
    "CONTRIBUTING.md",
    "CHANGELOG.md",
    "CODE_OF_CONDUCT.md",
    "SECURITY.md",
];

const NORMATIVE_SCOPE_EXCEPTIONS: &[&str] = &[
    "AGENTS.md",
    "CONTRIBUTING.md",
    "foundations/README.md",
    "foundations/normative-language.md",
    "rfcs/README.md",
];

const FORBIDDEN_MARKERS: &[&str] = &[
    concat!("to", "do"),
    concat!("t", "bd"),
    concat!("coming", " soon"),
    concat!("place", "holder"),
    concat!("lorem", " ipsum"),
    concat!("fill this", " later"),
    concat!("st", "ub"),
    concat!("future", " content"),
    concat!("not ", "implemented"),
];

#[derive(Debug)]
struct Diagnostic {
    path: PathBuf,
    message: String,
}

#[derive(Debug, Eq, PartialEq)]
struct RuleHeading {
    id: String,
    level: usize,
    line: usize,
}

impl Diagnostic {
    fn new(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct DoctrineManifest {
    schema_version: String,
    repository_version: String,
    doctrines: Vec<DoctrineEntry>,
}

#[derive(Debug, Deserialize)]
struct DoctrineEntry {
    id: String,
    slug: String,
    title: String,
    status: String,
    version: String,
    package_path: String,
    normative_path: String,
    applies_to: Vec<String>,
    risk_domains: Vec<String>,
    foundation_dependencies: Vec<String>,
    related_patterns: Vec<String>,
    related_boundaries: Vec<String>,
    related_case_studies: Vec<String>,
    supersedes: Vec<String>,
    superseded_by: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DoctrineMetadata {
    id: String,
    slug: String,
    title: String,
    status: String,
    version: String,
    normative: bool,
    applies_to: Vec<String>,
    risk_domains: Vec<String>,
    supersedes: Vec<String>,
    superseded_by: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AgentManifest {
    schema_version: String,
    packs: Vec<AgentPack>,
}

/// The registry enumerates membership only. Each record's own front matter is the
/// authority for its metadata, so the two cannot disagree about fields only one of
/// them carries.
#[derive(Debug, Deserialize)]
struct DecisionRecordRegistry {
    schema_version: String,
    active_decision_records: Vec<String>,
    archived_decision_records: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct DecisionRecordMetadata {
    id: String,
    title: String,
    status: String,
    owner: String,
    scope: String,
    #[serde(default)]
    executable_authority: Vec<String>,
    #[serde(default)]
    revalidate_on: Vec<String>,
    #[serde(default)]
    obsolete_when: Vec<String>,
    #[serde(default)]
    archived_reason: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RecordBucket {
    Active,
    Archived,
}

impl RecordBucket {
    fn directory(self) -> &'static str {
        match self {
            Self::Active => ACTIVE_RECORD_DIRECTORY,
            Self::Archived => ARCHIVED_RECORD_DIRECTORY,
        }
    }

    fn registry_field(self) -> &'static str {
        match self {
            Self::Active => "active_decision_records",
            Self::Archived => "archived_decision_records",
        }
    }

    fn accepts_status(self, status: &str) -> bool {
        match self {
            Self::Active => status == "active",
            Self::Archived => matches!(status, "superseded" | "expired" | "archival"),
        }
    }
}

#[derive(Debug, Deserialize)]
struct AgentPack {
    id: String,
    purpose: String,
    maximum_verbosity: String,
    ordering: u16,
    canonical_sources: Vec<String>,
    doctrine_selections: Vec<String>,
    review_checklists: Vec<String>,
    output_path: String,
}

fn main() {
    let mut arguments = env::args().skip(1);
    let command = arguments.next();
    if command.as_deref() != Some("check") || arguments.next().is_some() {
        eprintln!("usage: doctrine-lint check");
        process::exit(2);
    }

    let root = match env::current_dir() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("doctrine-lint: cannot read current directory: {error}");
            process::exit(2);
        }
    };

    let diagnostics = check_repository(&root);
    if diagnostics.is_empty() {
        println!("doctrine-lint: repository doctrine is valid");
        return;
    }

    for diagnostic in &diagnostics {
        eprintln!(
            "{}: {}",
            display_path(&root, &diagnostic.path),
            diagnostic.message
        );
    }
    eprintln!("doctrine-lint: {} diagnostic(s) found", diagnostics.len());
    process::exit(1);
}

fn check_repository(root: &Path) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let doctrine_path = root.join("manifest/doctrines.yaml");
    let agent_path = root.join("manifest/agents.yaml");

    let Some(doctrine_text) = read_text(&doctrine_path, &mut diagnostics) else {
        return diagnostics;
    };
    let Some(agent_text) = read_text(&agent_path, &mut diagnostics) else {
        return diagnostics;
    };

    validate_schema(
        &doctrine_path,
        &doctrine_text,
        &root.join("manifest/schema/doctrine.schema.json"),
        &mut diagnostics,
    );
    validate_schema(
        &agent_path,
        &agent_text,
        &root.join("manifest/schema/agent-pack.schema.json"),
        &mut diagnostics,
    );

    let doctrine_manifest = match serde_yaml_ng::from_str::<DoctrineManifest>(&doctrine_text) {
        Ok(manifest) => manifest,
        Err(error) => {
            diagnostics.push(Diagnostic::new(
                &doctrine_path,
                format!("cannot parse doctrine manifest: {error}"),
            ));
            return diagnostics;
        }
    };
    let agent_manifest = match serde_yaml_ng::from_str::<AgentManifest>(&agent_text) {
        Ok(manifest) => manifest,
        Err(error) => {
            diagnostics.push(Diagnostic::new(
                &agent_path,
                format!("cannot parse agent manifest: {error}"),
            ));
            return diagnostics;
        }
    };

    check_doctrines(root, &doctrine_manifest, &mut diagnostics);
    check_doctrine_index(root, &doctrine_manifest, &mut diagnostics);
    check_repository_version(root, &doctrine_manifest, &mut diagnostics);
    check_agents(root, &agent_manifest, &doctrine_manifest, &mut diagnostics);
    check_decision_records(root, &agent_manifest, &mut diagnostics);
    check_forbidden_markers(root, &mut diagnostics);
    check_normative_scope(root, &mut diagnostics);
    check_generated_files(root, &mut diagnostics);
    diagnostics
}

fn validate_schema(
    manifest_path: &Path,
    manifest_text: &str,
    schema_path: &Path,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(schema_text) = read_text(schema_path, diagnostics) else {
        return;
    };
    let schema = match serde_json::from_str::<Value>(&schema_text) {
        Ok(value) => value,
        Err(error) => {
            diagnostics.push(Diagnostic::new(
                schema_path,
                format!("cannot parse JSON Schema: {error}"),
            ));
            return;
        }
    };
    let instance = match serde_yaml_ng::from_str::<Value>(manifest_text) {
        Ok(value) => value,
        Err(error) => {
            diagnostics.push(Diagnostic::new(
                manifest_path,
                format!("cannot convert YAML to a JSON value: {error}"),
            ));
            return;
        }
    };
    let validator = match jsonschema::validator_for(&schema) {
        Ok(validator) => validator,
        Err(error) => {
            diagnostics.push(Diagnostic::new(
                schema_path,
                format!("cannot compile JSON Schema: {error}"),
            ));
            return;
        }
    };

    for error in validator.iter_errors(&instance) {
        diagnostics.push(Diagnostic::new(
            manifest_path,
            format!(
                "schema validation failed at {}: {error}",
                error.instance_path()
            ),
        ));
    }
}

fn check_doctrines(root: &Path, manifest: &DoctrineManifest, diagnostics: &mut Vec<Diagnostic>) {
    if manifest.schema_version != "1.0" {
        diagnostics.push(Diagnostic::new(
            root.join("manifest/doctrines.yaml"),
            "schema_version must be 1.0",
        ));
    }
    let known_ids: BTreeSet<&str> = manifest
        .doctrines
        .iter()
        .map(|entry| entry.id.as_str())
        .collect();
    if known_ids.len() != manifest.doctrines.len() {
        diagnostics.push(Diagnostic::new(
            root.join("manifest/doctrines.yaml"),
            "doctrine IDs must be unique",
        ));
    }

    let mut global_rule_ids = BTreeSet::new();
    for entry in &manifest.doctrines {
        check_doctrine_entry(root, entry, &known_ids, &mut global_rule_ids, diagnostics);
    }
}

fn check_doctrine_entry(
    root: &Path,
    entry: &DoctrineEntry,
    known_ids: &BTreeSet<&str>,
    global_rule_ids: &mut BTreeSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !check_doctrine_paths(root, entry, diagnostics) {
        return;
    }

    let expected_number = entry.id.strip_prefix("RUST-DOC-").unwrap_or("");
    let expected_folder = format!("{expected_number}-{}", entry.slug);
    let package_path = root.join(&entry.package_path);
    if package_path.file_name().and_then(|name| name.to_str()) != Some(expected_folder.as_str()) {
        diagnostics.push(Diagnostic::new(
            &package_path,
            format!(
                "package folder must be {expected_folder} to match {} and slug",
                entry.id
            ),
        ));
    }

    for name in PACKAGE_FILES {
        let path = package_path.join(name);
        match fs::metadata(&path) {
            Ok(metadata) if metadata.is_file() && metadata.len() >= 100 => {}
            Ok(_) => diagnostics.push(Diagnostic::new(
                &path,
                "required doctrine file must be a substantive regular file",
            )),
            Err(error) => diagnostics.push(Diagnostic::new(
                &path,
                format!("required doctrine file is unavailable: {error}"),
            )),
        }
    }

    check_front_matter(root, entry, diagnostics);
    check_related_paths(root, entry, diagnostics);
    check_supersession(root, entry, known_ids, diagnostics);
    check_doctrine_rules(root, entry, global_rule_ids, diagnostics);
}

fn check_doctrine_paths(
    root: &Path,
    entry: &DoctrineEntry,
    diagnostics: &mut Vec<Diagnostic>,
) -> bool {
    for (field, value) in [
        ("package_path", entry.package_path.as_str()),
        ("normative_path", entry.normative_path.as_str()),
    ] {
        if !valid_manifest_path(value) {
            diagnostics.push(Diagnostic::new(
                root.join("manifest/doctrines.yaml"),
                format!(
                    "{} {field} is not a normalized repository-relative path",
                    entry.id
                ),
            ));
            return false;
        }
    }
    true
}

fn check_supersession(
    root: &Path,
    entry: &DoctrineEntry,
    known_ids: &BTreeSet<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for superseded in &entry.supersedes {
        if !known_ids.contains(superseded.as_str()) {
            diagnostics.push(Diagnostic::new(
                root.join("manifest/doctrines.yaml"),
                format!("{} supersedes unknown doctrine {superseded}", entry.id),
            ));
        }
    }
    if let Some(superseding) = &entry.superseded_by {
        if !known_ids.contains(superseding.as_str()) {
            diagnostics.push(Diagnostic::new(
                root.join("manifest/doctrines.yaml"),
                format!(
                    "{} is superseded by unknown doctrine {superseding}",
                    entry.id
                ),
            ));
        }
    }
    if entry.status == "active" && entry.superseded_by.is_some() {
        diagnostics.push(Diagnostic::new(
            root.join("manifest/doctrines.yaml"),
            format!("active doctrine {} cannot set superseded_by", entry.id),
        ));
    }
}

fn check_doctrine_rules(
    root: &Path,
    entry: &DoctrineEntry,
    global_rule_ids: &mut BTreeSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let normative_path = root.join(&entry.normative_path);
    let Some(normative) = read_text(&normative_path, diagnostics) else {
        return;
    };
    check_structured_field_register(&normative_path, &normative, diagnostics);
    let rule_headings = extract_rule_headings(&normative);
    if rule_headings.is_empty() {
        diagnostics.push(Diagnostic::new(
            &normative_path,
            format!("{} has no normative rule IDs", entry.id),
        ));
    }
    let expected_prefix = format!("{}-R", entry.id);
    let mut rule_ids = Vec::new();
    for heading in rule_headings {
        let rule_id = heading.id;
        if heading.level != 2 {
            diagnostics.push(Diagnostic::new(
                &normative_path,
                format!(
                    "rule ID {rule_id} must use a level-2 heading; found level {} at line {}",
                    heading.level, heading.line
                ),
            ));
        }
        if !rule_id.starts_with(&expected_prefix) || !valid_rule_id(&rule_id) {
            diagnostics.push(Diagnostic::new(
                &normative_path,
                format!("invalid rule ID {rule_id} for {}", entry.id),
            ));
        }
        if !global_rule_ids.insert(rule_id.clone()) {
            diagnostics.push(Diagnostic::new(
                &normative_path,
                format!("duplicate rule ID {rule_id}"),
            ));
        }
        rule_ids.push(rule_id);
    }

    let review_path = root.join(&entry.package_path).join("review-standard.md");
    if let Some(review) = read_text(&review_path, diagnostics) {
        for rule_id in rule_ids {
            if !review.contains(&rule_id) {
                diagnostics.push(Diagnostic::new(
                    &review_path,
                    format!("review standard does not cite normative rule {rule_id}"),
                ));
            }
        }
    }
}

fn check_structured_field_register(
    path: &Path,
    normative: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (line_index, line) in normative.lines().enumerate() {
        let value = ["**Applicability.** ", "**Review evidence.** "]
            .iter()
            .find_map(|prefix| line.strip_prefix(prefix));
        let Some(value) = value else {
            continue;
        };
        if value.starts_with(|character: char| character.is_ascii_lowercase()) {
            diagnostics.push(Diagnostic::new(
                path,
                format!(
                    "structured rule field must use the capitalized noun-phrase register at line {}",
                    line_index + 1
                ),
            ));
        }
    }
}

/// Checks the reader-facing doctrine index against the manifest.
///
/// `RUST-DOC-0011-R004` permits a human-readable view of an enforced claim only
/// when it is generated or mechanically checked. This index carries a prose concern
/// column the manifest does not hold, so it stays hand-written and is checked here
/// instead: a doctrine cannot be added without the index following it.
fn check_doctrine_index(
    root: &Path,
    manifest: &DoctrineManifest,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let index_path = root.join(DOCTRINE_INDEX);
    let Some(index) = read_text(&index_path, diagnostics) else {
        return;
    };

    for entry in &manifest.doctrines {
        if entry.status != "active" {
            continue;
        }
        // A row, not a passing mention: the identifier and the manifest title have to
        // appear on one line, so a cross-reference in prose cannot satisfy the check.
        let has_row = index
            .lines()
            .any(|line| line.contains(&entry.id) && line.contains(&entry.title));
        if !has_row {
            diagnostics.push(Diagnostic::new(
                &index_path,
                format!(
                    "doctrine index has no row naming {} with its manifest title {:?}",
                    entry.id, entry.title
                ),
            ));
        }
    }

    for id in extract_doctrine_ids(&index) {
        if !manifest
            .doctrines
            .iter()
            .any(|entry| entry.id == id && entry.status == "active")
        {
            diagnostics.push(Diagnostic::new(
                &index_path,
                format!("doctrine index lists {id}, which is not an active doctrine"),
            ));
        }
    }
}

/// Doctrine identifiers named by the index, ignoring rule identifiers such as
/// `RUST-DOC-0011-R004` so a cross-reference is not mistaken for an index row.
fn extract_doctrine_ids(text: &str) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for (offset, _) in text.match_indices("RUST-DOC-") {
        let rest = &text[offset + "RUST-DOC-".len()..];
        let digits: String = rest.chars().take(4).collect();
        if digits.len() != 4 || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
            continue;
        }
        if rest[digits.len()..].starts_with("-R") {
            continue;
        }
        ids.insert(format!("RUST-DOC-{digits}"));
    }
    ids
}

fn check_repository_version(
    root: &Path,
    manifest: &DoctrineManifest,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let cargo_path = root.join("Cargo.toml");
    let Some(cargo) = read_text(&cargo_path, diagnostics) else {
        return;
    };
    let Some(workspace_version) = workspace_package_version(&cargo) else {
        diagnostics.push(Diagnostic::new(
            &cargo_path,
            "workspace.package version is missing or is not a quoted string",
        ));
        return;
    };
    if manifest.repository_version != workspace_version {
        diagnostics.push(Diagnostic::new(
            root.join("manifest/doctrines.yaml"),
            format!(
                "repository_version {} does not match workspace.package version {workspace_version}",
                manifest.repository_version
            ),
        ));
    }
}

fn workspace_package_version(cargo: &str) -> Option<&str> {
    let mut in_workspace_package = false;
    for line in cargo.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_workspace_package = line == "[workspace.package]";
            continue;
        }
        if !in_workspace_package {
            continue;
        }
        if let Some(value) = line.strip_prefix("version = \"") {
            return value.strip_suffix('"');
        }
    }
    None
}

fn check_front_matter(root: &Path, entry: &DoctrineEntry, diagnostics: &mut Vec<Diagnostic>) {
    let readme_path = root.join(&entry.package_path).join("README.md");
    let Some(readme) = read_text(&readme_path, diagnostics) else {
        return;
    };
    let metadata_text = match front_matter(&readme) {
        Ok(metadata) => metadata,
        Err(message) => {
            diagnostics.push(Diagnostic::new(&readme_path, message));
            return;
        }
    };
    let metadata = match serde_yaml_ng::from_str::<DoctrineMetadata>(metadata_text) {
        Ok(metadata) => metadata,
        Err(error) => {
            diagnostics.push(Diagnostic::new(
                &readme_path,
                format!("cannot parse doctrine front matter: {error}"),
            ));
            return;
        }
    };

    compare_metadata(&readme_path, "id", &metadata.id, &entry.id, diagnostics);
    compare_metadata(
        &readme_path,
        "slug",
        &metadata.slug,
        &entry.slug,
        diagnostics,
    );
    compare_metadata(
        &readme_path,
        "title",
        &metadata.title,
        &entry.title,
        diagnostics,
    );
    compare_metadata(
        &readme_path,
        "status",
        &metadata.status,
        &entry.status,
        diagnostics,
    );
    compare_metadata(
        &readme_path,
        "version",
        &metadata.version,
        &entry.version,
        diagnostics,
    );
    if !metadata.normative {
        diagnostics.push(Diagnostic::new(
            &readme_path,
            "doctrine front matter must set normative: true",
        ));
    }
    if metadata.applies_to != entry.applies_to {
        diagnostics.push(Diagnostic::new(
            &readme_path,
            "front-matter applies_to must match manifest order and values",
        ));
    }
    if metadata.risk_domains != entry.risk_domains {
        diagnostics.push(Diagnostic::new(
            &readme_path,
            "front-matter risk_domains must match manifest order and values",
        ));
    }
    if metadata.supersedes != entry.supersedes || metadata.superseded_by != entry.superseded_by {
        diagnostics.push(Diagnostic::new(
            &readme_path,
            "front-matter supersession metadata must match manifest",
        ));
    }
}

fn compare_metadata(
    path: &Path,
    field: &str,
    actual: &str,
    expected: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if actual != expected {
        diagnostics.push(Diagnostic::new(
            path,
            format!("front-matter {field} is {actual:?}, expected {expected:?}"),
        ));
    }
}

fn check_related_paths(root: &Path, entry: &DoctrineEntry, diagnostics: &mut Vec<Diagnostic>) {
    let paths = entry
        .foundation_dependencies
        .iter()
        .chain(&entry.related_patterns)
        .chain(&entry.related_boundaries)
        .chain(&entry.related_case_studies);
    for path in paths {
        if !valid_manifest_path(path) {
            diagnostics.push(Diagnostic::new(
                root.join("manifest/doctrines.yaml"),
                format!(
                    "related path referenced by {} is not a normalized repository-relative path: {path}",
                    entry.id
                ),
            ));
            continue;
        }
        if !root.join(path).exists() {
            diagnostics.push(Diagnostic::new(
                root.join(path),
                format!("related path referenced by {} does not exist", entry.id),
            ));
        }
    }
    if Path::new(&entry.normative_path).parent() != Some(Path::new(&entry.package_path)) {
        diagnostics.push(Diagnostic::new(
            root.join(&entry.normative_path),
            "normative_path must live in package_path",
        ));
    }
}

fn check_agents(
    root: &Path,
    manifest: &AgentManifest,
    doctrine_manifest: &DoctrineManifest,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let manifest_path = root.join("manifest/agents.yaml");
    if manifest.schema_version != "1.0" {
        diagnostics.push(Diagnostic::new(
            &manifest_path,
            "schema_version must be 1.0",
        ));
    }

    let doctrine_ids: BTreeSet<&str> = doctrine_manifest
        .doctrines
        .iter()
        .map(|entry| entry.id.as_str())
        .collect();
    let expected_ids: BTreeSet<&str> = [
        "shared",
        "planner",
        "implementer",
        "reviewer",
        "auditor",
        "maintainer",
    ]
    .into_iter()
    .collect();
    let actual_ids: BTreeSet<&str> = manifest.packs.iter().map(|pack| pack.id.as_str()).collect();
    if actual_ids != expected_ids || actual_ids.len() != manifest.packs.len() {
        diagnostics.push(Diagnostic::new(
            &manifest_path,
            "agent packs must contain each allowed role exactly once",
        ));
    }

    let mut orderings = BTreeSet::new();
    let mut outputs = BTreeSet::new();
    for pack in &manifest.packs {
        check_agent_pack(
            root,
            &manifest_path,
            pack,
            &doctrine_ids,
            &mut orderings,
            &mut outputs,
            diagnostics,
        );
    }
}

fn check_agent_pack<'a>(
    root: &Path,
    manifest_path: &Path,
    pack: &'a AgentPack,
    doctrine_ids: &BTreeSet<&str>,
    orderings: &mut BTreeSet<u16>,
    outputs: &mut BTreeSet<&'a str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if pack.purpose.trim().len() < 20 {
        diagnostics.push(Diagnostic::new(
            manifest_path,
            format!("agent {} purpose is too short", pack.id),
        ));
    }
    if !matches!(
        pack.maximum_verbosity.as_str(),
        "focused" | "operational" | "detailed" | "exhaustive"
    ) {
        diagnostics.push(Diagnostic::new(
            manifest_path,
            format!(
                "agent {} has invalid maximum_verbosity {}",
                pack.id, pack.maximum_verbosity
            ),
        ));
    }
    if !orderings.insert(pack.ordering) {
        diagnostics.push(Diagnostic::new(
            manifest_path,
            format!("agent ordering {} is duplicated", pack.ordering),
        ));
    }
    if !outputs.insert(pack.output_path.as_str()) {
        diagnostics.push(Diagnostic::new(
            manifest_path,
            format!("agent output {} is duplicated", pack.output_path),
        ));
    }
    if !pack.output_path.starts_with("dist/agents/") {
        diagnostics.push(Diagnostic::new(
            manifest_path,
            format!(
                "agent output {} must be under dist/agents",
                pack.output_path
            ),
        ));
    }

    for path in pack.canonical_sources.iter().chain(&pack.review_checklists) {
        check_agent_source(root, manifest_path, pack, path, diagnostics);
    }
    for doctrine in &pack.doctrine_selections {
        if !doctrine_ids.contains(doctrine.as_str()) {
            diagnostics.push(Diagnostic::new(
                manifest_path,
                format!("agent {} selects unknown doctrine {doctrine}", pack.id),
            ));
        }
    }
    if !valid_manifest_path(&pack.output_path) {
        diagnostics.push(Diagnostic::new(
            manifest_path,
            format!(
                "agent {} output path is not normalized and repository-relative",
                pack.id
            ),
        ));
    } else if !root.join(&pack.output_path).is_file() {
        diagnostics.push(Diagnostic::new(
            root.join(&pack.output_path),
            format!("generated output for agent {} is missing", pack.id),
        ));
    }
}

fn check_agent_source(
    root: &Path,
    manifest_path: &Path,
    pack: &AgentPack,
    path: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !valid_manifest_path(path) {
        diagnostics.push(Diagnostic::new(
            manifest_path,
            format!(
                "agent {} path is not normalized and repository-relative: {path}",
                pack.id
            ),
        ));
    } else if !root.join(path).is_file() {
        diagnostics.push(Diagnostic::new(
            root.join(path),
            format!("agent {} references a missing canonical file", pack.id),
        ));
    }
}

/// Validates the decision-record registry required by `RUST-DOC-0011-R007`.
///
/// The registry is the only entry point to the active set, so a record that is
/// unowned, endless, or filed in the wrong place has to fail here rather than in a
/// review that may never look.
fn check_decision_records(root: &Path, agents: &AgentManifest, diagnostics: &mut Vec<Diagnostic>) {
    let registry_path = root.join("manifest/decision-records.yaml");
    let Some(registry_text) = read_text(&registry_path, diagnostics) else {
        return;
    };

    validate_schema(
        &registry_path,
        &registry_text,
        &root.join("manifest/schema/decision-record.schema.json"),
        diagnostics,
    );

    let registry = match serde_yaml_ng::from_str::<DecisionRecordRegistry>(&registry_text) {
        Ok(registry) => registry,
        Err(error) => {
            diagnostics.push(Diagnostic::new(
                &registry_path,
                format!("cannot parse decision-record registry: {error}"),
            ));
            return;
        }
    };
    if registry.schema_version != "1.0" {
        diagnostics.push(Diagnostic::new(
            &registry_path,
            "schema_version must be 1.0",
        ));
    }

    let mut identifiers = BTreeSet::new();
    for path in &registry.active_decision_records {
        check_registered_record(
            root,
            &registry_path,
            path,
            RecordBucket::Active,
            &mut identifiers,
            diagnostics,
        );
    }
    for path in &registry.archived_decision_records {
        check_registered_record(
            root,
            &registry_path,
            path,
            RecordBucket::Archived,
            &mut identifiers,
            diagnostics,
        );
    }

    check_agent_packs_exclude_archive(root, agents, diagnostics);
}

/// Validates one registered record from its own front matter.
///
/// The registry supplies membership and nothing else, so every field checked here
/// is read from the record itself. The single fact both artifacts express, which
/// list a record appears in versus the `status` it declares, is compared rather
/// than trusted.
fn check_registered_record(
    root: &Path,
    registry_path: &Path,
    path: &str,
    bucket: RecordBucket,
    identifiers: &mut BTreeSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !check_record_path(
        root,
        registry_path,
        path,
        path,
        "record",
        Some(bucket.directory()),
        diagnostics,
    ) {
        return;
    }

    let record_path = root.join(path);
    let Some(text) = read_text(&record_path, diagnostics) else {
        return;
    };
    let metadata_text = match front_matter(&text) {
        Ok(metadata) => metadata,
        Err(message) => {
            diagnostics.push(Diagnostic::new(&record_path, message));
            return;
        }
    };
    let metadata = match serde_yaml_ng::from_str::<DecisionRecordMetadata>(metadata_text) {
        Ok(metadata) => metadata,
        Err(error) => {
            diagnostics.push(Diagnostic::new(
                &record_path,
                format!("cannot parse decision-record front matter: {error}"),
            ));
            return;
        }
    };

    if !valid_record_id(&metadata.id) {
        diagnostics.push(Diagnostic::new(
            &record_path,
            format!("decision record ID {} must match ADR-NNNN", metadata.id),
        ));
    }
    if !identifiers.insert(metadata.id.clone()) {
        diagnostics.push(Diagnostic::new(
            &record_path,
            format!("decision record ID {} is duplicated", metadata.id),
        ));
    }
    if !bucket.accepts_status(&metadata.status) {
        diagnostics.push(Diagnostic::new(
            &record_path,
            format!(
                "decision record {} declares status {} but the registry lists it under {}",
                metadata.id,
                metadata.status,
                bucket.registry_field()
            ),
        ));
    }
    for (field, value) in [
        ("title", &metadata.title),
        ("owner", &metadata.owner),
        ("scope", &metadata.scope),
    ] {
        if value.trim().is_empty() {
            diagnostics.push(Diagnostic::new(
                &record_path,
                format!("decision record {} has an empty {field}", metadata.id),
            ));
        }
    }

    match bucket {
        RecordBucket::Active => check_active_record(root, &record_path, &metadata, diagnostics),
        RecordBucket::Archived => {
            check_archived_record(&record_path, &text, &metadata, diagnostics);
        }
    }
}

fn check_active_record(
    root: &Path,
    record_path: &Path,
    metadata: &DecisionRecordMetadata,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (field, values) in [
        ("revalidate_on", &metadata.revalidate_on),
        ("obsolete_when", &metadata.obsolete_when),
        ("executable_authority", &metadata.executable_authority),
    ] {
        if values.is_empty() {
            diagnostics.push(Diagnostic::new(
                record_path,
                format!(
                    "active decision record {} states no {field}; RUST-DOC-0011-R007 requires one",
                    metadata.id
                ),
            ));
        }
    }
    for authority in &metadata.executable_authority {
        check_record_path(
            root,
            record_path,
            &metadata.id,
            authority,
            "executable authority",
            None,
            diagnostics,
        );
    }
}

fn check_archived_record(
    record_path: &Path,
    text: &str,
    metadata: &DecisionRecordMetadata,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if metadata.archived_reason.trim().is_empty() {
        diagnostics.push(Diagnostic::new(
            record_path,
            format!("archived decision record {} states no reason", metadata.id),
        ));
    }
    if !text.contains(ARCHIVAL_MARKER) {
        diagnostics.push(Diagnostic::new(
            record_path,
            format!(
                "archived decision record {} must carry {ARCHIVAL_MARKER}",
                metadata.id
            ),
        ));
    }
}

fn check_record_path(
    root: &Path,
    registry_path: &Path,
    id: &str,
    value: &str,
    field: &str,
    required_prefix: Option<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) -> bool {
    if !valid_manifest_path(value) {
        diagnostics.push(Diagnostic::new(
            registry_path,
            format!("decision record {id} {field} path is not normalized and repository-relative: {value}"),
        ));
        return false;
    }
    if let Some(prefix) = required_prefix {
        if !value.starts_with(prefix) {
            diagnostics.push(Diagnostic::new(
                registry_path,
                format!("decision record {id} {field} path must live under {prefix}"),
            ));
            return false;
        }
    }
    if !root.join(value).is_file() {
        diagnostics.push(Diagnostic::new(
            root.join(value),
            format!("decision record {id} {field} path does not resolve to a file"),
        ));
        return false;
    }
    true
}

fn check_agent_packs_exclude_archive(
    root: &Path,
    agents: &AgentManifest,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let manifest_path = root.join("manifest/agents.yaml");
    for pack in &agents.packs {
        for path in pack.canonical_sources.iter().chain(&pack.review_checklists) {
            if path.starts_with(ARCHIVED_RECORD_DIRECTORY) {
                diagnostics.push(Diagnostic::new(
                    &manifest_path,
                    format!(
                        "agent {} hydrates archived decision record {path}; RUST-DOC-0011-R018 excludes it",
                        pack.id
                    ),
                ));
            }
        }
    }
}

fn check_forbidden_markers(root: &Path, diagnostics: &mut Vec<Diagnostic>) {
    collect_repository_files(root, root, &mut |path| {
        scan_marker_file(path, diagnostics);
    });
}

fn check_normative_scope(root: &Path, diagnostics: &mut Vec<Diagnostic>) {
    for path in ROOT_DOCUMENTS {
        scan_normative_scope(root, &root.join(path), diagnostics);
    }
    for directory in CANONICAL_ROOTS {
        collect_files(&root.join(directory), &mut |path| {
            if path.extension().and_then(|extension| extension.to_str()) == Some("md") {
                scan_normative_scope(root, path, diagnostics);
            }
        });
    }
}

fn scan_marker_file(path: &Path, diagnostics: &mut Vec<Diagnostic>) {
    let Ok(text) = fs::read_to_string(path) else {
        return;
    };
    for (line_index, line) in text.lines().enumerate() {
        let lower = line.to_lowercase();
        for marker in FORBIDDEN_MARKERS {
            if lower.contains(marker) {
                diagnostics.push(Diagnostic::new(
                    path,
                    format!(
                        "forbidden filler marker {marker:?} at line {}",
                        line_index + 1
                    ),
                ));
            }
        }
    }
}

fn scan_normative_scope(root: &Path, path: &Path, diagnostics: &mut Vec<Diagnostic>) {
    let relative = path.strip_prefix(root).unwrap_or(path);
    let relative_text = relative.to_string_lossy().replace('\\', "/");
    if NORMATIVE_SCOPE_EXCEPTIONS.contains(&relative_text.as_str())
        || relative.file_name().and_then(|name| name.to_str()) == Some("doctrine.md")
    {
        return;
    }

    let Some(text) = read_text(path, diagnostics) else {
        return;
    };
    let mut in_fence = false;
    for (line_index, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        if !in_fence && contains_normative_term(line) {
            diagnostics.push(Diagnostic::new(
                path,
                format!(
                    "uppercase normative term outside doctrine.md or an explicit governance contract at line {}",
                    line_index + 1
                ),
            ));
        }
    }
}

fn contains_normative_term(line: &str) -> bool {
    line.split(|character: char| !character.is_ascii_alphabetic())
        .any(|word| matches!(word, "MUST" | "SHOULD" | "MAY"))
}

fn check_generated_files(root: &Path, diagnostics: &mut Vec<Diagnostic>) {
    let dist = root.join("dist");
    if !dist.is_dir() {
        diagnostics.push(Diagnostic::new(
            &dist,
            "generated distribution directory is missing",
        ));
        return;
    }

    collect_files(&dist, &mut |path| {
        if path.extension().and_then(|extension| extension.to_str()) != Some("md") {
            return;
        }
        let Some(text) = read_text(path, diagnostics) else {
            return;
        };
        if !text.starts_with(GENERATED_BANNER) {
            diagnostics.push(Diagnostic::new(
                path,
                "generated Markdown is missing the required banner",
            ));
        }
    });
}

fn collect_files(directory: &Path, visit: &mut impl FnMut(&Path)) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            collect_files(&path, visit);
        } else if path.is_file() {
            visit(&path);
        }
    }
}

fn collect_repository_files(root: &Path, directory: &Path, visit: &mut impl FnMut(&Path)) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    let mut paths = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    paths.sort();

    for path in paths {
        if path.is_dir() {
            let relative = path.strip_prefix(root).unwrap_or(&path);
            let top = relative
                .components()
                .next()
                .and_then(|component| match component {
                    std::path::Component::Normal(value) => value.to_str(),
                    _ => None,
                });
            if matches!(top, Some(".git" | "node_modules" | "target" | "templates")) {
                continue;
            }
            collect_repository_files(root, &path, visit);
        } else if path.is_file() {
            visit(&path);
        }
    }
}

fn read_text(path: &Path, diagnostics: &mut Vec<Diagnostic>) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(text) => Some(text),
        Err(error) => {
            diagnostics.push(Diagnostic::new(
                path,
                format!("cannot read UTF-8 text: {error}"),
            ));
            None
        }
    }
}

fn front_matter(text: &str) -> Result<&str, &'static str> {
    let body = text
        .strip_prefix("---\n")
        .ok_or("README must start with YAML front matter")?;
    let end = body
        .find("\n---\n")
        .ok_or("README front matter must end with ---")?;
    Ok(&body[..end])
}

fn extract_rule_headings(text: &str) -> Vec<RuleHeading> {
    text.lines()
        .enumerate()
        .filter_map(|(line_index, line)| {
            let line = line.trim_start_matches(' ');
            let level = line.bytes().take_while(|byte| *byte == b'#').count();
            if !(1..=6).contains(&level) || line.as_bytes().get(level) != Some(&b' ') {
                return None;
            }
            let heading = &line[level + 1..];
            let token = heading.split_whitespace().next()?;
            token.starts_with("RUST-DOC-").then(|| RuleHeading {
                id: token.to_owned(),
                level,
                line: line_index + 1,
            })
        })
        .collect()
}

fn valid_manifest_path(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('/')
        && !value.ends_with('/')
        && !value.contains('\\')
        && value
            .split('/')
            .all(|component| !component.is_empty() && !matches!(component, "." | ".."))
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'.' | b'_' | b'-' | b'/')
        })
}

fn valid_record_id(record_id: &str) -> bool {
    let Some(rest) = record_id.strip_prefix("ADR-") else {
        return false;
    };
    rest.len() == 4 && rest.bytes().all(|byte| byte.is_ascii_digit())
}

fn valid_rule_id(rule_id: &str) -> bool {
    let Some(rest) = rule_id.strip_prefix("RUST-DOC-") else {
        return false;
    };
    let Some((doctrine, rule)) = rest.split_once("-R") else {
        return false;
    };
    doctrine.len() == 4
        && doctrine.bytes().all(|byte| byte.is_ascii_digit())
        && rule.len() == 3
        && rule.bytes().all(|byte| byte.is_ascii_digit())
}

fn display_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{
        AgentManifest, AgentPack, DOCTRINE_INDEX, DoctrineEntry, DoctrineManifest, RecordBucket,
        RuleHeading, check_agent_packs_exclude_archive, check_doctrine_index,
        check_registered_record, check_structured_field_register, contains_normative_term,
        extract_doctrine_ids, extract_rule_headings, front_matter, valid_manifest_path,
        valid_record_id, valid_rule_id, workspace_package_version,
    };
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::{Path, PathBuf};

    fn temporary_directory(name: &str) -> PathBuf {
        let path =
            std::env::temp_dir().join(format!("doctrines-rust-lint-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create temporary test directory");
        path
    }

    /// Writes a record file and returns the repository-relative path it was written to.
    fn write_record(root: &Path, relative: &str, front_matter_body: &str, body: &str) -> String {
        let path = root.join(relative);
        fs::create_dir_all(path.parent().expect("record parent")).expect("create record directory");
        fs::write(&path, format!("---\n{front_matter_body}---\n\n{body}")).expect("write record");
        relative.to_owned()
    }

    fn active_front_matter() -> String {
        concat!(
            "id: ADR-0001\n",
            "title: Subscriber data stays inside one jurisdiction\n",
            "status: active\n",
            "owner: platform-governance\n",
            "scope: data-residency\n",
            "executable_authority:\n",
            "  - decisions/active/adr-0001-residency.md\n",
            "revalidate_on:\n",
            "  - contract-renewal\n",
            "obsolete_when:\n",
            "  - obligation-withdrawn\n",
        )
        .to_owned()
    }

    fn check_one(root: &Path, relative: &str, bucket: RecordBucket) -> Vec<String> {
        let mut diagnostics = Vec::new();
        check_registered_record(
            root,
            Path::new("manifest/decision-records.yaml"),
            relative,
            bucket,
            &mut BTreeSet::new(),
            &mut diagnostics,
        );
        diagnostics
            .into_iter()
            .map(|diagnostic| diagnostic.message)
            .collect()
    }

    fn doctrine_entry(id: &str, slug: &str, title: &str, status: &str) -> DoctrineEntry {
        DoctrineEntry {
            id: id.to_owned(),
            slug: slug.to_owned(),
            title: title.to_owned(),
            status: status.to_owned(),
            version: "0.1.0".to_owned(),
            package_path: format!("doctrines/{slug}"),
            normative_path: format!("doctrines/{slug}/doctrine.md"),
            applies_to: vec!["review".to_owned()],
            risk_domains: vec!["domain-modeling".to_owned()],
            foundation_dependencies: vec!["foundations/invariants.md".to_owned()],
            related_patterns: Vec::new(),
            related_boundaries: Vec::new(),
            related_case_studies: Vec::new(),
            supersedes: Vec::new(),
            superseded_by: None,
        }
    }

    fn doctrine_manifest() -> DoctrineManifest {
        DoctrineManifest {
            schema_version: "1.0".to_owned(),
            repository_version: "0.4.0".to_owned(),
            doctrines: vec![
                doctrine_entry(
                    "RUST-DOC-0001",
                    "0001-invalid-states",
                    "Making Invalid States Unrepresentable",
                    "active",
                ),
                doctrine_entry(
                    "RUST-DOC-0002",
                    "0002-error-modeling",
                    "Error Modeling as Domain Design",
                    "active",
                ),
            ],
        }
    }

    fn agent_pack(canonical_sources: Vec<String>) -> AgentPack {
        AgentPack {
            id: "auditor".to_owned(),
            purpose: "Adversarially locate ungoverned authority.".to_owned(),
            maximum_verbosity: "exhaustive".to_owned(),
            ordering: 50,
            canonical_sources,
            doctrine_selections: Vec::new(),
            review_checklists: Vec::new(),
            output_path: "dist/agents/auditor.md".to_owned(),
        }
    }

    #[test]
    fn a_well_formed_active_record_passes() {
        let root = temporary_directory("active-ok");
        let relative = write_record(
            &root,
            "decisions/active/adr-0001-residency.md",
            &active_front_matter(),
            "# ADR-0001\n",
        );

        assert!(
            check_one(&root, &relative, RecordBucket::Active).is_empty(),
            "well-formed record must pass"
        );
        fs::remove_dir_all(root).expect("remove temporary test directory");
    }

    #[test]
    fn active_record_metadata_is_read_from_the_record_not_the_registry() {
        let root = temporary_directory("active-missing-fields");
        let relative = write_record(
            &root,
            "decisions/active/adr-0001-residency.md",
            concat!(
                "id: ADR-0001\n",
                "title: Subscriber data stays inside one jurisdiction\n",
                "status: active\n",
                "owner: \"  \"\n",
                "scope: data-residency\n",
            ),
            "# ADR-0001\n",
        );

        let messages = check_one(&root, &relative, RecordBucket::Active).join("\n");
        assert!(messages.contains("empty owner"), "{messages}");
        assert!(messages.contains("no revalidate_on"), "{messages}");
        assert!(messages.contains("no obsolete_when"), "{messages}");
        assert!(messages.contains("no executable_authority"), "{messages}");
        fs::remove_dir_all(root).expect("remove temporary test directory");
    }

    /// The registry and the record express one overlapping fact: which set the record
    /// belongs to. It is compared, not trusted.
    #[test]
    fn a_record_whose_status_disagrees_with_its_registry_list_is_rejected() {
        let root = temporary_directory("status-mismatch");
        let relative = write_record(
            &root,
            "decisions/active/adr-0001-residency.md",
            &active_front_matter().replace("status: active", "status: expired"),
            "# ADR-0001\n",
        );

        let messages = check_one(&root, &relative, RecordBucket::Active).join("\n");
        assert!(
            messages.contains("declares status expired")
                && messages.contains("active_decision_records"),
            "{messages}"
        );
        fs::remove_dir_all(root).expect("remove temporary test directory");
    }

    #[test]
    fn an_executable_authority_that_does_not_resolve_is_rejected() {
        let root = temporary_directory("authority-missing");
        let relative = write_record(
            &root,
            "decisions/active/adr-0001-residency.md",
            &active_front_matter().replace(
                "  - decisions/active/adr-0001-residency.md",
                "  - deploy/policy/storage-regions.yaml",
            ),
            "# ADR-0001\n",
        );

        let messages = check_one(&root, &relative, RecordBucket::Active).join("\n");
        assert!(
            messages.contains("executable authority path does not resolve to a file"),
            "{messages}"
        );
        fs::remove_dir_all(root).expect("remove temporary test directory");
    }

    #[test]
    fn a_record_without_front_matter_is_rejected() {
        let root = temporary_directory("no-front-matter");
        let path = root.join("decisions/active/adr-0001-residency.md");
        fs::create_dir_all(path.parent().expect("record parent")).expect("create directory");
        fs::write(&path, "# ADR-0001\n\nNo front matter here.\n").expect("write record");

        let messages = check_one(
            &root,
            "decisions/active/adr-0001-residency.md",
            RecordBucket::Active,
        )
        .join("\n");
        assert!(messages.contains("front matter"), "{messages}");
        fs::remove_dir_all(root).expect("remove temporary test directory");
    }

    #[test]
    fn active_record_must_be_filed_under_the_active_directory() {
        let root = temporary_directory("wrong-directory");
        let relative = write_record(
            &root,
            "decisions/examples/justified-data-residency.md",
            &active_front_matter(),
            "# ADR-0001\n",
        );

        let messages = check_one(&root, &relative, RecordBucket::Active).join("\n");
        assert!(
            messages.contains("must live under decisions/active/"),
            "{messages}"
        );
        fs::remove_dir_all(root).expect("remove temporary test directory");
    }

    #[test]
    fn duplicate_record_identifiers_are_rejected() {
        let root = temporary_directory("duplicate-ids");
        let first = write_record(
            &root,
            "decisions/active/adr-0001-residency.md",
            &active_front_matter(),
            "# ADR-0001\n",
        );
        let second = write_record(
            &root,
            "decisions/active/adr-0001-duplicate.md",
            &active_front_matter().replace(
                "  - decisions/active/adr-0001-residency.md",
                "  - decisions/active/adr-0001-duplicate.md",
            ),
            "# ADR-0001\n",
        );

        let mut identifiers = BTreeSet::new();
        let mut diagnostics = Vec::new();
        for relative in [&first, &second] {
            check_registered_record(
                &root,
                Path::new("manifest/decision-records.yaml"),
                relative,
                RecordBucket::Active,
                &mut identifiers,
                &mut diagnostics,
            );
        }

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.contains("ADR-0001 is duplicated")),
            "{diagnostics:?}"
        );
        fs::remove_dir_all(root).expect("remove temporary test directory");
    }

    #[test]
    fn archived_record_must_carry_the_archival_marker_and_a_reason() {
        let root = temporary_directory("archival-marker");
        let archived = concat!(
            "id: ADR-0002\n",
            "title: A retired residency obligation\n",
            "status: expired\n",
            "owner: platform-governance\n",
            "scope: data-residency\n",
            "archived_reason: the residency obligation was withdrawn\n",
        );
        let relative = write_record(
            &root,
            "decisions/archive/adr-0002-old.md",
            archived,
            "# ADR-0002\n\nThis file omits the marker.\n",
        );

        let messages = check_one(&root, &relative, RecordBucket::Archived).join("\n");
        assert!(
            messages.contains("NOT CURRENT OPERATIONAL AUTHORITY"),
            "{messages}"
        );

        write_record(
            &root,
            "decisions/archive/adr-0002-old.md",
            archived,
            "# ADR-0002\n\nNOT CURRENT OPERATIONAL AUTHORITY\n",
        );
        assert!(
            check_one(&root, &relative, RecordBucket::Archived).is_empty(),
            "corrected archived record must pass"
        );

        write_record(
            &root,
            "decisions/archive/adr-0002-old.md",
            &archived.replace(
                "archived_reason: the residency obligation was withdrawn",
                "archived_reason: \"\"",
            ),
            "# ADR-0002\n\nNOT CURRENT OPERATIONAL AUTHORITY\n",
        );
        let messages = check_one(&root, &relative, RecordBucket::Archived).join("\n");
        assert!(messages.contains("states no reason"), "{messages}");

        fs::remove_dir_all(root).expect("remove temporary test directory");
    }

    #[test]
    fn agent_packs_may_not_hydrate_an_archived_record() {
        let manifest = AgentManifest {
            schema_version: "1.0".to_owned(),
            packs: vec![agent_pack(vec![
                "agents/shared.md".to_owned(),
                "decisions/archive/adr-0002-old.md".to_owned(),
            ])],
        };

        let mut diagnostics = Vec::new();
        check_agent_packs_exclude_archive(Path::new("/nonexistent"), &manifest, &mut diagnostics);

        assert_eq!(diagnostics.len(), 1);
        assert!(
            diagnostics[0].message.contains("RUST-DOC-0011-R018"),
            "{diagnostics:?}"
        );
    }

    #[test]
    fn agent_packs_without_archived_records_pass() {
        let manifest = AgentManifest {
            schema_version: "1.0".to_owned(),
            packs: vec![agent_pack(vec!["agents/shared.md".to_owned()])],
        };

        let mut diagnostics = Vec::new();
        check_agent_packs_exclude_archive(Path::new("/nonexistent"), &manifest, &mut diagnostics);

        assert!(diagnostics.is_empty(), "{diagnostics:?}");
    }

    #[test]
    fn doctrine_index_ids_ignore_rule_cross_references() {
        let index = concat!(
            "| RUST-DOC-0001 | [Making Invalid States Unrepresentable](0001-invalid-states/) |\n",
            "Applied by `RUST-DOC-0011-R004` and `RUST-DOC-0010-R022`.\n",
        );
        let ids = extract_doctrine_ids(index);
        assert!(ids.contains("RUST-DOC-0001"), "{ids:?}");
        assert_eq!(ids.len(), 1, "rule cross-references must not count as rows");
    }

    #[test]
    fn doctrine_index_must_list_every_active_doctrine_with_its_title() {
        let manifest = doctrine_manifest();
        let root = temporary_directory("doctrine-index");
        fs::create_dir_all(root.join("doctrines")).expect("create doctrines directory");

        // A prose cross-reference must not satisfy the row requirement, which is the
        // way the shipped index hid a missing row.
        fs::write(
            root.join(DOCTRINE_INDEX),
            concat!(
                "| RUST-DOC-0001 | Making Invalid States Unrepresentable |\n",
                "Read RUST-DOC-0002 when failure modelling matters.\n",
                "Error Modeling as Domain Design is worth reading early.\n",
            ),
        )
        .expect("write index");
        let mut diagnostics = Vec::new();
        check_doctrine_index(&root, &manifest, &mut diagnostics);
        let messages = diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            messages.contains("no row naming RUST-DOC-0002"),
            "{messages}"
        );

        fs::write(
            root.join(DOCTRINE_INDEX),
            concat!(
                "| RUST-DOC-0001 | Making Invalid States Unrepresentable |\n",
                "| RUST-DOC-0002 | An Invented Title |\n",
            ),
        )
        .expect("rewrite index");
        let mut diagnostics = Vec::new();
        check_doctrine_index(&root, &manifest, &mut diagnostics);
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.contains("no row naming RUST-DOC-0002")),
            "{diagnostics:?}"
        );

        fs::write(
            root.join(DOCTRINE_INDEX),
            concat!(
                "| RUST-DOC-0001 | Making Invalid States Unrepresentable |\n",
                "| RUST-DOC-0002 | Error Modeling as Domain Design |\n",
                "| RUST-DOC-0099 | A Doctrine That Does Not Exist |\n",
            ),
        )
        .expect("rewrite index");
        let mut diagnostics = Vec::new();
        check_doctrine_index(&root, &manifest, &mut diagnostics);
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.contains("RUST-DOC-0099, which is not")),
            "{diagnostics:?}"
        );

        fs::write(
            root.join(DOCTRINE_INDEX),
            concat!(
                "| RUST-DOC-0001 | Making Invalid States Unrepresentable |\n",
                "| RUST-DOC-0002 | Error Modeling as Domain Design |\n",
            ),
        )
        .expect("rewrite index");
        let mut diagnostics = Vec::new();
        check_doctrine_index(&root, &manifest, &mut diagnostics);
        assert!(diagnostics.is_empty(), "{diagnostics:?}");

        fs::remove_dir_all(root).expect("remove temporary test directory");
    }

    #[test]
    fn validates_record_shape() {
        assert!(valid_record_id("ADR-0007"));
        assert!(!valid_record_id("ADR-007"));
        assert!(!valid_record_id("ADR-EXAMPLE-0001"));
    }

    #[test]
    fn extracts_rule_headings_at_every_depth() {
        let text =
            "# Title\n\n## RUST-DOC-0001-R001 — Rule\n\n### RUST-DOC-0001-R002 — Wrong depth\n";
        assert_eq!(
            extract_rule_headings(text),
            [
                RuleHeading {
                    id: "RUST-DOC-0001-R001".to_owned(),
                    level: 2,
                    line: 3,
                },
                RuleHeading {
                    id: "RUST-DOC-0001-R002".to_owned(),
                    level: 3,
                    line: 5,
                },
            ]
        );
    }

    #[test]
    fn validates_rule_shape() {
        assert!(valid_rule_id("RUST-DOC-0009-R020"));
        assert!(!valid_rule_id("RUST-DOC-009-R020"));
        assert!(!valid_rule_id("RUST-DOC-0009-X020"));
    }

    #[test]
    fn parses_front_matter() {
        let text = "---\nid: RUST-DOC-0001\n---\n\n# Title\n";
        assert_eq!(
            front_matter(text).expect("valid front matter"),
            "id: RUST-DOC-0001"
        );
    }

    #[test]
    fn workspace_version_comes_from_workspace_package_section() {
        let cargo = "[package]\nversion = \"9.9.9\"\n\n[workspace.package]\nversion = \"0.2.0\"\n";
        assert_eq!(workspace_package_version(cargo), Some("0.2.0"));
    }

    #[test]
    fn manifest_paths_reject_parent_components() {
        assert!(valid_manifest_path("agents/shared.md"));
        assert!(!valid_manifest_path("../shared.md"));
        assert!(!valid_manifest_path("agents/../shared.md"));
    }

    #[test]
    fn normative_terms_require_whole_uppercase_words() {
        assert!(contains_normative_term("Callers MUST validate."));
        assert!(!contains_normative_term("Callers must validate."));
        assert!(!contains_normative_term("MAYBE is not a normative term."));
    }

    #[test]
    fn structured_fields_reject_lowercase_register() {
        let mut diagnostics = Vec::new();
        check_structured_field_register(
            std::path::Path::new("doctrine.md"),
            "**Applicability.** lower-case fragment.\n**Review evidence.** Capitalized artifact.\n",
            &mut diagnostics,
        );
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("capitalized noun-phrase"));
    }
}
