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

**Connectivity.** Seven checks, described below.

**Enforcement and evidence.** Every rule names an enforcing artifact that exists,
or states why it is unenforceable. Every review gate declares whether it is
judgment or a named mechanical command. Every generated file under
[`dist/`](../../dist/README.md) carries its banner.

**Pinned toolchains.** Three versions decide how this repository builds: the
development channel, the MSRV, and the Miri nightly. Each is declared once and
restated several times — in the clippy configuration, the examples matrix and its
display names, the CI install step, the Miri job's own name and install step, and
every Miri invocation the corpus gives a reader to run.
`check_pinned_versions` holds each restatement equal to the file that declares it,
reading [`rust-toolchain.toml`](../../rust-toolchain.toml),
`[workspace.package] rust-version`, and the workflow through the same parsers
Cargo and the workflow runner use. It never asks what rustup would resolve on a
contributor's machine, which is not a question this repository can decide from
what it owns.

The scan reads a toolchain only where one could actually be resolved: a named
channel, or a spec containing a digit. This paragraph is why. Its first draft
wrote a metasyntactic `cargo +…` command to describe the check, and the check
read that stand-in name as a toolchain and reported this file — a gate whose own
documentation cannot be written without tripping it is a gate that will trip on
the next author too.

## The connectivity gates

The repository is a graph a reader navigates by clicking. These checks hold the
invariant that every node **inside a declared root** is reachable, or is
registered as deliberately not. The qualifier is load-bearing: the root lists are
the real specification, and `check_declared_top_level` is what stops a directory
from sitting outside all of them unnoticed.

| Check                            | Obligation                                                                         |
| -------------------------------- | ---------------------------------------------------------------------------------- |
| `check_reachability`             | every maintained Markdown file has at least one inbound link                       |
| `check_path_references`          | every backticked path that resolves on disk is also linked, once per document      |
| `check_declared_top_level`       | every top-level directory is a declared root, or is registered with a reason       |
| `check_workspace_crate_coverage` | every Cargo workspace member is linked, as a directory or a file inside it         |
| `check_directory_indexes`        | every workspace crate, declared root, and linked directory has a `README.md`       |
| `check_workflow_index`           | the workflow index names exactly the workflows on disk                             |
| `check_lint_parity`              | every crate that cannot inherit workspace clippy lints restates them without drift |

`check_directory_indexes` builds its required set from committed content —
workspace members, the root lists, and every directory a maintained document
links to — never from a filesystem walk. A directory something links to is one a
reader can arrive at, so it owes the reader an index; generated output under
`dist/` and sanctioned scratch space are excluded, because the bundler emits no
index into what it generates and scratch is not repository content.

Deriving the requirement from whatever Markdown happened to be on disk was tried
and reverted twice over. It made a gitignored scratch directory fail the
mandatory sequence, would have forced an index into the GitHub issue-template
directory that GitHub renders as a selectable form, and silently exempted a
directory holding only dated records. Dropping it entirely was worse: two dozen
nested indexes lost their obligation, and an index could link a directory with
nothing to read in it while every gate stayed green. Doctrine packages keep their
index requirement through `check_front_matter`, which reads the path from the
manifest.

`check_lint_parity` reaches every member that does not write `[lints] workspace =
true`. Cargo has no partial inheritance, so a crate needing any lint entry of its
own must restate the whole clippy table; guarding only the one crate that had
already diverged would leave the next one silently outside the configuration.

Two checks that once sat here are gone, and both removals are the point rather than a
retreat.

A gate compared the `-p` package list in the examples workflow against the workspace.
The workflow now says `--workspace --exclude` the three tools instead of naming each
example, so there is no duplicated list left to guard. A gate that watches a duplicate is
worth less than not duplicating.

A gate required every remaining file — every workflow, configuration and snapshot — to be
named by some document. It asked a question worth asking once: twenty-six files were named
by nothing, and they are connected now. Keeping it permanently was the mistake. Answering
it needed a walk of the working tree, and a working tree is the repository plus whatever a
contributor's tools left there, so being correct meant reimplementing `.gitignore`
semantics. Three review rounds produced a new false failure each time — a filename ending
a sentence, a `git worktree` checkout, a developer's own exclude file, an editor
directory — and every one of them landed in a mandatory pre-commit sequence, which is the
worst place a false failure can land. The question was one-time; the gate was permanent;
the cost was permanent too.

What remains are the checks that earn it. Each answers a question about the corpus using
only the corpus: which documents link which, and which crates the workspace declares.
Neither needs to know what is on a contributor's disk.

`check_reachability` asks about inbound links, not reachability from the root: a file
linked only by an unreachable file passes. That is the weaker claim, and it is the one
made rather than the one the name suggests.

`check_workspace_crate_coverage` requires the link to come from outside the crate. A
crate whose own README is the only thing linking into it is an island, not a reachable
crate, and the two are indistinguishable to a check that pools every link together.

## The registers

Two constants record what is deliberately exempt, and why. Each is a
`&[(&str, &str)]` of path and reason, matched by exact string equality so a file
cannot acquire an exemption by resembling one; a test asserts that every reason
is long enough to be a reason rather than a word, and that every path named still
exists.

There were four. The two that are gone belonged to the file-coverage gate, and both were
growing: one had accrued ten entries in a single release, nine of them compiler snapshots
a contributor would have had to register one at a time. A register that a routine change
has to feed is a register nobody reads, and it was a symptom of the gate rather than a
property of the corpus.

| Register                  | Exempts                                                 |
| ------------------------- | ------------------------------------------------------- |
| `REACHABILITY_EXEMPTIONS` | documents a reader opens directly, and generator inputs |
| `INDEXLESS_DIRECTORIES`   | directories that legitimately carry no index            |

This table deliberately carries no entry counts. A count here would be a
hand-maintained integer restating a fact the source already holds — the drift
this repository has a check for, and the defect this release exists to remove.
Read the constants.

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
depend on, and control the Markdown scope against the real repository rather than
a fixture: that it reaches every extra root, and that it lists each file once.

This paragraph previously also claimed a control on a non-Markdown walk. That walk
and its test were deleted in 0.9.0 with the file-coverage gate, and the sentence
describing them survived the commit — so the README promised a pinned boundary
that no test held, which is worse than promising nothing. It is recorded here
rather than quietly corrected, because a maintainer who later narrows
[`REACHABILITY_EXTRA_ROOTS`] is the person the false claim would have misled.

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
