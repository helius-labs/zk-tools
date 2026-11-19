# Affine batch capacity at curve-window completion

On the original 1×2 request, the final batch took 166 of 415 queued G1 points and 254 of 921 queued G2 points per proof. These counts were identical at 16 and 32 workers. With both changes enabled the paired median change against the same binary with both disabled was +0.06% at 32 workers and −0.29% at 16. Enabling G1 alone gave the fastest 32-worker reference median (6.935 ms), but it lost eight of ten pairs at 16 workers. Keep the release path. The smaller memory peaks on some inputs and larger peaks on others give no consistent memory advantage.

The five request and worker combinations each have ten balanced blocks per arm, with a separate CPU profile for each. The frozen release is an additional build control. Each latency is the median of process means. The paired columns compare with both changes disabled in the candidate executable. All 156 native core tests passed with both changes enabled, and all 280 exported proofs independently verified.

| Input and workers | Release / disabled / G1 / G2 / both ms | G1 / G2 / both vs disabled | Peaks in the same order, MiB | Both vs M5 CPU / Metal |
|---|---:|---:|---:|---:|
| 1x2-0-w32 | 7.012 / 6.990 / 6.935 / 6.963 / 6.971 | -0.89% (6/10) / -0.60% (6/10) / +0.06% (5/10) | 62.5 / 61.0 / 61.5 / 62.2 / 61.2 | 5.31× / 3.44× |
| 1x2-0-w16 | 8.210 / 8.144 / 8.165 / 8.124 / 8.122 | +0.20% (2/10) / -0.17% (6/10) / -0.29% (6/10) | 57.0 / 56.5 / 60.2 / 57.0 / 58.0 | 4.56× / 2.95× |
| 1x2-29-w32 | 6.984 / 6.927 / 6.924 / 6.927 / 6.905 | -0.06% (5/10) / -0.22% (6/10) / -0.18% (6/10) | 58.2 / 59.0 / 59.0 / 59.2 / 58.8 | — |
| 1x2-32-w32 | 7.021 / 6.951 / 6.929 / 6.958 / 6.969 | -0.27% (6/10) / +0.61% (4/10) / +0.09% (4/10) | 58.5 / 61.5 / 58.2 / 58.0 / 59.5 | — |
| 2x2-0-w32 | 9.645 / 9.596 / 9.618 / 9.568 / 9.616 | -0.14% (6/10) / -0.57% (7/10) / -0.20% (7/10) | 110.8 / 109.5 / 110.2 / 109.2 / 108.2 | 6.14× / 3.12× |

[Protocol](protocol.md) · [All paired comparisons](artifacts/summary.json) · [Diagnostic events](artifacts/traces.json.gz) · [Samples and CPU profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
