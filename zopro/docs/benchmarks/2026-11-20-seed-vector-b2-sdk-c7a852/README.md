# SDK confirmation of the seeded solver and B2 vector profile

The [prototype](../2026-11-20-seed-vector-b2-c7a850/README.md) combined seeded solver work claims, encoded Poseidon emission and serial vector conversion of grouped B2 scalars. This run compares the same settings on a clean SDK build against its ordinary mode, the released build, the exact earlier prototype and prepared Aeglos. Both provers use the same 96 physical C7a cores. Zopro keeps 72 main and 24 frontend cores in separate L3 groups.

The two primary inputs have eight fresh matched process blocks and the additional inputs have six. Each process primes three proofs and times thirteen. All 144 exports independently verified, including zero optional memory probes. Three native tests and a portable core build passed. The benchmark driver uses the released worker placement adapter. The observed CPU IDs match the configured 72+24 split. The temporary instance and storage were removed.

The fastest trial median on the original input was 18.849 ms with `combined`. It took 19.046 ms on the held-out input. The SDK profile did not improve both controls with a majority of paired wins on every input.

| Input | Profile | Proof ms | Matched control | Paired vs control | Paired vs release | Paired vs prototype | Paired vs Aeglos | Peak MiB | Blocks at or below 18.9 ms |
|---|---|---:|---|---:|---:|---:|---:|---:|---:|
| catalog | release | 19.265 | release | +0.00% (0/8) | +0.00% (0/8) | +0.84% (3/8) | -37.92% (8/8) | 368.0 | 1/8 |
| catalog | ordinary | 19.148 | ordinary | +0.00% (0/8) | -0.48% (5/8) | -0.50% (5/8) | -38.07% (8/8) | 372.5 | 3/8 |
| catalog | combined | 18.849 | ordinary | -1.66% (5/8) | -1.68% (8/8) | -0.66% (8/8) | -38.92% (8/8) | 358.2 | 5/8 |
| catalog | archive | 19.037 | ordinary | +0.50% (3/8) | -0.83% (5/8) | +0.00% (0/8) | -38.10% (8/8) | 366.2 | 1/8 |
| catalog | aeglos | 30.917 | aeglos | +0.00% (0/8) | +61.08% (0/8) | +61.54% (0/8) | +0.00% (0/8) | 2488.5 | 0/8 |
| heldout | release | 19.399 | release | +0.00% (0/8) | +0.00% (0/8) | +0.10% (3/8) | -36.65% (8/8) | 373.9 | 0/8 |
| heldout | ordinary | 19.213 | ordinary | +0.00% (0/8) | -1.18% (6/8) | -0.26% (6/8) | -37.10% (8/8) | 369.5 | 1/8 |
| heldout | combined | 19.046 | ordinary | -0.25% (4/8) | -1.40% (8/8) | -1.33% (5/8) | -37.58% (8/8) | 361.2 | 1/8 |
| heldout | archive | 19.254 | ordinary | +0.26% (2/8) | -0.10% (5/8) | +0.00% (0/8) | -36.86% (8/8) | 366.4 | 0/8 |
| heldout | aeglos | 30.628 | aeglos | +0.00% (0/8) | +57.85% (0/8) | +58.39% (0/8) | +0.00% (0/8) | 2502.2 | 0/8 |
| v153 | release | 19.362 | release | +0.00% (0/6) | +0.00% (0/6) | +0.65% (1/6) | -36.74% (6/6) | 379.3 | 0/6 |
| v153 | ordinary | 19.369 | ordinary | +0.00% (0/6) | -0.17% (4/6) | +1.24% (2/6) | -37.09% (6/6) | 370.9 | 1/6 |
| v153 | combined | 19.093 | ordinary | -1.14% (4/6) | -1.60% (5/6) | -0.89% (4/6) | -37.39% (6/6) | 375.5 | 0/6 |
| v153 | archive | 19.235 | ordinary | -1.23% (4/6) | -0.65% (5/6) | +0.00% (0/6) | -37.08% (6/6) | 371.6 | 0/6 |
| v153 | aeglos | 30.649 | aeglos | +0.00% (0/6) | +58.07% (0/6) | +58.93% (0/6) | +0.00% (0/6) | 2465.9 | 0/6 |
| v156 | release | 19.503 | release | +0.00% (0/6) | +0.00% (0/6) | +0.47% (1/6) | -36.14% (6/6) | 357.6 | 0/6 |
| v156 | ordinary | 19.348 | ordinary | +0.00% (0/6) | -0.39% (3/6) | -0.07% (3/6) | -36.91% (6/6) | 380.6 | 0/6 |
| v156 | combined | 19.090 | ordinary | -1.09% (5/6) | -1.89% (5/6) | -1.33% (5/6) | -37.44% (6/6) | 357.4 | 1/6 |
| v156 | archive | 19.398 | ordinary | +0.07% (3/6) | -0.46% (5/6) | +0.00% (0/6) | -36.40% (6/6) | 358.1 | 0/6 |
| v156 | aeglos | 30.643 | aeglos | +0.00% (0/6) | +56.59% (0/6) | +57.23% (0/6) | +0.00% (0/6) | 2489.8 | 0/6 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [SDK source](artifacts/source.patch) · [Driver patch](artifacts/driver.patch) · [Verification](artifacts/verification.json)

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
