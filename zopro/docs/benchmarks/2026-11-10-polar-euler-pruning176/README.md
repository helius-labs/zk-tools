# Polarized Euler joint-pruning screen

**The fixed paired-layout candidate closes before native implementation.** Exact polarization and joint FFT input/output pruning reduce scalar product sites, but remove **zero paired root-product sites** on both tested circuits. No timing, proof or production change follows.

With E(f)=Xf′, S=A+B and D=A−B,

`E(A)B + A E(B) = (S E(S) − D E(D))/2`.

About 59% of constraint rows have the exact formal identity A=B, so D vanishes there. This identity keeps both derivatives: equal values at a domain point do not imply equal derivatives. The current native Euler operator pairs four A rows and four B rows into eight lanes sharing roots. This study keeps that arrangement, replacing the channels with S and D.

The new reader derives ordered support masks from cached raw solver images for 1x2 and 2x2. It expands stored prefix sums, includes assigned terms, and combines coefficients exactly over Fr with original circuit wires treated as independent variables. It consumes every byte and covers every row, sum and slot. It neither solves witnesses nor infers nonlinear identities. The prior category histograms could not supply row order; their census was not replayed.

|Shape|N|Real rows|Formal D support|A/B/S support each|
|---|---:|---:|---:|---:|
|1x2|32768|30922|12392|30922|
|2x2|65536|53874|21746|53874|

Forward known-zero propagation and backward output demand are applied together to inverse DIF → bit-reversed coefficient weights → forward DIT. Baseline derivative demand is support(B), support(A); polarized demand is support(S), support(D). Padded rows are zero; internal butterfly cancellation is not assumed. Multiplications by one are excluded.

|Shape|Scalar root sites, baseline → polarized|Scalar reduction|Paired root sites, baseline → polarized|Four-site groups, baseline → polarized|
|---|---:|---:|---:|---:|
|1x2|851972 → 832578|2.276%|425986 → 425986|114688 → 114688|
|2x2|1835012 → 1787082|2.612%|917506 → 917506|245760 → 245760|

All nonunit coefficient weights remain live in every channel: 32,766 for 1x2 and 65,534 for 2x2. The dense S channel retains every root-product site needed by the pair; D-only zeros cannot eliminate those shared operations. Four-site grouping is an additional occupancy screen, not a claim about the production kernel's precise packing.

These are logical dependency/product-site counts, **not measured SIMD instructions, cycles, IFMAs or latency**. Production fuses stages and uses special small-root operations. The fixed admission rule required strictly fewer paired sites and groups on both fixtures; both decisions fail. Preparation of S and D, masks, scaling by 1/2, normalization, branches and memory were optimistically unpriced. A different layout, algebraic cancellation scheme or fused kernel could behave differently; this screen proves no general lower bound. Offline expanded forms and graph buffers are analysis allocations, not proposed production solver structures or a RAM bound.

Exact qualification passes **288 Arkworks Fr cases** at N=8,16,32,64: **34560 full derivative outputs**, **17246 arbitrarily demanded outputs**, and **8640 complete polarized rows**. A separately constructed direct inverse DFT and coefficient-weighted evaluation supplies N·E(f), matching the production operator's unnormalized convention. Zero, equal, opposite, one-zero, prefix-sparse, alternating, irregular and dense inputs are included. Complete baseline and polarized expressions also agree with pruned evaluation.

A separate same-agent recursive dependency reader matches the iterative graph for **all 65,536 N=8 input-support/output-demand pairs**. It passes **2,048 outputs modulo 97** against its own direct DFT and recomputes scalar, paired and group counts and both decisions from the published masks. This independently implements graph traversal; it does not independently parse the raw images, qualify native code or constitute an external audit.

All four final serialized Rust 1.98.1/xtask commands pass; 193 pins and 176 production source hashes remain unchanged. Two prior compile failures are retained, for a JSON macro expression and a missing AdditiveGroup import. Three unused-code warnings from including the graph in the reader are retained. Exact argv, exit status and stdout/stderr hashes are checked, including failures. No successful qualification or previous performance capture was rerun.

Accepted local 32-worker full-proof observations and supplied historical M5 references remain unchanged:

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.829286|37 / 4.726×|24 / 3.065×|3.7|
|2x2|10.979335|59 / 5.374×|30 / 2.732×|5.9|
|4x2|15.803969|109 / 6.897×|48 / 3.037×|10.9|
|8x2|23.944017|189 / 7.893×|74 / 3.091×|18.9|
|12x2|33.703579|284 / 8.426×|101 / 2.997×|28.4|
|16x2|39.858478|335 / 8.405×|124 / 3.111×|33.5|
|24x2|66.186709|528 / 7.977×|176 / 2.659×|52.8|
|merge-24x1|65.489902|507 / 7.742×|175 / 2.672×|50.7|

Primary remains **7.829286 ms / 133.50 MiB**. Earlier 7.803404 ms used the same executable; primary at 16 workers remains 9.192237 ms. Merge 54 remains 162.541995 ms without a supplied M5 reference. Accepted source/SDK and the canonical 43,730 verified proofs are unchanged. CPU AVX ≥10× M5 CPU on all eight shapes remains **active and unmet, 0/8**. No fresh AWS/Aeglos, phone/ARM, huge-circuit, pressure/swap, minimum-RAM or concurrent-throughput result is added.

[Analysis](artifacts/analysis.json), [recursive reader](artifacts/review.json), [ordered masks](artifacts/masks.json), [decision](artifacts/decision.json), [fixed design](artifacts/design.md), [continuation and autoresearch loop](artifacts/next.md), [sources and exact receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).

Private experiment root: `/home/ubuntu/src/zopro-perf-work/polar-euler-pruning176`.
