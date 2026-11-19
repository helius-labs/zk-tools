# One glibc allocation threshold across full proofs

The same executables use five mmap thresholds with 32 glibc arenas and automatic trimming disabled throughout. The loader confirmed each setting. Five randomized blocks at 32 Ryzen workers compare end-to-end warm latency, maximum lifetime RSS and warm minor faults. Grouped 12×2 and 24×2 include separate public inputs. Thresholds per cell are 1, 4, 16, 64 and 1024 MiB, in that order. Changes use the matched 1024 MiB process in each block.

| Shape | Request | Warm ms | Peak MiB | Faults per proof | Paired change at 1 / 4 / 16 / 64 MiB |
|---|---|---:|---:|---:|---:|
| 1x2 | Original | 7.335 / 7.284 / 7.247 / 7.164 / 7.181 | 58 / 68 / 66 / 66 / 65 | 1512 / 68 / 78 / 73 / 61 | +2.97% / -0.55% / +0.14% / -0.23% |
| 12x2 | Original | 35.417 / 34.512 / 32.563 / 29.471 / 29.517 | 364 / 420 / 448 / 541 / 536 | 29196 / 24758 / 13769 / 2844 / 2510 | +20.47% / +16.71% / +9.24% / +0.07% |
| 12x2 | 142 | 53.891 / 50.847 / 46.847 / 46.046 / 46.198 | 406 / 456 / 625 / 990 / 1004 | 43667 / 29939 / 12231 / 9043 / 9854 | +16.66% / +10.56% / +1.00% / -0.36% |
| 24x2 | Original | 80.075 / 75.978 / 73.558 / 61.785 / 60.172 | 664 / 807 / 887 / 890 / 1127 | 82968 / 72885 / 64177 / 19343 / 5963 | +31.48% / +27.56% / +23.50% / +1.38% |
| 24x2 | 103 | 77.788 / 75.244 / 72.041 / 60.191 / 58.565 | 654 / 784 / 870 / 932 / 1150 | 83507 / 66050 / 63059 / 20503 / 5553 | +33.32% / +25.21% / +21.13% / +1.87% |
| merge-24x1 | Original | 81.495 / 80.682 / 70.702 / 66.709 / 65.320 | 619 / 714 / 943 / 949 / 1037 | 79434 / 70203 / 39780 / 24107 / 5867 | +25.50% / +23.52% / +9.55% / +2.47% |

All 150 process exports verified independently. [M5 CPU and Metal comparisons](artifacts/summary.json) · [Raw process samples](artifacts/samples.json.gz) · [Design](artifacts/design.json) · [Verification](artifacts/verification.json)
