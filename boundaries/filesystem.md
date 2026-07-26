# Filesystem boundary guide

## 1. What is untrusted?

Paths, directory entries, symlinks, metadata, file contents, mount behavior,
permissions, ownership, environment-derived base directories, archive members,
and concurrent filesystem changes are untrusted. A path string accepted by a
parser does not prove it names the intended object. Attackers or other processes
may replace components between checks and use.

Unicode and platform path representations are not universally valid UTF-8.

## 2. What parsing occurs?

Parse user-visible path syntax without forcing `Path`/`OsStr` through lossy
Unicode conversion. Join only after selecting a trusted base. Reject absolute
paths, parent traversal, device prefixes, or alternate separators according to
the target policy. Bound filename, path depth, directory entry count, file size,
archive expansion, and total extraction size.

File content parsing occurs only after resource limits and the appropriate
format/version checks.

## 3. What validation occurs?

Validate containment, allowed object type, ownership, permissions, link policy,
extension only where it is meaningful, and content invariants. Prefer
descriptor-relative operations or platform facilities that constrain traversal
when security depends on staying beneath a directory. Metadata checks and
operation must be coupled to avoid time-of-check/time-of-use gaps.

Do not rely on string prefix comparison for path containment.

## 4. How is a trusted type constructed?

A trusted path or opened-file capability is constructed by a filesystem service
after resolving through the approved policy and opening the actual object with
appropriate flags. Passing an already opened handle often preserves object
identity better than returning a checked path string. Content then decodes into
a raw representation and domain constructors.

For output, create a temporary file in the destination directory with explicit
permissions, write and flush as required, then atomically replace according to
platform semantics.

## 5. How can construction be bypassed?

Bypasses include concatenating strings, accepting absolute archive entries,
canonicalizing then reopening by path, following symlinks after a separate
check, using predictable temporary names, sharing a validated path while
another process can replace it, trusting extension as content type, and direct
filesystem calls outside the service.

Privileged repair tools and tests must follow or explicitly audit the same path
policy.

## 6. How is failure represented?

Distinguish invalid path policy, not found, wrong object type, symlink rejected,
permission denied, concurrent replacement, already exists, quota/no space,
partial read/write, content validation, lock contention, and durability
failure. Preserve OS source errors internally without exposing sensitive paths
or mount topology publicly.

After partial output failure, report whether the original, temporary, or
replacement file is present when that fact can be observed.

## 7. How are unknown or future values handled?

Version durable file formats. Reject, migrate, or preserve unknown versions.
Unknown directory entries should not be processed merely because they match a
broad glob. Archive formats and metadata require explicit supported subsets.
Platform-specific path forms, case sensitivity, and rename semantics belong to
the compatibility matrix.

Readers should tolerate safe additive fields only according to the file format,
not by ignoring every parse error.

## 8. How is sensitive data protected?

Create secrets with restrictive permissions from the first open, not by
tightening them after writing. Avoid exposing values in filenames, temporary
paths, logs, process arguments, or error messages. Ensure temporary and backup
files follow retention and deletion policy. Directory permissions, umask,
hardlinks, backups, snapshots, and crash dumps affect confidentiality.

Secure deletion is not generally guaranteed by ordinary file removal,
copy-on-write filesystems, or storage hardware.

## 9. How is evidence tested?

Test traversal, absolute paths, symlink chains, race-capable replacement where
the platform permits, wrong object types, permissions, Unicode/non-UTF-8 names,
partial writes, disk-full injection, temp cleanup, concurrent writers, lock
behavior, format versions, and oversized/archive-bomb inputs. Run on every
supported OS/filesystem family for semantic claims.

Crash tests verify atomic replacement and the configured durability sequence,
while documenting what the platform does not guarantee.

## 10. What remains uncertain?

An opened file can later be modified by another actor unless locking or access
control prevents it. Successful `write` may cover only a userspace/kernel
buffer; durable persistence depends on flush operations, directory metadata,
filesystem, mount, device, and power-failure behavior. File locks vary by
platform and cooperation. Network filesystems add distributed caching and
failure ambiguity.

## Safe replacement sequence

```text
trusted destination directory handle
    ↓ create unique temporary file with final permissions
write bounded complete content
    ↓ flush file according to durability requirement
atomic rename/replace within same filesystem
    ↓ flush directory metadata where required
report exact achieved guarantee and cleanup residuals
```

Atomic visibility and power-loss durability are different claims.

## Review prompts

- Does path containment rely on opened directory capabilities rather than
  string prefixes?
- Can a symlink, hardlink, mount, or concurrent rename change object identity
  between validation and use?
- Are temporary files unique, same-filesystem, permissioned at creation, and
  cleaned on every error?
- Are write completion, atomic replacement, file flush, and directory flush
  described as distinct guarantees?
- Do locks rely on cooperation, and what happens after process failure?
- Are non-UTF-8 and platform-specific paths preserved without lossy identity?
- Can archive expansion exceed per-file or total limits?
- Which files, backups, snapshots, and logs may retain sensitive content?
