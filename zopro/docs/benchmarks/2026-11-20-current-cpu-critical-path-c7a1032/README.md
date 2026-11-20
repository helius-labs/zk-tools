# Current CPU proof phases on 96-core C7a

This run traces the pinned 1×2, 2×2, 4×2, and 8×2 proof profiles with the same catalog and held-out requests as their timing panels. Plain runs use the same executable bytes on a separate inode. The C7a host has 96 physical cores. The phase clocks show elapsed intervals, including any waiting or work Rayon executes inside the traced calls.

Plain medians were 7.729 / 7.639 ms for 1×2, 10.338 / 10.352 ms for 2×2, 13.910 / 14.029 ms for 4×2, and 18.811 / 18.891 ms for 8×2. The 8×2 medians both clear the 18.9 ms M5 CPU target on this host. The overall 10× score remains five of eight CPU references and zero of eight Metal references.

The full probe set raised paired latency by 9.43–28.81%. Use plain runs for performance claims. The traces show that the 1×2 held-out G2 query finished about 1.46 ms after the last observed G1 call, while the catalog requests finished close together. On 2×2, G2 finished about 1.22–1.24 ms before the core returned. These are leads for a lower-overhead timing check. Both grouped profiles lack a unique observed G2 span.

| Shape | Request | Plain ms | Traced ms | Paired trace change | CPU ms per proof | Peak plain MiB | 10× M5 CPU / Metal target ms |
|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | 7.729 | 9.948 | +28.81% | 327.0 | 40.8 | 3.7 / 2.4 |
| 1×2 | heldout | 7.639 | 9.323 | +22.46% | 323.2 | 40.1 | 3.7 / 2.4 |
| 2×2 | catalog | 10.338 | 11.314 | +9.43% | 342.1 | 76.8 | 5.9 / 3 |
| 2×2 | heldout | 10.352 | 11.449 | +10.75% | 343.4 | 71.5 | 5.9 / 3 |
| 4×2 | catalog | 13.910 | 16.682 | +19.61% | 660.1 | 187.7 | 10.9 / 4.8 |
| 4×2 | heldout | 14.029 | 16.622 | +19.12% | 661.2 | 194.4 | 10.9 / 4.8 |
| 8×2 | catalog | 18.811 | 21.274 | +12.14% | 1032.2 | 337.3 | 18.9 / 7.4 |
| 8×2 | heldout | 18.891 | 21.292 | +12.71% | 1045.1 | 342.6 | 18.9 / 7.4 |

## Sequential wall intervals

| Shape | Request | Before solver | Solver | Solve → FFT | Quotient FFT | FFT → core return | Assembly and decode | Verification before final exp. | Final exp. | SDK return |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | 0.513 | 1.912 | 0.396 | 1.023 | 4.411 | 0.231 | 0.275 | 0.222 | 0.018 |
| 1×2 | heldout | 0.521 | 1.792 | 0.406 | 1.050 | 3.936 | 0.225 | 0.281 | 0.225 | 0.019 |
| 2×2 | catalog | 0.573 | 2.392 | 0.315 | 1.304 | 5.715 | 0.222 | 0.282 | 0.216 | 0.019 |
| 2×2 | heldout | 0.570 | 2.230 | 0.317 | 1.215 | 5.781 | 0.224 | 0.281 | 0.217 | 0.018 |
| 4×2 | catalog | 0.556 | 2.651 | 1.142 | 2.545 | 8.495 | 0.294 | 0.310 | 0.322 | 0.020 |
| 4×2 | heldout | 0.576 | 2.684 | 1.076 | 2.269 | 8.423 | 0.291 | 0.312 | 0.331 | 0.021 |
| 8×2 | catalog | 0.858 | 3.271 | 1.047 | 2.922 | 11.426 | 0.317 | 0.339 | 0.254 | 0.023 |
| 8×2 | heldout | 0.884 | 3.257 | 1.019 | 2.874 | 11.422 | 0.320 | 0.345 | 0.259 | 0.023 |

Each proof is partitioned along the enclosing calls. The table shows per-request medians in ms, so the columns can sum to a slightly different value from median total time. FFT-to-core time can include partially overlapped G2 work.

| Shape | Request | G2 start | G2 elapsed | FFT and G2 overlap | G2 return → core return | G2 windows | G2 pairing prep | Miller loop after generation |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | 2.901 | 5.107 | 0.993 | 0.209 | 4.796 | 0.069 | 0.174 |
| 1×2 | heldout | 2.764 | 4.604 | 1.020 | 0.191 | 4.322 | 0.071 | 0.176 |
| 2×2 | catalog | 3.385 | 5.909 | 1.275 | 1.236 | 5.690 | 0.069 | 0.172 |
| 2×2 | heldout | 3.354 | 5.877 | 1.188 | 1.222 | 5.664 | 0.069 | 0.171 |
| 4×2 | catalog | — | — | — | — | — | 0.078 | 0.193 |
| 4×2 | heldout | — | — | — | — | — | 0.079 | 0.202 |
| 8×2 | catalog | — | — | — | — | — | 0.087 | 0.213 |
| 8×2 | heldout | — | — | — | — | — | 0.087 | 0.217 |

G2 figures cover the unique enclosing query when the profiler can identify one for that proof. Dashes mark intervals without a unique observation. The JSON records coverage and individual nested spans. Pairing and FFT subintervals overlap their enclosing phases.

Four matched blocks prime three proofs and time eleven. Gnark independently reverified all 64 exports. The profiler captured 352 warm proof timelines with zero lost records. The temporary instance, storage and bucket were removed.

[All eight supplied M5 comparisons](../2026-11-20-geometry-placement-c7a1027/README.md#all-eight-supplied-m5-references) · [Summary](artifacts/summary.json) · [Spans and samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)
