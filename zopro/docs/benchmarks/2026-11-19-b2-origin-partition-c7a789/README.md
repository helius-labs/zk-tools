# Separate origin and residual scalar grouping on C7a

This trial replays the [Ryzen origin partition](../2026-11-19-b2-origin-partition-ryzen788/README.md) on one c7a.24xlarge. The optional per-key plan stores coordinate masks and point indices. Each proof groups terms with matching Poseidon states after checking the full scalar and classifies the remaining terms separately.

Zopro uses 64 main and 32 frontend cores. The same trial binary measures ordinary grouping, the existing origin path, and the partition. The released SDK control enables origins on 8×2. The archived 8×2 trial is the original origin implementation. All Zopro 8×2 arms use serial scalar conversion. Prepared Aeglos uses all 96 cores and every measured Aeglos request hit its preparation cache.

Each timed process ran three priming requests and thirteen measured proofs. The comparison uses four matched process blocks on 4×2 and six on 8×2. All 236 exported proofs independently verified. Route checks confirmed that zero optional memory selected the ordinary fallback. Peak memory is the maximum whole-process RSS observed across the corresponding process blocks. The AWS instance, volume and temporary bucket were removed.

- 4×2 measured 15.419 and 15.527 ms on the original and held-out inputs. The existing origin path measured 15.406 and 15.353 ms. The scorecard retains the previously measured 15.257 ms profile.
- 8×2 measured 20.520 and 20.844 ms on the original and held-out inputs. The existing origin path measured 20.383 and 20.598 ms. The scorecard retains the previously measured 20.098 ms profile.

The partition remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs origins | Paired vs release | Paired vs ordinary | Paired vs archive | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 4×2 | catalog | aeglos | 18.312 | +18.89% (0/4) | +18.76% (0/4) | +19.46% (0/4) | — | +0.00% (0/4) | 1347.3 | 5.95× / 2.62× |
| 4×2 | catalog | ordinary | 15.367 | -0.26% (3/4) | -0.60% (3/4) | +0.00% (0/4) | — | -16.29% (4/4) | 206.5 | 7.09× / 3.12× |
| 4×2 | catalog | origins | 15.406 | +0.00% (0/4) | +0.18% (1/4) | +0.26% (1/4) | — | -15.88% (4/4) | 202.9 | 7.08× / 3.12× |
| 4×2 | catalog | release | 15.437 | -0.18% (3/4) | +0.00% (0/4) | +0.60% (1/4) | — | -15.80% (4/4) | 201.2 | 7.06× / 3.11× |
| 4×2 | catalog | split | 15.419 | +0.08% (2/4) | +0.58% (1/4) | +0.60% (1/4) | — | -15.65% (4/4) | 205.0 | 7.07× / 3.11× |
| 4×2 | heldout | aeglos | 18.347 | +19.35% (0/4) | +18.51% (0/4) | +18.22% (0/4) | — | +0.00% (0/4) | 1346.1 | — |
| 4×2 | heldout | ordinary | 15.514 | +1.11% (0/4) | +0.41% (1/4) | +0.00% (0/4) | — | -15.41% (4/4) | 197.4 | — |
| 4×2 | heldout | origins | 15.353 | +0.00% (0/4) | -0.85% (3/4) | -1.09% (4/4) | — | -16.20% (4/4) | 204.9 | — |
| 4×2 | heldout | release | 15.451 | +0.86% (1/4) | +0.00% (0/4) | -0.40% (3/4) | — | -15.59% (4/4) | 205.7 | — |
| 4×2 | heldout | split | 15.527 | +1.13% (0/4) | +0.35% (1/4) | +0.19% (1/4) | — | -15.23% (4/4) | 212.4 | — |
| 4×2 | v153 | aeglos | 18.341 | +19.56% (0/4) | +16.95% (0/4) | +18.43% (0/4) | — | +0.00% (0/4) | 1350.4 | — |
| 4×2 | v153 | ordinary | 15.522 | +1.14% (0/4) | -1.25% (3/4) | +0.00% (0/4) | — | -15.56% (4/4) | 196.8 | — |
| 4×2 | v153 | origins | 15.351 | +0.00% (0/4) | -2.19% (3/4) | -1.13% (4/4) | — | -16.36% (4/4) | 199.0 | — |
| 4×2 | v153 | release | 15.683 | +2.24% (1/4) | +0.00% (0/4) | +1.27% (1/4) | — | -14.49% (4/4) | 201.9 | — |
| 4×2 | v153 | split | 15.380 | +0.45% (1/4) | -2.22% (3/4) | -0.92% (4/4) | — | -16.10% (4/4) | 198.2 | — |
| 4×2 | v156 | aeglos | 18.209 | +18.10% (0/4) | +17.87% (0/4) | +18.60% (0/4) | — | +0.00% (0/4) | 1342.0 | — |
| 4×2 | v156 | ordinary | 15.434 | -0.52% (4/4) | -0.44% (2/4) | +0.00% (0/4) | — | -15.69% (4/4) | 202.0 | — |
| 4×2 | v156 | origins | 15.535 | +0.00% (0/4) | +0.21% (2/4) | +0.52% (0/4) | — | -15.32% (4/4) | 209.5 | — |
| 4×2 | v156 | release | 15.496 | -0.21% (2/4) | +0.00% (0/4) | +0.44% (2/4) | — | -15.16% (4/4) | 207.2 | — |
| 4×2 | v156 | split | 15.402 | -1.00% (4/4) | -0.30% (4/4) | -0.58% (3/4) | — | -16.17% (4/4) | 194.4 | — |
| 8×2 | catalog | aeglos | 30.937 | +52.26% (0/6) | +52.22% (0/6) | +50.61% (0/6) | +51.63% (0/6) | +0.00% (0/6) | 2452.2 | 6.11× / 2.39× |
| 8×2 | catalog | archive | 20.429 | +0.29% (3/6) | +0.59% (3/6) | +0.01% (3/6) | +0.00% (0/6) | -34.05% (6/6) | 409.3 | 9.25× / 3.62× |
| 8×2 | catalog | ordinary | 20.390 | +0.68% (2/6) | +0.88% (1/6) | +0.00% (0/6) | -0.01% (3/6) | -33.60% (6/6) | 415.4 | 9.27× / 3.63× |
| 8×2 | catalog | origins | 20.383 | +0.00% (0/6) | -0.20% (4/6) | -0.67% (4/6) | -0.29% (3/6) | -34.32% (6/6) | 409.3 | 9.27× / 3.63× |
| 8×2 | catalog | release | 20.339 | +0.20% (2/6) | +0.00% (0/6) | -0.87% (5/6) | -0.58% (3/6) | -34.31% (6/6) | 405.3 | 9.29× / 3.64× |
| 8×2 | catalog | split | 20.520 | +0.74% (0/6) | +1.09% (0/6) | +0.64% (2/6) | +0.43% (2/6) | -33.18% (6/6) | 388.8 | 9.21× / 3.61× |
| 8×2 | heldout | aeglos | 31.179 | +51.92% (0/6) | +51.73% (0/6) | +51.37% (0/6) | +51.13% (0/6) | +0.00% (0/6) | 2466.6 | — |
| 8×2 | heldout | archive | 20.606 | -0.14% (4/6) | -0.48% (3/6) | -0.43% (4/6) | +0.00% (0/6) | -33.83% (6/6) | 403.2 | — |
| 8×2 | heldout | ordinary | 20.666 | +0.02% (3/6) | -0.43% (4/6) | +0.00% (0/6) | +0.43% (2/6) | -33.93% (6/6) | 413.1 | — |
| 8×2 | heldout | origins | 20.598 | +0.00% (0/6) | -0.70% (4/6) | -0.02% (3/6) | +0.14% (2/6) | -34.18% (6/6) | 400.4 | — |
| 8×2 | heldout | release | 20.742 | +0.70% (2/6) | +0.00% (0/6) | +0.44% (2/6) | +0.49% (3/6) | -34.09% (6/6) | 413.0 | — |
| 8×2 | heldout | split | 20.844 | +1.01% (1/6) | +0.23% (2/6) | +0.81% (1/6) | +1.16% (1/6) | -33.50% (6/6) | 383.7 | — |
| 8×2 | v153 | aeglos | 31.293 | +52.97% (0/6) | +52.36% (0/6) | +50.89% (0/6) | +53.05% (0/6) | +0.00% (0/6) | 2482.1 | — |
| 8×2 | v153 | archive | 20.497 | -0.34% (3/6) | +0.24% (3/6) | -0.85% (5/6) | +0.00% (0/6) | -34.66% (6/6) | 426.4 | — |
| 8×2 | v153 | ordinary | 20.746 | +1.13% (1/6) | +1.31% (2/6) | +0.00% (0/6) | +0.86% (1/6) | -33.73% (6/6) | 406.6 | — |
| 8×2 | v153 | origins | 20.541 | +0.00% (0/6) | +0.18% (2/6) | -1.12% (5/6) | +0.34% (3/6) | -34.63% (6/6) | 401.7 | — |
| 8×2 | v153 | release | 20.503 | -0.18% (4/6) | +0.00% (0/6) | -1.29% (4/6) | -0.24% (3/6) | -34.37% (6/6) | 416.8 | — |
| 8×2 | v153 | split | 20.771 | +0.68% (3/6) | +1.32% (0/6) | -0.10% (4/6) | +0.72% (0/6) | -33.55% (6/6) | 390.6 | — |
| 8×2 | v156 | aeglos | 31.297 | +51.54% (0/6) | +51.03% (0/6) | +51.07% (0/6) | +51.04% (0/6) | +0.00% (0/6) | 2481.9 | — |
| 8×2 | v156 | archive | 20.621 | +0.68% (1/6) | +0.06% (2/6) | -0.40% (4/6) | +0.00% (0/6) | -33.79% (6/6) | 418.2 | — |
| 8×2 | v156 | ordinary | 20.668 | +0.57% (2/6) | -0.23% (3/6) | +0.00% (0/6) | +0.40% (2/6) | -33.80% (6/6) | 417.4 | — |
| 8×2 | v156 | origins | 20.509 | +0.00% (0/6) | -1.04% (4/6) | -0.56% (4/6) | -0.68% (5/6) | -34.01% (6/6) | 404.9 | — |
| 8×2 | v156 | release | 20.715 | +1.05% (2/6) | +0.00% (0/6) | +0.23% (3/6) | -0.06% (4/6) | -33.79% (6/6) | 409.9 | — |
| 8×2 | v156 | split | 20.855 | +1.83% (1/6) | +0.26% (1/6) | +1.52% (1/6) | +1.63% (1/6) | -33.07% (6/6) | 387.2 | — |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU on measured C7a profiles.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 737 dedicated G1 80+16 | 7.953 | 37 / 24 | 4.65× / 3.02× | +4.253 |
| 2×2 | 750 archived-band 72+24 | 12.358 | 59 / 30 | 4.77× / 2.43× | +6.458 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 781 recognized B2 origins replay | 20.098 | 189 / 74 | 9.40× / 3.68× | +1.198 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
