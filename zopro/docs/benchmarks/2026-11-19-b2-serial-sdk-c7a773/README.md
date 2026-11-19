# Grouped B2 serial preparation in the CPU SDK

The candidate exposes the scalar preparation policy measured in [the classification screen](../2026-11-19-b2-class-phases-c7a771/README.md) as a per-key opt-in. It uses native serial conversion and requests one hash-table slot per three B2 terms. It applies to the ordinary grouped B2 path with 65,536–262,143 terms when B1 is omitted. The workspace policy still governs admission, and a denied reservation uses the ordinary fallback. The key retains no new table.

One c7a.24xlarge ran the released executable, its archived tuning trial, the new SDK with the option enabled and disabled, and prepared Aeglos. Zopro uses 64 main and 32 frontend cores, plus a paired 56+40 comparison for the two primary 8×2 transactions. It keeps the existing K and fused FFT settings, with packed solver work on 8×2 and the frontend solver on 4×2. Aeglos runs on all 96 cores.

The native core suite and generic example build passed. All 188 C7a and six Ryzen exports independently verified, including the zero optional-preparation fallback. Each timed process used three priming and thirteen measured proofs. The instance and temporary storage were removed.

The SDK option is available on Linux x86-64 and is disabled by default. It beat the released and same-build controls on both primary 8×2 inputs.

| Shape | Input | Profile | Proof ms | Paired vs same-width disabled | Paired vs release | Paired vs archive | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|
| 4×2 | catalog | aeglos | 18.625 | +19.34% (0/4) | +18.79% (0/4) | +21.21% (0/4) | +0.00% (0/4) | 1346.8 | 5.85× / 2.58× |
| 4×2 | catalog | archive | 15.452 | -0.65% (2/4) | -1.41% (3/4) | +0.00% (0/4) | -17.50% (4/4) | 200.4 | 7.05× / 3.11× |
| 4×2 | catalog | off | 15.483 | +0.00% (0/4) | -0.50% (4/4) | +0.67% (2/4) | -16.20% (4/4) | 199.1 | 7.04× / 3.10× |
| 4×2 | catalog | on | 15.480 | -0.48% (3/4) | -1.24% (3/4) | +0.41% (2/4) | -16.73% (4/4) | 207.9 | 7.04× / 3.10× |
| 4×2 | catalog | release | 15.598 | +0.50% (0/4) | +0.00% (0/4) | +1.43% (1/4) | -15.82% (4/4) | 207.1 | 6.99× / 3.08× |
| 4×2 | heldout | aeglos | 18.206 | +17.76% (0/4) | +17.48% (0/4) | +19.12% (0/4) | +0.00% (0/4) | 1362.8 | — |
| 4×2 | heldout | archive | 15.299 | -1.87% (4/4) | -1.40% (4/4) | +0.00% (0/4) | -16.05% (4/4) | 208.7 | — |
| 4×2 | heldout | off | 15.582 | +0.00% (0/4) | +0.59% (2/4) | +1.91% (0/4) | -15.08% (4/4) | 213.1 | — |
| 4×2 | heldout | on | 15.168 | -2.56% (4/4) | -2.14% (4/4) | -0.78% (4/4) | -16.75% (4/4) | 209.4 | — |
| 4×2 | heldout | release | 15.500 | -0.58% (2/4) | +0.00% (0/4) | +1.42% (0/4) | -14.88% (4/4) | 201.3 | — |
| 8×2 | catalog | aeglos | 31.068 | +49.16% (0/6) | +49.31% (0/6) | +51.05% (0/6) | +0.00% (0/6) | 2455.0 | 6.08× / 2.38× |
| 8×2 | catalog | archive | 20.564 | -1.28% (5/6) | -1.04% (6/6) | +0.00% (0/6) | -33.80% (6/6) | 413.6 | 9.19× / 3.60× |
| 8×2 | catalog | off | 20.712 | +0.00% (0/6) | -0.89% (4/6) | +1.30% (1/6) | -32.96% (6/6) | 424.2 | 9.13× / 3.57× |
| 8×2 | catalog | off56 | 21.034 | +0.00% (0/4) | +0.92% (1/4) | +2.11% (0/4) | -32.84% (4/4) | 429.4 | 8.99× / 3.52× |
| 8×2 | catalog | on | 20.569 | -0.67% (4/6) | -1.53% (5/6) | -0.02% (3/6) | -34.05% (6/6) | 408.3 | 9.19× / 3.60× |
| 8×2 | catalog | on56 | 20.875 | -0.77% (3/4) | -0.39% (3/4) | +1.49% (0/4) | -33.18% (4/4) | 423.3 | 9.05× / 3.54× |
| 8×2 | catalog | release | 20.784 | +0.90% (2/6) | +0.00% (0/6) | +1.05% (0/6) | -33.03% (6/6) | 411.6 | 9.09× / 3.56× |
| 8×2 | heldout | aeglos | 31.068 | +47.86% (0/6) | +46.66% (0/6) | +49.13% (0/6) | +0.00% (0/6) | 2490.5 | — |
| 8×2 | heldout | archive | 20.812 | -1.03% (4/6) | -1.01% (4/6) | +0.00% (0/6) | -32.94% (6/6) | 412.2 | — |
| 8×2 | heldout | off | 21.172 | +0.00% (0/6) | +0.45% (2/6) | +1.04% (2/6) | -32.37% (6/6) | 416.7 | — |
| 8×2 | heldout | off56 | 21.259 | +0.00% (0/4) | +1.41% (1/4) | +1.07% (0/4) | -31.54% (4/4) | 402.3 | — |
| 8×2 | heldout | on | 20.709 | -2.24% (5/6) | -1.73% (4/6) | -0.85% (4/6) | -33.54% (6/6) | 417.0 | — |
| 8×2 | heldout | on56 | 20.951 | -1.03% (3/4) | -0.63% (2/4) | +0.54% (1/4) | -32.25% (4/4) | 421.1 | — |
| 8×2 | heldout | release | 20.946 | -0.45% (4/6) | +0.00% (0/6) | +1.02% (2/6) | -31.82% (6/6) | 424.3 | — |
| 8×2 | v153 | aeglos | 31.123 | +48.10% (0/6) | +48.35% (0/6) | +48.98% (0/6) | +0.00% (0/6) | 2461.2 | — |
| 8×2 | v153 | archive | 20.729 | -0.63% (5/6) | -0.83% (4/6) | +0.00% (0/6) | -32.88% (6/6) | 408.4 | — |
| 8×2 | v153 | off | 21.048 | +0.00% (0/6) | -0.19% (4/6) | +0.63% (1/6) | -32.47% (6/6) | 422.7 | — |
| 8×2 | v153 | on | 20.645 | -1.07% (6/6) | -1.36% (6/6) | -0.57% (5/6) | -33.51% (6/6) | 413.0 | — |
| 8×2 | v153 | release | 20.991 | +0.19% (2/6) | +0.00% (0/6) | +0.84% (2/6) | -32.59% (6/6) | 412.1 | — |
| 8×2 | v156 | aeglos | 30.906 | +49.36% (0/6) | +46.58% (0/6) | +48.83% (0/6) | +0.00% (0/6) | 2461.1 | — |
| 8×2 | v156 | archive | 20.766 | -0.03% (3/6) | -1.21% (6/6) | +0.00% (0/6) | -32.81% (6/6) | 406.7 | — |
| 8×2 | v156 | off | 20.754 | +0.00% (0/6) | -1.25% (6/6) | +0.03% (3/6) | -33.05% (6/6) | 410.6 | — |
| 8×2 | v156 | on | 20.895 | +0.30% (2/6) | -0.60% (5/6) | +0.01% (3/6) | -31.82% (6/6) | 417.3 | — |
| 8×2 | v156 | release | 21.027 | +1.26% (0/6) | +0.00% (0/6) | +1.23% (0/6) | -31.78% (6/6) | 420.4 | — |

[Proof samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU on measured C7a profiles.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 737 dedicated G1 80+16 | 7.953 | 37 / 24 | 4.65× / 3.02× | +4.253 |
| 2×2 | 750 archived-band 72+24 | 12.358 | 59 / 30 | 4.77× / 2.43× | +6.458 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 771 B2 classification reserve3-serial | 20.510 | 189 / 74 | 9.22× / 3.61× | +1.610 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
