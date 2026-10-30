# Witness/query readiness in solver program order

30 October 2026. This structural census changes no prover code and produces no timing samples or proofs. The exact accepted libraries export two existing solver programs; a separate Rust reader accounts for their wire producers and final solver reads.

Halfway through primary 1×2's serialized program, roughly half of A/B query operands are defined, but only about **14%** form a contiguous prefix in key order. At 90% of operations, roughly 90% are defined but the ordered prefix covers only **32–33%**. For 2×2, the ordered prefix covers about **10%** at halfway and **19–20%** at 90%. This limits the assumption that an ordinary key-order stream could consume most operands early. It does not measure a latency loss or rule out out-of-order streaming.

The prerequisite follows the [qualified stage timeline](../2026-10-30-query-timeline175/README.md), which measured a 1.864279 ms median native prefix through product completion at 32 workers. Existing [fused product overlap](../2026-10-26-fused-product-overlap/README.md) regressed, and [pressure-admitted overlap](../2026-10-23-pressure-admitted-product-overlap/README.md) was marginal. Neither unchanged mechanism is retested here. The [exact FMA arithmetic trial](../2026-10-23-exact-fma-rne-component/README.md) also remains closed.

## Structural results

Program percentages count serialized operations, not CPU time, dependency depth, parallel epochs or elapsed readiness. Readiness is after an operation completes. Query indices are strictly increasing, matching the retained image reader. A contiguous prefix stops at the first undefined query wire. Values past their final solver read may still be needed by public correction and proving; this is not permission to mutate Montgomery witness storage early.

|Case / query|Points|Program %|Ready in any order|Ready key-order prefix|Past final solver read|
|---|---:|---:|---:|---:|---:|
|1x2/A|21006|10|2278 (10.8%)|183 (0.9%)|1322|
|1x2/A|21006|50|10621 (50.6%)|2896 (13.8%)|10388|
|1x2/A|21006|90|18930 (90.1%)|6722 (32.0%)|18891|
|1x2/A|21006|100|21006 (100.0%)|21006 (100.0%)|21006|
|1x2/B|30089|10|2802 (9.3%)|206 (0.7%)|1844|
|1x2/B|30089|50|14573 (48.4%)|4248 (14.1%)|14342|
|1x2/B|30089|90|26991 (89.7%)|9964 (33.1%)|26956|
|1x2/B|30089|100|30089 (100.0%)|30089 (100.0%)|30089|
|2x2/A|36637|10|3921 (10.7%)|288 (0.8%)|2027|
|2x2/A|36637|50|18548 (50.6%)|3563 (9.7%)|18135|
|2x2/A|36637|90|33024 (90.1%)|7119 (19.4%)|32852|
|2x2/A|36637|100|36637 (100.0%)|36637 (100.0%)|36637|
|2x2/B|52274|10|4685 (9.0%)|316 (0.6%)|2781|
|2x2/B|52274|50|25256 (48.3%)|5209 (10.0%)|24848|
|2x2/B|52274|90|46883 (89.7%)|10521 (20.1%)|46719|
|2x2/B|52274|100|52274 (100.0%)|52274 (100.0%)|52274|

All eleven cuts, including the initial input-ready state, are retained in analysis.json. H is excluded because it requires the product; K's selected folded-slot membership is not reconstructed or claimed.

The declared rejection condition was an ordered prefix that had not advanced beyond its initially known input terms by 90% of operations. **That condition is false for all four query populations.** This census therefore does not reject every ordered-stream design. It quantifies a substantial difference between arbitrary readiness and sequential key consumption. No implementation or performance eligibility follows automatically.

A useful next design would need actual parallel-schedule readiness, the cost of index/gather or reordered consumption, preservation of scalar grouping, the G2 high/low component order, and workspace admission. An overlap mechanism alone would reorder work rather than remove the dominant field arithmetic. These counts do not establish a path to a twofold CPU improvement.

## Validation and reproducibility

All seven workflow steps pass once: link the test harness, run three structural tests, link the census, then export/analyze primary and 2×2. Tests cover hints, within-constraint stored slots, last-read and readiness frontiers; duplicate writes, undefined reads, truncated input, unsorted queries and trailing data are rejected. The producer-index aggregation and independently maintained online query frontier agree at every cut. They share one structural parser; this is not an independent field-arithmetic oracle.

Every value is defined once, all consumed wires are defined, all constraints are accounted for, and the complete version2 image is consumed. Primary has 30,521 wires, 31,061 operations and 30,922 constraints; 2×2 has 53,064 wires, 54,083 operations and 53,874 constraints. Full dimensions and program hashes are retained. Private solver exports remain local; their contents are circuit metadata, and no witness values are collected.

The helper links the same eight coherent native libraries as accepted SDK SHA-256 `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f`. All original input/helper/library pins and all 175 production source hashes close. Existing field/core/prover qualification is inherited; no prover library or field suite is rebuilt or repeated. Disposable analysis arrays are not a production execution plan or a prover memory measurement.

There are **zero new witness solves, SDK requests, Gnark exports or timing samples**. The distinct-proof ledger stays **25,231**. Accepted local primary remains **8.022165 ms / 134 MiB**; all nine local/AWS rows and eight supplied CPU/Metal reference rows remain unchanged, with **0/8 CPU 10× targets achieved**. No new AWS/Aeglos, mobile/ARM/TEE, pressure/swap, huge-circuit or concurrency result is claimed. [Full accepted comparisons](../2026-10-30-current-bench-report/README.md), [exact accepted rows](accepted-rows.json).

[Analysis](artifacts/analysis.json), [protocol](artifacts/design.md), [reader](artifacts/model.rs), [consumer](artifacts/census.rs), [workflow](artifacts/plan.json), [pins](artifacts/pins.json), [review](artifacts/review.json), [test/export/analysis receipts](artifacts/receipts.json.gz), [exact pre-census summary](artifacts/inherited-summary.json.gz), [archive review](artifacts/publication-review.json).
