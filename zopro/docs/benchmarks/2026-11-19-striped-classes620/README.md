# Timing the same grouped terms in two orders

With G1 and G2 widened, changing only term order saved 14.359 ms in the paired median and won all seven blocks. The same permutation added 0.683 ms on natural 6×2 and 1.235 ms on natural 5×16. Keep the natural layouts in the prover.

After separating the repeated terms, widening 6×2 cost **2.867 ms** against natural 6×2. The natural 5×16 minus 6×2 difference was **2.841 ms** in these same blocks. The direct paired difference between reordered widened 6×2 and natural 5×16 was −0.191 ms. The controlled replay reproduces the size of the local gap and shows how artificial adjacency inflated the original intervention. These measurements cover one witness per key on Ryzen. The corresponding AWS comparison still needs a matched run. [Paired block differences](artifacts/natural-gap.json) · [Measured digit sequences](../2026-11-19-class-weights619/README.md)

Each paired comparison keeps the curve points and their scalar values fixed and includes the in-place permutation cost. The interaction subtracts the same-block natural copy order effect from the corresponding split order effect.

| Shape | Layout | Ordinary / striped ms | Paired ms | Paired percent and wins | Order effect less natural order effect, ms | Peak ordinary / striped MiB |
|---|---|---:|---:|---:|---:|---:|
| 6x2 | copy | 21.638 / 22.263 | +0.683 | +3.17% (2/7) | +0.000 | 413.2 / 420.8 |
| 6x2 | g2 | 32.935 / 23.715 | -9.088 | -27.79% (7/7) | -9.468 | 455.5 / 439.5 |
| 6x2 | both | 38.894 / 24.548 | -14.359 | -37.20% (7/7) | -14.699 | 464.5 / 453.5 |
| 5x16 | copy | 24.726 / 26.029 | +1.235 | +5.02% (0/7) | +0.000 | 499.2 / 481.8 |

All 74 exports verified. [Protocol](protocol.md) · [Implementation](artifacts/source.patch) · [Raw samples and probes](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)
