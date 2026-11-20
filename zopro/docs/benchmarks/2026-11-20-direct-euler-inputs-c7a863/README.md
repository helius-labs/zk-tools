# Mixed-radix Euler FFT on 96 C7a cores

The [local direct-load trial](../2026-11-20-direct-euler-inputs-ryzen862/README.md) measured a shorter 4×2 quotient on both Ryzen inputs, with mixed full-proof comparisons. This C7a follow-up checks the quotient and complete proof on the target host. The original R260 implementation, the first mixed-radix build and the mixed-radix build with direct loads all use the same seeded solver, packed FFT settings, and 64 main and 32 frontend physical cores. The accepted SDK and warm prepared Aeglos provide comparisons.

The two primary inputs have six matched process blocks and the additional inputs have four. Each process primes three proofs and times thirteen. All 116 exports independently verified. Separate probes cover direct mixed-radix dispatch and unfused fallback. The temporary instance and storage were removed.

The fastest trial median on the original input was 15.125 ms with `mixed`. It took 15.071 ms on the held-out input. No configuration improved both controls consistently across all four inputs.

| Input | Profile | Proof ms | Matched control | Paired vs control | Paired vs release | Paired vs first mixed-radix build | Paired vs Aeglos | Peak MiB | Blocks at or below 10.9 ms |
|---|---|---:|---|---:|---:|---:|---:|---:|---:|
| catalog | release | 15.002 | ordinary | +0.20% (3/6) | +0.00% (0/6) | -0.49% (4/6) | -19.11% (6/6) | 195.2 | 0/6 |
| catalog | ordinary | 15.064 | ordinary | +0.00% (0/6) | -0.20% (3/6) | -0.21% (3/6) | -18.64% (6/6) | 201.1 | 0/6 |
| catalog | previous | 15.178 | ordinary | +0.24% (3/6) | +0.49% (2/6) | +0.00% (0/6) | -18.87% (6/6) | 201.3 | 0/6 |
| catalog | mixed | 15.125 | ordinary | -0.86% (4/6) | -0.21% (3/6) | +0.22% (3/6) | -19.25% (6/6) | 199.7 | 0/6 |
| catalog | aeglos | 18.650 | ordinary | +22.92% (0/6) | +23.63% (0/6) | +23.26% (0/6) | +0.00% (0/6) | 1360.2 | 0/6 |
| heldout | release | 15.145 | ordinary | -1.46% (4/6) | +0.00% (0/6) | +1.13% (2/6) | -19.21% (6/6) | 199.7 | 0/6 |
| heldout | ordinary | 15.239 | ordinary | +0.00% (0/6) | +1.49% (2/6) | +2.36% (1/6) | -18.21% (6/6) | 209.5 | 0/6 |
| heldout | previous | 14.947 | ordinary | -2.27% (5/6) | -1.12% (4/6) | +0.00% (0/6) | -20.55% (6/6) | 195.2 | 0/6 |
| heldout | mixed | 15.071 | ordinary | -0.62% (3/6) | -0.49% (4/6) | +0.67% (2/6) | -19.98% (6/6) | 195.9 | 0/6 |
| heldout | aeglos | 18.859 | ordinary | +22.27% (0/6) | +23.78% (0/6) | +25.86% (0/6) | +0.00% (0/6) | 1351.2 | 0/6 |
| v153 | release | 15.118 | ordinary | +0.03% (1/4) | +0.00% (0/4) | +0.08% (2/4) | -20.17% (4/4) | 197.5 | 0/4 |
| v153 | ordinary | 15.251 | ordinary | +0.00% (0/4) | -0.03% (3/4) | +1.01% (1/4) | -19.59% (4/4) | 198.1 | 0/4 |
| v153 | previous | 15.130 | ordinary | -1.00% (3/4) | -0.06% (2/4) | +0.00% (0/4) | -18.91% (4/4) | 197.5 | 0/4 |
| v153 | mixed | 15.060 | ordinary | -1.25% (2/4) | -0.83% (2/4) | -0.00% (2/4) | -19.61% (4/4) | 197.3 | 0/4 |
| v153 | aeglos | 18.824 | ordinary | +24.38% (0/4) | +25.27% (0/4) | +23.33% (0/4) | +0.00% (0/4) | 1343.5 | 0/4 |
| v156 | release | 15.152 | ordinary | -0.75% (2/4) | +0.00% (0/4) | -0.13% (3/4) | -18.93% (4/4) | 196.7 | 0/4 |
| v156 | ordinary | 15.227 | ordinary | +0.00% (0/4) | +0.77% (2/4) | +0.26% (2/4) | -19.37% (4/4) | 197.3 | 0/4 |
| v156 | previous | 15.106 | ordinary | -0.25% (2/4) | +0.13% (1/4) | +0.00% (0/4) | -20.35% (4/4) | 200.5 | 0/4 |
| v156 | mixed | 15.121 | ordinary | +0.67% (1/4) | -0.70% (3/4) | -0.24% (3/4) | -19.40% (4/4) | 193.7 | 0/4 |
| v156 | aeglos | 18.833 | ordinary | +24.03% (0/4) | +23.37% (0/4) | +25.55% (0/4) | +0.00% (0/4) | 1343.7 | 0/4 |

## Quotient phase timing

Three separate matched blocks on each primary input time the packed Euler FFT and fused product. Clocked proofs have an unclocked companion in the same block.

| Input | Profile | Quotient ms | Full proof with clock ms | Paired clock effect |
|---|---|---:|---:|---:|
| catalog | ordinary | 2.043 | 15.090 | -0.25% (2/3) |
| catalog | mixed | 2.002 | 15.146 | -0.50% (2/3) |
| heldout | ordinary | 2.042 | 15.053 | -0.14% (2/3) |
| heldout | mixed | 2.020 | 15.317 | +1.33% (1/3) |

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
