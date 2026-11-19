# Warm latency and curve overlap on every M5 reference shape

Four paired blocks per shape use the selected experimental profiles from panel 653. One executable copy runs plain, and a byte-identical separate copy records Linux function entry and return events. Each process primes three requests and times nine. Pinning, distinct proofs, expected public inputs and independent Gnark verification are checked. The table uses medians of process means, with paired trace effects. M5 speedups and target gaps use only the plain measurements. All 72 exports verified.

Curve intervals are unions of the observed grouped, canonical, raw and in-place evaluators of each curve, including the experimental group-plan functions when present. Overlap is the intersection of those unions. The probes do not cover every fused or inlined kernel. A zero therefore means no covered interval was observed. Solver and FFT columns time the named plan and quotient-product wrappers. Timings describe concurrent work and do not add to a proof's elapsed time. Each traced process has a matched uninstrumented control.

| Shape | Plain ms | M5 CPU / Metal | Gap to 10× CPU ms | Trace effect | Solver / FFT ms | Observed G1 / G2 ms | G1 and G2 overlap ms | Peak MiB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.134 | 5.19× / 3.36× | +3.434 | +6.92% | 0.85 / 0.60 | 4.31 / 5.04 | 4.31 | 59.5 |
| 2x2 | 9.613 | 6.14× / 3.12× | +3.713 | +4.91% | 1.16 / 1.32 | 5.21 / 6.36 | 5.17 | 111.0 |
| 4x2 | 14.963 | 7.28× / 3.21× | +4.063 | +4.05% | 2.04 / 2.94 | 7.74 / 9.05 | 7.62 | 269.2 |
| 8x2 | 23.112 | 8.18× / 3.20× | +4.212 | +5.86% | 3.20 / 4.82 | 13.32 / 14.17 | 13.13 | 492.5 |
| 12x2 | 28.881 | 9.83× / 3.50× | +0.481 | +1.71% | 2.69 / 0.00 | 14.84 / 14.13 | 14.12 | 529.8 |
| 16x2 | 36.908 | 9.08× / 3.36× | +3.408 | +5.20% | 3.54 / 9.77 | 21.94 / 21.11 | 20.43 | 729.7 |
| 24x2 | 56.634 | 9.32× / 3.11× | +3.834 | +4.49% | 6.57 / 0.00 | 29.24 / 25.94 | 25.93 | 931.0 |
| merge-24x1 | 61.889 | 8.19× / 2.83× | +11.189 | +2.26% | 7.92 / 21.57 | 27.86 / 11.99 | 10.47 | 985.2 |
| merge-54x1 | 150.060 | — | — | +2.90% | 19.89 / 50.02 | 74.85 / 35.37 | 35.36 | 1708.0 |

On 1×2 the observed G1 work overlapped G2 for 4.31 ms of its 5.04 ms span. This leaves roughly 0.73 ms of observed G2 time outside G1, versus a 3.434 ms plain-time gap to the M5 CPU target. Accelerating G2 in isolation would also have to accelerate or reschedule concurrent work to close that gap. At 16×2 the two curves overlapped for 20.43 ms. Merge54 showed a different balance, with 74.85 ms of covered G1, 35.37 ms of G2 and 50.02 ms in the FFT wrapper. These measurements support optimizing both curves on the smaller transfers and including G1 and FFT in merge scaling experiments.

None of the eight median warm latencies reached the 10× CPU target. Tracing increased median paired latency by 5–7% on three shapes. The timing gaps use uninstrumented processes. The selected implementations on 12×2 and 24×2 did not enter the named FFT wrapper. All temporary Linux probes were removed.

[Exact profile selection](../2026-11-19-reference-panel653/README.md) · [Binary identities and sites](artifacts/symbols.json) · [Cases](artifacts/config.json.gz) · [Per-proof intervals](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
