# Fused Euler FFT on 16 Ryzen cores

The fused transform reduced merge24 wall time in each of three paired blocks. The other shapes had mixed block results. The trial used warm, sequential single proofs on the native CPU path.

| Shape | Accepted ms | Disabled ms | Enabled ms | Paired wall change vs disabled | Faster blocks | Paired CPU change vs disabled | Peak MiB accepted / disabled / enabled | M5 CPU / Metal speedup, enabled |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 46.450 | 46.393 | 46.078 | -1.52% | 2/3 | -0.59% | 580.8 / 611.8 / 584.5 | 7.27× / 2.69× |
| merge-24x1 | 73.258 | 75.164 | 71.438 | -5.13% | 3/3 | -2.23% | 873.5 / 903.9 / 867.8 | 7.10× / 2.45× |
| 24x2 | 76.735 | 75.062 | 74.393 | -0.45% | 2/3 | -1.15% | 935.6 / 959.9 / 940.3 | 7.10× / 2.37× |

Each cell is the median of three process means, each covering 11 timed proofs after three primes. Percentages use within-block ratios. CPU is user plus system time per proof. Memory is the maximum process-lifetime RSS in an arm. The exact accepted binary provides a second control for rebuild variation.

All 27 exported proofs passed independent Gnark verification. The coefficient oracle matched the original pair of transforms on four domains and two generators. The generic fallback compiled. The fused path reuses the existing root tables and performs the central inverse butterfly, integer weighting and forward butterfly before repacking. Eligibility is determined by the Euler path and domain. The measurement driver recorded the general proof route without a dedicated fused-kernel counter.

The local result supports testing merge24 on C7a. Adoption requires target-machine evidence. See the [sample data](artifacts/rows.json.gz), [source patch](artifacts/source.patch) and [protocol](artifacts/protocol.md).
