# Per-key G1 K placement in the CPU SDK

The build exposes a per-key option for the ordinary large CPU MSM path. It keeps the same G1 K arithmetic and moves only that evaluation to the caller's smaller pool when its actual workspace geometry can be admitted. The benchmark sets the option with ZOPRO_BENCH_G1_K_FRONTEND. The default is disabled. The released binary, disabled arm and enabled arm otherwise use the same 64+32 layout and fused FFT. Additional arms compare frontend solver placement and 56+40 or 72+24 core splits.

Six matched blocks use the two primary transactions, with four additional blocks per input on two more requests. Each timed process warms three proofs and times thirteen. Separate route checks verify dispatch with the option enabled, and fallback with the option disabled, scratch unavailable or the pool detached. Native core tests passed and the generic examples compiled. All 256 C7a and eight Ryzen exports independently verified. The temporary instance and storage were removed.

- 4x2 at 64+32 took **15.389 / 15.513 / 15.245 / 15.413 ms** enabled and **16.561 / 16.750 / 16.734 / 16.536 ms** disabled on the original, second, v153 and v156 inputs. Paired changes were -7.54% / -7.84% / -8.75% / -7.04%.
- 8x2 at 64+32 took **21.089 / 21.422 / 21.759 / 21.469 ms** enabled and **22.891 / 22.971 / 22.852 / 23.042 ms** disabled on the original, second, v153 and v156 inputs. Paired changes were -8.27% / -6.43% / -5.54% / -6.06%.

| Shape | Input | Profile | Proof ms | Paired vs same-layout off | Paired vs released SDK | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---|---:|---:|---:|---:|
| 4x2 | catalog | aeglos | 18.130 | off, +9.20% (0/6) | +10.33% (0/6) | +0.00% (0/6) | 1349.0 | 6.01× / 2.65× |
| 4x2 | catalog | off | 16.561 | off, +0.00% (0/6) | -0.08% (3/6) | -8.42% (6/6) | 226.6 | 6.58× / 2.90× |
| 4x2 | catalog | on | 15.389 | off, -7.54% (6/6) | -6.35% (6/6) | -15.18% (6/6) | 211.6 | 7.08× / 3.12× |
| 4x2 | catalog | release | 16.526 | off, +0.09% (3/6) | +0.00% (0/6) | -9.36% (6/6) | 213.9 | 6.60× / 2.90× |
| 4x2 | catalog | solve-front-off | 16.942 | solve-front-off, +0.00% (0/6) | +2.80% (0/6) | -6.75% (6/6) | 237.9 | 6.43× / 2.83× |
| 4x2 | catalog | solve-front-on | 15.257 | solve-front-off, -10.34% (6/6) | -8.34% (6/6) | -15.47% (6/6) | 205.0 | 7.14× / 3.15× |
| 4x2 | heldout | aeglos | 18.014 | off, +8.23% (0/6) | +7.16% (0/6) | +0.00% (0/6) | 1340.3 | — |
| 4x2 | heldout | off | 16.750 | off, +0.00% (0/6) | -0.55% (4/6) | -7.60% (6/6) | 225.2 | — |
| 4x2 | heldout | on | 15.513 | off, -7.84% (6/6) | -7.99% (6/6) | -14.36% (6/6) | 209.5 | — |
| 4x2 | heldout | release | 16.847 | off, +0.56% (2/6) | +0.00% (0/6) | -6.68% (6/6) | 228.2 | — |
| 4x2 | heldout | solve-front-off | 17.066 | solve-front-off, +0.00% (0/6) | +1.53% (1/6) | -4.96% (6/6) | 235.6 | — |
| 4x2 | heldout | solve-front-on | 15.473 | solve-front-off, -9.38% (6/6) | -8.44% (6/6) | -14.06% (6/6) | 204.2 | — |
| 4x2 | v153 | off | 16.734 | off, +0.00% (0/4) | -0.05% (2/4) | — | 224.5 | — |
| 4x2 | v153 | on | 15.245 | off, -8.75% (4/4) | -8.62% (4/4) | — | 202.7 | — |
| 4x2 | v153 | release | 16.704 | off, +0.05% (2/4) | +0.00% (0/4) | — | 226.8 | — |
| 4x2 | v156 | off | 16.536 | off, +0.00% (0/4) | -0.82% (3/4) | — | 219.1 | — |
| 4x2 | v156 | on | 15.413 | off, -7.04% (4/4) | -7.56% (4/4) | — | 197.4 | — |
| 4x2 | v156 | release | 16.712 | off, +0.83% (1/4) | +0.00% (0/4) | — | 229.6 | — |
| 8x2 | catalog | aeglos | 30.481 | off, +33.74% (0/6) | +36.51% (0/6) | +0.00% (0/6) | 2488.4 | 6.20× / 2.43× |
| 8x2 | catalog | off | 22.891 | off, +0.00% (0/6) | +2.41% (0/6) | -25.23% (6/6) | 446.9 | 8.26× / 3.23× |
| 8x2 | catalog | off56 | 22.515 | off56, +0.00% (0/6) | +0.61% (1/6) | -26.48% (6/6) | 446.3 | 8.39× / 3.29× |
| 8x2 | catalog | off72 | 22.685 | off72, +0.00% (0/6) | +2.12% (2/6) | -25.06% (6/6) | 425.3 | 8.33× / 3.26× |
| 8x2 | catalog | on | 21.089 | off, -8.27% (6/6) | -5.39% (6/6) | -30.70% (6/6) | 411.3 | 8.96× / 3.51× |
| 8x2 | catalog | on56 | 21.305 | off56, -5.25% (6/6) | -4.61% (6/6) | -30.25% (6/6) | 412.9 | 8.87× / 3.47× |
| 8x2 | catalog | on72 | 21.701 | off72, -4.72% (6/6) | -2.55% (6/6) | -28.52% (6/6) | 415.3 | 8.71× / 3.41× |
| 8x2 | catalog | release | 22.317 | off, -2.35% (6/6) | +0.00% (0/6) | -26.74% (6/6) | 435.1 | 8.47× / 3.32× |
| 8x2 | catalog | solve-front-off | 22.976 | solve-front-off, +0.00% (0/6) | +2.95% (0/6) | -24.35% (6/6) | 411.2 | 8.23× / 3.22× |
| 8x2 | catalog | solve-front-on | 21.436 | solve-front-off, -6.49% (6/6) | -3.80% (6/6) | -29.66% (6/6) | 369.6 | 8.82× / 3.45× |
| 8x2 | heldout | aeglos | 30.564 | off, +33.35% (0/6) | +36.37% (0/6) | +0.00% (0/6) | 2459.2 | — |
| 8x2 | heldout | off | 22.971 | off, +0.00% (0/6) | +2.24% (1/6) | -25.01% (6/6) | 430.8 | — |
| 8x2 | heldout | off56 | 22.821 | off56, +0.00% (0/6) | +1.14% (1/6) | -25.28% (6/6) | 445.3 | — |
| 8x2 | heldout | off72 | 23.203 | off72, +0.00% (0/6) | +2.36% (0/6) | -24.42% (6/6) | 438.1 | — |
| 8x2 | heldout | on | 21.422 | off, -6.43% (6/6) | -4.82% (6/6) | -29.64% (6/6) | 436.2 | — |
| 8x2 | heldout | on56 | 21.641 | off56, -5.48% (6/6) | -4.04% (6/6) | -29.45% (6/6) | 421.0 | — |
| 8x2 | heldout | on72 | 21.888 | off72, -5.73% (6/6) | -2.85% (6/6) | -28.28% (6/6) | 419.9 | — |
| 8x2 | heldout | release | 22.537 | off, -2.19% (5/6) | +0.00% (0/6) | -26.67% (6/6) | 453.6 | — |
| 8x2 | heldout | solve-front-off | 23.074 | solve-front-off, +0.00% (0/6) | +2.37% (0/6) | -24.61% (6/6) | 405.7 | — |
| 8x2 | heldout | solve-front-on | 21.457 | solve-front-off, -6.69% (6/6) | -5.04% (6/6) | -30.49% (6/6) | 369.4 | — |
| 8x2 | v153 | off | 22.852 | off, +0.00% (0/4) | +1.33% (1/4) | — | 433.4 | — |
| 8x2 | v153 | on | 21.759 | off, -5.54% (4/4) | -4.50% (4/4) | — | 414.4 | — |
| 8x2 | v153 | release | 22.640 | off, -1.31% (3/4) | +0.00% (0/4) | — | 442.7 | — |
| 8x2 | v156 | off | 23.042 | off, +0.00% (0/4) | +0.05% (2/4) | — | 440.1 | — |
| 8x2 | v156 | on | 21.469 | off, -6.06% (4/4) | -6.66% (4/4) | — | 396.6 | — |
| 8x2 | v156 | release | 22.906 | off, -0.04% (2/4) | +0.00% (0/4) | — | 440.9 | — |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Dispatch checks](artifacts/route-observations.json) · [Verification](artifacts/verification.json)

The SDK includes the default-off per-key option. The 64+32 settings repeated the improvement against the two controls on all four inputs for 4x2, 8x2.

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU on the measured C7a profiles. [Previous comparison](../2026-11-19-early-g1-c7a755/README.md)

| Shape | C7a profile | C7a ms | M5 CPU / Metal ms | Speedup CPU / Metal | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 737 dedicated G1 80+16 | 7.953 | 37 / 24 | 4.65× / 3.02× | +4.253 |
| 2×2 | 750 archived-band 72+24 | 12.358 | 59 / 30 | 4.77× / 2.43× | +6.458 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 757 on | 21.089 | 189 / 74 | 8.96× / 3.51× | +2.189 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
