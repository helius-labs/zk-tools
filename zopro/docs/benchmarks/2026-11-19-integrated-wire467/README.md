# Public wire offsets on the general CPU solver

A Ryzen 9950X ran all eight original requests on 32 pinned logical workers. Two additional 16-worker configurations checked the smaller main pool. Five randomized matched blocks compared the previous release and the checkout build using the same worker placement. Each process primed three proofs, then timed 51 full verified SDK requests on 1×2 and 2×2 or eleven on larger shapes. The table reports the median process mean, median within-block percentage change and maximum whole-process RSS. The 1×2 and 32-worker 2×2 used their previously qualified files on both binaries. The new build used the 737,616-byte 12×2 file only at 32 workers.

| Shape | Workers | Previous / checkout ms | Paired change (wins) | Peak RSS MiB | M5 CPU / Metal ms | Speedup vs CPU / Metal |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 32 | 7.055 / 7.033 | -0.64% (3/5) | 65.0 | 37 / 24 | 5.26× / 3.41× |
| 2x2 | 32 | 9.555 / 9.619 | +1.17% (1/5) | 117.5 | 59 / 30 | 6.13× / 3.12× |
| 2x2 | 16 | 11.246 / 11.228 | -0.17% (3/5) | 105.0 | 59 / 30 | 5.25× / 2.67× |
| 4x2 | 32 | 15.198 / 15.421 | +1.01% (0/5) | 263.5 | 109 / 48 | 7.07× / 3.11× |
| 8x2 | 32 | 23.924 / 23.898 | +0.63% (2/5) | 436.2 | 189 / 74 | 7.91× / 3.10× |
| 12x2 | 32 | 32.894 / 31.064 | -5.10% (5/5) | 580.4 | 284 / 101 | 9.14× / 3.25× |
| 12x2 | 16 | 36.070 / 36.918 | +3.42% (1/5) | 532.5 | 284 / 101 | 7.69× / 2.74× |
| 16x2 | 32 | 39.877 / 39.860 | +1.06% (2/5) | 703.0 | 335 / 124 | 8.40× / 3.11× |
| 24x2 | 32 | 66.586 / 67.619 | +1.54% (2/5) | 1117.0 | 528 / 176 | 7.81× / 2.60× |
| merge-24x1 | 32 | 65.757 / 66.450 | -0.06% (3/5) | 1026.8 | 507 / 175 | 7.63× / 2.63× |

On 32-worker 12×2 the identical checkout binary without the file took 32.030 ms. Enabling the file changed paired latency by -3.43% (5/5 blocks faster). The general solver restores the original public inputs after gathering the A and B scalars, then adds key-bound correction points to A, private K and B2. A separate [qualification on eight held-out transactions](../2026-11-19-general-wire466/README.md) measured setup and fresh-input latency.

The native core passed 155 tests and the portable core passed 106. Low-budget probes selected ordinary proving at zero and 1 MiB, and enabled the file at 4 MiB. On both 4×2 and 12×2, the complete five-part outputs of ordinary solve and explicit parts matched their pre-preparation values byte for byte. The checks exercised arbitrary coefficients, invalid-file replacement and reloading the solver plan. All 120 process and route exports verified independently with Gnark. 0/8 original Ryzen shapes reached 10× the M5 CPU reference.

The initial 16-worker 12×2 comparison with the file omitted triggered a nine-block recheck. The previous build measured 35.751 ms and the checkout 35.576 ms (-0.06% paired, 5/9 faster). All 18 additional exports passed independent verification. [Recheck samples](artifacts/recheck.json.gz)

[Samples](artifacts/processes.json.gz) · [Release and candidate settings](artifacts/settings.json) · [Verification](artifacts/verification.json) · [Optional 12×2 public coefficients](artifacts/public.wo)
