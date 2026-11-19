# Timing the same grouped terms in two orders

Each paired comparison keeps the curve points and their scalar values fixed and includes the in-place permutation cost. The interaction subtracts the same-block natural copy order effect from the corresponding split order effect.

| Shape | Layout | Ordinary / striped ms | Paired ms | Paired percent and wins | Order effect less natural order effect, ms | Peak ordinary / striped MiB |
|---|---|---:|---:|---:|---:|---:|
| 6x2 | copy | 21.638 / 22.263 | +0.683 | +3.17% (2/7) | +0.000 | 413.2 / 420.8 |
| 6x2 | g2 | 32.935 / 23.715 | -9.088 | -27.79% (7/7) | -9.468 | 455.5 / 439.5 |
| 6x2 | both | 38.894 / 24.548 | -14.359 | -37.20% (7/7) | -14.699 | 464.5 / 453.5 |
| 5x16 | copy | 24.726 / 26.029 | +1.235 | +5.02% (0/7) | +0.000 | 499.2 / 481.8 |

All 74 exports verified. [Protocol](protocol.md) · [Implementation](artifacts/source.patch) · [Raw samples and probes](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)
