# Shared L3 placement for the frontend pool on C7a

Zopro uses 64 main and 32 frontend cores for 2×2 and 72 main and 24 frontend for 8×2. The ordinary layout gives whole L3 groups to each pool. The interleaved variants assign some cores in a shared L3 group to each pool. The all-group variant spreads frontend cores across all twelve L3 groups and the tail variant assigns half the cores in each selected L3 group to the frontend. Prepared Aeglos uses the same 96 cores. Observed worker CPU IDs match the configured layouts.

The two primary 2×2 inputs use five matched process blocks. The two primary 8×2 inputs use six, and two additional inputs use four. Each process primes three proofs and measures thirteen. All 150 exports independently verified. The first attempt stopped when an output path in the 2×2 verifier pointed to the wrong directory. The complete capture used the corrected path on a fresh host. Both temporary instances and their storage were removed.

- 2×2 `interleave-tail` took 12.187 ms on the original and 12.375 ms on the held-out input.
- 8×2 `interleave-tail` took 19.413 ms on the original and 19.626 ms on the held-out input.

Neither interleaved placement improved both controls consistently across the tested inputs.

| Shape | Input | Layout | Proof ms | Paired vs ordinary | Paired vs release | Paired vs Aeglos | Peak MiB | Blocks within target |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 2×2 | catalog | release | 12.099 | -0.00% (3/5) | +0.00% (0/5) | +5.08% (0/5) | 72.3 | 0/5 |
| 2×2 | catalog | ordinary | 12.095 | +0.00% (0/5) | +0.00% (2/5) | +4.73% (0/5) | 75.2 | 0/5 |
| 2×2 | catalog | interleave-all | 12.303 | +1.71% (0/5) | +1.96% (1/5) | +6.01% (0/5) | 74.5 | 0/5 |
| 2×2 | catalog | interleave-tail | 12.187 | +0.77% (1/5) | +0.95% (2/5) | +5.01% (0/5) | 70.0 | 0/5 |
| 2×2 | catalog | aeglos | 11.546 | -4.52% (5/5) | -4.83% (5/5) | +0.00% (0/5) | 813.7 | 0/5 |
| 2×2 | heldout | release | 12.122 | -0.07% (3/5) | +0.00% (0/5) | +4.81% (1/5) | 77.5 | 0/5 |
| 2×2 | heldout | ordinary | 12.171 | +0.00% (0/5) | +0.07% (2/5) | +5.23% (1/5) | 72.9 | 0/5 |
| 2×2 | heldout | interleave-all | 12.439 | +2.34% (0/5) | +2.27% (0/5) | +6.41% (0/5) | 74.5 | 0/5 |
| 2×2 | heldout | interleave-tail | 12.375 | +1.70% (0/5) | +1.49% (0/5) | +7.43% (1/5) | 72.9 | 0/5 |
| 2×2 | heldout | aeglos | 11.566 | -4.97% (4/5) | -4.59% (4/5) | +0.00% (0/5) | 801.2 | 0/5 |
| 8×2 | catalog | release | 19.068 | +0.78% (1/6) | +0.00% (0/6) | -36.22% (6/6) | 360.6 | 0/6 |
| 8×2 | catalog | ordinary | 18.991 | +0.00% (0/6) | -0.77% (5/6) | -37.09% (6/6) | 363.4 | 2/6 |
| 8×2 | catalog | interleave-all | 19.468 | +3.06% (0/6) | +1.69% (0/6) | -35.20% (6/6) | 363.5 | 0/6 |
| 8×2 | catalog | interleave-tail | 19.413 | +1.71% (0/6) | +1.09% (1/6) | -35.48% (6/6) | 358.2 | 0/6 |
| 8×2 | catalog | aeglos | 30.113 | +58.96% (0/6) | +56.80% (0/6) | +0.00% (0/6) | 2499.0 | 0/6 |
| 8×2 | heldout | release | 19.400 | +2.17% (1/6) | +0.00% (0/6) | -35.84% (6/6) | 366.5 | 0/6 |
| 8×2 | heldout | ordinary | 18.979 | +0.00% (0/6) | -2.12% (5/6) | -37.39% (6/6) | 366.4 | 1/6 |
| 8×2 | heldout | interleave-all | 19.672 | +3.67% (0/6) | +1.53% (0/6) | -34.69% (6/6) | 355.1 | 0/6 |
| 8×2 | heldout | interleave-tail | 19.626 | +3.02% (0/6) | +1.08% (1/6) | -35.32% (6/6) | 356.8 | 0/6 |
| 8×2 | heldout | aeglos | 30.272 | +59.71% (0/6) | +55.87% (0/6) | +0.00% (0/6) | 2495.3 | 0/6 |
| 8×2 | v153 | release | 19.392 | +2.34% (0/4) | +0.00% (0/4) | -36.75% (4/4) | 365.0 | 0/4 |
| 8×2 | v153 | ordinary | 18.887 | +0.00% (0/4) | -2.27% (4/4) | -37.37% (4/4) | 359.6 | 2/4 |
| 8×2 | v153 | interleave-all | 19.590 | +4.13% (0/4) | +0.49% (1/4) | -35.42% (4/4) | 355.0 | 0/4 |
| 8×2 | v153 | interleave-tail | 19.585 | +4.00% (0/4) | +1.58% (1/4) | -35.00% (4/4) | 355.3 | 0/4 |
| 8×2 | v153 | aeglos | 30.375 | +59.67% (0/4) | +58.10% (0/4) | +0.00% (0/4) | 2493.8 | 0/4 |
| 8×2 | v156 | release | 19.208 | -0.47% (3/4) | +0.00% (0/4) | -36.79% (4/4) | 384.5 | 0/4 |
| 8×2 | v156 | ordinary | 19.349 | +0.00% (0/4) | +0.47% (1/4) | -36.65% (4/4) | 365.7 | 1/4 |
| 8×2 | v156 | interleave-all | 19.824 | +2.40% (0/4) | +3.06% (0/4) | -34.90% (4/4) | 358.9 | 0/4 |
| 8×2 | v156 | interleave-tail | 19.579 | +1.81% (0/4) | +2.11% (0/4) | -35.45% (4/4) | 361.9 | 0/4 |
| 8×2 | v156 | aeglos | 30.352 | +57.85% (0/4) | +58.19% (0/4) | +0.00% (0/4) | 2462.1 | 0/4 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [CPU topology](artifacts/topology.txt) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU on the recorded profiles.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 828 frontend solver full | 15.103 | 109 / 48 | 7.22× / 3.18× | +4.203 |
| 8×2 | 833 SDK release-e4 72+24 | 19.117 | 189 / 74 | 9.89× / 3.87× | +0.217 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
