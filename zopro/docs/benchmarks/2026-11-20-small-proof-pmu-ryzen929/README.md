# CPU work and wall time in small Ryzen proofs

Adding eight main workers cut unprofiled proof latency by **18–21%** across the six inputs. The gain was concentrated after the quotient. On the original 1×2 request, that interval fell from **6.968 to 5.117 ms** while its sampled CPU work rose from **110.0 to 124.4 CPU-ms per proof**. The solver interval stayed near **0.85 ms** with the same eight frontend CPUs, and grew slightly at the wider setting.

The run uses the byte-identical release executable from the recorded 1×2 and 2×2 profiles and the seeded 4×2 profile. Both layouts use 16 physical Ryzen 9950X cores. At 16+8, 16 main workers use both hardware threads of the eight cores in the first L3 group and the eight frontend workers use one thread per core in the other group. At 24+8, the additional main workers occupy the frontend cores' sibling hardware threads. Each cell has three paired blocks with three primes and 37 warm proofs per process.

## Wall time by phase

The reported width change uses paired plain runs. Phase durations come from the separate profiled runs. Values in each slash-separated cell are 16+8 followed by 24+8. The interval after quotient covers return of the quotient through entry to final exponentiation. It includes curve work, assembly and early verification.

| Shape | Input | Plain proof ms | Paired width change | Solver ms | Quotient ms | After quotient ms | Profile vs plain calibration |
|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | 8.771 / 6.960 | -20.79% | 0.845 / 0.858 | 0.557 / 0.561 | 6.968 / 5.117 | +1.20% / +1.00% |
| 1×2 | heldout | 8.582 / 6.808 | -20.03% | 0.834 / 0.860 | 0.551 / 0.563 | 6.849 / 5.024 | +2.17% / +2.24% |
| 2×2 | catalog | 11.700 / 9.374 | -20.11% | 1.071 / 1.167 | 1.122 / 1.316 | 9.362 / 6.423 | +3.27% / +2.80%* |
| 2×2 | heldout | 11.870 / 9.449 | -20.40% | 1.080 / 1.160 | 1.141 / 1.326 | 9.169 / 6.474 | +1.03% / +2.47% |
| 4×2 | catalog | 16.812 / 13.719 | -18.29% | 1.377 / 1.522 | 2.449 / 2.265 | 12.147 / 8.974 | +0.26% / -0.04% |
| 4×2 | heldout | 16.801 / 13.625 | -19.12% | 1.419 / 1.503 | 2.457 / 2.246 | 12.141 / 9.054 | +0.64% / +0.81% |

*The original 2×2 at 16+8 missed the 3% calibration threshold by 0.27 percentage points. Its profiled phases are descriptive. The other 11 cells cleared calibration.*

## Sampled CPU work

A FIFO enables sampling after the warmups. Linux uprobes bracket matching calls on the same thread. The analysis assigns software CPU-clock and hardware cycle samples by timestamp across every thread in the process. The CPU totals are estimates based on sample periods, and the symbol categories describe instruction attribution. Off-CPU waits and the latency of individual dependencies require separate measurement.

| Shape | Input | After quotient CPU-ms per proof | After quotient million user cycles per proof |
|---|---|---:|---:|
| 1×2 | catalog | 109.969 / 124.420 | 534.664 / 570.543 |
| 1×2 | heldout | 105.958 / 120.777 | 517.232 / 553.178 |
| 2×2 | catalog | 143.426 / 159.073 | 696.354 / 732.030 |
| 2×2 | heldout | 143.167 / 160.297 | 698.111 / 735.949 |
| 4×2 | catalog | 203.569 / 228.964 | 995.477 / 1062.301 |
| 4×2 | heldout | 204.589 / 229.522 | 996.625 / 1066.423 |

At 24+8, the named G1 and G2 streaming functions together account for about **43–54%** of sampled CPU time after the quotient. The separate Fq inverse symbol accounts for another **8.8–14.6 CPU-ms per proof**. These values include work inlined into the sampled functions.

| Shape | Input | G1 streaming CPU-ms | G2 streaming CPU-ms | Fq inverse CPU-ms |
|---|---|---:|---:|---:|
| 1×2 | catalog | 33.52 | 27.53 | 8.81 |
| 1×2 | heldout | 33.86 | 27.88 | 9.00 |
| 2×2 | catalog | 60.66 | 25.59 | 12.17 |
| 2×2 | heldout | 59.82 | 25.69 | 11.60 |
| 4×2 | catalog | 64.43 | 34.76 | 14.64 |
| 4×2 | heldout | 63.75 | 34.79 | 14.18 |

At 24+8, the general solver and Poseidon executor symbols dominate attributable solver work. Explicit scheduler symbols account for **0.7–5.6%** of interval CPU time. The solver's main worker CPU IDs contribute less than 0.1 sampled CPU-ms per proof in both layouts. The data supports targeting work within the solver and the proof stage after the quotient. Sleep and wait time can still affect latency.

| Shape | Input | Solver CPU-ms per proof at 24+8 | General solver | Poseidon executor | Scheduler symbols | Unresolved |
|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | 3.18 | 34.6% | 30.8% | 5.6% | 22.6% |
| 1×2 | heldout | 3.03 | 36.8% | 30.9% | 5.4% | 24.7% |
| 2×2 | catalog | 5.95 | 36.5% | 33.8% | 3.7% | 20.5% |
| 2×2 | heldout | 5.85 | 38.6% | 33.7% | 3.7% | 20.2% |
| 4×2 | catalog | 9.34 | 62.6% | 33.5% | 0.7% | 0.0% |
| 4×2 | heldout | 9.03 | 58.7% | 36.0% | 0.9% | 0.0% |

All 36 recorded processes supplied the expected proof, solver, folded quotient and final exponentiation boundaries with zero reported lost events. 4×2 enters a different core function, so the shared comparison uses the phase boundaries present on all three shapes. An initial 8+8 setup was rejected by the SDK because the frontend must be smaller than the main pool. The completed comparison keeps frontend width fixed and uses 16+8 and 24+8. Gnark independently verified all 77 exported proofs, including five setup exports.

AWS SSO remained expired. The C7a score remains **5/8 at 10× M5 CPU**, with CPU and Metal comparisons for all supplied shapes. [C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings)

[Protocol](artifacts/protocol.json) · [Probe sites](artifacts/sites.json) · [Per-process samples and recording hashes](artifacts/rows.json.gz) · [Aggregates](artifacts/summary.json) · [Proof verification](artifacts/verification.json)
