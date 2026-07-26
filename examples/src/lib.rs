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

    #[test]
    fn inventory_has_unique_adjacent_entries() {
        assert!(EXAMPLE_PACKAGES.windows(2).all(|pair| pair[0] != pair[1]));
        assert_eq!(EXAMPLE_PACKAGES.len(), 6);
    }
}
