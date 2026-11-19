# Verified Ryzen scorecard for the integrated public SDK benchmark

All eight shapes used the same public Rust benchmark executable. Each arm ran five blocks of warmed SDK requests, with 51 calls per process for 1×2 and 2×2 and eleven for larger shapes. The table reports median process means, median within-block change and maximum process RSS. The arms differ only for 2×2 at 16 workers, where the current profile uses a shared K/H bucket set. The other rows repeat identical settings.

| Shape | Main workers | Previous / current ms | Paired change | Wins / 5 | CPU-ms current | Peak MiB previous / current | M5 CPU / Metal ms | M5 CPU / Metal speedup |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 32 | 7.197 / 7.242 | +0.20% | 1 | 148.3 | 62.5 / 61.8 | 37 / 24 | 5.11× / 3.31× |
| 2x2 | 32 | 9.832 / 9.789 | -0.12% | 5 | 198.8 | 122.0 / 124.2 | 59 / 30 | 6.03× / 3.06× |
| 2x2 | 16 | 11.434 / 11.248 | -0.74% | 5 | 135.0 | 111.8 / 104.2 | 59 / 30 | 5.25× / 2.67× |
| 4x2 | 32 | 15.204 / 15.581 | +3.16% | 1 | 411.4 | 261.0 / 257.2 | 109 / 48 | 7.00× / 3.08× |
| 8x2 | 32 | 24.102 / 24.213 | -0.72% | 4 | 664.3 | 426.8 / 434.2 | 189 / 74 | 7.81× / 3.06× |
| 12x2 | 32 | 32.595 / 32.223 | -0.96% | 3 | 892.2 | 602.5 / 602.0 | 284 / 101 | 8.81× / 3.13× |
| 12x2 | 16 | 36.149 / 36.070 | +0.33% | 2 | 524.7 | 516.0 / 508.5 | 284 / 101 | 7.87× / 2.80× |
| 16x2 | 32 | 40.031 / 39.784 | -1.15% | 3 | 1150.4 | 720.5 / 719.2 | 335 / 124 | 8.42× / 3.12× |
| 24x2 | 32 | 67.803 / 67.781 | -0.03% | 3 | 1956.1 | 1114.1 / 1090.8 | 528 / 176 | 7.79× / 2.60× |
| merge-24x1 | 32 | 65.930 / 67.408 | +2.94% | 1 | 1887.3 | 1039.2 / 1074.3 | 507 / 175 | 7.52× / 2.60× |

At 32 workers 0/8 shapes reached 10× the supplied M5 CPU reference by median. Independent Gnark verification succeeded for all 100 exported proofs.

[Process samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Settings](artifacts/settings.json) · [Protocol](artifacts/protocol.md) · [Varied K/H comparison](../2026-11-19-varied-kh450/README.md)
