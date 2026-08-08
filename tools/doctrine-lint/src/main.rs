use doctrine_manifest::{
    AgentManifest, AgentPack, AgentRole, DecisionRecordMetadata, DecisionRecordRegistry,
    DoctrineEntry, DoctrineManifest, DoctrineMetadata, RecordStatus, SourcePolicy, Verbosity,
    front_matter, markers, states_obligations,
};
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

/// Maintained Markdown that legitimately has nothing linking to it, and why.
///
/// A reader reaches every other canonical document by clicking. These three are not
/// reached that way: one is where a reader starts, and two are inputs a generator
/// reads rather than pages anyone opens. Each is named individually, so a file cannot
/// acquire the exemption by resembling one.
/// Directories the reachability gate walks beyond the canonical roots.
///
/// Each holds maintained Markdown a reader navigates but no doctrine projection
/// carries: scaffolding, discovery schemas, and the tool and example indexes. They are
/// listed here rather than added to `CANONICAL_ROOTS` because that list also selects
/// what the normative-term scan and the drift checks read, and widening it would apply
/// those rules to files deliberately outside them.
const REACHABILITY_EXTRA_ROOTS: &[&str] = &["templates", "manifest", "tools", "examples"];

const REACHABILITY_EXEMPTIONS: &[(&str, &str)] = &[
    (
        "README.md",
        "the repository entry point, which a reader opens directly",
    ),
    (
        "doctrines/map-overview.md",
        "a generator input for doctrines/map.md rather than a page",
    ),
    (
        "rfcs/accepted/overview.md",
        "a generator input for rfcs/accepted/README.md rather than a page",
    ),
];

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

    /// Whether a record declaring this status belongs in this registry list. Both
    /// sides are typed, so a status outside the vocabulary cannot reach the
    /// comparison; it fails when the record's front matter is decoded.
    fn accepts(self, status: RecordStatus) -> bool {
        match self {
            Self::Active => matches!(status, RecordStatus::Active),
            Self::Archived => matches!(
                status,
                RecordStatus::Superseded | RecordStatus::Expired | RecordStatus::Archival
            ),
        }
    }
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
    check_evidence_rule_counts(root, &doctrine_manifest, &rules, &mut diagnostics);
    check_rule_citations(root, &rules.all, &mut diagnostics);
    check_validation_sequence_copies(root, &mut diagnostics);
    check_repository_version(root, &doctrine_manifest, &mut diagnostics);
    check_agents(root, &agent_manifest, &doctrine_manifest, &mut diagnostics);
    check_decision_records(root, &agent_manifest, &mut diagnostics);
    check_forbidden_markers(root, &mut diagnostics);
    check_normative_scope(root, &mut diagnostics);
    check_verbosity_annotations(root, &doctrine_manifest, &agent_manifest, &mut diagnostics);
    check_reserved_ceiling(root, &agent_manifest, &mut diagnostics);
    check_alert_vocabulary(root, &mut diagnostics);
    check_reachability(root, &mut diagnostics);
    check_path_references(root, &mut diagnostics);
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
        if entry.status.is_active() {
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
    if entry.status.is_active() && entry.superseded_by.is_some() {
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
///
/// A directory that cannot be read is reported rather than treated as empty. An
/// unreadable root would otherwise scan nothing and let every root-document check
/// pass, which is the same silent-omission failure the enumeration replaced. Unlike
/// the canonical-root walks, an absent root is reported too: this directory is the
/// repository, so its absence is never a legitimate empty.
///
/// Entries are classified by `DirEntry::file_type` for the reasons `classified_entries`
/// states, so a symbolic link is reported rather than scanned as its target.
fn root_documents(root: &Path, diagnostics: &mut Vec<Diagnostic>) -> Vec<PathBuf> {
    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(error) => {
            diagnostics.push(Diagnostic::new(
                root,
                format!("cannot enumerate the repository root: {error}"),
            ));
            return Vec::new();
        }
    };

    let mut paths = Vec::new();
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                diagnostics.push(Diagnostic::new(
                    root,
                    format!("cannot read a repository-root entry: {error}"),
                ));
                continue;
            }
        };
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("md") {
            continue;
        }
        match entry.file_type() {
            Ok(file_type) if file_type.is_symlink() => diagnostics.push(Diagnostic::new(
                &path,
                "symbolic link is not scanned; a root document must be a regular file \
                 inside the repository",
            )),
            Ok(file_type) if file_type.is_file() => paths.push(path),
            Ok(_) => {}
            Err(error) => diagnostics.push(Diagnostic::new(
                &path,
                format!("cannot classify a repository-root entry: {error}"),
            )),
        }
    }
    paths.sort();
    paths
}

/// Files both drift checks scan: maintained canonical Markdown plus the root
/// documents, excluding dated records.
fn maintained_markdown(root: &Path, diagnostics: &mut Vec<Diagnostic>) -> Vec<PathBuf> {
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

    for path in root_documents(root, diagnostics) {
        push_if_maintained(&path, &mut paths);
    }
    for directory in CANONICAL_ROOTS {
        let mut collected = Vec::new();
        collect_files(&root.join(directory), diagnostics, &mut |path| {
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
    let active_doctrines = manifest.active().count();
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

    let scanned = maintained_markdown(root, diagnostics);
    for path in scanned {
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
    let scanned = maintained_markdown(root, diagnostics);
    for path in scanned {
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

    for entry in manifest.active() {
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
            .any(|entry| &entry.id == id && entry.status.is_active())
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
            diagnostics.push(Diagnostic::new(&readme_path, message.to_string()));
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
        metadata.status.as_str(),
        entry.status.as_str(),
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
    // The role vocabulary is the manifest schema's, decoded into `AgentRole`. Listing
    // the roles here again would be the second maintained copy RUST-DOC-0011-R004
    // prohibits.
    let expected_ids: BTreeSet<AgentRole> = AgentRole::ALL.into_iter().collect();
    let actual_ids: BTreeSet<AgentRole> = manifest.packs.iter().map(|pack| pack.id).collect();
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
    // `maximum_verbosity` needs no vocabulary check here. It decodes into `Verbosity`, so
    // a value outside the schema fails when the manifest is parsed. Which tiers a pack may
    // declare is a separate question, answered by `check_reserved_ceiling`.
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
            diagnostics.push(Diagnostic::new(&record_path, message.to_string()));
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
    if !bucket.accepts(metadata.status) {
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
    let mut unreadable = Vec::new();
    collect_repository_files(root, root, &mut unreadable, &mut |path| {
        scan_marker_file(path, diagnostics);
    });
    diagnostics.append(&mut unreadable);
}

fn check_normative_scope(root: &Path, diagnostics: &mut Vec<Diagnostic>) {
    for path in root_documents(root, diagnostics) {
        scan_normative_scope(root, &path, diagnostics);
    }
    let mut unreadable = Vec::new();
    for directory in CANONICAL_ROOTS {
        collect_files(&root.join(directory), &mut unreadable, &mut |path| {
            if path.extension().and_then(|extension| extension.to_str()) == Some("md") {
                scan_normative_scope(root, path, diagnostics);
            }
        });
    }
    diagnostics.append(&mut unreadable);
}

/// Scans one file for forbidden filler markers.
///
/// A file this repository-wide gate cannot read is reported. Skipping it silently
/// let the gate announce a clean repository having never opened the file, which is
/// the same false success as walking an unreadable directory as empty.
///
/// Content that is not UTF-8 is not a failure and stays silent: the markers are
/// Markdown filler, so a binary file carries none and is not evidence of anything.
fn scan_marker_file(path: &Path, diagnostics: &mut Vec<Diagnostic>) {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::InvalidData => return,
        Err(error) => {
            diagnostics.push(Diagnostic::new(
                path,
                format!("cannot read file for the forbidden-marker scan: {error}"),
            ));
            return;
        }
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

/// Rejects maintained Markdown that no other maintained Markdown file links to.
///
/// The corpus reached 245 files with 104 of them unreachable by clicking, because a
/// package index named its siblings in backticks rather than links. Backticked prose
/// looks like a reference and navigates nowhere, and nothing detected the difference.
///
/// This checks inbound links, not reachability from the root: a file linked only by an
/// unreachable file still passes. That is the weaker claim, and it is the one made here
/// rather than the one the name might suggest.
fn check_reachability(root: &Path, diagnostics: &mut Vec<Diagnostic>) {
    let files = reachability_scope(root, diagnostics);
    let mut linked: BTreeSet<String> = BTreeSet::new();

    for file in &files {
        let Some(text) = read_text(file, diagnostics) else {
            continue;
        };
        for href in outbound_links(&text) {
            let Some(relative) = resolve_link(root, file, &href) else {
                continue;
            };
            // A link to a directory reaches that directory's index, so credit it.
            let absolute = root.join(&relative);
            if absolute.is_dir() {
                linked.insert(format!("{relative}/README.md"));
            }
            linked.insert(relative);
        }
    }

    for file in &files {
        let relative = file
            .strip_prefix(root)
            .unwrap_or(file)
            .to_string_lossy()
            .replace('\\', "/");
        if REACHABILITY_EXEMPTIONS
            .iter()
            .any(|(exempt, _)| *exempt == relative)
        {
            continue;
        }
        if !linked.contains(&relative) {
            diagnostics.push(Diagnostic::new(
                file,
                "no maintained Markdown file links to it; add a link from its index, or \
                 exempt it in REACHABILITY_EXEMPTIONS with a stated reason",
            ));
        }
    }
}

/// Rejects a backticked path that resolves to a real file and is never linked.
///
/// A reader cannot click `foundations/invariants.md`. Backticked prose reads like a
/// reference and navigates nowhere, which is how 104 files ended up unreachable and how
/// 133 further mentions stayed inert after that was fixed. The rule is per document and
/// per target: mention a path as many times as reads well, but link it at least once.
///
/// A directory under `dist/` is never required, and must not be linked. The bundler
/// rewrites a link relative to the output carrying it, so a canonical file emitted into
/// `dist/<x>/` that links the `dist/<x>` directory produces an empty relative path and
/// then `(/)`. Only the link checker catches that, and only in the generated file.
fn check_path_references(root: &Path, diagnostics: &mut Vec<Diagnostic>) {
    for file in reachability_scope(root, diagnostics) {
        let Some(text) = read_text(&file, diagnostics) else {
            continue;
        };

        let mut linked: BTreeSet<String> = BTreeSet::new();
        for href in outbound_links(&text) {
            if let Some(target) = resolve_link(root, &file, &href) {
                linked.insert(target);
            }
        }

        let relative_file = file
            .strip_prefix(root)
            .unwrap_or(&file)
            .to_string_lossy()
            .replace('\\', "/");

        // A generated projection is not hand-edited, so reporting an inert path in one
        // would name a file no author can fix. Its source is scanned instead.
        if GENERATED_IN_CANONICAL_ROOTS.contains(&relative_file.as_str()) {
            continue;
        }

        let mut reported: BTreeSet<String> = BTreeSet::new();
        for span in inline_code_spans(&text) {
            if !looks_like_path(&span) {
                continue;
            }
            // A mention is written either relative to its own file or from the
            // repository root, and both forms are used throughout the corpus. Trying
            // only the first silently skipped every root-relative mention, which is
            // most of them, and left this check reporting nothing at all.
            let Some(target) = resolve_link(root, &file, &span)
                .filter(|candidate| root.join(candidate).exists())
                .or_else(|| {
                    let from_root = span.trim_end_matches('/').to_owned();
                    root.join(&from_root).exists().then_some(from_root)
                })
            else {
                continue;
            };
            let absolute = root.join(&target);
            if target == relative_file {
                continue;
            }
            if absolute.is_dir() && (target == "dist" || target.starts_with("dist/")) {
                continue;
            }
            if linked.contains(&target) || !reported.insert(target.clone()) {
                continue;
            }
            diagnostics.push(Diagnostic::new(
                &file,
                format!(
                    "mentions `{span}` as code but never links it; link the first mention, \
                     so a reader can reach {target}"
                ),
            ));
        }
    }
}

/// Whether an inline-code span is shaped like a repository path rather than an
/// identifier. A bare word is only a candidate when it carries a known file extension,
/// so `unwrap` and `NonZeroU64` are not mistaken for paths.
fn looks_like_path(span: &str) -> bool {
    if span.is_empty() || span.contains(' ') || span.contains("://") {
        return false;
    }
    if !span
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '/' | '-'))
    {
        return false;
    }
    span.contains('/')
        || matches!(
            Path::new(span).extension().and_then(|e| e.to_str()),
            Some("md" | "yaml" | "yml" | "toml" | "rs" | "json" | "jsonc")
        )
}

/// Inline-code spans in prose, skipping front matter, fenced blocks, and the text of
/// existing links.
///
/// Link constructs are removed before scanning, so `[`doctrine.md`](doctrine.md)` does
/// not report the very path it already links.
fn inline_code_spans(text: &str) -> Vec<String> {
    let body = match text.strip_prefix("---\n") {
        Some(rest) => rest.find("\n---\n").map_or(text, |end| &rest[end + 5..]),
        None => text,
    };

    let mut prose = String::new();
    let mut in_fence = false;
    for line in body.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        if !in_fence {
            prose.push_str(line);
            prose.push('\n');
        }
    }

    let without_links = remove_link_constructs(&prose);

    // Scanned per line, so one unbalanced backtick cannot flip the parity of the whole
    // document and reclassify every span after it.
    let mut spans = Vec::new();
    for line in without_links.lines() {
        for (index, chunk) in line.split('`').enumerate() {
            if index % 2 == 1 {
                spans.push(chunk.trim().to_owned());
            }
        }
    }
    spans
}

/// Removes every `[text](destination)` construct, so its contents are not rescanned.
///
/// A bracket only opens a link when its `]` is immediately followed by `(`. Searching
/// for the next `](` anywhere ahead instead made a callout marker such as `[!NOTE]`
/// swallow everything up to the next real link, which silently emptied the text this
/// scan reads and made the check pass on a document it had never examined.
fn remove_link_constructs(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    loop {
        let Some(open) = rest.find('[') else {
            out.push_str(rest);
            return out;
        };
        out.push_str(&rest[..open]);
        let after_open = &rest[open + 1..];

        let is_link = after_open
            .find(']')
            .filter(|close| after_open[close + 1..].starts_with('('))
            .and_then(|close| {
                let tail = &after_open[close + 2..];
                tail.find(')').map(|end| &tail[end + 1..])
            });

        if let Some(remainder) = is_link {
            rest = remainder;
        } else {
            out.push('[');
            rest = after_open;
        }
    }
}

/// Every Markdown file the reachability gate holds to an inbound link.
///
/// This is the canonical set plus [`REACHABILITY_EXTRA_ROOTS`], and it is assembled
/// here rather than by widening `CANONICAL_ROOTS`. Those roots also drive the
/// normative-term scan and both drift checks, and `templates/` is deliberately exempt
/// from the marker scan because scaffolding text is expected there. Navigation is a
/// different question from normative scope, so it gets a different set.
fn reachability_scope(root: &Path, diagnostics: &mut Vec<Diagnostic>) -> Vec<PathBuf> {
    let mut paths = maintained_markdown(root, diagnostics);
    for directory in REACHABILITY_EXTRA_ROOTS {
        let mut collected = Vec::new();
        collect_files(&root.join(directory), diagnostics, &mut |path| {
            if path.extension().and_then(|value| value.to_str()) == Some("md") {
                collected.push(path.to_path_buf());
            }
        });
        paths.append(&mut collected);
    }
    paths.sort();
    paths.dedup();
    paths
}

/// Link destinations in Markdown prose, skipping fenced code.
///
/// Fenced content is skipped for the reason the normative-term scan skips it: a path
/// inside an example is not navigation, and counting it would let a code sample satisfy
/// the reachability of a real document.
fn outbound_links(text: &str) -> Vec<String> {
    let mut targets = Vec::new();
    let mut in_fence = false;
    for line in text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        let mut index = 0;
        while let Some(offset) = line[index..].find("](") {
            let start = index + offset + 2;
            let Some(length) = line[start..].find(')') else {
                break;
            };
            targets.push(line[start..start + length].to_owned());
            index = start + length + 1;
        }
    }
    targets
}

/// Resolves one Markdown link to a repository-relative path, or `None` when it does not
/// name a file in this repository.
///
/// Normalization is textual rather than `canonicalize`, because a link to a path that
/// does not exist must resolve to a comparable string rather than an error; the link
/// checker owns whether a target exists, and this check owns only what points where.
fn resolve_link(root: &Path, from: &Path, href: &str) -> Option<String> {
    let target = href.split('#').next()?.trim();
    if target.is_empty() || target.contains("://") || target.starts_with("mailto:") {
        return None;
    }
    let joined = from.parent()?.join(target);

    let mut parts: Vec<std::ffi::OsString> = Vec::new();
    for component in joined.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                parts.pop();
            }
            other => parts.push(other.as_os_str().to_owned()),
        }
    }
    let mut absolute = PathBuf::new();
    for part in parts {
        absolute.push(part);
    }

    Some(
        absolute
            .strip_prefix(root)
            .ok()?
            .to_string_lossy()
            .replace('\\', "/"),
    )
}

/// The generated files that live inside a canonical root rather than under `dist/`.
///
/// Each carries a banner comment naming its sources, so each is exempt from the scan that
/// treats an HTML comment as a candidate verbosity annotation. They are named rather than
/// pattern-matched, so a hand-written file cannot acquire the exemption by resembling one.
const GENERATED_IN_CANONICAL_ROOTS: &[&str] = &["rfcs/accepted/README.md", "doctrines/map.md"];

/// The alert vocabulary the corpus uses, and what each one asserts.
///
/// A callout is closed to these five because the idiomatic use of a callout is to restate
/// the key point, and a hand-written restatement of an enforced claim is what
/// `RUST-DOC-0011-R004` prohibits. Each entry here instead marks a distinction the corpus
/// already draws, so the device carries meaning rather than emphasis.
const ALERT_VOCABULARY: &[&str] = &["NOTE", "TIP", "IMPORTANT", "WARNING", "CAUTION"];

/// The evidence map, which carries one hand-written rule count per doctrine.
const EVIDENCE_MAP: &str = "EVIDENCE.md";

/// Rejects a per-doctrine rule count in the evidence map that disagrees with the corpus.
///
/// `check_stated_counts` only sees an integer immediately before one of three literal
/// phrases, so a bare `| 22 |` in a table cell was invisible to it. That left eleven
/// machine-derivable integers maintained by hand with nothing checking them, which is the
/// drift `RUST-DOC-0011-R004` prohibits. Recomputing them is what makes the column a
/// checked view rather than a second source.
fn check_evidence_rule_counts(
    root: &Path,
    manifest: &DoctrineManifest,
    rules: &RuleInventory,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let path = root.join(EVIDENCE_MAP);
    let Some(text) = read_text(&path, diagnostics) else {
        return;
    };
    let rows: Vec<Vec<String>> = text.lines().filter_map(table_row_cells).collect();

    for entry in manifest.active() {
        let prefix = format!("{}-R", entry.id);
        let expected = rules
            .all
            .iter()
            .filter(|rule| rule.starts_with(&prefix))
            .count();
        let stated = rows
            .iter()
            .find(|cells| cells.iter().any(|cell| cell.contains(&entry.id)))
            .and_then(|cells| cells.iter().find_map(|cell| cell.parse::<usize>().ok()));
        match stated {
            Some(count) if count == expected => {}
            Some(count) => diagnostics.push(Diagnostic::new(
                &path,
                format!(
                    "states {count} rules for {}; the corpus has {expected}",
                    entry.id
                ),
            )),
            None => diagnostics.push(Diagnostic::new(
                &path,
                format!("has no table row stating a rule count for {}", entry.id),
            )),
        }
    }
}

/// Rejects an HTML comment in maintained canonical Markdown that is not a well-formed
/// verbosity annotation.
///
/// The sentinel is the comment opener rather than the word the annotation uses, so a near
/// miss is reported instead of silently doing nothing. The grammar itself lives in
/// `doctrine-manifest` and is called rather than restated, so this tool and the bundler
/// cannot disagree about which lines are annotations.
fn check_verbosity_annotations(
    root: &Path,
    doctrines: &DoctrineManifest,
    agents: &AgentManifest,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for path in maintained_markdown(root, diagnostics) {
        let relative = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        if GENERATED_IN_CANONICAL_ROOTS.contains(&relative.as_str()) {
            continue;
        }
        let Some(text) = read_text(&path, diagnostics) else {
            continue;
        };
        // Validation does not consult a ceiling; only whether the file states obligations
        // changes what is accepted. The ceiling a pack applies is the bundler's concern.
        let policy = if states_obligations(&relative, doctrines, agents) {
            SourcePolicy::Normative
        } else {
            SourcePolicy::Tiered(Verbosity::Exhaustive)
        };
        if let Err(error) = markers(&text, policy) {
            diagnostics.push(Diagnostic::new(&path, error.to_string()));
        }
    }
}

/// Rejects an agent pack declared at the widest verbosity the schema permits.
///
/// The widest tier is reserved so that a section annotated with it reaches
/// `dist/full-doctrine.md` and no agent pack. That reservation is what lets the corpus
/// gain reader-facing material without any pack growing, so it is enforced here rather
/// than left as a convention a one-word manifest edit could end.
fn check_reserved_ceiling(root: &Path, agents: &AgentManifest, diagnostics: &mut Vec<Diagnostic>) {
    let Some(reserved) = Verbosity::ALL.last() else {
        return;
    };
    let path = root.join("manifest/agents.yaml");
    for pack in &agents.packs {
        if pack.maximum_verbosity == *reserved {
            diagnostics.push(Diagnostic::new(
                &path,
                format!(
                    "agent {} declares the reserved {reserved} ceiling; that tier carries \
                     reader-facing material to dist/full-doctrine.md and no pack",
                    pack.id
                ),
            ));
        }
    }
}

/// Rejects an alert outside the closed vocabulary.
///
/// An unrecognized alert renders as an ordinary blockquote on every viewer, so the
/// mistake is invisible in the rendered document and only a scan finds it.
fn check_alert_vocabulary(root: &Path, diagnostics: &mut Vec<Diagnostic>) {
    for path in maintained_markdown(root, diagnostics) {
        let Some(text) = read_text(&path, diagnostics) else {
            continue;
        };
        for (index, line) in text.lines().enumerate() {
            if let Some(name) = unknown_alert(line) {
                diagnostics.push(Diagnostic::new(
                    &path,
                    format!(
                        "line {} uses the alert {name:?}, which is outside the corpus vocabulary",
                        index + 1
                    ),
                ));
            }
        }
    }
}

/// The alert name on a line, when that name is outside the corpus vocabulary.
fn unknown_alert(line: &str) -> Option<&str> {
    let rest = line.trim_start().strip_prefix("> [!")?;
    let name = rest.split(']').next()?;
    if ALERT_VOCABULARY.contains(&name) {
        None
    } else {
        Some(name)
    }
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

    let mut unreadable = Vec::new();
    collect_files(&dist, &mut unreadable, &mut |path| {
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
    diagnostics.append(&mut unreadable);
}

/// Walks `directory`, reporting any directory that exists but cannot be enumerated.
///
/// `unreadable` is separate from the diagnostics a caller's `visit` closure may
/// already hold, so a walk that reports errors can still visit files that report
/// their own. Treating a read failure as an empty directory would let a check pass
/// having observed nothing, which is the silent-omission class these walks exist to
/// avoid.
///
/// An absent directory is genuinely empty and stays silent: the canonical roots are
/// optional here, and a missing doctrine package is already reported against its
/// manifest entry. Only a directory that exists and refuses to be read is a defect
/// this walk can see and no other check would.
fn collect_files(
    directory: &Path,
    unreadable: &mut Vec<Diagnostic>,
    visit: &mut impl FnMut(&Path),
) {
    for (path, file_type) in classified_entries(directory, unreadable) {
        if file_type.is_dir() {
            collect_files(&path, unreadable, visit);
        } else if file_type.is_file() {
            visit(&path);
        }
    }
}

fn collect_repository_files(
    root: &Path,
    directory: &Path,
    unreadable: &mut Vec<Diagnostic>,
    visit: &mut impl FnMut(&Path),
) {
    for (path, file_type) in classified_entries(directory, unreadable) {
        if file_type.is_dir() {
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
            collect_repository_files(root, &path, unreadable, visit);
        } else if file_type.is_file() {
            visit(&path);
        }
    }
}

/// One directory's entries as sorted `(path, file type)`, reporting what it cannot
/// read or classify.
///
/// `Path::is_dir` and `Path::is_file` follow symbolic links and report `false` for a
/// metadata error, so a link is classified as whatever it points at and a
/// classification failure is indistinguishable from "neither". `DirEntry::file_type`
/// describes the entry itself and reports its own failure.
///
/// A symbolic link is reported and not followed. The corpus is built from regular
/// files inside the repository, and a link can name a target outside it, so following
/// one would let content the repository does not contain reach a scan or a bundle.
/// This repository contains no symbolic links, so the policy costs nothing today and
/// states itself rather than being implied by whatever `is_dir` happened to return.
///
/// A per-entry failure is reported rather than dropped, which `filter_map(Result::ok)`
/// did: one unreadable entry vanished from the walk as quietly as an unreadable parent
/// removed all of them.
fn classified_entries(
    directory: &Path,
    unreadable: &mut Vec<Diagnostic>,
) -> Vec<(PathBuf, fs::FileType)> {
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Vec::new(),
        Err(error) => {
            unreadable.push(Diagnostic::new(
                directory,
                format!("cannot enumerate directory: {error}"),
            ));
            return Vec::new();
        }
    };

    let mut classified = Vec::new();
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                unreadable.push(Diagnostic::new(
                    directory,
                    format!("cannot read a directory entry: {error}"),
                ));
                continue;
            }
        };
        let path = entry.path();
        match entry.file_type() {
            Ok(file_type) if file_type.is_symlink() => unreadable.push(Diagnostic::new(
                &path,
                "symbolic link is not followed; the corpus is built from regular files \
                 inside the repository",
            )),
            Ok(file_type) => classified.push((path, file_type)),
            Err(error) => unreadable.push(Diagnostic::new(
                &path,
                format!("cannot classify directory entry: {error}"),
            )),
        }
    }
    classified.sort_by(|left, right| left.0.cmp(&right.0));
    classified
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
        GENERATED_IN_CANONICAL_ROOTS, REACHABILITY_EXEMPTIONS, REACHABILITY_EXTRA_ROOTS,
        RuleInventory, check_path_references, check_reachability, check_reserved_ceiling,
        check_stated_counts, collect_files, doctrine_index_rows, inline_code_spans,
        looks_like_path, maintained_markdown, outbound_links, reachability_scope, resolve_link,
        root_documents, scan_marker_file, table_row_cells, unknown_alert,
        validation_sequence_copies,
    };
    use doctrine_manifest::{AgentRole, DoctrineStatus, Verbosity, states_obligations};
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

    fn doctrine_entry(id: &str, slug: &str, title: &str, status: DoctrineStatus) -> DoctrineEntry {
        DoctrineEntry {
            id: id.to_owned(),
            slug: slug.to_owned(),
            title: title.to_owned(),
            status,
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
                    DoctrineStatus::Active,
                ),
                doctrine_entry(
                    "RUST-DOC-0002",
                    "0002-error-modeling",
                    "Error Modeling as Domain Design",
                    DoctrineStatus::Active,
                ),
            ],
        }
    }

    /// The widest tier carries reader-facing material into `dist/full-doctrine.md` and no
    /// pack. Enforcing that reservation is what lets the corpus gain navigation and
    /// commentary without any generated pack growing.
    #[test]
    fn the_reserved_ceiling_rejects_only_the_widest_tier() {
        let reserved = *Verbosity::ALL.last().expect("a verbosity vocabulary");
        for verbosity in Verbosity::ALL {
            let mut pack = agent_pack(Vec::new());
            pack.maximum_verbosity = verbosity;
            let manifest = AgentManifest {
                schema_version: "1.0".to_owned(),
                packs: vec![pack],
            };
            let mut diagnostics = Vec::new();
            check_reserved_ceiling(Path::new("."), &manifest, &mut diagnostics);
            assert_eq!(
                diagnostics.len(),
                usize::from(verbosity == reserved),
                "{verbosity} was judged wrongly"
            );
        }
    }

    /// End-to-end against the real manifests: no source any role pack lists may be
    /// withheld from it.
    ///
    /// The regression this pins is concrete. An earlier obligation classifier omitted
    /// `agents/`, so an annotation under `## Boundary obligations` in `agents/shared.md`
    /// was accepted by both tools and replaced those rules with a receipt in every role
    /// pack. Because the widest tier is reserved from every pack, the obligation then
    /// reached none of them.
    #[test]
    fn every_source_a_role_pack_lists_states_obligations() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let doctrines: DoctrineManifest = serde_yaml_ng::from_str(
            &fs::read_to_string(root.join("manifest/doctrines.yaml")).expect("read doctrines"),
        )
        .expect("parse doctrines");
        let agents: AgentManifest = serde_yaml_ng::from_str(
            &fs::read_to_string(root.join("manifest/agents.yaml")).expect("read agents"),
        )
        .expect("parse agents");

        let mut tierable = Vec::new();
        for pack in &agents.packs {
            for path in pack.canonical_sources.iter().chain(&pack.review_checklists) {
                if !states_obligations(path, &doctrines, &agents) {
                    tierable.push(format!("{} lists {path}", pack.id));
                }
            }
            for id in &pack.doctrine_selections {
                let Some(entry) = doctrines.doctrines.iter().find(|entry| &entry.id == id) else {
                    continue;
                };
                if !states_obligations(&entry.normative_path, &doctrines, &agents) {
                    tierable.push(format!("{} selects {id}", pack.id));
                }
            }
        }
        assert!(
            tierable.is_empty(),
            "a role pack source that can be withheld: {tierable:?}"
        );
    }

    #[test]
    fn an_alert_outside_the_vocabulary_is_named() {
        assert_eq!(unknown_alert("> [!HINT]"), Some("HINT"));
        assert_eq!(unknown_alert("> [!Note]"), Some("Note"));
        for name in ["NOTE", "TIP", "IMPORTANT", "WARNING", "CAUTION"] {
            assert_eq!(unknown_alert(&format!("> [!{name}]")), None);
        }
        assert_eq!(unknown_alert("> An ordinary blockquote."), None);
        assert_eq!(unknown_alert("Prose mentioning [!HINT] inline."), None);
    }

    /// The annotation scan treats every HTML comment as a candidate, which is only free
    /// while the corpus carries none outside the generated files that declare their own
    /// sources.
    ///
    /// The assertion is containment rather than equality. A generated file is produced by
    /// `bundle-agent-context`, so a working tree that has not been regenerated legitimately
    /// lacks one the constant names, and requiring equality would make this test depend on
    /// whether generation had run.
    #[test]
    fn the_corpus_carries_no_html_comment_outside_the_generated_files() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let permitted: BTreeSet<&str> = GENERATED_IN_CANONICAL_ROOTS.iter().copied().collect();
        let carriers: BTreeSet<String> = maintained_markdown(&root, &mut Vec::new())
            .into_iter()
            .filter(|path| fs::read_to_string(path).is_ok_and(|text| text.contains("<!--")))
            .map(|path| {
                path.strip_prefix(&root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect();
        let unexpected: Vec<&String> = carriers
            .iter()
            .filter(|path| !permitted.contains(path.as_str()))
            .collect();
        assert!(
            unexpected.is_empty(),
            "a new HTML comment in canonical Markdown must be a verbosity annotation: {unexpected:?}"
        );
    }

    fn agent_pack(canonical_sources: Vec<String>) -> AgentPack {
        AgentPack {
            id: AgentRole::Auditor,
            purpose: "Adversarially locate ungoverned authority.".to_owned(),
            maximum_verbosity: Verbosity::Exhaustive,
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

        let found: BTreeSet<String> = root_documents(&root, &mut Vec::new())
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

    /// A directory that exists but cannot be read must be reported rather than walked
    /// as empty. Returning nothing let a check announce a clean result having observed
    /// no files at all, which is the silent-omission class that hid `EVIDENCE.md`.
    #[cfg(unix)]
    #[test]
    fn an_unreadable_directory_is_reported_rather_than_treated_as_empty() {
        use std::os::unix::fs::PermissionsExt;

        let root = temporary_directory("unreadable-directory");
        let locked = root.join("locked");
        fs::create_dir_all(&locked).expect("create directory");
        fs::write(locked.join("hidden.md"), "body\n").expect("write file");
        fs::set_permissions(&locked, fs::Permissions::from_mode(0o000))
            .expect("remove directory permissions");

        // A process with CAP_DAC_OVERRIDE, typically root in a container, ignores the
        // permission bits, so the condition under test cannot be created. Restore and
        // skip rather than assert something the environment made false.
        let enforced = fs::read_dir(&locked).is_err();
        let mut unreadable = Vec::new();
        let mut visited = 0_usize;
        collect_files(&locked, &mut unreadable, &mut |_| visited += 1);

        fs::set_permissions(&locked, fs::Permissions::from_mode(0o755))
            .expect("restore directory permissions");
        fs::remove_dir_all(&root).expect("remove temporary test directory");

        if enforced {
            assert_eq!(visited, 0, "an unreadable directory yields no files");
            assert_eq!(unreadable.len(), 1, "{unreadable:?}");
            assert!(
                unreadable[0].message.contains("cannot enumerate directory"),
                "{unreadable:?}"
            );
        }
    }

    /// A file this repository-wide gate cannot read must be reported. Skipping it let
    /// `doctrine-lint check` exit zero and call the repository valid with a single
    /// unreadable file present, which is the same false success as walking an
    /// unreadable directory as empty.
    #[cfg(unix)]
    #[test]
    fn an_unreadable_file_is_reported_by_the_marker_scan() {
        use std::os::unix::fs::PermissionsExt;

        let root = temporary_directory("unreadable-file");
        let locked = root.join("LOCKED.md");
        fs::write(&locked, "body\n").expect("write file");
        fs::set_permissions(&locked, fs::Permissions::from_mode(0o000))
            .expect("remove file permissions");

        // See the directory case: a process that overrides permission bits cannot
        // construct the condition, so restore and skip rather than assert falsely.
        let enforced = fs::read_to_string(&locked).is_err();
        let mut diagnostics = Vec::new();
        scan_marker_file(&locked, &mut diagnostics);

        fs::set_permissions(&locked, fs::Permissions::from_mode(0o644))
            .expect("restore file permissions");
        fs::remove_dir_all(&root).expect("remove temporary test directory");

        if enforced {
            assert_eq!(diagnostics.len(), 1, "{diagnostics:?}");
            assert!(
                diagnostics[0]
                    .message
                    .contains("cannot read file for the forbidden-marker scan"),
                "{diagnostics:?}"
            );
        }
    }

    /// Content that is not UTF-8 is not a read failure and stays silent, so the check
    /// above cannot be satisfied by reporting every file that fails to decode. The
    /// markers are Markdown filler, so a binary file carries none.
    #[test]
    fn a_non_utf8_file_is_not_reported_by_the_marker_scan() {
        let root = temporary_directory("non-utf8-file");
        let binary = root.join("icon.bin");
        fs::write(&binary, [0xff_u8, 0xfe, 0x00, 0x01]).expect("write file");

        let mut diagnostics = Vec::new();
        scan_marker_file(&binary, &mut diagnostics);

        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        fs::remove_dir_all(root).expect("remove temporary test directory");
    }

    /// A symbolic link is reported and not followed, while regular entries beside it
    /// are still visited. `Path::is_file` classifies a link as its target, so a linked
    /// document would be scanned as though the repository contained it.
    #[cfg(unix)]
    #[test]
    fn a_symbolic_link_is_reported_and_not_followed() {
        let root = temporary_directory("symlink-entry");
        let outside = root.join("outside.md");
        fs::write(&outside, "outside the walk\n").expect("write link target");
        let directory = root.join("walk");
        fs::create_dir_all(&directory).expect("create directory");
        fs::write(directory.join("real.md"), "body\n").expect("write regular file");
        std::os::unix::fs::symlink(&outside, directory.join("linked.md"))
            .expect("create symbolic link");

        let mut unreadable = Vec::new();
        let mut visited = Vec::new();
        collect_files(&directory, &mut unreadable, &mut |path| {
            visited.push(
                path.file_name()
                    .expect("file name")
                    .to_string_lossy()
                    .into_owned(),
            );
        });

        fs::remove_dir_all(&root).expect("remove temporary test directory");
        assert_eq!(visited, vec!["real.md".to_owned()]);
        assert_eq!(unreadable.len(), 1, "{unreadable:?}");
        assert!(
            unreadable[0]
                .message
                .contains("symbolic link is not followed"),
            "{unreadable:?}"
        );
    }

    /// An absent directory is genuinely empty and stays silent, so the check above
    /// cannot be satisfied by reporting every path that fails to open.
    #[test]
    fn an_absent_directory_is_not_reported() {
        let root = temporary_directory("absent-directory");
        let mut unreadable = Vec::new();
        collect_files(&root.join("nowhere"), &mut unreadable, &mut |_| {});

        assert!(unreadable.is_empty(), "{unreadable:?}");
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
            DoctrineStatus::Deprecated,
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

    #[test]
    fn outbound_links_ignore_fenced_paths() {
        let text = "See [one](a.md) and [two](../b.md#frag).\n\
                    ```text\n\
                    [three](c.md)\n\
                    ```\n\
                    Then [four](https://example.com/d).\n";
        assert_eq!(
            outbound_links(text),
            vec![
                "a.md".to_owned(),
                "../b.md#frag".to_owned(),
                "https://example.com/d".to_owned(),
            ]
        );
    }

    #[test]
    fn resolve_link_normalizes_and_rejects_external() {
        let root = Path::new("/repo");
        let from = Path::new("/repo/doctrines/0001-invalid-states/README.md");
        assert_eq!(
            resolve_link(root, from, "doctrine.md").as_deref(),
            Some("doctrines/0001-invalid-states/doctrine.md")
        );
        assert_eq!(
            resolve_link(root, from, "../../foundations/evidence.md#anchor").as_deref(),
            Some("foundations/evidence.md")
        );
        assert_eq!(resolve_link(root, from, "https://example.com"), None);
        assert_eq!(resolve_link(root, from, "mailto:someone@example.com"), None);
        assert_eq!(resolve_link(root, from, "#section-only"), None);
    }

    /// The reachability check ships with a corpus that satisfies it, so the failing
    /// direction is exercised here rather than left unproved.
    #[test]
    fn reachability_reports_a_file_nothing_links_to() {
        let root = temporary_directory("reachability");
        fs::create_dir_all(root.join("sources")).unwrap();
        fs::write(root.join("README.md"), "# Root\n\n[index](sources/)\n").unwrap();
        fs::write(
            root.join("sources/README.md"),
            "# Sources\n\n[linked](linked.md)\n",
        )
        .unwrap();
        fs::write(root.join("sources/linked.md"), "# Linked\n").unwrap();
        fs::write(root.join("sources/orphan.md"), "# Orphan\n").unwrap();

        let mut diagnostics = Vec::new();
        check_reachability(&root, &mut diagnostics);

        let reported: Vec<String> = diagnostics
            .iter()
            .map(|diagnostic| diagnostic.path.to_string_lossy().replace('\\', "/"))
            .collect();
        assert!(
            reported
                .iter()
                .any(|path| path.ends_with("sources/orphan.md")),
            "orphan.md was not reported; reported {reported:?}"
        );
        assert!(
            !reported
                .iter()
                .any(|path| path.ends_with("sources/linked.md")),
            "linked.md was reported despite an inbound link"
        );
        // The directory link in README.md reaches sources/README.md.
        assert!(
            !reported
                .iter()
                .any(|path| path.ends_with("sources/README.md")),
            "a directory link did not credit that directory's index"
        );
        // The root document is exempt, so its absence of inbound links is not a finding.
        assert!(
            !reported
                .iter()
                .any(|path| path.ends_with("/README.md") && !path.contains("sources")),
            "the repository entry point should be exempt"
        );

        let _ = fs::remove_dir_all(&root);
    }

    /// A check is only as wide as its file list. This one silently covered nothing
    /// under `templates/`, `manifest/`, `tools/`, or `examples/` until the scope was
    /// widened, and a passing repository looks identical either way — so the scope is
    /// asserted against the real tree rather than inferred from the constant.
    #[test]
    fn reachability_scope_covers_every_extra_root() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let mut diagnostics = Vec::new();
        let scope = reachability_scope(&root, &mut diagnostics);
        let relative: BTreeSet<String> = scope
            .iter()
            .map(|path| {
                path.strip_prefix(&root)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect();

        for directory in REACHABILITY_EXTRA_ROOTS {
            let prefix = format!("{directory}/");
            assert!(
                relative.iter().any(|path| path.starts_with(&prefix)),
                "the reachability scope covers no Markdown under {directory}/, so nothing \
                 there can ever be reported"
            );
        }
        // The canonical roots must survive the widening.
        assert!(relative.contains("doctrines/README.md"));
        assert!(relative.contains("README.md"));
    }

    #[test]
    fn reachability_scope_lists_each_file_once() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let mut diagnostics = Vec::new();
        let scope = reachability_scope(&root, &mut diagnostics);
        let mut unique = scope.clone();
        unique.dedup();
        assert_eq!(
            scope.len(),
            unique.len(),
            "a duplicated path would report the same file twice"
        );
    }

    /// A callout marker is not a link. Searching for the next `](` anywhere ahead made
    /// `[!NOTE]` swallow the text up to the next real link, which emptied the scanned
    /// prose and made the inert-path check pass on documents it never examined.
    #[test]
    fn a_callout_marker_does_not_swallow_the_text_after_it() {
        let text = "> [!NOTE]\n> Keep `foundations/invariants.md` visible.\n\nSee [x](y.md).\n";
        let spans = inline_code_spans(text);
        assert!(
            spans.iter().any(|s| s == "foundations/invariants.md"),
            "span was swallowed; got {spans:?}"
        );
    }

    #[test]
    fn inline_code_spans_skip_fences_front_matter_and_link_text() {
        let text = "---\ntitle: `front/matter.md`\n---\n\nProse `a/one.md` here.\n\n\
                    ```text\n`b/two.md`\n```\n\nAnd [`c/three.md`](c/three.md).\n";
        let spans = inline_code_spans(text);
        assert_eq!(spans, vec!["a/one.md".to_owned()]);
    }

    #[test]
    fn path_shaped_spans_exclude_identifiers() {
        assert!(looks_like_path("foundations/invariants.md"));
        assert!(looks_like_path("Cargo.toml"));
        assert!(looks_like_path("dist/"));
        assert!(!looks_like_path("unwrap"));
        assert!(!looks_like_path("NonZeroU64"));
        assert!(!looks_like_path("cargo run -p doctrine-lint"));
        assert!(!looks_like_path("https://example.com/a.md"));
    }

    /// Mentions are written both ways in this corpus. Resolving only against the file's
    /// own directory skipped every root-relative one, which is most of them, and left
    /// the check reporting nothing.
    #[test]
    fn an_inert_root_relative_mention_is_reported() {
        let root = temporary_directory("path-references");
        fs::create_dir_all(root.join("reviews")).unwrap();
        fs::create_dir_all(root.join("manifest")).unwrap();
        fs::write(
            root.join("manifest/doctrines.yaml"),
            "schema_version: \"1.0\"\n",
        )
        .unwrap();
        fs::write(root.join("README.md"), "# Root\n\n[reviews](reviews/)\n").unwrap();
        fs::write(
            root.join("reviews/README.md"),
            "# Reviews\n\nSee `manifest/doctrines.yaml` for discovery.\n",
        )
        .unwrap();

        let mut diagnostics = Vec::new();
        check_path_references(&root, &mut diagnostics);
        assert_eq!(diagnostics.len(), 1, "got {diagnostics:?}");
        assert!(diagnostics[0].message.contains("manifest/doctrines.yaml"));

        // Linking it anywhere in the file clears the finding.
        fs::write(
            root.join("reviews/README.md"),
            "# Reviews\n\nSee [`manifest/doctrines.yaml`](../manifest/doctrines.yaml).\n",
        )
        .unwrap();
        let mut cleared = Vec::new();
        check_path_references(&root, &mut cleared);
        assert!(cleared.is_empty(), "got {cleared:?}");

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn reachability_exemptions_name_a_reason() {
        assert!(!REACHABILITY_EXEMPTIONS.is_empty());
        for (path, reason) in REACHABILITY_EXEMPTIONS {
            assert!(!path.is_empty(), "an exemption has no path");
            assert!(
                reason.len() > 20,
                "exemption {path} has no stated reason worth reading"
            );
        }
    }
}
