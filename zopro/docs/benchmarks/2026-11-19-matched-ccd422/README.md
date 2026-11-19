# How shared L3 placement affects Zopro and Aeglos on Ryzen

Each prover used eight physical cores without SMT on the same Ryzen 9950X. Compact placement used eight cores sharing one L3. Spread placement used four cores from each L3. Eight matched blocks alternated complementary CPU masks. Each fresh process warmed up three times and timed 31 complete proofs on 1×2 or eleven on 16×2. Zopro’s 1×2 frontend shared the eight cores with its main workers. Aeglos used its warm key cache. Both were constrained by the process affinity mask. Zopro also verified each main worker’s pinning before and after timing.

| Shape | Prover | Compact ms | Spread ms | Spread vs compact, paired | Compact wins / 8 | Paired change by mask half | Peak MiB compact / spread | CPU-ms compact / spread |
|---|---|---:|---:|---:|---:|---|---:|---:|
| 1x2 | Zopro | 13.002 | 13.198 | +1.43% | 5 | +3.06% / -0.62% | 47.2 / 49.0 | 93.4 / 94.9 |
| 1x2 | Aeglos | 18.624 | 19.506 | +5.05% | 8 | +7.23% / +2.66% | 413.6 / 417.5 | 146.1 / 149.5 |
| 16x2 | Zopro | 71.375 | 70.647 | -0.49% | 4 | +0.99% / -2.91% | 501.2 / 508.8 | 541.8 / 535.7 |
| 16x2 | Aeglos | 330.434 | 290.243 | -12.54% | 0 | -12.17% / -12.55% | 4330.6 / 4390.1 | 2630.8 / 2253.9 |

On 1x2, the median paired spread/compact latency ratio for Zopro divided by the corresponding Aeglos ratio changed by -3.20%. Zopro had the larger relative placement penalty in 0/8 blocks.

On 16x2, the median paired spread/compact latency ratio for Zopro divided by the corresponding Aeglos ratio changed by +13.64%. Zopro had the larger relative placement penalty in 8/8 blocks.

These full-proof measurements isolate placement at a fixed worker count on this host. The shared-cache capacity and communication distance both change between masks. Establishing which cost dominates and how it changes at 96 C7a cores requires a same-host comparison.

[Protocol](artifacts/protocol.md) · [CPU masks and L3 IDs](artifacts/masks.json) · [Measurements](artifacts/processes.json.gz) · [Proof verification](artifacts/verification.json) · [Earlier 48-core C7a placement experiment](../2026-11-18-ccd-placement280/README.md)
