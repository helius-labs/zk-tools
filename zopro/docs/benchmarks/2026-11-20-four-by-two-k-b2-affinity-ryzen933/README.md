# K and B2 worker placement on Ryzen 4×2 proofs

The frozen 4×2 proof took **13.691 / 13.643 ms** at the existing 24+8 worker placement, in original and held-out order. The physically disjoint 16+8 placement took **17.297 / 17.276 ms**. Both full-proof controls on the existing 24+8 placement won every matched block against each alternative. The 13.691 ms original request is 7.96× the supplied M5 CPU and 3.51× the Metal result. The C7a score remains [5 of 8 at 10× the M5 CPU reference](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings). AWS SSO expired before the run.

The extra saving from joint replay was **1.179 / 1.208 ms** at 24+8 and **-0.098 / -0.157 ms** at physically disjoint 16+8. At 16+8 with eight shared cores it was also close to zero at **0.014 / 0.060 ms**. The joint saving depends on the worker layout and which query is last to finish. K still took longer with B2 active when the pools had separate physical cores. In that layout, B2 ran long enough that replaying K alone saved effectively no whole-proof time. Changing placement also changes the number of physical cores available to the main pool and the L3 groups it uses.

All four placements use the same Ryzen 9950X, two pinned requests, and the same frontend CPU IDs. The main pool uses both hardware threads of CCD0 and either one thread on eight, four or zero CCD1 cores for 24-compact, 20-mixed and 16-disjoint. At 16-shared, it uses one hardware thread on each of the 16 physical cores. The resulting counts of shared physical cores appear below. A separate traced panel verified that K ran on eight frontend workers and H ran on the configured main pool. The two full-proof controls executed every query. The study uses the [exact 916 observer binary and replay checks](../2026-11-20-four-by-two-query-factorial-ryzen916/README.md).

## Full proof latency

Each case has eight counterbalanced blocks, with three primes and thirteen timed proofs per process. Latency is the median of process means. A new placement needs a lower paired median and six wins on each input for each of the two controls against 24-compact. Peak RSS covers the full frozen process. CPU-ms includes userspace and system time per timed proof across the process.

| Input | Placement | Frozen ms | Ordinary ms | Frozen vs 24-compact | Ordinary vs 24-compact | Frozen CPU-ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| catalog | 24-compact | 13.691 | 13.791 | +0.00% (0/8) | +0.00% (0/8) | 294.6 | 205.0 |
| catalog | 20-mixed | 15.268 | 15.335 | +12.08% (0/8) | +11.22% (0/8) | 273.3 | 196.5 |
| catalog | 16-disjoint | 17.297 | 17.257 | +26.88% (0/8) | +25.08% (0/8) | 265.7 | 197.2 |
| catalog | 16-shared | 14.853 | 14.730 | +8.29% (0/8) | +6.75% (0/8) | 235.6 | 189.0 |
| heldout | 24-compact | 13.643 | 13.703 | +0.00% (0/8) | +0.00% (0/8) | 294.5 | 201.5 |
| heldout | 20-mixed | 15.307 | 15.330 | +11.79% (0/8) | +12.03% (0/8) | 274.9 | 203.0 |
| heldout | 16-disjoint | 17.276 | 17.359 | +26.63% (0/8) | +26.71% (0/8) | 265.6 | 197.0 |
| heldout | 16-shared | 14.877 | 14.775 | +8.92% (0/8) | +8.21% (0/8) | 235.4 | 183.8 |

## K and B2 replay

Each replay checks all input coefficients against the original primed query. Relative to all-active, the joint saving minus the two individual savings is the interaction. The two observer calibrations per case compare ordinary with frozen and all-active with ordinary, using a 2% maximum paired median difference. **8/8 cases** passed both checks. An asterisk marks a descriptive comparison.

| Input | Placement | Shared cores | All active ms | K saved ms | B2 saved ms | Joint saved ms | Additional joint saving ms |
|---|---|---:|---:|---:|---:|---:|---:|
| catalog | 24-compact | 8 | 13.716 | +0.593 | +2.087 | +3.885 | +1.179 (8/8) |
| catalog | 20-mixed | 4 | 15.324 | +0.382 | +3.438 | +3.997 | +0.213 (7/8) |
| catalog | 16-disjoint | 0 | 17.301 | -0.024 | +4.652 | +4.642 | -0.098 (2/8) |
| catalog | 16-shared | 8 | 14.731 | +0.755 | +3.537 | +4.242 | +0.014 (5/8) |
| heldout | 24-compact | 8 | 13.726 | +0.585 | +2.245 | +4.041 | +1.208 (8/8) |
| heldout | 20-mixed | 4 | 15.388 | +0.349 | +3.504 | +4.123 | +0.256 (5/8) |
| heldout | 16-disjoint | 0 | 17.287 | +0.002 | +4.690 | +4.530 | -0.157 (1/8) |
| heldout | 16-shared | 8 | 14.808 | +0.759 | +3.445 | +4.274 | +0.060 (5/8) |

A separate three-block traced panel clocks query elapsed time with the other query active and then replayed. Spans include dispatch and waits. The timing and traced runs produced 480 exported proofs, all independently verified with Gnark. Replay timing serves as a diagnostic and is excluded from the M5 score.

| Input | Placement | K span with B2 active → replayed, ms | B2 span with K active → replayed, ms |
|---|---|---:|---:|
| catalog | 24-compact | 8.308 → 6.710 | 8.784 → 8.041 |
| catalog | 20-mixed | 8.400 → 6.599 | 9.795 → 9.413 |
| catalog | 16-disjoint | 9.394 → 7.314 | 11.794 → 12.169 |
| catalog | 16-shared | 8.874 → 6.728 | 9.944 → 9.347 |
| heldout | 24-compact | 8.152 → 6.796 | 8.660 → 8.137 |
| heldout | 20-mixed | 8.514 → 6.592 | 9.907 → 9.397 |
| heldout | 16-disjoint | 9.444 → 6.838 | 12.119 → 11.984 |
| heldout | 16-shared | 8.962 → 6.621 | 10.010 → 9.324 |

[Protocol](artifacts/protocol.json) · [CPU topology](artifacts/topology.json) · [Summary](artifacts/summary.json) · [Timing and clock samples](artifacts/rows.json.gz) · [Source pins](artifacts/pins.json) · [Proof verification](artifacts/verification.json)
