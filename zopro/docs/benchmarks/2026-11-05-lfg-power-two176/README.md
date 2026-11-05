# Power-of-two LFG: algebra and storage screen

The literal shifted-point-table approach in the new LFG paper is unsuitable for this prover's universal memory requirements. At width 12, its two-multiplier construction needs approximately **218.02 MiB of raw affine A/B table payload for primary and 3.79 GiB for 24×2**, before other proving data or workspace. No such table is built. Production and accepted performance remain unchanged.

## Paper and scope

[Guiwen Luo, ePrint 2026/2750](https://eprint.iacr.org/2026/2750), published November 3, gives binary-radix bucket sets for multiplier sets {1,3} and {1,2,3}, with sizes q/4+2 and ceil(q/6)+floor(c/2). The LFG online bound uses precomputed m·q^j·P for every point, multiplier and scalar window. It does not follow from storing only a small scalar-independent digit table. The full 18-page PDF, including the appendix, was read from [the PDF mirror](https://eprint.iacr.asia/2026/2750.pdf); original/browser fetches failed, while shell mirror retrieval succeeded. Source hashes and read scope are retained.

Our separate Rust model checks the constructions and derives a table-free two-multiplier digit converter using inverse3 modulo2^c. It does not copy an upstream implementation or modify field/curve code. The model is an algebra/storage screen, not a native MSM, BN254 speed measurement or proof implementation.

## Exact model validation

Three tests pass, unfiltered, on their first execution. Both bucket sets cover every signed residue for widths4–18; all30 cardinalities and gaps agree with independently enumerated sets, including the appendix gaps for widths10–18. Every updated digit in [-1,q+1] is checked for exact signed decomposition, allowed bucket/multiplier and carry. Full scalar tests cover powers/neighbors, scalar-modulus boundaries and16,384 deterministic random254-bit integers per width10/12/14. **51,453 exact reconstructions** use a separate384-bit two's-complement accumulator. Additional small additive-group checks are integer oracles, not elliptic-curve proofs. No q-entry digit lookup table is needed for this model.

The four-step pinned workflow compiles/runs tests and compiles/runs the model once. Independent closeout checks all pins, four successful command receipts/stream hashes,30 independently rebuilt sets,16 independently recomputed query payloads, and all176 production source files unchanged. Existing field/core/prover qualification is inherited; no native prover suite or proof is rerun.

## Literal point-table payload

For this explicit comparison, scalar bits=254,width12 gives h=22. StrategyI uses2nh or3nh point records; StrategyII addsn. Stored counts include the originalnpoints. Affine records use64bytes for A/G1 and128bytes for B/G2. All8real A/B sizes come from the preceding [grouping census](../2026-11-05-chunk-grouping176/README.md). This applies the paper's formulas directly; it does not assume composition with our GLS split or proof-local grouping. Grouped points vary with each witness and are not fixed key points available at load time.

|Shape|Original A+B point payload MiB|M13 StrategyI MiB|M123 StrategyI MiB|
|---|---:|---:|---:|
|1x2|4.955|218.023|327.035|
|2x2|8.617|379.159|568.738|
|4x2|15.761|693.483|1040.225|
|8x2|30.259|1331.381|1997.072|
|12x2|44.693|1966.486|2949.729|
|16x2|59.157|2602.923|3904.385|
|24x2|88.271|3883.918|5825.878|
|merge-24x1|83.924|3692.654|5538.981|

These are uncompressed point payload formulas, not measured allocations, process peaks, minimum RAM or a complete key image. They exclude metadata, alignment, allocator overhead, base key/other queries, stacks and proving workspace. StrategyII adds one original A+B payload column to either StrategyI result. Compressing or paging tables would introduce costs not measured here. No table/cgroup/swap experiment is performed.

## Bounded-memory alternative: unresolved costs

Generating only a tile's small multiples avoids retaining all shifted points, but it requires computing the multiples and returning to per-window Horner/reduction. In particular it loses the literal algorithm's one shared weighted reduction across all pre-shifted windows. Any such alternative must account for formation, nonconsecutive bucket weights, carry terms, preserving global scalar grouping, task parallelism and every simultaneous live state. It cannot inherit the paper's online-addition bound or a CPU gain without a separate actual-caller experiment. This study does not implement or qualify that variant and does not reject all possible low-memory uses of these sets.

Prior raised grouping-cutoff, sparse grouping, fixed2P/3P, hexagonal/binary-ring and local-coalescing trials remain closed. The new paper's binary construction is new evidence; it does not authorize repeating those implementations unchanged. The literal large retained table route is rejected by architecture. No next runtime candidate or twofold performance path is established.

## Accepted CPU and Metal comparisons, inherited

No performance timing is captured here. Accepted SDK SHA-256 remains `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. All eight CPU10× targets remain unmet.

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.928660|37 / 4.667x|24 / 3.027x|3.7|
|2x2|11.151932|59 / 5.291x|30 / 2.690x|5.9|
|4x2|16.086386|109 / 6.776x|48 / 2.984x|10.9|
|8x2|24.179453|189 / 7.817x|74 / 3.060x|18.9|
|12x2|33.884455|284 / 8.381x|101 / 2.981x|28.4|
|16x2|40.077381|335 / 8.359x|124 / 3.094x|33.5|
|24x2|65.850777|528 / 8.018x|176 / 2.673x|52.8|
|merge-24x1|65.060937|507 / 7.793x|175 / 2.690x|50.7|

Merge54 remains161.883899ms without a supplied M5 reference. Latest full-SDK latency evidence remains epoch-clock176. The independent-proof ledger remains29,461; no fresh SDK/Gnark proof, AWS/Aeglos, device, throughput or pressure result is added. Goal all-eight CPU10× remains active.

## Artifacts

[Design](artifacts/design.md), [source/applicability notes](artifacts/research.md), [model](artifacts/model.rs), [all results](artifacts/analysis.json), [independent reviewer](artifacts/review.rs), [review](artifacts/review.json), [source provenance](artifacts/source-provenance.json), [real input counts](artifacts/input-counts.json), [protocol](artifacts/plan.json), [pins](artifacts/pins.json), [raw command receipts](artifacts/receipts.json.gz), [publisher](artifacts/publish.rs), [checksums](SHA256SUMS). Receipt JSON gzip roundtrips exactly; compiled tools, full paper/text, private inputs, keys, points, witnesses and libraries are excluded.
