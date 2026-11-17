# Adopted CPU SDK194 on C7a: all circuit shapes

**The exact adopted SDK is now measured on C7a across all eight supplied shapes and merge54.** Primary 1×2 is **11.098 ms at 32 cores** and **12.515 ms at 96**. Pinned Aeglos Sorted is **12.143 / 8.311 ms** respectively: Zopro is faster at 32 cores, while Aeglos has the lower best tested latency. None of the eight shapes meets the supplied M5 CPU 10× target.

This is a new measurement of an existing adopted build, with no runtime change. SDK SHA-256 is `bcc8e0775bb5bc6c09ad86b1987ca45eebac62617340911266c84aa601fa05b1`; all 179 production sources still match [qualification196](../2026-11-17-certified-all196/README.md). The flags are `ZOPRO_BENCH_CERTIFIED_OUTPUT=1`, `ZOPRO_BENCH_FUSED_PREPARATION=1` and `ZOPRO_CPU_INDEXED_TILE_BUNDLES=1`. The absent-certificate control uses the same executable with only the certificate flag set to zero.

## Complete-proof timings

One c7a.24xlarge, 96 physical EPYC 9R14 cores, no SMT; native host execution with fixed process CPU masks and existing individual Zopro worker placement. Values are medians of nine process means, each containing 17 timed, fresh, internally verified proofs after three excluded primes. Key loading and preparation are outside latency and inside lifetime RSS. SDK clocks include solving, quotient work, fresh blinding, proof construction, SDK serialization and verification; Aeglos uses its existing `verified_ms` boundary.

M5 values are supplied historical references on different hardware, not fresh measurements. Both declared C7a counts are retained. Speedups divide each reference by the displayed-context unrounded median.

| Shape | 32-core ms | 96-core ms | M5 CPU ms | CPU speedup 32 /96 | M5 Metal ms | Metal speedup 32 /96 |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 11.098 | 12.515 | 37 | 3.33× /2.96× | 24 | 2.16× /1.92× |
| 2x2 | 14.800 | 16.591 | 59 | 3.99× /3.56× | 30 | 2.03× /1.81× |
| 4x2 | 19.774 | 20.603 | 109 | 5.51× /5.29× | 48 | 2.43× /2.33× |
| 8x2 | 27.414 | 27.645 | 189 | 6.89× /6.84× | 74 | 2.70× /2.68× |
| 12x2 | 35.783 | 36.061 | 284 | 7.94× /7.88× | 101 | 2.82× /2.80× |
| 16x2 | 41.734 | 42.277 | 335 | 8.03× /7.92× | 124 | 2.97× /2.93× |
| 24x2 | 61.021 | 58.541 | 528 | 8.65× /9.02× | 176 | 2.88× /3.01× |
| merge-24x1 | 62.309 | 62.711 | 507 | 8.14× /8.08× | 175 | 2.81× /2.79× |

Merge54 is **142.890 ms at 32 cores /126.856 ms at 96**, without a supplied M5 reference. Its paired 96/32 ratio is 0.876322, with 9/9 wins. For 24×2 the corresponding ratio is 0.950874, also 9/9 wins. Primary instead has 96/32 =1.126285 with 0/9 wins. Higher core counts help some large cases; the result is not a uniform scaling failure.

All sixteen M5 worker-context checks fail: the upper rank7 process-mean bound exceeds CPUref/10. The closest point estimate is 24×2/96 at about 9.02×, with 59.466 ms upper bound versus its 52.8 ms target. Full per-context bounds, samples, CPU/Metal ratios and all backing medians are retained in the artifacts. The full objective remains active, 0/8.

## Primary controls

The peer is the retained [Aeglos reference](https://github.com/helius-labs/aeglos/tree/0290613e8eeca17a4ce88f6f2d726e8e06a5385d), executable `eab9fbfd4b94aeef9ced3ce3e3ede21a37ab70ad38b217720f1afd06fceba99b`. This is not a newly built or latest-branch Aeglos measurement. Both engines process the same request/public binding; Gnark verifies exports against the Zopro image's bound verifying key.

| Workers | SDK certificate OFF ms | Adopted ON ms | Aeglos Sorted ms | OFF /ON /Aeglos peak RSS MiB |
|---|---:|---:|---:|---:|
| 32 | 11.245 | 11.098 | 12.143 | 84.58 /79.80 /409.96 |
| 96 | 12.505 | 12.515 | 8.311 | 73.10 /75.27 /460.98 |

At 32 cores, paired ON/OFF is **0.986170 [0.978216,0.999130], 8/9 wins**: target-side gain evidence for the already adopted flag. Displayed median latency falls by 146.988 µs; this difference and the paired ratio summarize different distributions. At 96 cores, ON/OFF is **0.995240 [0.984912,1.020687], 5/9**: inconclusive. The pooled ON median is slightly higher there; it does not divide into the paired median.

Paired Aeglos/ON is 1.093662 [1.068327,1.201868] at 32 cores and 0.660418 [0.654660,0.679671] at 96 cores. Comparing each engine's best declared count per block gives 0.755514, with 9/9 Aeglos wins. Best latency and equal-worker comparisons answer different questions. These results do not establish memory capacity, page faults or scheduling as the sole cause.

Intervals use sorted zero-based ranks1..7 from nine paired blocks, nominal 96.09375% marginal median coverage under independent continuous-block assumptions; there is no familywise guarantee. No gain floor or new adoption veto is imposed by this measurement.

## Resource observations

CPU values are median timed user+system CPU per proof. RSS is maximum lifetime process RSS across the nine runs, including loading and primes; it is not minimum RAM.

| Shape | CPU ms/proof 32 /96 | Peak RSS MiB 32 /96 |
|---|---:|---:|
| 1x2 | 237.67 /638.83 | 79.80 /75.27 |
| 2x2 | 326.57 /917.74 | 216.21 /209.39 |
| 4x2 | 505.86 /1347.23 | 269.04 /268.04 |
| 8x2 | 754.52 /1933.93 | 456.12 /462.16 |
| 12x2 | 1004.40 /2482.63 | 645.41 /648.48 |
| 16x2 | 1185.82 /2814.09 | 768.43 /769.21 |
| 24x2 | 1765.22 /3749.67 | 1197.44 /1223.35 |
| merge-24x1 | 1723.65 /3825.86 | 1093.64 /1138.06 |
| merge-54x1 | 3665.96 /6331.00 | 2148.99 /1958.43 |

The native service has MemoryMax=30 GiB and MemorySwapMax=0; both are observed before and after, with zero high/max/OOM/kill events. This is infrastructure headroom, not a prover architecture limit or physical-pressure qualification. Input files were extracted outside that service, so mapped file-cache pages can be charged elsewhere. Service memory.peak is therefore not a complete process-footprint measurement. `cpu.max` is unavailable in this unit cgroup; no CPUQuota option was supplied, but ancestor quotas were not independently recorded. No absence-of-throttling claim is made.

## Verification and retained evidence

Four local functional smoke exports precede launch: primary ON/OFF/Aeglos and merge54 ON. All pass independent Gnark checks before packaging; their proofs enter the remote freshness seed. Twenty-two independently verified remote preflights precede 198 main processes in nine rotated/reversed blocks. Each engine has three byte-identical executable copies with distinct inodes; copies rotate by block. All main captures finish before late independent verification. No owned build/analysis overlaps capture.

The campaign completes **4,048 remote requests and 220 distinct remote exports**, plus 16 local requests/four exports. All 220 remote exports pass a second host-side Gnark verification with proof/public/image bindings and freshness against prior history. The canonical inventory becomes **58,257**, SHA-256 `7f4ec2097957bcc2f063d32778300edba00d8adc238787e44c0d3e9e2e2eab9e`, at private `host-remote/proof-sha256.json`. Reverification does not count as additional distinct proofs.

SDK/tool/config pins, input hashes before/after capture, all source hashes, primary certificate activation, worker placement, output bindings and six backing identities pass their checks. A separate same-agent reader reconstructs means, intervals, all M5 comparisons and primary ratios from retained receipts. This is not an external audit. The offline reader initially assumed `cpu.max` existed; the preserved repair records its absence and limits the quota claim. No timing rule or sample changes, and no successful SDK/verifier process is replayed. No prover rebuild, new field suite, PMU profile, ARM/phone, forester 250/4999, pressure, swap-performance or throughput claim is added.

The remote service completes in 4 min 39.365 s. Protocol-to-remote completion is about 14 min 31 s, within the 30-minute preparation/first-result allowance. One owned VM `i-06be398bc3f0ff17b` is terminated; its volume is absent and the temporary transfer bucket/objects are deleted. No ECR image is created. Input transport is private. Raw points, witnesses, keys, proof/public bytes, full ledgers, executables and secret URL parameters are excluded from the public evidence archive.

The exact adopted SDK has a current full-shape C7a baseline now. Retain its small qualified improvements, keep the closed tree/shifted/equality screens closed, and use these results when selecting a distinct reduction in complete-caller work. More unchanged baseline runs or a worker/width sweep are not selected. No new runtime candidate is adopted. Changes remain local; nothing is pushed.

[Protocol](artifacts/protocol.md), [analysis](artifacts/analysis.json), [independent derived checks](artifacts/derived-review.json), [host verification](artifacts/host-review.json), [local smoke verification](artifacts/smoke-review.json), [consumer/input configuration](artifacts/config.json), [source and receipts](evidence.tar.gz), [checksums](SHA256SUMS).

Private root: `/home/ubuntu/src/zopro-perf-work/accepted-c7a201`.
