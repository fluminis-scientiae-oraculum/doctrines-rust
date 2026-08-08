//! Inventory crate for the executable examples in this workspace.

/// Example package names in their intended learning order.
pub const EXAMPLE_PACKAGES: &[&str] = &[
    "domain-modeling",
    "validated-newtypes",
    "typestate",
    "staged-protocol",
    "boundary-validation",
    "distributed-outcomes",
    "unsafe-evidence",
    "doctrine-compile-fail",
];

/// Executable evidence for `RUST-DOC-0008-R022`.
///
/// An assertion expecting absence reports "searched and found none" and "the search
/// matched nothing" identically. These tests hold a registry that *violates* the
/// invariant and show the vacuous form passing on it, then show the two forms the rule
/// accepts failing on the same data.
#[cfg(test)]
mod evidence_of_absence {
    /// A registry that violates the invariant under test: a secret-bearing column is
    /// present. Every assertion below runs against this same data.
    const COLUMNS: &[&str] = &["invoice_total", "session_token", "customer_email"];

    fn columns_containing(needle: &str) -> Vec<&'static str> {
        COLUMNS
            .iter()
            .copied()
            .filter(|column| column.contains(needle))
            .collect()
    }

    /// The defect the rule exists to prevent: the predicate is misspelled, so it selects
    /// nothing and the absence assertion passes although the violation is present.
    #[test]
    fn a_zero_count_passes_when_the_predicate_matched_nothing() {
        let offending = columns_containing("scession");
        assert!(
            offending.is_empty(),
            "this assertion passes for the wrong reason, which is the point"
        );
    }

    /// Accepted form one: a positive control asserted before the absence claim. The
    /// control separates the two predicates that the absence claim alone cannot tell
    /// apart — in real use it is the control that fails, and the vacuous absence claim
    /// beside it is never reached.
    #[test]
    fn a_positive_control_separates_a_blind_predicate_from_a_seeing_one() {
        assert!(
            columns_containing("scession").is_empty(),
            "the misspelled predicate observes nothing, so any absence claim it makes is empty"
        );
        assert_eq!(
            columns_containing("session"),
            vec!["session_token"],
            "the correct predicate observes the subject, which is what the control asserts"
        );
    }

    /// Accepted form two: a paired assertion whose expected count is non-zero. The pair
    /// fails on the same data the vacuous form accepted.
    #[test]
    fn a_non_zero_pair_detects_the_violation() {
        let secrets = columns_containing("token");
        assert_eq!(
            secrets.len(),
            1,
            "the paired non-zero case proves the counter counts"
        );
        assert_eq!(
            secrets,
            vec!["session_token"],
            "and names the violation the vacuous form missed"
        );
    }
}

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
