# Skipping inactive point runs in small G2 queries on C7a

The [Ryzen trial](../2026-11-20-g2-active-runs-ryzen855/README.md) reuses point runs in G2 window production. The identities mode checks the original point slices once. The component mode also filters zero GLV component magnitudes. Both preserve point order, signed digit extraction and the indexed top window.

Zopro uses 80 main and 16 frontend cores on 1×2 and 64 main with 32 frontend cores on 2×2. Prepared Aeglos uses all 96 physical cores. The two primary inputs have six matched blocks per shape and the two additional 1×2 inputs have four. Each process primes three proofs and times thirteen. All 168 exports independently verified, including the zero optional workspace fallback. The temporary instance and storage were removed.

- 1×2 `identities` took 8.058 ms on the original and 8.026 ms on the held-out input.
- 2×2 `identities` took 12.413 ms on the original and 12.506 ms on the held-out input.

No selected-run profile improved both controls consistently across all tested inputs.

| Shape | Input | Profile | Proof ms | Paired vs ordinary | Paired vs release | Paired vs Aeglos | Peak MiB | Blocks within target |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 7.900 | +0.62% (1/6) | +0.00% (0/6) | -2.32% (6/6) | 42.3 | 0/6 |
| 1×2 | catalog | ordinary | 7.876 | +0.00% (0/6) | -0.62% (5/6) | -2.23% (6/6) | 43.1 | 0/6 |
| 1×2 | catalog | identities | 8.058 | +2.35% (0/6) | +1.99% (0/6) | -0.82% (4/6) | 42.9 | 0/6 |
| 1×2 | catalog | components | 8.089 | +2.24% (0/6) | +1.78% (0/6) | +0.05% (3/6) | 43.7 | 0/6 |
| 1×2 | catalog | aeglos | 8.109 | +2.29% (0/6) | +2.38% (0/6) | +0.00% (0/6) | 454.0 | 0/6 |
| 1×2 | heldout | release | 7.858 | -0.56% (4/6) | +0.00% (0/6) | -5.17% (6/6) | 41.4 | 0/6 |
| 1×2 | heldout | ordinary | 7.900 | +0.00% (0/6) | +0.56% (2/6) | -4.50% (6/6) | 43.8 | 0/6 |
| 1×2 | heldout | identities | 8.026 | +1.62% (0/6) | +1.95% (0/6) | -3.05% (6/6) | 39.9 | 0/6 |
| 1×2 | heldout | components | 8.011 | +1.49% (0/6) | +1.94% (0/6) | -3.19% (6/6) | 40.0 | 0/6 |
| 1×2 | heldout | aeglos | 8.281 | +4.72% (0/6) | +5.46% (0/6) | +0.00% (0/6) | 454.3 | 0/6 |
| 1×2 | v25 | release | 7.893 | -0.17% (2/4) | +0.00% (0/4) | -3.97% (4/4) | 41.5 | 0/4 |
| 1×2 | v25 | ordinary | 7.902 | +0.00% (0/4) | +0.17% (2/4) | -4.33% (4/4) | 39.3 | 0/4 |
| 1×2 | v25 | identities | 8.065 | +2.14% (1/4) | +2.32% (0/4) | -2.28% (4/4) | 40.0 | 0/4 |
| 1×2 | v25 | components | 8.145 | +2.62% (0/4) | +3.02% (0/4) | -0.90% (4/4) | 40.1 | 0/4 |
| 1×2 | v25 | aeglos | 8.219 | +4.53% (0/4) | +4.14% (0/4) | +0.00% (0/4) | 466.0 | 0/4 |
| 1×2 | v26 | release | 7.836 | -0.11% (2/4) | +0.00% (0/4) | -4.63% (4/4) | 39.4 | 0/4 |
| 1×2 | v26 | ordinary | 7.854 | +0.00% (0/4) | +0.11% (2/4) | -4.53% (4/4) | 42.9 | 0/4 |
| 1×2 | v26 | identities | 8.074 | +2.87% (0/4) | +2.84% (0/4) | -2.21% (3/4) | 42.3 | 0/4 |
| 1×2 | v26 | components | 8.041 | +2.29% (0/4) | +2.28% (0/4) | -2.46% (3/4) | 39.9 | 0/4 |
| 1×2 | v26 | aeglos | 8.216 | +4.75% (0/4) | +4.86% (0/4) | +0.00% (0/4) | 457.4 | 0/4 |
| 2×2 | catalog | release | 12.160 | -0.01% (3/6) | +0.00% (0/6) | +2.41% (1/6) | 75.2 | 0/6 |
| 2×2 | catalog | ordinary | 12.158 | +0.00% (0/6) | +0.01% (3/6) | +3.05% (1/6) | 72.9 | 0/6 |
| 2×2 | catalog | identities | 12.413 | +2.54% (0/6) | +2.95% (0/6) | +5.21% (0/6) | 78.2 | 0/6 |
| 2×2 | catalog | components | 12.487 | +2.82% (0/6) | +3.09% (0/6) | +6.10% (0/6) | 73.7 | 0/6 |
| 2×2 | catalog | aeglos | 11.798 | -2.96% (5/6) | -2.35% (5/6) | +0.00% (0/6) | 807.1 | 0/6 |
| 2×2 | heldout | release | 12.190 | -0.25% (3/6) | +0.00% (0/6) | +4.88% (0/6) | 73.0 | 0/6 |
| 2×2 | heldout | ordinary | 12.121 | +0.00% (0/6) | +0.25% (3/6) | +4.52% (0/6) | 76.7 | 0/6 |
| 2×2 | heldout | identities | 12.506 | +2.88% (1/6) | +2.48% (0/6) | +6.45% (0/6) | 78.2 | 0/6 |
| 2×2 | heldout | components | 12.569 | +3.28% (0/6) | +3.11% (0/6) | +7.27% (0/6) | 74.5 | 0/6 |
| 2×2 | heldout | aeglos | 11.722 | -4.32% (6/6) | -4.65% (6/6) | +0.00% (0/6) | 797.5 | 0/6 |

## Run geometry

| Shape | Input | Partitions | Runs | Entries skipped / total |
|---|---|---:|---:|---:|
| 1×2 | catalog | 4 | 637 | 5,921 / 60,178 |
| 1×2 | heldout | 4 | 671 | 5,700 / 60,178 |
| 2×2 | catalog | 2 | 10 | 9 / 54,436 |
| 2×2 | heldout | 2 | 9 | 11 / 54,534 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [CPU topology](artifacts/topology.txt) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**5/8** original requests reach 10× M5 CPU on the recorded profiles.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 854 seed4-emit | 14.726 | 109 / 48 | 7.40× / 3.26× | +3.826 |
| 8×2 | 852 SDK seeded tasks and B2 vector conversion | 18.849 | 189 / 74 | 10.03× / 3.93× | -0.051 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
