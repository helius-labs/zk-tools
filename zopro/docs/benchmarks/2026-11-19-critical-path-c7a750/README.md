# G2 bucket ownership and indexed FFT fusion on C7a

The 2×2 trial builds the ordinary buckets across both point ranges in one tree per band. It keeps the narrower top-window path and the dedicated 24-core G1 A pool. The original two-range path builds two bucket arrays per ordinary window and then merges them. Both trials use the same binary and 72 main cores. The 96-core Aeglos reference uses its prepared key.

The 12×2 trial applies the existing final-column FFT fusion to the indexed profile, with identical prepared groups, request wires and proof verification for the four FFT configurations. It compares fixed column stages and the ordinary stages with fusion enabled and disabled, using 64 main and 32 frontend cores. The archived indexed executable and current SDK with fusion provide additional references.

The numbers below are original / second for 2×2, and original / second / v139 / v58 for 12×2.

- 2×2 split bands took **12.348 / 12.352 ms**. Full bucket ownership at widths 128 and 256 took **12.409 / 12.622 ms** and **12.671 / 12.814 ms**. The paired changes relative to split bands were +1.19% / +3.33% and +2.84% / +4.26%, respectively. Aeglos took **11.741 / 11.743 ms**.
- 12×2 with fixed stages and fusion took **27.790 / 28.079 / 39.472 / 27.899 ms** (-1.32% / -1.01% / -1.20% / -1.68% versus the same binary with fusion disabled). With ordinary stages and fusion it took **28.088 / 27.989 / 39.776 / 28.197 ms** (-1.06% / -1.05% / -0.07% / -1.00%). The original fixed and ordinary fused configurations had 5/6 and 5/6 process averages at or below 28.4 ms.

The native G2 arithmetic test used signed and zero coefficients and identity and opposite bases with one and eight Rayon workers. The indexed FFT arithmetic test passed with both column policies. The 2×2 zero-byte preparation limit verified both requests with the bucket ownership path declined. The normal route probes confirmed the expected bucket or FFT path. Timed processes warmed three requests and measured thirteen. All 230 exports passed independent verification. The instance and temporary storage were removed.

| Shape | Input | Configuration | Full proof ms | Paired comparison (wins) | Paired vs Aeglos on 2×2 or SDK on 12×2 | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---|---:|---:|---:|
| 2x2 | catalog | aeglos | 11.741 | split-band, -4.68% (7/7) | +0.00% (0/7) | 810.3 | 5.03× / 2.56× |
| 2x2 | catalog | archived-band | 12.358 | split-band, -0.26% (4/7) | +5.79% (0/7) | 114.1 | 4.77× / 2.43× |
| 2x2 | catalog | owner-128 | 12.409 | split-band, +1.19% (2/7) | +6.08% (0/7) | 108.2 | 4.75× / 2.42× |
| 2x2 | catalog | owner-256 | 12.671 | split-band, +2.84% (0/7) | +7.89% (0/7) | 132.8 | 4.66× / 2.37× |
| 2x2 | catalog | split-band | 12.348 | split-band, +0.00% (0/7) | +4.90% (0/7) | 114.9 | 4.78× / 2.43× |
| 2x2 | heldout | aeglos | 11.743 | split-band, -4.93% (7/7) | +0.00% (0/7) | 807.9 | — |
| 2x2 | heldout | archived-band | 12.247 | split-band, -0.10% (4/7) | +4.38% (0/7) | 115.7 | — |
| 2x2 | heldout | owner-128 | 12.622 | split-band, +3.33% (0/7) | +7.57% (0/7) | 107.5 | — |
| 2x2 | heldout | owner-256 | 12.814 | split-band, +4.26% (1/7) | +9.05% (0/7) | 128.4 | — |
| 2x2 | heldout | split-band | 12.352 | split-band, +0.00% (0/7) | +5.19% (0/7) | 118.7 | — |
| 12x2 | catalog | fixed-off | 28.399 | index-fixed, +1.15% (2/6) | -4.61% (6/6) | 517.8 | 10.00× / 3.56× |
| 12x2 | catalog | fixed-on | 27.790 | fixed-off, -1.32% (5/6) | -6.47% (6/6) | 512.4 | 10.22× / 3.63× |
| 12x2 | catalog | index-fixed | 28.346 | index-fixed, +0.00% (0/6) | -4.44% (6/6) | 540.9 | 10.02× / 3.56× |
| 12x2 | catalog | plain-off | 28.348 | index-fixed, -0.30% (4/6) | -4.97% (6/6) | 514.9 | 10.02× / 3.56× |
| 12x2 | catalog | plain-on | 28.088 | plain-off, -1.06% (4/6) | -5.19% (6/6) | 532.0 | 10.11× / 3.60× |
| 12x2 | catalog | sdk-fused | 29.667 | index-fixed, +4.66% (0/6) | +0.00% (0/6) | 543.3 | 9.57× / 3.40× |
| 12x2 | heldout | fixed-off | 28.284 | index-fixed, +0.49% (3/6) | -4.11% (6/6) | 535.0 | — |
| 12x2 | heldout | fixed-on | 28.079 | fixed-off, -1.01% (4/6) | -6.33% (6/6) | 520.8 | — |
| 12x2 | heldout | index-fixed | 28.344 | index-fixed, +0.00% (0/6) | -5.18% (6/6) | 522.1 | — |
| 12x2 | heldout | plain-off | 28.369 | index-fixed, +0.11% (3/6) | -5.07% (6/6) | 531.2 | — |
| 12x2 | heldout | plain-on | 27.989 | plain-off, -1.05% (5/6) | -5.53% (6/6) | 513.9 | — |
| 12x2 | heldout | sdk-fused | 29.851 | index-fixed, +5.46% (0/6) | +0.00% (0/6) | 554.6 | — |
| 12x2 | v139 | fixed-off | 40.004 | index-fixed, +0.63% (3/6) | +1.04% (2/6) | 749.4 | — |
| 12x2 | v139 | fixed-on | 39.472 | fixed-off, -1.20% (5/6) | -0.89% (3/6) | 805.6 | — |
| 12x2 | v139 | index-fixed | 40.083 | index-fixed, +0.00% (0/6) | +0.31% (3/6) | 765.9 | — |
| 12x2 | v139 | plain-off | 39.749 | index-fixed, -1.04% (5/6) | -0.35% (4/6) | 759.2 | — |
| 12x2 | v139 | plain-on | 39.776 | plain-off, -0.07% (3/6) | -0.61% (3/6) | 727.7 | — |
| 12x2 | v139 | sdk-fused | 39.916 | index-fixed, -0.31% (3/6) | +0.00% (0/6) | 819.1 | — |
| 12x2 | v58 | fixed-off | 28.448 | index-fixed, +1.08% (2/6) | -5.04% (6/6) | 528.2 | — |
| 12x2 | v58 | fixed-on | 27.899 | fixed-off, -1.68% (5/6) | -5.92% (6/6) | 509.4 | — |
| 12x2 | v58 | index-fixed | 28.335 | index-fixed, +0.00% (0/6) | -5.73% (6/6) | 522.2 | — |
| 12x2 | v58 | plain-off | 28.328 | index-fixed, -0.28% (4/6) | -5.82% (6/6) | 520.7 | — |
| 12x2 | v58 | plain-on | 28.197 | plain-off, -1.00% (4/6) | -5.77% (6/6) | 518.5 | — |
| 12x2 | v58 | sdk-fused | 29.892 | index-fixed, +6.08% (0/6) | +0.00% (0/6) | 559.7 | — |

[Samples](artifacts/rows.json.gz) · [Exact workflow](artifacts/workflow.json.gz) · [Routes](artifacts/routes.json) · [G2 source](artifacts/band.patch) · [FFT source](artifacts/fft.patch) · [Proof verification](artifacts/verification.json)

## Eight supplied M5 reference timings

These C7a measurements cover all eight original reference transactions. **4/8** reach 10× versus M5 CPU. Profiles include the experimental 2×2 banded executable and 12×2 prepared point groups. Earlier results come from the [previous M5 panel](../2026-11-19-indexed-confirm743/README.md) and the [opt-in fusion measurements](../2026-11-19-paired-euler-c7a747/README.md).

| Shape | C7a profile | C7a ms | M5 CPU / Metal ms | Speedup CPU / Metal | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 737 dedicated G1 80+16 | 7.953 | 37 / 24 | 4.65× / 3.02× | +4.253 |
| 2×2 | 750 archived-band 72+24 | 12.358 | 59 / 30 | 4.77× / 2.43× | +6.458 |
| 4×2 | 714 catalog | 17.210 | 109 / 48 | 6.33× / 2.79× | +6.310 |
| 8×2 | 747 opt-in fusion 64+32 | 22.583 | 189 / 74 | 8.37× / 3.28× | +3.683 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
