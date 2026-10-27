# Parallel exact scalar grouping

Complete SDK performance gate: **false**. The private candidate takes **8.305753 ms** on primary, versus **8.328096 ms accepted** and **8.288431 ms fresh**. Paired candidate/accepted is **0.998793487**, individual median interval **[0.994910999, 1.002366556]**, 10/15 wins, with a median paired saving of **10.018 µs**. Candidate/fresh is **1.003904581**. Positive estimates are retained without a gain floor. Displayed medians and paired estimates have different aggregation.

The candidate is not adopted in this experiment. The exact accepted 171-file implementation and local 824164 SDK remain unchanged. The original all-eight supplied M5 CPU 10× goal remains active, achieved 0/8.

## Change

The candidate stably partitions nonzero scalar indices by their low three bits and builds eight disjoint exact maps in the existing Rayon pool. Each map records the original first index. Summed cardinality preserves the existing cutoff; reconstruction traverses original positions to reproduce the exact values, counts, starts and stable input-index lists. The original sample, zero-aware policy and initial-capacity rule remain. Small inputs and single-worker pools retain the original code. No point arithmetic, scalar representation, bucket geometry, key preparation, queue implementation or persistent table changes. One original file changes and one private module is added; the other 170 originals are exact. Inverse edits recover all 171 production files.

The temporary index/representative buffer uses eight bytes per nonzero input. Before allocating it, checked bounds include the sample set, all map-growth envelopes, descriptors and reconstruction arrays; both peaks must fit the existing grouping reservation. Otherwise the untouched reservation returns to the serial fallback. Maps drop before reconstruction and pairs before final list allocation. Consumer layout and inherited bounds remain. This is no universal RSS bound; allocator retention, stacks, concurrent callers and actual pressure still need qualification. See the [memory review](artifacts/memory-review.md). There is no new unsafe code, runtime flag, ISA setting, worker pool, synthetic universal cap or shard-count sweep.

Retained stage recordings showed serial primary A/G2 grouping maps that ultimately decline. This prototype targets their critical serial work rather than task-search policy. It differs from earlier exact-prefix/bitmap/borrowed-key trials; their original outcomes remain. No new C7a latency or causal prediction follows from those older recordings. The reviewed thesis clustering mechanism overlaps an already unfavorable search census and supplies no new performance claim here. The [research audit](artifacts/research-audit.md) also corrects an older design-only entry using completed negative shared-point experiments.

## Full SDK results

Ryzen 9950X, fifteen fixed rotated/reversed fresh-process blocks. All nine shapes use 32 workers on CPUs 0–31; primary also uses 16 workers on physical CPUs 0–15. Both primary cases use 151 requests after three primes, other cases 31 after three. All 450 latency processes finish before six separate warm profiles. Loading/setup/primes are excluded from latency and included in lifetime RSS. Fresh solving, randomness, proving, internal SDK verification and serialization are timed. No owned build, helper, hash, archive, independent verifier or second benchmark overlaps captures. No successful capture is excluded, extended or repeated.

Times are medians of process medians; ratios and savings pair blocks. RSS is the maximum whole-process observation, not minimum RAM. Individual intervals assume independent/exchangeable blocks and do not provide familywise coverage.

|Case|Accepted ms|Fresh ms|Candidate ms|Paired C/A|Individual interval|Wins|Saving µs|RSS A/C MiB|
|---|---:|---:|---:|---:|---|---:|---:|---:|
|1x2-w32|8.328096|8.288431|8.305753|0.998793487|[0.994910999, 1.002366556]|10/15|10.018|136.75/85.25|
|2x2-w32|11.498399|11.387248|11.428048|0.995448062|[0.988824045, 1.001787402]|11/15|52.340|276.00/228.00|
|4x2-w32|16.060417|15.923239|15.861382|0.987734375|[0.979842384, 1.002377958]|11/15|196.161|314.00/291.75|
|8x2-w32|24.051948|24.183456|24.602121|1.025162162|[1.014201696, 1.031906021]|2/15|-601.806|517.75/485.00|
|12x2-w32|34.007873|33.665438|34.767947|1.027144319|[1.020584464, 1.033479887]|1/15|-917.111|707.00/672.75|
|16x2-w32|40.032346|40.027940|41.847474|1.043492730|[1.038468757, 1.049825189]|0/15|-1741.116|808.00/829.25|
|24x2-w32|67.146308|67.254070|71.086713|1.057225706|[1.051746530, 1.068878550]|0/15|-3872.968|1312.03/1253.24|
|merge-24x1-w32|65.828410|66.010879|69.280526|1.054182697|[1.042761507, 1.062293927]|0/15|-3563.015|1168.59/1183.50|
|merge-54x1-w32|163.467660|163.338347|176.044468|1.078055755|[1.072636894, 1.085730401]|0/15|-12748.168|1903.69/1891.14|
|1x2-w16|10.228112|10.085646|10.021421|0.994565475|[0.922291611, 1.049518462]|10/15|54.653|92.00/67.50|

Primary peak RSS falls from 136.75 to 85.25 MiB, with smaller peaks in several other cases. This is mixed: 16×2 and 24→1 have higher candidate peaks. The memory observations do not compensate for the speed regressions. Primary's positive 10.018 µs paired estimate against accepted has an interval crossing parity; against fresh it loses 32.451 µs, winning 4/15 blocks. Merge54 regresses 7.81% against accepted, losing all blocks. Partition passes, concurrent maps, reconstruction and allocator behavior are possible costs, not isolated causal explanations.

All fresh/control pairs, CPU observations and every block are retained in analysis.json. Original-eight paired geometric aggregates:

|Comparison|Median ratio|Individual interval|
|---|---:|---|
|candidate/accepted|1.023765332|[1.020599432, 1.025811724]|
|candidate/fresh|1.025417817|[1.023010271, 1.027720774]|

## Frozen performance qualification

Every fresh/accepted individual median interval must lie wholly within factor 1.03. Candidate upper bounds must be ≤1.02 against both controls in every case. Primary32 must have upper <1 against both; the original-eight within-block geometric aggregate must have upper ≤1 against both. Fifteen blocks use ranks 4..12, with 96.484375% nominal individual coverage under independent/exchangeable-block assumptions. There is no minimum gain floor or post-result exclusion, threshold change or unchanged rerun. Known physical16 variability remains visible. The earlier adopted source's engineering exception is unchanged and does not waive this study's checks.

calibration_failures: `["1x2-w16 fresh/accepted interval [0.9310725581148819,1.010957964637294]"]`.

regressions: `["12x2-w32 candidate/accepted upper=1.0334798873444382","12x2-w32 candidate/fresh upper=1.0397007801090017","16x2-w32 candidate/accepted upper=1.0498251886754826","16x2-w32 candidate/fresh upper=1.055680091469326","1x2-w16 candidate/accepted upper=1.0495184620246099","1x2-w16 candidate/fresh upper=1.0459441730489936","24x2-w32 candidate/accepted upper=1.0688785502175713","24x2-w32 candidate/fresh upper=1.0652426316931254","8x2-w32 candidate/accepted upper=1.0319060207381716","8x2-w32 candidate/fresh upper=1.0277784930267093","merge-24x1-w32 candidate/accepted upper=1.0622939271993785","merge-24x1-w32 candidate/fresh upper=1.0594766062513434","merge-54x1-w32 candidate/accepted upper=1.0857304007410395","merge-54x1-w32 candidate/fresh upper=1.088256505077268"]`.

primary_gain_failures: `["1x2-w32 candidate/accepted upper=1.0023665557289745","1x2-w32 candidate/fresh upper=1.0066205092998683"]`.

aggregate_failures: `["candidate/accepted original8 upper=1.0258117242547697","candidate/fresh original8 upper=1.0277207744828993"]`.

## CPU and profiles

Warm user+system CPU is summed across threads per request. Profile shares are sampled user-cycle weights with inlining/attribution limits, not removable wall-time fractions or exclusive critical-path costs. Profiles occur after latency and do not replace its acceptance observations.

|Case|Variant|CPU ms/request|Whole-process RSS MiB|
|---|---|---:|---:|
|1x2-w32|accepted|191.087|136.75|
|1x2-w32|fresh|191.088|135.75|
|1x2-w32|candidate|191.677|85.25|
|2x2-w32|accepted|265.071|276.00|
|2x2-w32|fresh|264.331|275.50|
|2x2-w32|candidate|264.956|228.00|
|merge-54x1-w32|accepted|4471.040|1903.69|
|merge-54x1-w32|fresh|4483.314|1848.53|
|merge-54x1-w32|candidate|4888.445|1891.14|
|1x2-w16|accepted|123.992|92.00|
|1x2-w16|fresh|124.258|91.50|
|1x2-w16|candidate|124.523|67.50|

The six separate 199 Hz profiles contain 84048 samples, zero lost records and one sampled SDK PID each. Each measures 251 requests after three primes.

|Case|Variant|Samples|Profile CPU ms/request|Epoch %|Scheduler %|G1 %|G2 %|
|---|---|---:|---:|---:|---:|---:|---:|
|1x2-w32|accepted|9393|193.651|9.953|3.758|35.232|30.184|
|1x2-w32|fresh|9475|192.515|9.538|3.729|35.952|29.972|
|1x2-w32|candidate|9480|192.850|10.456|4.424|36.014|30.481|
|4x2-w32|fresh|18353|414.945|10.579|5.183|34.634|18.284|
|4x2-w32|candidate|18667|417.618|10.725|5.110|34.808|19.376|
|4x2-w32|accepted|18680|418.077|10.499|4.918|35.362|18.420|

The original category matcher omits the new `grouping::parallel` path. In the candidate, 0.793% of primary and 0.834% of 4×2 sampled cycle weight explicitly names that closure but lands in `other`. This is only visible symbol weight; inlining prevents a complete stage estimate. The small original `grouping` category therefore does not show that grouping work disappeared. The [offline audit](artifacts/profile-category-audit.json) preserves original captures and categories without remeasurement or recategorization.

## Supplied M5 references

Historical supplied CPU/Metal values, not fresh Apple measurements. No 54→1 reference was supplied. Candidate values are experimental.

|Shape|Candidate ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required CPU10× ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.305753|37|4.455×|24|2.890×|3.7|
|2x2|11.428048|59|5.163×|30|2.625×|5.9|
|4x2|15.861382|109|6.872×|48|3.026×|10.9|
|8x2|24.602121|189|7.682×|74|3.008×|18.9|
|12x2|34.767947|284|8.168×|101|2.905×|28.4|
|16x2|41.847474|335|8.005×|124|2.963×|33.5|
|24x2|71.086713|528|7.428×|176|2.476×|52.8|
|merge-24x1|69.280526|507|7.318×|175|2.526×|50.7|

## Validation and scope

Fresh passes all 116 native core tests and candidate 119, including three new parallel-grouping tests over exact stable metadata, cutoff boundaries, adversarial shard populations, zero/full-word inputs, one/two/four-worker pools, nested/concurrent callers and checked memory/overflow fallback. Both pass two raw-image and six cubic tests; candidate portable/no-IFMA consumers pass. Existing complete-curve grouping oracles remain unfiltered. Field source and the accepted 33-test native field qualification are inherited, with no fresh field-suite claim. Ordinary RSS and the conservative workspace review do not establish minimum RAM, pressure or concurrent-server behavior.

Nine four-worker candidate preflights add 36 SDK calls/nine exports; latency adds 26,100/450; profiles 1,524/six. Total **27,660 SDK calls, 465 distinct independently Gnark-verified exports and 930 proof/public members**, ledger **10,645→11,110**. All proofs are 164 bytes, fixture-bound and fresh against all preceding/new exports; every SDK call verifies internally. Source, library, executable, input and proof pins are rehashed at closure.

All initial setup, build and observer receipts are retained. No successful capture, test or verifier is repeated as part of analysis or publication. Raw perf.data files remain local with hashes; privileged capture ownership is transferred to the experiment owner after measurement for reading, without altering bytes or mode.

This local study creates no AWS resources and makes no new Aeglos, phone, ARM, TEE, swap, minimum-RAM, pressure, concurrent-throughput or huge-circuit claim. Any performance pass still requires a separate compatibility/concurrency/resource review before adoption. The original all-eight CPU10× objective remains active.

[Protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [raw rows](artifacts/performance/rows.json), [source edit](artifacts/edits.json), [native workflow](artifacts/native/result.json), [proof inventory](artifacts/proof-inventory.json), [raw profile hashes](artifacts/raw-local-only-sha256.json), [closeout](artifacts/closeout.json), [artifact hashes](artifact-index.json).
