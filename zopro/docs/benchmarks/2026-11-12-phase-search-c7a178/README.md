# C7a solver-phase search test preparation

The target package is ready locally; **the AWS experiment has not run**.
A fresh STS request failed with an expired SSO token and failed refresh.
No cloud image was published and no instance was created. Production is unchanged.

Reuse accepted SDK `b646c7b8` and retained candidate `1f1f432a` from the
[local experiment](../2026-11-12-phase-search178/README.md). OFF and ON share
the candidate binary. Its local screen did not establish a benefit; the separate
question is whether solver-phase search participation explains C7a scaling loss.

The frozen target covers 1x2 and 2x2 at 32/96 workers, three arms, nine blocks
and three executable backings. Twelve preflights and 108 measurement processes
would produce 120 verified exports. Four subsequent gated counter/profile panels
would bring the target total to 124 exports and 17,596 SDK calls. These are planned
counts, not completed measurements. Report best-count latency separately from
the 96-worker improvement; this screen cannot authorize adoption.

The completed local container preflight made 16 SDK calls and produced four fresh
Gnark-verified exports. Its independent reader passed. The final container build
and profile-path/version check passed; target PMU access remains untested. No
successful native test, local performance screen or SDK build was repeated.

Private resumable root:
`/home/ubuntu/src/zopro-perf-work/phase-search-c7a178`.
Run ID: `phase-search-c7a-20261112-012656`.
Final local image: `sha256:39216f29e4cdaa9cafb3d309a077b70a8f59e8ba6ec92bb88e3ce8a0d333a7e7`.
The canonical ledger now contains 55,028 exports, SHA-256
`22edc7fdac8573e74fd2d476a7af32bc694f4c1ecb5c3ae3b24bde6940a0ff1b`.

Resume by validating prepared pins and the retained image, then use the existing
owned AWS lifecycle with fresh credentials. Preserve the single live measurement
handle through latency and profiles. Retrieve with archive hash verification,
reverify exports locally, analyze and verify resource cleanup. The immutable
prepublication manifest is pinned separately from the lifecycle's mutable manifest.

The full all-eight CPU10x goal remains unmet, 0/8. Accepted CPU/Metal results
remain in the [adoption report](../2026-11-12-bundle-adoption178/README.md).
This preparation report and its source records are local and unpushed.
