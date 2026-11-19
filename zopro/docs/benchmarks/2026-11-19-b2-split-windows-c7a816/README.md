# Grouped G2 windows across both C7a pools

The [Ryzen trial](../2026-11-19-b2-split-windows-ryzen815/README.md) distributes some of the grouped B2 query's unsigned windows to the existing frontend pool. Point grouping stays on the main pool. Scratch for the additional workers is charged to the optional workspace. The trial uses fresh inputs for every proof.

Zopro uses 64 main and 32 frontend physical cores on one c7a.24xlarge. 4×2 enables the frontend solver. 8×2 uses recognized Poseidon origins and serial scalar conversion. Prepared Aeglos uses all 96 cores. There are four paired blocks per 4×2 input and six per primary 8×2 input, with four per additional 8×2 input. Each process primes three calls and measures thirteen. All 204 exported proofs independently verified, including the second-pool and zero optional-memory routing checks. The temporary instance and storage were removed.

- 4×2 `front-4` took 16.169 / 16.465 ms across original and held-out, paired +5.05% / +5.97% from the same-build ordinary arm.
- 8×2 `front-4` took 20.627 / 20.771 ms across original and held-out, paired +2.09% / +3.04% from the same-build ordinary arm.

No split improved both controls consistently across the tested inputs. The source is experimental.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 4×2 | catalog | release | 15.313 | -0.49% (4/4) | +0.00% (0/4) | -15.36% (4/4) | 202.2 |
| 4×2 | catalog | ordinary | 15.389 | +0.00% (0/4) | +0.50% (0/4) | -14.81% (4/4) | 202.6 |
| 4×2 | catalog | front-4 | 16.169 | +5.05% (0/4) | +5.30% (0/4) | -10.49% (4/4) | 201.9 |
| 4×2 | catalog | front-8 | 16.466 | +6.94% (0/4) | +7.19% (0/4) | -9.21% (4/4) | 199.7 |
| 4×2 | catalog | front-12 | 16.729 | +8.32% (0/4) | +8.67% (0/4) | -7.48% (4/4) | 220.6 |
| 4×2 | catalog | front-16 | 16.940 | +10.09% (0/4) | +10.37% (0/4) | -6.48% (4/4) | 204.2 |
| 4×2 | catalog | aeglos | 18.053 | +17.38% (0/4) | +18.15% (0/4) | +0.00% (0/4) | 1342.1 |
| 4×2 | heldout | release | 15.308 | -0.85% (2/4) | +0.00% (0/4) | -15.14% (4/4) | 207.9 |
| 4×2 | heldout | ordinary | 15.485 | +0.00% (0/4) | +0.87% (2/4) | -14.92% (4/4) | 202.0 |
| 4×2 | heldout | front-4 | 16.465 | +5.97% (0/4) | +6.93% (0/4) | -9.38% (4/4) | 194.5 |
| 4×2 | heldout | front-8 | 16.649 | +7.84% (0/4) | +8.71% (0/4) | -7.99% (4/4) | 204.9 |
| 4×2 | heldout | front-12 | 16.802 | +8.49% (0/4) | +10.01% (0/4) | -7.32% (4/4) | 215.4 |
| 4×2 | heldout | front-16 | 16.968 | +9.90% (0/4) | +10.85% (0/4) | -6.63% (4/4) | 195.2 |
| 4×2 | heldout | aeglos | 18.173 | +17.54% (0/4) | +17.84% (0/4) | +0.00% (0/4) | 1347.2 |
| 8×2 | catalog | release | 20.274 | +0.45% (1/6) | +0.00% (0/6) | -34.20% (6/6) | 419.6 |
| 8×2 | catalog | ordinary | 20.162 | +0.00% (0/6) | -0.45% (5/6) | -34.48% (6/6) | 406.9 |
| 8×2 | catalog | front-4 | 20.627 | +2.09% (0/6) | +1.73% (0/6) | -33.17% (6/6) | 414.5 |
| 8×2 | catalog | front-8 | 21.441 | +6.31% (0/6) | +6.20% (0/6) | -29.99% (6/6) | 413.8 |
| 8×2 | catalog | front-12 | 21.423 | +6.56% (0/6) | +6.41% (0/6) | -29.91% (6/6) | 411.6 |
| 8×2 | catalog | front-16 | 21.823 | +7.69% (0/6) | +7.56% (0/6) | -28.17% (6/6) | 406.0 |
| 8×2 | catalog | aeglos | 30.713 | +52.63% (0/6) | +51.97% (0/6) | +0.00% (0/6) | 2459.6 |
| 8×2 | heldout | release | 20.516 | +1.21% (2/6) | +0.00% (0/6) | -32.44% (6/6) | 408.7 |
| 8×2 | heldout | ordinary | 20.353 | +0.00% (0/6) | -1.20% (4/6) | -33.24% (6/6) | 414.5 |
| 8×2 | heldout | front-4 | 20.771 | +3.04% (0/6) | +1.41% (0/6) | -31.55% (6/6) | 411.4 |
| 8×2 | heldout | front-8 | 22.042 | +7.89% (0/6) | +7.84% (0/6) | -27.81% (6/6) | 424.2 |
| 8×2 | heldout | front-12 | 21.730 | +7.62% (0/6) | +5.92% (0/6) | -28.18% (6/6) | 421.3 |
| 8×2 | heldout | front-16 | 22.058 | +8.63% (0/6) | +7.52% (0/6) | -27.83% (6/6) | 413.0 |
| 8×2 | heldout | aeglos | 30.382 | +49.78% (0/6) | +48.01% (0/6) | +0.00% (0/6) | 2460.1 |
| 8×2 | v153 | release | 20.612 | +0.38% (1/4) | +0.00% (0/4) | -32.59% (4/4) | 403.2 |
| 8×2 | v153 | ordinary | 20.428 | +0.00% (0/4) | -0.38% (3/4) | -32.96% (4/4) | 424.2 |
| 8×2 | v153 | front-4 | 20.889 | +2.35% (0/4) | +1.35% (1/4) | -31.36% (4/4) | 407.5 |
| 8×2 | v153 | front-8 | 21.798 | +6.65% (0/4) | +5.95% (0/4) | -28.17% (4/4) | 413.9 |
| 8×2 | v153 | front-12 | 21.645 | +5.86% (0/4) | +5.42% (0/4) | -29.03% (4/4) | 403.1 |
| 8×2 | v153 | front-16 | 22.043 | +8.19% (0/4) | +7.06% (0/4) | -27.44% (4/4) | 404.7 |
| 8×2 | v153 | aeglos | 30.373 | +49.16% (0/4) | +48.35% (0/4) | +0.00% (0/4) | 2460.8 |
| 8×2 | v156 | release | 20.466 | -0.07% (2/4) | +0.00% (0/4) | -32.42% (4/4) | 409.3 |
| 8×2 | v156 | ordinary | 20.521 | +0.00% (0/4) | +0.08% (2/4) | -32.08% (4/4) | 402.4 |
| 8×2 | v156 | front-4 | 20.929 | +1.82% (0/4) | +2.11% (0/4) | -30.66% (4/4) | 415.8 |
| 8×2 | v156 | front-8 | 21.686 | +5.31% (0/4) | +5.73% (0/4) | -28.24% (4/4) | 403.3 |
| 8×2 | v156 | front-12 | 21.798 | +6.05% (0/4) | +6.47% (0/4) | -27.69% (4/4) | 430.3 |
| 8×2 | v156 | front-16 | 21.988 | +7.30% (0/4) | +7.31% (0/4) | -27.12% (4/4) | 408.4 |
| 8×2 | v156 | aeglos | 30.138 | +47.23% (0/4) | +47.98% (0/4) | +0.00% (0/4) | 2457.5 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 781 recognized B2 origins replay | 20.098 | 189 / 74 | 9.40× / 3.68× | +1.198 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
