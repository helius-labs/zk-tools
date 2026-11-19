# One public-training predicate for conditional query offsets

The model selects one B2 scalar row whose zero/nonzero state partitions the training witnesses into two sets, each containing at least two examples. Predicates producing the same training partition share a candidate, with Boolean-valued and lower-index rows preferred. It selects the predicate by sampled incremental signed width-12 digit savings relative to the frequent whole-row origin rule. For each row and branch whose training values are all identical, that value becomes the offset. All other rows retain their ordinary origin. Selection and fitting finish before any held-out scalar vectors are loaded. Each held-out scalar is reconstructed exactly by adding its measured residual to the predicted offset.

The 1×2 and 2×2 screens alternate eight source requests between discovery and evaluation. The 12×2 screen trains on eight adjacent requests, then scores the original, three other adjacent requests and four scattered requests. These are operation counts on public synthetic fixtures. The counts exclude existing runtime grouping, checks, key preparation, lookup traffic and curve addition. A conditional file with index and scalar per additional origin would use 36 bytes per record before headers or prepared points. No prover implementation or latency measurement is implied.

| Shape | Input | Eligible partitions | Additional origins | Modeled ordinary / conditional digits | Change | Correct zero residuals / changed offsets |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 2 | 3 | 318 | 558,271 / 558,190 | -0.015% | 78 / 159 |
| 1x2 | 4 | 3 | 318 | 558,257 / 558,242 | -0.003% | 87 / 159 |
| 1x2 | 6 | 3 | 318 | 558,415 / 558,412 | -0.001% | 81 / 159 |
| 1x2 | 8 | 3 | 318 | 558,331 / 558,212 | -0.021% | 97 / 159 |
| 2x2 | 34 | 3 | 729 | 979,269 / 979,299 | +0.003% | 175 / 380 |
| 2x2 | 36 | 3 | 729 | 979,407 / 979,075 | -0.034% | 183 / 349 |
| 2x2 | 38 | 3 | 729 | 979,495 / 979,519 | +0.002% | 178 / 380 |
| 2x2 | 40 | 3 | 729 | 979,501 / 979,193 | -0.031% | 171 / 349 |
| 12x2 | Original | 119 | 1,455 | 5,078,583 / 5,077,899 | -0.013% | 337 / 734 |
| 12x2 | 57 | 119 | 1,455 | 5,084,157 / 5,084,092 | -0.001% | 393 / 721 |
| 12x2 | 59 | 119 | 1,455 | 5,085,192 / 5,085,083 | -0.002% | 415 / 721 |
| 12x2 | 64 | 119 | 1,455 | 5,082,042 / 5,081,216 | -0.016% | 408 / 734 |
| 12x2 | 141 | 119 | 1,455 | 5,103,215 / 5,103,216 | +0.000% | 360 / 721 |
| 12x2 | 142 | 119 | 1,455 | 5,103,075 / 5,103,130 | +0.001% | 333 / 721 |
| 12x2 | 143 | 119 | 1,455 | 5,102,300 / 5,102,239 | -0.001% | 391 / 721 |
| 12x2 | 144 | 119 | 1,455 | 5,103,890 / 5,103,951 | +0.001% | 330 / 721 |

[Counts and verified source identities](artifacts/counts.json) · [Model](artifacts/screen.py) · [Signed digit arithmetic](artifacts/zp_glv513.py)
