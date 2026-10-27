# G2 scalar decomposition task boundary

Complete SDK performance gate: **false**. The private candidate takes **8.321766 ms** on primary, versus **8.332936 ms accepted** and **8.361881 ms fresh**. Paired candidate/accepted is **0.997331229**, individual median interval **[0.995287557, 1.001897544]**, 11/15 wins, with a median paired saving of **22.171 µs**. Candidate/fresh is **0.995962153**. Positive estimates are retained without a gain floor. Displayed medians and paired estimates have different aggregation.

The candidate is not adopted in this experiment. The exact accepted 171-file implementation and local 824164 SDK remain unchanged. The original all-eight supplied M5 CPU 10× goal remains active, achieved 0/8.

## Change

Only the existing in-place `phi_split` parallel iterator adds `with_min_len(128)`. It applies the accepted canonical-conversion task boundary to this separate decomposition stage. The minimum covers 4 KiB of existing scalar storage and adds no buffer or memory cap. Per-scalar arithmetic, ordering, subgroup formulas, points, bucket geometry, dependencies, worker pool, queue, key preparation and resource forecast remain unchanged. No runtime flag, unsafe code, ISA change, lookup table or grain sweep is introduced. The other 170 files are byte-exact; reversing this edit recovers all 171 accepted sources.

Historical C7a recordings showed late G2 release and increasing preparation/coordination costs. They did not isolate a removable decomposition cost or predict a speedup. Earlier weighted-reduction grain, partition, guard-reuse and shared-phase failures remain evidence; this changes a different iterator.

The changed iterator belongs to the exclusive small-G2 path. Larger original queries retain their existing decomposition path. Differences in those other rows cannot be attributed to this iterator's work reduction; fresh-process variation and linked code layout remain possible influences.

## Full SDK results

Ryzen 9950X, fifteen fixed rotated/reversed fresh-process blocks. All nine shapes use 32 workers on CPUs 0–31; primary also uses 16 workers on physical CPUs 0–15. Both primary cases use 151 requests after three primes, other cases 31 after three. All 450 latency processes finish before six separate warm profiles. Loading/setup/primes are excluded from latency and included in lifetime RSS. Fresh solving, randomness, proving, internal SDK verification and serialization are timed. No owned build, helper, hash, archive, independent verifier or second benchmark overlaps captures. No successful capture is excluded, extended or repeated.

Times are medians of process medians; ratios and savings pair blocks. RSS is the maximum whole-process observation, not minimum RAM. Individual intervals assume independent/exchangeable blocks and do not provide familywise coverage.

|Case|Accepted ms|Fresh ms|Candidate ms|Paired C/A|Individual interval|Wins|Saving µs|RSS A/C MiB|
|---|---:|---:|---:|---:|---|---:|---:|---:|
|1x2-w32|8.332936|8.361881|8.321766|0.997331229|[0.995287557, 1.001897544]|11/15|22.171|134.75/134.50|
|2x2-w32|11.486945|11.447269|11.444545|0.993769613|[0.989654947, 1.008200562]|11/15|71.495|267.25/271.25|
|4x2-w32|15.978927|15.928882|15.803015|0.989192567|[0.983291268, 1.006555044]|11/15|172.656|314.25/324.00|
|8x2-w32|24.088421|24.039950|24.169805|0.992862723|[0.987087469, 1.014801091]|10/15|174.230|500.00/507.50|
|12x2-w32|34.092441|34.072393|34.125382|0.998893754|[0.986468069, 1.015614691]|9/15|38.773|719.00/716.00|
|16x2-w32|40.263725|40.061768|40.233292|0.997544139|[0.992848318, 1.009883333]|9/15|98.636|815.75/824.25|
|24x2-w32|67.238808|67.054452|67.022116|0.998752369|[0.989522011, 1.004712345]|8/15|83.638|1276.50/1325.00|
|merge-24x1-w32|65.979323|66.451962|66.105108|1.002320020|[0.998929942, 1.009914212]|4/15|-153.290|1163.28/1183.50|
|merge-54x1-w32|164.222695|164.043742|163.813092|0.996211247|[0.990715885, 1.001068772]|11/15|624.487|1863.34/1876.77|
|1x2-w16|10.281682|10.247887|10.063110|0.978741611|[0.936537513, 0.995357649]|12/15|218.572|89.75/89.00|

All fresh/control pairs, CPU observations and every block are retained in analysis.json. Original-eight paired geometric aggregates:

|Comparison|Median ratio|Individual interval|
|---|---:|---|
|candidate/accepted|0.998903853|[0.996863537, 1.000286152]|
|candidate/fresh|0.997658386|[0.995123881, 1.002833964]|

## Frozen performance qualification

Every fresh/accepted individual median interval must lie wholly within factor 1.03. Candidate upper bounds must be ≤1.02 against both controls in every case. Primary32 must have upper <1 against both; the original-eight within-block geometric aggregate must have upper ≤1 against both. Fifteen blocks use ranks 4..12, with 96.484375% nominal individual coverage under independent/exchangeable-block assumptions. There is no minimum gain floor or post-result exclusion, threshold change or unchanged rerun. Known physical16 variability remains visible. The earlier adopted source's engineering exception is unchanged and does not waive this study's checks.

calibration_failures: `["1x2-w16 fresh/accepted interval [0.9653793354936588,1.0210141129259243]"]`.

regressions: `[]`.

primary_gain_failures: `["1x2-w32 candidate/accepted upper=1.001897544187222","1x2-w32 candidate/fresh upper=1.0000564159351661"]`.

aggregate_failures: `["candidate/accepted original8 upper=1.0002861521322115","candidate/fresh original8 upper=1.0028339643208781"]`.

## CPU and profiles

Warm user+system CPU is summed across threads per request. Profile shares are sampled user-cycle weights with inlining/attribution limits, not removable wall-time fractions or exclusive critical-path costs. Profiles occur after latency and do not replace its acceptance observations.

|Case|Variant|CPU ms/request|Whole-process RSS MiB|
|---|---|---:|---:|
|1x2-w32|accepted|191.175|134.75|
|1x2-w32|fresh|192.406|133.75|
|1x2-w32|candidate|191.244|134.50|
|2x2-w32|accepted|264.458|267.25|
|2x2-w32|fresh|265.562|266.50|
|2x2-w32|candidate|264.234|271.25|
|merge-54x1-w32|accepted|4503.919|1863.34|
|merge-54x1-w32|fresh|4504.515|1886.94|
|merge-54x1-w32|candidate|4484.620|1876.77|
|1x2-w16|accepted|124.503|89.75|
|1x2-w16|fresh|124.909|90.25|
|1x2-w16|candidate|125.449|89.00|

The six separate 199 Hz profiles contain 64454 samples, zero lost records and one sampled SDK PID each. Each measures 251 requests after three primes.

|Case|Variant|Samples|Profile CPU ms/request|Epoch %|Scheduler %|G1 %|G2 %|
|---|---|---:|---:|---:|---:|---:|---:|
|1x2-w32|accepted|9327|192.511|9.847|3.479|38.805|28.448|
|1x2-w32|fresh|9394|193.093|9.407|3.734|36.363|30.462|
|1x2-w32|candidate|9440|192.535|10.853|3.572|36.559|29.976|
|2x2-w32|fresh|12321|267.471|11.948|4.782|32.427|23.683|
|2x2-w32|candidate|12111|261.914|10.926|4.638|33.470|24.424|
|2x2-w32|accepted|11861|262.484|11.133|5.070|33.246|25.464|

## Supplied M5 references

Historical supplied CPU/Metal values, not fresh Apple measurements. No 54→1 reference was supplied. Candidate values are experimental.

|Shape|Candidate ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required CPU10× ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.321766|37|4.446×|24|2.884×|3.7|
|2x2|11.444545|59|5.155×|30|2.621×|5.9|
|4x2|15.803015|109|6.897×|48|3.037×|10.9|
|8x2|24.169805|189|7.820×|74|3.062×|18.9|
|12x2|34.125382|284|8.322×|101|2.960×|28.4|
|16x2|40.233292|335|8.326×|124|3.082×|33.5|
|24x2|67.022116|528|7.878×|176|2.626×|52.8|
|merge-24x1|66.105108|507|7.670×|175|2.647×|50.7|

## Validation and scope

Fresh and candidate each pass all 116 native core tests, two raw-image and six cubic tests. Existing exact phi decomposition and complete subgroup-query generator oracles cover the unchanged per-scalar arithmetic. Candidate portable/no-IFMA consumer checks pass. Field source and the accepted 33-test native qualification are inherited; no fresh field-suite execution is claimed. The edit adds no allocations or storage and retains conservative forecasts, but scheduling can affect live allocations, queue reclamation and RSS. No minimum RAM or pressure benefit is established.

Nine four-worker candidate preflights add 36 SDK calls/nine exports; latency adds 26,100/450; profiles 1,524/six. Total **27,660 SDK calls, 465 distinct independently Gnark-verified exports and 930 proof/public members**, ledger **10,180→10,645**. All proofs are 164 bytes, fixture-bound and fresh against all preceding/new exports; every SDK call verifies internally. Source, library, executable, input and proof pins are rehashed at closure.

The first preparation helper uses an incorrect manifest field name after completing the two snapshots and exits before workflow construction, build, tests or SDK execution. A separate retained helper completes pins and the plan using those same snapshots. No prover change or successful test/capture is repeated for that repair. Original helper and correction remain archived.

The first offline closure cannot read six root-owned, mode-0600 raw captures created by privileged perf. Changing ownership of exactly those files permits the unchanged close helper to hash them; permissions and capture bytes are preserved. No SDK capture or verifier is repeated. The [closure repair](artifacts/closure-failure.json) is retained.

This local study creates no AWS resources and makes no new Aeglos, phone, ARM, TEE, swap, minimum-RAM, pressure, concurrent-throughput or huge-circuit claim. Any performance pass still requires a separate compatibility/concurrency/resource review before adoption. The original all-eight CPU10× objective remains active.

[Protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [raw rows](artifacts/performance/rows.json), [source edit](artifacts/edits.json), [native workflow](artifacts/native/result.json), [proof inventory](artifacts/proof-inventory.json), [raw profile hashes](artifacts/raw-local-only-sha256.json), [closeout](artifacts/closeout.json), [artifact hashes](artifact-index.json).
