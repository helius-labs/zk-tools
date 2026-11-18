# Worker count before MSM on C7a

Limiting the witness solve, scalar preparation and FFT to 32 workers of a 96-core c7a.24xlarge reduced mean 1×2 verified latency from **12.68 to 11.26 ms** in the first block, an 11.2% improvement against the same executable with the restriction disabled. Summed CPU time fell from **688 to 499 ms per request**. Peak process RSS was **119.5 MiB** with the cap on, **114.2 MiB** off. Exact accepted Zopro took 12.61 ms at 96 cores and 10.91 ms at 32 cores.

**The broad policy failed on merge54.** Its 96-core ON process aborted with a thread stack overflow in the 17-round latency panel after passing the one-round preflight. Both full-length controls completed. The workflow stopped at that failure. Two planned blocks and the profiling panel were never reached. The precise failing call stack is unavailable. This partial run supports further testing of the small canonical solver separately from the general proof path.

## First block at 96 physical cores

Each cell is the arithmetic mean of one process's timed verified requests after three primes. REF is the accepted build. OFF and ON share executable bytes. M5 ratios use the supplied reference divided by ON latency.

| Shape | REF ms | OFF ms | ON ms | ON vs OFF | M5 CPU / ON | M5 Metal / ON |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 12.61 | 12.68 | 11.26 | -11.2% | 3.29× | 2.13× |
| 2x2 | 16.64 | 16.82 | 15.54 | -7.6% | 3.80× | 1.93× |
| 4x2 | 21.08 | 20.98 | 20.20 | -3.7% | 5.40× | 2.38× |
| 8x2 | 28.54 | 28.55 | 25.25 | -11.6% | 7.49× | 2.93× |
| 12x2 | 35.10 | 35.14 | 35.85 | +2.0% | 7.92× | 2.82× |
| 16x2 | 41.20 | 42.35 | 46.09 | +8.8% | 7.27× | 2.69× |
| 24x2 | 57.03 | 59.01 | 68.74 | +16.5% | 7.68× | 2.56× |
| merge-24x1 | 61.81 | 60.71 | 70.76 | +16.6% | 7.16× | 2.47× |
| merge-54x1 | 127.36 | 126.89 | Aborted | — | — | — |

The cap also shortened 2×2, 4×2 and 8×2 in this block. Larger completed shapes slowed, including 24×2 and merge24. None of the eight observed 96-core ON means reached the supplied 10× M5 CPU target. The separate matched-engine measurement gave 7.73 ms for Aeglos on 1×2 at 96 cores. This run did not remeasure Aeglos. [Matched comparison](../2026-11-18-c7a-phase251/README.md)

The first local diagnostic engaged only on 1×2. A revision added external-caller admission and extended the scope into the general solver, with an explicit release after FFT and before MSM. The canonical path returns from the scope before starting MSM. Three threading tests passed. All six original and six revised local exports passed independent verification. The revised cap engaged on 1×2, 2×2 and merge54 at 8 of 32 local workers.

All 54 remote preflights passed the active-route and server Gnark checks. They and 53 completed latency exports passed host Gnark verification after hash-checked retrieval. These 107 processes reported 1,963 timed requests. The aborted process produced no export. The unique inventory reached 64,564. The instance terminated, and its tagged storage and transfer bucket were removed. [All completed process results](artifacts/processes.json.gz) · [Prototype patch](artifacts/prototype.patch.gz) · [Verification reader](artifacts/partial.rs) · [Failure output](artifacts/merge54-abort.log)
