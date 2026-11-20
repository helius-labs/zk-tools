# Bounded K/H bucket bands on Ryzen

On 2×8, a single shared bucket set reduced peak RSS from **217.0 to 151.5 MiB** on the primary request and from **202.0 to 146.2 MiB** on the held-out request. Paired latency fell **1.32% and 2.08%** with six of six wins on each request against the same build. Against the pinned earlier build, the reductions were 2.08% and 1.43%, with four of six and six of six wins. The qualification rule requires five of six wins on each request against both controls. The trial stays isolated.

The same one-set variant slowed 4×2 by 3.37–5.19% and 8×2 by 8.66–9.67%. Splitting into two or four bands slowed every tested request. The largest relative peak reduction was 214.2 to 146.2 MiB on the 4×2 held-out request with two bands. Its paired latency rose 33.11%.

The one-set variant streams both queries into a single signed bucket array per window. The two-band and four-band variants reserve disjoint ranges of the same bucket domain. Each range scans the existing scalars and writes its own buckets. A single weighted reduction then uses the original bucket positions. The trial has no key tables or per-point index buffers. With 24 main workers, the requested scratch allowances were 38.1, 27.1 and 18.5 MiB for one, two and four ranges respectively. Admission failure uses the original separate sums.

| Shape | Input | Profile | Proof ms | Paired vs same build | Wins | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 2×8 | catalog | Pinned earlier build | 14.277 | +0.14% | 2/6 | 206.7 |
| 2×8 | catalog | Same build, separate | 14.257 | — | — | 217.0 |
| 2×8 | catalog | One shared set | 14.100 | -1.32% | 6/6 | 151.5 |
| 2×8 | catalog | Two bucket bands | 15.256 | +6.74% | 0/6 | 163.0 |
| 2×8 | catalog | Four bucket bands | 21.147 | +47.56% | 0/6 | 153.0 |
| 2×8 | heldout | Pinned earlier build | 14.326 | -0.58% | 5/6 | 210.2 |
| 2×8 | heldout | Same build, separate | 14.399 | — | — | 202.0 |
| 2×8 | heldout | One shared set | 14.081 | -2.08% | 6/6 | 146.2 |
| 2×8 | heldout | Two bucket bands | 15.253 | +5.82% | 0/6 | 158.0 |
| 2×8 | heldout | Four bucket bands | 21.115 | +46.31% | 0/6 | 148.8 |
| 4×2 | catalog | Pinned earlier build | 15.215 | -0.47% | 4/6 | 212.5 |
| 4×2 | catalog | Same build, separate | 15.292 | — | — | 220.2 |
| 4×2 | catalog | One shared set | 16.072 | +5.19% | 0/6 | 164.8 |
| 4×2 | catalog | Two bucket bands | 20.489 | +34.06% | 0/6 | 150.5 |
| 4×2 | catalog | Four bucket bands | 29.604 | +94.02% | 0/6 | 181.8 |
| 4×2 | heldout | Pinned earlier build | 15.169 | +0.22% | 2/6 | 221.2 |
| 4×2 | heldout | Same build, separate | 15.183 | — | — | 214.2 |
| 4×2 | heldout | One shared set | 15.758 | +3.37% | 0/6 | 164.5 |
| 4×2 | heldout | Two bucket bands | 20.097 | +33.11% | 0/6 | 146.2 |
| 4×2 | heldout | Four bucket bands | 29.061 | +90.94% | 0/6 | 148.2 |
| 8×2 | catalog | Pinned earlier build | 24.521 | -0.05% | 3/6 | 392.8 |
| 8×2 | catalog | Same build, separate | 24.599 | — | — | 371.5 |
| 8×2 | catalog | One shared set | 26.976 | +9.67% | 0/6 | 285.0 |
| 8×2 | catalog | Two bucket bands | 36.117 | +46.63% | 0/6 | 273.0 |
| 8×2 | catalog | Four bucket bands | 55.554 | +125.96% | 0/6 | 271.0 |
| 8×2 | heldout | Pinned earlier build | 24.887 | -0.16% | 3/6 | 386.0 |
| 8×2 | heldout | Same build, separate | 24.965 | — | — | 372.5 |
| 8×2 | heldout | One shared set | 27.037 | +8.66% | 0/6 | 284.0 |
| 8×2 | heldout | Two bucket bands | 36.153 | +44.82% | 0/6 | 281.0 |
| 8×2 | heldout | Four bucket bands | 55.480 | +120.98% | 0/6 | 276.0 |

Six order-balanced blocks per shape and input used three warmups and thirteen timed proofs per process. The table reports medians of block means and maxima of whole-process RSS. The earlier executables and inputs are pinned to the corresponding C7a profiles. This run uses the Ryzen host with 24 main and eight frontend logical workers. The fused variants stream raw K scalars and skip the query-specific grouping available to the controls. Timing captures the full change. [C7a and M5 reference timings](../2026-11-20-geometry-placement-c7a1027/README.md#all-eight-supplied-m5-references)

A native arithmetic test exercised zero and signed scalars, identity and repeated points, and one and eight Rayon workers. Diagnostic runs confirmed all three fusion paths for both requests on each shape. Six zero-scratch runs confirmed fallback. Gnark independently reverified all 216 run exports and 60 preflight exports. The first preflight exposed that 2×8 uses the canonical-wire route, which the trial did not yet cover. The timing panel uses the corrected build.

[Paired data](artifacts/summary.json) · [Samples and route checks](artifacts/rows.json.gz) · [First preflight](artifacts/discovery-rows.json.gz) · [Protocol](artifacts/protocol.json) · [Workflow](artifacts/workflow.json.gz) · [Trial patch](artifacts/source.patch.gz)
