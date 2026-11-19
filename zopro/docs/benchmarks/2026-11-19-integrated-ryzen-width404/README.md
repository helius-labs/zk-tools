# Integrated prover across the eight M5 reference shapes on Ryzen

Sixteen workers used one hardware thread on each Ryzen 9950X core. Thirty-two workers used both hardware threads on the same cores. Each cell contains five fresh process means with fixed worker placement. Task claiming and packing are enabled on 1×2 and 2×2. The 16×2 runs use compact Poseidon output addresses. Each request generates and verifies one proof.

| Shape | 16 workers ms | 32 workers ms | Paired change at 32 (wins / 5) | Peak MiB 16 / 32 | CPU-ms 16 / 32 | M5 CPU / Metal ms | 32-worker speedup CPU / Metal | Blocks reaching 10× CPU / 5 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.392 | 7.248 | -14.09% (5) | 59.5 / 65.8 | 100.5 / 150.9 | 37 / 24 | 5.10× / 3.31× | 0 |
| 2x2 | 11.499 | 10.074 | -12.50% (5) | 109.8 / 119.0 | 136.7 / 202.0 | 59 / 30 | 5.86× / 2.98× | 0 |
| 4x2 | 16.940 | 15.436 | -9.62% (5) | 220.5 / 260.0 | 235.1 / 406.5 | 109 / 48 | 7.06× / 3.11× | 0 |
| 8x2 | 26.632 | 24.143 | -10.53% (5) | 360.8 / 426.0 | 390.2 / 661.3 | 189 / 74 | 7.83× / 3.07× | 0 |
| 12x2 | 38.725 | 33.718 | -12.90% (5) | 524.2 / 619.8 | 572.5 / 936.8 | 284 / 101 | 8.42× / 3.00× | 0 |
| 16x2 | 44.857 | 40.657 | -9.36% (5) | 595.5 / 708.5 | 663.0 / 1176.1 | 335 / 124 | 8.24× / 3.05× | 0 |
| 24x2 | 73.293 | 69.141 | -5.66% (5) | 1000.0 / 1125.9 | 1104.5 / 2016.7 | 528 / 176 | 7.64× / 2.55× | 0 |
| merge-24x1 | 71.838 | 67.014 | -6.05% (4) | 893.8 / 1041.5 | 1056.6 / 1922.9 | 507 / 175 | 7.57× / 2.61× | 0 |

Using SMT reduced median latency on all eight shapes, winning 39 of 40 matched blocks. CPU time per proof and the observed process peak increased on every shape. At 32 workers the closest shape to the CPU target was 12×2 at 33.718 ms against 28.4 ms. All 40 process means remained above their respective 10× CPU targets. These are single-request latency results for this local Ryzen. The C7a scorecard used a different machine and build.

[Protocol](artifacts/protocol.md) · [Process samples](artifacts/samples.json.gz) · [Verification](artifacts/verification.json) · [C7a scorecard](../2026-11-18-overlap-c7a369/README.md)
