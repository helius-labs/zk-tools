# Frontend width and G2 partitions on C7a 1×2

Seven blocks rotated seven configurations through every order position on one c7a.24xlarge. The main pool used 96 physical cores throughout. Each process primed three proofs and timed 31. The results give the median of process means and the median paired percentage change, with faster block counts. CPU time covers warm proofs. RSS includes preparation. Every timed Aeglos request hit its prepared cache. All 49 exported proofs passed remote and independent local verification.

Release is the ordinary SDK. Ryzen is the eight-core frontend with two G2 ranges. Front16 changes only the frontend to sixteen cores. Ranges4 changes only G2 to four ranges with serial decomposition. Server applies both changes. Historic runs the archived AWS executable and its original frontend configuration. The supplied Apple references are 37 ms CPU and 24 ms Metal.

| Configuration | Proof ms | vs Aeglos | vs Ryzen | CPU-ms | Peak MiB | M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|---:|
| release | 12.430 | +51.70% (0/7) | +29.18% (0/7) | 622.9 | 59.4 | 2.98× / 1.93× |
| ryzen | 9.641 | +17.24% (0/7) | +0.00% (0/7) | 399.1 | 49.8 | 3.84× / 2.49× |
| front16 | 9.616 | +15.22% (0/7) | -1.03% (4/7) | 412.1 | 51.2 | 3.85× / 2.50× |
| ranges4 | 8.179 | +1.34% (3/7) | -15.13% (7/7) | 361.5 | 44.5 | 4.52× / 2.93× |
| server | 8.048 | -0.37% (5/7) | -16.74% (7/7) | 372.7 | 45.9 | 4.60× / 2.98× |
| historic | 8.171 | +1.56% (3/7) | -15.52% (7/7) | 382.8 | 47.4 | 4.53× / 2.94× |
| aeglos | 8.093 | +0.00% (0/7) | -14.71% (7/7) | 511.4 | 463.1 | 4.57× / 2.97× |

The fastest current configuration was server at 8.048 ms, against 8.093 ms for Aeglos. It won 5 of seven paired blocks.

[Configuration](artifacts/workflow.json.gz) · [Pinned inputs](artifacts/pins.json) · [Measurements](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
