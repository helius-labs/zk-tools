# Prepared C7a MSM phase comparison

The matched C7a diagnostic is packaged locally with the exact SDKs from the
[local phase comparison](../2026-11-15-msm-local180/README.md). Four new container
proofs pass Gnark verification inside the container and again on the host.
The complete target plan passes structural checks without running target proofs.
**No AWS capture, new performance result or production change is claimed.**

The question remains which MSM phase accounts for Sorted's improvement at
32→96 physical cores while current Zopro full-request time increases. Local
SMT measurements do not reproduce that behavior. This package carries the
qualified coarse observers to the relevant topology and retains controls for
observer overhead and differences between freshly built and retained binaries.

## Frozen target

Primary 1x2 only: Zopro accepted REF and diagnostic OFF/ON; Aeglos retained REF,
fresh original native source with matched harness, and diagnostic OFF/ON,
separately for Sorted and Windowed. All five distinct executables are reused;
the prover code is not rebuilt. Key, request, public-input bindings and existing
preparation policies remain fixed.

Each executable has three independently written, byte-identical files. The
container checks 15 distinct device/inode pairs and matching hashes. Nine blocks
rotate controls and worker counts, with backing index `block % 3`, giving three
blocks per backing in every context. This addresses the previous local panel's
single-backing limitation; it does not establish target calibration before data
is captured.

| Target item | Fixed count |
|---|---:|
| Worker counts | 32, 96 physical cores |
| Functional preflight processes | 22 |
| Timed processes | 198 |
| Timed requests per main process | 51, after 3 primes |
| Total SDK calls, including preflights/primes | 10,780 |
| Fresh independently verified exports | 220 |
| Calibration/observer interval checks | 16 |
| Additional per-backing median checks | 48 |

The 16 paired-ratio intervals and all 48 backing medians must lie inside
[0.95, 1.05] for their respective controls. These qualify diagnostic
representativeness, not a production improvement. Environmental checks require
the expected physical topology and no cgroup CPU throttling or memory-limit/OOM/
swap events during capture. Missing data rejects interpretation. The inherited
30 GiB container ceiling is an isolation guard, not a recommended prover memory
budget or a test of minimum RAM.

All emitted phase records, including primes, must be complete, ordered and
bounded with conserved per-job counts. Phase means exclude primes. Zopro's
overlapping query callbacks may help other work or wait; they are not exclusive
CPU spans. Scheduled items are not equal-cost operations. Full-request timing
includes observer setup and emission. Preserve residuals and every merge round.
The [prospective protocol](artifacts/protocol.md) contains the exact scope and
interpretation rules.

## Completed local validation

The new container boundary is checked with Zopro ON, Aeglos Sorted FRESH and ON,
and Windowed ON at four workers: 16 SDK calls and four fresh exports. All four
exported proofs independently verify twice, with fixed image/public bindings,
and are disjoint from the prior canonical ledger. The host reader also checks
raw sample accounting, 12 phase records, stream receipts and source pins.

The initial container attempt could not create its output directory and stopped
before any SDK call. Correcting the dedicated bind-directory permissions allowed
the same image to pass. Both attempts are retained. No successful SDK capture or
verification was replayed. The final image adds the verified smoke receipt;
its plan-only check executes zero SDK calls.

The dry plan validates 220 unique jobs, 10,780 calls, 662 workflow steps, matching
worker/environment/affinity settings, 15 distinct executable backings and 66
mode/worker/arm/backing contexts with three blocks each. Main proof verification
follows all timed captures. The existing owned AWS lifecycle and transfer helper
are adapted to a new run ID; they retain one-instance ownership checks, a
90-minute termination timer, encrypted delete-on-termination storage, no ingress
and cleanup of the instance, volume, temporary registry tag and transfer bucket.
These cloud actions have not been executed for this package.

The host finalizer verifies transferred proofs and unions their disjoint delta
into the actual current canonical ledger. It preserves later local proof history
rather than replacing it with the target's older seed branch. Its four-export
container integration passes; the 220-export AWS path remains unexecuted.

Protocol creation to the sealed package took about 9min54s, including the
container builds, compatibility check, host verification and plan review.
This is preparation elapsed time, not a proof-latency measurement.

## Current state

Final local image:
`sha256:972261a656107c5c5368571bf809a2edc7d4dfd3aaeb01f30a3b7fbb8e3d1d11`.
Run ID: `msm-c7a180-20261115-192941`. No registry upload or instance launch.
AWS SSO authentication fails because the token expired and refresh failed.
There are no live capture handles or owned AWS resources from this package.

Current canonical ledger is **55,444**, at
`/home/ubuntu/src/zopro-perf-work/msm-c7a180/smoke-transport/proof-sha256.json`,
SHA256 `dbcd55c1e554260a2f26e8e5f3b4d25e1eb22b31578a6be3520cd733be9b8ec3`.
An otherwise unchanged successful target run would produce 55,664; merge against
the actual current ledger if other verified exports appear first.

The full all-eight CPU10x goal remains unmet (0/8). Accepted production timings
and CPU/Metal comparisons remain unchanged. No optimization, all-shape,
huge-circuit, mobile, memory-pressure or throughput result follows from this
preparation. No GitHub publication is authorized.

Private root: `/home/ubuntu/src/zopro-perf-work/msm-c7a180`.
[Package checks](artifacts/package-review.json),
[host verification](artifacts/smoke-review.json),
[resume instructions](artifacts/resume.md),
[sources, frozen plans and receipts](artifacts/preparation-records.tar.gz).
