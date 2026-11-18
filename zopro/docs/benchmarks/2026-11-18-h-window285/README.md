# Wider temporary buckets for the dense H query on Ryzen

The 13-bit variant reduced full-proof time against disabled in 15/15 pairs across three shapes. The 14-bit variant won 14/15. On the primary merge24 fixture both passed the local screen.

The prototype changed the radix only inside the dense Montgomery H query. The separate workspace forecast charges the selected bucket geometry.

| Shape | Mode | Median ms | Paired wall change | Wins / 5 | CPU ms | Paired CPU change | Peak MiB | M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| merge-24x1 | accepted | 73.132 | -0.82% | 5 | 1078.8 | -0.19% | 874.8 | 6.93× / 2.39× |
| merge-24x1 | off | 73.893 | +0.00% | — | 1078.7 | +0.00% | 870.0 | 6.86× / 2.37× |
| merge-24x1 | 13 | 71.905 | -2.47% | 5 | 1066.0 | -1.08% | 885.2 | 7.05× / 2.43× |
| merge-24x1 | 14 | 71.534 | -2.62% | 4 | 1057.7 | -2.05% | 856.5 | 7.09× / 2.45× |
| 16x2 | accepted | 46.229 | -0.44% | 3 | 683.0 | -0.01% | 607.2 | 7.25× / 2.68× |
| 16x2 | off | 46.431 | +0.00% | — | 683.1 | +0.00% | 605.8 | 7.21× / 2.67× |
| 16x2 | 13 | 45.462 | -2.44% | 5 | 676.2 | -0.99% | 604.0 | 7.37× / 2.73× |
| 16x2 | 14 | 45.333 | -2.45% | 5 | 670.2 | -1.78% | 605.0 | 7.39× / 2.74× |
| 24x2 | accepted | 75.722 | -0.06% | 3 | 1123.1 | +0.03% | 938.2 | 6.97× / 2.32× |
| 24x2 | off | 75.871 | +0.00% | — | 1126.7 | +0.00% | 915.4 | 6.96× / 2.32× |
| 24x2 | 13 | 73.270 | -3.43% | 5 | 1111.5 | -1.53% | 951.8 | 7.21× / 2.40× |
| 24x2 | 14 | 72.870 | -4.60% | 5 | 1101.2 | -2.57% | 977.1 | 7.25× / 2.42× |

Each cell uses five fresh processes with rotated execution order at 16 physical Ryzen cores, 11 timed requests and three primes per process. Paired ratios use the off run in the same block. CPU is summed user and system time per proof. Each RSS entry is the maximum process lifetime peak across the five runs. All 60 exported proofs passed independent Gnark verification. [Protocol](artifacts/protocol.md) · [Block ratios](artifacts/cells.json) · [Per-process records](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)

The native generator-sum oracle covered the 13- and 14-bit kernel with zeros, negative one, large scalars, repeated points and identity points. The tested code stays isolated from the production path. [Source patch](artifacts/source.patch.gz) · [Native test](artifacts/tests.json)
