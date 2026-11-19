# Affine relations between varying B2 scalars

The scanner fits relations of the form y = a·x + b in the BN254 scalar field, using public training fixtures. It drops rows that remain constant on that set. For each other row it subtracts its first observed value and normalizes the resulting vector by its first nonzero difference. Rows with the same normalized vector belong to the same affine class. The first row represents the class. Each held-out witness is reconstructed exactly by adding its measured residual to the prediction.

The transfer panel alternates the eight public requests between discovery and evaluation. On the 12×2, 16×2 and 24×2 panels this holds out deposit and data-bearing requests. Merge uses eight sparse-tree requests and eight populated-tree requests, splitting each population in half by the same alternating rule. Every archived scalar file was checked against its recorded hash and associated proof verification. Synthetic controls cover equal and opposite scalars, nonzero shifts, general slopes and a wrong prediction.

| Shape | Train / held out | Constant training rows excluded | Signed-equal members beyond representatives | Matched on every held-out request | Signed-equal digits saved | Extra digits saved by other affine predictions |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 4 / 4 | 3,986 | 411 | 86 | -0.10% | -0.24% |
| 2x2 | 4 / 4 | 6,193 | 18,102 | 17,385 | +35.28% | -0.35% |
| 12x2 | 4 / 4 | 31,081 | 194,157 | 190,131 | +75.49% | -0.38% |
| 16x2 | 4 / 4 | 41,387 | 263,778 | 258,577 | +77.57% | -0.29% |
| 24x2 | 4 / 4 | 62,498 | 404,707 | 396,739 | +80.00% | -0.39% |
| merge-24x1 | 8 / 8 | 51,193 | 314,294 | 229,269 | +58.93% | -0.57% |

The savings columns divide the difference in nonzero signed width-12 digits between the observed scalars and prediction residuals by all original B2 digits on the held-out set. Negative values mean the residuals increased the digit count. Runtime grouping already handles some equal values. These counts precede point addition, existing grouping, memory traffic and scheduling, so they do not predict end-to-end latency. A runtime prototype can test the narrower hypothesis that preparing repeated point sums removes work already performed in each proof. Any untrusted or previously unseen witness still requires exact checks or residual correction.

[Aggregate counts and fixture hashes](artifacts/counts.json) · [Exact scanner](artifacts/scan.py)
