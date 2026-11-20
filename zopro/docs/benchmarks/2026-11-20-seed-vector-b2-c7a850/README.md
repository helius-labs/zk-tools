# Combining seeded solver work with vector decoding on C7a

This 8×2 comparison enables [eight-lane conversion of distinct B2 scalars](../2026-11-20-serial-b2-vector-decode-c7a848/README.md), [seeded solver tasks with encoded Poseidon emission](../2026-11-19-frontend-solver-full-c7a840/README.md), and both together. Zopro uses 72 main and 24 frontend physical cores with whole L3 groups assigned to a pool. Prepared Aeglos uses all 96 cores.

The two primary inputs have six matched process blocks and the two additional inputs have four. Each process primes three proofs and times thirteen. All 136 exports independently verified, including probes with zero optional workspace. The temporary instance and storage were removed.

The fastest trial median on the original input was 18.808 ms with `combined`. It took 19.083 ms on the held-out input. None of the three trial profiles improved both controls consistently across the four inputs.

| Input | Profile | Proof ms | Matched control | Paired vs control | Paired vs release | Paired vs vector | Paired vs seed and emit | Paired vs Aeglos | Peak MiB | Blocks at or below 18.9 ms |
|---|---|---:|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | release | 19.101 | release | +0.00% (0/6) | +0.00% (0/6) | +0.41% (2/6) | -0.18% (3/6) | -36.67% (6/6) | 362.7 | 0/6 |
| catalog | ordinary | 19.149 | ordinary | +0.00% (0/6) | -0.07% (4/6) | +0.56% (1/6) | +0.09% (3/6) | -36.74% (6/6) | 364.4 | 0/6 |
| catalog | vector | 19.130 | ordinary | -0.56% (5/6) | -0.41% (4/6) | +0.00% (0/6) | -0.29% (4/6) | -36.89% (6/6) | 371.0 | 0/6 |
| catalog | seed-emit | 19.185 | ordinary | -0.08% (3/6) | +0.19% (3/6) | +0.29% (2/6) | +0.00% (0/6) | -36.27% (6/6) | 362.8 | 1/6 |
| catalog | combined | 18.808 | ordinary | -1.76% (6/6) | -1.42% (6/6) | -1.92% (4/6) | -2.07% (5/6) | -37.72% (6/6) | 353.7 | 4/6 |
| catalog | aeglos | 30.206 | aeglos | +0.00% (0/6) | +57.91% (0/6) | +58.44% (0/6) | +56.93% (0/6) | +0.00% (0/6) | 2462.5 | 0/6 |
| heldout | release | 19.416 | release | +0.00% (0/6) | +0.00% (0/6) | +1.01% (2/6) | -0.17% (4/6) | -36.43% (6/6) | 359.8 | 0/6 |
| heldout | ordinary | 19.471 | ordinary | +0.00% (0/6) | +0.32% (2/6) | +0.41% (2/6) | +0.38% (2/6) | -36.72% (6/6) | 379.2 | 0/6 |
| heldout | vector | 19.326 | ordinary | -0.41% (4/6) | -1.00% (4/6) | +0.00% (0/6) | -0.18% (3/6) | -36.50% (6/6) | 366.4 | 0/6 |
| heldout | seed-emit | 19.383 | ordinary | -0.38% (4/6) | +0.17% (2/6) | +0.18% (3/6) | +0.00% (0/6) | -36.47% (6/6) | 371.7 | 0/6 |
| heldout | combined | 19.083 | ordinary | -2.26% (5/6) | -2.12% (6/6) | -1.22% (4/6) | -1.89% (5/6) | -37.56% (6/6) | 386.2 | 0/6 |
| heldout | aeglos | 30.683 | aeglos | +0.00% (0/6) | +57.32% (0/6) | +57.49% (0/6) | +57.41% (0/6) | +0.00% (0/6) | 2463.4 | 0/6 |
| v153 | release | 19.143 | release | +0.00% (0/4) | +0.00% (0/4) | -0.33% (2/4) | -0.01% (2/4) | -37.39% (4/4) | 377.1 | 0/4 |
| v153 | ordinary | 19.417 | ordinary | +0.00% (0/4) | +0.89% (0/4) | +0.64% (1/4) | +0.69% (1/4) | -36.26% (4/4) | 350.0 | 0/4 |
| v153 | vector | 19.328 | ordinary | -0.64% (3/4) | +0.33% (2/4) | +0.00% (0/4) | +0.33% (2/4) | -36.76% (4/4) | 373.2 | 0/4 |
| v153 | seed-emit | 19.254 | ordinary | -0.69% (3/4) | +0.01% (2/4) | -0.33% (2/4) | +0.00% (0/4) | -36.89% (4/4) | 372.3 | 0/4 |
| v153 | combined | 19.171 | ordinary | -1.86% (3/4) | -0.18% (2/4) | -0.82% (3/4) | -0.51% (3/4) | -37.28% (4/4) | 358.8 | 0/4 |
| v153 | aeglos | 30.507 | aeglos | +0.00% (0/4) | +59.72% (0/4) | +58.14% (0/4) | +58.46% (0/4) | +0.00% (0/4) | 2493.6 | 0/4 |
| v156 | release | 19.183 | release | +0.00% (0/4) | +0.00% (0/4) | -1.72% (3/4) | +0.28% (2/4) | -36.68% (4/4) | 362.6 | 0/4 |
| v156 | ordinary | 19.338 | ordinary | +0.00% (0/4) | +0.81% (0/4) | -0.30% (2/4) | +1.01% (0/4) | -36.11% (4/4) | 376.8 | 0/4 |
| v156 | vector | 19.584 | ordinary | +0.30% (2/4) | +1.75% (1/4) | +0.00% (0/4) | +2.83% (1/4) | -35.72% (4/4) | 374.7 | 0/4 |
| v156 | seed-emit | 19.104 | ordinary | -1.00% (4/4) | -0.28% (2/4) | -2.76% (3/4) | +0.00% (0/4) | -37.21% (4/4) | 371.0 | 0/4 |
| v156 | combined | 19.059 | ordinary | -0.76% (4/4) | -0.40% (3/4) | -2.01% (3/4) | -0.22% (2/4) | -36.95% (4/4) | 367.9 | 0/4 |
| v156 | aeglos | 30.322 | aeglos | +0.00% (0/4) | +57.94% (0/4) | +55.57% (0/4) | +59.27% (0/4) | +0.00% (0/4) | 2464.4 | 0/4 |

## Stage clocks

Three pairs per primary input separately clock solver execution and B2 scalar decoding. The last column counts proofs in which the B2 query ended after every other recorded curve query.

| Input | Profile | Proof ms | Paired clock effect | Solver ms | Decode µs | B2 last |
|---|---|---:|---|---:|---:|
| catalog | ordinary | 19.166 | +0.28% (1/3) | 3.085 | 758.9 | 22/39 |
| catalog | combined | 19.254 | +2.10% (0/3) | 3.135 | 174.3 | 21/39 |
| heldout | ordinary | 19.548 | +0.41% (0/3) | 3.175 | 798.2 | 27/39 |
| heldout | combined | 19.404 | +0.30% (1/3) | 3.053 | 182.3 | 19/39 |

[Samples and clocks](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Trial source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU on the recorded profiles.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 828 frontend solver full | 15.103 | 109 / 48 | 7.22× / 3.18× | +4.203 |
| 8×2 | 833 SDK release-e4 72+24 | 19.117 | 189 / 74 | 9.89× / 3.87× | +0.217 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
