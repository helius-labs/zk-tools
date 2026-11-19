# A single carry pass for inverse FFT difference products

The inverse butterfly multiplies a difference by a root immediately. For BN254 Fr and radix R = 2²⁶⁰, inputs below 2p allow a normalized intermediate d = 2p + a − b below 4p. Multiplication by a root below 2p gives REDC below p + 8p²/R < 2p. The alternate inverse kernel feeds this intermediate directly to the existing Montgomery multiplier. The difference requires a single signed carry pass. The intermediate stays within the helper and the packed FFT workspace uses its ordinary representation. The approach adds no persistent tables or key setup.

A native test compares scalar field results and raw output bounds for varying lanes, values near p and 2p, and radix limb boundaries. Three transform and polynomial tests cover weighting, padded input prefixes, root layouts and full coefficients. Five randomized blocks compare the frozen grouped build, a new ordinary build and the new difference path on all eight original requests and two additional public requests. Each process uses 32 Ryzen workers, three warmups and eleven timed proofs, or 31 on the two smallest shapes. The 12×2 and 24×2 profiles use fixed prepared groups across all arms.

| Shape | Request | Frozen / ordinary / candidate ms | Paired vs ordinary | Paired vs frozen | Peak MiB ordinary / candidate | Candidate vs M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | Original | 7.259 / 7.181 / 7.138 | -0.60% (3/5) | -1.26% (4/5) | 66.5 / 63.5 | 5.18× / 3.36× |
| 2x2 | Original | 9.707 / 9.567 / 9.630 | +1.52% (1/5) | -0.49% (3/5) | 114.2 / 117.5 | 6.13× / 3.12× |
| 4x2 | Original | 15.515 / 15.710 / 15.573 | -1.05% (3/5) | -0.81% (3/5) | 250.2 / 256.5 | 7.00× / 3.08× |
| 8x2 | Original | 24.168 / 24.360 / 24.657 | +0.06% (2/5) | -0.04% (3/5) | 429.2 / 438.8 | 7.67× / 3.00× |
| 12x2 | Original | 29.587 / 29.253 / 29.408 | +0.52% (1/5) | +0.67% (2/5) | 544.3 / 537.9 | 9.66× / 3.43× |
| 16x2 | Original | 40.051 / 39.472 / 40.233 | +1.97% (0/5) | +0.08% (2/5) | 662.8 / 673.3 | 8.33× / 3.08× |
| 24x2 | Original | 59.727 / 60.172 / 60.071 | -0.63% (3/5) | -0.44% (3/5) | 1084.6 / 1099.5 | 8.79× / 2.93× |
| merge-24x1 | Original | 63.516 / 64.067 / 63.413 | -1.65% (4/5) | -0.01% (3/5) | 1026.7 / 1034.5 | 8.00× / 2.76× |
| 12x2 | 142 | 46.087 / 46.073 / 46.996 | +2.64% (1/5) | +1.14% (1/5) | 1006.1 / 993.2 | — |
| 24x2 | 103 | 59.216 / 58.152 / 59.650 | +1.93% (1/5) | +0.66% (2/5) | 1136.0 / 1125.6 | — |

All 150 process exports verified. [Samples](artifacts/timings.json.gz) · [Source and oracle](artifacts/source.patch) · [Verification](artifacts/verification.json)
