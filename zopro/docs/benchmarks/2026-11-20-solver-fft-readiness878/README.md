# FFT tile readiness across solver bands

The first inverse radix-32 layer has no complete 16-column tile before the last solver band on 1×2, 2×2 or 4×2. On 4×2, a radix-4 first layer has all required rows for 372 of its 512 tiles before the final solver band. On 1×2 and 2×2 it has 48 of 128 and 128 of 256 tiles, respectively. A smaller first layer changes the part of the FFT that can overlap solving.

The reader checks the three original solver-image hashes from the [Poseidon comparison](../2026-11-20-poseidon-first-round-ryzen877/README.md). It reconstructs dependency depths from operation reads and writes and applies the recorded four-band assignment, including the blended depth and work rule when packing is enabled. Each FFT column uses a four-row lane group from every input stripe of the first layer. A tile becomes available after the latest band among its input rows. Rows beyond the constraint count supply zeros. The JSON includes tile widths of 1, 16 and 64 columns.

| Shape | First-layer radix | 16-column tiles | Cumulative tiles ready after each of four bands |
|---|---:|---:|---:|
| 1×2 | 2 | 256 | 11 / 86 / 176 / 256 |
| 1×2 | 4 | 128 | 0 / 0 / 48 / 128 |
| 1×2 | 8 | 64 | 0 / 0 / 6 / 64 |
| 1×2 | 16 | 32 | 0 / 0 / 0 / 32 |
| 1×2 | 32 | 16 | 0 / 0 / 0 / 16 |
| 2×2 | 2 | 512 | 124 / 250 / 384 / 512 |
| 2×2 | 4 | 256 | 0 / 11 / 128 / 256 |
| 2×2 | 8 | 128 | 0 / 0 / 8 / 128 |
| 2×2 | 16 | 64 | 0 / 0 / 0 / 64 |
| 2×2 | 32 | 32 | 0 / 0 / 0 / 32 |
| 4×2 | 2 | 1024 | 364 / 596 / 830 / 1024 |
| 4×2 | 4 | 512 | 141 / 257 / 372 / 512 |
| 4×2 | 8 | 256 | 0 / 2 / 116 / 256 |
| 4×2 | 16 | 128 | 0 / 0 / 6 / 128 |
| 4×2 | 32 | 64 | 0 / 0 / 0 / 64 |

The counts describe availability at outer band boundaries. Lower-level solver tasks may finish rows earlier within a band. An overlapping implementation would still pay for extra dispatch, intermediate FFT storage and the remaining transform. The scan does not measure those costs or full-proof latency.

[Counts](artifacts/counts.json) · [Reader](artifacts/reader.py) · [Pinned workflow](../2026-11-20-poseidon-first-round-ryzen877/artifacts/workflow.json.gz) · [Image pins](../2026-11-20-poseidon-first-round-ryzen877/artifacts/pins.json)
