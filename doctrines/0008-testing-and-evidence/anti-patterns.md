# Anti-pattern catalogue

## Happy-path certificate

**Weak example.** One accepted value test is cited as proof a validated type is
correct.

**Why it fails.** The excluded set and boundary behavior are unobserved.

**Risk.** invalid construction and weak error contracts.

**Improved direction.** test accepted, rejected, boundary, normalization, and
conversion paths.

**When justified.** A thin delegation may test integration while citing the
complete underlying suite.

## `is_err()` everywhere

**Weak example.** Tests assert only that some error occurred.

**Why it fails.** Error categories can collapse or the wrong validation can
trigger.

**Risk.** callers lose actionable distinctions.

**Improved direction.** assert structured category and relevant safe context.

**When justified.** Exact category may be intentionally opaque at a security
boundary; assert the public contract there.

## Examples as properties

**Weak example.** Three hand-picked strings are used to claim parser robustness.

**Why it fails.** broad input partitions and interactions remain unexplored.

**Risk.** rare panics or normalization defects.

**Improved direction.** define properties and generators plus targeted edge
fixtures.

**When justified.** Exhaustive small finite domains can use tables.

## Self-confirming oracle

**Weak example.** A serializer is tested by decoding only with its paired
implementation and comparing a value both normalize identically.

**Why it fails.** shared defects can preserve round trips.

**Risk.** external incompatibility.

**Improved direction.** add specification fixtures or independent
implementation/contract evidence.

**When justified.** Round-trip remains one useful property among independent
checks.

## Compile-fail for the wrong reason

**Weak example.** A UI fixture lacks an import, so it fails before testing field
privacy.

**Why it fails.** the harness is green while the intended prohibition may have
vanished.

**Risk.** forged trusted values.

**Improved direction.** minimize source and inspect exact diagnostics.

**When justified.** Never as evidence for the intended rule.

## Overwrite compiler output

**Weak example.** All new `.stderr` files are accepted after a toolchain update
without reading them.

**Why it fails.** expected failure cause can change.

**Risk.** silent API weakening.

**Improved direction.** review each diagnostic semantically and group only
equivalent wording changes.

**When justified.** Formatting-only compiler changes can share a documented
review.

## Perfect mock network

**Weak example.** The mock either returns success or fails before execution.

**Why it fails.** it cannot express delayed success, lost response, or duplicate
execution.

**Risk.** timeout collapses into rejection.

**Improved direction.** use a controllable fake with failure points and real
integration evidence.

**When justified.** A narrow pure mapping unit test may omit network semantics.

## Sleep for ordering

**Weak example.** task A sleeps ten milliseconds so task B is expected to run.

**Why it fails.** scheduler and host timing are not controlled.

**Risk.** flakiness and false schedule evidence.

**Improved direction.** use barriers, channels, paused clocks, or model
checking.

**When justified.** Sleep may define a deadline, not establish the event.

## Retry green

**Weak example.** CI retries a flaky test until one run passes and reports
success.

**Why it fails.** the failure mechanism and frequency disappear.

**Risk.** races and production instability persist.

**Improved direction.** capture first failure, make it reproducible, fix cause,
and use temporary visible quarantine only.

**When justified.** Bounded retries may gather diagnostic samples while owned
remediation proceeds.

## Snapshot approval as review

**Weak example.** A broad snapshot update command is followed by commit because
the files now match.

**Why it fails.** current behavior becomes expected without semantic judgment.

**Risk.** overclaims, UI regressions, or diagnostic weakening.

**Improved direction.** classify and explain focused changes.

**When justified.** A deterministic mechanical formatting migration may be
reviewed as one classified transformation.

## Coverage target as quality

**Weak example.** A high line percentage is the sole release criterion.

**Why it fails.** executed lines may contain no meaningful assertions or
adverse inputs.

**Risk.** untested invariants under impressive metrics.

**Improved direction.** maintain an invariant-to-evidence matrix and use
coverage for gap discovery.

**When justified.** Coverage can enforce that new code is not wholly
unexercised.

## Production has not failed

**Weak example.** absence of incidents is cited as proof a protocol is correct.

**Why it fails.** failure may be rare, invisible, or absent from observed
workloads.

**Risk.** unsupported guarantee claims.

**Improved direction.** test detection, inject faults, and state observation
limits.

**When justified.** Production evidence can update likelihood estimates, not
establish impossibility.
