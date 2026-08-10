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

| Check                            | Obligation                                                                                         |
| -------------------------------- | -------------------------------------------------------------------------------------------------- |
| `check_reachability`             | every maintained Markdown file has at least one inbound link                                       |
| `check_path_references`          | every backticked path that resolves on disk is also linked, once per document                      |
| `check_workspace_crate_coverage` | every Cargo workspace member is linked, as a directory or a file inside it                         |
| `check_directory_indexes`        | every workspace crate, and every directory holding maintained Markdown, has a `README.md`          |
| `check_referenced_files`         | every remaining file is named by some maintained Markdown, and no Markdown sits outside every root |

There was briefly a sixth, comparing the `-p` package list in the examples
workflow against the workspace. It is gone, and the workflow now says
`--workspace --exclude` the three tools instead of naming each example. A gate
that guards a duplicated list is worth less than not duplicating the list: the
`-p` form had to be kept in sync, and the check that watched it could be
satisfied by a package named in a comment or tested only under Miri.

`check_reachability` asks about inbound links, not reachability from the root: a
file linked only by an unreachable file passes. That is the weaker claim, and it
is the one made rather than the one the name suggests.

`check_referenced_files` accepts a bare mention where the others demand a link.
Requiring a Markdown link to every crate manifest would add thirteen links no
reader wants; requiring the name to appear at all catches the class that matters,
which is a file the corpus has never heard of.

The mention has to name the whole file, on both sides. An unanchored search
credited a file because a longer name ended in its name; anchoring only the left
side then let a prefix through the same way. Two costs remain, and neither is
fixable without turning this into a link check: two files with the _same_
basename cover each other, and prose that discusses a filename names it. This
document does that deliberately in places, which is one reason this check does
not replace `check_path_references`.

Its walk is over the repository, not over whatever is on disk. It reads
[`.gitignore`](../../.gitignore) and Git's per-repository exclude file — files,
not Git; this binary calls no external process — and honours a deliberate subset
of that syntax: a bare name, a `*.suffix`, and either anchored with a leading or
trailing slash. Anchoring is respected, so a root-anchored pattern does not
apply at depth and an unanchored one does.

**Every pattern outside that subset is reported.** Not only a negation: a
mid-path slash, an embedded wildcard, and a character class are named too. The
first version detected negation alone and silently turned the rest into names no
file could equal, which is the "a rule this walk ignores must not look obeyed"
state its own comment claimed to prevent. A present-but-unreadable ignore file is
reported for the same reason.

Two ignore sources are **not** read, because both need Git or a config parser: a
user's global `core.excludesFile`, and nested `.gitignore` files. A file covered
only by those is reported here and has to be registered. If you keep editor or
tool state in the tree, put its pattern in [`.gitignore`](../../.gitignore) or in
the per-repository exclude file instead.

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
| `UNSCANNED_TOP_LEVEL`          | repository-root directories no connectivity gate walks  |

This table deliberately carries no entry counts. A count here would be a
hand-maintained integer restating a fact the source already holds — the drift
this repository has a check for, and the defect this release exists to remove.
Read the constants.

`UNSCANNED_TOP_LEVEL` is the boundary of what these gates can see, written down
rather than left as an accident of whichever walker skipped what. `dist/` is on
it because the bundler's own drift check already holds that tree to exact set
equality, over every extension.

It applies at the repository root only, as its name says. Matching those names at
any depth silently unwalked a nested `dist` or `target` that no drift check
covers. Depth belongs to the ignore file, which anchors the build directory to
the root and leaves the dependency directory unanchored, and that distinction is
honoured.

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
with one seeded violation that fails. Further tests pin the parsers the checks
depend on, and control the file sets against the real repository rather than a
fixture: that the Markdown scope reaches every extra root, and that the
non-Markdown walk reaches [`.github/`](../../.github/README.md),
[`templates/`](../../templates/README.md) and the compile-fail cases while
descending into none of the directories it declares it skips.

A check can pass its predicate control because its file list omitted the file it
was supposed to read, and a repository that passes looks identical either way.
Two review rounds found defects here that every green run had hidden, so each
control names the specific wrong behaviour it exists to catch rather than
asserting the check merely fires.

```text
cargo test --locked -p doctrine-lint
```

## Doctrine

The enforcement obligation these checks satisfy is
[RUST-DOC-0011](../../doctrines/0011-executable-narrative/README.md), and the
rule that each doctrine rule names what enforces it is
[RFC-0007](../../rfcs/accepted/RFC-0007-every-rule-names-what-enforces-it.md).
