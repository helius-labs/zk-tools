# Full frontend solver placement for seeded 4×2 proofs on C7a

The [prior small-shape study](../2026-11-20-small-seeded-solver-c7a854/README.md) found a gain on 4×2 from four epochs, certified buffers, seeded claims and encoded Poseidon stores. This follow-up repeats that configuration and varies the main and frontend worker split. The full frontend mode also runs solver buffer allocation and witness copying on the frontend pool. Prepared Aeglos uses the same 96 physical cores.

The two primary inputs have six matched process blocks and the additional inputs have four. Each process primes three proofs and times thirteen. All 202 exports independently verified. Tests also cover zero optional workspace and concurrent solver requests. The temporary instance and storage were removed.

The fastest trial median on the original input was 14.867 ms with `seed-56`. It took 14.966 ms on the held-out input. These configurations improved both controls across all four inputs: sdk-seed, seed, full-seed, full-packed, seed-56.

| Input | Profile | Proof ms | Matched control | Paired vs control | Paired vs release | Paired vs prior SDK settings | Paired vs Aeglos | Peak MiB | Blocks at or below 10.9 ms |
|---|---|---:|---|---:|---:|---:|---:|---:|---:|
| catalog | release | 15.305 | ordinary | -1.20% (4/6) | +0.00% (0/6) | +2.21% (0/6) | -14.96% (6/6) | 204.4 | 0/6 |
| catalog | ordinary | 15.498 | ordinary | +0.00% (0/6) | +1.22% (2/6) | +3.69% (0/6) | -14.75% (6/6) | 213.9 | 0/6 |
| catalog | sdk-seed | 15.018 | ordinary | -3.56% (6/6) | -2.16% (6/6) | +0.00% (0/6) | -17.22% (6/6) | 198.9 | 0/6 |
| catalog | seed | 15.005 | ordinary | -3.23% (6/6) | -1.95% (6/6) | +0.21% (2/6) | -17.20% (6/6) | 197.5 | 0/6 |
| catalog | full-seed | 15.037 | ordinary | -2.35% (6/6) | -2.06% (5/6) | +0.92% (2/6) | -16.89% (6/6) | 195.9 | 0/6 |
| catalog | full-packed | 14.937 | ordinary | -2.97% (5/6) | -2.39% (5/6) | -0.40% (4/6) | -16.73% (6/6) | 198.2 | 0/6 |
| catalog | seed-56 | 14.867 | ordinary | -3.72% (6/6) | -2.93% (6/6) | -0.55% (3/6) | -18.00% (6/6) | 205.0 | 0/6 |
| catalog | seed-72 | 15.166 | ordinary | -3.02% (4/6) | -1.28% (5/6) | +1.87% (2/6) | -15.66% (6/6) | 190.7 | 0/6 |
| catalog | aeglos | 18.076 | ordinary | +17.30% (0/6) | +17.59% (0/6) | +20.80% (0/6) | +0.00% (0/6) | 1349.6 | 0/6 |
| heldout | release | 15.334 | ordinary | -1.64% (4/6) | +0.00% (0/6) | +2.42% (1/6) | -14.12% (6/6) | 207.2 | 0/6 |
| heldout | ordinary | 15.535 | ordinary | +0.00% (0/6) | +1.67% (2/6) | +3.09% (0/6) | -13.47% (6/6) | 201.2 | 0/6 |
| heldout | sdk-seed | 15.063 | ordinary | -2.99% (6/6) | -2.36% (5/6) | +0.00% (0/6) | -16.04% (6/6) | 203.6 | 0/6 |
| heldout | seed | 15.101 | ordinary | -2.95% (6/6) | -1.93% (6/6) | -0.13% (4/6) | -15.87% (6/6) | 204.2 | 0/6 |
| heldout | full-seed | 15.158 | ordinary | -2.38% (6/6) | -1.80% (5/6) | +0.16% (3/6) | -15.25% (6/6) | 184.6 | 0/6 |
| heldout | full-packed | 15.100 | ordinary | -2.74% (6/6) | -1.98% (6/6) | +0.70% (2/6) | -15.97% (6/6) | 189.4 | 0/6 |
| heldout | seed-56 | 14.966 | ordinary | -3.33% (6/6) | -2.81% (6/6) | -0.13% (3/6) | -16.84% (6/6) | 204.9 | 0/6 |
| heldout | seed-72 | 15.043 | ordinary | -3.42% (6/6) | -1.90% (6/6) | -0.13% (3/6) | -16.15% (6/6) | 198.1 | 0/6 |
| heldout | aeglos | 17.952 | ordinary | +15.56% (0/6) | +16.44% (0/6) | +19.11% (0/6) | +0.00% (0/6) | 1344.1 | 0/6 |
| v153 | release | 15.336 | ordinary | -1.32% (4/4) | +0.00% (0/4) | +4.12% (0/4) | -14.47% (4/4) | 210.9 | 0/4 |
| v153 | ordinary | 15.541 | ordinary | +0.00% (0/4) | +1.33% (0/4) | +5.68% (0/4) | -13.27% (4/4) | 203.5 | 0/4 |
| v153 | sdk-seed | 14.742 | ordinary | -5.37% (4/4) | -3.96% (4/4) | +0.00% (0/4) | -17.75% (4/4) | 194.4 | 0/4 |
| v153 | seed | 15.162 | ordinary | -2.88% (4/4) | -1.39% (4/4) | +2.85% (0/4) | -15.41% (4/4) | 201.9 | 0/4 |
| v153 | full-seed | 14.997 | ordinary | -3.65% (4/4) | -2.32% (4/4) | +1.62% (0/4) | -16.21% (4/4) | 190.6 | 0/4 |
| v153 | full-packed | 15.040 | ordinary | -3.20% (4/4) | -2.11% (4/4) | +2.17% (0/4) | -16.08% (4/4) | 190.7 | 0/4 |
| v153 | seed-56 | 15.005 | ordinary | -3.45% (4/4) | -2.34% (4/4) | +1.83% (0/4) | -16.24% (4/4) | 196.6 | 0/4 |
| v153 | seed-72 | 15.322 | ordinary | -1.85% (4/4) | -0.09% (2/4) | +3.94% (0/4) | -15.06% (4/4) | 192.1 | 0/4 |
| v153 | aeglos | 17.947 | ordinary | +15.30% (0/4) | +16.93% (0/4) | +21.59% (0/4) | +0.00% (0/4) | 1345.9 | 0/4 |
| v156 | release | 15.350 | ordinary | -1.20% (3/4) | +0.00% (0/4) | +2.73% (0/4) | -16.15% (4/4) | 202.7 | 0/4 |
| v156 | ordinary | 15.421 | ordinary | +0.00% (0/4) | +1.22% (1/4) | +2.86% (0/4) | -14.87% (4/4) | 206.4 | 0/4 |
| v156 | sdk-seed | 14.961 | ordinary | -2.78% (4/4) | -2.66% (4/4) | +0.00% (0/4) | -17.72% (4/4) | 196.6 | 0/4 |
| v156 | seed | 14.779 | ordinary | -3.97% (4/4) | -3.01% (3/4) | -1.22% (3/4) | -18.58% (4/4) | 198.1 | 0/4 |
| v156 | full-seed | 15.016 | ordinary | -2.42% (4/4) | -2.30% (3/4) | +0.37% (1/4) | -17.47% (4/4) | 185.4 | 0/4 |
| v156 | full-packed | 14.961 | ordinary | -3.56% (4/4) | -2.39% (4/4) | -0.55% (3/4) | -17.50% (4/4) | 189.9 | 0/4 |
| v156 | seed-56 | 14.897 | ordinary | -3.72% (4/4) | -2.90% (4/4) | -0.96% (3/4) | -18.00% (4/4) | 200.6 | 0/4 |
| v156 | seed-72 | 15.088 | ordinary | -2.70% (4/4) | -1.66% (3/4) | +0.29% (1/4) | -16.76% (4/4) | 208.7 | 0/4 |
| v156 | aeglos | 18.192 | ordinary | +17.46% (0/4) | +19.27% (0/4) | +21.54% (0/4) | +0.00% (0/4) | 1345.9 | 0/4 |

## Solver phase timing

Three separate matched blocks on each primary input time solver allocation, execution and finishing before scalar gather. Clocked proofs have an unclocked companion in the same block.

| Input | Profile | Solve ms | Full proof with clock ms | Paired clock effect |
|---|---|---:|---:|---:|
| catalog | ordinary | 3.597 | 15.648 | +1.81% (0/3) |
| catalog | seed | 2.972 | 14.815 | +0.14% (1/3) |
| catalog | full-seed | 3.055 | 15.052 | +0.04% (1/3) |
| heldout | ordinary | 3.547 | 15.532 | -0.14% (2/3) |
| heldout | seed | 3.017 | 15.200 | +0.39% (1/3) |
| heldout | full-seed | 3.044 | 15.146 | +0.23% (1/3) |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [SDK source](artifacts/source.patch) · [Verification](artifacts/verification.json)

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
