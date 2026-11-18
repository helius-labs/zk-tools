# Caller-owned frontend pool integration

The public Linux x86-64 prover accepts a shared optional Rayon pool. The accelerated canonical CPU path uses a smaller attached pool for witness and product FFT preparation. Default loading constructs the key with no attached pool. The generic non-IFMA SDK compiled and 142 native core tests passed.

Two loaded SDK provers shared a caller-owned pool, switched to an equal-width pool and detached the pool on the same loaded handles. The final detach released the last strong reference. These transitions exported 24 independently verified proofs across two shapes with normal and zero optional preparation allowance. Concurrent two-client runs exported 30 more. The eight-shape latency comparison below used 16 pinned Ryzen cores and an optional eight-worker frontend.

| Shape | Previously integrated G2-only ms | Retained 308 with pool ms | New API disabled ms | New API enabled ms | Paired enabled / retained | Peak enabled MiB |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.531 | 8.880 | 8.500 | 8.830 | -0.56% | 58 |
| 2x2 | 11.800 | 12.149 | 12.055 | 12.219 | +1.04% | 105 |
| 4x2 | 16.977 | 17.171 | 16.985 | 16.930 | -0.76% | 207 |
| 8x2 | 27.712 | 26.750 | 26.679 | 26.799 | +0.01% | 368 |
| 12x2 | 39.192 | 38.513 | 38.714 | 39.267 | +0.63% | 496 |
| 16x2 | 45.815 | 46.660 | 47.883 | 46.402 | -0.55% | 608 |
| 24x2 | 75.566 | 75.598 | 75.227 | 75.398 | -0.28% | 994 |
| merge-24x1 | 73.794 | 73.010 | 73.505 | 73.579 | +0.09% | 904 |

All timings are local medians of three process means; the 8.49 ms C7a result belongs to the retained candidate executable. The integrated SDK has its own digest and requires its own C7a comparison.
