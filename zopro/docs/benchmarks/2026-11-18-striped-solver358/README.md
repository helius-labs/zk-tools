# Grouping ready solver tasks on Ryzen

This experiment assigns sorted ready tasks round-robin to groups. Each group runs serially on a Rayon job. The number of groups is half, equal to, or twice the size of the solving pool. Narrow waves retain the original execution order. The proof format and prepared key are identical in all comparisons.

The table reports medians of five fresh-process means. Paired changes use block-matched processes. Small shapes have 32 logical workers and eight frontend workers. Large shapes have 16 physical workers. All timed runs use eight solver epochs.

| Shape | Group count | Integrated ms | Disabled ms | Grouped ms | Change vs disabled | Faster blocks | Changes vs integrated | Observed marked processes | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | half | 7.751 | 7.693 | 7.533 | -0.60% | 3/5 | -2.93% | 5/5 | 71 |
| 1x2 | one | 7.751 | 7.693 | 7.449 | -2.59% | 5/5 | -3.19% | 5/5 | 64 |
| 1x2 | two | 7.751 | 7.693 | 7.447 | -3.43% | 5/5 | -3.76% | 5/5 | 64 |
| 2x2 | half | 10.836 | 10.696 | 10.944 | +1.72% | 2/5 | -1.40% | 5/5 | 110 |
| 2x2 | one | 10.836 | 10.696 | 10.383 | -2.89% | 4/5 | -4.18% | 5/5 | 122 |
| 2x2 | two | 10.836 | 10.696 | 10.413 | -2.23% | 5/5 | -4.10% | 5/5 | 113 |
| 16x2 | half | 45.693 | 45.513 | 47.353 | +3.82% | 0/5 | +3.51% | 5/5 | 585 |
| 16x2 | one | 45.693 | 45.513 | 46.204 | +1.52% | 1/5 | +1.15% | 5/5 | 594 |
| 16x2 | two | 45.693 | 45.513 | 47.522 | +2.95% | 0/5 | +3.22% | 5/5 | 618 |
| merge-24x1 | half | 72.725 | 72.145 | 73.984 | +2.59% | 0/5 | +1.25% | 5/5 | 890 |
| merge-24x1 | one | 72.725 | 72.145 | 73.520 | +1.91% | 1/5 | +1.84% | 5/5 | 895 |
| merge-24x1 | two | 72.725 | 72.145 | 72.295 | +0.53% | 1/5 | -0.59% | 5/5 | 910 |

## Parallel work in the prepared schedule

All 144 native tests passed, including the check that grouped execution visits each ready task exactly once and propagates errors. Each entry counts operations in the stored dependency schedule. The model assigns the same cost to every operation and sums the longest serial chain across epochs. Different field operations and Poseidon specialization have different costs in the interpreter. The ratios characterize the schedule under this unit-cost assumption.

| Shape | Work / span, four epochs | Eight | Sixteen | Leaf tasks, eight |
|---|---:|---:|---:|---:|
| 12x2 | 14.00 | 22.11 | 23.12 | 630 |
| 16x2 | 15.82 | 28.45 | 29.43 | 829 |
| 1x2 | 2.70 | 2.66 | 2.58 | 118 |
| 24x2 | 19.48 | 37.99 | 41.15 | 1235 |
| 2x2 | 4.18 | 4.45 | 4.19 | 165 |
| 4x2 | 6.87 | 8.15 | 8.23 | 242 |
| 8x2 | 11.02 | 14.42 | 15.69 | 435 |
| merge-24x1 | 11.46 | 15.11 | 15.50 | 1378 |

[Source](artifacts/source.patch.gz) · [Timing samples](artifacts/local.json.gz) · [Epoch counts](artifacts/census.json.gz) · [Verification](artifacts/verification.json)

[Same-host C7a follow-up](aws.md)

[Matched small-circuit scaling and Aeglos timings](small.md)
