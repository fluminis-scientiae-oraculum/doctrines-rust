# RUST-DOC-0002 attribution

Primary Rust and crate documentation is linked rather than reproduced. `thiserror`
and `anyhow` are cited as implementation choices, not endorsed as mandatory
dependencies. Their names and APIs remain the property of their respective
projects.

The repository's error categories, rule severities, retry/reconciliation
requirements, and public-compatibility review are original governance synthesis.
No external error message catalogue or copyrighted text is copied.

This package is not an exhaustive survey of Rust error libraries, API evolution,
internationalization, or incident reporting. Maintainers should recheck crate
versions and MSRV before changing executable dependencies.
