# Hybrid signed and unsigned dense H windows

Both candidates keep the 19 fields of the mixed radix layout. Wide-only recodes the 14-bit fields as signed digits; all-lower signs every field except the final one. The positive final field absorbs the carry. Every enabled signed window halves its bucket count. Both modes retain the conservative scratch reservation.

| Shape | Recoding | Median ms | Paired vs unsigned | Wins / 5 | CPU ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| merge-24x1 | reference | 68.705 | -1.83% | 3 | 1010 | 910 |
| merge-24x1 | off | 69.639 | +0.00% | — | 1009 | 871 |
| merge-24x1 | wide-only | 71.256 | +0.17% | 2 | 1010 | 862 |
| merge-24x1 | all-lower | 69.675 | +0.53% | 2 | 1004 | 892 |
| 12x2 | reference | 37.553 | -1.18% | 4 | 551 | 504 |
| 12x2 | off | 37.731 | +0.00% | — | 550 | 516 |
| 12x2 | wide-only | 37.955 | -0.12% | 3 | 555 | 519 |
| 12x2 | all-lower | 37.428 | -0.98% | 3 | 547 | 492 |
| 16x2 | reference | 44.817 | +0.79% | 2 | 652 | 588 |
| 16x2 | off | 44.467 | +0.00% | — | 655 | 578 |
| 16x2 | wide-only | 44.787 | +0.72% | 2 | 658 | 594 |
| 16x2 | all-lower | 45.384 | +2.68% | 1 | 660 | 600 |
| 24x2 | reference | 71.565 | +0.01% | 2 | 1053 | 969 |
| 24x2 | off | 71.659 | +0.00% | — | 1055 | 929 |
| 24x2 | wide-only | 72.666 | +1.54% | 2 | 1067 | 1028 |
| 24x2 | all-lower | 71.129 | -0.68% | 3 | 1057 | 939 |

Each of the 80 fresh processes used 16 physical Ryzen cores and performed three primes and eleven timed proofs. All exports passed independent verification. The arithmetic test reconstructed every 12-bit integer under all four lower-field signing patterns and compared both full BN254 layouts against a generator-sum oracle. The patch applies to experiment 315.
