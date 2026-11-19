# Public wire offsets for 2×2 at 32 Ryzen workers

Eight public training requests produced 4,222 B2 positions with one nonzero coefficient in common. The file occupies 152,040 bytes. Its contents were fixed before timing eight held-out transactions and the original request. The transaction families include private transfers, SPL deposits and withdrawals, and data-bearing outputs. Every held-out request has different public inputs. Each worker configuration used seven matched randomized blocks of 31 timed full SDK calls with three primes. The main pool had 32 logical workers and the frontend pool twelve workers on one L3.

| Input | Transaction | Previous release / candidate disabled / enabled ms | Paired vs release | Paired vs disabled | Peak RSS disabled / enabled MiB |
|---:|---|---:|---:|---:|---:|
| 0 | original | 9.925 / 9.805 / 9.576 | -3.52% (7/7) | -2.09% (5/7) | 124.0 / 118.0 |
| 41 | private split | 9.902 / 9.834 / 9.590 | -2.91% (7/7) | -2.54% (7/7) | 119.0 / 116.8 |
| 42 | deposit | 9.879 / 9.896 / 9.616 | -3.17% (7/7) | -2.73% (7/7) | 117.8 / 112.2 |
| 43 | withdrawal | 9.929 / 9.866 / 9.584 | -3.08% (7/7) | -2.78% (7/7) | 121.5 / 115.5 |
| 44 | data-bearing, zero second amount | 9.898 / 9.857 / 9.541 | -3.70% (7/7) | -3.23% (7/7) | 123.2 / 118.0 |
| 45 | private split | 9.925 / 9.866 / 9.553 | -3.75% (7/7) | -3.44% (7/7) | 117.2 / 117.5 |
| 46 | deposit | 9.897 / 9.871 / 9.585 | -3.43% (7/7) | -2.99% (7/7) | 118.5 / 117.5 |
| 47 | withdrawal | 9.923 / 9.852 / 9.544 | -4.15% (7/7) | -3.20% (7/7) | 118.2 / 115.0 |
| 48 | data-bearing, zero second amount | 9.939 / 9.862 / 9.568 | -3.40% (7/7) | -3.02% (7/7) | 123.8 / 116.0 |

Across the eight held-out inputs, the median paired change against release was -3.41% (8/8 faster) and -3.01% against disabled (8/8 faster). The original request took 9.576 ms, 6.16× M5 CPU and 3.13× Metal. Eager preparation took 13.54 ms per key. The 32-worker profile passed its qualification criterion. The 16-worker profile requires a separate measurement. The zero optional budget selected ordinary computation. At a 1 MiB optional budget the key accepted the 2×2 offset file. All 208 exports, including eight training proofs and two optional-budget probes, passed independent Gnark verification.

[Protocol](artifacts/protocol.md) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
 · [Public coefficient file](artifacts/public.wo)

## Latest measurements on the eight original requests

All rows use the same 32-worker Ryzen checkout binary. The 2×2 row is from the separate seven-block comparison above. The other seven rows are from the five-block [eight-shape scorecard](../2026-11-19-qualified-wire463/README.md). Both 1×2 and 2×2 use their own optional coefficient files.

| Shape | Warm proof ms | Peak RSS MiB | M5 CPU ms | M5 Metal ms | Speedup vs CPU | Speedup vs Metal |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.074 | 66.2 | 37 | 24 | 5.23× | 3.39× |
| 2x2 | 9.576 | 118.0 | 59 | 30 | 6.16× | 3.13× |
| 4x2 | 15.279 | 257.2 | 109 | 48 | 7.13× | 3.14× |
| 8x2 | 24.273 | 428.2 | 189 | 74 | 7.79× | 3.05× |
| 12x2 | 32.226 | 598.5 | 284 | 101 | 8.81× | 3.13× |
| 16x2 | 40.298 | 735.2 | 335 | 124 | 8.31× | 3.08× |
| 24x2 | 66.996 | 1127.5 | 528 | 176 | 7.88× | 2.63× |
| merge-24x1 | 65.215 | 1031.2 | 507 | 175 | 7.77× | 2.68× |

Across these measurements, 0/8 shapes reach 10× the supplied M5 CPU reference.
