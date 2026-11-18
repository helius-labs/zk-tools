# More preparation workers than curve workers

The experiment used the same 16 physical Ryzen cores for every configuration. The wider pool uses 16 workers for selected preparation stages, while the main pool has 12 workers. The full-width control uses 16 throughout. Other controls keep the main pool at 12, with an idle attached pool or without it. Five randomized blocks each included eleven timed SDK proofs per fresh process.

| Shape | Setting | Median ms | Paired change vs uniform 16 | Faster blocks | CPU ms | Peak MiB | M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|---:|
| 4x2 | reference | 16.950 | -0.71% | 3/5 | 236 | 214 | 6.43× / 2.83× |
| 4x2 | full | 17.053 | +0.00% | 0/5 | 238 | 216 | 6.39× / 2.81× |
| 4x2 | narrow | 19.548 | +14.63% | 0/5 | 208 | 190 | 5.58× / 2.46× |
| 4x2 | off | 19.358 | +14.16% | 0/5 | 206 | 193 | 5.63× / 2.48× |
| 4x2 | solver | 20.136 | +18.27% | 0/5 | 213 | 215 | 5.41× / 2.38× |
| 4x2 | fft | 19.510 | +14.18% | 0/5 | 215 | 170 | 5.59× / 2.46× |
| 4x2 | both | 20.005 | +17.54% | 0/5 | 223 | 237 | 5.45× / 2.40× |
| 8x2 | reference | 26.508 | -0.33% | 3/5 | 390 | 364 | 7.13× / 2.79× |
| 8x2 | full | 26.546 | +0.00% | 0/5 | 390 | 366 | 7.12× / 2.79× |
| 8x2 | narrow | 31.523 | +19.20% | 0/5 | 340 | 328 | 6.00× / 2.35× |
| 8x2 | off | 31.109 | +17.19% | 0/5 | 338 | 320 | 6.08× / 2.38× |
| 8x2 | solver | 31.018 | +15.99% | 0/5 | 349 | 366 | 6.09× / 2.39× |
| 8x2 | fft | 30.698 | +16.51% | 0/5 | 355 | 306 | 6.16× / 2.41× |
| 8x2 | both | 30.779 | +15.49% | 0/5 | 365 | 424 | 6.14× / 2.40× |
| 12x2 | reference | 37.136 | -1.02% | 3/5 | 544 | 517 | 7.65× / 2.72× |
| 12x2 | full | 37.420 | +0.00% | 0/5 | 547 | 505 | 7.59× / 2.70× |
| 12x2 | narrow | 45.299 | +19.78% | 0/5 | 484 | 477 | 6.27× / 2.23× |
| 12x2 | off | 44.631 | +18.99% | 0/5 | 481 | 478 | 6.36× / 2.26× |
| 12x2 | solver | 44.479 | +19.54% | 0/5 | 492 | 522 | 6.39× / 2.27× |
| 12x2 | fft | 44.331 | +18.09% | 0/5 | 518 | 440 | 6.41× / 2.28× |
| 12x2 | both | 44.276 | +18.64% | 0/5 | 529 | 768 | 6.41× / 2.28× |
| 16x2 | reference | 44.160 | -1.78% | 3/5 | 647 | 592 | 7.59× / 2.81× |
| 16x2 | full | 44.681 | +0.00% | 0/5 | 645 | 601 | 7.50× / 2.78× |
| 16x2 | narrow | 53.301 | +20.19% | 0/5 | 571 | 552 | 6.29× / 2.33× |
| 16x2 | off | 53.179 | +21.01% | 0/5 | 572 | 553 | 6.30× / 2.33× |
| 16x2 | solver | 53.316 | +19.10% | 0/5 | 591 | 622 | 6.28× / 2.33× |
| 16x2 | fft | 51.649 | +17.41% | 0/5 | 605 | 524 | 6.49× / 2.40× |
| 16x2 | both | 52.714 | +19.40% | 0/5 | 624 | 749 | 6.36× / 2.35× |
| 24x2 | reference | 70.769 | -0.05% | 3/5 | 1050 | 946 | 7.46× / 2.49× |
| 24x2 | full | 70.905 | +0.00% | 0/5 | 1047 | 981 | 7.45× / 2.48× |
| 24x2 | narrow | 85.558 | +20.67% | 0/5 | 919 | 924 | 6.17× / 2.06× |
| 24x2 | off | 85.214 | +19.84% | 0/5 | 915 | 908 | 6.20× / 2.07× |
| 24x2 | solver | 85.868 | +20.90% | 0/5 | 948 | 1012 | 6.15× / 2.05× |
| 24x2 | fft | 86.963 | +22.26% | 0/5 | 992 | 821 | 6.07× / 2.02× |
| 24x2 | both | 85.413 | +20.40% | 0/5 | 1014 | 840 | 6.18× / 2.06× |
| merge-24x1 | reference | 68.803 | +0.15% | 2/5 | 1000 | 882 | 7.37× / 2.54× |
| merge-24x1 | full | 68.037 | +0.00% | 0/5 | 998 | 871 | 7.45× / 2.57× |
| merge-24x1 | narrow | 81.110 | +18.93% | 0/5 | 875 | 812 | 6.25× / 2.16× |
| merge-24x1 | off | 81.077 | +18.98% | 0/5 | 875 | 833 | 6.25× / 2.16× |
| merge-24x1 | solver | 81.894 | +18.83% | 0/5 | 900 | 872 | 6.19× / 2.14× |
| merge-24x1 | fft | 82.918 | +21.48% | 0/5 | 953 | 778 | 6.11× / 2.11× |
| merge-24x1 | both | 82.516 | +20.40% | 0/5 | 980 | 810 | 6.14× / 2.12× |

Entry-thread probes on 8×2 and 16×2 checked that solver and FFT calls executed on the selected pool. All 218 exported proofs verified.

[Protocol](artifacts/protocol.md) · [Process measurements](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Source change](artifacts/source.patch)
