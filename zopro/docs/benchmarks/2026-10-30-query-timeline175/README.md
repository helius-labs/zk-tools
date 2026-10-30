# In-process query critical-path timeline

30 October 2026. Diagnostic stage profiling of the exact accepted algorithm, with OFF and traced sharing one executable. This is not an optimization or adoption. The previous balanced G2 full SDK trial remains unadopted.

The **one-worker and 32-worker overhead checks pass**; both physical-16 checks fail. At 32 workers, traced/OFF has paired ratio **1.000368633 [0.995358595, 1.005165604]** and OFF/accepted **0.999372770 [0.996096515, 1.006144230]**, within the declared factor 1.01. The fresh accepted median is 7.955612 ms; this diagnostic does not replace its historical qualification result of 8.022165 ms / 134 MiB.

Across 4,681 traced requests at 32 workers, **K/H returns last in 2,136 (45.6%), B2 in 1,656 (35.4%), and A in 889 (19.0%)**. The median last-to-second return gap is **0.008335 ms**. The native prefix through product completion takes a median **1.864279 ms**, followed by a **5.237387 ms** query-join interval. These are separate medians and must not be added to reconstruct a median proof.

This replaces the earlier overhead-failing uprobe observation with a qualified 32-worker measurement. It does not identify one query as the sole bottleneck. Query functions can execute and wait for internal Rayon work, so close return times do not imply equal arithmetic costs or independently prove which leaf controls completion. Further work should address shared MSM work or the prefix and should retain full-proof qualification. The smaller CPU saving but limited wall gain from balanced G2 remains a separate measured result.

At one worker, the queries execute serially. B2 returning last in every request reflects the existing execution order; all three queries contribute to latency. The physical-16 observations remain instrumented-only because overhead bounds failed. No topology or sample is discarded or repeated.

Three primes precede151fresh verified SDK requests per process. Thirty-one fixed rotated/reversed blocks compare frozen accepted, OFF and traced at1/16/32workers, with whole-process affinity CPU2 / CPUs0–15 / CPUs0–31. Shared Ryzen9950X host. Timings include solving, randomized proving, SDK verification and serialization; loading is excluded from latency and included in lifetime RSS. All helpers compile/pin before capture; no owned build/helper/hash/archive/analyzer or independent workload overlaps captures.

## Overhead qualification

Both OFF/accepted and traced/OFF paired median intervals must fit factor1.01 before treating a topology as unperturbed. Ranks10–22/31 provide nominal97.055%individual median coverage under independent/exchangeable blocks, without familywise guarantees. All rows and failed checks are retained, without exclusions/extensions/rerolls.

|Case / variant|Median ms|CPU ms/request|Peak RSS MiB|
|---|---:|---:|---:|
|1x2-w1/OFF|95.583678|95.609|34.50|
|1x2-w1/accepted|96.069122|96.111|34.50|
|1x2-w1/traced|95.547965|95.555|34.50|
|1x2-w16/OFF|9.503627|117.221|91.25|
|1x2-w16/accepted|9.565148|117.704|91.00|
|1x2-w16/traced|9.563923|117.247|92.00|
|1x2-w32/OFF|7.954394|179.238|135.25|
|1x2-w32/accepted|7.955612|180.611|134.00|
|1x2-w32/traced|7.946960|179.286|136.50|

|Pair|Ratio|Bounds|Qualified|
|---|---:|---|---|
|1x2-w1/OFF/accepted|0.994384005|[0.992095253, 0.997271246]|true|
|1x2-w1/traced/OFF|1.000665118|[0.998565841, 1.003918491]|true|
|1x2-w16/OFF/accepted|0.995620597|[0.977384417, 1.006818082]|false|
|1x2-w16/traced/OFF|1.010502131|[0.992368276, 1.025516314]|false|
|1x2-w32/OFF/accepted|0.999372770|[0.996096515, 1.006144230]|true|
|1x2-w32/traced/OFF|1.000368633|[0.995358595, 1.005165604]|true|

Failed overhead checks: `["1x2-w16/OFF/accepted","1x2-w16/traced/OFF"]`.

## Recorded stage intervals

These are medians across4681retained traced requests per topology. They overlap and must not be added. Query intervals include internal work/waits; start is function entry, not scheduler readiness. Last-return counts identify observed function ordering, not a causal CPU/cache/scheduler explanation or a removable wall-time fraction. A topology that fails overhead qualification remains instrumented-only evidence.

### 1x2-w1

Unperturbed overhead qualification: **true**. Last query counts: `{"A":0,"B2":4681,"KH":0,"ties":0}`.

|Interval|Median ms|
|---|---:|
|canonical_conversion|0.275059|
|canonical_span|94.740164|
|join_after_last_query|0.000080|
|last_minus_second_query_end|39.659635|
|postjoin_output|0.003146|
|prefix_to_product_end|5.199083|
|product|3.040670|
|public_correction_and_release|0.000852|
|query_A_inclusive|14.598369|
|query_B2_inclusive|39.659520|
|query_KH_inclusive|35.290202|
|query_join|89.541706|
|scalar_extraction|0.031239|
|solver|1.851256|
### 1x2-w16

Unperturbed overhead qualification: **false**. Last query counts: `{"A":141,"B2":1601,"KH":2939,"ties":0}`.

|Interval|Median ms|
|---|---:|
|canonical_conversion|0.045687|
|canonical_span|8.881484|
|join_after_last_query|0.002895|
|last_minus_second_query_end|0.009518|
|postjoin_output|0.003567|
|prefix_to_product_end|1.644584|
|product|0.502750|
|public_correction_and_release|0.001082|
|query_A_inclusive|7.085484|
|query_B2_inclusive|7.205853|
|query_KH_inclusive|7.211676|
|query_join|7.222245|
|scalar_extraction|0.077045|
|solver|1.008694|
### 1x2-w32

Unperturbed overhead qualification: **true**. Last query counts: `{"A":889,"B2":1656,"KH":2136,"ties":0}`.

|Interval|Median ms|
|---|---:|
|canonical_conversion|0.069872|
|canonical_span|7.133339|
|join_after_last_query|0.002485|
|last_minus_second_query_end|0.008335|
|postjoin_output|0.004188|
|prefix_to_product_end|1.864279|
|product|0.544177|
|public_correction_and_release|0.001292|
|query_A_inclusive|5.190558|
|query_B2_inclusive|5.224504|
|query_KH_inclusive|5.203468|
|query_join|5.237387|
|scalar_extraction|0.105288|
|solver|1.131876|

## Verification, memory and limits

All576workflow steps complete:43,002internally verified requests and288fresh164-byte exports independently pass Gnark with common image/public/VK bindings. All exports are distinct against24,943prior hashes; the new union is25,231. Exports are the last timed proof, written after timing. Native126core/eightprover tests pass, including two journal/order tests; all17field sources remain exact and field qualification is inherited. Source176has174exact originals; reversing one parent's edits recovers all175.

The fixed journal requests26,624payload bytes per process, allocated before primes in OFF and traced. No query-boundary locks occur. One journal push follows joined output serialization. Overflow rejects analysis; no records are silently dropped. Records contain only durations. Lifetime RSS is not a minimum-RAM or incremental-workspace bound. No new PMU or sampled-profile execution accompanies this timestamp diagnostic; existing full-proof profiles motivated the study.

Production and all nine accepted local/AWS rows remain unchanged. The original eight CPU10× targets remain unmet. No new AWS/Aeglos, mobile/ARM/TEE, huge-circuit, memory-pressure/swap or concurrent-throughput result is claimed. Full supplied CPU/Metal comparisons and AWS configurations remain in the [current benchmark report](../2026-10-30-current-bench-report/README.md). This diagnostic selects further investigation, not a proof speedup.

A separate offline review reproduces all nine latency/CPU/RSS cells, six paired intervals, stage medians and query-return counts. It checks all 451 original pins, all 175 production files and their candidate reversals, every archived source/receipt/proof member, all 288 Gnark receipts and proof bytes, and all 14,046 timestamp records including preflights. It executes no test, benchmark, proof or verifier again. The original capture/analyzer/publisher helpers remain unchanged. [Review result](artifacts/offline-review.json), [reviewer](artifacts/offline-review.rs).

The per-request JSON is compressed after publication; decompression matches the original observations byte for byte. This changes storage only and adds no samples. [Archive check](artifacts/row-archive-review.json).

[Analysis](artifacts/analysis.json), [all per-request samples/timestamps](artifacts/rows.json.gz), [protocol](artifacts/design.md), [capture accounting](artifacts/capture-protocol.md), [source audit](artifacts/source-audit.json), [original pins](artifacts/pins.json), [workflow](artifacts/plan.json), [source archive](artifacts/source.json.gz), [proofs](artifacts/proofs.json.gz), [receipts](artifacts/receipts.json.gz), [new ledger](artifacts/proof-sha256-after.json), [exact pre-capture summary](artifacts/inherited-summary.json.gz), [accepted local/AWS rows with CPU and Metal references](accepted-rows.json).
