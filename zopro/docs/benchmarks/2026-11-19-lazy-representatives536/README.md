# Reading nested group representatives directly from the witness

The upper B2 query can read the first-level representatives by their source indices. This removes a full intermediate array of field elements on the compact route. When grouping declines, the dense route writes canonical words directly. The experiment retains the conservative scratch reservation in both modes. Other prepared points, fallback conditions and correction arithmetic are identical.

The frozen two-level build and the two modes of the experimental build used the same 30 Ryzen workers within each randomized block. The original, two adjacent and two scattered public requests each used five blocks, three priming proofs and eleven timed calls per process. Peak RSS includes key setup.

| 12×2 request | Frozen / ordinary / direct view ms | View vs ordinary paired | View vs frozen paired | Peak MiB frozen / ordinary / view |
|---|---:|---:|---:|---:|
| Original | 29.722 / 29.426 / 29.225 | -1.24% (3/5) | -1.23% (4/5) | 583.7 / 577.5 / 538.2 |
| 59 | 29.695 / 29.318 / 29.313 | -0.80% (3/5) | -0.50% (3/5) | 595.5 / 587.9 / 539.0 |
| 64 | 29.475 / 29.600 / 29.436 | -0.55% (4/5) | -1.09% (5/5) | 584.3 / 586.8 / 534.3 |
| 141 | 46.087 / 45.223 / 46.165 | +2.08% (2/5) | -1.51% (3/5) | 980.5 / 993.3 / 976.4 |
| 144 | 45.121 / 45.355 / 45.575 | +0.14% (2/5) | +0.66% (2/5) | 996.4 / 1024.9 / 979.0 |

The original direct view reached 9.72× M5 CPU and 3.46× M5 Metal. 0/5 process means reached 28.4 ms. All 83 proof exports verified. Separate probes exercised the upper compact route, the dense route and a zero preparation allowance. [Samples](artifacts/samples.json.gz) · [Route probes](artifacts/probes.json) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)
