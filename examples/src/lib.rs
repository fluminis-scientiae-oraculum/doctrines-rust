//! Inventory crate for the executable examples in this workspace.

/// Example package names in their intended learning order.
pub const EXAMPLE_PACKAGES: &[&str] = &[
    "domain-modeling",
    "validated-newtypes",
    "typestate",
    "boundary-validation",
    "distributed-outcomes",
    "doctrine-compile-fail",
];

#[cfg(test)]
mod tests {
    use super::EXAMPLE_PACKAGES;
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::Path;

    #[test]
    fn inventory_matches_example_workspace_members() {
        let examples = Path::new(env!("CARGO_MANIFEST_DIR"));
        let workspace_manifest =
            fs::read_to_string(examples.join("../Cargo.toml")).expect("read workspace manifest");
        let mut discovered = BTreeSet::new();

        for entry in fs::read_dir(examples).expect("read examples directory") {
            let path = entry.expect("read examples entry").path();
            let package_manifest = path.join("Cargo.toml");
            if !package_manifest.is_file() {
                continue;
            }

            let manifest = fs::read_to_string(&package_manifest).expect("read package manifest");
            let package = manifest
                .lines()
                .find_map(|line| {
                    line.trim()
                        .strip_prefix("name = \"")
                        .and_then(|name| name.strip_suffix('"'))
                })
                .expect("example package has a name");
            assert!(
                discovered.insert(package.to_owned()),
                "duplicate example package name {package}"
            );

            let directory = path.file_name().expect("example directory has a name");
            let member = format!("\"examples/{}\"", directory.to_string_lossy());
            assert!(
                workspace_manifest.contains(&member),
                "{member} is absent from workspace members"
            );
        }

        let expected: BTreeSet<String> = EXAMPLE_PACKAGES
            .iter()
            .map(|package| (*package).to_owned())
            .collect();
        assert_eq!(expected.len(), EXAMPLE_PACKAGES.len());
        assert_eq!(expected, discovered);
    }
}
