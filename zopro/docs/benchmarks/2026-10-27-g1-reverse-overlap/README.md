# Overlapping G1 streaming reverse groups

Component advancement: **true**. Accepted production remains unchanged; no full-proof latency gain is claimed. The all-eight CPU10× objective remains active and unmet.

The targeted signed-batch aggregate improves by **0.817–0.818%** against both identical-code controls, with **7/7** winning block aggregates. All 40 cases pass control calibration and the 2% nonregression limit. This supports measuring the implementation in complete proofs; no minimum gain cutoff is applied.

The candidate exposes the next group's slope before finishing the current group's point outputs in the actual G1 streaming reverse loop. It processes two consecutive groups in the original descending order and preserves the single inverse-chain order, all field formulas, forward records, scalar tails, output order, signed/canonical guards and scratch capacities. G2 and affine trees are unchanged. There is no second inverse chain, dual-CIOS helper, table, cache, execution plan, point copy, heap storage or annotation. Extra live vector temporaries may affect registers and stack. This tests instruction overlap rather than fewer mathematical additions.

## Complete-batch caller

Ryzen9950X, CPU2, one serial thread, no Rayon pool. Forty cases: batch sizes8/9/16/17/24/25/200/203/352/355, unsigned/signed and two seed families. Each batch has unique scattered destinations. Repeated calls update real buckets, so denominators vary across calls. Independent expected outputs are initial+iterations×signed-input, including untouched buckets. Exact accepted copy controlsA/B and candidate execute in one native fat-LTO binary. Seven rotated/reversed paired blocks, three primes then reset,1024timed complete calls per stage. One timer brackets the entire stage; setup, reset, expected-output construction and checks are excluded. Mutable state is blackboxed per call. All modes use prewarmed scratch and identical fixture addresses. Timed allocations are zero.

|Batch|Signed|Seed|A µs/batch|B µs/batch|Overlap µs/batch|Overlap/A|Overlap/B|Wins/A|Wins/B|
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
|8|false|0|0.754|0.755|0.749|0.994935|0.993317|6|5|
|9|false|0|0.859|0.858|0.856|0.992618|0.994972|6|7|
|16|false|0|0.888|0.886|0.894|1.007962|1.006192|0|1|
|17|false|0|1.001|1.002|1.000|0.997499|1.000219|6|6|
|24|false|0|1.021|1.021|1.036|1.014069|1.015137|0|0|
|25|false|0|1.132|1.131|1.136|1.004574|1.003061|1|1|
|200|false|0|3.964|3.961|4.009|1.011226|1.011433|0|0|
|203|false|0|4.259|4.258|4.289|1.006658|1.007223|0|0|
|352|false|0|6.518|6.517|6.612|1.014221|1.015201|0|0|
|355|false|0|6.873|6.873|6.934|1.008846|1.008977|0|0|
|8|true|0|0.773|0.770|0.752|0.970919|0.973551|7|7|
|9|true|0|0.888|0.888|0.865|0.976836|0.976012|6|6|
|16|true|0|0.914|0.916|0.905|0.990622|0.989173|7|6|
|17|true|0|1.032|1.031|1.022|0.986979|0.989245|7|7|
|24|true|0|1.051|1.051|1.036|0.985484|0.983496|7|7|
|25|true|0|1.180|1.178|1.148|0.973886|0.976107|7|7|
|200|true|0|4.056|4.055|4.051|0.999006|0.998960|6|6|
|203|true|0|4.412|4.411|4.326|0.980544|0.980357|7|7|
|352|true|0|6.687|6.685|6.665|0.996280|0.996389|7|7|
|355|true|0|7.016|7.011|6.951|0.990817|0.991326|7|7|
|8|false|1|0.756|0.758|0.750|0.985173|0.987153|7|7|
|9|false|1|0.858|0.857|0.849|0.988456|0.985959|7|7|
|16|false|1|0.886|0.885|0.894|1.010397|1.012181|0|0|
|17|false|1|0.997|0.998|1.010|1.009941|1.012897|1|0|
|24|false|1|1.020|1.020|1.022|1.003556|1.001087|0|2|
|25|false|1|1.134|1.136|1.122|0.989910|0.989553|7|7|
|200|false|1|3.959|3.959|4.009|1.012857|1.012692|0|0|
|203|false|1|4.258|4.259|4.294|1.007776|1.007416|0|0|
|352|false|1|6.499|6.500|6.588|1.013211|1.013287|0|0|
|355|false|1|6.785|6.787|6.865|1.011440|1.011187|0|0|
|8|true|1|0.772|0.772|0.753|0.974789|0.973819|7|7|
|9|true|1|0.885|0.885|0.862|0.976124|0.976940|7|7|
|16|true|1|0.904|0.904|0.905|0.996271|0.999049|5|4|
|17|true|1|1.031|1.028|1.022|0.991211|0.990375|7|7|
|24|true|1|1.041|1.041|1.030|0.989050|0.989068|7|6|
|25|true|1|1.170|1.169|1.152|0.984380|0.985253|7|7|
|200|true|1|4.065|4.065|4.031|0.991813|0.991863|7|7|
|203|true|1|4.399|4.399|4.339|0.986508|0.986904|7|7|
|352|true|1|6.779|6.779|6.759|0.997520|0.997100|6|7|
|355|true|1|7.008|7.011|6.952|0.992292|0.991756|7|7|

Times are medians of stage averages; ratios are geometric means of paired block ratios and need not equal ratios of displayed times. [Rows](artifacts/rows.json) retain every observation and CPU, faults, switches, lifetime RSS and record-capacity receipts. Whole-process component RSS is not incremental stack or minimum prover RAM. No successful capture was repeated or case excluded.

## Frozen decision

Every identical-control B/A case must have median within factor1.03 and geometric ratio within factor1.05. Every candidate case must have geometric ratio≤1.02 against both controls. The fixed targeted signed200/203/352/355 aggregate over both seeds must improve against both controls with at least4/7 winning block aggregates. There is no minimum gain floor, per-block veto, phase-order sweep or partial promotion. A passing component supports a separate standard SDK trial, not adoption or proof-speed claims. A failure closes this candidate with all positives retained.

Calibration failures: `[]`.

Nonregression failures: `[]`.

Aggregate failures: `[]`.

Targeted signed aggregates: `{"control-a":{"block_ratios":[0.9920871788698968,0.9921661820049006,0.9912828655454756,0.9918399807306988,0.9918994728719024,0.991881168888596,0.991660384186268],"geometric_ratio":0.9918309962182582,"median_ratio":0.991881168888596,"wins":7},"control-b":{"block_ratios":[0.9915845933754284,0.9922607244067077,0.9922022229374816,0.991545403327906,0.9918810076121316,0.9912103164630977,0.9920263211755328],"geometric_ratio":0.9918157344400528,"median_ratio":0.9918810076121316,"wins":7}}`.

## Validation and limits

All111 native core tests pass, including two new independent complete-curve accumulation/reuse tests. They cover both canonical modes, signed/unsigned inputs, both inverse modes, all three record-storage modes, empty/one/two/odd/even full-group boundaries and scalar tails. Growing/shrinking batches retain bucket addresses and reuse scratch. All backing buckets match independent final curve points, including untouched destinations. Raw2/cubic6, generator and portable/no-IFMA consumer checks pass. Field primitives and accepted field qualification remain unchanged. Exact original function copies and fixture exports exist only under test/diagnostic cfg. Inverse source edits recover accepted files; all170 production sources and exact accepted SDK are unchanged.

Four warmFIFO999Hzcycles:u profiles follow all840latency stages: signed200/352, seed1, originalA/candidate,131072calls each. Every profile samples one own PID/TID with zero lost records. They cover the full repeated caller. [Profile summaries](artifacts/profile-summary.json) retain every symbol period and sample count; [symbol listings](artifacts/codegen-symbols.txt) describe actual linked functions. Neither static code sizes nor sampled periods establish dynamic instruction counts, cache causes or removable proof walltime. Raw profiles remain local with [hashes](artifacts/raw-local-only-sha256.json). All source/library/consumer/tool/helper pins are frozen before capture and rechecked at close. No build, helper, hash, archive or verifier overlaps a capture.

The four profiles contain 2,766 samples: 519/520 for original/candidate at 200 inputs and 865/862 at 352. The linked canonical signed G1 function grows from 26,411 to 38,378 bytes. Extra code and vector temporaries remain potential tradeoffs for a whole-proof trial; no instruction-overlap or spill cause is established by these timings alone.

This component produces0SDK/Gnark exports; the ledger stays8310. No new M5/AWS/Aeglos/phone/TEE/giant/swap/pressure/concurrency/throughput or minimum-RAM result is claimed. The [latest full SDK report](../2026-10-27-tree-kernel-split/README.md) retains all supplied M5 CPU/Metal comparisons for a different candidate, with0/8CPU10×rows. Earlier adaptive31.530µs9/9primary gains remain positive alongside unresolved qualification.

[Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [edits](artifacts/edits.json), [analysis](artifacts/analysis.json), [native tests](artifacts/native/core/tests.json), [artifact hashes](artifact-index.json).
