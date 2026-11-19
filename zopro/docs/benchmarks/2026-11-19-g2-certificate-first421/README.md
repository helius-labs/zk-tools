# Evaluating G2 scalar diversity before sampling

On a Ryzen 9950X with 32 hardware threads, the prototype checks the existing 32 KiB bitmap before allocating the sample hash set. Each latency is the median of three process means, each with 31 timed SDK requests after three warmups. The paired percentages compare measurements in the same block. The eight varied 1×2 requests change the input coin and Merkle trees.

| Shape / witness | Bitmap first (ms) | Sampler first, same binary (ms) | Current release (ms) | Paired vs sampler first | Paired vs release |
|---|---:|---:|---:|---:|---:|
| 1x2 / 0 | 7.298 | 7.255 | 7.279 | +0.84% | -0.27% |
| 1x2 / 1 | 7.090 | 7.052 | 7.075 | +0.78% | +0.46% |
| 1x2 / 2 | 7.125 | 7.110 | 7.135 | -1.15% | -0.63% |
| 1x2 / 3 | 7.079 | 7.112 | 7.148 | -0.41% | -0.39% |
| 1x2 / 4 | 7.092 | 7.068 | 7.082 | +0.40% | +1.16% |
| 1x2 / 5 | 7.124 | 7.127 | 7.075 | -0.04% | +1.10% |
| 1x2 / 6 | 7.150 | 7.138 | 7.078 | -0.51% | +0.98% |
| 1x2 / 7 | 7.132 | 7.119 | 7.098 | +0.15% | +0.59% |
| 1x2 / 8 | 7.096 | 7.044 | 7.120 | +0.14% | -0.07% |
| 2x2 / 0 | 10.117 | 10.165 | 10.175 | -0.55% | -0.64% |

Across the eight varied witnesses, the median paired changes were +0.05% against the same binary and +0.53% against the release. The candidate won 4/8 and 3/8 respectively, including 2/8 against both controls. The result misses the prespecified latency criterion.

| Varied witnesses | Median CPU-ms per proof | Maximum peak RSS (MiB) |
|---|---:|---:|
| first | 146.9 | 64.5 |
| off | 146.7 | 65.2 |
| release | 146.5 | 66.2 |

The three route probes recorded 4, 4, 0 early exits respectively on the original 1×2, the first varied 1×2 and the original 2×2 request (four calls per process). All 93 final proof exports passed independent Gnark verification.

[Protocol](artifacts/protocol.md) · [Patch](artifacts/source.patch) · [Measurements](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
