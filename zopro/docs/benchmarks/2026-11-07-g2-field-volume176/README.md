# Accepted G2 field-operation accounting

Ordinary streaming bucket updates account for 83.69–84.49% of the counted vector-work proxy for the retained primary G2 query, and 81.12–81.96% for the retained grouped 2×2 query. This prioritizes the main accumulation path over another reduction-tree-only arithmetic change. These percentages are not CPU-time shares or predicted proof improvements. No runtime candidate is selected or adopted.

This is new offline accounting of the saved [operation census](../2026-11-05-g2-operation-census176/README.md), using its original OFF cases. No native query, scalar scan, proof, verifier, timing or profile was rerun. The census correctness/observer qualification is inherited. The analyzer checks all 176 current accepted source hashes, the inherited review flags, agreement between one- and 32-worker aggregate counts, and per-component histogram conservation. Its compile and analysis workflow commands passed on their first executions.

## Units and scope

VM means a logical eight-lane ordinary base-field product; VF means a logical eight-lane fused two-product operation. SM, SS and SF are scalar base-field product, square and fused product counts; I is a scalar inversion. The vector-only proxy is `100 VM + 150 VF`, using nominal IFMA instruction sites. Scalar work, inversion, correction, conversion, memory traffic, dependencies and scheduling are not priced in that proxy.

Accepted signed borrowed prefix arithmetic costs 7 VM + 5 VF per complete eight-pair block. Its scalar tail costs 8 SM + 2 SS + 4 SF per pair. The generic reduction tail instead costs 7 SM + 5 SF per pair. Each batch or reduction layer has an inverse bridge costing 5 VM + 1 SM + 1 I. Accepted x86 scalar Fq2 multiplication uses two scalar fused products; it is not counted as three ordinary products.

Reduction prefix blocks cost 7 VM + 5 VF; captured-record blocks cost 8 VM + 5 VF. The census does not preserve each layer's generic-pair count, so the exact mode split is unknown. The table bounds the additional VM by `min(total blocks, 3 × layers)`, since captured mode requires fewer than 32 generic pairs. The two ends need not each describe an attainable distribution; they are conservative bounds.

|Query and path|VM|VF|SM|SS|SF|I|
|---|---:|---:|---:|---:|---:|---:|
|1×2 streaming|464,812|322,705|3,789|296|592|2,605|
|1×2 reduction|94,274–104,931|53,285|72,801|0|49,190|3,935|
|2×2 streaming|481,628|334,420|3,904|304|608|2,688|
|2×2 reduction|115,205–127,607|67,525|71,131|0|47,855|4,134|

Primary streaming has 516,476 ordinary affine pairs, including 148 scalar-tail pairs. Reduction has 95,094 generic pairs, including 9,838 scalar-tail pairs. Both original cases have zero layers with no generic pairs; skipping empty inversions supplies no opportunity on these inputs. Streaming's larger vector volume does not make the reduction scalar tails or inversions free.

This is an incomplete whole-query field ledger. It excludes projective spill additions and their normalization, weighted reconstruction, identity/equal-x pair slots, indexed preparation, additions/subtractions, packing, allocation and synchronization. The primary census records 749 additions to existing projective accumulators, 429 normalized projective points and 15 identity/equal-x reduction slots; their complete branch costs are not reconstructed here. Do not combine this ledger with another census's totals without reconciling fixture and stage scope. There is no new RSS, minimum-RAM, mobile, pressure, large-circuit, AWS or Aeglos result.

## Decision and evidence

Prioritize a distinct reduction in ordinary main-MSM accumulation work. This accounting does not justify reopening the closed bucket ownership, ring/hexagonal recoding, width, layout or worker experiments. The [real-norm common-scale screen](../2026-11-07-g2-real-norm-z176/README.md) remains closed without native integration; its inversion savings do not remove the larger ordinary streaming population.

The accepted full-proof implementation is unchanged. Latest matched local 32-worker 1×2 remains **7.915095 ms**, versus supplied M5 CPU **37 ms** and Metal **24 ms**. The all-eight verified CPU 10× goal remains active and unmet, **0/8**, with the full inherited shape table in the linked real-norm report. The distinct-proof ledger remains 31,387. These offline counts add zero proofs or performance observations.

See [results](artifacts/ledger.json), [analyzer](artifacts/ledger.rs), [declared question](artifacts/design.md), [input pins](artifacts/input-pins.json) and [workflow](artifacts/plan.json). Input hashes identify the existing local census/review; those original public artifacts are in the linked census report. Local absolute paths in the workflow require adaptation to reproduce elsewhere. Receipts preserve the exact command, exit status and stream hashes. No binary, library, proving key, witness or proof payload is published.
