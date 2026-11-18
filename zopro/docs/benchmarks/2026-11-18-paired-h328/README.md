# Sharing a tile of H inputs between adjacent scalar fields

Each pair of fields scans the same base points and scalar words in tiles. Two independent bucket accumulators preserve arithmetic and collision handling. The optional memory forecast covers both windows while nested tasks execute. Five randomized full-proof blocks ran on 16 physical Ryzen cores with eleven timed requests per fresh process.

| Shape | Mode | Median ms | Paired change vs individual | Faster blocks | CPU ms | Peak MiB | M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|---:|
| merge-24x1 | reference | 70.518 | -1.00% | 3/5 | 1036 | 886 | 7.19× / 2.48× |
| merge-24x1 | single | 71.232 | +0.00% | —/5 | 1037 | 882 | 7.12× / 2.46× |
| merge-24x1 | 256 | 71.751 | +0.01% | 2/5 | 1051 | 919 | 7.07× / 2.44× |
| merge-24x1 | 2048 | 71.075 | -0.48% | 4/5 | 1039 | 878 | 7.13× / 2.46× |
| 16x2 | reference | 44.708 | -0.01% | 3/5 | 654 | 594 | 7.49× / 2.77× |
| 16x2 | single | 44.712 | +0.00% | —/5 | 654 | 573 | 7.49× / 2.77× |
| 16x2 | 256 | 45.244 | +1.04% | 0/5 | 666 | 606 | 7.40× / 2.74× |
| 16x2 | 2048 | 45.129 | +1.23% | 1/5 | 663 | 590 | 7.42× / 2.75× |
| 24x2 | reference | 71.672 | -1.44% | 3/5 | 1074 | 980 | 7.37× / 2.46× |
| 24x2 | single | 72.879 | +0.00% | —/5 | 1075 | 952 | 7.24× / 2.41× |
| 24x2 | 256 | 72.712 | -0.23% | 3/5 | 1089 | 941 | 7.26× / 2.42× |
| 24x2 | 2048 | 72.914 | +0.05% | 2/5 | 1090 | 941 | 7.24× / 2.41× |
| 12x2 | reference | 37.575 | +0.15% | 2/5 | 555 | 512 | 7.56× / 2.69× |
| 12x2 | single | 38.602 | +0.00% | —/5 | 556 | 513 | 7.36× / 2.62× |
| 12x2 | 256 | 38.304 | -0.71% | 3/5 | 562 | 503 | 7.41× / 2.64× |
| 12x2 | 2048 | 38.408 | -0.50% | 3/5 | 560 | 518 | 7.39× / 2.63× |

The generator-sum oracle covers field crossings, identities, signed bases, full-width field values and uneven tiles. Route probes checked both enabled tile sizes, individual fields, a smaller circuit and a zero optional-memory allowance. All 85 exports verified.

[Protocol](artifacts/protocol.md) · [Individual processes](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Patch against experiment 325](artifacts/source.patch)
