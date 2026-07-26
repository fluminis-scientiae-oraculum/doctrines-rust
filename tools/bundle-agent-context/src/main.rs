use serde::Deserialize;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

const GENERATED_BANNER: &str = "<!--\nGENERATED FILE. DO NOT EDIT DIRECTLY.\nCanonical sources \
live under /foundations, /doctrines, /patterns,\n /boundaries, /reviews, and /agents.\n-->\n";

const DOCTRINE_PACKAGE_FILES: &[&str] = &[
    "README.md",
    "doctrine.md",
    "rationale.md",
    "decision-framework.md",
    "review-standard.md",
    "anti-patterns.md",
    "glossary.md",
    "references.md",
];

const FOUNDATION_FILES: &[&str] = &[
    "foundations/README.md",
    "foundations/normative-language.md",
    "foundations/invariants.md",
    "foundations/evidence.md",
    "foundations/trust-boundaries.md",
    "foundations/guarantee-honesty.md",
    "foundations/complexity-budget.md",
];

const PATTERN_FILES: &[&str] = &[
    "patterns/README.md",
    "patterns/sum-types.md",
    "patterns/opaque-newtypes.md",
    "patterns/smart-constructors.md",
    "patterns/typestate.md",
    "patterns/capability-types.md",
    "patterns/consuming-transitions.md",
    "patterns/validated-collections.md",
    "patterns/hybrid-state-machines.md",
    "patterns/explicit-uncertainty.md",
];

const BOUNDARY_FILES: &[&str] = &[
    "boundaries/README.md",
    "boundaries/serde.md",
    "boundaries/database-decoding.md",
    "boundaries/http-and-rpc.md",
    "boundaries/messaging.md",
    "boundaries/configuration.md",
    "boundaries/filesystem.md",
    "boundaries/ffi.md",
];

const REVIEW_FILES: &[&str] = &[
    "reviews/README.md",
    "reviews/pre-implementation.md",
    "reviews/domain-model-review.md",
    "reviews/boundary-review.md",
    "reviews/typestate-review.md",
    "reviews/distributed-effects-review.md",
    "reviews/final-correctness-audit.md",
];

#[derive(Debug, Deserialize)]
struct DoctrineManifest {
    doctrines: Vec<DoctrineEntry>,
}

#[derive(Debug, Deserialize)]
struct DoctrineEntry {
    id: String,
    status: String,
    package_path: String,
    normative_path: String,
}

#[derive(Debug, Deserialize)]
struct AgentManifest {
    packs: Vec<AgentPack>,
}

#[derive(Debug, Deserialize)]
struct AgentPack {
    id: String,
    purpose: String,
    ordering: u16,
    canonical_sources: Vec<String>,
    doctrine_selections: Vec<String>,
    review_checklists: Vec<String>,
    output_path: String,
}

#[derive(Debug, Eq, PartialEq)]
struct GeneratedFile {
    path: PathBuf,
    content: String,
}

fn main() {
    let mut arguments = env::args().skip(1);
    let command = arguments.next();
    if !matches!(command.as_deref(), Some("generate" | "check")) || arguments.next().is_some() {
        eprintln!("usage: bundle-agent-context <generate|check>");
        process::exit(2);
    }

    let root = match env::current_dir() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("bundle-agent-context: cannot read current directory: {error}");
            process::exit(2);
        }
    };
    let outputs = match build_outputs(&root) {
        Ok(outputs) => outputs,
        Err(error) => {
            eprintln!("bundle-agent-context: {error}");
            process::exit(1);
        }
    };

    let result = match command.as_deref() {
        Some("generate") => generate(&root, &outputs),
        Some("check") => check(&root, &outputs),
        _ => unreachable!("command validated above"),
    };
    if let Err(error) = result {
        eprintln!("bundle-agent-context: {error}");
        process::exit(1);
    }
}

fn build_outputs(root: &Path) -> Result<Vec<GeneratedFile>, String> {
    let doctrine_manifest: DoctrineManifest = read_yaml(root, "manifest/doctrines.yaml")?;
    let mut agent_manifest: AgentManifest = read_yaml(root, "manifest/agents.yaml")?;
    agent_manifest.packs.sort_by_key(|pack| pack.ordering);

    let mut outputs = Vec::new();

    let mut distribution = generated_document("Generated doctrine distributions");
    append_source(root, "agents/distribution.md", &mut distribution)?;
    outputs.push(GeneratedFile {
        path: PathBuf::from("dist/README.md"),
        content: distribution,
    });

    outputs.push(GeneratedFile {
        path: PathBuf::from("dist/full-doctrine.md"),
        content: build_full(root, &doctrine_manifest)?,
    });
    outputs.push(GeneratedFile {
        path: PathBuf::from("dist/compact-doctrine.md"),
        content: build_compact(root, &doctrine_manifest)?,
    });

    for pack in &agent_manifest.packs {
        outputs.push(GeneratedFile {
            path: PathBuf::from(&pack.output_path),
            content: build_role_pack(root, pack, &doctrine_manifest)?,
        });
    }

    outputs.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(outputs)
}

fn build_full(root: &Path, doctrines: &DoctrineManifest) -> Result<String, String> {
    let mut output = generated_document("Full Rust doctrine corpus");
    append_source(root, "README.md", &mut output)?;
    append_sources(root, FOUNDATION_FILES, &mut output)?;

    for doctrine in doctrines
        .doctrines
        .iter()
        .filter(|entry| entry.status == "active")
    {
        for file in DOCTRINE_PACKAGE_FILES {
            let path = format!("{}/{}", doctrine.package_path, file);
            append_source(root, &path, &mut output)?;
        }
    }

    append_sources(root, PATTERN_FILES, &mut output)?;
    append_sources(root, BOUNDARY_FILES, &mut output)?;
    append_sources(root, REVIEW_FILES, &mut output)?;
    append_source(root, "agents/shared.md", &mut output)?;
    Ok(normalize_final_newline(output))
}

fn build_compact(root: &Path, doctrines: &DoctrineManifest) -> Result<String, String> {
    let mut output = generated_document("Compact Rust doctrine hydration");
    append_source(root, "agents/compact-core.md", &mut output)?;
    append_source(root, "foundations/README.md", &mut output)?;
    append_source(root, "foundations/guarantee-honesty.md", &mut output)?;
    append_source(
        root,
        "doctrines/0001-invalid-states/decision-framework.md",
        &mut output,
    )?;

    for doctrine in doctrines
        .doctrines
        .iter()
        .filter(|entry| entry.status == "active")
    {
        append_source(root, &doctrine.normative_path, &mut output)?;
    }

    append_source(root, "patterns/README.md", &mut output)?;
    append_source(root, "reviews/final-correctness-audit.md", &mut output)?;
    append_source(root, "agents/shared.md", &mut output)?;
    Ok(normalize_final_newline(output))
}

fn build_role_pack(
    root: &Path,
    pack: &AgentPack,
    doctrines: &DoctrineManifest,
) -> Result<String, String> {
    let mut output = generated_document(&format!("{} agent doctrine pack", title_case(&pack.id)));
    output.push('\n');
    output.push_str(&pack.purpose);
    output.push('\n');

    let mut included = BTreeSet::new();
    for path in &pack.canonical_sources {
        append_unique_source(root, path, &mut included, &mut output)?;
    }
    for doctrine_id in &pack.doctrine_selections {
        let doctrine = doctrines
            .doctrines
            .iter()
            .find(|entry| entry.id == *doctrine_id)
            .ok_or_else(|| format!("agent {} selects unknown doctrine {doctrine_id}", pack.id))?;
        append_unique_source(root, &doctrine.normative_path, &mut included, &mut output)?;
    }
    for path in &pack.review_checklists {
        append_unique_source(root, path, &mut included, &mut output)?;
    }
    Ok(normalize_final_newline(output))
}

fn append_sources(root: &Path, paths: &[&str], output: &mut String) -> Result<(), String> {
    for path in paths {
        append_source(root, path, output)?;
    }
    Ok(())
}

fn append_unique_source(
    root: &Path,
    path: &str,
    included: &mut BTreeSet<String>,
    output: &mut String,
) -> Result<(), String> {
    if included.insert(path.to_owned()) {
        append_source(root, path, output)?;
    }
    Ok(())
}

fn append_source(root: &Path, relative: &str, output: &mut String) -> Result<(), String> {
    let path = root.join(relative);
    let content =
        fs::read_to_string(&path).map_err(|error| format!("cannot read {relative}: {error}"))?;
    output.push_str("\n---\n\n## Source: `");
    output.push_str(relative);
    output.push_str("`\n\n");
    output.push_str(content.trim_end());
    output.push('\n');
    Ok(())
}

fn generated_document(title: &str) -> String {
    let mut output = String::from(GENERATED_BANNER);
    output.push('\n');
    output.push_str("# ");
    output.push_str(title);
    output.push('\n');
    output
}

fn normalize_final_newline(mut text: String) -> String {
    let length = text.trim_end().len();
    text.truncate(length);
    text.push('\n');
    text
}

fn read_yaml<T: for<'de> Deserialize<'de>>(root: &Path, relative: &str) -> Result<T, String> {
    let text = fs::read_to_string(root.join(relative))
        .map_err(|error| format!("cannot read {relative}: {error}"))?;
    serde_yaml_ng::from_str(&text).map_err(|error| format!("cannot parse {relative}: {error}"))
}

fn title_case(value: &str) -> String {
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
        return String::new();
    };
    first.to_uppercase().chain(characters).collect()
}

fn generate(root: &Path, outputs: &[GeneratedFile]) -> Result<(), String> {
    for output in outputs {
        let path = root.join(&output.path);
        let parent = path
            .parent()
            .ok_or_else(|| format!("output {} has no parent", output.path.display()))?;
        fs::create_dir_all(parent)
            .map_err(|error| format!("cannot create {}: {error}", parent.display()))?;
        let current = fs::read_to_string(&path).ok();
        if current.as_deref() != Some(output.content.as_str()) {
            fs::write(&path, &output.content)
                .map_err(|error| format!("cannot write {}: {error}", output.path.display()))?;
        }
    }
    println!("bundle-agent-context: generated {} file(s)", outputs.len());
    Ok(())
}

fn check(root: &Path, outputs: &[GeneratedFile]) -> Result<(), String> {
    let mut drift = Vec::new();
    let expected: BTreeSet<PathBuf> = outputs.iter().map(|output| output.path.clone()).collect();

    for output in outputs {
        let path = root.join(&output.path);
        match fs::read_to_string(&path) {
            Ok(current) if current == output.content => {}
            Ok(_) => drift.push(format!("changed: {}", output.path.display())),
            Err(error) => drift.push(format!("missing: {} ({error})", output.path.display())),
        }
    }

    for path in markdown_files(&root.join("dist")) {
        let relative = path
            .strip_prefix(root)
            .map_err(|error| format!("cannot relativize {}: {error}", path.display()))?
            .to_path_buf();
        if !expected.contains(&relative) {
            drift.push(format!("unexpected: {}", relative.display()));
        }
    }

    if drift.is_empty() {
        println!("bundle-agent-context: generated files are current");
        Ok(())
    } else {
        drift.sort();
        Err(format!("generated drift detected:\n{}", drift.join("\n")))
    }
}

fn markdown_files(directory: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_markdown(directory, &mut files);
    files.sort();
    files
}

fn collect_markdown(directory: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            collect_markdown(&path, files);
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("md") {
            files.push(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GENERATED_BANNER, append_source, generated_document, normalize_final_newline, title_case,
    };
    use std::fs;

    #[test]
    fn generated_document_starts_with_required_banner() {
        let document = generated_document("Example");
        assert!(document.starts_with(GENERATED_BANNER));
        assert!(document.contains("# Example\n"));
    }

    #[test]
    fn source_append_has_provenance_and_normalized_tail() {
        let temporary = std::env::temp_dir().join(format!(
            "doctrines-rust-bundler-test-{}",
            std::process::id()
        ));
        fs::create_dir_all(&temporary).expect("create temporary test directory");
        fs::write(temporary.join("source.md"), "# Source\n\nBody\n\n").expect("write test source");

        let mut output = String::new();
        append_source(&temporary, "source.md", &mut output).expect("append source");
        let output = normalize_final_newline(output);

        assert!(output.contains("## Source: `source.md`"));
        assert!(output.ends_with("Body\n"));
        fs::remove_dir_all(temporary).expect("remove temporary test directory");
    }

    #[test]
    fn role_title_is_stable() {
        assert_eq!(title_case("planner"), "Planner");
        assert_eq!(title_case(""), "");
    }
}
