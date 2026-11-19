# SDK folded FFT placement with stock and tuned glibc

**Stock glibc did not reproduce the SDK’s memory saving on the original 24×2 request.** Enabling the smaller pool raised the observed maximum from 892.3 to 897.9 MiB. With the benchmark’s tuned glibc settings, it reduced peak RSS from 1,067.3 to 946.2 MiB and improved latency in seven of seven pairs. For this input, stock glibc on the main pool had the lowest observed peak. Tuned glibc on the smaller pool had the lowest median latency. The evidence supports enabling the SDK option for the measured tuned server profile.

Allocator settings also changed the number of minor faults substantially. Median faults per timed proof were 30,292 and 27,874 on stock glibc (main and pool), compared with 4,491 and 2,621 on tuned glibc. The differences cover the entire SDK call. They cannot identify the individual allocations responsible.

All four arms use the same release executable, original 24×2 request, key and pinning. Each main/pool contrast changes the 30-worker dispatch and its attached pool together. The paired percentage is the median of seven same-block changes in full proof latency. Each reported time is the median of the seven process means. Maximum lifetime RSS includes loading, three warmups and all seventeen timed calls.

| glibc | Main / pool ms | Pool vs main | Max main / pool MiB | Median paired RSS difference, MiB | Pool vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| stock | 68.615 / 69.114 | +0.70% (3/7) | 892.3 / 897.9 | +6.7 (3/7 lower) | 7.64× / 2.55× |
| tuned | 66.111 / 62.770 | -4.73% (7/7) | 1067.3 / 946.2 | -123.8 (7/7 lower) | 8.41× / 2.80× |

The eight independent profile processes recorded user-mode CPU cycles across the warmed request loop. All 36 exported proofs verified against the original public input. [Protocol](protocol.md) · [Per-process timings and sampled symbols](artifacts/rows.json.gz) · [Paired comparisons](artifacts/paired.json) · [xtask result](artifacts/xtask-result.json) · [Verification](artifacts/verification.json)
