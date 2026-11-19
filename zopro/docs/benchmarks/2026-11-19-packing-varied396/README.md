# Solver task packing across varied transfer requests on Ryzen

The same clean executable ran with task claiming, task packing or both. The integrated release ran with claiming enabled. Each row compares five fresh randomized blocks at 32 pinned hardware threads with 31 timed proofs per process. The sixteen requests have distinct pinned public inputs. The primary question is the marginal effect of enabling packing while keeping claiming enabled.

| Shape | Request | Release ms | Claim ms | Pack ms | Both ms | Paired vs release (wins) | Paired vs claim (wins) | Paired vs pack (wins) | Peak MiB release / both |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 1 | 7.493 | 7.320 | 7.313 | 7.306 | -1.84% (5) | -0.19% (3) | +0.04% (2) | 66.2 / 65.5 |
| 1x2 | 2 | 7.458 | 7.354 | 7.343 | 7.324 | -1.92% (5) | -0.52% (3) | -0.08% (3) | 62.8 / 63.8 |
| 1x2 | 3 | 7.460 | 7.363 | 7.332 | 7.374 | -1.32% (3) | -0.14% (3) | +0.02% (2) | 64.0 / 64.0 |
| 1x2 | 4 | 7.446 | 7.306 | 7.298 | 7.290 | -2.23% (5) | -0.58% (3) | -0.10% (4) | 63.0 / 65.5 |
| 1x2 | 5 | 7.458 | 7.397 | 7.314 | 7.285 | -2.30% (5) | -1.48% (5) | -0.83% (5) | 65.0 / 64.5 |
| 1x2 | 6 | 7.417 | 7.374 | 7.322 | 7.310 | -1.45% (4) | -0.80% (3) | -0.16% (3) | 64.0 / 62.8 |
| 1x2 | 7 | 7.455 | 7.340 | 7.375 | 7.306 | -2.64% (5) | -0.82% (3) | -0.50% (3) | 66.0 / 64.5 |
| 1x2 | 8 | 7.393 | 7.364 | 7.365 | 7.269 | -1.69% (5) | -1.30% (4) | -1.21% (4) | 64.8 / 65.0 |
| 2x2 | 1 | 10.451 | 10.273 | 10.228 | 10.151 | -3.52% (5) | +0.27% (2) | -0.89% (3) | 118.8 / 114.0 |
| 2x2 | 2 | 10.406 | 10.312 | 10.232 | 10.158 | -2.33% (5) | -1.09% (5) | -0.27% (5) | 116.2 / 116.8 |
| 2x2 | 3 | 10.345 | 10.334 | 10.210 | 10.171 | -1.95% (4) | -1.57% (5) | -0.80% (5) | 113.8 / 121.5 |
| 2x2 | 4 | 10.328 | 10.286 | 10.205 | 10.168 | -1.96% (5) | -1.03% (5) | -0.77% (4) | 114.2 / 114.5 |
| 2x2 | 5 | 10.386 | 10.280 | 10.234 | 10.220 | -1.75% (4) | -0.33% (4) | -0.51% (4) | 113.8 / 111.8 |
| 2x2 | 6 | 10.396 | 10.254 | 10.283 | 10.156 | -3.05% (4) | -0.96% (5) | -2.11% (4) | 115.5 / 119.0 |
| 2x2 | 7 | 10.400 | 10.319 | 10.142 | 10.306 | -0.91% (4) | -0.13% (3) | +0.91% (2) | 115.2 / 125.2 |
| 2x2 | 8 | 10.460 | 10.308 | 10.259 | 10.179 | -2.69% (4) | -1.25% (3) | -0.69% (3) | 111.8 / 114.0 |

On 2×2 the combined setting beat the release on eight of eight distinct requests. The median of their paired changes was −2.15%. Against the same executable with claiming alone it won seven of eight, with a median change of −0.99%. On 1×2 it won all eight comparisons against each claiming control. These results support exposing the independent native packing option. The integrated solver source matched the benchmarked source byte for byte. All 150 native core tests passed with both options enabled. Packing is opt-in through `ZOPRO_SOLVE_PACK_TASKS=1`. This experiment used 320 processes, 9,920 timed proofs, and independent Gnark verification of each process export.

The prespecified 2×2 marginal comparison passed its requirement against both claiming controls. This study targets the effect of enabling packing in an existing configuration. The preceding full-shape study measured the separate question of whether both settings outperform either standalone policy.

[Request and public-input hashes](../2026-11-18-varied-requests309/artifacts/variants.json) · [Full-shape results](../2026-11-19-packed-claims-release393/README.md) · [Protocol](artifacts/protocol.md) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
