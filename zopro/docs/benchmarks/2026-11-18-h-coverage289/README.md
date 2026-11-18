# H bucket coverage across the other five shapes on Ryzen

The 13-bit prototype was evaluated across the five remaining M5 shapes. Only 12×2 has enough active H rows to use this option. On the four smaller shapes, the enabled and disabled binaries execute the same H path.

| Shape | Mode | Median ms | Paired wall change | Wins / 5 | CPU ms | Paired CPU change | Peak MiB | M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | accepted | 8.347 | -1.08% | 4 | 104.8 | -0.15% | 71.5 | 4.43× / 2.88× |
| 1x2 | off | 8.456 | +0.00% | — | 104.9 | +0.00% | 72.2 | 4.38× / 2.84× |
| 1x2 | 13 | 8.440 | +0.23% | 2 | 104.5 | -0.14% | 69.0 | 4.38× / 2.84× |
| 2x2 | accepted | 11.853 | +0.24% | 2 | 150.0 | +0.22% | 158.2 | 4.98× / 2.53× |
| 2x2 | off | 11.827 | +0.00% | — | 150.1 | +0.00% | 161.5 | 4.99× / 2.54× |
| 2x2 | 13 | 11.820 | -0.56% | 4 | 149.8 | +0.37% | 156.0 | 4.99× / 2.54× |
| 4x2 | accepted | 16.914 | -0.43% | 3 | 235.4 | -0.28% | 212.5 | 6.44× / 2.84× |
| 4x2 | off | 16.788 | +0.00% | — | 234.9 | +0.00% | 214.8 | 6.49× / 2.86× |
| 4x2 | 13 | 16.906 | +0.94% | 1 | 236.7 | +0.22% | 220.0 | 6.45× / 2.84× |
| 8x2 | accepted | 26.337 | -0.74% | 3 | 389.9 | -0.22% | 365.5 | 7.18× / 2.81× |
| 8x2 | off | 26.447 | +0.00% | — | 390.4 | +0.00% | 363.5 | 7.15× / 2.80× |
| 8x2 | 13 | 26.460 | +0.11% | 2 | 389.7 | +0.14% | 356.2 | 7.14× / 2.80× |
| 12x2 | accepted | 38.657 | -2.96% | 3 | 570.5 | -0.47% | 500.8 | 7.35× / 2.61× |
| 12x2 | off | 39.374 | +0.00% | — | 573.4 | +0.00% | 507.8 | 7.21× / 2.57× |
| 12x2 | 13 | 38.099 | -1.54% | 5 | 565.8 | -1.34% | 498.5 | 7.45× / 2.65× |

Each cell uses five fresh processes with rotated execution order at 16 physical Ryzen cores, 11 timed requests and three primes per process. Paired ratios use the off run in the same block. CPU is summed user and system time per proof. Each RSS entry is the maximum process lifetime peak across the five runs. All 75 exported proofs passed independent Gnark verification. [Protocol](artifacts/protocol.md) · [Block ratios](artifacts/cells.json) · [Per-process records](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)

## All eight shapes at 16 physical Ryzen cores

The same opt-in 13-bit executable ran on the five shapes above and on three shapes in the [large-query panel](../2026-11-18-h-window285/README.md). Each row uses its own five process means. Only 12×2, 16×2, 24×2 and merge24 cross the H size threshold.

| Shape | Zopro 13-bit mode ms | M5 CPU ms | M5 Metal ms | CPU speedup | Metal speedup | 10× CPU target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.440 | 37 | 24 | 4.38× | 2.84× | 3.7 |
| 2x2 | 11.820 | 59 | 30 | 4.99× | 2.54× | 5.9 |
| 4x2 | 16.906 | 109 | 48 | 6.45× | 2.84× | 10.9 |
| 8x2 | 26.460 | 189 | 74 | 7.14× | 2.80× | 18.9 |
| 12x2 | 38.099 | 284 | 101 | 7.45× | 2.65× | 28.4 |
| 16x2 | 45.462 | 335 | 124 | 7.37× | 2.73× | 33.5 |
| 24x2 | 73.270 | 528 | 176 | 7.21× | 2.40× | 52.8 |
| merge-24x1 | 71.905 | 507 | 175 | 7.05× | 2.43× | 50.7 |

On 12×2 the enabled mode beat disabled in five of five blocks, with a median paired gain of 1.54%. The four ineligible shapes varied in both directions. The [48-core C7a panel](../2026-11-18-ccd-placement280/README.md) provides AWS results for the accepted build on these eight shapes.
