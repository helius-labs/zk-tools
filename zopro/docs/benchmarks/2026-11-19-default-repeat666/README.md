# Recheck the three largest disabled-build regressions

From the 41-shape Ryzen screen, select the three largest paired latency increases of the integrated binary with the option disabled against the previous release. Each independent case repeats twelve balanced blocks, with three primes and thirteen timed proofs per process. The CPU sampling processes run separately. Off and on differ only in the grouped G2 option. All 117 independently exported proofs verified against the original circuits.

| Shape | Mode | Warm ms | Paired vs prior release | Paired vs off | Peak MiB |
|---|---|---:|---:|---:|---:|
| 6x8 | release | 23.059 | +0.00% (0/12) | +0.28% (5/12) | 416.5 |
| 6x8 | off | 22.942 | -0.28% (7/12) | +0.00% (0/12) | 429.5 |
| 6x8 | on | 22.771 | -1.26% (8/12) | -0.43% (7/12) | 423.0 |
| 4x4 | release | 16.432 | +0.00% (0/12) | +0.13% (5/12) | 286.2 |
| 4x4 | off | 16.446 | -0.13% (7/12) | +0.00% (0/12) | 288.0 |
| 4x4 | on | 16.157 | -2.02% (11/12) | -1.46% (12/12) | 295.2 |
| 2x8 | release | 13.926 | +0.00% (0/12) | -1.22% (11/12) | 249.8 |
| 2x8 | off | 14.079 | +1.23% (1/12) | +0.00% (0/12) | 256.0 |
| 2x8 | on | 14.116 | +1.55% (3/12) | -0.33% (7/12) | 247.5 |

The disabled-build regressions on 6×8 and 4×4 did not repeat. On 2×8 the previous executable remained faster, winning 11 of 12 disabled comparisons. The disabled build cost 1.23% paired and the enabled build cost 1.55% paired against the previous release. Retain the previous executable for latency-sensitive 2×8 deployments. The 5×16 selection is specific to its measured configuration.

[Initial all-shape screen](../2026-11-19-wide-catalog662/README.md) · [Configuration](artifacts/configs.json.gz) · [Timings and CPU profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
