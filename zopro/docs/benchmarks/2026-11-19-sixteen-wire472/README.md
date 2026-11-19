# Public wire offset qualification on 16×2

Eight public training transactions supplied 27,132 fixed nonzero coefficients from 358,343 B2 positions. The key-bound file occupies 976,800 bytes. Eight further transactions varied the coins, Merkle tree and transfer mode. The original and eight held-out inputs used the same accepted prover binary at 32 pinned logical workers on the Ryzen 9950X. Each of seven randomized matched blocks primed three requests and timed eleven full verified SDK calls per process. Both arms used the accepted 16×2 FFT and curve-query settings.

| Input | Ordinary / offset ms | Paired change (wins) | Peak RSS ordinary / offset MiB |
|---:|---:|---:|---:|
| 0 | 40.108 / 39.553 | -0.66% (5/7) | 731.0 / 670.1 |
| 73 | 40.869 / 40.123 | -1.65% (6/7) | 718.5 / 674.7 |
| 74 | 42.316 / 39.973 | -2.37% (5/7) | 726.5 / 692.3 |
| 75 | 41.164 / 39.516 | -3.84% (5/7) | 735.5 / 679.7 |
| 76 | 40.682 / 39.954 | -2.16% (6/7) | 735.2 / 679.3 |
| 77 | 40.782 / 39.993 | -2.18% (6/7) | 739.0 / 689.1 |
| 78 | 40.711 / 39.246 | -3.53% (6/7) | 736.8 / 695.6 |
| 79 | 40.397 / 40.610 | +0.38% (3/7) | 730.5 / 681.6 |
| 80 | 40.716 / 39.677 | -2.08% (7/7) | 723.5 / 687.3 |

Across the eight held-out inputs the median paired change was -2.17% and 7/8 favored the file. The original request took 39.553 ms, 8.47× M5 CPU and 3.14× Metal. Eager setup took 84.16 ms per key. At optional preparation budgets of 0, 1, 4 and 8 MiB the prover selected ordinary proving, ordinary proving, offsets, offsets, respectively. The profile qualified. Gnark independently verified 146 exports.

[Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
 · [Optional 16×2 file](artifacts/public.wo)

## Current 32-worker Ryzen scorecard

The 16×2 row comes from this seven-block run. The other rows use the same production executable from the [eight-shape comparison](../2026-11-19-integrated-wire467/README.md).

| Shape | Warm proof ms | Peak RSS MiB | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|---:|
| 1x2 | 7.033 | 65.0 | 37.0 / 24.0 | 5.26× / 3.41× |
| 2x2 | 9.619 | 117.5 | 59.0 / 30.0 | 6.13× / 3.12× |
| 4x2 | 15.421 | 263.5 | 109.0 / 48.0 | 7.07× / 3.11× |
| 8x2 | 23.898 | 436.2 | 189.0 / 74.0 | 7.91× / 3.10× |
| 12x2 | 31.064 | 580.4 | 284.0 / 101.0 | 9.14× / 3.25× |
| 16x2 | 39.553 | 670.1 | 335.0 / 124.0 | 8.47× / 3.14× |
| 24x2 | 67.619 | 1117.0 | 528.0 / 176.0 | 7.81× / 2.60× |
| merge-24x1 | 66.450 | 1026.8 | 507.0 / 175.0 | 7.63× / 2.63× |
