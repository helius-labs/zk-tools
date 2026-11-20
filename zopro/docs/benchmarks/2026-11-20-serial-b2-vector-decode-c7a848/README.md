# Vector decoding of grouped B2 scalars on C7a

The trial uses eight-lane IFMA in the existing serial conversion pass for distinct B2 scalars. The 8×2 proof uses 72 main and 24 frontend physical cores, with the quotient on the main pool and four solver epochs. In the [previous timeline run](../2026-11-20-folded-quotient-placement-c7a843/README.md), B2 finished last in 53 of 78 ordinary clocked proofs across the two main inputs. Prepared Aeglos uses all 96 cores.

There are six matched process blocks per primary input and four per additional input. Each process primes three proofs and times thirteen. All 96 exports independently verified, including vector route probes at ordinary and zero optional workspace. The temporary instance and storage were removed.

The fastest trial median on the original input was 18.814 ms with `vector`. It took 19.303 ms on the held-out input. Vector decoding did not improve both controls consistently across the four inputs.

| Input | Profile | Proof ms | Matched control | Paired vs control | Paired vs release | Paired vs Aeglos | Peak MiB | Blocks at or below 18.9 ms |
|---|---|---:|---|---:|---:|---:|---:|---:|
| catalog | release | 19.208 | release | +0.00% (0/6) | +0.00% (0/6) | -36.88% (6/6) | 371.8 | 0/6 |
| catalog | ordinary | 19.020 | ordinary | +0.00% (0/6) | -1.30% (5/6) | -37.61% (6/6) | 363.6 | 2/6 |
| catalog | vector | 18.814 | ordinary | -0.73% (5/6) | -1.58% (6/6) | -38.02% (6/6) | 358.0 | 4/6 |
| catalog | aeglos | 30.468 | aeglos | +0.00% (0/6) | +58.42% (0/6) | +0.00% (0/6) | 2484.8 | 0/6 |
| heldout | release | 19.429 | release | +0.00% (0/6) | +0.00% (0/6) | -36.47% (6/6) | 370.9 | 0/6 |
| heldout | ordinary | 19.229 | ordinary | +0.00% (0/6) | -1.03% (4/6) | -36.88% (6/6) | 366.7 | 0/6 |
| heldout | vector | 19.303 | ordinary | -0.39% (3/6) | -0.69% (4/6) | -37.37% (6/6) | 370.3 | 1/6 |
| heldout | aeglos | 30.584 | aeglos | +0.00% (0/6) | +57.42% (0/6) | +0.00% (0/6) | 2458.2 | 0/6 |
| v153 | release | 19.224 | release | +0.00% (0/4) | +0.00% (0/4) | -37.29% (4/4) | 358.8 | 0/4 |
| v153 | ordinary | 19.125 | ordinary | +0.00% (0/4) | -0.40% (3/4) | -37.80% (4/4) | 364.5 | 1/4 |
| v153 | vector | 18.992 | ordinary | -0.23% (3/4) | -1.22% (3/4) | -37.49% (4/4) | 367.9 | 0/4 |
| v153 | aeglos | 30.344 | aeglos | +0.00% (0/4) | +59.46% (0/4) | +0.00% (0/4) | 2492.0 | 0/4 |
| v156 | release | 19.255 | release | +0.00% (0/4) | +0.00% (0/4) | -37.01% (4/4) | 356.1 | 0/4 |
| v156 | ordinary | 19.210 | ordinary | +0.00% (0/4) | -0.89% (3/4) | -36.93% (4/4) | 361.3 | 0/4 |
| v156 | vector | 19.230 | ordinary | -0.36% (2/4) | -0.34% (2/4) | -37.16% (4/4) | 350.7 | 1/4 |
| v156 | aeglos | 30.569 | aeglos | +0.00% (0/4) | +58.76% (0/4) | +0.00% (0/4) | 2459.8 | 0/4 |

## Stage clocks

Three paired process blocks per primary input separately clock the conversion pass. The phase measures only the scalar decoding loop. Full-proof time captures the effects of concurrent curve work.

| Input | Profile | Proof ms | Paired clock effect | Classes | Decode µs |
|---|---|---:|---|---:|---:|
| catalog | ordinary | 18.950 | +0.40% (1/3) | 38,075 | 765.3 |
| catalog | vector | 18.833 | +1.28% (1/3) | 38,075 | 176.1 |
| heldout | ordinary | 19.239 | +0.48% (1/3) | 39,891 | 797.0 |
| heldout | vector | 19.022 | -0.34% (2/3) | 39,891 | 184.8 |

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
