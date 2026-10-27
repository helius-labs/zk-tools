# Clean bounded serial FFT leaves

Complete SDK performance gate: **false**. The private candidate takes **8.324853 ms** on primary, versus **8.372564 ms accepted** and **8.296841 ms fresh**. Paired candidate/accepted is **0.996749952**, individual median interval **[0.990824680, 1.000480097]**, 11/15 wins, with a median paired saving of **27.342 µs**. Candidate/fresh is **0.999533265**. Positive estimates are retained without a gain floor. Displayed medians and paired estimates have different aggregation.

The candidate is not adopted in this experiment. The exact accepted 171-file implementation and local 824164 SDK remain unchanged. The original all-eight supplied M5 CPU 10× goal remains active, achieved 0/8.

## Change

The candidate replaces the stored FFT transform with one implementation that carries a fixed cutoff through the existing 32-way outer recursion. Each expanded leaf still holds at most 1024 vectors. Inside it, binary forks stop at floor_power_of_two(clamp(vectors/(2*workers),64,1024)); terminal DIF/DIT loops use the existing quad and within-pair butterflies. The cutoff is computed once per stored transform, with saturated arithmetic. Root/column caches, packed storage, expansion allocation, load/store/drop and field operations remain. One original file changes and one private leaf module is added; the other 170 originals are exact. Inverse edits recover all 171 production files.

The earlier serial-leaf trial kept a runtime dispatcher and a duplicate outer-transform body. This clean source removes both. No runtime option, experimental cfg, OnceLock, second transform implementation, new buffer, key table, plan, pool, unsafe code or cutoff sweep is introduced. Existing terminal out-of-line annotations are inherited without tuning. New frames carry only references and a scalar cutoff. The allocation forecast is unchanged for the same storage, but scheduling can change lifetimes and RSS; actual resource/concurrency qualification is still required. See the [integration review](artifacts/integration-review.md).

The earlier 2026-10-26 serial-leaf experiment passed every 32-worker condition but failed overall physical16 OFF calibration. Its original positive estimates and failed gate remain unchanged. This is a different implementation and new complete-proof study; no previous captures are repeated or pooled. Cheap-pass grain, queue and retained-deque outcomes remain separate evidence. No arithmetic or universal twofold gain is inferred from the earlier measurements.

## Full SDK results

Ryzen 9950X, fifteen fixed rotated/reversed fresh-process blocks. All nine shapes use 32 workers on CPUs 0–31; primary also uses 16 workers on physical CPUs 0–15. Primary32 uses 151 requests after three primes; physical16 uses 501 after three; other cases 31 after three. The longer physical16 observation was declared before capture because its earlier controls varied; its calibration/nonregression bounds remain unchanged. All 450 latency processes finish before six separate warm profiles. Loading/setup/primes are excluded from latency and included in lifetime RSS. Fresh solving, randomness, proving, internal SDK verification and serialization are timed. No owned build, helper, hash, archive, independent verifier or second benchmark overlaps captures. No successful capture is excluded, extended or repeated.

Times are medians of process medians; ratios and savings pair blocks. RSS is the maximum whole-process observation, not minimum RAM. Individual intervals assume independent/exchangeable blocks and do not provide familywise coverage.

|Case|Accepted ms|Fresh ms|Candidate ms|Paired C/A|Individual interval|Wins|Saving µs|RSS A/C MiB|
|---|---:|---:|---:|---:|---|---:|---:|---:|
|1x2-w32|8.372564|8.296841|8.324853|0.996749952|[0.990824680, 1.000480097]|11/15|27.342|135.25/134.50|
|2x2-w32|11.425210|11.430436|11.460675|0.995481622|[0.992421376, 1.007397201]|8/15|51.116|269.50/275.75|
|4x2-w32|16.102861|15.950026|15.854365|0.992302049|[0.982373816, 1.000601346]|11/15|122.571|311.50/310.50|
|8x2-w32|24.382666|24.162810|24.084626|0.999404914|[0.981837671, 1.015974128]|8/15|14.227|509.50/516.75|
|12x2-w32|34.164589|33.974509|33.693397|0.988858612|[0.980772624, 0.996861282]|13/15|378.264|707.50/718.75|
|16x2-w32|40.154939|40.002315|39.981636|0.994779591|[0.989735871, 0.999285572]|12/15|209.205|815.50/836.75|
|24x2-w32|66.972265|67.208254|66.010670|0.985641892|[0.979553155, 0.989545279]|15/15|961.595|1287.43/1287.50|
|merge-24x1-w32|66.125565|66.112380|65.086624|0.985864319|[0.978186118, 0.991434917]|15/15|939.082|1170.29/1161.87|
|merge-54x1-w32|164.132958|163.761572|161.341835|0.986881616|[0.973756676, 0.990715937]|15/15|2141.933|1876.76/1910.19|
|1x2-w16|10.168078|10.078075|10.036855|0.988926610|[0.977724882, 1.013370972]|9/15|112.363|94.25/93.25|

All ten calibration checks pass, including physical16. Primary32's paired estimates are positive but uncertain: 27.342 µs versus accepted and 3.867 µs versus fresh, with both intervals crossing parity. The 8×2 candidate/fresh estimate is also positive (ratio 0.992893), but its upper bound 1.024137 does not support the required ≤1.02 limit. This is insufficient evidence for that nonregression bound, not an established causal slowdown.

Larger-case gains remain useful evidence. Merge54 saves a paired 2.142 ms versus accepted and 2.572 ms versus fresh; both intervals are below parity. Original-eight aggregate intervals are below parity against both controls. Whole-process RSS remains mixed, including 1876.76/1910.19 MiB accepted/candidate for merge54. No universal memory reduction is established. Any later engineering adoption would need a separate explicit decision and resource qualification while retaining this failed gate.

All fresh/control pairs, CPU observations and every block are retained in analysis.json. Original-eight paired geometric aggregates:

|Comparison|Median ratio|Individual interval|
|---|---:|---|
|candidate/accepted|0.991436260|[0.988255655, 0.995691838]|
|candidate/fresh|0.995810485|[0.991857076, 0.998117612]|

## Frozen performance qualification

Every fresh/accepted individual median interval must lie wholly within factor 1.03. Candidate upper bounds must be ≤1.02 against both controls in every case. Primary32 must have upper <1 against both; the original-eight within-block geometric aggregate must have upper ≤1 against both. Fifteen blocks use ranks 4..12, with 96.484375% nominal individual coverage under independent/exchangeable-block assumptions. There is no minimum gain floor or post-result exclusion, threshold change or unchanged rerun. Known physical16 variability remains visible. The earlier adopted source's engineering exception is unchanged and does not waive this study's checks.

calibration_failures: `[]`.

regressions: `["8x2-w32 candidate/fresh upper=1.0241365008166312"]`.

primary_gain_failures: `["1x2-w32 candidate/accepted upper=1.0004800967220997","1x2-w32 candidate/fresh upper=1.0078757451919376"]`.

aggregate_failures: `[]`.

## CPU and profiles

Warm user+system CPU is summed across threads per request. Profile shares are sampled user-cycle weights with inlining/attribution limits, not removable wall-time fractions or exclusive critical-path costs. Profiles occur after latency and do not replace its acceptance observations.

|Case|Variant|CPU ms/request|Whole-process RSS MiB|
|---|---|---:|---:|
|1x2-w32|accepted|191.212|135.25|
|1x2-w32|fresh|190.789|134.50|
|1x2-w32|candidate|190.274|134.50|
|2x2-w32|accepted|264.058|269.50|
|2x2-w32|fresh|264.789|265.50|
|2x2-w32|candidate|263.682|275.75|
|merge-54x1-w32|accepted|4492.901|1876.76|
|merge-54x1-w32|fresh|4480.872|1849.05|
|merge-54x1-w32|candidate|4427.522|1910.19|
|1x2-w16|accepted|125.159|94.25|
|1x2-w16|fresh|124.913|94.00|
|1x2-w16|candidate|124.609|93.25|

The six separate 199 Hz profiles contain 685243 samples, zero lost records and one sampled SDK PID each. Each measures 251 requests after three primes.

|Case|Variant|Samples|Profile CPU ms/request|Epoch %|Scheduler %|G1 %|G2 %|
|---|---|---:|---:|---:|---:|---:|---:|
|1x2-w32|accepted|9333|192.887|9.196|4.332|39.153|29.434|
|1x2-w32|fresh|9229|191.939|9.455|4.396|35.775|30.542|
|1x2-w32|candidate|9174|191.139|11.791|3.897|35.316|30.465|
|merge-54x1-w32|fresh|219599|4475.470|2.471|2.204|27.681|8.588|
|merge-54x1-w32|candidate|217449|4433.613|2.118|2.034|28.292|8.669|
|merge-54x1-w32|accepted|220459|4484.495|2.438|2.307|27.898|8.370|

## Supplied M5 references

Historical supplied CPU/Metal values, not fresh Apple measurements. No 54→1 reference was supplied. Candidate values are experimental.

|Shape|Candidate ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required CPU10× ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.324853|37|4.445×|24|2.883×|3.7|
|2x2|11.460675|59|5.148×|30|2.618×|5.9|
|4x2|15.854365|109|6.875×|48|3.028×|10.9|
|8x2|24.084626|189|7.847×|74|3.072×|18.9|
|12x2|33.693397|284|8.429×|101|2.998×|28.4|
|16x2|39.981636|335|8.379×|124|3.101×|33.5|
|24x2|66.010670|528|7.999×|176|2.666×|52.8|
|merge-24x1|65.086624|507|7.790×|175|2.689×|50.7|

## Validation and scope

Fresh passes all 116 native core tests and candidate 119. Three reused independent oracle tests check cutoff bounds/available parallelism, scalar-polynomial inverse coefficients before ordinary/twisted forward outputs, cached/uncached roots, packed/unpacked storage, and concurrent shared-root callers with 512 KiB worker stacks. These are previous oracle cases applied to a different binary, not new distinct mathematical cases. Both pass two raw-image and six cubic tests; candidate portable/no-IFMA consumers pass. Field source and the accepted 33-test native qualification are inherited, with no fresh field-suite claim. Kernel concurrency tests do not establish whole-SDK pressure/concurrency/huge-circuit behavior.

Nine four-worker candidate preflights add 36 SDK calls/nine exports; latency adds 41,850/450; profiles 1,524/six. Total **43,410 SDK calls, 465 distinct independently Gnark-verified exports and 930 proof/public members**, ledger **11,110→11,575**. All proofs are 164 bytes, fixture-bound and fresh against all preceding/new exports; every SDK call verifies internally. Source, library, executable, input and proof pins are rehashed at closure.

All initial setup, build and observer receipts are retained. No successful capture, test or verifier is repeated as part of analysis or publication. Raw perf.data files remain local with hashes; privileged capture ownership is transferred to the experiment owner after measurement for reading, without altering bytes or mode.

This local study creates no AWS resources and makes no new Aeglos, phone, ARM, TEE, swap, minimum-RAM, pressure, concurrent-throughput or huge-circuit claim. Any performance pass still requires a separate compatibility/concurrency/resource review before adoption. The original all-eight CPU10× objective remains active.

[Protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [raw rows](artifacts/performance/rows.json), [source edit](artifacts/edits.json), [native workflow](artifacts/native/result.json), [proof inventory](artifacts/proof-inventory.json), [raw profile hashes](artifacts/raw-local-only-sha256.json), [closeout](artifacts/closeout.json), [artifact hashes](artifact-index.json).
