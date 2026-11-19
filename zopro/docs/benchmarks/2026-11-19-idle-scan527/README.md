# Rayon idle searches across pool widths

Rayon 1.13.0 scans other workers' queues when its own queue is empty. Its default sleep protocol permits 32 unsuccessful search rounds before marking a worker sleepy. The experiment sets the number of rounds to the smaller of 32 and ceil(budget / (workers−1)) in pools of at least 16 workers. Budgets of 128 and 512 correspond to five and seventeen rounds at 32 workers. The setting controls when the existing sleep handshake begins. A search finding work can stop early, and a contended scan can retry, so the budget is a nominal count of victim probes. The wake and latch protocol and work distribution retain the upstream implementation. The small frontend pools keep their default settings.

Four randomized Ryzen blocks compare the frozen executable, the patched executable with ordinary sleep, and the two thresholds. All eight original requests ran at 32 workers. The 16×2 and merge also used 16 and 24 workers on the same 16 physical cores with additional SMT siblings rotated evenly between CCDs. Each process warmed three proofs then timed 31 small or eleven larger requests. Every exported proof was checked by the independent verifier. The disabled and experimental processes reported each created registry's actual worker count and round limit. CPU time and context switches cover the warm proof loop. Peak RSS includes setup.

| Shape | Workers | Frozen / disabled / 128 / 512 ms | 128 / 512 paired vs disabled | CPU-ms disabled / 128 / 512 | Peak MiB disabled / 128 / 512 | 128 vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 32 | 7.134 / 7.033 / 7.110 / 7.142 | +0.61% (0/4) / +1.17% (1/4) | 145.7 / 138.8 / 142.7 | 66 / 63 / 62 | 5.20× / 3.38× |
| 2x2 | 32 | 9.591 / 9.612 / 9.628 / 9.625 | +0.31% (1/4) / +0.01% (2/4) | 193.8 / 185.0 / 189.7 | 116 / 114 / 122 | 6.13× / 3.12× |
| 4x2 | 32 | 15.357 / 15.477 / 15.926 / 15.523 | +1.82% (0/4) / +1.16% (1/4) | 406.2 / 387.2 / 394.4 | 260 / 224 / 264 | 6.84× / 3.01× |
| 8x2 | 32 | 23.854 / 23.896 / 24.958 / 24.065 | +4.18% (0/4) / +0.67% (1/4) | 654.8 / 649.3 / 650.4 | 434 / 371 / 426 | 7.57× / 2.97× |
| 12x2 | 32 | 31.027 / 31.043 / 32.096 / 31.557 | +3.72% (1/4) / +0.60% (2/4) | 860.2 / 851.9 / 857.0 | 574 / 457 / 547 | 8.85× / 3.15× |
| 16x2 | 32 | 39.766 / 39.370 / 39.948 / 40.171 | +1.41% (1/4) / +2.07% (2/4) | 1139.9 / 1114.0 / 1129.3 | 671 / 560 / 660 | 8.39× / 3.10× |
| 24x2 | 32 | 64.303 / 64.210 / 64.373 / 63.666 | +0.27% (2/4) / -1.30% (3/4) | 1832.5 / 1816.2 / 1827.2 | 1021 / 874 / 993 | 8.20× / 2.73× |
| merge-24x1 | 32 | 63.428 / 63.258 / 63.755 / 63.336 | +1.07% (1/4) / +0.18% (2/4) | 1800.9 / 1760.7 / 1778.8 | 1040 / 834 / 1030 | 7.95× / 2.74× |
| 16x2 | 16 | 43.479 / 43.386 / 43.709 / 43.240 | +0.94% (0/4) / -0.39% (3/4) | 637.3 / 633.6 / 634.3 | 561 / 533 / 560 | 7.66× / 2.84× |
| 16x2 | 24 | 39.771 / 40.075 / 40.774 / 39.988 | +1.87% (1/4) / -0.08% (2/4) | 875.5 / 869.6 / 866.3 | 627 / 535 / 608 | 8.22× / 3.04× |
| merge-24x1 | 16 | 68.371 / 68.172 / 68.673 / 68.355 | +1.08% (1/4) / +0.05% (2/4) | 1001.7 / 993.6 / 1003.6 | 914 / 796 / 919 | 7.38× / 2.55× |
| merge-24x1 | 24 | 63.030 / 63.125 / 63.630 / 63.885 | +1.14% (0/4) / +1.72% (1/4) | 1390.5 / 1373.6 / 1402.7 | 955 / 789 / 978 | 7.97× / 2.75× |

All 192 process exports verified. Hardware results in this report are for Ryzen. [All samples and switch counts](artifacts/processes.json.gz) · [Scheduler patch](artifacts/rayon-core.patch) · [Runtime tests](artifacts/build.json) · [Verification](artifacts/verification.json)
