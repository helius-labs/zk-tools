# Integrated G2 scheduling and CPU scaling on C7a

On 1×2 at 96 physical cores, switching the integrated release from parallel to serial G2 scalar decomposition with four point ranges changed verified proof latency from 8.985 to 8.512 ms. The paired change was -5.40% with 7/7 faster blocks. The integrated serial build differed by -0.23% from the earlier serial candidate. The prespecified replication criteria were met.

The ordinary two-range release took 8.451 ms at 64 cores and 9.918 ms at 96. Aeglos measured 8.850 and 7.987 ms at those same widths. At 32 cores the ordinary release took 10.819 ms. At fixed settings every reference shape had a higher median at 96 than at 64. The size of the regression varied by shape.

[Scaling on all eight reference shapes](scaling.md) · [Timing inside each request](prefix.md) · [Frontend worker counts](frontend.md) · [Solver experiment](solver.md)

## 1×2 by core count

The benchmark uses seven fresh processes per setting in randomized order. Each times 101 requests after three primes. Wall time and aggregate CPU time are medians of process means. Percentiles pool 707 requests and peak RSS is the largest process maximum. All Zopro rows share 16 pinned frontend workers, on the same core mask as the main pool.

| Physical cores | Engine or G2 configuration | Wall ms | Request p50 / p95 / p99 ms | ≥10 ms | CPU-ms/proof | Peak MiB |
|---:|---|---:|---:|---:|---:|---:|
| 32 | Two, parallel | 10.819 | 10.766 / 11.294 / 11.788 | 707/707 | 221.8 | 66 |
| 32 | Four, parallel | 11.260 | 11.236 / 11.661 / 11.925 | 707/707 | 234.3 | 66 |
| 32 | Four, serial | 11.317 | 11.299 / 11.734 / 11.967 | 707/707 | 231.7 | 66 |
| 64 | Two, parallel | 8.451 | 8.347 / 8.647 / 11.691 | 22/707 | 274.3 | 54 |
| 64 | Four, parallel | 9.181 | 8.933 / 9.889 / 10.108 | 16/707 | 297.5 | 54 |
| 64 | Four, serial | 9.056 | 9.036 / 9.390 / 9.579 | 1/707 | 294.5 | 54 |
| 64 | Prior parallel release | 9.021 | 8.824 / 9.855 / 10.004 | 9/707 | 293.9 | 56 |
| 64 | Prior serial candidate | 9.031 | 9.005 / 9.370 / 9.598 | 0/707 | 293.6 | 54 |
| 64 | Aeglos | 8.850 | 8.798 / 9.660 / 10.093 | 9/707 | 427.3 | 445 |
| 80 | Two, parallel | 9.517 | 8.594 / 11.975 / 12.184 | 238/707 | 333.3 | 55 |
| 80 | Four, parallel | 8.782 | 8.825 / 9.249 / 10.751 | 21/707 | 344.3 | 55 |
| 80 | Four, serial | 8.813 | 8.791 / 9.202 / 9.350 | 0/707 | 335.0 | 52 |
| 88 | Two, parallel | 10.126 | 8.852 / 12.100 / 12.276 | 327/707 | 372.4 | 55 |
| 88 | Four, parallel | 8.732 | 8.408 / 10.764 / 11.005 | 82/707 | 373.4 | 51 |
| 88 | Four, serial | 8.545 | 8.527 / 8.936 / 9.210 | 4/707 | 352.7 | 49 |
| 96 | Two, parallel | 9.918 | 8.894 / 12.257 / 12.438 | 272/707 | 403.3 | 56 |
| 96 | Four, parallel | 8.985 | 8.509 / 11.019 / 11.180 | 163/707 | 408.5 | 52 |
| 96 | Four, serial | 8.512 | 8.495 / 8.839 / 8.994 | 4/707 | 390.2 | 53 |
| 96 | Prior parallel release | 9.074 | 8.541 / 11.061 / 11.339 | 164/707 | 409.6 | 53 |
| 96 | Prior serial candidate | 8.531 | 8.483 / 8.845 / 8.995 | 2/707 | 388.8 | 53 |
| 96 | Aeglos | 7.987 | 7.914 / 8.571 / 9.021 | 0/707 | 512.1 | 478 |

## Supplied M5 comparisons

Each Zopro cell uses the shape and width specified before launch. Every shape except 1×2 times 21 requests per process. The last column counts blocks meeting the 10× M5 CPU threshold.

| Shape | Cores | Wall ms | Peak MiB | M5 CPU / Metal ms | CPU / Metal speedup | CPU target |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 64 | 8.451 | 54 | 37 / 24 | 4.38× / 2.84× | 0/7 |
| 2x2 | 48 | 14.127 | 101 | 59 / 30 | 4.18× / 2.12× | 0/7 |
| 4x2 | 48 | 17.984 | 278 | 109 / 48 | 6.06× / 2.67× | 0/7 |
| 8x2 | 48 | 24.651 | 455 | 189 / 74 | 7.67× / 3.00× | 0/7 |
| 12x2 | 64 | 33.529 | 656 | 284 / 101 | 8.47× / 3.01× | 0/7 |
| 16x2 | 64 | 36.845 | 809 | 335 / 124 | 9.09× / 3.37× | 0/7 |
| 24x2 | 64 | 50.994 | 1239 | 528 / 176 | 10.35× / 3.45× | 6/7 |
| merge-24x1 | 64 | 53.708 | 1122 | 507 / 175 | 9.44× / 3.26× | 0/7 |

The C7a instance terminated, its tagged volume count returned to zero and its temporary bucket was deleted. All 385 retrieved timing exports and 30 diagnostic exports verified independently on both machines. A separate 70-process frontend sweep was excluded because an archive filename pattern omitted its output directory.

[Protocol](artifacts/protocol.md) · [Configuration](artifacts/settings.json) · [Per-process and per-request data](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
