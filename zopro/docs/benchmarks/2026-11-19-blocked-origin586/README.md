# Encoding the public Euler row reference in 64-row blocks

Each block stores its most frequent public pair, a 64-bit exception mask, and an offset into packed exception indices. The native Montgomery pair dictionary and block streams can be read directly from a little-endian image. A decoder recreated both canonical field elements on every row, checked them against the original stream, and matched its full SHA-256 digest for each of the eight shapes. The same reference in the earlier ordinary layout uses a decoded 64-byte pair dictionary and a flat index for every row.

Uniform zero blocks would allow a centering routine to leave their A and B values untouched. Other uniform blocks can reuse one pair across all rows. The last column compares the previous decoded array payload with the complete proposed disk representation, including header and per-block offsets. This measures representation size and potential visits. Neither runtime memory use nor proof latency was measured here.

| Shape | Rows | Rows matching block default | Rows in uniform blocks / uniform zero blocks | Previous decoded / block image KiB |
|---|---:|---:|---:|---:|
| 1x2 | 30,922 | 89.2% | 70.2% / 69.6% | 149.9 / 107.4 |
| 2x2 | 53,874 | 88.9% | 69.9% / 69.1% | 227.5 / 153.7 |
| 4x2 | 98,674 | 89.4% | 70.2% / 69.3% | 330.0 / 193.9 |
| 8x2 | 189,618 | 89.3% | 70.2% / 69.6% | 603.2 / 341.8 |
| 12x2 | 280,242 | 89.3% | 70.4% / 69.3% | 857.8 / 471.6 |
| 16x2 | 371,122 | 89.4% | 70.0% / 69.0% | 1113.4 / 601.6 |
| 24x2 | 554,226 | 89.2% | 70.1% / 69.4% | 1644.3 / 881.6 |
| merge-24x1 | 525,759 | 20.0% | 12.1% / 10.6% | 2183.4 / 2170.9 |

[Layout sizes and digests](artifacts/layouts.json) · [Encoder and round-trip check](artifacts/encode.rs) · [Original verified reference capture](../2026-11-19-euler-origin566/README.md)
