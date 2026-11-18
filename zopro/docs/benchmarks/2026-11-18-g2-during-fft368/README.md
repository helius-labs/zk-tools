# Starting G2 before the product FFT finishes

The existing frontend pool runs the solver and the FFT. G2 uses the main pool as soon as its canonical scalars are available. The g2a setting also starts A early. The wait control uses the same arithmetic and completes the FFT first. A process-wide reservation covers the extra lifetime of the FFT buffers, inputs, leaf expansions and weights. The zero-allowance preflights produced verified proofs through the sequential path.

| Shape | Selected mode / front | Release ms | Wait ms | Overlap ms | Paired vs release | Faster blocks | Paired vs wait | Faster blocks | Wait / overlap peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | g2 / 8 | 7.619 | 7.693 | 8.023 | +5.78% | 0/9 | +4.61% | 1/9 | 85 / 74 |
| 2x2 | g2 / 16 | 10.814 | 10.680 | 11.571 | +6.64% | 3/9 | +8.45% | 0/9 | 165 / 171 |

## All eight references

The first two rows use the confirmed overlap settings. The other rows use the same experimental binary with no attached frontend.

| Shape | SDK ms | Peak MiB | M5 CPU / Metal ms | CPU / Metal gain | Blocks at 10× CPU |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.023 | 74 | 37 / 24 | 4.61× / 2.99× | 0/9 |
| 2x2 | 11.571 | 171 | 59 / 30 | 5.10× / 2.59× | 0/9 |
| 4x2 | 17.126 | 213 | 109 / 48 | 6.36× / 2.80× | 0/5 |
| 8x2 | 27.113 | 362 | 189 / 74 | 6.97× / 2.73× | 0/5 |
| 12x2 | 39.014 | 515 | 284 / 101 | 7.28× / 2.59× | 0/5 |
| 16x2 | 45.881 | 609 | 335 / 124 | 7.30× / 2.70× | 0/5 |
| 24x2 | 75.147 | 949 | 528 / 176 | 7.03× / 2.34× | 0/5 |
| merge-24x1 | 71.445 | 888 | 507 / 175 | 7.10× / 2.45× | 0/5 |

[Source](artifacts/source.patch.gz) · [Timing samples](artifacts/local.json.gz) · [Protocol](artifacts/protocol.md)

[Paired FFT/G2 profiles](phases.md)
