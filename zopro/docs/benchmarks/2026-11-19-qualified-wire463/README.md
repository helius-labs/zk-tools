# Eight-shape Ryzen scorecard with optional public wire offsets

The candidate adds a CPU preparation API for externally supplied public coefficients. The original 1×2 uses the 42,096-byte file from experiment 461. Other circuit shapes use their prior per-shape settings. A Ryzen 9950X has 16 physical cores and 32 logical workers. Five randomized matched blocks used the same key, request and worker placement for both binaries. Small shapes used 51 timed full SDK requests per process after three primes, and larger shapes used eleven. Latency is the median of process means. Each paired change is the median within-block percentage difference.

| Shape | Workers | Previous / candidate ms | Paired vs previous | Candidate CPU-ms | Peak RSS previous / candidate MiB | M5 CPU / Metal ms | M5 CPU / Metal speedup |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 32 | 7.227 / 7.074 | -2.22% (5/5) | 144.6 | 62.0 / 66.2 | 37 / 24 | 5.23× / 3.39× |
| 2x2 | 32 | 9.849 / 9.823 | -0.26% (3/5) | 198.8 | 126.8 / 124.5 | 59 / 30 | 6.01× / 3.05× |
| 2x2 | 16 | 11.209 / 11.345 | +0.47% (1/5) | 135.0 | 98.8 / 105.0 | 59 / 30 | 5.20× / 2.64× |
| 4x2 | 32 | 15.434 / 15.279 | -0.76% (4/5) | 405.6 | 276.0 / 257.2 | 109 / 48 | 7.13× / 3.14× |
| 8x2 | 32 | 23.964 / 24.273 | +1.49% (2/5) | 671.0 | 430.8 / 428.2 | 189 / 74 | 7.79× / 3.05× |
| 12x2 | 32 | 32.262 / 32.226 | +0.28% (1/5) | 892.6 | 596.5 / 598.5 | 284 / 101 | 8.81× / 3.13× |
| 12x2 | 16 | 36.022 / 36.256 | +1.39% (1/5) | 526.9 | 504.2 / 509.0 | 284 / 101 | 7.83× / 2.79× |
| 16x2 | 32 | 40.340 / 40.298 | +0.70% (2/5) | 1159.9 | 730.8 / 735.2 | 335 / 124 | 8.31× / 3.08× |
| 24x2 | 32 | 67.984 / 66.996 | -2.38% (5/5) | 1947.3 | 1117.0 / 1127.5 | 528 / 176 | 7.88× / 2.63× |
| merge-24x1 | 32 | 65.573 / 65.215 | -0.64% (3/5) | 1865.7 | 1029.5 / 1031.2 | 507 / 175 | 7.77× / 2.68× |

For 1×2 the same candidate executable with the file omitted took 7.175 ms. The file changed paired warm latency by -1.22% (5/5 faster blocks). At 32 workers 0/8 shapes met the 10× CPU target. The native core passed 155 tests and the portable core passed 106. Ten malformed or foreign offset files were rejected in direct SDK checks, with correct separate and combined K/H proofs on a second key. The 1 MiB optional budget admitted the file, and a zero budget selected ordinary computation. Gnark verified all 109 exports.

[Protocol](artifacts/protocol.md) · [Samples](artifacts/processes.json.gz) · [Source and test identities](artifacts/checks.json) · [Verification](artifacts/verification.json) · [Public 1×2 coefficient file](../2026-11-19-external-wire461/artifacts/public.wo) · [Transaction variety check](../2026-11-19-wire-variants462/README.md)
