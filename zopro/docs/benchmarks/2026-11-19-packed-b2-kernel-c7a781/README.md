# Curve kernel selection for the packed B2 prefix

The [preceding packed image study](../2026-11-19-packed-b2-equality-c7a778/README.md) found that packing 8×2 helped on Ryzen and regressed on C7a. The packed prefix has 63,474 terms, which takes a different curve kernel from the ordinary 183,347-term input. This comparison keeps the exact packed image, residual detection, scalar conversion and correction calculations fixed. Only the sparse prefix kernel changes in the same executable. The ordinary-kernel arm uses the grouping and final MSM used by the larger ordinary B2 route.

One c7a.24xlarge runs six blocks on each of four inputs. Zopro uses 64 main and 32 frontend cores with packed solver tasks, fused FFT and late frontend K. Prepared Aeglos runs on all 96 cores. Each process discarded three priming requests and timed thirteen proofs. Separate route checks exercise the dense calculation and zero optional-memory fallback. All 156 AWS and six local exports independently verified against the original key image. The AWS instance and temporary storage were removed.

The earlier scorecard remains the fastest qualifying comparison.

| Input | Profile | Proof ms | Paired vs release | Paired vs ordinary same build | Paired vs origins | Paired vs packed Phi | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | aeglos | 30.240 | +48.24% (0/6) | +48.18% (0/6) | +50.47% (0/6) | +34.77% (0/6) | +0.00% (0/6) | 2462.9 | 6.25× / 2.45× |
| catalog | ordinary | 20.404 | +0.00% (3/6) | +0.00% (0/6) | +1.27% (1/6) | -8.76% (6/6) | -32.51% (6/6) | 408.4 | 9.26× / 3.63× |
| catalog | origin | 20.098 | -1.48% (4/6) | -1.26% (5/6) | +0.00% (0/6) | -9.90% (6/6) | -33.54% (6/6) | 406.9 | 9.40× / 3.68× |
| catalog | packed-eq | 22.327 | +9.69% (0/6) | +9.60% (0/6) | +10.99% (0/6) | +0.00% (0/6) | -25.80% (6/6) | 408.3 | 8.47× / 3.31× |
| catalog | packed-eq-raw | 21.653 | +5.49% (0/6) | +6.21% (0/6) | +7.41% (0/6) | -3.45% (6/6) | -28.83% (6/6) | 411.4 | 8.73× / 3.42× |
| catalog | release | 20.399 | +0.00% (0/6) | -0.00% (3/6) | +1.50% (2/6) | -8.83% (6/6) | -32.54% (6/6) | 404.5 | 9.27× / 3.63× |
| heldout | aeglos | 30.478 | +47.65% (0/6) | +46.59% (0/6) | +48.49% (0/6) | +33.24% (0/6) | +0.00% (0/6) | 2496.3 | — |
| heldout | ordinary | 20.720 | +0.75% (2/6) | +0.00% (0/6) | +1.34% (0/6) | -8.94% (6/6) | -31.78% (6/6) | 407.0 | — |
| heldout | origin | 20.353 | -1.56% (6/6) | -1.32% (6/6) | +0.00% (0/6) | -11.49% (6/6) | -32.66% (6/6) | 408.4 | — |
| heldout | packed-eq | 22.902 | +11.22% (0/6) | +9.82% (0/6) | +12.98% (0/6) | +0.00% (0/6) | -24.95% (6/6) | 407.7 | — |
| heldout | packed-eq-raw | 21.612 | +4.60% (0/6) | +3.69% (0/6) | +6.04% (0/6) | -6.46% (6/6) | -29.14% (6/6) | 416.5 | — |
| heldout | release | 20.657 | +0.00% (0/6) | -0.74% (4/6) | +1.58% (0/6) | -10.08% (6/6) | -32.27% (6/6) | 411.7 | — |
| v153 | aeglos | 30.455 | +47.74% (0/6) | +48.10% (0/6) | +48.89% (0/6) | +34.17% (0/6) | +0.00% (0/6) | 2461.3 | — |
| v153 | ordinary | 20.650 | +0.19% (3/6) | +0.00% (0/6) | +0.51% (1/6) | -8.56% (6/6) | -32.48% (6/6) | 434.7 | — |
| v153 | origin | 20.507 | -0.66% (5/6) | -0.51% (5/6) | +0.00% (0/6) | -10.44% (6/6) | -32.84% (6/6) | 413.7 | — |
| v153 | packed-eq | 22.674 | +10.50% (0/6) | +9.39% (0/6) | +11.65% (0/6) | +0.00% (0/6) | -25.43% (6/6) | 408.0 | — |
| v153 | packed-eq-raw | 20.945 | +1.65% (2/6) | +1.03% (2/6) | +2.37% (0/6) | -8.32% (6/6) | -31.40% (6/6) | 416.5 | — |
| v153 | release | 20.592 | +0.00% (0/6) | -0.19% (3/6) | +0.66% (1/6) | -9.49% (6/6) | -32.31% (6/6) | 415.1 | — |
| v156 | aeglos | 30.618 | +48.90% (0/6) | +49.57% (0/6) | +50.25% (0/6) | +31.11% (0/6) | +0.00% (0/6) | 2464.7 | — |
| v156 | ordinary | 20.558 | -0.59% (4/6) | +0.00% (0/6) | +1.01% (1/6) | -11.30% (6/6) | -33.14% (6/6) | 426.3 | — |
| v156 | origin | 20.295 | -1.50% (5/6) | -1.00% (5/6) | +0.00% (0/6) | -12.02% (6/6) | -33.44% (6/6) | 415.2 | — |
| v156 | packed-eq | 23.233 | +13.38% (0/6) | +12.74% (0/6) | +13.66% (0/6) | +0.00% (0/6) | -23.73% (6/6) | 409.8 | — |
| v156 | packed-eq-raw | 21.677 | +5.14% (0/6) | +5.79% (0/6) | +5.98% (0/6) | -6.88% (6/6) | -29.65% (6/6) | 415.1 | — |
| v156 | release | 20.506 | +0.00% (0/6) | +0.60% (2/6) | +1.53% (1/6) | -11.79% (6/6) | -32.84% (6/6) | 416.1 | — |

[Samples and route checks](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Packed source](artifacts/source.patch) · [Kernel switch](artifacts/kernel.patch) · [Image layout](artifacts/image-layout.json) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU. [Preceding scorecard](../2026-11-19-packed-b2-equality-c7a778/README.md)

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
