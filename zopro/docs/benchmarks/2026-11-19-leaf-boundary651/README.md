# Packed Euler FFT leaf boundaries in complete proofs

Each process times fifteen warm proofs. Latency is the median of process means and peak RSS is the highest observed process peak. Paired percentages use within-block changes. Parentheses give faster blocks or, in the last column, blocks meeting the supplied M5 CPU threshold. All 162 native core tests passed with the halves mode. Both Euler transform equivalence tests also passed in fused mode. All 148 independent proof exports verified.

On the original 12×2, fused reached 28.519 ms and beat the same-binary ordinary path in five of twelve pairs. On 24×2, processing independent halves took 56.148 ms, winning six of eight pairs against disabled and five of eight against frozen. Peak RSS reached 908.5 MiB versus 935.0 and 941.9 MiB, respectively. The 24×2 result is being checked on three further witnesses. [Confirmation protocol](../2026-11-19-large-leaf654/protocol.md)

| Input | Leaf | Latency ms | Paired vs disabled | Paired vs frozen | Peak MiB | M5 CPU / Metal and 10× CPU blocks |
|---|---|---:|---:|---:|---:|---:|
| 12x2-0 | frozen | 28.713 | +0.37% (5/12) | +0.00% (0/12) | 567.0 | 9.89× / 3.52× (1/12) |
| 12x2-0 | off | 28.583 | +0.00% (0/12) | -0.37% (7/12) | 561.4 | 9.94× / 3.53× (4/12) |
| 12x2-0 | fused | 28.519 | +0.28% (5/12) | -0.41% (8/12) | 563.5 | 9.96× / 3.54× (1/12) |
| 12x2-0 | halves | 28.629 | +0.05% (6/12) | -0.08% (7/12) | 567.0 | 9.92× / 3.53× (3/12) |
| 12x2-58 | frozen | 28.643 | -0.15% (3/4) | +0.00% (0/4) | 558.0 | — |
| 12x2-58 | off | 28.685 | +0.00% (0/4) | +0.15% (1/4) | 547.2 | — |
| 12x2-58 | fused | 28.671 | -0.09% (2/4) | -0.43% (3/4) | 559.0 | — |
| 12x2-58 | halves | 28.734 | +0.75% (2/4) | +0.18% (2/4) | 563.3 | — |
| 12x2-61 | frozen | 28.818 | +0.51% (0/4) | +0.00% (0/4) | 551.5 | — |
| 12x2-61 | off | 28.672 | +0.00% (0/4) | -0.51% (4/4) | 540.8 | — |
| 12x2-61 | fused | 28.588 | -0.04% (2/4) | -0.60% (3/4) | 535.3 | — |
| 12x2-61 | halves | 28.922 | +1.27% (1/4) | +0.55% (2/4) | 565.3 | — |
| 12x2-139 | frozen | 43.198 | +0.31% (1/4) | +0.00% (0/4) | 746.5 | — |
| 12x2-139 | off | 43.063 | +0.00% (0/4) | -0.31% (3/4) | 736.3 | — |
| 12x2-139 | fused | 43.517 | +1.37% (1/4) | +0.83% (0/4) | 777.5 | — |
| 12x2-139 | halves | 43.290 | +0.75% (2/4) | +0.20% (1/4) | 756.5 | — |
| 24x2-0 | frozen | 56.764 | -0.60% (5/8) | +0.00% (0/8) | 941.9 | 9.30× / 3.10× (0/8) |
| 24x2-0 | off | 56.781 | +0.00% (0/8) | +0.60% (3/8) | 935.0 | 9.30× / 3.10× (0/8) |
| 24x2-0 | fused | 56.739 | -0.83% (5/8) | +0.21% (3/8) | 915.7 | 9.31× / 3.10× (0/8) |
| 24x2-0 | halves | 56.148 | -1.67% (6/8) | -0.85% (5/8) | 908.5 | 9.40× / 3.13× (0/8) |

[Protocol](protocol.md) · [Implementation](artifacts/source.patch) · [Process samples and CPU profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)

## Warm CPU work and dispatch

CPU time sums user and system time across workers. CPU milliseconds and minor faults are median process means per timed proof. The final column is the share of sampled cycles in the new inner leaf function in the separate profile.

| Input | Leaf | CPU ms per proof | Minor faults per proof | Inner leaf sampled cycles |
|---|---|---:|---:|---:|
| 12x2-0 | frozen | 744.4 | 2409 | 0.00% |
| 12x2-0 | off | 742.7 | 2306 | 0.00% |
| 12x2-0 | fused | 742.6 | 2220 | 3.23% |
| 12x2-0 | halves | 743.1 | 2325 | 4.06% |
| 24x2-0 | frozen | 1587.8 | 2381 | 0.00% |
| 24x2-0 | off | 1589.0 | 2551 | 0.00% |
| 24x2-0 | fused | 1584.0 | 2143 | 3.58% |
| 24x2-0 | halves | 1577.9 | 2397 | 3.68% |
