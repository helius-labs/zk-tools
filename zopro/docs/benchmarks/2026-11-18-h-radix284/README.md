# Narrower windows for large raw curve queries on Ryzen

The accepted build was faster on all three shapes with its default radix. Reducing the maximum width for large inputs to 10 or 11 bits increased end-to-end time in all 30 paired comparisons.

| Shape | Mode | Median ms | Paired wall change | Wins / 5 | CPU ms | Paired CPU change | Peak MiB | M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| merge-24x1 | default | 73.601 | +0.00% | — | 1078.7 | +0.00% | 859.2 | 6.89× / 2.38× |
| merge-24x1 | 10 | 82.337 | +10.46% | 0 | 1162.5 | +7.18% | 865.5 | 6.16× / 2.13× |
| merge-24x1 | 11 | 76.130 | +3.50% | 0 | 1111.4 | +2.76% | 892.0 | 6.66× / 2.30× |
| 16x2 | default | 46.374 | +0.00% | — | 683.8 | +0.00% | 583.0 | 7.22× / 2.67× |
| 16x2 | 10 | 50.620 | +9.58% | 0 | 737.9 | +7.79% | 590.2 | 6.62× / 2.45× |
| 16x2 | 11 | 47.772 | +4.43% | 0 | 704.2 | +3.02% | 597.0 | 7.01× / 2.60× |
| 24x2 | default | 75.134 | +0.00% | — | 1124.3 | +0.00% | 948.8 | 7.03× / 2.34× |
| 24x2 | 10 | 83.832 | +11.51% | 0 | 1213.9 | +7.74% | 957.3 | 6.30× / 2.10× |
| 24x2 | 11 | 78.393 | +4.16% | 0 | 1157.3 | +2.86% | 961.2 | 6.74× / 2.25× |

Each cell uses five fresh processes with rotated execution order at 16 physical Ryzen cores, 11 timed requests and three primes per process. Paired ratios use the default run in the same block. CPU is summed user and system time per proof. Each RSS entry is the maximum process lifetime peak across the five runs. All 45 exported proofs passed independent Gnark verification. [Protocol](artifacts/protocol.md) · [Block ratios](artifacts/cells.json) · [Per-process records](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
