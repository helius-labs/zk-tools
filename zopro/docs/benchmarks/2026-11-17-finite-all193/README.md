# Finite-base SDK qualification

The initial small positive screen did not reproduce against accepted production.
The exact candidate passes correctness and resource qualification, but passes
30/33 performance checks. It remains a retained experiment; production stays on
fused-preparation182. No minimum gain threshold was imposed.

For primary1x2/16, the paired candidate/reference latency ratio is
**1.005618 [0.998778,1.009191]**. The point estimate is 0.562% slower and the
interval includes parity. Two of three backing medians also exceed1. These are
the three failed checks. This establishes neither a repeatable improvement nor
a statistically resolved slowdown. Primary32 is1.004272 [0.996165,1.010670],
also inconclusive. All ten CPU and all ten peak-RSS guards pass.

| Local Ryzen9950X context | REF ms | Candidate ms | Paired latency ratio | REF / candidate peak MiB |
|---|---:|---:|---:|---:|
| 1x2,16workers | 8.887604 | 8.939010 | 1.005618 | 75.00 / 77.00 |
| 1x2,32workers | 7.669544 | 7.722446 | 1.004272 | 99.75 / 94.50 |
| 2x2,32workers | 10.747497 | 10.791285 | 1.004956 | 235.50 / 240.25 |
| 4x2,32workers | 15.977488 | 16.070777 | 1.006093 | 289.25 / 291.75 |
| 8x2,32workers | 24.472422 | 24.420416 | 1.000382 | 478.00 / 476.50 |
| 12x2,32workers | 34.108254 | 33.906677 | 0.995725 | 659.00 / 672.25 |
| 16x2,32workers | 40.216724 | 40.027520 | 0.991880 | 784.50 / 772.50 |
| 24x2,32workers | 66.604402 | 66.353318 | 0.995711 | 1229.52 / 1260.50 |
| merge24x1,32workers | 66.001542 | 65.770536 | 0.997653 | 1108.25 / 1123.75 |
| merge54x1,32workers | 163.027579 | 162.937094 | 0.998002 | 1847.48 / 1855.17 |

Times are medians of30 process means. Paired ratios are medians of30 within-block
ratios and need not equal ratios of the displayed medians. The short screen used
process medians; this qualification prospectively used means. Different summaries
and captures must not be treated as a controlled explanation for their difference.
Full results and backing strata are in [assessment.json](artifacts/assessment.json).

The declared primary was16workers: latency upper<1 and each backing median<1.
Its three backing medians are1.013297,1.005325,0.998789. Other nine latency uppers
must be<=1.02; all ten CPU uppers<=1.02; each maximum RSS<=REF*1.05+1MiB.
All33 decisions were fixed before capture. Bounds use sorted paired ratios at
one-based ranks10/21: nominal95.7226% marginal median coverage assuming independent
exchangeable blocks, without a joint confidence claim. There was no sample
extension, dropped backing or successful-capture replay. A separate arithmetic
reader reproduced all33 decisions from the raw samples.

The retained [screen](../2026-11-17-finite-sdk191/README.md) showed small positive
primary medians and passed its progression criteria. The exact-build
[profile](../2026-11-17-finite-profile192/README.md) passed observer/PMU checks and
measured about0.20–0.22% fewer instructions, with mixed cycles and latency.
Those observations remain useful. They do not establish an accepted end-to-end
speedup. Some larger shapes improve here despite the finite route being inactive;
those differences cannot be credited to identity-check elimination.

The candidate certifies immutable B2 points at loading for eligible native keys,
then omits repeated identity checks only in ordinary raw G2 feeding. Grouped sums
and narrow top windows retain checks. It adds one Boolean per Key and three
global atomic counters; there is no point lookup table or new shared scratch pool.
Keys outside1024<=n<65536 skip certification. Both arms enable accepted fusion
and indexed batching; only the candidate enables `ZOPRO_BENCH_FINITE_G2=1`.
The measured candidate retains its counter overhead. Primary ON requests record
known/raw entries;2x2 records the grouped route; larger shapes record no finite
entries. Configured preparation-denial cases retain correct fallback behavior.

Qualification reused the exact191SDK and three independent backings per binary.
There were23 preflights followed by30 paired rounds in ten contexts:600 main
processes,17 timed requests plus3 primes each. Context order rotates and reverses,
arm order alternates, and each backing supplies ten pairs per context. All timing
preceded Gnark verification within each phase. No owned builds or analysis overlapped
capture. The fresh-request SDK boundary includes witness solving, proof randomness,
proving, SDK verification and serialization; loading, primes and export are excluded.

All623 exported proofs independently verify with Gnark, fixed image/public-input
bindings and global freshness checks. The SDK verifies all12092 requests including
primes. Across screen, profile and qualification this is724 exports and19426 calls.
Canonical exports advance56686->57309; final ledger SHA256:
`cec72464ff1de85ef6ee11cebf62f1ddd59fbc4fa41608fe52dc66e11cafca37`.

Release checks retain the original135 passing native core tests. A separate source
snapshot appends only a native test: four simultaneous OS callers share a checked
immutable certificate, each with private scalar buffers, across1/4/16/32worker
pools with512KiB worker stacks. All32 results match the retained independent
Arkworks oracle, and counters confirm32 raw finite calls. The unfiltered portable
no-IFMA/no-ASM suite passes103 tests, with4 explicit timing tests ignored.
All179 original source hashes and release receipts were rechecked. These tests
generated no new proof exports. Production's178 source hashes remain unchanged.
See [release-review.json](artifacts/release-review.json).

| Shape,32workers | Candidate ms | Supplied M5 CPU ms | CPU speedup | Supplied M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| 1x2 | 7.722446 | 37 | 4.79x | 24 | 3.11x |
| 2x2 | 10.791285 | 59 | 5.47x | 30 | 2.78x |
| 4x2 | 16.070777 | 109 | 6.78x | 48 | 2.99x |
| 8x2 | 24.420416 | 189 | 7.74x | 74 | 3.03x |
| 12x2 | 33.906677 | 284 | 8.38x | 101 | 2.98x |
| 16x2 | 40.027520 | 335 | 8.37x | 124 | 3.10x |
| 24x2 | 66.353318 | 528 | 7.96x | 176 | 2.65x |
| merge24x1 | 65.770536 | 507 | 7.71x | 175 | 2.66x |

These are descriptive cross-machine ratios against supplied references, not new
M5 measurements. Merge54 has no supplied M5 comparator. The all-eight CPU10x
goal remains unmet,0/8. RSS is measured whole-process peak, not minimum RAM.
There is no new physical ARM/mobile, huge-circuit, physical memory-pressure,
throughput or AWS claim. No AWS resources were created and nothing was pushed.

Exact candidate SDK SHA256:
`b655c4c68483d54e2343a0593818b49be05e140c949fe83cd6216d332ecb1897`.
Accepted REF:
`c285cbc88f783abac203c66db43ea86bda8207338b9fb209df7eb5d846126202`.
Private root: `/home/ubuntu/src/zopro-perf-work/finite-all193`.
Plans, readers, receipts, raw samples, release evidence and exported proofs are
retained in `artifacts/evidence.tar.gz`; integrity hashes are in
[SHA256SUMS](SHA256SUMS). The exact candidate source remains in191's pinned snapshot.
See [decision.json](artifacts/decision.json) for the recorded disposition.
