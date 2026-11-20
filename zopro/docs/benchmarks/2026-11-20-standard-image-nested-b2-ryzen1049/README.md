# Nested G2 grouping with the standard 12×2 image

The trial attaches the archived nested G2 group index to the current SDK and ordinary Euler key. It keeps the original FFT inputs and G1 queries. It combines points once when a key is loaded, chooses a scalar representative per group on each proof, and adds exact corrections for unequal members. Proofs that exceed the mismatch or scratch allowance fall back to the ordinary G2 query at the affected level.

The two-level arm met the local paired criterion on catalog, heldout, v58, v139. The experimental source remains in the trial artifact.

| Input | Frozen / control / first level / two levels ms | Two levels vs control | Two levels vs frozen | Control / two levels peak MiB |
|---|---:|---:|---:|---:|
| catalog | 32.214 / 32.215 / 32.036 / 31.097 | -3.40% (6/6) | -3.63% (6/6) | 507.3 / 498.3 |
| heldout | 32.111 / 32.433 / 32.002 / 31.123 | -4.05% (6/6) | -2.80% (6/6) | 507.0 / 487.3 |
| v58 | 32.016 / 32.302 / 31.888 / 31.153 | -3.06% (6/6) | -2.75% (6/6) | 507.1 / 509.8 |
| v139 | 46.460 / 46.483 / 46.390 / 46.184 | -0.79% (6/6) | -0.47% (6/6) | 729.4 / 652.0 |

The probes confirmed 270,757 G2 terms reduced to 148,668 after the first level, then 43,189 after the second level on the catalog, held-out, and v58 inputs before mismatch corrections. The v139 input declined the second level. Under a zero optional preparation budget all four inputs declined group preparation and produced a valid proof. The two levels reserve about 25.9 MiB for retained point sums and indices. Additional scratch is admitted separately.

Ryzen 9950X with 24 main and eight frontend logical workers. Six order-balanced blocks warm three proofs and measure thirteen in each process. The table gives medians of process means and maximum whole-process RSS. The local criterion requires at least five of six paired wins and a lower paired median against both controls, with same-build control calibration within 2% of the frozen SDK. The summary also contains first-level comparisons. The first build attempt exercised only the fallback because the 12×2 circuit uses the large-query SDK path. Its timings are excluded.

The native arithmetic test checks exact sums and modular mismatch corrections. Gnark independently reverified all 108 exports in the final run against the original standard image and expected public inputs.

[Summary](artifacts/summary.json) · [Samples and route probes](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial patch](artifacts/source.patch.gz) · [Archived profile decomposition](../2026-11-20-indexed-12x2-provenance-ryzen1048/README.md)
