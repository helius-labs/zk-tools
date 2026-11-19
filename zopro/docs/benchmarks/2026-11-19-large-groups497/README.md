# Prepared point sums on larger transfer and merge proofs

Each shape uses its own groups of at least three varying, equal B2 scalars learned from verified public training fixtures. The two transfer shapes use eight training requests each. Merge pools eight sparse and eight populated training requests. Each group maps into A and private K. Three randomized blocks compare the release, groups disabled, and sampled point groups on the same 32-worker Ryzen. Each process primes three times, times eleven calls and exports a proof for Gnark verification. The following peak RSS includes preparation. M5 comparisons use the supplied original-request references.

| Shape | Request | Release / off / grouped ms | Paired vs release | Paired vs off | Peak MiB release / grouped | M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|---:|
| 16x2 | Original | 39.599 / 39.437 / 38.696 | -0.85% (2/3) | -3.27% (2/3) | 668.3 / 786.3 | 8.66× / 3.20× |
| 16x2 | 73 | 40.275 / 41.980 / 36.321 | -10.64% (3/3) | -12.85% (3/3) | 665.3 / 771.2 | — |
| 16x2 | 80 | 39.750 / 40.011 / 37.046 | -6.80% (3/3) | -7.41% (3/3) | 683.4 / 720.5 | — |
| 24x2 | Original | 64.008 / 64.358 / 63.589 | -0.52% (3/3) | -1.07% (3/3) | 1008.5 / 1077.2 | 8.30× / 2.77× |
| 24x2 | 97 | 63.833 / 63.383 / 57.335 | -10.18% (3/3) | -9.57% (3/3) | 1032.0 / 1086.3 | — |
| 24x2 | 104 | 65.116 / 62.619 / 56.813 | -12.78% (3/3) | -9.14% (3/3) | 1024.6 / 1145.2 | — |
| merge-24x1 | Original | 64.035 / 64.329 / 64.171 | -0.44% (2/3) | -0.66% (2/3) | 1061.5 / 1213.2 | 7.90× / 2.73× |
| merge-24x1 | 129 | 65.888 / 65.474 / 66.776 | +1.35% (1/3) | +1.59% (0/3) | 1086.2 / 1248.2 | — |
| merge-24x1 | 136 | 73.202 / 72.632 / 71.492 | -1.54% (3/3) | -0.36% (2/3) | 1225.0 / 1429.0 | — |

All 90 process exports verified, including nine additional instrumented route probes. Only two held-out requests per shape enter this screen. [Measurements](artifacts/processes.json.gz) · [Routes](artifacts/probes.json) · [Group sizes](artifacts/layouts.json) · [Verification](artifacts/verification.json)
