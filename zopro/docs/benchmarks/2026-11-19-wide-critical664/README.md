# Which curve operation finishes last after grouped G2 decomposition

The integrated executable processes the same two 5×16 inputs with packed FFT and with grouped G2 either disabled or enabled. All runs used 32 Ryzen hardware threads. Two byte-identical copies provided plain and probed controls. Six shuffled blocks each warmed three requests, timed thirteen and independently verified one exported proof. Every traced request entered exactly two grouped G1 operations and one grouped or owned G2 operation. G1 time is the union of the two G1 intervals. For each timing and interval the table uses the median of six process means. The G1 minus G2 finish column uses signed time, so positive values put G1 later. The final column measures query-driver time after both observed curve operations returned. The group_affine column includes all invocations of the shared reduction routine during the proof, including nested reductions. Its union counts overlapping time once. Other operations in the driver can overlap the observed intervals. One diagnostic capture was repeated after the interval reader encountered a nested call to the same function on one worker. The reader matches entries and returns with a per-thread stack.

| Input | G2 | Plain / traced ms | Trace effect | G1 / G2 span ms | G1 / G2 group_affine union ms | G1 and G2 overlap ms | G1 minus G2 finish ms | G1 later | Driver tail ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 5x16 | off | 22.966 / 23.776 | +3.87% | 9.582 / 8.869 | 4.982 / 3.188 | 7.796 | +0.300 | 24/78 | 0.025 |
| 5x16 | on | 22.777 / 23.449 | +2.77% | 8.817 / 8.801 | 4.645 / 3.100 | 7.187 | -0.419 | 31/78 | 0.020 |
| 5x16-1702 | off | 23.423 / 23.933 | +2.30% | 9.479 / 8.717 | 4.819 / 3.404 | 7.223 | +0.791 | 40/78 | 0.037 |
| 5x16-1702 | on | 22.787 / 23.318 | +2.48% | 8.832 / 8.722 | 4.699 / 3.009 | 7.258 | -0.035 | 34/78 | 0.021 |

On the integrated 5×16 route, G1 finished later on 31/78 and 34/78 proofs. G2 finished later on the others. Their measured intervals overlapped for 7.19 and 7.26 ms, and the query driver returned about 0.02 ms after the last of the two. Accelerating only the last function seen in one profile cannot address this pattern. The next arithmetic experiment should reduce operations paid by both curves and measure the effect on both return times.

[Release result](../2026-11-19-owned-phi661/README.md) · [Resolved symbols](artifacts/symbols.json) · [Per-proof intervals](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
