# Skipping unproductive G2 scalar grouping on C7a

On 1×2 at 96 cores, checking scalar diversity during the FFT reduced median verified proof time from **11.62 to 10.11 ms** with the same executable and frontend worker limit. The three paired block gains were 12.4%, 13.0% and 13.2%. CPU time per proof fell 6.7% in the median pair. The largest observed process RSS fell from 127.9 to 79.1 MiB. At 32 cores the paired latency gain was 2.0%.

Before G2 computes its multiscalar multiplication, it can combine points with equal scalars. When the scalars are too diverse, the existing policy discards its grouping map. The prototype counts distinct occupied bits in a temporary bitmap while the FFT runs. Every occupied bit establishes a distinct nonzero scalar. Once the count exceeds the existing cutoff, the prover can skip building the map. Collisions or memory pressure preserve the original fallback. The check needs at most 128 KiB of scratch on the tested path and retains no per-key table. [Source changes relative to experiment 255](artifacts/prototype.patch.gz)

## Nine-shape panel

REF is the exact experiment 255 width-limited prototype. OFF and ON are the new executable with the same width policy. An inherited baseline label in the run configuration referred to the accepted build. The executable digest and frozen protocol identify the experimental reference. Times are medians of three independent process means. CPU time sums across cores. Peak RSS covers the process lifetime. Certificate counts cover timed requests in the three plain blocks.

| Shape | Cores | Certificates / attempts | REF ms | OFF ms | ON ms | CPU ms, OFF / ON | Peak MiB, REF / ON | M5 CPU / ON | M5 Metal / ON |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 32 | 453/453 | 10.86 | 10.95 | 10.70 | 231 / 231 | 120 / 74 | 3.46× | 2.24× |
| 1x2 | 96 | 453/453 | 11.48 | 11.62 | 10.11 | 488 / 455 | 116 / 79 | 3.66× | 2.37× |
| 2x2 | 32 | 0/153 | 14.51 | 14.46 | 14.44 | 315 / 314 | 272 / 270 | 4.09× | 2.08× |
| 2x2 | 96 | 0/153 | 15.43 | 15.52 | 15.50 | 702 / 700 | 265 / 283 | 3.81× | 1.94× |
| 4x2 | 32 | 0/0 | 19.47 | 19.61 | 19.56 | 493 / 493 | 278 / 283 | 5.57× | 2.45× |
| 4x2 | 96 | 0/0 | 20.76 | 20.77 | 20.72 | 1346 / 1337 | 273 / 253 | 5.26× | 2.32× |
| 8x2 | 32 | 0/0 | 27.10 | 26.82 | 26.88 | 735 / 736 | 439 / 455 | 7.03× | 2.75× |
| 8x2 | 96 | 0/0 | 28.23 | 27.98 | 27.50 | 1956 / 1945 | 449 / 446 | 6.87× | 2.69× |
| 12x2 | 32 | 0/0 | 35.60 | 35.72 | 35.26 | 988 / 982 | 636 / 649 | 8.06× | 2.86× |
| 12x2 | 96 | 0/0 | 35.59 | 36.17 | 35.48 | 2476 / 2465 | 639 / 623 | 8.00× | 2.85× |
| 16x2 | 32 | 0/0 | 41.69 | 42.14 | 40.65 | 1195 / 1156 | 741 / 769 | 8.24× | 3.05× |
| 16x2 | 96 | 0/0 | 41.84 | 41.40 | 41.54 | 2764 / 2774 | 728 / 754 | 8.07× | 2.99× |
| 24x2 | 32 | 0/0 | 59.11 | 60.53 | 60.08 | 1723 / 1743 | 1169 / 1220 | 8.79× | 2.93× |
| 24x2 | 96 | 0/0 | 56.79 | 56.84 | 56.57 | 3661 / 3636 | 1177 / 1222 | 9.33× | 3.11× |
| merge-24x1 | 32 | 0/0 | 62.45 | 61.30 | 63.78 | 1670 / 1713 | 1140 / 1027 | 7.95× | 2.74× |
| merge-24x1 | 96 | 0/0 | 61.62 | 61.18 | 61.16 | 3754 / 3766 | 1112 / 1126 | 8.29× | 2.86× |
| merge-54x1 | 32 | 0/0 | 141.83 | 140.52 | 140.99 | 3557 / 3583 | 2114 / 2055 | — | — |
| merge-54x1 | 96 | 0/0 | 123.31 | 124.86 | 123.13 | 6257 / 6264 | 2034 / 1955 | — | — |

1×2 certified on every attempt. 2×2 always fell back and the general route did not attempt this check. All 18 shape/width cells passed the prespecified control, median latency and peak RSS limits. None of the eight shapes with M5 references reached 10× CPU. The 24×2 result came closest at 9.33×. Merge54 has no supplied M5 comparison.

All 234 remote exports passed Gnark on the benchmark host and after retrieval. The remote run contained 11,052 proof calls, including 10,350 timed calls. Six local exports exercised success, fallback and the general route. The merged verification ledger reached 65,069 distinct exported proof hashes. Both ON profiling modes exceeded the 5% overhead limit, so their counters cannot explain the plain latency gain quantitatively. This remains an experimental implementation. The VM, tagged storage and temporary bucket were removed. [Protocol](artifacts/protocol.md) · [Metrics](artifacts/metrics.json) · [Per-process results](artifacts/processes.json.gz) · [Verification](artifacts/review.json) · [Profiles](artifacts/profiles.json.gz)
