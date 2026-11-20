# Early grouped B2 on C7a

The existing layout remains selected for both shapes. The 8×2 early profile with a 20-core solver reached 18.721 ms on catalog and 19.103 ms on held-out. It won six and five blocks, respectively, against the archived profile. The held-out median remains above the 18.9 ms M5 CPU threshold.

The Ryzen experiment ran grouped B2 on the frontend pool before the quotient. This follow-up tests the same build on one c7a.24xlarge against the [previous matched C7a profiles](../2026-11-20-grouped-sdk-widths-c7a1000/README.md), same-binary ordinary and late controls, and prepared Aeglos. The main pool computes the quotient and G1. All updated 8×2 arms include Euler leaf fusion and packed final verification.

| Shape | Request | Selected profile | Zopro ms | Archived ms | Paired vs archive | M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | Archived 72+24, solver 16 | 14.040 | 14.040 | +0.00% (0/8) | 7.76× / 3.42× |
| 4×2 | heldout | Archived 72+24, solver 16 | 14.032 | 14.032 | +0.00% (0/8) | 7.77× / 3.42× |
| 8×2 | catalog | Archived 72+24, solver 16 | 18.910 | 18.910 | +0.00% (0/8) | 9.99× / 3.91× |
| 8×2 | heldout | Archived 72+24, solver 16 | 19.151 | 19.151 | +0.00% (0/8) | 9.87× / 3.86× |

A profile qualifies when it has lower paired medians and wins at least six of eight blocks on both requests against the archive. Among qualifying profiles, selection uses the smallest worse-request median. The archived profile remains selected when updated profiles fall short.

| Shape | Request | Profile | Proof ms | Paired vs archive | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | Pinned | 14.106 | +0.16% (2/8) | -22.07% (8/8) | 210.1 |
| 4×2 | catalog | Archived 72+24, solver 16 | 14.040 | +0.00% (0/8) | -22.84% (8/8) | 211.0 |
| 4×2 | catalog | Ordinary 72+24, solver 16 | 14.054 | +0.01% (4/8) | -22.82% (8/8) | 209.4 |
| 4×2 | catalog | Early 72+24, solver 16 | 14.294 | +2.31% (0/8) | -21.30% (8/8) | 204.2 |
| 4×2 | catalog | Late 72+24, solver 16 | 15.549 | +10.69% (0/8) | -14.34% (8/8) | 205.0 |
| 4×2 | catalog | Ordinary 64+32, solver 16 | 14.070 | +0.10% (3/8) | -22.30% (8/8) | 215.4 |
| 4×2 | catalog | Early 64+32, solver 16 | 14.261 | +1.71% (1/8) | -21.26% (8/8) | 201.3 |
| 4×2 | catalog | Early 72+24, solver 20 | 14.481 | +3.11% (0/8) | -19.90% (8/8) | 196.0 |
| 4×2 | catalog | Prepared Aeglos | 18.102 | +29.60% (0/8) | +0.00% (0/8) | 1350.0 |
| 4×2 | heldout | Pinned | 14.204 | +1.80% (1/8) | -21.67% (8/8) | 220.0 |
| 4×2 | heldout | Archived 72+24, solver 16 | 14.032 | +0.00% (0/8) | -22.48% (8/8) | 210.9 |
| 4×2 | heldout | Ordinary 72+24, solver 16 | 14.011 | +0.06% (4/8) | -22.55% (8/8) | 223.0 |
| 4×2 | heldout | Early 72+24, solver 16 | 14.441 | +2.76% (0/8) | -20.46% (8/8) | 199.8 |
| 4×2 | heldout | Late 72+24, solver 16 | 15.474 | +10.21% (0/8) | -14.75% (8/8) | 200.4 |
| 4×2 | heldout | Ordinary 64+32, solver 16 | 14.072 | +0.44% (2/8) | -22.34% (8/8) | 213.9 |
| 4×2 | heldout | Early 64+32, solver 16 | 14.177 | +1.36% (0/8) | -21.61% (8/8) | 200.5 |
| 4×2 | heldout | Early 72+24, solver 20 | 14.546 | +3.99% (0/8) | -19.46% (8/8) | 202.7 |
| 4×2 | heldout | Prepared Aeglos | 18.167 | +29.01% (0/8) | +0.00% (0/8) | 1349.2 |
| 8×2 | catalog | Pinned | 19.266 | +1.93% (2/8) | -36.32% (8/8) | 373.6 |
| 8×2 | catalog | Archived 72+24, solver 16 | 18.910 | +0.00% (0/8) | -37.10% (8/8) | 371.0 |
| 8×2 | catalog | Ordinary 72+24, solver 16 | 18.839 | -0.25% (5/8) | -37.50% (8/8) | 358.0 |
| 8×2 | catalog | Early 72+24, solver 16 | 19.176 | +1.48% (3/8) | -36.41% (8/8) | 338.8 |
| 8×2 | catalog | Late 72+24, solver 16 | 20.230 | +6.20% (0/8) | -33.19% (8/8) | 350.1 |
| 8×2 | catalog | Ordinary 64+32, solver 16 | 19.121 | +1.21% (3/8) | -36.91% (8/8) | 363.7 |
| 8×2 | catalog | Early 64+32, solver 16 | 19.249 | +1.46% (2/8) | -36.46% (8/8) | 353.8 |
| 8×2 | catalog | Early 72+24, solver 20 | 18.721 | -1.28% (6/8) | -38.02% (8/8) | 345.6 |
| 8×2 | catalog | Prepared Aeglos | 30.182 | +59.00% (0/8) | +0.00% (0/8) | 2462.5 |
| 8×2 | heldout | Pinned | 19.395 | +0.91% (3/8) | -37.00% (8/8) | 397.9 |
| 8×2 | heldout | Archived 72+24, solver 16 | 19.151 | +0.00% (0/8) | -37.28% (8/8) | 365.8 |
| 8×2 | heldout | Ordinary 72+24, solver 16 | 19.267 | +0.34% (3/8) | -37.03% (8/8) | 359.8 |
| 8×2 | heldout | Early 72+24, solver 16 | 19.323 | +0.74% (1/8) | -36.53% (8/8) | 347.2 |
| 8×2 | heldout | Late 72+24, solver 16 | 20.405 | +6.28% (0/8) | -33.28% (8/8) | 362.8 |
| 8×2 | heldout | Ordinary 64+32, solver 16 | 19.412 | +1.69% (3/8) | -36.37% (8/8) | 371.8 |
| 8×2 | heldout | Early 64+32, solver 16 | 19.468 | +1.84% (2/8) | -36.30% (8/8) | 344.7 |
| 8×2 | heldout | Early 72+24, solver 20 | 19.103 | -0.38% (5/8) | -37.42% (8/8) | 350.9 |
| 8×2 | heldout | Prepared Aeglos | 30.583 | +59.45% (0/8) | +0.00% (0/8) | 2484.2 |

Eight timing blocks prime three proofs and time seventeen. The independent diagnostics check both scoped dispatches and zero-budget fallback with two timed proofs per case. All 304 process exports passed independent verification. The temporary AWS resources were removed.

[Full results](artifacts/summary.json) · [Samples and diagnostics](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)
