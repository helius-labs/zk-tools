# Frontend placement and packed FFT on 1×16 and 5×16

These two transfer shapes had different kinds of latency loss at 96 cores in the earlier AWS catalog. This experiment tests the current ordinary executable with the catalog images and requests on 32 Ryzen hardware threads. The four configurations use the release defaults, an eight-worker frontend pinned to CPUs 0–7, packed Euler FFT input and center, or both. The frontend uses the existing SDK pool for eligible preparation and solving. Main worker count and placement are fixed. Each of eight balanced blocks warms three proofs and times eleven per arm. The ratios use medians of process means. Each separate CPU profile covers fifteen warm requests. All 72 exports verified using the original verification keys.

| Shape | Configuration | Warm ms | Release / arm | Paired change (wins) | CPU-ms/proof | Busy cores | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| 1x16 | release | 12.857 | 1.000× | +0.00% (0/8) | 306.9 | 23.8 | 247.0 |
| 1x16 | front8 | 13.058 | 0.985× | +1.48% (1/8) | 250.3 | 19.1 | 128.0 |
| 1x16 | combined | 13.072 | 0.984× | +1.83% (1/8) | 249.4 | 19.1 | 125.2 |
| 1x16 | packed | 12.846 | 1.001× | +0.08% (3/8) | 294.3 | 22.8 | 246.2 |
| 5x16 | release | 23.873 | 1.000× | +0.00% (0/8) | 648.3 | 27.2 | 420.0 |
| 5x16 | front8 | 24.809 | 0.962× | +4.43% (1/8) | 619.8 | 25.0 | 412.2 |
| 5x16 | combined | 24.048 | 0.993× | +0.86% (2/8) | 595.6 | 24.8 | 425.5 |
| 5x16 | packed | 23.465 | 1.017× | -1.52% (8/8) | 635.6 | 27.1 | 405.8 |

The eight-worker frontend nearly halved 1×16 peak memory (247.0 to 128.0 MiB) and cut CPU time by 18%, while warm latency increased 1.48% in paired blocks. The packed FFT on the main pool won all eight pairs on 5×16. It reduced CPU time and peak memory on that witness. On 1×16 it split three wins to five losses. Diagnostic output confirmed that the combined arm's packed FFT ran on eight workers on 1×16 and on 32 on 5×16. The two keys dispatch different work to the frontend pool.

[Historical AWS work and parallelism](../2026-11-19-c7a-work655/README.md) · [Exact xtask configurations](artifacts/configs.json.gz) · [Warm timings and separate CPU profiles](artifacts/rows.json.gz) · [Diagnostic markers](artifacts/profiles.json) · [Verification](artifacts/verification.json)
