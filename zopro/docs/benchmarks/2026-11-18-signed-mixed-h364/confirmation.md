# Independent confirmation of signed H on C7a

For one prespecified width on each shape, select the faster signed layout from the screen. Advance it when it beats both controls in the paired median and a majority of blocks against each, or when the 16×2 screen median clears 33.5 ms. A confirmation consists of seven fresh blocks of 25 requests each.

| Shape | Cores | Windows | Release ms | Disabled ms | Signed ms | Paired vs release | Faster blocks | Peak MiB release / signed | Blocks at 10× CPU |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 64 | 18 | 34.630 | 34.170 | 34.375 | -0.86% | 5/7 | 836 / 813 | 1/7 |
| 24x2 | 96 | 17 | 49.688 | 48.839 | 49.510 | +1.24% | 3/7 | 1302 / 1333 | 7/7 |
| merge-24x1 | 96 | 18 | 50.635 | 50.767 | 50.470 | -0.38% | 4/7 | 1071 / 1058 | 5/7 |

[Selection](artifacts/selection.json) · [Confirmation process samples](artifacts/confirmation.json.gz)
