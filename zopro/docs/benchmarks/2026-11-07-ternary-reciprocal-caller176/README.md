# Reciprocal ternary codec: complete-query comparison

The reciprocal codec improves the previous ternary implementation in all six matched contexts, but the resulting query still loses to the original algorithm in every threaded context. The candidate fails its declared gate and is closed without adoption. Production source, accepted SDK, and full-proof performance remain unchanged.

On the primary 32-worker case, the median process mean is 3.266 ms versus 3.323 ms for the old ternary codec and 3.044 ms for the fresh original-algorithm control. The paired median is 1.8% faster than the old ternary implementation and 7.5% slower than the fresh original algorithm. These are complete G2 MSM query times, not 1×2 proof times.

## Method and identities

The [preceding arithmetic and code-generation screen](../2026-11-07-ternary-reciprocal-codec176/README.md) qualifies bounded reciprocal digit extraction and verifies removal of wide division helpers. This trial changes only that codec: 177 of 178 source-snapshot files and the complete driver match the previous qualified ternary implementation. Radix, windows, buckets, padding, queues, batching, grouping, Phi, scalar storage, thread placement, and 512 KiB worker stacks remain fixed. The original algorithm does not call the changed codec.

| Arms | Consumer SHA-256 | Selectors |
|---|---|---|
| OLD_BASE / OLD_TERNARY | `b3cb2d26210e243a7b2a014771da3d5541af800fd9fe1c92ad953f5c0e6a31c6` | B / T |
| FRESH_BASE / RECIPROCAL | `28bce6f68ae92bf79acfc22161f9a59c17c6aaf34e432dd389cf26d2cceeb35a` | B / T |

The actual consumers use O3, native CPU targeting, fat LTO, and one codegen unit. Both drivers include scalar reset, conversion, grouping, allocation, pool installation, queue processing, and normalization in the query timer. The fixed protocol retains both original fixtures and their existing independent full Arkworks MSM oracles, at 1/16/32 workers pinned to CPU 2, CPUs 0–15, or CPUs 0–31.

All 24 correctness preflights pass. The latency run contains exactly 31 blocks × 6 cases × 4 arms = 744 processes, with three priming calls, seventeen observed calls, and one postcheck per process. Every result is checked against the existing full-MSM oracle; point immutability, scalar reset, selectors, worker identities, and affinity are checked. Preflight clocks are excluded from performance decisions. Case order rotates by block modulo six and reverses on odd blocks; arm order rotates by block plus original case index modulo four and reverses on odd parity. There are no exclusions, extensions, rerolls, or selected worker subsets.

Each timing unit is process `elapsed_ns / 17`. Paired intervals use sorted ranks 10 and 22 of 31 ratios, with rank 16 as median: nominal 97.055% marginal sign coverage conditional on exchangeable blocks, not familywise coverage or a stationarity guarantee. A ratio below one favors RECIPROCAL. OLD_TERNARY is a mechanism comparator; beating it does not qualify adoption.

## Query results

Times are medians of process means, in milliseconds.

| Case | OLD_BASE | FRESH_BASE | OLD_TERNARY | RECIPROCAL |
|---|---:|---:|---:|---:|
| 1×2, 1 worker | 37.510762 | 37.158000 | 34.851781 | 33.982889 |
| 1×2, 16 workers | 3.477235 | 3.493659 | 3.924542 | 3.804384 |
| 1×2, 32 workers | 3.076908 | 3.044235 | 3.323373 | 3.265888 |
| 2×2, 1 worker | 39.916312 | 39.399544 | 37.447575 | 36.393470 |
| 2×2, 16 workers | 3.928535 | 3.962705 | 4.369189 | 4.263827 |
| 2×2, 32 workers | 3.574853 | 3.531733 | 3.827921 | 3.759354 |

| Case | RECIPROCAL / FRESH_BASE, median [interval] | Wins / 31 | RECIPROCAL / OLD_TERNARY, median [interval] | Wins / 31 |
|---|---:|---:|---:|---:|
| 1×2, 1 worker | 0.915352 [0.913986, 0.915872] | 31 | 0.975599 [0.973695, 0.977099] | 31 |
| 1×2, 16 workers | 1.087875 [1.073797, 1.094364] | 1 | 0.965768 [0.960325, 0.972766] | 30 |
| 1×2, 32 workers | 1.075442 [1.066105, 1.085046] | 0 | 0.982409 [0.976937, 0.989279] | 27 |
| 2×2, 1 worker | 0.923343 [0.922819, 0.923937] | 31 | 0.972083 [0.970810, 0.972915] | 31 |
| 2×2, 16 workers | 1.078542 [1.070248, 1.086330] | 0 | 0.977256 [0.973683, 0.983785] | 30 |
| 2×2, 32 workers | 1.067583 [1.050300, 1.089384] | 0 | 0.981399 [0.973223, 0.996472] | 25 |

The equal-log two-population 32-worker aggregate is 1.070615 [1.065750, 1.081140] against FRESH_BASE and 1.061363 [1.050128, 1.065199] against OLD_BASE, with zero wins in each comparison.

The fixed gate has 13 failures: eight threaded nonregression checks against the two base controls, four primary/aggregate gain checks, and one calibration check. The 2×2/32-worker FRESH_BASE/OLD_BASE interval is [0.968020, 1.001635], outside the declared [1/1.03, 1.03] range at its lower end. Other calibrations pass. This limits old-versus-fresh interpretation and prevents promotion; it does not erase the same-binary threaded regressions. No threshold was widened. Single-worker gains are retained but do not justify post hoc subset adoption.

## CPU diagnostics and memory observations

After all latency recording, the fixed diagnostic run executes one 201-query stat panel and one 251-query cycles profile per arm, plus priming and postcheck calls. All 28 diagnostic workflow commands pass. Counter running percentages are 100%. The four profiles contain 5,016 / 4,973 / 4,697 / 4,617 samples for OLD_BASE / FRESH_BASE / OLD_TERNARY / RECIPROCAL, with 0.917% / 1.227% / 0.809% / 0.801% unknown samples and zero lost events. Period, event, FIFO gating, PID/TID ownership, worker affinity, and executable mapping checks pass. Each new recording is decoded once; the qualified lost-event reader is reused.

| Counter ratio | RECIPROCAL / FRESH_BASE | RECIPROCAL / OLD_TERNARY |
|---|---:|---:|
| User cycles | 0.930350 | 0.981943 |
| Instructions | 0.923148 | 1.008970 |
| Cache misses | 0.942105 | 0.949936 |
| Branch misses | 0.806813 | 0.981834 |

The codec improves the old ternary query while the one-shot panel retires slightly more instructions. Against the original algorithm, it still uses fewer aggregate cycles despite worse threaded wall time. Helper removal therefore addresses some codec cost but does not explain or remove the remaining parallel completion gap. These panels do not establish exclusive critical paths, per-query tails, useful CPU utilization, DRAM bandwidth, or page-fault service time. Flat symbol bins include inlined work and cannot supply those causal conclusions.

Maximum observed process RSS, MiB, across each arm's 31 latency processes:

| Case | OLD_BASE | FRESH_BASE | OLD_TERNARY | RECIPROCAL |
|---|---:|---:|---:|---:|
| 1×2, 1 worker | 15.25 | 15.25 | 15.25 | 15.25 |
| 1×2, 16 workers | 27.75 | 27.75 | 32.00 | 32.75 |
| 1×2, 32 workers | 32.25 | 32.50 | 34.75 | 37.25 |
| 2×2, 1 worker | 27.00 | 27.00 | 27.00 | 27.00 |
| 2×2, 16 workers | 43.25 | 43.00 | 47.75 | 47.00 |
| 2×2, 32 workers | 53.75 | 60.50 | 57.25 | 58.00 |

RSS includes fixture loading and priming. It is not full-prover peak memory, minimum required RAM, or a rigorous live-workspace bound. The codec adds no heap allocation or point cache, but the inherited ternary geometry still has larger bucket storage and is not qualified for production resource admission, phones, pressure, swap, or very large circuits.

## Decision and accepted proof status

Close this fixed reciprocal-ternary candidate without SDK integration or adoption. Do not rerun it unchanged or tune window/worker/layout parameters to rescue this failed gate. The positive codec result remains evidence; it is insufficient to replace the original algorithm. A useful next question is where per-query parallel completion is delayed, which aggregate worker profiles have not resolved.

Independent review reconstructs all rotation schedules, means, ranks, ratios, RSS maxima, and 13 gate failures from raw receipts. It verifies 797 command receipts, 776 complete-query processes, 17,584 queries, 19,303 profile samples, 4,578 source-pin occurrences, and recovery of all 176 unchanged production files from the new snapshot. It is an offline evidence review, not a new curve oracle or cryptographic verifier. No successful test, build, capture, oracle, or decoder was repeated for bookkeeping. The canonical verified-proof ledger remains 32,196.

The accepted SDK's inherited local proof times and supplied M5 references remain:

| Shape | Local CPU ms | M5 CPU ms | M5 Metal ms | CPU 10× limit ms |
|---|---:|---:|---:|---:|
| 1×2 | 8.041470 | 37 | 24 | 3.7 |
| 2×2 | 11.237839 | 59 | 30 | 5.9 |
| 4×2 | 16.143230 | 109 | 48 | 10.9 |
| 8×2 | 24.397442 | 189 | 74 | 18.9 |
| 12×2 | 33.736405 | 284 | 101 | 28.4 |
| 16×2 | 40.082192 | 335 | 124 | 33.5 |
| 24×2 | 65.765454 | 528 | 176 | 52.8 |
| Merge 24 | 65.350546 | 507 | 175 | 50.7 |
| Merge 54 | 161.495859 | — | — | — |

All-eight CPU 10× status remains 0/8. These are different machines; the comparison does not establish equal hardware efficiency. This trial supplies no fresh SDK proof latency, AWS/Aeglos comparison, account inventory, or cloud resource. No new AWS resource was created.

## Evidence

[Evidence archive](artifacts/evidence.tar.gz), [member hashes](artifacts/evidence-members.json), and [archive checksum](artifacts/SHA256SUMS) contain the frozen design, Rust helpers, plans, raw text receipts, counter CSVs, profile text, analyses, and independent review. Binaries, libraries, raw recordings, and private proving inputs are excluded. The preceding screen provides source and native-build provenance. Original absolute paths identify the working environment and must be reconstructed before executing plans elsewhere.
