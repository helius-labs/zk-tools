# Cancelling the Montgomery radix change in Euler products

The field arrays store Montgomery values using R = 2²⁵⁶. The eight-lane FFT uses R = 2²⁶⁰. Reading the four original limbs directly scales their mathematical value by 1/16 in the transform representation. The Euler derivative is linear. Applying this scale to either its inputs or the other operands of the final product leaves exactly one factor of 1/16 in the result. Normalizing and unpacking the limbs directly at the original radix cancels that factor. Applying the change to both operands requires multiplying the Euler coefficient index by 16. The experiment restricts that variant to domains where the scaled weight fits 28 bits.

The release, disabled and three experimental modes use the same original requests, 32-worker placements, wire files and qualified per-shape FFT settings. Prepared point groups are disabled in every arm. Five shuffled process blocks used three warmups and 31 timed calls on 1×2/2×2 or eleven elsewhere. The native oracle checked direct polynomial coefficients. An additional equivalence test covered full-width inputs, zeros and partial extents across the ordinary transform, packed columns and local Euler center. Peak RSS includes setup.

| Shape | Release / disabled / derivative / product / both ms | Changes vs disabled in experimental order | Both vs release | Peak MiB disabled / both | Both vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| 1x2 | 7.162 / 7.201 / 7.167 / 7.057 / 7.136 | -0.13% (3/5) / -2.00% (5/5) / -1.09% (3/5) | -0.55% (4/5) | 66.5 / 65.2 | 5.18× / 3.36× |
| 2x2 | 9.851 / 9.697 / 9.669 / 9.572 / 9.656 | +0.82% (2/5) / -0.06% (3/5) / -1.19% (3/5) | +0.31% (2/5) | 119.2 / 116.8 | 6.11× / 3.11× |
| 4x2 | 15.375 / 15.515 / 15.557 / 15.615 / 15.661 | +1.76% (2/5) / +1.82% (2/5) / +0.84% (1/5) | +0.30% (1/5) | 250.2 / 255.8 | 6.96× / 3.06× |
| 8x2 | 24.784 / 24.404 / 24.485 / 23.902 / 24.104 | -0.16% (4/5) / -0.94% (4/5) / -0.33% (3/5) | -2.83% (5/5) | 429.8 / 450.0 | 7.84× / 3.07× |
| 12x2 | 31.329 / 31.215 / 31.347 / 31.224 / 31.299 | -0.82% (3/5) / -0.22% (3/5) / -0.59% (3/5) | -0.00% (3/5) | 579.4 / 552.3 | 9.07× / 3.23× |
| 16x2 | 39.738 / 39.084 / 40.235 / 39.751 / 39.593 | +1.44% (2/5) / +0.93% (2/5) / +0.26% (2/5) | +0.25% (2/5) | 655.3 / 675.1 | 8.46× / 3.13× |
| 24x2 | 63.997 / 63.802 / 63.342 / 63.427 / 63.568 | -0.73% (4/5) / +0.65% (2/5) / +0.29% (1/5) | -0.01% (3/5) | 1016.0 / 1039.9 | 8.31× / 2.77× |
| merge-24x1 | 64.008 / 63.444 / 64.011 / 64.257 / 63.180 | +2.21% (1/5) / +1.64% (0/5) / +0.61% (1/5) | -1.39% (4/5) | 1055.0 / 1049.8 | 8.02× / 2.77× |

All 200 exports verified independently against the original key and public witness. [Samples](artifacts/processes.json.gz) · [Arithmetic and oracle](artifacts/incremental.patch) · [Verification](artifacts/verification.json)
