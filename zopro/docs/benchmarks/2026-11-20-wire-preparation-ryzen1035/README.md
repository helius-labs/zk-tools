# Packed wire conversion and in-place solver slots on Ryzen

Eight-lane IFMA conversion reduced the observed gather interval by 30–39 µs on 1×2 and 2×2. Compaction applies to samples with 10–59 selected slots and had a sub-microsecond effect on the slot interval. No configuration improved full-proof latency consistently on both inputs against both controls.

| Shape | Request | Arm | Proof ms | Paired vs control | Paired vs frozen | Gather µs | Slot selection µs |
|---|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | frozen | 7.515 | -0.14% (4/6) | +0.00% (0/6) | — | — |
| 1×2 | catalog | control | 7.488 | +0.00% (0/6) | +0.14% (2/6) | 84.39 | 0.33 |
| 1×2 | catalog | packed | 7.501 | +0.09% (3/6) | -0.78% (4/6) | 54.53 | 0.17 |
| 1×2 | catalog | compact | 7.493 | +0.08% (3/6) | -0.47% (4/6) | 84.86 | 0.17 |
| 1×2 | catalog | both | 7.492 | -0.08% (4/6) | -0.79% (4/6) | 59.36 | 0.19 |
| 1×2 | heldout | frozen | 7.539 | +0.67% (3/6) | +0.00% (0/6) | — | — |
| 1×2 | heldout | control | 7.540 | +0.00% (0/6) | -0.66% (3/6) | 81.53 | 0.25 |
| 1×2 | heldout | packed | 7.480 | -0.63% (4/6) | -0.82% (5/6) | 51.75 | 0.17 |
| 1×2 | heldout | compact | 7.480 | +0.54% (2/6) | -0.76% (4/6) | 82.61 | 0.19 |
| 1×2 | heldout | both | 7.409 | -1.65% (4/6) | -1.42% (6/6) | 64.00 | 0.15 |
| 2×2 | catalog | frozen | 11.220 | -0.94% (4/6) | +0.00% (0/6) | — | — |
| 2×2 | catalog | control | 11.333 | +0.00% (0/6) | +0.95% (2/6) | 116.00 | 0.32 |
| 2×2 | catalog | packed | 11.288 | -0.66% (5/6) | +0.40% (2/6) | 83.66 | 0.36 |
| 2×2 | catalog | compact | 11.250 | -0.39% (3/6) | +0.54% (1/6) | 129.34 | 0.34 |
| 2×2 | catalog | both | 11.209 | -1.04% (5/6) | +0.01% (3/6) | 86.04 | 0.23 |
| 2×2 | heldout | frozen | 11.274 | -0.30% (4/6) | +0.00% (0/6) | — | — |
| 2×2 | heldout | control | 11.273 | +0.00% (0/6) | +0.30% (2/6) | 134.95 | 0.26 |
| 2×2 | heldout | packed | 11.275 | +0.22% (2/6) | -0.25% (4/6) | 95.97 | 0.34 |
| 2×2 | heldout | compact | 11.288 | +0.32% (2/6) | +0.39% (2/6) | 129.20 | 0.41 |
| 2×2 | heldout | both | 11.258 | -0.04% (3/6) | -0.27% (3/6) | 89.83 | 0.27 |
| 4×2 | catalog | frozen | 15.366 | +0.73% (2/6) | +0.00% (0/6) | — | — |
| 4×2 | catalog | control | 15.100 | +0.00% (0/6) | -0.72% (4/6) | — | 0.50 |
| 4×2 | catalog | compact | 15.101 | -0.28% (3/6) | -1.74% (5/6) | — | 0.52 |
| 4×2 | heldout | frozen | 15.015 | -0.08% (3/6) | +0.00% (0/6) | — | — |
| 4×2 | heldout | control | 15.009 | +0.00% (0/6) | +0.09% (3/6) | — | 0.52 |
| 4×2 | heldout | compact | 14.921 | -0.28% (3/6) | -0.67% (6/6) | — | 0.54 |
| 8×2 | catalog | frozen | 24.750 | -0.12% (3/6) | +0.00% (0/6) | — | — |
| 8×2 | catalog | control | 24.866 | +0.00% (0/6) | +0.12% (3/6) | — | 0.95 |
| 8×2 | catalog | compact | 24.744 | +0.14% (3/6) | -0.12% (3/6) | — | 0.74 |
| 8×2 | heldout | frozen | 24.954 | +0.82% (2/6) | +0.00% (0/6) | — | — |
| 8×2 | heldout | control | 24.822 | +0.00% (0/6) | -0.82% (4/6) | — | 1.07 |
| 8×2 | heldout | compact | 24.917 | -0.12% (3/6) | +0.03% (3/6) | — | 0.89 |

Separate route runs checked the packed conversion and in-place compaction. The grouped 4×2 and 8×2 routes were tested for slot compaction. Two native tests cover the scalar and packed conversions, tails, and the fallback for unordered or duplicate slots. Six timing blocks used 24 main and eight frontend Ryzen workers. Gnark independently reverified all 216 exports.

[Summary](artifacts/summary.json) · [Samples and route counts](artifacts/rows.json.gz) · [Source patch](artifacts/source.patch.gz) · [Protocol](artifacts/protocol.json)
