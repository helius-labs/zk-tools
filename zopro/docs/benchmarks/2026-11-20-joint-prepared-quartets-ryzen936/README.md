# Prepared point sums for K and B2 on 4×2

The joint mode took **13.345 / 13.331 ms** on the original and held-out Ryzen requests. Its paired change was **-2.35% (8/8) / -1.96% (8/8)** against ordinary in the same build and **-1.83% (8/8) / -1.46% (7/8)** against the frozen seeded SDK. The combined mode cleared the frozen and ordinary controls. It missed the paired-win requirement against B2 alone on the held-out request. B2 alone took **13.387 / 13.482 ms**, with frozen comparisons of **-1.38% (8/8) / -0.90% (6/8)**. The joint mode beat B2 alone by **-0.37% (8/8) / -1.22% (5/8)**. B2 alone is the smaller confirmation candidate, using 2.60 MiB of retained buffers. AWS SSO expired, so the [C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings) still shows five of eight shapes at 10× M5 CPU.

The trial reuses the [935 B2 plan](../2026-11-20-g2-prepared-quartets-ryzen935/README.md) and projects its 17,611 four-point groups onto the private wires of the pinned solver image. The K cache belongs to the exact folded private basis. Both caches compute sums from the loaded key and require the proof's entire current class to match all four stored indices. Other classes retain the ordinary reduction. Misses use the same per-worker point capacity bound as ordinary.

The route captures hit 17,611 groups per query on the original input and 16,897 per query on the held-out input. Combined retained buffers use **4.13 MiB**. In the joint route probes, K constructed its cache in 17.17 / 22.74 ms and B2 in 32.22 / 42.18 ms. These are per-query elapsed times from two first-use captures. Full scalar classification still precedes reuse. The test checked ordinary, K alone, B2 alone, both, zero budget, and an invalid K dimension with B2 still active. Gnark independently verified all 108 exports. The native correctness test covered both curves and mismatched classes.

Ryzen uses 24 main and eight frontend hardware threads. Each cell uses eight counterbalanced blocks with three primes and seventeen timed proofs. Latency is the median of process means. The separate trace processes are outside the timing panel. The exact earlier B2 trial is the prior-B2 reference. Ordinary calibrated at **+0.81% (2/8) / +0.84% (1/8)** against frozen. Modes clearing the two-control consistency check are **b2, both**. The within-block joint latency minus the latency predicted from the two individual changes is **-0.024 / -0.136 ms**.

| Input | Mode | Proof ms | Paired vs frozen | Paired vs ordinary | Paired vs K | Paired vs B2 | Paired vs prior B2 | CPU-ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| catalog | frozen | 13.590 | +0.00% (0/8) | -0.80% (6/8) | -0.53% (5/8) | +1.40% (0/8) | +0.04% (4/8) | 291.4 | 216.8 |
| catalog | prior-b2 | 13.590 | -0.04% (4/8) | -0.62% (8/8) | -0.65% (6/8) | +1.46% (0/8) | +0.00% (0/8) | 289.8 | 238.0 |
| catalog | ordinary | 13.690 | +0.81% (2/8) | +0.00% (0/8) | +0.16% (3/8) | +2.04% (0/8) | +0.62% (0/8) | 291.9 | 210.5 |
| catalog | k | 13.620 | +0.54% (3/8) | -0.16% (5/8) | +0.00% (0/8) | +1.98% (0/8) | +0.65% (2/8) | 288.9 | 215.0 |
| catalog | b2 | 13.387 | -1.38% (8/8) | -2.00% (8/8) | -1.94% (8/8) | +0.00% (0/8) | -1.44% (8/8) | 288.2 | 230.5 |
| catalog | both | 13.345 | -1.83% (8/8) | -2.35% (8/8) | -2.34% (8/8) | -0.37% (8/8) | -1.88% (8/8) | 283.9 | 215.0 |
| heldout | frozen | 13.558 | +0.00% (0/8) | -0.83% (7/8) | -0.12% (4/8) | +0.91% (2/8) | -0.28% (6/8) | 291.7 | 210.2 |
| heldout | prior-b2 | 13.625 | +0.28% (2/8) | -0.31% (4/8) | +0.12% (4/8) | +1.08% (0/8) | +0.00% (0/8) | 290.8 | 230.5 |
| heldout | ordinary | 13.638 | +0.84% (1/8) | +0.00% (0/8) | +0.38% (2/8) | +1.75% (1/8) | +0.31% (4/8) | 292.1 | 210.2 |
| heldout | k | 13.613 | +0.12% (4/8) | -0.38% (6/8) | +0.00% (0/8) | +1.14% (2/8) | -0.11% (4/8) | 290.7 | 219.0 |
| heldout | b2 | 13.482 | -0.90% (6/8) | -1.72% (7/8) | -1.12% (6/8) | +0.00% (0/8) | -1.07% (8/8) | 288.8 | 214.8 |
| heldout | both | 13.331 | -1.46% (7/8) | -1.96% (8/8) | -1.92% (7/8) | -1.22% (5/8) | -2.39% (7/8) | 284.9 | 215.8 |

[Reproducible patch](artifacts/source.patch) · [Projected K plan](artifacts/projected-k-quartets.bin.gz) · [Source plan pins](artifacts/plan.json) · [Protocol](artifacts/protocol.json) · [Timing and route data](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
