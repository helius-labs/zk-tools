# Direct quotient output with Euler FFT leaf fusion

Both candidates fell short of the matched full-proof requirement on Ryzen. Fusing both column passes reduced the median paired latency by 0.07–0.63% across the four cases, with six wins out of eight in one case and five in the other three.

The first candidate applies the shifted Euler center and writes the product during the final column transform. The second also fuses input packing with the inverse column transform. Both reuse the existing product emitter and add no retained tables. The 2×2 control is the [qualified leaf-fusion SDK](../2026-11-20-euler-fft-sdk-c7a943/README.md). The 4×2 control is the seeded SDK with its packed input and output layout.

The eight matched blocks per input used 24 main and eight frontend Ryzen threads, three primes, and seventeen timed proofs per process. The table reports medians of process means. Paired changes compare with the same-build control. All four control calibrations were within 0.24%.

| Shape | Input | Frozen ms | Same-build ms | Output fused ms | Both fused ms | Output vs same build | Both vs same build |
|---|---|---:|---:|---:|---:|---:|---:|
| 2×2 | catalog | 9.395 | 9.442 | 9.429 | 9.435 | +0.17% (3/8) | -0.23% (6/8) |
| 2×2 | heldout | 9.425 | 9.470 | 9.440 | 9.439 | -0.17% (4/8) | -0.07% (5/8) |
| 4×2 | catalog | 13.631 | 13.700 | 13.642 | 13.653 | -0.39% (5/8) | -0.63% (5/8) |
| 4×2 | heldout | 13.623 | 13.597 | 13.611 | 13.585 | -0.06% (5/8) | -0.21% (5/8) |

All seven native Euler-center tests passed, including direct comparison of the fused products with the ordinary products across six domain sizes and two root choices. All 128 process exports independently verified with Gnark. The comparisons are local. [C7a M5 CPU and Metal scorecard](../2026-11-20-packed-final-sdk-c7a944/README.md#eight-supplied-m5-references)

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Experimental source](artifacts/source.patch.gz)
