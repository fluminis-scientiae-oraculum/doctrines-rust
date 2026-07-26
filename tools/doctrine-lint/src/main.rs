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
    "rfcs",
    "sources",
];

const ROOT_DOCUMENTS: &[&str] = &[
    "README.md",
    "AGENTS.md",
    "CONTRIBUTING.md",
    "CHANGELOG.md",
    "CODE_OF_CONDUCT.md",
    "SECURITY.md",
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
    check_agents(root, &agent_manifest, &doctrine_manifest, &mut diagnostics);
    check_forbidden_markers(root, &mut diagnostics);
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
    if manifest.repository_version != "0.1.0" {
        diagnostics.push(Diagnostic::new(
            root.join("manifest/doctrines.yaml"),
            "repository_version must match the initial 0.1.0 release",
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

    let normative_path = root.join(&entry.normative_path);
    let Some(normative) = read_text(&normative_path, diagnostics) else {
        return;
    };
    let rule_ids = extract_rule_ids(&normative);
    if rule_ids.is_empty() {
        diagnostics.push(Diagnostic::new(
            &normative_path,
            format!("{} has no normative rule IDs", entry.id),
        ));
    }
    let expected_prefix = format!("{}-R", entry.id);
    for rule_id in rule_ids {
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
    }
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
        if pack.purpose.trim().len() < 20 {
            diagnostics.push(Diagnostic::new(
                &manifest_path,
                format!("agent {} purpose is too short", pack.id),
            ));
        }
        if !matches!(
            pack.maximum_verbosity.as_str(),
            "focused" | "operational" | "detailed" | "exhaustive"
        ) {
            diagnostics.push(Diagnostic::new(
                &manifest_path,
                format!(
                    "agent {} has invalid maximum_verbosity {}",
                    pack.id, pack.maximum_verbosity
                ),
            ));
        }
        if !orderings.insert(pack.ordering) {
            diagnostics.push(Diagnostic::new(
                &manifest_path,
                format!("agent ordering {} is duplicated", pack.ordering),
            ));
        }
        if !outputs.insert(pack.output_path.as_str()) {
            diagnostics.push(Diagnostic::new(
                &manifest_path,
                format!("agent output {} is duplicated", pack.output_path),
            ));
        }
        if !pack.output_path.starts_with("dist/agents/") {
            diagnostics.push(Diagnostic::new(
                &manifest_path,
                format!(
                    "agent output {} must be under dist/agents",
                    pack.output_path
                ),
            ));
        }

        for path in pack.canonical_sources.iter().chain(&pack.review_checklists) {
            if !root.join(path).is_file() {
                diagnostics.push(Diagnostic::new(
                    root.join(path),
                    format!("agent {} references a missing canonical file", pack.id),
                ));
            }
        }
        for doctrine in &pack.doctrine_selections {
            if !doctrine_ids.contains(doctrine.as_str()) {
                diagnostics.push(Diagnostic::new(
                    &manifest_path,
                    format!("agent {} selects unknown doctrine {doctrine}", pack.id),
                ));
            }
        }
        if !root.join(&pack.output_path).is_file() {
            diagnostics.push(Diagnostic::new(
                root.join(&pack.output_path),
                format!("generated output for agent {} is missing", pack.id),
            ));
        }
    }
}

fn check_forbidden_markers(root: &Path, diagnostics: &mut Vec<Diagnostic>) {
    for path in ROOT_DOCUMENTS {
        scan_marker_file(&root.join(path), diagnostics);
    }
    for directory in CANONICAL_ROOTS {
        collect_files(&root.join(directory), &mut |path| {
            if path.extension().and_then(|extension| extension.to_str()) == Some("md") {
                scan_marker_file(path, diagnostics);
            }
        });
    }
}

fn scan_marker_file(path: &Path, diagnostics: &mut Vec<Diagnostic>) {
    let Some(text) = read_text(path, diagnostics) else {
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

fn extract_rule_ids(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| line.strip_prefix("## "))
        .filter_map(|heading| heading.split_whitespace().next())
        .filter(|token| token.starts_with("RUST-DOC-"))
        .map(str::to_owned)
        .collect()
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
    use super::{extract_rule_ids, front_matter, valid_rule_id};

    #[test]
    fn extracts_only_doctrine_rule_headings() {
        let text = "# Title\n\n## RUST-DOC-0001-R001 — Rule\n\n## Rationale\n";
        assert_eq!(extract_rule_ids(text), ["RUST-DOC-0001-R001"]);
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
}
