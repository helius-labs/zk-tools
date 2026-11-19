# Reading the public Euler row pairs from a mapped block image

The loader retains a shared reference to the immutable key and range metadata. It validates the field limbs, dictionary bounds, block offsets, tail bits and exception IDs before proving. Each block subtracts its common pair from the matching rows and its exceptions from the remaining rows. Entire zero blocks can be skipped. Independent arbitrary-parts calls resolve individual rows through the same image. The old flat dictionaries remain readable. The native test checks all three ID widths, an unaligned image slice, a partial last block and malformed layouts.

Both key images retain identical bytes through the complete transformed proving key and original solver. Four independent arbitrary-parts vectors on each key reproduced all five component encodings from the ordinary images. At 32 Ryzen workers five shuffled blocks measure the prior flat executable, a new executable using the flat image, and that same executable using block encoding. The five input requests share a loaded key and have identical order across modes within each block.

| Shape | Request | Prior / flat / block ms | Paired block vs flat | Paired block vs prior | Block vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| 12x2 | Original | 30.851 / 30.426 / 30.462 | -0.72% (3/5) | -1.56% (4/5) | 9.32× / 3.32× |
| 12x2 | 53 | 30.763 / 30.772 / 30.474 | -1.57% (4/5) | -0.13% (4/5) | — |
| 12x2 | 56 | 31.066 / 30.441 / 30.483 | +1.14% (2/5) | -1.70% (4/5) | — |
| 12x2 | 141 | 47.510 / 47.261 / 47.200 | +1.22% (2/5) | +0.69% (2/5) | — |
| 12x2 | 142 | 46.438 / 46.528 / 46.894 | +1.01% (2/5) | -0.22% (3/5) | — |
| merge-24x1 | Original | 61.580 / 63.348 / 61.466 | -2.51% (4/5) | -0.19% (4/5) | 8.25× / 2.85× |
| merge-24x1 | 109 | 64.675 / 65.021 / 63.879 | -0.79% (3/5) | -0.91% (4/5) | — |
| merge-24x1 | 112 | 66.363 / 66.545 / 65.901 | -0.52% (3/5) | +0.44% (2/5) | — |
| merge-24x1 | 122 | 65.902 / 72.467 / 66.761 | -8.26% (4/5) | +0.47% (2/5) | — |
| merge-24x1 | 124 | 69.132 / 69.854 / 68.782 | -0.16% (3/5) | +0.69% (1/5) | — |

| Shape | Owned dictionary array payload bytes, flat / block | Dictionary disk bytes, flat / block | Median entire process peak MiB, flat / block |
|---|---:|---:|---:|
| 12x2 | 878,372 / 0 | 739,416 / 482,936 | 1134.5 / 1146.1 |
| merge-24x1 | 2,235,774 / 0 | 1,717,494 / 2,223,006 | 1298.4 / 1310.4 |

All 150 SDK exports verified. Mapped pages can contribute to RSS. [Native and loader change](artifacts/source.patch) · [Image hashes](artifacts/images.json) · [Arbitrary-parts checks](artifacts/parts.json) · [Samples](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)
