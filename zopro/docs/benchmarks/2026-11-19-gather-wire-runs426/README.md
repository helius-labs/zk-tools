# Copying consecutive B-query wires in bulk

The solver image and compiled program both require strictly increasing query indices. When the difference between the first and last wire in a block equals its length minus one, the prover can copy that block directly. The prototype splits any gapped block until it finds contiguous runs or reaches a short indexed fallback. It uses the same output allocation and requires no per-key table. Only the ordinary B query uses this path. FFT input-column packing was disabled throughout this experiment. The separate 425 measurement qualified packing on both shapes; a deployment using both options requires a combined measurement.

| Shape | B rows eligible for bulk copying | Release / disabled / enabled (ms) | Paired vs release (wins / 7) | Paired vs disabled (wins / 7) | Peak MiB release / disabled / enabled | CPU-ms release / disabled / enabled | M5 CPU / Metal speedup, enabled |
|---|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 98.3% | 40.728 / 40.186 / 40.147 | -1.65% (6/7) | -0.10% (4/7) | 790.2 / 795.2 / 774.8 | 1168 / 1164 / 1157 | 8.34× / 3.09× |
| merge-24x1 | 98.9% | 66.894 / 66.437 / 66.548 | -0.72% (4/7) | +0.09% (3/7) | 1115.2 / 1113.1 / 1100.2 | 1905 / 1913 / 1908 | 7.62× / 2.63× |

The paired confirmation criterion was met by neither shape. The route probes recorded one entry per timed enabled proof on both shapes. Independent Gnark verification succeeded on all 46 exports. These results cover the fixed inputs on one Ryzen at 32 hardware threads.

[Source patch](artifacts/source.patch) · [Per-key coverage](artifacts/coverage.json) · [CPU and wall samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json) · [C7a trace analysis](../2026-11-19-c7a-curve-tail424/README.md)
