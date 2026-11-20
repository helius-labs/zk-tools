# Twiddle reuse inside Euler FFT columns on Ryzen

The column kernel loads and broadcasts a twiddle once for every position in a butterfly stage, then applies it to the independent blocks at that position. The control visits blocks first. The change preserves butterfly dependencies and both root layouts, and adds no workspace. It affects the radix-32 column kernel used by the packed and ordinary-layout Euler paths.

Paired full-proof latency changed by -0.38% to +1.25% against the same build. In the separate diagnostic processes, elapsed time for the complete Euler FFT changed by -9.0% to +3.9% across the eight inputs. The test build remains isolated.

| Shape | Input | Frozen / control / reuse ms | Reuse vs control | Reuse vs frozen | Control / reuse FFT ms | Control / reuse peak MiB |
|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | 7.472 / 7.489 / 7.470 | +0.61% (2/6) | -0.34% (4/6) | 0.501 / 0.508 | 58.8 / 57.8 |
| 1×2 | heldout | 7.494 / 7.467 / 7.508 | +1.25% (2/6) | -0.02% (3/6) | 0.518 / 0.538 | 58.0 / 57.0 |
| 2×2 | catalog | 11.173 / 11.214 / 11.320 | +0.67% (2/6) | +0.60% (2/6) | 1.152 / 1.048 | 85.2 / 91.8 |
| 2×2 | heldout | 11.339 / 11.329 / 11.376 | +0.16% (3/6) | -0.18% (4/6) | 1.024 / 1.032 | 85.8 / 89.5 |
| 4×2 | catalog | 15.070 / 15.176 / 15.107 | -0.23% (5/6) | -0.38% (3/6) | 2.282 / 2.164 | 221.0 / 230.8 |
| 4×2 | heldout | 14.992 / 15.036 / 15.019 | +0.14% (3/6) | -0.26% (4/6) | 2.317 / 2.292 | 226.2 / 219.2 |
| 12×2 | catalog | 32.165 / 32.179 / 32.189 | +0.69% (2/6) | +0.27% (3/6) | 8.703 / 8.581 | 497.3 / 522.2 |
| 12×2 | heldout | 32.142 / 32.210 / 32.177 | -0.38% (4/6) | +0.05% (3/6) | 8.705 / 8.854 | 494.5 / 501.5 |

Six order-balanced blocks measure thirteen proof requests after three warmups on the Ryzen 9950X, with 24 main and eight frontend logical workers. The frozen binary, requests and images are pinned to the three-prover C7a benchmark. The local worker counts and hardware differ from C7a. The qualification threshold is five of six paired wins against each control on both requests, a lower paired median and calibration within 2%. Peak memory is the highest whole-process RSS.

The native test compares all eight scalar lanes in both FFT directions, at outer and inner levels, with cached and ordinary roots. Separate proof probes cover the selected radix-32 path for every input. Gnark independently reverified all 160 exported proofs.

[Summary](artifacts/summary.json) · [Block samples and route probes](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial patch](artifacts/source.patch.gz) · [C7a baseline](../2026-11-20-c7a-three-provers/README.md)
