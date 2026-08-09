# `doctrine-lint`

The gate. One subcommand, one exit code, path-specific diagnostics:

```text
cargo run -p doctrine-lint -- check
```

It reads the repository from its root, calls no external process, touches no
network, and writes nothing. Every finding names the file it is about.

## What it checks

**Manifest integrity.** Both YAML manifests under
[`manifest/`](../../manifest/README.md) validate against their Draft 2020-12 JSON
Schemas; doctrine identifiers are unique; a manifest path cannot escape the
repository; and `repository_version` equals the workspace package version in
[`Cargo.toml`](../../Cargo.toml).

**Package anatomy.** Each doctrine package holds all eight required files with
real content, its folder name agrees with its identifier and slug, its front
matter agrees with the manifest, its rule headings are level two and unique, and
every rule is cited by the package's review standard.

**Claims that restate a derivable fact.** A counted claim in prose — the number
of normative rules, doctrine packages, or active doctrines — is recomputed from
the corpus and compared. The doctrine index and
[`EVIDENCE.md`](../../EVIDENCE.md) are held to one row per active doctrine. The
local validation sequence has to appear in exactly one place.

**Language boundaries.** Uppercase normative terms appear only where obligations
are stated. Filler markers appear nowhere. Callouts are limited to a closed
vocabulary. Every HTML comment in maintained Markdown is a well-formed verbosity
annotation, and no pack declares the widest tier.

**Connectivity.** Five checks, described below.

**Enforcement and evidence.** Every rule names an enforcing artifact that exists,
or states why it is unenforceable. Every review gate declares whether it is
judgment or a named mechanical command. Every generated file under
[`dist/`](../../dist/README.md) carries its banner.

## The connectivity gates

The repository is a graph a reader navigates by clicking. These checks hold the
invariant that every node in it is reachable, or is registered as deliberately
not.

| Check                            | Obligation                                                                                |
| -------------------------------- | ----------------------------------------------------------------------------------------- |
| `check_reachability`             | every maintained Markdown file has at least one inbound link                              |
| `check_path_references`          | every backticked path that resolves on disk is also linked, once per document             |
| `check_workspace_crate_coverage` | every Cargo workspace member is linked, as a directory or a file inside it                |
| `check_directory_indexes`        | every workspace crate, and every directory holding maintained Markdown, has a `README.md` |
| `check_referenced_files`         | every remaining file is named by some maintained Markdown                                 |

A sixth, `check_example_workflow_packages`, is connectivity of a different kind:
the `-p` package list in
[`.github/workflows/rust-examples.yml`](../../.github/workflows/rust-examples.yml)
has to equal the example half of the workspace, in both directions. That list is
a hand-maintained second copy of `[workspace] members`, and an example crate
missing from it is never tested by a run that still reports success.

`check_reachability` asks about inbound links, not reachability from the root: a
file linked only by an unreachable file passes. That is the weaker claim, and it
is the one made rather than the one the name suggests.

`check_referenced_files` accepts a bare mention where the others demand a link.
Requiring a Markdown link to every crate manifest would add thirteen links no
reader wants; requiring the name to appear at all catches the class that matters,
which is a file the corpus has never heard of. The mention has to name the whole
file: a plain substring search credited `ci.yml` because some document mentioned
`doctrine-ci.yml`. The cost that remains is that two files with the _same_
basename cover each other, which is why this check does not replace
`check_path_references`.

Its walk is over the repository, not over whatever is on disk. It reads
[`.gitignore`](../../.gitignore) — it does not call Git — and honours a
deliberate subset of that syntax: a bare name, a suffix wildcard, and a
directory written with a leading slash, a leading double-star, or a trailing
slash, which is every pattern the file uses. A pattern it cannot honour, such as
a negation, is reported rather than silently misread, because a rule this walk
ignores must not look obeyed. The skip test matches a directory name at any
depth, since the ignore file names build directories without anchoring them to
the root and means them everywhere.

## The registers

Four constants record what is deliberately exempt, and why. The first three are
`&[(&str, &str)]` of path and reason, matched by exact string equality so a file
cannot acquire an exemption by resembling one; a test asserts that every reason
is long enough to be a reason rather than a word, and that every path named still
exists. `UNSCANNED_TOP_LEVEL` is a bare list, because its four entries are
explained once by the paragraph below rather than one at a time.

| Register                       | Exempts                                                 |
| ------------------------------ | ------------------------------------------------------- |
| `REACHABILITY_EXEMPTIONS`      | documents a reader opens directly, and generator inputs |
| `INDEXLESS_DIRECTORIES`        | directories that legitimately carry no index            |
| `UNREFERENCED_FILE_EXEMPTIONS` | files no document names, correctly                      |
| `UNSCANNED_TOP_LEVEL`          | directories no connectivity gate walks, at any depth    |

This table deliberately carries no entry counts. A count here would be a
hand-maintained integer restating a fact the source already holds — the drift
this repository has a check for, and the defect this release exists to remove.
Read the constants.

`UNSCANNED_TOP_LEVEL` is the boundary of what these gates can see, written down
rather than left as an accident of whichever walker skipped what. `dist/` is on
it because the bundler's own drift check already holds that tree to exact set
equality, over every extension.

An empty register is the healthy state, and `INDEXLESS_DIRECTORIES` is empty.
Build layout — `src`, `tests`, `ui` — needs no entry, because it holds no
Markdown and is not a workspace member, so neither index obligation reaches it.
An exemption a routine change has to feed is an exemption nobody reads.

## Reading a failure

Every diagnostic names the file and the register that would silence it. Silencing
is the second-best answer. Before adding an entry, check whether the finding is
the gate working: the pull-request template's inert manifest references and the
missing index for this very crate were both found by these checks on the change
that introduced them.

## Evidence

Each check is exercised in both directions — a tree that passes, and the same tree
with one seeded violation that fails. Two further tests control the file sets
against the real repository rather than a fixture: one asserts the Markdown scope
reaches every extra root, the other that the non-Markdown walk reaches
[`.github/`](../../.github/README.md),
[`templates/`](../../templates/README.md), and the compile-fail cases, and
descends into none of the four
directories it declares it skips. A check can pass its predicate control because
its file list omitted the file it was supposed to read, and a repository that
passes looks identical either way.

```text
cargo test --locked -p doctrine-lint
```

## Doctrine

The enforcement obligation these checks satisfy is
[RUST-DOC-0011](../../doctrines/0011-executable-narrative/README.md), and the
rule that each doctrine rule names what enforces it is
[RFC-0007](../../rfcs/accepted/RFC-0007-every-rule-names-what-enforces-it.md).
