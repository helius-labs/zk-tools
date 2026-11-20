# Restoring indexed leading-window reduction at 13 bits

Restoring indexed reduction at 13 bits recovered most of the width-only loss. The two-range indexed path improved 35–43% versus matched 13-bit streaming, but still took 6.960–7.874 ms on 1×2 and 9.693–9.754 ms on 2×2. The 12-bit ordinary control kept the lower latency on all inputs.

The [B2 work model](../2026-11-20-four-dimensional-b2-model-ryzen940/README.md) predicted fewer curve pairs for 13-bit windows. The original 12-bit layout has a six-bit leading remainder and uses indexed reduction there. At 13 bits the remainder is nine bits, so the original eight-bit threshold sends that window through the streaming path. The wider footprint and the leading-window route both change in the width-only experiment.

The reference is frozen SDK 851. The same-build ordinary comparison calibrates within 2% on each fixture. Each timing block primes three proofs and measures seventeen. Main and frontend use 24 and eight Ryzen hardware threads. Separate route probes confirm the expected width, two and four ranges, and ordinary fallback with zero optional memory. Paired changes versus the 13-bit four-range streaming arm are included for the separate leading-window control. 1×2 qualifying modes were none. 2×2 qualifying modes were none. The native G2 tests passed, and Gnark independently verified all 270 exported proofs.

| Shape and input | Mode | Proof ms | Paired vs frozen | Paired vs ordinary | Paired vs 13-bit two ranges | Paired vs 13-bit four ranges | CPU-ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2/catalog | release | 6.898 | +0.00% (0/8) | -0.68% (6/8) | -46.14% (8/8) | -32.01% (8/8) | 133.7 | 58.5 |
| 1x2/catalog | ordinary | 6.944 | +0.68% (2/8) | +0.00% (0/8) | -45.70% (8/8) | -31.26% (8/8) | 134.3 | 57.8 |
| 1x2/catalog | 13x2 | 12.808 | +85.68% (0/8) | +84.15% (0/8) | +0.00% (0/8) | +26.52% (0/8) | 154.2 | 75.2 |
| 1x2/catalog | 13x4 | 10.117 | +47.08% (0/8) | +45.49% (0/8) | -20.96% (8/8) | +0.00% (0/8) | 170.3 | 93.5 |
| 1x2/catalog | indexed2 | 7.874 | +14.36% (0/8) | +13.54% (1/8) | -38.62% (8/8) | -22.10% (8/8) | 145.4 | 75.8 |
| 1x2/catalog | indexed4 | 8.148 | +18.08% (0/8) | +17.53% (1/8) | -36.37% (8/8) | -19.46% (8/8) | 159.7 | 92.2 |
| 1x2/heldout | release | 6.771 | +0.00% (0/8) | -1.18% (7/8) | -44.83% (8/8) | -31.74% (8/8) | 129.5 | 58.2 |
| 1x2/heldout | ordinary | 6.842 | +1.19% (1/8) | +0.00% (0/8) | -44.06% (8/8) | -30.90% (8/8) | 129.6 | 58.0 |
| 1x2/heldout | 13x2 | 12.262 | +81.25% (0/8) | +78.75% (0/8) | +0.00% (0/8) | +23.64% (0/8) | 140.2 | 70.5 |
| 1x2/heldout | 13x4 | 9.928 | +46.49% (0/8) | +44.72% (0/8) | -19.12% (8/8) | +0.00% (0/8) | 156.8 | 87.8 |
| 1x2/heldout | indexed2 | 6.976 | +3.11% (0/8) | +1.90% (0/8) | -43.04% (8/8) | -29.68% (8/8) | 132.4 | 69.8 |
| 1x2/heldout | indexed4 | 7.408 | +10.11% (0/8) | +8.85% (0/8) | -39.29% (8/8) | -24.45% (8/8) | 145.2 | 84.2 |
| 1x2/v27 | release | 6.781 | +0.00% (0/8) | +0.03% (4/8) | -44.65% (8/8) | -30.72% (8/8) | 129.2 | 58.5 |
| 1x2/v27 | ordinary | 6.805 | -0.03% (4/8) | +0.00% (0/8) | -44.48% (8/8) | -30.56% (8/8) | 129.8 | 57.8 |
| 1x2/v27 | 13x2 | 12.227 | +80.66% (0/8) | +80.13% (0/8) | +0.00% (0/8) | +24.63% (0/8) | 140.6 | 69.2 |
| 1x2/v27 | 13x4 | 9.784 | +44.34% (0/8) | +44.02% (0/8) | -19.76% (8/8) | +0.00% (0/8) | 156.1 | 88.0 |
| 1x2/v27 | indexed2 | 6.960 | +2.79% (0/8) | +2.42% (0/8) | -43.09% (8/8) | -28.76% (8/8) | 132.4 | 69.2 |
| 1x2/v27 | indexed4 | 7.411 | +9.44% (0/8) | +8.80% (0/8) | -39.27% (8/8) | -24.03% (8/8) | 144.8 | 85.8 |
| 2x2/catalog | release | 9.494 | +0.00% (0/8) | -0.38% (5/8) | -36.71% (8/8) | -24.92% (8/8) | 180.9 | 93.8 |
| 2x2/catalog | ordinary | 9.545 | +0.39% (3/8) | +0.00% (0/8) | -36.46% (8/8) | -24.58% (8/8) | 180.9 | 98.5 |
| 2x2/catalog | 13x2 | 14.976 | +58.01% (0/8) | +57.39% (0/8) | +0.00% (0/8) | +18.45% (0/8) | 192.3 | 107.2 |
| 2x2/catalog | 13x4 | 12.629 | +33.18% (0/8) | +32.58% (0/8) | -15.58% (8/8) | +0.00% (0/8) | 209.6 | 123.0 |
| 2x2/catalog | indexed2 | 9.693 | +2.09% (0/8) | +2.43% (1/8) | -35.24% (8/8) | -23.07% (8/8) | 184.4 | 107.0 |
| 2x2/catalog | indexed4 | 10.357 | +8.88% (0/8) | +8.52% (0/8) | -30.80% (8/8) | -18.56% (7/8) | 199.6 | 123.5 |
| 2x2/heldout | release | 9.537 | +0.00% (0/8) | -0.04% (5/8) | -36.53% (8/8) | -24.61% (8/8) | 182.0 | 94.8 |
| 2x2/heldout | ordinary | 9.539 | +0.04% (3/8) | +0.00% (0/8) | -36.59% (8/8) | -24.55% (8/8) | 181.5 | 99.2 |
| 2x2/heldout | 13x2 | 15.009 | +57.56% (0/8) | +57.71% (0/8) | +0.00% (0/8) | +18.99% (0/8) | 193.2 | 104.2 |
| 2x2/heldout | 13x4 | 12.611 | +32.64% (0/8) | +32.54% (0/8) | -15.96% (8/8) | +0.00% (0/8) | 209.8 | 122.8 |
| 2x2/heldout | indexed2 | 9.754 | +2.14% (0/8) | +2.35% (1/8) | -35.16% (8/8) | -22.82% (8/8) | 185.5 | 108.0 |
| 2x2/heldout | indexed4 | 10.300 | +8.30% (0/8) | +8.19% (1/8) | -31.18% (8/8) | -18.49% (8/8) | 199.9 | 121.2 |

[Timing](artifacts/summary.json) · [Source patch](artifacts/source.patch) · [Proof verification](artifacts/verification.json)
