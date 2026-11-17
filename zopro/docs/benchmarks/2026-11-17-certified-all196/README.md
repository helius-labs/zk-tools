# Generated-proof certificates: qualified native opt-in

The exact candidate from experiment 194 passes all 33 declared performance and
resource checks and its remaining portable checks. Its tested source is adopted with
explicit `ZOPRO_BENCH_CERTIFIED_OUTPUT=1`. External proof parsing retains all
validation; without the selector, generated outputs retain ordinary validation.
No SDK was rebuilt or cleaned up for adoption.

Primary1x2/16 has a paired candidate/reference ratio of
**0.996150 [0.992989,0.999939]**, about0.385%lower latency. The paired median
absolute saving is **34.055µs**. The upper bound is close to parity; this is a
small qualified improvement under the declared local protocol, not a breakthrough.
All three primary16 backing medians are below1:0.998727,0.994816,0.996908.

Primary32 is **0.992321 [0.990415,1.000062]**: a favorable point estimate, with
the interval still including parity. Its gain remains inconclusive. All nine
non-primary latency guards,ten CPU guards andten peak-RSS guards pass. No minimum
gain floor, excluded backing, extra sample or rescue replay was used.

| Local Ryzen9950X context | REF ms | Adopted ON ms | Paired latency ratio | REF / ON peak MiB |
|---|---:|---:|---:|---:|
| 1x2,16workers | 8.888139 | 8.837719 | 0.996150 | 77.50 / 75.50 |
| 1x2,32workers | 7.701332 | 7.628251 | 0.992321 | 92.50 / 96.00 |
| 2x2,32workers | 10.681639 | 10.708637 | 1.001259 | 239.25 / 235.25 |
| 4x2,32workers | 15.778171 | 15.851327 | 1.001672 | 288.25 / 295.00 |
| 8x2,32workers | 24.022553 | 24.008343 | 1.002485 | 472.00 / 484.25 |
| 12x2,32workers | 33.899084 | 34.020761 | 1.003351 | 664.75 / 675.25 |
| 16x2,32workers | 39.971984 | 40.083109 | 1.004123 | 789.50 / 790.75 |
| 24x2,32workers | 66.410292 | 66.332847 | 0.998430 | 1208.35 / 1253.14 |
| merge24x1,32workers | 65.913984 | 66.233652 | 1.005061 | 1141.25 / 1097.59 |
| merge54x1,32workers | 163.336164 | 163.384864 | 0.997120 | 1817.85 / 1881.32 |

Times are medians of30 process means. Paired ratios and the34.055µs saving use
within-block differences/ratios and need not equal differences/ratios of the
displayed medians. RSS is the largest whole-process peak including loading and
priming, not minimum RAM or added certificate storage. Larger fallback shapes
show build/run variation; their differences cannot be attributed to an inactive
certificate path. Primary16 CPU ratio is1.000253 [0.999332,1.003125],primary32
1.001376 [0.995588,1.005933]; no CPU reduction is established by these intervals.

The primary16 latency upper must be<1 and all three backing medians<1. Other nine
latency uppers and all ten CPU uppers must be<=1.02; each maximum RSS must be
<=REF*1.05+1MiB. These33 checks were frozen before preflight. Bounds use sorted
paired process-mean ratios at one-based ranks10/21 of30: nominal95.7226%marginal
median coverage under independent exchangeable blocks, without a joint confidence
claim. A second same-agent reader recomputes every raw process statistic and all
33 decisions using a separate order-selection implementation. This is not an
external audit. See [assessment.json](artifacts/assessment.json) and
[independent-main-review.json](artifacts/independent-main-review.json).

The certificate moves one validation dependency to immutable input preparation.
It checks every eligible B2 point, plus fixed beta2/delta2, on-curve and in the
subgroup. Group closure then certifies freshly generated G2 outputs, including
the ordinary A0=identity fallback. The opaque generated-output wrapper retains
coordinate bounds and curve checks. Public external `Proof::from_raw`, pairing
verification and fresh rerandomization remain checked as before. Invalid input
certificates fall back to ordinary parsing. The existing trusted mapped-image
contract requires key bytes to remain unchanged while used.

There are two small owner-bound OnceLocks and global observer counters, with no
point/scalar table, key copy, new owned point buffer or worker pool. Racing first
callers may duplicate cold validation; the repaired implementation scans before
publishing a completed Boolean, so no initializer is held across Rayon work.
The native route is explicit opt-in and limited to B2 lengths below65536.
Large/unsupported keys skip scanning and keep ordinary output validation.

Cold certificate validation adds setup work. The retained
[screen](../2026-11-17-certified-proof194/README.md) measured median scan spans
of63.160ms forprimary16,52.056ms forprimary32,and88.676ms for2x2/32. These are
not full first-proof latency or CPU service. A first proof pays that work; the
qualified latency is warm and suitable only when preparation can be amortized.
The three priming calls remain outside timing. This is why adoption keeps the
selector explicit. No phone or one-shot benefit is claimed.

All enabled primary/2x2 processes record one cold key scan and one cold fixed-input
scan,zero timed scans,andone certified parse per timed request. Larger ON shapes
record zero scans/fast parses. Three configured scratch-denial preflights pass
and retain indexed-path admission assertions. Configured denial is not physical
OOM or proof of operation at any particular RAM limit.

This campaign has23preflights and600main SDK processes,30matched rounds in ten
contexts,17timed requests plus3primes per main process. Contexts rotate and reverse,
arm order alternates,andthree existing independent backings each supply ten pairs
per context. All timing precedes independent verification within each phase.
No owned build,hashing,analysis or reporting overlaps capture. Timed fresh requests
include witness solving,randomness,proving,SDK verification and serialization;
loading,priming/certification andexport are excluded.

All623fresh exports independently verify with Gnark, fixed input/image bindings
and global freshness checks; the SDK verifies all12092calls includingprimes.
Across screen/profile/qualification,this is724exports and19426SDKcalls. The
[exact-build profile](../2026-11-17-certified-profile195/README.md) retains all4
observer and12PMU quality checks, small instruction differences and mixed cycles.
It does not assign exclusive critical-path savings.

Release qualification inherits134native core,2prover and6cubic passing tests from
the exact194source. New unfiltered portable no-IFMA/no-ASM suites pass102core,
2prover and6cubic tests,noneignored. New certificate tests cover invalid/non-subgroup
points,external parser rejection,distinct fixed owners,andfour simultaneous OS
callers on1/4/16/32worker pools with512KiB stacks. The first native initializer
deadlock and its pre-SDK repair remain preserved in194. Unchanged field arithmetic
qualification is inherited by source identity. No successful native test or SDK
capture was replayed for this release. An initial196setup-helper compile failed
on a moved String before setup; its borrow-only repair changed no runtime source.

All179measured source hashes and release receipts were rechecked before copying
the exact four changed parents andnew module into production. The complete179file
production manifest matches afterward. There is no cosmetic integration change.
See [release-review.json](artifacts/release-review.json),
[production-source-manifest.json](artifacts/production-source-manifest.json),and
[decision.json](artifacts/decision.json). Assessment and independent-reader snapshots
precede adoption; the decision and manifest record the final adopted status.
Worker count is not fixed by the manifest:
the strict gain is qualified at16; the best measured primary latency is at32.

| Shape,32workers | Adopted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup | CPU10x target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.628251 | 37 | 4.85x | 24 | 3.15x | 3.7 |
| 2x2 | 10.708637 | 59 | 5.51x | 30 | 2.80x | 5.9 |
| 4x2 | 15.851327 | 109 | 6.88x | 48 | 3.03x | 10.9 |
| 8x2 | 24.008343 | 189 | 7.87x | 74 | 3.08x | 18.9 |
| 12x2 | 34.020761 | 284 | 8.35x | 101 | 2.97x | 28.4 |
| 16x2 | 40.083109 | 335 | 8.36x | 124 | 3.09x | 33.5 |
| 24x2 | 66.332847 | 528 | 7.96x | 176 | 2.65x | 52.8 |
| merge24x1 | 66.233652 | 507 | 7.65x | 175 | 2.64x | 50.7 |

M5 comparisons are descriptive ratios against supplied historical references on
different hardware; no M5 was remeasured. Merge54 is163.384864ms without a supplied
M5 reference. The full all-eight CPU10xgoal remains active and unmet,0/8. No fresh
C7a/Aeglos comparison,physical ARM/mobile,huge-circuit,pressure/swap,minimum-RAM or
concurrent-throughput claim follows. No AWS resources were created. Adoption and
this report are committed locally; nothing is pushed.

SDK SHA256:`bcc8e0775bb5bc6c09ad86b1987ca45eebac62617340911266c84aa601fa05b1`.
Previous REF:`c285cbc88f783abac203c66db43ea86bda8207338b9fb209df7eb5d846126202`.
Canonical57410->57433->58033,SHA256
`eeb7ab3b169b922a23cb95bc8b447d86e9ab2df3b72347e48d738ca13ca32618`.
Private root:`/home/ubuntu/src/zopro-perf-work/certified-all196`.
Plans,readers,raw samples,proofs andreceipts are retained in
`artifacts/evidence.tar.gz`;integrity hashes are in [SHA256SUMS](SHA256SUMS).
