# Completion order of the four curve queries on Ryzen

A private build of the accepted source records a monotonic timestamp immediately after each parallel query returns and a fifth timestamp when the enclosing join completes. The B branch includes scalar grouping and, on these runs, computes only B2. Each process used the public original request, 32 logical workers, three warmups and eleven measured proofs. Three randomized pairs compare the plain release with the instrumented binary. The final column measures how long the last branch continued after the other three had returned. Instrumentation and recompilation can affect worker scheduling.

| Shape | Plain / traced proof ms | Paired difference | Last to return A / K / H / B2 (of 33) | Median gap to previous return, ms |
|---|---:|---:|---:|---:|
| 16x2 | 39.347 / 39.989 | +0.89% | 3 / 3 / 22 / 5 | 0.095 |
| merge-24x1 | 63.478 / 64.373 | +1.71% | 6 / 2 / 18 / 7 | 0.120 |

All 12 process exports verified independently. The timestamps establish branch completion on this Ryzen. The C7a 64- and 96-core cases require their own measurements.

[Per-proof timestamps](artifacts/processes.json.gz) · [Diagnostic source](artifacts/source.patch) · [Verification](artifacts/verification.json)
