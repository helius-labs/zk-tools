# Combined borrowed G1, adaptive G2 and Poseidon formulas

The combined candidate improves primary32 from **8.653 ms to 8.372 ms**, with a paired median reduction of **3.13%** against accepted and **3.59%** against fresh. It wins all **15/15** paired blocks against each control. The original-eight geometric aggregate improves **1.98% / 2.21%**, also winning every block against each control. These are measured gains; small reliable gains have no minimum cutoff.

Full performance gate: **false**, solely because the physical16 fresh/accepted control interval exceeds the declared calibration range. All candidate nonregression, primary-gain and aggregate checks pass. Supplied CPU10× rows: **0/8**. Source adoption remains pending calibration and resource qualification.

This is a new combination of three previously measured implementations. [Borrowed G1 operands](../2026-10-26-borrowed-g1/README.md) improved primary32 but failed physical16 nonregression. [Adaptive G2 records](../2026-10-27-adaptive-tree/README.md) retained primary and large-shape gains but failed calibration and4×2 checks. [Direct width-three Poseidon](../2026-10-27-poseidon-direct-dot/README.md) retained positive estimates but failed physical16 qualification. Those study verdicts remain unchanged. This prospective combination is neither an unchanged rerun nor a waiver of their failures. The rejected two-group G1 reverse overlap is absent.

## Implementation and memory

G1 pending and collision records borrow existing point slices through safe synchronous lifetimes, retaining the original owned fallback and nativeIFMA/deferred-sign/packed/direct plus geometry guards. G2 tree layers use the earlier fixed32actual-ordinary-pair rule with prefix/norm records; other layers retain original arithmetic. Width-three recognized Poseidon rows use the existing fused two-product operation and an ordinary third product; all other widths and original buffer writes remain. Field primitives, point order, tree/query/task topology, pool, key/image format and fresh blinding remain unchanged.

The composition changes four of170originals and adds one private module. Reversing every recorded patch reproduces all170accepted files byte-for-byte. Borrowed production arithmetic is checked against its parent after removing two experimental selector checks and formatting; no public probe or diagnostic reference copy is added. Forecasts conservatively retain the original owned-window payload estimates and explicitly charge the additional vector headers using checked arithmetic. There is no new point table, copied key, duplicate solver plan, proof/witness cache or circuit/core selector.

[Memory argument](artifacts/record-memory-argument.md) separates retained payload bounds from allocator/transient/process peaks. The G2 retained two-buffer payload fits the prior record bound under the pinned implementation; its extra header is separately charged. Borrowed entries are16bytes versus72owned on64-bit builds and omit unused scalar arrays, without taking that saving as lower admission charges. None of these layout facts is a minimum-RAM, universal pressure or process-RSS guarantee.

## Verified request latency

Ryzen9950X, fifteen rotated/reversed fresh-process blocks over all nine shapes at32workers plus primary16 on CPUs0–15. Exact accepted and exact reused fresh baseline SDKs retain source/library/consumer/binary pins; only the combined candidate is newly built with the standard native fat-LTO/one-codegen-unit driver. Each32-worker process runs31timed fresh SDK requests after3primes; physical16 runs151after3. Timed work includes witness solving, fresh blinding/proving, SDK verification and serialization, excluding loading/primes. All450latency processes precede six separate profiles. There is no short screen, extension, successful repeat, exclusion or postcapture source change.

Times are medians of process medians; ratios, savings and intervals use within-block observations. Ratios/differences of displayed medians need not match paired estimates. RSS is the maximum lifetime process peak across blocks, including loading/primes; CPU includes coordination.

|Case|Accepted ms|Fresh ms|Combined ms|Paired /accepted|Individual median interval|Wins /accepted|Paired saving µs|Accepted / combined RSS MiB|Combined CPU ms/request|
|---|---:|---:|---:|---:|---|---:|---:|---:|---:|
|1x2-w32|8.653411|8.704628|8.372140|0.968653|[0.964835, 0.982140]|15/15|271.412|114.50 / 114.75|191.815|
|2x2-w32|11.748332|11.787626|11.444740|0.975281|[0.968618, 0.984413]|15/15|292.011|273.75 / 271.75|263.390|
|4x2-w32|16.160088|16.210183|16.088793|0.987697|[0.984909, 1.011876]|9/15|199.217|313.75 / 314.75|422.302|
|8x2-w32|24.714464|24.837886|23.986009|0.973851|[0.960630, 0.995478]|13/15|648.463|523.50 / 516.00|664.581|
|12x2-w32|34.637390|34.553623|33.931699|0.985598|[0.964121, 0.994428]|15/15|493.220|721.50 / 706.50|937.087|
|16x2-w32|40.812446|41.070628|40.287411|0.985305|[0.971673, 0.997289]|12/15|594.762|853.50 / 812.00|1125.559|
|24x2-w32|68.503602|68.624285|67.228752|0.982660|[0.976938, 0.989950]|15/15|1183.093|1288.11 / 1337.31|1924.465|
|merge-24x1-w32|67.641491|67.457441|66.259942|0.976213|[0.964242, 0.986633]|15/15|1621.750|1210.34 / 1184.11|1832.864|
|merge-54x1-w32|175.446051|176.983299|163.533643|0.933918|[0.920701, 0.937457]|15/15|11610.682|1925.09 / 1838.41|4485.732|
|1x2-w16|10.617007|10.735592|10.070846|0.960044|[0.938349, 0.990891]|12/15|421.435|91.50 / 89.25|125.023|

## Frozen qualification

Individual distribution-free median intervals have at least95% nominal coverage under independent/exchangeable block assumptions. Fifteen-block endpoints4..12 have96.484375% coverage under those assumptions; no simultaneous/familywise guarantee is claimed. Fifteen blocks were specified before captures for this distinct implementation, not added to closed data. Fresh/accepted calibration intervals must lie wholly within factor1.03 in all10cases. Combined nonregression upper bounds must be≤1.02 against both controls in all10. Primary32 strict gain upper bounds must be<1 against both. Original-eight within-block geometric aggregate upper bounds must be≤1 against both. Primary16 requires nonregression only. No positive-gain floor or failed-bound waiver is used. Unmet bounds can reflect insufficient precision and do not by themselves prove a causal slowdown. Performance success still requires resource/pressure/compatibility qualification before adoption.

Calibration failures: `["1x2-w16 fresh/accepted interval [0.9908907472699228,1.0678053272258377]"]`.

Nonregression failures: `[]`.

Primary gain failures: `[]`.

Original-eight aggregate failures: `[]`.

Original-eight aggregate intervals: `{"composed/accepted":{"all_values":[0.9819777630571928,0.9761748402932788,0.9843288689091908,0.9732239212908024,0.9822274069424356,0.9734399912915804,0.9792275174533974,0.988560509546976,0.9783278190174086,0.984846767354414,0.9807582111721512,0.9778914772751875,0.9852323776346534,0.9758828499549456,0.9801709642344087],"coverage_under_independent_exchangeable_blocks":0.96484375,"lower":0.9761748402932788,"median":0.9801709642344087,"n":15,"rank_one_based":4,"upper":0.9843288689091908},"composed/fresh":{"all_values":[0.9779486128956904,0.980335396572307,0.9797723162412796,0.9755902948699198,0.9729027763705808,0.978829586209776,0.9856113434222066,0.9793328466808734,0.9756465844445076,0.9815198102190152,0.9748743787899248,0.9688668031967048,0.9776682775791364,0.9671325342292292,0.985835478577514],"coverage_under_independent_exchangeable_blocks":0.96484375,"lower":0.9748743787899248,"median":0.9779486128956904,"n":15,"rank_one_based":4,"upper":0.980335396572307}}`.

[Analysis](artifacts/analysis.json) retains every paired ratio, saving, interval, block and failure. Positive estimates remain alongside adverse cases.

The earlier adaptive-G2 study's **31.530 µs** primary saving is **0.031530 ms**, about **0.37%** of an 8.6 ms proof. Its nine paired primary wins remain a positive result despite that study's separate failed checks. No minimum effect-size cutoff excludes it here.

The combined 54→1 candidate improves **6.61%** against accepted, with 15/15 paired wins. Lifetime RSS varies by shape: primary is approximately unchanged, 24×2 increases from **1,288.11 to 1,337.31 MiB**, and 54→1 decreases from **1,925.09 to 1,838.41 MiB**. This does not establish a universal memory reduction.

The postcapture [physical16 control audit](artifacts/physical16-control-audit.json) verifies identical native compiler flags for all three variants and retains all 45 physical16 process rows. Slow processes occur in all three variants; their CPU time and switch counts do not identify a cause. No physical16 CPU profile was captured. The calibration failure remains recorded, with no exclusions, unchanged rerun or retrospective waiver.

## Supplied M5 references

Historical supplied CPU/Metal figures are not fresh Apple measurements. No54→1reference was supplied. Candidate values are experimental.

|Shape|Combined ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required ms for CPU10×|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.372140|37|4.419×|24|2.867×|3.7|
|2x2|11.444740|59|5.155×|30|2.621×|5.9|
|4x2|16.088793|109|6.775×|48|2.983×|10.9|
|8x2|23.986009|189|7.880×|74|3.085×|18.9|
|12x2|33.931699|284|8.370×|101|2.977×|28.4|
|16x2|40.287411|335|8.315×|124|3.078×|33.5|
|24x2|67.228752|528|7.854×|176|2.618×|52.8|
|merge-24x1|66.259942|507|7.652×|175|2.641×|50.7|
|merge-54x1|163.533643|—|—|—|—|—|

## Correctness and scope

All116native core tests pass, including retained independent borrowed G1 signed-stream/ordinary/KH generator, owned-control, collision, tail, dispatch and reuse oracles; G2 complete-curve/boundary/retained-capacity checks; and all-width Poseidon symbolic/full-buffer/error comparisons. Raw2/cubic6, generator and portable/no-IFMA consumer checks pass. Field source and its accepted qualification are unchanged. The clean selectors, combined source and header forecasts are tested together. Initial source-audit preparation stopped before any native build on formatting normalization and an extraction newline; the archived setup revisions were repaired before native testing/captures, without changing arithmetic.

All nine candidate shapes pass four-worker preflight, one request after three primes:36SDKcalls/nine fresh independently Gnark-verified exports. The fixed latency matrix adds20,700calls/450exports. After all latency, primary32 and4×2 profiles run all three variants,251requests after3primes:1,524calls/six exports. Total **22,260SDK calls**, **465distinct independently verified exports**, **930proof/public members**; ledger **8,595→9,060**. Every export has164-byte proof length, pinned public binding, freshness against all prior/new identities and independent Gnark verification; every SDK call also verifies. Subinventories are not counted twice.

Six warm199Hzcycles:u profiles follow all latency, each sampling one own process with zero lost records: **86,772 samples**, comprising 28,611 primary32 and 58,161 for 4×2. [Rows](artifacts/performance/rows.json) retain decoded categories, symbols and process receipts; raw profiles stay local with hashes. Period shares are not removable proof-wall fractions or cache/allocator causes. No helper/build/hash/archive/verifier overlaps a timed capture. Closing audits rehash all source/library/consumer/helper/tool/control pins and proof/public members. All170production sources and the exact accepted SDK remain unchanged.

Lifetime RSS is not incremental workspace or minimumRAM. Requested-capacity arguments do not establish transient allocator, stack, cgroup, giant-key or mobile bounds. No new AWS/Aeglos, phone, TEE,193GiB,swap,pressure,concurrent-caller,throughput or minimum-RAM qualification is claimed by this latency report.

accepted executable: `/tmp/zopro-grouping-module/native/candidate-sdk`; SHA-256 `03e1260e89068c2bec37a2471c34e93628a9f081903e1b7eee9cab9c306aaf3d`.

fresh executable: `/tmp/zopro-poseidon-fused-dot/native/fresh-sdk`; SHA-256 `d847ae9fd2ae52c6c74172a95004780eaa5481ca2a5f8889d7c9d0063dcd1eaa`.

composed executable: `/tmp/zopro-composed-cpu/native/composed-sdk`; SHA-256 `8241648f298893953bcacac515d3d52a7f2d363cb264e08d232af3037decb265`.

[Design](artifacts/design.md), [source recovery](artifacts/source-audit.json), [composition edits](artifacts/composition-edits.json), [native tests](artifacts/native/core/tests.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).
