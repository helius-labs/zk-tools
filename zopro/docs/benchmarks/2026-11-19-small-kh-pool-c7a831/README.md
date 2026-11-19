# Fused K/H pool placement on C7a

The [Ryzen trial](../2026-11-19-small-kh-pool-ryzen830/README.md) moved fused K/H onto the frontend. The split arm keeps G1 A on the main pool. The shared arm retains its ordinary frontend placement. The C7a panel also tests the current solver definite-assignment option for skipping buffer initialization, with the ordinary and split query schedules. All modes preserve the existing curve arithmetic.

1×2 uses 80 main and 16 frontend physical cores. 2×2 uses 64 and 32 with medium fused K/H. Prepared Aeglos uses all 96 cores. Five matched blocks per input measure thirteen calls after three primes. All 152 exported proofs independently verified, including zero optional-workspace probes. The temporary instance and storage were removed.

- 1×2 `ordinary-uninit` took 8.129 / 8.064 ms across original and held-out, paired +3.98% / +3.91% from the same-build ordinary arm.
- 2×2 `ordinary-uninit` took 12.187 / 12.234 ms across original and held-out, paired +0.44% / +1.45% from the same-build ordinary arm.

No trial mode beat both controls consistently across its tested inputs.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 7.789 | -1.08% (3/5) | +0.00% (0/5) | -1.88% (4/5) | 42.2 |
| 1×2 | catalog | ordinary | 7.817 | +0.00% (0/5) | +1.09% (2/5) | -1.95% (4/5) | 39.2 |
| 1×2 | catalog | split | 10.155 | +30.15% (0/5) | +30.38% (0/5) | +27.15% (0/5) | 41.4 |
| 1×2 | catalog | shared | 9.980 | +27.94% (0/5) | +28.06% (0/5) | +25.12% (0/5) | 40.6 |
| 1×2 | catalog | ordinary-uninit | 8.129 | +3.98% (0/5) | +5.23% (1/5) | +2.03% (0/5) | 40.8 |
| 1×2 | catalog | split-uninit | 10.513 | +34.24% (0/5) | +35.04% (0/5) | +31.63% (0/5) | 37.7 |
| 1×2 | catalog | aeglos | 7.975 | +1.99% (1/5) | +1.92% (1/5) | +0.00% (0/5) | 456.7 |
| 1×2 | heldout | release | 7.773 | -0.34% (4/5) | +0.00% (0/5) | -3.75% (5/5) | 38.7 |
| 1×2 | heldout | ordinary | 7.791 | +0.00% (0/5) | +0.34% (1/5) | -2.90% (5/5) | 40.7 |
| 1×2 | heldout | split | 9.931 | +27.16% (0/5) | +28.11% (0/5) | +23.48% (0/5) | 39.9 |
| 1×2 | heldout | shared | 9.763 | +25.45% (0/5) | +25.66% (0/5) | +21.19% (0/5) | 37.7 |
| 1×2 | heldout | ordinary-uninit | 8.064 | +3.91% (0/5) | +4.16% (0/5) | +0.26% (1/5) | 37.1 |
| 1×2 | heldout | split-uninit | 10.230 | +31.32% (0/5) | +31.77% (0/5) | +26.46% (0/5) | 38.4 |
| 1×2 | heldout | aeglos | 8.045 | +2.98% (0/5) | +3.89% (0/5) | +0.00% (0/5) | 456.6 |
| 2×2 | catalog | release | 12.125 | +0.29% (2/5) | +0.00% (0/5) | +3.84% (0/5) | 71.5 |
| 2×2 | catalog | ordinary | 12.127 | +0.00% (0/5) | -0.29% (3/5) | +5.07% (0/5) | 71.5 |
| 2×2 | catalog | split | 12.209 | +0.61% (1/5) | +0.30% (0/5) | +4.15% (0/5) | 79.7 |
| 2×2 | catalog | shared | 12.592 | +4.17% (0/5) | +4.25% (0/5) | +8.40% (0/5) | 70.6 |
| 2×2 | catalog | ordinary-uninit | 12.187 | +0.44% (1/5) | +0.60% (1/5) | +4.99% (0/5) | 71.5 |
| 2×2 | catalog | split-uninit | 12.304 | +1.02% (0/5) | +0.70% (0/5) | +5.92% (0/5) | 78.2 |
| 2×2 | catalog | aeglos | 11.616 | -4.83% (5/5) | -3.70% (5/5) | +0.00% (0/5) | 810.6 |
| 2×2 | heldout | release | 12.062 | -0.44% (3/5) | +0.00% (0/5) | +4.07% (0/5) | 74.4 |
| 2×2 | heldout | ordinary | 12.086 | +0.00% (0/5) | +0.45% (2/5) | +4.54% (0/5) | 71.6 |
| 2×2 | heldout | split | 12.246 | +1.60% (0/5) | +1.53% (2/5) | +4.93% (0/5) | 82.7 |
| 2×2 | heldout | shared | 12.757 | +5.41% (0/5) | +5.15% (0/5) | +9.48% (0/5) | 74.5 |
| 2×2 | heldout | ordinary-uninit | 12.234 | +1.45% (0/5) | +1.32% (1/5) | +4.57% (0/5) | 73.7 |
| 2×2 | heldout | split-uninit | 12.388 | +2.33% (0/5) | +2.66% (0/5) | +6.80% (0/5) | 76.7 |
| 2×2 | heldout | aeglos | 11.581 | -4.34% (5/5) | -3.91% (5/5) | +0.00% (0/5) | 806.2 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 828 frontend solver full | 15.103 | 109 / 48 | 7.22× / 3.18× | +4.203 |
| 8×2 | 826 released SDK 72+24 packed frontend | 19.499 | 189 / 74 | 9.69× / 3.79× | +0.599 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
