# Initial-affine accumulator state: complete-proof comparison

**The candidate is not adopted.** Primary 1×2 at 32 workers measures **7.803404 ms accepted REF, 7.848711 ms rebuilt OFF and 7.817299 ms ON**. ON improves over OFF by **0.492%**, with 24/31 paired wins, but has no verified gain over the accepted build: ON/REF is 1.001356, interval [0.999430, 1.003233], with 11/31 wins. Four of 79 declared checks fail. The accepted production source and SDK remain unchanged.

The [qualified candidate](../2026-11-09-initial-unit176/README.md) records initial nonidentity affine provenance in an existing exceptional-accumulator index. It consumes that state for one specialized update or finish join, then clears it. Both arms use the unchanged accepted field library. Three accepted core files contain reversible private edits and one private helper is added; all 176 originals recover, with 173 byte-exact. No Window fields, vector dimensions/capacities, table, key copy, plan, pool, allocator, forecast or memory policy changes. The previous global unit-Z recipe remains rejected.

Each sample includes solving, fresh randomized proving, internal verification and serialization. Loading, pool creation, three warm proofs and final export are outside latency. Case/arm order rotates and reverses under the frozen protocol. Primary 16/32 use 31 blocks of 151 requests; other conditions use 15 blocks of 31 requests, except primary solo uses 11 requests. Every observation is retained without repeated blocks or extensions.

|Case|REF ms|OFF ms|ON ms|ON/OFF paired ratio [95%]|Wins|ON/REF paired ratio [95%]|Wins|
|---|---:|---:|---:|---|---:|---|---:|
|1x2-w32|7.803404|7.848711|7.817299|0.995081 [0.993129, 0.996922]|24/31|1.001356 [0.999430, 1.003233]|11/31|
|2x2-w32|10.940319|11.095197|11.062430|1.000260 [0.995054, 1.005463]|6/15|1.008744 [1.003175, 1.014040]|3/15|
|4x2-w32|15.809849|15.880259|15.870876|1.002213 [0.996082, 1.007953]|6/15|1.006233 [1.000541, 1.012058]|4/15|
|8x2-w32|24.043743|24.465900|24.379404|1.003254 [0.994804, 1.012340]|8/15|1.011390 [1.000908, 1.022221]|5/15|
|12x2-w32|33.720355|34.068502|34.002981|0.999616 [0.994043, 1.005529]|9/15|1.004762 [0.995587, 1.012158]|4/15|
|16x2-w32|39.753010|40.020447|40.031868|1.003801 [0.999832, 1.007808]|5/15|1.008218 [1.003374, 1.013425]|3/15|
|24x2-w32|65.983137|66.440991|66.457689|0.997905 [0.992763, 1.002924]|9/15|1.004945 [0.999798, 1.009468]|3/15|
|merge-24x1-w32|65.378103|65.868695|65.781732|1.000815 [0.996779, 1.004887]|8/15|1.006336 [1.001307, 1.011396]|4/15|
|merge-54x1-w32|162.189200|162.527896|162.096149|1.000452 [0.997275, 1.003932]|8/15|1.002144 [0.999728, 1.004506]|5/15|
|1x2-w1|94.508760|95.869562|93.968371|0.981230 [0.977055, 0.984932]|15/15|0.994605 [0.991962, 0.997474]|12/15|
|1x2-w16|9.195841|9.300008|9.221445|0.992869 [0.990811, 0.994882]|28/31|1.001306 [0.999666, 1.002854]|8/31|

Displayed times are medians of process means of integer-rounded emitted nanosecond samples. Paired ratios use 20,000 log-bootstrap resamples, so dividing displayed marginal medians need not reproduce the paired estimate. Intervals assume exchangeable blocks and have no simultaneous familywise guarantee. All-eight aggregates independently resample within each shape with equal log weight; the different block counts are not treated as synchronized across shapes.

The positive component signal is retained. Primary ON/OFF improves at 1, 16 and 32 workers; solo ON also beats REF by 0.539%, interval [0.253%, 0.804%]. At 32 workers, however, the rebuilt OFF is itself 0.631% slower than REF and ON does not establish an accepted-build improvement. All 11 declared calibration checks pass; the OFF/REF primary interval [1.004515, 1.008116] is within the declared range. No unique compiler, layout, cache or clock explanation is established for the difference.

All-eight ON/OFF is **1.000365 [0.998464, 1.002328]**: no overall gain. All-eight ON/REF is **1.006494 [1.004209, 1.008729]**, a 0.649% slowdown. The complete recipe is closed without adoption, unchanged recapture, compiler/inlining/layout/threshold changes or a favorable curve/shape/worker subset. This decision uses the failed complete gate; it does not discard the small positive estimates or impose a minimum gain floor.

All 11 calibration, 22 nonregression, 22 RSS and 18 diagnostic checks pass. The four failures are:

- `1x2-w32/ON/REF/primary-gain`
- `1x2-w32/ON/REF/primary-wins`
- `ALL8/ON/OFF/gain`
- `ALL8/ON/REF/gain`

Maximum observed whole-process RSS, including loading and warm proofs:

|Case|REF MiB|OFF MiB|ON MiB|
|---|---:|---:|---:|
|1x2-w32|132.50|133.50|134.50|
|2x2-w32|277.75|267.00|267.25|
|4x2-w32|315.25|317.75|308.25|
|8x2-w32|505.25|509.00|520.00|
|12x2-w32|719.75|723.00|717.50|
|16x2-w32|831.25|836.00|840.25|
|24x2-w32|1280.09|1309.00|1282.50|
|merge-24x1-w32|1175.52|1184.06|1155.50|
|merge-54x1-w32|1984.09|1945.55|1981.81|
|1x2-w1|34.25|34.75|34.75|
|1x2-w16|88.50|89.00|89.00|

All declared 5% RSS guards pass. Primary ON reaches 134.50 MiB versus REF 132.50 MiB. These observations are neither minimum RAM nor memory-pressure, mobile or stack-peak measurements. Logical storage remains unchanged.

Nine PMU panels meet the required 99.9% running coverage. Nine fixed-period profiles qualify with **119,775 samples**, minimum 8,880, maximum unknown 2.088816%, zero LOST and checked executable/PID/TID/CPU ownership. Recording ends before offline decoding and Gnark verification. Absolute period coverage remains unvalidated; flat sample shares are not wall-time critical-path fractions.

The following single PMU panels have no replication confidence interval and do not isolate a memory, frequency or dependency cause:

|Diagnostic case|Instructions ON/OFF|Cycles ON/OFF|Cache misses ON/OFF|Branch misses ON/OFF|
|---|---:|---:|---:|---:|
|1x2-w32|0.993493|0.989284|0.987149|0.982860|
|1x2-w16|0.994423|0.987683|1.001648|0.998229|
|2x2-w32|0.998066|0.995393|1.006755|0.974818|

The primary 32-worker panel reduces instructions by 0.651% and cycles by 1.072% against OFF. This is consistent with a small local work reduction; it does not override the repeated latency gate. The accepted profile still attributes 21.47% of flat samples to the packed G1 stream and 15.81% to borrowed G2 application. Further scalar-path tuning has not produced the required algorithmic improvement. The next mechanism should address bulk work or dependencies, while preserving the low-memory architecture and relevant prior negative results.

Four frozen helper builds, 52 preflight commands and 1,263 main commands complete on first execution in this measurement stage. Prior native qualification's test-only oracle correction is inherited from the linked report. No runtime/helper/reader/recording repair occurs here. The separate same-agent reader checks 451 capture pins, recovery of 176 original sources, coherent original-field/library bindings, every raw mean, receipt, placement, proof binding and all 79 decisions. Bootstrap and diagnostic parsers are shared unchanged; this is not an external audit. Manual preparation/analysis/publication helper invocations do not all have xtask receipts.

Preflights add 104 SDK requests and 26 fresh Gnark exports, then the main plan adds 44,106 requests and 609 exports. Total: **44,210 SDK requests and 635 fresh unique independently Gnark-verified exports**, advancing the canonical inventory **42,438 → 42,464 → 43,073**. All exported proofs are 164 bytes with checked image/public/freshness bindings. Requests are not distinct independently exported proofs.

Fresh REF observations below measure the unchanged accepted artifact. M5 CPU/Metal figures are supplied historical measurements on different hardware and were not remeasured. Candidate ON remains rejected.

|Shape|Accepted REF ms|Rejected ON ms|M5 CPU ms / REF speedup|M5 Metal ms / REF speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|
|1x2|7.803404|7.817299|37.0 /4.742x|24.0 /3.076x|3.7|
|2x2|10.940319|11.062430|59.0 /5.393x|30.0 /2.742x|5.9|
|4x2|15.809849|15.870876|109.0 /6.894x|48.0 /3.036x|10.9|
|8x2|24.043743|24.379404|189.0 /7.861x|74.0 /3.078x|18.9|
|12x2|33.720355|34.002981|284.0 /8.422x|101.0 /2.995x|28.4|
|16x2|39.753010|40.031868|335.0 /8.427x|124.0 /3.119x|33.5|
|24x2|65.983137|66.457689|528.0 /8.002x|176.0 /2.667x|52.8|
|merge-24x1|65.378103|65.781732|507.0 /7.755x|175.0 /2.677x|50.7|

The full verified CPU 10× goal remains active and unmet, **0/8**. The accepted build was already below 8 ms; this experiment establishes no new speed milestone. No new AWS/Aeglos, ARM/mobile, huge-circuit, memory-pressure, swap, minimum-RAM or concurrent-throughput result is added.

[Results](artifacts/analysis.json), [protocol](artifacts/protocol.md), [raw reader](artifacts/review-result.json), [prospective review](artifacts/prospective-review.json), [preflights](artifacts/preflight-review.json), [proof inventory](artifacts/proof-inventory.json), [diagnostics](artifacts/diagnostic-summary.json), [CPU/Metal comparison](artifacts/fresh-comparison.json), [decision](artifacts/decision.json), [continuation](artifacts/next.md), [sources and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
