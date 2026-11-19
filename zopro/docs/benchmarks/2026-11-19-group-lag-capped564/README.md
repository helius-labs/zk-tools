# Scalar repeat-distance grouping under hard RAM limits

The same clean experimental executable runs with ordinary class construction or a single repeat-distance prediction. Five randomized blocks per shape and limit use 32 pinned Ryzen workers. A fresh Linux cgroup enforces memory.max and disables swap. Each process loads the key, primes three proofs and measures eleven requests. The supervisor records charged peak memory and local OOM kills. The filesystem cache was warm and some shared cached pages may be charged elsewhere. Paired times include only blocks that completed both arms.

| Shape | Hard cap MiB | Completed ordinary / candidate | Median ordinary / candidate ms | Paired vs ordinary | Peak charged MiB ordinary / candidate | Candidate vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|---:|
| 12x2 | 400 | 5/5 / 5/5 | 87.328 / 87.946 | +0.41% (2/5) | 164.6 / 164.2 | 3.23× / 1.15× |
| 12x2 | 768 | 5/5 / 5/5 | 56.982 / 57.032 | +0.05% (2/5) | 258.9 / 264.1 | 4.98× / 1.77× |
| 12x2 | 4096 | 5/5 / 5/5 | 31.372 / 31.172 | -0.21% (4/5) | 417.5 / 417.3 | 9.11× / 3.24× |
| 16x2 | 640 | 5/5 / 5/5 | 90.085 / 90.637 | +0.65% (1/5) | 261.6 / 255.9 | 3.70× / 1.37× |
| 16x2 | 1024 | 5/5 / 5/5 | 78.041 / 77.757 | -0.57% (4/5) | 312.5 / 320.7 | 4.31× / 1.59× |
| 16x2 | 4096 | 5/5 / 5/5 | 39.810 / 40.788 | +2.06% (1/5) | 459.7 / 494.0 | 8.21× / 3.04× |
| merge-24x1 | 950 | 5/5 / 5/5 | 141.624 / 142.445 | +0.28% (1/5) | 403.9 / 395.9 | 3.56× / 1.23× |
| merge-24x1 | 1536 | 5/5 / 5/5 | 125.218 / 123.232 | -1.37% (5/5) | 637.4 / 632.9 | 4.11× / 1.42× |
| merge-24x1 | 4096 | 5/5 / 5/5 | 80.679 / 83.977 | +7.93% (1/5) | 751.2 / 750.9 | 6.04× / 2.08× |

90 of 90 processes exported a proof. Every completed export passed independent verification. [Samples and limit events](artifacts/samples.json.gz) · [Scope supervisor](artifacts/scoped.py) · [Verification](artifacts/verification.json)
