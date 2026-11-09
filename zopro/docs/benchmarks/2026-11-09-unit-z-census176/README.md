# Actual-proof unit-Z mixed-add census

**Real proofs contain eligible unit-Z calls on every measured shape. This selects private integration; it is not a measured speedup or adoption.** The [qualified prototype](../2026-11-09-unit-z-mixed176/README.md) uses the standard 4M+2S specialization instead of general 7M+4S mixed addition when the left Jacobian Z is one. This diagnostic executes the original arithmetic throughout.

At primary32, G1 has **8,110 ordinary unit-Z calls out of 20,770 ordinary mixed adds (39.05%)**; G2 has **441 out of 2,009 (21.95%)**. These fractions apply to this scalar operation inside one complete SDK request. The main SIMD affine streams bypass it. They are not fractions of all curve work, proof time, CPU cycles or removable critical-path time.

|Counted request|G1 unit / ordinary mixed calls|G1 eligible %|G2 unit / ordinary mixed calls|G2 eligible %|
|---|---:|---:|---:|---:|
|1x2-w32|8110 / 20770|39.05%|441 / 2009|21.95%|
|2x2-w32|492 / 5056|9.73%|371 / 1203|30.84%|
|4x2-w32|642 / 6723|9.55%|309 / 2046|15.10%|
|8x2-w32|813 / 6912|11.76%|154 / 1882|8.18%|
|12x2-w32|669 / 6747|9.92%|379 / 2111|17.95%|
|16x2-w32|564 / 6649|8.48%|310 / 2040|15.20%|
|24x2-w32|987 / 8601|11.48%|269 / 2003|13.43%|
|merge-24x1-w32|774 / 6871|11.26%|286 / 2012|14.21%|
|merge-54x1-w32|804 / 9944|8.09%|348 / 2085|16.69%|
|1x2-w1|8110 / 20767|39.05%|440 / 2010|21.89%|
|1x2-w16|8110 / 20769|39.05%|440 / 2003|21.97%|

There are 33,822 ordinary unit-Z calls across the eleven counted requests. Each case is one observation, with fresh proof randomness; this is not a frequency confidence interval or a controlled explanation of cross-case differences. Primary G1 eligibility is similar at one, sixteen and thirty-two workers. There is no exact thread-invariance claim.

Diagnostic field hooks classify entries, right/left identity, and ordinary/equal/opposite branches split by unit/nonunit Z. Identity precedence and original arithmetic/returns stay unchanged. Classification reuses the original scaled X/Y results. Two nine-counter arrays and an enable flag exist only behind the private diagnostic cfg. Counters stay disabled during loading and three warm proofs, then cover exactly one fresh `sdk.prove` call, including its internal verification and serialization, and stop before export. Atomic overhead makes diagnostic times unsuitable for comparison; the driver contains no request timer.

All eight original shapes plus merge54 run at32 workers, with primary also solo and at16. Every process uses the original fixed-placement setup/snapshot code. All eleven SDK processes finish before eleven offline Gnark verifications. Source, executable, dependencies, images, requests, order, analyzer and reader are pinned before SDK execution; no independent owned compute/hash/build/report overlaps recording.

The focused counter test passes every classification, disabled/reset boundaries, all53 actual broadcast workers across1/4/16/32 pools with requested512KiB stacks, and four OS callers sharing one eight-worker pool. This tests counters, not concurrent SDK throughput. The initial build completes field/counter-test/core/prover/zolana stages, then the SDK driver fails to compile an ambiguous integer sum. A driver-only `sum::<u64>()` annotation fixes that last link; all six successful steps are reused. The failed command/source remain archived. No SDK run, arithmetic test, profile, decoder or proof is replayed.

Review verifies all29 successful workflow commands and the retained failed link, their exact arguments/exits/stream hashes, all176 production/source originals, all11 raw censuses, branch conservation, fixed worker placement, proof freshness and image/public bindings. The two observer edits and one child recover the original arithmetic/source; production remains unchanged. Helper compiler/review/publisher invocations have tool-transcript evidence rather than complete xtask receipts. The separate reader is by the same agent, not an external audit.

The experiment makes **44 SDK requests** (three warm plus one counted per process) and adds **11 fresh unique independently Gnark-verified 164-byte exports**, advancing the canonical proof inventory **41,792 → 41,803**. Calls are not unique exports. No new memory/RSS/minimum-RAM, CPU/profile, latency, AWS/Aeglos, ARM/mobile/TEE, huge250/4999, pressure/swap or concurrent-throughput result is added. Diagnostic counter/driver storage is not proposed product memory. No runtime key table/copy, solver plan, production pool or memory policy is added.

The next implementation inserts the qualified formula after the identity guards already inside `Projective::add_affine`, avoiding the standalone wrapper's repeated guards and fallback prologue. It must qualify the actual integrated field and coherent downstream SDK, then face a prospective all-shape performance comparison. The census establishes opportunity, not a speed projection or adoption exception. Static nominal MULX savings are retained in JSON as a source-work diagnostic only.

The accepted local timings below are inherited from the [bitmap-reset adoption](../2026-11-09-busy-reset-adoption176/README.md). Supplied M5 CPU/Metal references are historical observations on different hardware. The diagnostic SDK is not a new latency baseline.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10x target ms|
|---|---:|---:|---:|---:|
|1x2|7.809034|37 / 4.738x|24 / 3.073x|3.7|
|2x2|10.940974|59 / 5.393x|30 / 2.742x|5.9|
|4x2|15.750370|109 / 6.920x|48 / 3.048x|10.9|
|8x2|23.951790|189 / 7.891x|74 / 3.090x|18.9|
|12x2|33.857141|284 / 8.388x|101 / 2.983x|28.4|
|16x2|39.726526|335 / 8.433x|124 / 3.121x|33.5|
|24x2|65.984808|528 / 8.002x|176 / 2.667x|52.8|
|merge-24x1|65.660588|507 / 7.722x|175 / 2.665x|50.7|

All-eight CPU10x goal remains active and unmet, **0/8**. Primary accepted peak process RSS remains132.50MiB; merge54 remains163.167792ms without an M5 reference. The baseline was already below8ms; historical7.699867ms used a different campaign/statistic.

[All counts and bindings](artifacts/analysis.json), [raw reader](artifacts/review-result.json), [source recovery](artifacts/source-audit.json), [fixed design](artifacts/design.md), [continuation](artifacts/next.md), [source/helpers/receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Full source snapshots, binaries/libraries, key/image/witness/proof payloads and the full historical proof ledger are omitted.
