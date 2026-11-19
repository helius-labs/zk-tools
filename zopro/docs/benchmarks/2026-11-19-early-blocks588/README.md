# Applying public row offsets while collecting curve scalars on 12×2

All arms use the same 30-worker placement, two-stage B2 plan, A/K groups and external wire offsets. The new executable accepts both the flat and block dictionaries. With the early setting, two independent tasks read the solved witness to gather curve-query scalars and subtract public pairs in place from the solver's A/B rows. The normal Euler FFT starts after both complete. The ordinary arbitrary-parts API retains its independent quotient. The late block setting performs the same subtraction immediately before the FFT. A frozen grouped-and-centered executable supplies a separate comparison.

Six diagnostic exports check the number of early centering calls and nested B2 routes on the original and a scattered request. Seven shuffled timing blocks use the original and four other transfers, three warmups and 21 observed proofs per process. Each of the 146 exported proofs verified with the original key.

| Request | Frozen / flat / blocked / early ms | Blocked vs flat / early vs blocked / early vs frozen | Max early RSS MiB | Early vs M5 CPU / Metal | Blocks at 28.4 ms in arm order, of seven |
|---|---:|---:|---:|---:|---:|
| Original | 28.598 / 28.619 / 28.519 / 28.495 | -0.13% (4/7) / -0.08% (5/7) / -0.50% (4/7) | 622.5 | 9.97× / 3.54× | 0 / 1 / 2 / 3 |
| 57 | 28.782 / 29.041 / 28.547 / 28.736 | -1.30% (6/7) / +0.86% (2/7) / -0.20% (4/7) | 629.5 | — | — |
| 61 | 28.702 / 28.659 / 28.519 / 28.451 | -0.49% (5/7) / -0.13% (6/7) / -1.50% (6/7) | 621.0 | — | — |
| 139 | 45.110 / 45.140 / 44.865 / 44.999 | -0.88% (4/7) / -0.42% (4/7) / -0.49% (5/7) | 1117.3 | — | — |
| 143 | 45.216 / 45.255 / 45.262 / 44.815 | +0.64% (3/7) / -1.20% (5/7) / -0.68% (6/7) | 1149.0 | — | — |

[Source](artifacts/source.patch) · [Samples](artifacts/samples.json.gz) · [Key and SDK digests](artifacts/pins.json) · [Verification](artifacts/verification.json)
