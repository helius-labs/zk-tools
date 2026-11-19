# Direct final FFT butterfly consumption in full proofs

The 12×2 tiled candidate beat the parent in seven of eight paired blocks. Its median gain of 0.66% matched the 0.65% gain on the ordinary control path in the same build. Within each block, subtracting the ordinary route’s elapsed change from the tiled route’s change gave a median of −0.004 ms on 12×2 and +0.104 ms on 24×2. Each difference favored the candidate in four of eight blocks. Peak RSS rose on both reference inputs and the scattered input. Keep the existing FFT implementation.

The tiled route feeds the last butterfly directly into the derivative writer. The main route uses the ordinary output algorithm with matching pool placement in each executable. The candidate passed all 162 native core tests and all 82 benchmark exports verified independently.

| Input | Route | Parent / candidate ms | Paired change and wins | Peak parent / candidate MiB | Candidate vs M5 CPU / Metal | Blocks meeting 10× CPU |
|---|---|---:|---:|---:|---:|---:|
| 12x2-0 | main | 28.723 / 28.631 | -0.65% (6/8) | 563.3 / 583.1 | 9.92× / 3.53× | 1/8 |
| 12x2-0 | tile | 28.849 / 28.638 | -0.66% (7/8) | 562.3 / 566.0 | 9.92× / 3.53× | 2/8 |
| 24x2-0 | main | 57.959 / 58.078 | +1.02% (3/8) | 939.3 / 964.7 | 9.09× / 3.03× | 0/8 |
| 24x2-0 | tile | 57.232 / 56.972 | -0.37% (4/8) | 944.7 / 950.8 | 9.27× / 3.09× | 0/8 |
| 12x2-139 | tile | 43.182 / 43.379 | +0.23% (2/4) | 744.6 / 779.3 | — | — |

[Protocol](protocol.md) · [Source](artifacts/source.patch) · [Samples and CPU profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)

The reference 12×2 request consumed 280,242 of 524,288 field outputs. Its outer stage therefore computes 70,062 packed sums or differences in the new writer, down from 131,072, while all final-stage root products are still needed. The analogous 24×2 count is 138,558 of 262,144. One warm CPU profile per arm attributed 7.11% / 6.74% of sampled CPU cycles to the old / new column consumer on 12×2 and 9.82% / 9.27% on 24×2. The scattered witness moved from 4.77% to 5.13%. These shares cover all workers. [Per-block control adjustment and profile shares](artifacts/route-check.json)
