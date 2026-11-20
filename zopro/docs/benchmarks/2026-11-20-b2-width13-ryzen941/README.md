# Wider two-component B2 windows on small proofs

Both 13-bit layouts regressed on every Ryzen input. Four ranges limited the 1×2 regression to 43–48% and the 2×2 regression to 32–33% compared with same-build ordinary. CPU time and peak RSS increased.

The [B2 work model](../2026-11-20-four-dimensional-b2-model-ryzen940/README.md) predicted fewer curve pairs for 13-bit windows. The original 12-bit layout has a six-bit leading remainder and uses indexed reduction there. At 13 bits the remainder is nine bits, so the original eight-bit threshold sends that window through the streaming path. The wider footprint and the leading-window route both change in the width-only experiment.

The reference is frozen SDK 851. The same-build ordinary comparison calibrates within 2% on each fixture. Each timing block primes three proofs and measures seventeen. Main and frontend use 24 and eight Ryzen hardware threads. Separate route probes confirm the expected width, two and four ranges, and ordinary fallback with zero optional memory. 1×2 qualifying modes were none. 2×2 qualifying modes were none. The native G2 tests passed, and Gnark independently verified all 180 exported proofs.

| Shape and input | Mode | Proof ms | Paired vs frozen | Paired vs ordinary | Paired vs 13-bit two ranges | CPU-ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| 1x2/catalog | release | 6.912 | +0.00% (0/8) | -0.09% (4/8) | -45.86% (8/8) | 133.7 | 58.0 |
| 1x2/catalog | ordinary | 6.899 | +0.09% (4/8) | +0.00% (0/8) | -45.61% (8/8) | 133.7 | 57.5 |
| 1x2/catalog | 13x2 | 12.742 | +84.71% (0/8) | +83.85% (0/8) | +0.00% (0/8) | 153.5 | 75.2 |
| 1x2/catalog | 13x4 | 10.188 | +46.92% (0/8) | +48.29% (0/8) | -20.11% (8/8) | 170.6 | 92.8 |
| 1x2/heldout | release | 6.809 | +0.00% (0/8) | +0.52% (3/8) | -44.30% (8/8) | 129.1 | 58.2 |
| 1x2/heldout | ordinary | 6.789 | -0.51% (5/8) | +0.00% (0/8) | -44.74% (8/8) | 129.1 | 58.2 |
| 1x2/heldout | 13x2 | 12.278 | +79.54% (0/8) | +80.95% (0/8) | +0.00% (0/8) | 140.3 | 69.8 |
| 1x2/heldout | 13x4 | 9.835 | +44.40% (0/8) | +44.94% (0/8) | -19.97% (8/8) | 156.6 | 84.8 |
| 1x2/v27 | release | 6.800 | +0.00% (0/8) | -0.40% (6/8) | -44.73% (8/8) | 129.4 | 58.2 |
| 1x2/v27 | ordinary | 6.804 | +0.41% (2/8) | +0.00% (0/8) | -44.86% (8/8) | 129.4 | 58.0 |
| 1x2/v27 | 13x2 | 12.303 | +80.93% (0/8) | +81.37% (0/8) | +0.00% (0/8) | 140.2 | 69.8 |
| 1x2/v27 | 13x4 | 9.737 | +43.47% (0/8) | +43.13% (0/8) | -20.96% (8/8) | 156.1 | 88.5 |
| 2x2/catalog | release | 9.481 | +0.00% (0/8) | +0.20% (1/8) | -36.24% (8/8) | 181.5 | 95.0 |
| 2x2/catalog | ordinary | 9.474 | -0.20% (7/8) | +0.00% (0/8) | -36.42% (8/8) | 180.8 | 94.2 |
| 2x2/catalog | 13x2 | 14.859 | +56.84% (0/8) | +57.28% (0/8) | +0.00% (0/8) | 192.1 | 104.0 |
| 2x2/catalog | 13x4 | 12.506 | +32.24% (0/8) | +32.13% (0/8) | -15.84% (8/8) | 208.9 | 121.0 |
| 2x2/heldout | release | 9.539 | +0.00% (0/8) | +0.21% (3/8) | -36.66% (8/8) | 181.5 | 98.2 |
| 2x2/heldout | ordinary | 9.509 | -0.21% (5/8) | +0.00% (0/8) | -36.61% (8/8) | 181.3 | 97.5 |
| 2x2/heldout | 13x2 | 14.964 | +57.89% (0/8) | +57.74% (0/8) | +0.00% (0/8) | 192.2 | 107.5 |
| 2x2/heldout | 13x4 | 12.637 | +32.71% (0/8) | +32.70% (0/8) | -15.40% (8/8) | 210.5 | 121.2 |

[Timing](artifacts/summary.json) · [Source patch](artifacts/source.patch) · [Proof verification](artifacts/verification.json)
