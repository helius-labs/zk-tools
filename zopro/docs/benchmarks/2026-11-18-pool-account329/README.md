# Thread CPU time by prover pool on Ryzen

Linux per-thread scheduler runtime was read while the existing SDK driver waited for its start and stop acknowledgements. The executable and proving libraries were identical across the observed and plain arms. The main pool was identified from its reported worker TIDs, the caller from the process ID and the frontend workers from the remaining tasks and their validated affinity. CPU time includes work and scheduler overhead incurred on each thread. It does not identify useful arithmetic or idle spinning. Three randomized blocks used 31 measured proofs per process.

| Shape | Main workers | Frontend workers | G2 geometry | Plain ms | Observed/plain | Main CPU ms/proof | Frontend CPU ms/proof | Caller CPU ms/proof |
|---|---:|---:|---|---:|---:|---:|---:|---:|
| 1x2 | 16 | 0 | off | 8.473 | +0.42% | 104.7 | 0.0 | 0.4 |
| 1x2 | 16 | 8 | off | 8.954 | -0.09% | 92.4 | 8.9 | 0.4 |
| 1x2 | 32 | 0 | off | 7.373 | +1.65% | 166.1 | 0.0 | 0.4 |
| 1x2 | 32 | 8 | off | 7.748 | -1.49% | 142.0 | 9.5 | 0.4 |
| 1x2 | 32 | 8 | split | 7.894 | -1.03% | 148.6 | 9.4 | 0.4 |
| 2x2 | 16 | 0 | off | 11.990 | -0.50% | 149.7 | 0.0 | 0.7 |
| 2x2 | 16 | 8 | off | 12.124 | +0.61% | 119.5 | 17.4 | 0.5 |
| 2x2 | 32 | 0 | off | 10.516 | -0.38% | 237.8 | 0.0 | 0.7 |
| 2x2 | 32 | 8 | off | 10.842 | +1.46% | 186.6 | 17.8 | 0.6 |
| 2x2 | 32 | 8 | split | 10.922 | +0.28% | 193.3 | 18.0 | 0.6 |

The summed per-thread CPU estimate agreed with process user and system accounting to within the larger of 4 ms/proof or 5% for every observed process. The 32-worker local settings include SMT. These results describe the Ryzen host.

[Per-process measurements](artifacts/observations.json.gz) · [Observer source](artifacts/observer.rs) · [Verification](artifacts/verification.json)
