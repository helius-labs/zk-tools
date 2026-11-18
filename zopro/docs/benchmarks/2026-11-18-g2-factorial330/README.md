# Separating G2 window width from point partition count

The same binary varies the two G2 parameters independently. The 12-bit/two-range cell uses the original geometry. The exact integrated and earlier experimental executables serve as diagonal controls. All cells used 32 logical Ryzen workers and eight frontend workers. Each latency is the median of five process means.

| Shape | Geometry | Median ms | Paired change vs 12-bit/two-range | Faster blocks | CPU ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | integrated | 7.671 | +0.47% | 2/5 | 152 | 63 |
| 1x2 | old-four | 7.888 | +1.86% | 1/5 | 159 | 63 |
| 1x2 | 12x2 | 7.659 | +0.00% | 0/5 | 151 | 62 |
| 1x2 | 11x2 | 7.881 | +3.25% | 1/5 | 151 | 61 |
| 1x2 | 12x4 | 7.875 | +2.31% | 1/5 | 160 | 74 |
| 1x2 | 11x4 | 8.010 | +5.34% | 1/5 | 162 | 66 |
| 2x2 | integrated | 10.933 | -1.32% | 3/5 | 204 | 107 |
| 2x2 | old-four | 10.849 | +0.27% | 2/5 | 213 | 110 |
| 2x2 | 12x2 | 10.912 | +0.00% | 0/5 | 206 | 110 |
| 2x2 | 11x2 | 10.803 | -0.81% | 4/5 | 204 | 106 |
| 2x2 | 12x4 | 11.037 | +0.66% | 1/5 | 220 | 113 |
| 2x2 | 11x4 | 11.061 | +1.77% | 2/5 | 214 | 111 |

Probes counted 22, 24, 44 and 48 partition calls for 12×2, 11×2, 12×4 and 11×4. The zero-allowance 12×4 request used the original 22 calls. These counts refer to digit width and point partitions, distinct from circuit shape. All 69 exported proofs verified.

[Protocol](artifacts/protocol.md) · [Process measurements](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Source](artifacts/source.patch)
