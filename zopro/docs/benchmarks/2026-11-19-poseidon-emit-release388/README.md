# In-place Poseidon output addresses on Ryzen

The clean candidate enables eligible address encoding with `ZOPRO_CPU_POSEIDON_EMIT=1`. It checks every row and wire address after the existing coefficient recognition. Each supported S-box stores its addresses in the three existing task words. Unsupported S-boxes keep their original operation indices. Integrated 379 and exact prototype 384 are separate executable controls. The disabled and enabled columns use the same clean executable.

| Shape | Blocks | Release ms | Disabled ms | Prototype ms | Enabled ms | Paired vs release | Wins | Paired vs disabled | Wins | Peak MiB release / enabled | M5 CPU / Metal speedup |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 9 | 7.346 | 7.339 | 7.335 | 7.332 | -0.19% | 5 | -0.16% | 6 | 66.8 / 69.8 | 5.05× / 3.27× |
| 2x2 | 3 | 10.348 | 10.304 | 10.254 | 10.300 | -0.85% | 3 | -0.15% | 3 | 113.8 / 113.5 | 5.73× / 2.91× |
| 4x2 | 3 | 17.037 | 16.904 | 17.457 | 16.900 | -0.75% | 3 | -0.02% | 2 | 207.0 / 206.2 | 6.45× / 2.84× |
| 8x2 | 3 | 26.673 | 26.825 | 26.914 | 26.465 | -0.54% | 2 | -1.34% | 2 | 358.2 / 352.8 | 7.14× / 2.80× |
| 12x2 | 5 | 38.564 | 38.850 | 38.316 | 38.412 | -0.94% | 5 | -1.76% | 4 | 551.2 / 533.2 | 7.39× / 2.63× |
| 16x2 | 9 | 45.387 | 45.570 | 44.748 | 44.764 | -1.38% | 8 | -1.84% | 9 | 636.2 / 626.9 | 7.48× / 2.77× |
| 24x2 | 3 | 73.469 | 74.064 | 72.693 | 72.045 | -2.12% | 2 | -1.57% | 3 | 949.2 / 895.1 | 7.33× / 2.44× |
| merge-24x1 | 3 | 71.685 | 71.500 | 71.101 | 71.209 | -1.03% | 3 | -0.41% | 3 | 862.5 / 887.2 | 7.12× / 2.46× |

On a Ryzen 9950X, the enabled 16×2 proof took 44.764 ms. It won eight of nine blocks against the integrated release and nine of nine against the identical executable with encoding disabled. Peak RSS was 626.9 MiB enabled and 636.2 MiB for the release. On 1×2 the enabled build won five of nine pairs against the release. The integration therefore exposes an opt-in setting. The eight rows cover the M5 reference shapes. This build still needs a C7a measurement. [Prototype measurements](../2026-11-19-poseidon-emit384/README.md)

Each table entry is the median of per-process mean warm, verified proof times. The 1×2 and 2×2 runs used 32 pinned hardware threads, and the larger circuits used 16. Each process exported one proof for independent Gnark verification, for 153 exports across 4,537 timed requests.

The native core suite passed 148 tests, including full output comparisons across widths 2–13, both multiplication orientations, optional C rows, nonencodable addresses and a mixed encoded/fallback leaf. With zero optional preparation allowance, the enabled SDK exported a verified proof through the fallback.

The integrated source also passed the full native (148 tests) and generic x86-64 (106 tests) core suites with one test thread. A separate generic build compiled the optional sparse schedule and passed all 3 Poseidon tests with address encoding enabled. [Integration checks](artifacts/integration.json)

[Samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md) · [Clean source](artifacts/source.patch.gz)
