# Folded quotient placement on C7a

The 8×2 comparison runs the existing folded quotient FFT in the main or frontend Rayon pool under the four-epoch solver profile. The selected core split budgets 72 main and 24 frontend physical cores. An additional arm uses 68+28. The trial repeats the seeded scheduler with encoded Poseidon emission and tests both quotient placements. Prepared Aeglos uses the same 96 cores.

There are six matched process blocks per primary input and four per additional input. Each process primes three proofs and times thirteen. All 172 exports independently verified. The temporary instance and storage were removed.

The fastest trial median on the original input was 19.239 ms with `seed-emit`. It took 19.386 ms on the held-out input. No profile improved both matched controls consistently across the four inputs.

| Input | Profile | Proof ms | Matched control | Paired vs control | Paired vs release | Paired vs Aeglos | Peak MiB | Blocks at or below 18.9 ms |
|---|---|---:|---|---:|---:|---:|---:|---:|
| catalog | release | 19.498 | release | +0.00% (0/6) | +0.00% (0/6) | -36.93% (6/6) | 354.4 | 0/6 |
| catalog | release-quote-front | 21.285 | release | +8.87% (0/6) | +8.87% (0/6) | -31.42% (6/6) | 269.0 | 0/6 |
| catalog | ordinary | 19.354 | ordinary | +0.00% (0/6) | -1.07% (4/6) | -37.67% (6/6) | 359.1 | 0/6 |
| catalog | ordinary-quote-front | 21.104 | ordinary | +10.03% (0/6) | +8.43% (0/6) | -31.68% (6/6) | 262.3 | 0/6 |
| catalog | seed-emit | 19.239 | ordinary | -0.74% (4/6) | -1.57% (6/6) | -37.92% (6/6) | 353.0 | 0/6 |
| catalog | seed-emit-quote-front | 20.984 | seed-emit | +9.65% (0/6) | +7.95% (0/6) | -32.64% (6/6) | 269.2 | 0/6 |
| catalog | ordinary-quote-front-68 | 21.243 | ordinary | +9.04% (0/6) | +8.27% (0/6) | -32.23% (6/6) | 278.8 | 0/6 |
| catalog | aeglos | 31.042 | aeglos | +0.00% (0/6) | +58.56% (0/6) | +0.00% (0/6) | 2459.5 | 0/6 |
| heldout | release | 19.623 | release | +0.00% (0/6) | +0.00% (0/6) | -37.63% (6/6) | 361.2 | 0/6 |
| heldout | release-quote-front | 21.759 | release | +10.56% (0/6) | +10.56% (0/6) | -30.71% (6/6) | 272.0 | 0/6 |
| heldout | ordinary | 19.397 | ordinary | +0.00% (0/6) | -0.28% (6/6) | -38.15% (6/6) | 365.7 | 0/6 |
| heldout | ordinary-quote-front | 21.338 | ordinary | +9.66% (0/6) | +8.81% (0/6) | -32.21% (6/6) | 273.4 | 0/6 |
| heldout | seed-emit | 19.386 | ordinary | -0.51% (4/6) | -1.15% (5/6) | -38.30% (6/6) | 376.8 | 0/6 |
| heldout | seed-emit-quote-front | 21.451 | seed-emit | +10.90% (0/6) | +9.93% (0/6) | -31.79% (6/6) | 271.4 | 0/6 |
| heldout | ordinary-quote-front-68 | 21.567 | ordinary | +11.38% (0/6) | +9.98% (0/6) | -31.25% (6/6) | 284.0 | 0/6 |
| heldout | aeglos | 31.450 | aeglos | +0.00% (0/6) | +60.34% (0/6) | +0.00% (0/6) | 2474.7 | 0/6 |
| v153 | release | 19.786 | release | +0.00% (0/4) | +0.00% (0/4) | -37.65% (4/4) | 356.0 | 0/4 |
| v153 | release-quote-front | 21.564 | release | +9.37% (0/4) | +9.37% (0/4) | -31.15% (4/4) | 261.5 | 0/4 |
| v153 | ordinary | 19.677 | ordinary | +0.00% (0/4) | -0.31% (2/4) | -37.51% (4/4) | 362.8 | 0/4 |
| v153 | ordinary-quote-front | 21.417 | ordinary | +8.60% (0/4) | +8.97% (0/4) | -32.19% (4/4) | 257.7 | 0/4 |
| v153 | seed-emit | 19.369 | ordinary | -1.53% (4/4) | -2.61% (3/4) | -38.77% (4/4) | 374.7 | 0/4 |
| v153 | seed-emit-quote-front | 21.127 | seed-emit | +9.32% (0/4) | +7.41% (0/4) | -33.42% (4/4) | 278.7 | 0/4 |
| v153 | ordinary-quote-front-68 | 21.183 | ordinary | +7.37% (0/4) | +7.68% (0/4) | -33.10% (4/4) | 270.3 | 0/4 |
| v153 | aeglos | 31.490 | aeglos | +0.00% (0/4) | +60.42% (0/4) | +0.00% (0/4) | 2459.8 | 0/4 |
| v156 | release | 19.657 | release | +0.00% (0/4) | +0.00% (0/4) | -37.13% (4/4) | 368.0 | 0/4 |
| v156 | release-quote-front | 21.454 | release | +8.84% (0/4) | +8.84% (0/4) | -31.56% (4/4) | 266.1 | 0/4 |
| v156 | ordinary | 19.372 | ordinary | +0.00% (0/4) | -1.07% (4/4) | -37.98% (4/4) | 360.0 | 0/4 |
| v156 | ordinary-quote-front | 21.268 | ordinary | +9.05% (0/4) | +8.03% (0/4) | -32.07% (4/4) | 260.6 | 0/4 |
| v156 | seed-emit | 19.477 | ordinary | +0.18% (2/4) | -0.78% (4/4) | -37.81% (4/4) | 365.8 | 0/4 |
| v156 | seed-emit-quote-front | 21.382 | seed-emit | +9.98% (0/4) | +8.15% (0/4) | -32.00% (4/4) | 265.3 | 0/4 |
| v156 | ordinary-quote-front-68 | 21.458 | ordinary | +10.71% (0/4) | +8.98% (0/4) | -31.71% (4/4) | 271.8 | 0/4 |
| v156 | aeglos | 31.321 | aeglos | +0.00% (0/4) | +59.05% (0/4) | +0.00% (0/4) | 2463.7 | 0/4 |

## Stage clocks

Three paired process blocks per primary input separately clock the ordinary build with each quotient placement. The curve interval starts when the FFT returns and ends when the last curve query returns. It excludes proof assembly and final verification.

| Input | Profile | Proof ms | Paired clock effect | Solver ms | Gather ms | Quotient ms | Curve interval ms |
|---|---|---:|---:|---:|---:|---:|---:|
| catalog | ordinary | 19.123 | -0.21% (3/3) | 3.207 | 1.018 | 2.890 | 9.907 |
| catalog | ordinary-quote-front | 21.362 | +0.27% (1/3) | 3.342 | 1.069 | 4.546 | 10.447 |
| heldout | ordinary | 19.407 | +0.71% (0/3) | 3.140 | 1.016 | 2.851 | 10.310 |
| heldout | ordinary-quote-front | 21.475 | +1.66% (1/3) | 3.168 | 1.031 | 4.578 | 10.691 |

[Samples and clocks](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Trial source](artifacts/source.patch) · [Verification](artifacts/verification.json)

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
