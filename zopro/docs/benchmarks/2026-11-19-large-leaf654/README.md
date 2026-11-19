# The Euler half-leaf on four 24×2 transfers

Each process times fifteen warm proofs. Latency is the median of process means and peak RSS is the highest observed process peak. Paired percentages use within-block changes. Parentheses give faster blocks or, in the last column, blocks meeting the supplied M5 CPU threshold. All 162 native core tests passed with the halves mode. Both Euler transform equivalence tests also passed in fused mode. All 102 independent proof exports verified.

| Input | Leaf | Latency ms | Paired vs disabled | Paired vs frozen | Peak MiB | M5 CPU / Metal and 10× CPU blocks |
|---|---|---:|---:|---:|---:|---:|
| 24x2-0 | frozen | 56.879 | +0.58% (6/12) | +0.00% (0/12) | 944.1 | 9.28× / 3.09× (0/12) |
| 24x2-0 | off | 56.553 | +0.00% (0/12) | -0.57% (6/12) | 951.8 | 9.34× / 3.11× (0/12) |
| 24x2-0 | halves | 56.324 | -0.45% (7/12) | -0.65% (7/12) | 943.6 | 9.37× / 3.12× (0/12) |
| 24x2-97 | frozen | 54.930 | -0.54% (3/6) | +0.00% (0/6) | 938.1 | — |
| 24x2-97 | off | 55.388 | +0.00% (0/6) | +0.55% (3/6) | 906.1 | — |
| 24x2-97 | halves | 55.187 | -0.17% (3/6) | +0.51% (2/6) | 909.3 | — |
| 24x2-99 | frozen | 55.372 | -0.98% (4/6) | +0.00% (0/6) | 936.9 | — |
| 24x2-99 | off | 56.057 | +0.00% (0/6) | +0.99% (2/6) | 946.2 | — |
| 24x2-99 | halves | 55.512 | -0.60% (5/6) | -0.14% (3/6) | 947.6 | — |
| 24x2-101 | frozen | 56.949 | +2.33% (1/6) | +0.00% (0/6) | 965.5 | — |
| 24x2-101 | off | 55.562 | +0.00% (0/6) | -2.27% (5/6) | 931.0 | — |
| 24x2-101 | halves | 56.140 | +0.50% (3/6) | -2.40% (3/6) | 909.5 | — |

The original showed a second small gain against both controls, with seven wins in twelve pairs against each. On the other three witnesses the half-leaf won 8/18 pairs against frozen and 11/18 against disabled. Peak RSS varied by witness, and the original missed the 52.8 ms target in every block. These results support retaining the existing production leaf.

[Protocol](protocol.md) · [Implementation](../2026-11-19-leaf-boundary651/artifacts/source.patch) · [Process samples and CPU profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
