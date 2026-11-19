# Full-proof latency sensitivity to B2 work on 1×2

Only a fixed public synthetic request can take the shortcut. The diagnostic compares the B2 coefficient suffix byte for byte with an independently captured public fixture, then reuses its mathematically exact contribution computed on the same loaded key. A different request fell back to ordinary G2 in every probe. The prefix, the other curve queries, the FFT, the solver, randomization and proof verification still execute. Seven randomized blocks measured 51 warmed SDK calls per process on a Ryzen 9950X using 32 logical workers. The table gives median process means.

| Mode | Full SDK ms | Paired vs disabled | CPU-ms/proof | Peak RSS MiB |
|---|---:|---:|---:|---:|
| integrated release | 7.008 | +0.54% | 142.4 | 62.5 |
| diagnostic disabled | 7.035 | +0.00% | 142.6 | 65.8 |
| 15,044 online rows | 5.937 | -15.82% | 117.2 | 66.2 |
| 7,522 online rows | 5.625 | -19.57% | 108.0 | 70.5 |
| 0 online rows | 4.742 | -31.78% | 83.6 | 62.5 |

With all B2 rows removed from the online MSM, this diagnostic measured 4.742 ms, 1.042 ms above the 3.700 ms goal derived from the supplied 37 ms M5 CPU time. The suffix check remains in the timed path. Changing the query size can also change its kernel choice, scheduling and interference with other branches. These timings apply only to repeated identical public scalar data; the production prover still computes the full B2 result on fresh requests. All 43 process exports passed independent Gnark verification.

[Protocol](artifacts/protocol.md) · [Samples](artifacts/processes.json.gz) · [Verification and fixture identity](artifacts/verification.json) · [Diagnostic source](artifacts/source.patch)
