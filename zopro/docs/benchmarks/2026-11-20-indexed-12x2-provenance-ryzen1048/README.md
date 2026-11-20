# Where the archived 12×2 gain comes from

The archived indexed binary accepts the standard Euler key and the experimental shifted key. The shifted key adds 482,944 bytes and contains a compact table for subtracting a fixed reference from the FFT inputs, with matching changes to the K and public queries. The two grouping files are separate from the key image. Nested G2 grouping combines bases in two levels and computes an exact correction for each scalar that differs from its representative. Linear G1 grouping covers A and K.

The shifted image alone lowered paired latency by 1.97–2.80% on the catalog, held-out, and v58 requests. Adding only nested G2 grouping lowered it by a further 4.07–4.77% on the same image. Both groups on the standard image lowered paired latency by 4.35–4.71%. The high-diversity v139 request showed smaller gains. Its route probes used the first G2 grouping level and declined the second level and the two linear G1 plans. For the catalog, held-out, and v58 inputs, G2 grouping reduced 270,757 base terms to 43,189 representatives before mismatch corrections at the two levels.

## Warm latency and peak RSS

Each cell gives time in ms / peak RSS in MiB.

| Configuration | Catalog | Held-out | v58 | v139 |
|---|---:|---:|---:|---:|
| Current SDK | 32.213 / 500.2 | 31.976 / 493.0 | 32.240 / 500.9 | 46.501 / 730.9 |
| Archived SDK | 32.231 / 496.1 | 32.193 / 506.1 | 32.016 / 508.0 | 46.376 / 717.5 |
| Indexed, standard | 32.141 / 490.8 | 32.070 / 479.5 | 32.259 / 515.0 | 46.440 / 728.9 |
| Indexed, shifted | 31.337 / 482.8 | 31.372 / 494.5 | 31.428 / 509.5 | 46.072 / 736.2 |
| Indexed, standard, both groups | 30.674 / 480.5 | 30.675 / 487.8 | 30.750 / 492.9 | 46.200 / 666.5 |
| Indexed, shifted, both groups | 29.688 / 482.0 | 29.672 / 483.5 | 29.582 / 494.0 | 45.653 / 672.0 |
| Indexed, shifted, G2 groups | 30.145 / 470.8 | 30.106 / 497.3 | 29.941 / 484.3 | 45.363 / 662.7 |
| Indexed, shifted, G1 groups | 31.573 / 506.3 | 31.543 / 505.0 | 31.530 / 512.5 | 46.371 / 721.5 |

## Paired latency changes

| Change | Catalog | Held-out | v58 | v139 |
|---|---:|---:|---:|---:|
| Build alone, indexed vs archived SDK | -0.23% (3/6) | -0.38% (4/6) | +0.63% (2/6) | -0.14% (3/6) |
| Shifted image, groups off | -2.80% (6/6) | -1.97% (6/6) | -2.71% (6/6) | -0.68% (5/6) |
| Shifted image, both groups | -3.26% (6/6) | -3.01% (6/6) | -3.78% (6/6) | -1.33% (6/6) |
| Both groups, standard image | -4.64% (6/6) | -4.35% (6/6) | -4.71% (6/6) | -0.71% (5/6) |
| Nested G2 groups, shifted image | -4.10% (6/6) | -4.07% (6/6) | -4.77% (6/6) | -1.68% (6/6) |
| Linear G1 groups, shifted image | +0.94% (1/6) | +0.62% (2/6) | +0.64% (2/6) | +0.62% (1/6) |
| Both groups, shifted image | -5.38% (6/6) | -5.38% (6/6) | -5.92% (6/6) | -1.02% (6/6) |
| Shifted image and both groups vs current SDK | -7.68% (6/6) | -7.23% (6/6) | -8.51% (6/6) | -2.45% (6/6) |

Six order-balanced blocks ran on a Ryzen 9950X using 24 main and eight frontend logical workers. Each fresh process warmed three proofs and timed eleven. The first table gives the median of process means and maximum whole-process RSS. Timed runs disabled group tracing. The current and archived builds use pinned executables. The same-binary rows isolate the key and grouping changes. The mapped image's point and metadata changes are evaluated as a unit.

Gnark independently reverified 32 preflight exports and 192 timed exports against the original standard image and the expected public-input digests. The old profile remains an experimental build. These measurements isolate what merits a current-SDK trial.

[Summary](artifacts/summary.json) · [Samples and route checks](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Pins](artifacts/pins.json) · [Archived C7a profile](../2026-11-19-critical-path-c7a750/README.md) · [Current C7a comparison](../2026-11-20-c7a-three-provers/README.md)
