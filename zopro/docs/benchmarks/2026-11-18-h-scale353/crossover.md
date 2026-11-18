# H-window splitting across core counts

These exploratory cells were selected after viewing preliminary primary results. All configurations used fresh randomized processes after the primary panel and profile. The split candidate uses the earlier experimental executable. The integrated release uses the single-range H19 layout. Each of five process means covers eleven SDK requests.

| Shape | Cores | Integrated ms | Two-range candidate ms | Paired change | Candidate faster | M5 CPU / Metal speedup on candidate | Blocks at 10× CPU |
|---|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 48 | 37.355 | 35.680 | -4.84% | 5/5 | 9.39× / 3.48× | 0/5 |
| 16x2 | 56 | 36.773 | 35.414 | -3.67% | 5/5 | 9.46× / 3.50× | 0/5 |
| 16x2 | 64 | 37.203 | 36.204 | -1.32% | 4/5 | 9.25× / 3.43× | 0/5 |
| 16x2 | 72 | 37.716 | 36.054 | -4.51% | 5/5 | 9.29× / 3.44× | 0/5 |
| 16x2 | 80 | 38.431 | 36.992 | -2.70% | 5/5 | 9.06× / 3.35× | 0/5 |
| 16x2 | 96 | 39.872 | 38.738 | -2.25% | 5/5 | 8.65× / 3.20× | 0/5 |
| merge-24x1 | 48 | 53.893 | 51.270 | -4.95% | 5/5 | 9.89× / 3.41× | 0/5 |
| merge-24x1 | 56 | 54.226 | 50.724 | -6.94% | 5/5 | 10.00× / 3.45× | 2/5 |
| merge-24x1 | 64 | 54.397 | 52.374 | -3.96% | 5/5 | 9.68× / 3.34× | 1/5 |
| merge-24x1 | 72 | 54.690 | 51.284 | -7.39% | 5/5 | 9.89× / 3.41× | 1/5 |
| merge-24x1 | 80 | 57.176 | 53.632 | -6.95% | 5/5 | 9.45× / 3.26× | 0/5 |
| merge-24x1 | 96 | 59.471 | 56.105 | -5.68% | 5/5 | 9.04× / 3.12× | 0/5 |

The lowest observed merge24 median was 50.724 ms at 56 cores. This was 0.024 ms above the 50.7 ms target. Two of five process means cleared it.

[Protocol](artifacts/crossover-protocol.md) · [Primary panel](README.md)
