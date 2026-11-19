# Combined K and H streams on the current 2×2 SDK entry

Seven randomized blocks ran the public SDK example on Ryzen. The sixteen-worker processes used eight frontend workers on the same L3. The 32-worker processes added four SMT siblings to that frontend. Each process timed 51 verified requests after three warmups. The candidate streams K and H together into the existing width-13 buckets. The option requires workspace admission. Both worker configurations ran additional routing checks with the optional budget set to zero.

| Main workers | Release / disabled / enabled ms | Paired vs release | Paired vs disabled | CPU-ms disabled / enabled | Peak MiB disabled / enabled | M5 CPU / Metal speedup |
|---|---:|---:|---:|---:|---:|---:|
| 16 | 11.466 / 11.480 / 11.277 | -1.74% (6/7) | -1.61% (6/7) | 136.5 / 134.9 | 113.0 / 110.8 | 5.23× / 2.66× |
| 32 | 9.939 / 9.776 / 9.669 | -1.48% (7/7) | -0.97% (6/7) | 200.0 / 197.0 | 121.0 / 112.5 | 6.10× / 3.10× |

Both worker counts passed the original-input comparison. The candidate won six of seven pairs against the identical disabled build at each width. Both profiles proceed to the eight-witness comparison.

All 48 exports passed independent Gnark verification. The zero-budget checks selected the fallback.

[Source](artifacts/source.patch) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Earlier 16-worker experiment](../2026-11-18-medium-kh-confirm338/README.md)
