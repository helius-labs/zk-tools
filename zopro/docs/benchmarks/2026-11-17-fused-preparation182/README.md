# Fused canonical conversion and query extraction

The candidate passes all 33 local release checks and is integrated as an explicit
opt-in; see release qualification below. The preceding screen and profile are
retained separately. The candidate
converts each wire block and immediately gathers its A/B scalars, removing the
global conversion-to-gather barrier and separate gather iterator trees. It adds
no lookup table, persistent metadata or output buffer beyond the existing two
query vectors. Enable the measured path with `ZOPRO_BENCH_FUSED_PREPARATION=1`;
without that setting the previous path remains active. C7a comparison is pending.

| Local Ryzen 9950X case | Accepted REF, ms | Candidate OFF, ms | Candidate ON, ms | REF / ON peak RSS, MiB |
|---|---:|---:|---:|---:|
| 1x2, 16 workers | 8.948060 | 8.847901 | 8.815680 | 80.00 / 78.25 |
| 1x2, 32 workers | 7.721298 | 7.668980 | 7.566416 | 111.75 / 101.00 |
| 2x2, 32 workers | 10.751928 | 10.686846 | 10.561951 | 270.75 / 263.25 |

Times are medians of nine process medians. RSS is the largest whole-process
peak, including loading and primes; it is not minimum required RAM. This is a
local screen, not a C7a result or a new Aeglos comparison.

The paired ON/OFF median ratios are 0.991961, 0.985234 and 0.989692 respectively,
with 7/9, 9/9 and 8/9 winning blocks. Their descriptive rank-1-through-7 bounds
(zero-based sorted ranks) are [0.977751, 1.000392], [0.980674, 0.991862] and
[0.972767, 0.996148]. The 16-worker bounds include parity. These finite-sample
bounds assume independent blocks and are not simultaneous confidence bounds.
ON/OFF mean-CPU paired median ratios are 0.993201, 0.979920 and 0.981119.
All declared latency, CPU and RSS screen guards pass.

Three independent executable backings were balanced across nine rotated/reversed
blocks. The accepted reference's first backing was slower, particularly at 16
workers. Consequently the full ON/REF difference cannot all be attributed to
the algorithm. OFF and ON use the same candidate binary and backing; each of
their three backing medians improves in each case. No block or backing was
discarded. This screen advances the candidate; it does not establish adoption.

Both solver loaders already validate strictly increasing, in-range query indices.
The candidate partitions sorted indices and output slices at each wire midpoint;
leaves of at most 1024 wires convert once and fill disjoint spare-capacity slices.
Vector lengths are published only after all joined work returns. The leaf size
was fixed before capture. The existing canonical-wire eligibility and portable
fallback remain in place.

All 131 native core tests pass, including two added tests comparing converted
wires and selected outputs with a serial oracle across leaf boundaries, concurrent
calls, 1/4/16/32 workers and 512 KiB worker stacks, plus panic handling. All 83
fresh exported proofs pass independent Gnark verification and image/public-input
bindings. The panel made 2762 SDK calls, including primes and preflights.
Portable, all-shape and remaining resource qualification are still pending.

The exact candidate SDK SHA-256 is
`c285cbc88f783abac203c66db43ea86bda8207338b9fb209df7eb5d846126202`;
accepted REF is `b646c7b8c1d8a26a0db45fb3f67c453dd616bd9f02e25c36192721ae02855abe`.
Runtime selector: `ZOPRO_BENCH_FUSED_PREPARATION=0/1`.
Private experiment root: `/home/ubuntu/src/zopro-perf-work/fused-preparation182`.
The canonical proof ledger is now 55861 entries, SHA-256
`7c6fa1eeaace1afcf4736845278c8de7d03dce5a61f9794d9beaa30ddf4b7f3e`.

The archive preserves the frozen protocol, source overlay, build receipts,
all screen receipts, verification records and analysis. Binaries, proving keys
and large historical ledgers are retained privately. No AWS resources were
created and nothing was pushed. The all-eight-shape CPU 10x goal remains unmet;
retain the [accepted all-shape comparison](../2026-11-12-bundle-adoption178/README.md)
until this candidate completes qualification.

## Exact-build mechanism profile

The subsequent 32-worker profile passes all four observer checks and all twelve
counter/sample quality checks. Eighteen fresh exports independently verify;
4572 SDK calls extend the canonical ledger to 55879, SHA-256
`dce77ba26e6b6635bfcdfaa65b98af87dc9f493c6f884544ecfdde48ad4bcec4`.
The retained SDK and three screen backings are unchanged.

ON is faster in all three plain blocks (7.550–7.578 ms versus 7.628–7.692 ms).
All three counter blocks show fewer instructions and cycles. The fused fill
appears in all ON cycle profiles, with 75/93/94 samples, and never in OFF.
Crossbeam epoch-pin samples decrease in each block, but sample counts are not
exclusive wall time or a measure of a removable barrier. Cache misses are mixed;
this does not establish a cache-hit improvement. These observations support
continuing qualification, not a precise attribution of the complete saving.

Counters have at least 99.9% running coverage. Cycle captures have 11997–12233
samples each, validated executable/PID/TID/CPU ownership and no lost samples.
Per-arm instrumented/plain median latency ratios are within 1.004. The coarse
1.05 observer guard is not statistical equivalence. No capture was rerun.
Private profile root: `/home/ubuntu/src/zopro-perf-work/fused-preparation-profile182`.

## Release qualification and opt-in adoption

The retained candidate subsequently passes all **33/33** declared local checks:
primary32 latency improvement, all three primary32 backing medians, all-context
latency/CPU nonregression and whole-process RSS limits. The exact runtime source
is now integrated as an **explicit opt-in**, `ZOPRO_BENCH_FUSED_PREPARATION=1`.
Without that setting the previous path remains active. No default-on performance
claim or cosmetic rebuild is substituted for the qualified binary.

Thirty paired processes per context each run 17 timed fresh requests after three
primes. The paired primary32 median ratio is 0.967386 with marginal bounds
[0.954932, 0.978215]; CPU ratio is 0.967004 [0.948682, 0.976751]. Bounds use
one-based ranks 10 and 21 of 30: nominal 95.7226% median coverage under independent
exchangeable blocks, not a joint confidence guarantee. The three backing medians
are 0.937845, 0.991761 and 0.973244. Their 1.0575 max/min spread triggers the
prospective diagnostic flag, consistent with the retained reference-backing
variation in the screen. Do not interpret the full 3.26% paired gain as the
isolated fusion effect; same-binary ON/OFF evidence supports the smaller gain.

All 623 fresh exports independently verify: 23 preflight and 600 main processes,
12092 SDK calls including primes. All 99 generic no-IFMA/no-ASM core tests pass;
fresh native tests additionally check fully dense and sparse indices through
65535 wires, four OS callers sharing 1/4/16/32-worker pools and 512 KiB stacks.
The original 131 native core tests and unchanged arithmetic/MSM qualification
remain applicable. Configured scratch-denial preflights verify, but do not test
physical OOM, swap, phones or large forester circuits.

Fresh local comparison follows. Times are medians of process means; RSS is the
largest lifetime peak. Supplied M5 timings come from different hardware.

| Shape | Previous REF ms | Opt-in CPU ms | Peak RSS MiB | M5 CPU / speedup | M5 Metal / speedup |
|---|---:|---:|---:|---:|---:|
| 1x2 | 7.901307 | 7.671398 | 93.25 | 37.0 / 4.823110x | 24.0 / 3.128503x |
| 2x2 | 11.044044 | 10.695904 | 238.75 | 59.0 / 5.516129x | 30.0 / 2.804811x |
| 4x2 | 15.847478 | 15.824055 | 281.25 | 109.0 / 6.888246x | 48.0 / 3.033356x |
| 8x2 | 24.057822 | 24.069715 | 476.75 | 189.0 / 7.852190x | 74.0 / 3.074402x |
| 12x2 | 34.004063 | 33.935862 | 675.5 | 284.0 / 8.368727x | 101.0 / 2.976202x |
| 16x2 | 39.910741 | 39.865144 | 773.75 | 335.0 / 8.403330x | 124.0 / 3.110486x |
| 24x2 | 66.277972 | 66.178551 | 1210.25 | 528.0 / 7.978415x | 176.0 / 2.659471x |
| merge-24x1 | 65.532595 | 65.549681 | 1107.25 | 507.0 / 7.734591x | 175.0 / 2.669730x |

All eight CPU10x targets remain unmet. Merge54 has no supplied M5 reference;
its complete timings and memory checks are retained in assessment.json.
Canonical ledger is now 56502, SHA-256
`2721a5c49351ae5dea2fdf91c3889d99a3f48619ee22fc520b5e36adfdeb3b23`.
Private qualification root: `/home/ubuntu/src/zopro-perf-work/fused-preparation-all182`.
The separate result reader reproduces all 33 decisions; this is a same-agent
cross-check. C7a qualification/comparison remains subsequent work. No push.
