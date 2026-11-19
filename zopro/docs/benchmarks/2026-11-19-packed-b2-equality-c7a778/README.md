# Mapped replacement B2 image on C7a

One c7a.24xlarge runs the ordinary image and the invertible B2 image for the same 8×2 key. The experimental image keeps the original 183,347 points and adds a 1,466,792-byte index trailer. The sparse route consumes a 63,474-point prefix plus the exact nonzero corrections. The dense route retains every term in the replacement basis and serves as the low-memory fallback. The equality arm counts mismatches by comparing complete scalars and computes field differences for the emitted residuals.

Zopro uses 64 main and 32 frontend cores with packed solver tasks, fused FFT and late frontend K. Controls include the currently released SDK, the same experimental executable on the ordinary image, and the archived origin classifier. Prepared Aeglos uses all 96 cores. Six matched blocks per input discard three warmups and time thirteen complete proofs. The two primary inputs include forced dense and zero optional-memory route checks. All 156 proof exports independently verified against the ordinary image and public witness. The temporary AWS resources were removed.

The earlier scorecard remains the fastest qualifying comparison.

| Input | Profile | Proof ms | Paired vs release | Paired vs ordinary same build | Paired vs origins | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | aeglos | 30.927 | +52.34% (0/6) | +52.02% (0/6) | +52.18% (0/6) | +0.00% (0/6) | 2461.8 | 6.11× / 2.39× |
| catalog | ordinary | 20.371 | +0.11% (2/6) | +0.00% (0/6) | +1.01% (1/6) | -34.22% (6/6) | 413.7 | 9.28× / 3.63× |
| catalog | origin | 20.220 | -1.00% (5/6) | -1.00% (5/6) | +0.00% (0/6) | -34.28% (6/6) | 405.2 | 9.35× / 3.66× |
| catalog | packed | 24.074 | +18.44% (0/6) | +18.17% (0/6) | +18.27% (0/6) | -22.52% (6/6) | 392.5 | 7.85× / 3.07× |
| catalog | packed-eq | 23.138 | +14.04% (0/6) | +13.58% (0/6) | +14.73% (0/6) | -25.07% (6/6) | 396.3 | 8.17× / 3.20× |
| catalog | release | 20.371 | +0.00% (0/6) | -0.11% (4/6) | +1.02% (1/6) | -34.36% (6/6) | 414.2 | 9.28× / 3.63× |
| heldout | aeglos | 30.929 | +50.06% (0/6) | +49.79% (0/6) | +50.34% (0/6) | +0.00% (0/6) | 2460.4 | — |
| heldout | ordinary | 20.573 | +0.08% (3/6) | +0.00% (0/6) | +0.13% (2/6) | -33.24% (6/6) | 415.0 | — |
| heldout | origin | 20.547 | -0.04% (3/6) | -0.13% (4/6) | +0.00% (0/6) | -33.48% (6/6) | 413.0 | — |
| heldout | packed | 24.034 | +16.67% (0/6) | +16.42% (0/6) | +16.42% (0/6) | -22.48% (6/6) | 418.1 | — |
| heldout | packed-eq | 23.262 | +13.34% (0/6) | +12.23% (0/6) | +13.21% (0/6) | -24.96% (6/6) | 395.7 | — |
| heldout | release | 20.628 | +0.00% (0/6) | -0.08% (3/6) | +0.04% (3/6) | -33.36% (6/6) | 409.1 | — |
| v153 | aeglos | 30.805 | +49.71% (0/6) | +49.61% (0/6) | +50.63% (0/6) | +0.00% (0/6) | 2464.4 | — |
| v153 | ordinary | 20.589 | -0.54% (5/6) | +0.00% (0/6) | +0.29% (2/6) | -33.16% (6/6) | 415.8 | — |
| v153 | origin | 20.561 | -0.66% (4/6) | -0.29% (4/6) | +0.00% (0/6) | -33.61% (6/6) | 395.9 | — |
| v153 | packed | 23.761 | +14.69% (0/6) | +15.68% (0/6) | +15.79% (0/6) | -22.68% (6/6) | 405.4 | — |
| v153 | packed-eq | 23.040 | +11.30% (0/6) | +11.90% (0/6) | +12.04% (0/6) | -25.30% (6/6) | 412.8 | — |
| v153 | release | 20.702 | +0.00% (0/6) | +0.55% (1/6) | +0.67% (2/6) | -33.20% (6/6) | 417.4 | — |
| v156 | aeglos | 30.896 | +50.34% (0/6) | +50.62% (0/6) | +49.45% (0/6) | +0.00% (0/6) | 2501.3 | — |
| v156 | ordinary | 20.595 | +0.24% (2/6) | +0.00% (0/6) | +0.14% (3/6) | -33.61% (6/6) | 399.2 | — |
| v156 | origin | 20.684 | -0.18% (3/6) | -0.14% (3/6) | +0.00% (0/6) | -33.09% (6/6) | 418.4 | — |
| v156 | packed | 23.881 | +16.16% (0/6) | +15.85% (0/6) | +16.50% (0/6) | -22.87% (6/6) | 408.4 | — |
| v156 | packed-eq | 23.058 | +12.51% (0/6) | +12.14% (0/6) | +11.86% (0/6) | -25.55% (6/6) | 415.8 | — |
| v156 | release | 20.569 | +0.00% (0/6) | -0.24% (4/6) | +0.18% (3/6) | -33.48% (6/6) | 418.8 | — |

[Samples and route checks](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Image layout](artifacts/image-layout.json) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU. [Preceding scorecard](../2026-11-19-b2-origin-clock-c7a776/README.md)

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 737 dedicated G1 80+16 | 7.953 | 37 / 24 | 4.65× / 3.02× | +4.253 |
| 2×2 | 750 archived-band 72+24 | 12.358 | 59 / 30 | 4.77× / 2.43× | +6.458 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 776 recognized B2 origins | 20.341 | 189 / 74 | 9.29× / 3.64× | +1.441 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
