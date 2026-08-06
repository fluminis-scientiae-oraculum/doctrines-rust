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

/// The one document that carries the local validation sequence. Every other
/// governance document links to it rather than repeating it.
const VALIDATION_SEQUENCE_OWNER: &str = "README.md";

/// Governance documents that could plausibly carry a validation sequence, and are
/// therefore counted.
const VALIDATION_SEQUENCE_DOCUMENTS: &[&str] = &[
    "README.md",
    "AGENTS.md",
    "CONTRIBUTING.md",
    ".github/pull_request_template.md",
];

/// Commands whose co-occurrence in one fenced block identifies a validation
/// sequence rather than a passing mention of a single command.
const VALIDATION_SEQUENCE_COMMANDS: &[&str] = &[
    "cargo fmt --all --check",
    "cargo clippy --workspace --all-targets --all-features",
    "cargo test --workspace --all-features",
    "cargo run -p doctrine-lint -- check",
    "cargo run -p bundle-agent-context -- check",
    "cargo deny check",
];

/// Distinct commands that make a fenced block a copy of the sequence.
const VALIDATION_SEQUENCE_THRESHOLD: usize = 3;

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

    let rules = check_doctrines(root, &doctrine_manifest, &mut diagnostics);
    check_doctrine_index(root, &doctrine_manifest, &mut diagnostics);
    check_stated_counts(root, &doctrine_manifest, &rules, &mut diagnostics);
    check_rule_citations(root, &rules.all, &mut diagnostics);
    check_validation_sequence_copies(root, &mut diagnostics);
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

/// Rule identifiers defined across the corpus, split by the status of the doctrine
/// that defines them.
///
/// The two sets answer different questions and must not be conflated. A citation of
/// a rule in a deprecated doctrine still resolves, so citations check `all`. A
/// counted claim about current normative rules must exclude retired ones, so it
/// counts `active`.
#[derive(Debug, Default)]
struct RuleInventory {
    all: BTreeSet<String>,
    active: BTreeSet<String>,
}

/// Returns the rule identifiers actually defined across the corpus, so later checks
/// compare citations and counted claims against what exists rather than against a
/// second list.
fn check_doctrines(
    root: &Path,
    manifest: &DoctrineManifest,
    diagnostics: &mut Vec<Diagnostic>,
) -> RuleInventory {
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

    let mut inventory = RuleInventory::default();
    for entry in &manifest.doctrines {
        // A set is sorted rather than insertion-ordered, so the rules this entry
        // contributed are its difference against the set as it stood before.
        let before = inventory.all.clone();
        check_doctrine_entry(root, entry, &known_ids, &mut inventory.all, diagnostics);
        if entry.status == "active" {
            let added: Vec<String> = inventory.all.difference(&before).cloned().collect();
            inventory.active.extend(added);
        }
    }
    inventory
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

/// Whether a path is a finalized decision document rather than maintained material.
///
/// Lifecycle, not directory membership, decides this. `RUST-DOC-0011-R011` and
/// `RUST-DOC-0011-R019` permit an artifact to state the contract as it stood when a
/// decision was taken and then stop being maintained; rewriting one to satisfy a
/// linter would destroy the record. Three classes qualify:
///
/// - the RFC documents themselves, in any lifecycle directory. A finalized RFC
///   states counts and rule identifiers as of its decision, and a *proposed* RFC
///   may name identifiers that do not exist yet, so both are exempt;
/// - archived decision records under `decisions/archive/`, which
///   `RUST-DOC-0011-R009` marks as no longer current authority;
/// - `CHANGELOG.md`, which is a dated release record.
///
/// Everything else under `rfcs/` is maintained governance and stays scanned:
/// `rfcs/README.md` continues to govern the change process under
/// `RUST-DOC-0011-R011`, `rfcs/accepted/overview.md` is the canonical prose source
/// of a generated index, and the state-directory READMEs describe current policy.
///
/// Template and proposal *skeletons* need no exemption: they write hypothetical
/// identifiers with letter positions, as in `RUST-DOC-NNNN-R001`, which the
/// citation extractor never matches. That is asserted by test rather than assumed.
fn is_dated_record(relative: &str) -> bool {
    if relative == "CHANGELOG.md" || relative.starts_with("decisions/archive/") {
        return true;
    }
    let Some(rest) = relative.strip_prefix("rfcs/") else {
        return false;
    };
    let Some((_state, file)) = rest.split_once('/') else {
        return false;
    };
    let markdown = Path::new(file).extension().and_then(|value| value.to_str()) == Some("md");
    file.starts_with("RFC-") && markdown
}

/// Every Markdown file at the repository root, discovered rather than listed.
///
/// `EVIDENCE.md` escaped the drift checks and the normative-scope scan because it
/// was missing from a hardcoded inventory. Enumerating the directory removes the
/// class: a future root document is covered on the day it is added.
fn root_documents(root: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(root) else {
        return Vec::new();
    };
    let mut paths: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file() && path.extension().and_then(|value| value.to_str()) == Some("md")
        })
        .collect();
    paths.sort();
    paths
}

/// Files both drift checks scan: maintained canonical Markdown plus the root
/// documents, excluding dated records.
fn maintained_markdown(root: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let push_if_maintained = |path: &Path, paths: &mut Vec<PathBuf>| {
        let relative = path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        if !is_dated_record(&relative) {
            paths.push(path.to_path_buf());
        }
    };

    for path in root_documents(root) {
        push_if_maintained(&path, &mut paths);
    }
    for directory in CANONICAL_ROOTS {
        let mut collected = Vec::new();
        collect_files(&root.join(directory), &mut |path| {
            if path.extension().and_then(|extension| extension.to_str()) == Some("md") {
                collected.push(path.to_path_buf());
            }
        });
        for path in collected {
            push_if_maintained(&path, &mut paths);
        }
    }
    paths
}

/// Rejects a counted claim in prose that disagrees with the corpus.
///
/// "The 207 normative rules" is a machine-derivable fact restated by hand, which is
/// the shape of drift that `RUST-DOC-0011-R004` prohibits and that no reader can
/// detect. The count is recomputed and compared rather than trusted.
fn check_stated_counts(
    root: &Path,
    manifest: &DoctrineManifest,
    rules: &RuleInventory,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let active_doctrines = manifest
        .doctrines
        .iter()
        .filter(|entry| entry.status == "active")
        .count();
    // Each phrase sits on one dimension. "Normative rules" means the rules in force,
    // so retired doctrines' rules are excluded. "Doctrine packages" is physical and
    // counts every package the manifest carries, active or not. "Active doctrines"
    // is the status count. Conflating them would let a deprecated doctrine be
    // reported as current, or the real package count be rejected as wrong.
    let expectations: [(&str, usize); 3] = [
        ("normative rules", rules.active.len()),
        ("doctrine packages", manifest.doctrines.len()),
        ("active doctrines", active_doctrines),
    ];

    for path in maintained_markdown(root) {
        let Some(text) = read_text(&path, diagnostics) else {
            continue;
        };
        for (line_index, line) in text.lines().enumerate() {
            for (phrase, expected) in expectations {
                for stated in stated_counts(line, phrase) {
                    if stated != expected {
                        diagnostics.push(Diagnostic::new(
                            &path,
                            format!(
                                "line {} states {stated} {phrase}; the corpus has {expected}",
                                line_index + 1
                            ),
                        ));
                    }
                }
            }
        }
    }
}

/// Integers immediately preceding `phrase` on one line.
fn stated_counts(line: &str, phrase: &str) -> Vec<usize> {
    let mut counts = Vec::new();
    for (offset, _) in line.match_indices(phrase) {
        let prefix = line[..offset].trim_end();
        let digits: String = prefix
            .chars()
            .rev()
            .take_while(char::is_ascii_digit)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        if digits.is_empty() {
            continue;
        }
        // "0.4.1 normative rules" is a version, not a count.
        if prefix[..prefix.len() - digits.len()].ends_with('.') {
            continue;
        }
        if let Ok(value) = digits.parse::<usize>() {
            counts.push(value);
        }
    }
    counts
}

/// Rejects a citation of a rule identifier no doctrine defines.
///
/// The corpus already checks that every rule appears in its own review standard.
/// Nothing checked the other direction, so a renamed or removed rule left dangling
/// citations that read as authoritative.
fn check_rule_citations(
    root: &Path,
    rule_ids: &BTreeSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for path in maintained_markdown(root) {
        let Some(text) = read_text(&path, diagnostics) else {
            continue;
        };
        for cited in extract_rule_citations(&text) {
            if !rule_ids.contains(&cited) {
                diagnostics.push(Diagnostic::new(
                    &path,
                    format!("cites {cited}, which no doctrine defines"),
                ));
            }
        }
    }
}

/// Rule identifiers cited anywhere in a document, in `RUST-DOC-NNNN-RNNN` form.
///
/// Template material uses letter positions such as `RUST-DOC-NNNN-R001`, which does
/// not match and is therefore never reported.
fn extract_rule_citations(text: &str) -> BTreeSet<String> {
    let mut cited = BTreeSet::new();
    for (offset, _) in text.match_indices("RUST-DOC-") {
        let rest = &text[offset + "RUST-DOC-".len()..];
        let doctrine: String = rest.chars().take(4).collect();
        if doctrine.len() != 4 || !doctrine.bytes().all(|byte| byte.is_ascii_digit()) {
            continue;
        }
        let after = &rest[doctrine.len()..];
        let Some(after) = after.strip_prefix("-R") else {
            continue;
        };
        let rule: String = after.chars().take(3).collect();
        if rule.len() != 3 || !rule.bytes().all(|byte| byte.is_ascii_digit()) {
            continue;
        }
        // Reject a longer run of digits, so `-R0011` is not read as `-R001`.
        if after[rule.len()..].starts_with(|character: char| character.is_ascii_digit()) {
            continue;
        }
        cited.insert(format!("RUST-DOC-{doctrine}-R{rule}"));
    }
    cited
}

/// Rejects a second copy of the local validation sequence.
///
/// The sequence is one fact about how this repository is checked. It was carried in
/// full by three governance documents at once, so a change to any gate had to be
/// made in three places correctly, with nothing announcing a miss. This counts the
/// copies and requires exactly one, in the document that owns it, which is
/// `RUST-DOC-0011-R017` applied to the repository's own governance.
fn check_validation_sequence_copies(root: &Path, diagnostics: &mut Vec<Diagnostic>) {
    let mut carriers = Vec::new();
    for document in VALIDATION_SEQUENCE_DOCUMENTS {
        let path = root.join(document);
        if !path.is_file() {
            continue;
        }
        let Some(text) = read_text(&path, diagnostics) else {
            continue;
        };
        let copies = validation_sequence_copies(&text);
        if copies > 0 {
            carriers.push((*document, copies));
        }
    }

    let total: usize = carriers.iter().map(|(_, copies)| copies).sum();
    if total == 1
        && carriers
            .iter()
            .all(|(document, _)| *document == VALIDATION_SEQUENCE_OWNER)
    {
        return;
    }

    let listed = carriers
        .iter()
        .map(|(document, copies)| format!("{document} ({copies})"))
        .collect::<Vec<_>>()
        .join(", ");
    diagnostics.push(Diagnostic::new(
        root.join(VALIDATION_SEQUENCE_OWNER),
        format!(
            "the validation sequence must appear once, in {VALIDATION_SEQUENCE_OWNER}; found {total} copy/copies in [{listed}]"
        ),
    ));
}

/// Fenced blocks carrying at least [`VALIDATION_SEQUENCE_THRESHOLD`] distinct
/// validation commands. A document naming one command in passing is not a copy.
fn validation_sequence_copies(text: &str) -> usize {
    let mut copies = 0;
    let mut in_fence = false;
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            if in_fence {
                if seen.len() >= VALIDATION_SEQUENCE_THRESHOLD {
                    copies += 1;
                }
                seen.clear();
            }
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            for command in VALIDATION_SEQUENCE_COMMANDS {
                if line.contains(command) {
                    seen.insert(*command);
                }
            }
        }
    }
    copies
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

    let rows = doctrine_index_rows(&index);

    for entry in &manifest.doctrines {
        if entry.status != "active" {
            continue;
        }
        // A parsed table row, not a same-line mention. A sentence carrying both the
        // identifier and the title would otherwise satisfy the check while the row
        // it claims to verify is absent.
        let has_row = rows.iter().any(|(id, cells)| {
            id == &entry.id && cells.iter().any(|cell| cell.contains(&entry.title))
        });
        if !has_row {
            diagnostics.push(Diagnostic::new(
                &index_path,
                format!(
                    "doctrine index has no table row naming {} with its manifest title {:?}",
                    entry.id, entry.title
                ),
            ));
        }
    }

    for (id, _) in &rows {
        if !manifest
            .doctrines
            .iter()
            .any(|entry| &entry.id == id && entry.status == "active")
        {
            diagnostics.push(Diagnostic::new(
                &index_path,
                format!("doctrine index has a row for {id}, which is not an active doctrine"),
            ));
        }
    }
}

/// Table rows of the doctrine index, as `(doctrine id, cells)`.
///
/// A row qualifies when one whole cell is exactly a doctrine identifier. Scanning
/// prose for identifiers would report a sentence about a retired doctrine as an
/// index entry, and matching a line rather than a row would accept a sentence in
/// place of the row it stands for.
fn doctrine_index_rows(text: &str) -> Vec<(String, Vec<String>)> {
    let mut rows = Vec::new();
    for line in text.lines() {
        let Some(cells) = table_row_cells(line) else {
            continue;
        };
        if let Some(id) = cells.iter().find(|cell| is_doctrine_id(cell)) {
            rows.push((id.clone(), cells));
        }
    }
    rows
}

/// Trimmed cells of a Markdown table row, or `None` for any other line, including
/// the header separator.
fn table_row_cells(line: &str) -> Option<Vec<String>> {
    let trimmed = line.trim();
    if trimmed.len() < 2 || !trimmed.starts_with('|') || !trimmed.ends_with('|') {
        return None;
    }
    let cells: Vec<String> = trimmed[1..trimmed.len() - 1]
        .split('|')
        .map(|cell| cell.trim().to_owned())
        .collect();
    let separator = cells
        .iter()
        .all(|cell| !cell.is_empty() && cell.chars().all(|value| value == '-' || value == ':'));
    if separator { None } else { Some(cells) }
}

/// Whether a cell is exactly a doctrine identifier, so `RUST-DOC-0011-R004` and any
/// surrounding prose are excluded.
fn is_doctrine_id(cell: &str) -> bool {
    let Some(digits) = cell.strip_prefix("RUST-DOC-") else {
        return false;
    };
    digits.len() == 4 && digits.bytes().all(|byte| byte.is_ascii_digit())
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
    for path in root_documents(root) {
        scan_normative_scope(root, &path, diagnostics);
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
        extract_rule_citations, extract_rule_headings, front_matter, is_dated_record,
        stated_counts, valid_manifest_path, valid_record_id, valid_rule_id,
        workspace_package_version,
    };
    use super::{
        RuleInventory, check_stated_counts, doctrine_index_rows, root_documents, table_row_cells,
        validation_sequence_copies,
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
    fn stated_counts_read_the_integer_before_the_phrase() {
        assert_eq!(
            stated_counts("The 207 normative rules define", "normative rules"),
            [207]
        );
        assert_eq!(
            stated_counts("from 187 to 207 normative rules", "normative rules"),
            [207]
        );
        assert_eq!(
            stated_counts("no digits here normative rules", "normative rules"),
            [] as [usize; 0]
        );
        // A version is not a count.
        assert_eq!(
            stated_counts("shipped with 0.4.1 normative rules", "normative rules"),
            [] as [usize; 0]
        );
    }

    #[test]
    fn rule_citations_ignore_template_and_longer_identifiers() {
        let cited = extract_rule_citations(concat!(
            "Applies under `RUST-DOC-0011-R004` and RUST-DOC-0010-R022.\n",
            "Template material writes RUST-DOC-NNNN-R001.\n",
            "A longer run such as RUST-DOC-0011-R0011 is not a citation.\n",
        ));
        assert!(cited.contains("RUST-DOC-0011-R004"), "{cited:?}");
        assert!(cited.contains("RUST-DOC-0010-R022"), "{cited:?}");
        assert_eq!(cited.len(), 2, "{cited:?}");
    }

    /// The exemption follows artifact lifecycle, not directory membership. Both
    /// sides are asserted: finalized records are exempt, and the governance around
    /// them stays scanned.
    #[test]
    fn the_historical_exemption_follows_lifecycle_not_directory() {
        for frozen in [
            "rfcs/accepted/RFC-0001-isolation-and-time-assumptions.md",
            "rfcs/superseded/RFC-0000-example.md",
            "rfcs/proposed/RFC-0004-draft.md",
            "decisions/archive/adr-0002-old.md",
            "CHANGELOG.md",
        ] {
            assert!(is_dated_record(frozen), "{frozen} should be exempt");
        }
        for maintained in [
            "rfcs/README.md",
            "rfcs/accepted/README.md",
            "rfcs/accepted/overview.md",
            "rfcs/proposed/README.md",
            "rfcs/superseded/README.md",
            "rfcs/template.md",
            "decisions/README.md",
            "decisions/active/adr-0001-residency.md",
            "EVIDENCE.md",
            "doctrines/README.md",
        ] {
            assert!(
                !is_dated_record(maintained),
                "{maintained} should stay scanned"
            );
        }
    }

    /// Root documents are discovered, not listed. `EVIDENCE.md` escaped the checks
    /// because a hardcoded inventory omitted it; set equality against the directory
    /// closes the class rather than that instance.
    #[test]
    fn root_documents_equal_the_markdown_files_on_disk() {
        let root = temporary_directory("root-documents");
        for name in ["README.md", "EVIDENCE.md", "ARCHITECTURE.md", "notes.txt"] {
            fs::write(root.join(name), "body\n").expect("write file");
        }
        fs::create_dir_all(root.join("doctrines")).expect("create directory");
        fs::write(root.join("doctrines/README.md"), "nested\n").expect("write nested");

        let found: BTreeSet<String> = root_documents(&root)
            .into_iter()
            .map(|path| {
                path.file_name()
                    .expect("file name")
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        let expected: BTreeSet<String> = ["ARCHITECTURE.md", "EVIDENCE.md", "README.md"]
            .into_iter()
            .map(str::to_owned)
            .collect();

        assert_eq!(found, expected, "a new root document must be discovered");
        fs::remove_dir_all(root).expect("remove temporary test directory");
    }

    #[test]
    fn index_rows_come_from_table_cells_not_prose() {
        let index = concat!(
            "| ID | Doctrine |\n",
            "| -- | -------- |\n",
            "| RUST-DOC-0001 | [Making Invalid States Unrepresentable](0001-invalid-states/) |\n",
            "Applied by `RUST-DOC-0011-R004` and `RUST-DOC-0010-R022`.\n",
            "Read RUST-DOC-0099, A Retired Doctrine, for history.\n",
        );
        let rows = doctrine_index_rows(index);
        assert_eq!(rows.len(), 1, "{rows:?}");
        assert_eq!(rows[0].0, "RUST-DOC-0001");
    }

    #[test]
    fn table_row_cells_reject_separators_and_prose() {
        assert_eq!(
            table_row_cells("| RUST-DOC-0001 | Title |"),
            Some(vec!["RUST-DOC-0001".to_owned(), "Title".to_owned()])
        );
        assert_eq!(table_row_cells("| --- | :--- |"), None);
        assert_eq!(table_row_cells("Read RUST-DOC-0001, Title, next."), None);
    }

    /// The defect the previous same-line predicate allowed: the row is gone, but a
    /// sentence carries both the identifier and the title.
    #[test]
    fn a_prose_sentence_does_not_stand_in_for_a_missing_row() {
        let manifest = doctrine_manifest();
        let root = temporary_directory("index-prose-mention");
        fs::create_dir_all(root.join("doctrines")).expect("create doctrines directory");
        fs::write(
            root.join(DOCTRINE_INDEX),
            concat!(
                "| RUST-DOC-0001 | Making Invalid States Unrepresentable |\n",
                "Read RUST-DOC-0002, Error Modeling as Domain Design, next.\n",
            ),
        )
        .expect("write index");

        let mut diagnostics = Vec::new();
        check_doctrine_index(&root, &manifest, &mut diagnostics);
        assert!(
            diagnostics.iter().any(|diagnostic| {
                diagnostic
                    .message
                    .contains("no table row naming RUST-DOC-0002")
            }),
            "{diagnostics:?}"
        );
        fs::remove_dir_all(root).expect("remove temporary test directory");
    }

    /// The complementary defect: prose about a doctrine the manifest does not carry
    /// as active must not be reported as an index row.
    #[test]
    fn prose_about_an_inactive_doctrine_is_not_an_index_row() {
        let manifest = doctrine_manifest();
        let root = temporary_directory("index-inactive-prose");
        fs::create_dir_all(root.join("doctrines")).expect("create doctrines directory");
        fs::write(
            root.join(DOCTRINE_INDEX),
            concat!(
                "| RUST-DOC-0001 | Making Invalid States Unrepresentable |\n",
                "| RUST-DOC-0002 | Error Modeling as Domain Design |\n",
                "RUST-DOC-0099 was withdrawn before adoption.\n",
            ),
        )
        .expect("write index");

        let mut diagnostics = Vec::new();
        check_doctrine_index(&root, &manifest, &mut diagnostics);
        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        fs::remove_dir_all(root).expect("remove temporary test directory");
    }

    #[test]
    fn counted_phrases_track_their_own_status_dimension() {
        let root = temporary_directory("counted-dimensions");
        let mut manifest = doctrine_manifest();
        manifest.doctrines.push(doctrine_entry(
            "RUST-DOC-0003",
            "0003-retired",
            "A Retired Doctrine",
            "deprecated",
        ));
        // Two rules in force, one retired: three packages, two active doctrines.
        let rules = RuleInventory {
            all: [
                "RUST-DOC-0001-R001",
                "RUST-DOC-0002-R001",
                "RUST-DOC-0003-R001",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            active: ["RUST-DOC-0001-R001", "RUST-DOC-0002-R001"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
        };

        fs::write(
            root.join("EVIDENCE.md"),
            "The 2 normative rules, 3 doctrine packages, and 2 active doctrines.\n",
        )
        .expect("write evidence");
        let mut diagnostics = Vec::new();
        check_stated_counts(&root, &manifest, &rules, &mut diagnostics);
        assert!(diagnostics.is_empty(), "{diagnostics:?}");

        // Counting retired rules as current, or packages by active status, must fail.
        fs::write(
            root.join("EVIDENCE.md"),
            "The 3 normative rules and 2 doctrine packages.\n",
        )
        .expect("rewrite evidence");
        let mut diagnostics = Vec::new();
        check_stated_counts(&root, &manifest, &rules, &mut diagnostics);
        let messages = diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            messages.contains("states 3 normative rules; the corpus has 2"),
            "{messages}"
        );
        assert!(
            messages.contains("states 2 doctrine packages; the corpus has 3"),
            "{messages}"
        );

        fs::remove_dir_all(root).expect("remove temporary test directory");
    }

    #[test]
    fn the_validation_sequence_is_counted_per_fenced_block() {
        let one_copy = concat!(
            "```bash\n",
            "cargo fmt --all --check\n",
            "cargo test --workspace --all-features\n",
            "cargo deny check\n",
            "```\n",
        );
        assert_eq!(validation_sequence_copies(one_copy), 1);

        // A single command named in passing is not a copy of the sequence.
        let passing_mention = "```bash\ncargo run -p doctrine-lint -- check\n```\n";
        assert_eq!(validation_sequence_copies(passing_mention), 0);

        let two_copies = format!("{one_copy}\nprose\n\n{one_copy}");
        assert_eq!(validation_sequence_copies(&two_copies), 2);
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
            messages.contains("no table row naming RUST-DOC-0002"),
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
            diagnostics.iter().any(|diagnostic| diagnostic
                .message
                .contains("no table row naming RUST-DOC-0002")),
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
