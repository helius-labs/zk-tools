# Seeded solver tasks on smaller proofs on C7a

The trial applies configurations from the [8×2 solver profile](../2026-11-20-seeded-b2-sdk-profile/README.md) to 1×2, 2×2 and 4×2. The packed mode uses four epochs, certified uninitialized buffers and dynamic claims. Seeded modes give claimants one ready job each before the shared queue. The emit modes also encode the stores of recognized Poseidon operations, with four or eight epochs. The candidate modes alter only solver settings.

Zopro uses 80 main and 16 frontend cores on 1×2 and 64 main with 32 frontend cores on the two larger shapes. Prepared Aeglos uses all 96 physical C7a cores. There are five matched blocks per input. Each process primes three proofs and times thirteen. All 210 exports independently verified. The temporary instance and storage were removed.

- 1×2 `packed4` took 8.139 ms on the original and 8.061 ms on the held-out input.
- 2×2 `packed4` took 12.222 ms on the original and 12.237 ms on the held-out input.
- 4×2 `seed4-emit` took 14.726 ms on the original and 14.782 ms on the held-out input.

The following solver profiles improved both controls across both inputs: 4×2 seed4-emit.

| Shape | Input | Profile | Proof ms | Paired vs ordinary | Paired vs release | Paired vs Aeglos | Peak MiB | Blocks within target |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 7.780 | -0.88% (3/5) | +0.00% (0/5) | -5.14% (5/5) | 43.8 | 0/5 |
| 1×2 | catalog | ordinary | 7.861 | +0.00% (0/5) | +0.89% (2/5) | -3.87% (4/5) | 44.4 | 0/5 |
| 1×2 | catalog | packed4 | 8.139 | +3.35% (0/5) | +3.65% (0/5) | -0.65% (3/5) | 37.8 | 0/5 |
| 1×2 | catalog | seed4 | 8.144 | +3.68% (0/5) | +3.50% (0/5) | -1.82% (3/5) | 40.0 | 0/5 |
| 1×2 | catalog | seed4-emit | 8.173 | +3.96% (0/5) | +4.76% (0/5) | -1.30% (3/5) | 40.7 | 0/5 |
| 1×2 | catalog | seed8-emit | 8.479 | +7.86% (0/5) | +8.36% (0/5) | +6.28% (1/5) | 39.2 | 0/5 |
| 1×2 | catalog | aeglos | 8.260 | +4.03% (1/5) | +5.42% (0/5) | +0.00% (0/5) | 458.9 | 0/5 |
| 1×2 | heldout | release | 7.822 | +0.67% (2/5) | +0.00% (0/5) | -3.58% (5/5) | 43.8 | 0/5 |
| 1×2 | heldout | ordinary | 7.772 | +0.00% (0/5) | -0.66% (3/5) | -4.73% (5/5) | 39.2 | 0/5 |
| 1×2 | heldout | packed4 | 8.061 | +4.06% (0/5) | +2.96% (0/5) | -0.01% (3/5) | 40.0 | 0/5 |
| 1×2 | heldout | seed4 | 8.209 | +5.14% (0/5) | +5.76% (0/5) | +1.43% (1/5) | 39.9 | 0/5 |
| 1×2 | heldout | seed4-emit | 8.134 | +4.83% (0/5) | +3.96% (0/5) | -0.22% (3/5) | 37.0 | 0/5 |
| 1×2 | heldout | seed8-emit | 8.413 | +8.32% (0/5) | +7.36% (0/5) | +2.96% (0/5) | 37.8 | 0/5 |
| 1×2 | heldout | aeglos | 8.152 | +4.97% (0/5) | +3.72% (0/5) | +0.00% (0/5) | 458.2 | 0/5 |
| 2×2 | catalog | release | 11.976 | -0.89% (3/5) | +0.00% (0/5) | +3.58% (1/5) | 72.9 | 0/5 |
| 2×2 | catalog | ordinary | 12.047 | +0.00% (0/5) | +0.89% (2/5) | +4.07% (0/5) | 75.3 | 0/5 |
| 2×2 | catalog | packed4 | 12.222 | +1.73% (1/5) | +2.05% (1/5) | +5.50% (0/5) | 75.3 | 0/5 |
| 2×2 | catalog | seed4 | 12.334 | +2.30% (0/5) | +3.29% (1/5) | +6.37% (0/5) | 73.8 | 0/5 |
| 2×2 | catalog | seed4-emit | 12.337 | +1.87% (0/5) | +2.78% (0/5) | +5.64% (0/5) | 71.4 | 0/5 |
| 2×2 | catalog | seed8-emit | 12.566 | +4.31% (0/5) | +4.67% (0/5) | +8.47% (0/5) | 70.7 | 0/5 |
| 2×2 | catalog | aeglos | 11.585 | -3.91% (5/5) | -3.45% (4/5) | +0.00% (0/5) | 804.4 | 0/5 |
| 2×2 | heldout | release | 12.101 | -0.31% (4/5) | +0.00% (0/5) | +4.04% (0/5) | 75.9 | 0/5 |
| 2×2 | heldout | ordinary | 12.128 | +0.00% (0/5) | +0.31% (1/5) | +4.36% (0/5) | 79.0 | 0/5 |
| 2×2 | heldout | packed4 | 12.237 | +0.81% (0/5) | +1.12% (0/5) | +5.21% (0/5) | 69.9 | 0/5 |
| 2×2 | heldout | seed4 | 12.389 | +2.64% (0/5) | +2.95% (0/5) | +7.11% (0/5) | 78.2 | 0/5 |
| 2×2 | heldout | seed4-emit | 12.338 | +2.69% (0/5) | +2.64% (0/5) | +6.85% (0/5) | 71.5 | 0/5 |
| 2×2 | heldout | seed8-emit | 12.645 | +4.58% (0/5) | +3.75% (0/5) | +9.13% (0/5) | 72.2 | 0/5 |
| 2×2 | heldout | aeglos | 11.631 | -4.18% (5/5) | -3.88% (5/5) | +0.00% (0/5) | 806.4 | 0/5 |
| 4×2 | catalog | release | 15.175 | -1.10% (4/5) | +0.00% (0/5) | -16.64% (5/5) | 195.3 | 0/5 |
| 4×2 | catalog | ordinary | 15.311 | +0.00% (0/5) | +1.11% (1/5) | -14.87% (5/5) | 208.6 | 0/5 |
| 4×2 | catalog | packed4 | 14.913 | -2.18% (5/5) | -1.79% (4/5) | -18.04% (5/5) | 199.6 | 0/5 |
| 4×2 | catalog | seed4 | 14.953 | -2.85% (5/5) | -1.63% (4/5) | -18.76% (5/5) | 195.9 | 0/5 |
| 4×2 | catalog | seed4-emit | 14.726 | -3.26% (5/5) | -3.02% (5/5) | -19.11% (5/5) | 195.8 | 0/5 |
| 4×2 | catalog | seed8-emit | 15.942 | +2.99% (0/5) | +4.48% (0/5) | -12.35% (5/5) | 200.4 | 0/5 |
| 4×2 | catalog | aeglos | 18.196 | +17.47% (0/5) | +19.96% (0/5) | +0.00% (0/5) | 1347.9 | 0/5 |
| 4×2 | heldout | release | 15.312 | -0.78% (4/5) | +0.00% (0/5) | -17.26% (5/5) | 208.7 | 0/5 |
| 4×2 | heldout | ordinary | 15.517 | +0.00% (0/5) | +0.78% (1/5) | -15.54% (5/5) | 208.7 | 0/5 |
| 4×2 | heldout | packed4 | 14.896 | -3.75% (5/5) | -2.45% (5/5) | -18.70% (5/5) | 204.9 | 0/5 |
| 4×2 | heldout | seed4 | 14.899 | -4.00% (5/5) | -2.29% (5/5) | -19.15% (5/5) | 204.9 | 0/5 |
| 4×2 | heldout | seed4-emit | 14.782 | -4.74% (5/5) | -3.35% (5/5) | -19.54% (5/5) | 205.0 | 0/5 |
| 4×2 | heldout | seed8-emit | 15.762 | +1.16% (0/5) | +3.57% (0/5) | -14.08% (5/5) | 205.8 | 0/5 |
| 4×2 | heldout | aeglos | 18.372 | +18.39% (0/5) | +20.86% (0/5) | +0.00% (0/5) | 1346.6 | 0/5 |

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
