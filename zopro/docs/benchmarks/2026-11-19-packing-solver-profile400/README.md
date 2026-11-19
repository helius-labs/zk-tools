# Solver wall time and scheduler CPU samples on Ryzen

The observer brackets the general solver call during full 16×2 proofs and samples user CPU time every 2.004008 ms across all running threads. Three fresh processes per cell timed 21 requests each. The plain SDK timings come from five earlier processes per cell. The solver wall column uses the median of each process’s mean solver interval. CPU samples and Crossbeam samples are pooled over all observed intervals.

| Workers | Packing | Plain SDK ms | Traced SDK ms | Solver wall ms | Solver CPU samples | In Crossbeam | Estimated Crossbeam CPU-ms per proof |
|---:|---|---:|---:|---:|---:|---:|---:|
| 8 | off | 69.768 | 69.367 | 4.406 | 912 | 0.8% | 0.22 |
| 8 | on | 68.769 | 68.708 | 3.897 | 872 | 0.5% | 0.13 |
| 32 | off | 40.233 | 40.588 | 3.549 | 2844 | 17.6% | 15.94 |
| 32 | on | 40.371 | 41.889 | 3.684 | 2851 | 17.1% | 15.52 |

Packing shortened the observed solver interval by 0.509 ms at eight workers. At 32 workers it lengthened by 0.135 ms. The estimated Crossbeam CPU time at that width fell by 0.42 CPU-ms per proof while wall time rose. A lower scheduler sample count alone would give the wrong performance conclusion here.

Eight workers used eight physical cores. Thirty-two workers used both hardware threads on sixteen cores. Each solver entry had one return on the same Linux thread, for 252 measured intervals. The probe sites used the executable file offset resolved from its symbol table and load segment. An initial symbol lookup and a trace registration attempt ended before collecting any requests. The successful recording attached both probes and removed them after measurement.

[Plain measurements](../2026-11-19-packing-width398/README.md) · [Protocol](artifacts/protocol.md) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
