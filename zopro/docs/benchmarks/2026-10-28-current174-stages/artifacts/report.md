# Stages in the adopted CPU executable

This diagnostic traces the exact adopted SDK without rebuilding the prover. All 820 SDK requests verify internally; 25 fresh exports independently pass Gnark. Every recording has one sampled PID, complete mandatory spans and no lost records. All 14 owned probes were removed. Source174 and SDK81033 remain unchanged. The result locates work; it is not a new optimization or accepted latency measurement.

Each of three fixed interleaved blocks contains tracing OFF and ON at the same root identity, affinity and executable inode. Each process primes three times and measures31 requests. ON records coarse uprobes plus199Hz user cycles inside acknowledged FIFO boundaries. Phase medians below are medians of the three process medians, retaining every request and block. Independently reported medians need not add exactly.

| Case | OFF SDK ms | ON SDK ms | Paired ON/OFF | Solver ms | Gather/public/reclaim ms | FFT product ms | Post-product ms | Direct small join ms | SDK outside inner core ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2-w32 | 8.151 | 8.572 | 1.051619 | 1.171 | 0.193 | 0.606 | 5.560 | 5.547 | 0.778 |
| 1x2-w16 | 9.612 | 10.259 | 1.058905 | 1.042 | 0.133 | 0.558 | 7.552 | 7.543 | 0.734 |
| 16x2-w32 | 40.981 | 41.756 | 1.025218 | 5.288 | 1.388 | 10.222 | 23.352 | unavailable | 1.351 |
| merge-54x1-w32 | 163.202 | 166.075 | 1.016425 | 20.461 | 4.471 | 65.120 | 72.857 | unavailable | 2.169 |

The inner core is the shortest matched solve_inner span containing the unique solver and product calls on its TID. Outer main-thread recursion remains in raw data. The post-product phase includes all remaining query work, joins, assembly and cleanup. Primary has a direct callsite join interval; other paths have no such marker and are reported unavailable. G2 callback intervals overlap other work and cannot be summed as exclusive CPU cost.

Instrumentation overhead varies by block. All three ratios and resource observations follow; no slow block is discarded. These values do not replace the accepted8.366931ms primary timing. The phase fractions cannot be subtracted mechanically to predict a speedup.

| Case | Block | OFF ms | ON ms | ON/OFF |
|---|---:|---:|---:|---:|
| 16x2-w32 | 0 | 39.759276 | 41.568951 | 1.045516 |
| 16x2-w32 | 1 | 40.980689 | 41.755682 | 1.018911 |
| 16x2-w32 | 2 | 40.989315 | 42.022965 | 1.025218 |
| 1x2-w16 | 0 | 9.405809 | 9.959855 | 1.058905 |
| 1x2-w16 | 1 | 10.109506 | 10.259439 | 1.014831 |
| 1x2-w16 | 2 | 9.612047 | 14.142937 | 1.471376 |
| 1x2-w32 | 0 | 8.048347 | 8.405831 | 1.044417 |
| 1x2-w32 | 1 | 8.151452 | 8.572225 | 1.051619 |
| 1x2-w32 | 2 | 8.189553 | 8.614545 | 1.051894 |
| merge-54x1-w32 | 0 | 160.403870 | 166.128904 | 1.035691 |
| merge-54x1-w32 | 1 | 163.391746 | 166.075448 | 1.016425 |
| merge-54x1-w32 | 2 | 163.202402 | 165.465113 | 1.013864 |

Sampled cycle period is assigned by each request's global wall-phase boundaries, across its worker threads. Phase shares locate CPU activity during that interval, including coordination and inlined work. They are not exclusive query CPU costs, removable wall time, occupancy, bandwidth, or a matched causal attribution. Adaptive cycle periods and start/stop-boundary samples are retained; cycles outside SDK spans remain explicit. Categories below are exclusive name matches, with other covering unknown symbols. Demangled top symbols and full raw events remain available.

| Case | Phase | Share of recorded user-cycle period | Leading categories (% of phase) |
|---|---|---:|---|
| 16x2-w32 | FFT_product | 27.477% | FFT_or_basis 82.88, scheduler_other 8.35, epoch 6.10, other 2.67 |
| 16x2-w32 | SDK_outside_innercore | 0.116% | other 60.51, epoch 39.36, field_inverse 0.07, scheduler_other 0.06 |
| 16x2-w32 | gather_public_reclaim | 2.465% | other 70.91, scheduler_other 23.24, epoch 5.85 |
| 16x2-w32 | outside_SDK_spans | 0.000% |  |
| 16x2-w32 | post_product | 60.570% | MSM_other 81.05, field_inverse 7.08, G1_stream 6.94, epoch 2.57 |
| 16x2-w32 | pre_solver | 0.000% | other 100.00 |
| 16x2-w32 | solver | 9.372% | solver 72.52, epoch 23.53, scheduler_other 1.69, field_inverse 1.15 |
| 1x2-w16 | FFT_product | 7.552% | FFT_or_basis 58.70, scheduler_other 22.52, epoch 18.51, other 0.27 |
| 1x2-w16 | SDK_outside_innercore | 0.536% | other 76.86, scheduler_other 22.63, field_inverse 0.35, epoch 0.15 |
| 1x2-w16 | gather_public_reclaim | 1.600% | scheduler_other 54.54, MSM_other 38.26, epoch 6.77, other 0.43 |
| 1x2-w16 | outside_SDK_spans | 0.000% |  |
| 1x2-w16 | post_product | 85.148% | MSM_other 24.13, borrowed_G2 24.01, G1_stream 23.95, field_inverse 17.00 |
| 1x2-w16 | pre_solver | 0.106% | epoch 99.99, scheduler_other 0.01, other 0.00 |
| 1x2-w16 | solver | 5.058% | solver 39.69, epoch 37.06, scheduler_other 19.94, other 3.27 |
| 1x2-w32 | FFT_product | 10.337% | epoch 40.12, FFT_or_basis 39.40, scheduler_other 19.78, other 0.70 |
| 1x2-w32 | SDK_outside_innercore | 0.799% | epoch 66.26, other 33.60, scheduler_other 0.14 |
| 1x2-w32 | gather_public_reclaim | 1.643% | epoch 62.60, scheduler_other 35.86, MSM_other 0.79, other 0.75 |
| 1x2-w32 | outside_SDK_spans | 0.000% |  |
| 1x2-w32 | post_product | 84.228% | borrowed_G2 26.30, MSM_other 24.63, G1_stream 23.57, other 11.01 |
| 1x2-w32 | pre_solver | 0.002% | epoch 75.08, other 24.37, scheduler_other 0.55 |
| 1x2-w32 | solver | 2.992% | epoch 37.67, solver 36.65, other 13.52, scheduler_other 12.08 |
| merge-54x1-w32 | FFT_product | 44.902% | FFT_or_basis 93.70, scheduler_other 3.02, epoch 1.68, other 1.60 |
| merge-54x1-w32 | SDK_outside_innercore | 0.050% | other 84.24, epoch 15.70, field_inverse 0.04, scheduler_other 0.03 |
| merge-54x1-w32 | gather_public_reclaim | 2.752% | other 82.54, scheduler_other 15.98, epoch 1.48 |
| merge-54x1-w32 | outside_SDK_spans | 0.000% |  |
| merge-54x1-w32 | post_product | 45.661% | MSM_other 91.47, field_inverse 5.22, other 2.04, epoch 1.12 |
| merge-54x1-w32 | solver | 6.634% | solver 85.18, epoch 11.57, other 1.88, scheduler_other 1.00 |

The original all-eight10× CPU objective remains active and unmet0/8. These inherited accepted-source results use supplied historical M5 references; this diagnostic does not remeasure all eight or compare AWS/Aeglos. No new mobile/ARM/TEE, giant, swap, minimum-RAM or pressure qualification. The adopted source's recorded performance exceptions remain.

| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| 12x2 | 34.858408 | 284.0 | 8.147× | 101.0 | 2.897× |
| 16x2 | 41.377043 | 335.0 | 8.096× | 124.0 | 2.997× |
| 1x2 | 8.366931 | 37.0 | 4.422× | 24.0 | 2.868× |
| 24x2 | 67.764001 | 528.0 | 7.792× | 176.0 | 2.597× |
| 2x2 | 11.539335 | 59.0 | 5.113× | 30.0 | 2.600× |
| 4x2 | 16.540233 | 109.0 | 6.590× | 48.0 | 2.902× |
| 8x2 | 25.136722 | 189.0 | 7.519× | 74.0 | 2.944× |
| merge-24x1 | 67.096516 | 507.0 | 7.556× | 175.0 | 2.608× |

[Protocol](artifacts/design.md), [phase summaries and every block](artifacts/summary.json), [complete analysis](artifacts/analysis.json), [probe definitions](artifacts/probes.json), [disassembly](artifacts/canonical-disassembly.txt), [cleanup](artifacts/cleanup.json). Tool semantics follow the [Linux uprobe documentation](https://docs.kernel.org/trace/uprobetracer.html) and [perf record manual](https://man7.org/linux/man-pages/man1/perf-record.1.html). Setup/decoder failures and repairs precede the remaining captures and are preserved; the successful preflight and its independent verification were reused. The final analysis verifies25fresh164-byte exports,50proof/public members, and ledger15028→15053. No production change or new speed claim.
