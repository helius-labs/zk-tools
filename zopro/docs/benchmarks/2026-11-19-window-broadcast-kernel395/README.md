# Broadcasting one input across eight G2 bucket destinations

The native component receives one finite point, eight disjoint bucket positions and eight independent signs per packet. It broadcasts the input coordinates in both passes through the existing affine prefix-inversion kernel. The control uses the current signed borrowed G2 kernel on the same updates. Each timed worker checks all final buckets against independent generator multiples after four priming batches and 320 timed batches. The correctness suite also compares every sign mask with complete group addition.

| Workers | Points per batch | Original µs | Broadcast µs | Median paired change | Faster blocks / 7 |
|---:|---:|---:|---:|---:|---:|
| 1 | 8 | 0.965 | 0.940 | -2.62% | 7 |
| 1 | 64 | 2.775 | 2.723 | -1.83% | 7 |
| 1 | 200 | 7.202 | 7.000 | -2.82% | 7 |
| 16 | 8 | 1.045 | 1.021 | -2.06% | 5 |
| 16 | 64 | 3.015 | 2.963 | -1.81% | 7 |
| 16 | 200 | 7.873 | 7.668 | -2.79% | 7 |
| 32 | 8 | 1.514 | 1.421 | -6.47% | 6 |
| 32 | 64 | 5.161 | 4.876 | -5.55% | 7 |
| 32 | 200 | 14.976 | 13.954 | -4.71% | 7 |

The broadcast kernel won all seven paired blocks on 64- and 200-point batches at both multiworker widths. The median paired improvement on these batches was 1.8–2.8% at 16 workers and 4.7–5.6% at 32. One packet used 48 bytes for eight destinations and the shared input reference. Full packets arrive already formed in this test. An end-to-end evaluation must charge for finding them, incomplete batches, exceptional points and the change in window parallelism. These measurements justify the next complete-window test. They are not SDK proof timings.

[Source, caller and oracle](artifacts/kernel.rs) · [Protocol](artifacts/protocol.md) · [Process timings](artifacts/processes.json.gz) · [Packet opportunity](../2026-11-19-window-broadcast394/README.md)
