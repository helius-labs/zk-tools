# Ordinary-layout Euler FFT on C7a 2×2 proofs

The shift variant reached **11.926 / 11.954 ms** on the original and held-out inputs. It cleared both paired controls, with changes of **-1.44% (11/12) / -2.02% (12/12)** versus frozen and **-1.26% (9/12) / -1.61% (11/12)** versus the same-build ordinary mode. Prepared Aeglos took **11.640 / 11.544 ms**. The original request is **4.95× M5 CPU** and **2.52× Metal**.

The candidate combines the inner Euler FFT work within the ordinary outer packing and column layout. It uses the [native build previously checked on Ryzen](../2026-11-20-euler-ordinary-layout-ryzen908/README.md). Zopro uses 64 main and 32 frontend physical cores on one c7a.24xlarge. Prepared Aeglos uses all 96 cores with cache hits on every timed proof. The timing panel has 12 counterbalanced blocks per input, three primes and thirteen timed proofs per process. The independent clock panel measures only the inner packed FFT stage.

The ordinary control calibrated within 2% of frozen on both inputs. Each variant needs a lower paired median and at least nine matched wins against both controls on each input. The shift variant alone cleared that requirement. All 165 exports were independently verified on the runner and reverified after retrieval. The instance terminated and the temporary bucket was deleted. The cleanup check found zero tagged volumes and zero active benchmark instances.

| Input | Mode | Proof ms | Paired vs frozen | Paired vs ordinary | Paired vs Aeglos | Inner FFT ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| catalog | release | 12.105 | +0.00% (0/12) | +0.71% (5/12) | +3.77% (1/12) | — | 73.7 |
| catalog | ordinary | 12.013 | -0.71% (7/12) | +0.00% (0/12) | +3.16% (1/12) | — | 75.3 |
| catalog | fused | 11.872 | -1.39% (11/12) | -1.75% (8/12) | +1.35% (1/12) | 0.533 | 76.7 |
| catalog | paired | 11.916 | -0.92% (11/12) | -0.79% (8/12) | +1.94% (1/12) | 0.516 | 74.5 |
| catalog | shift | 11.926 | -1.44% (11/12) | -1.26% (9/12) | +2.58% (1/12) | 0.520 | 75.2 |
| catalog | aeglos | 11.640 | -3.63% (11/12) | -3.06% (11/12) | +0.00% (0/12) | — | 807.5 |
| heldout | release | 12.221 | +0.00% (0/12) | +1.22% (3/12) | +6.48% (0/12) | — | 76.8 |
| heldout | ordinary | 12.112 | -1.21% (9/12) | +0.00% (0/12) | +4.46% (0/12) | — | 73.8 |
| heldout | fused | 11.957 | -2.41% (10/12) | -1.07% (11/12) | +3.16% (0/12) | 0.527 | 75.8 |
| heldout | paired | 11.963 | -1.70% (12/12) | -1.23% (9/12) | +3.00% (0/12) | 0.535 | 79.0 |
| heldout | shift | 11.954 | -2.02% (12/12) | -1.61% (11/12) | +3.61% (1/12) | 0.533 | 79.7 |
| heldout | aeglos | 11.544 | -6.08% (12/12) | -4.27% (12/12) | +0.00% (0/12) | — | 809.5 |

## Eight supplied M5 references

The qualified shift profile updates the 2×2 row. The other rows use the [previously verified profiles](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings). The target remains 10× the M5 CPU timing on every shape. Five of the eight measured shapes meet it.

| Shape | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.858 | 37 / 24 | 4.71× / 3.05× |
| 2×2 | 11.926 | 59 / 30 | 4.95× / 2.52× |
| 4×2 | 14.726 | 109 / 48 | 7.40× / 3.26× |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× |

[Timing and verification data](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Source patch](artifacts/source.patch) · [Cleanup](artifacts/cleanup.json)
