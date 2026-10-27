# G1 temporary arrays of references

The reference-staging candidate establishes **no complete-proof gain** and is not adopted. Primary32 measures **8.468905 ms**, versus accepted **8.379486 ms** and fresh **8.340873 ms**. Paired median ratios are **1.007710 / 1.011824**, with3/9and2/9wins; both primary intervals include parity. The original-eight aggregate intervals also include parity. Several nonregression bounds and the physical16 control-calibration bound fail. Positive estimates for other cases remain in the results below.

The adopted combined implementation and exact accepted824164SDK remain unchanged. This experiment adds no field primitive, intrinsic, table, cache, pool or flag. The original all-eight supplied CPU10× goal remains active/unmet0/8.

## Change and actual SDK code

Only stream_g1_impl in msm.rs changes: two [Affine;LANES] staging arrays become [&Affine;LANES] in each of the two existing forward-record storage modes. Both still feed the original FqBatch constructor; one Borrow call per operand/lane, index/sign accesses, field arithmetic, inverse order, record writes, tails and outputs are preserved. Safe references end in each synchronous group. There is no new unsafe code or heap allocation. All170other accepted source files remain exact; reversing the recorded four edits recovers all171accepted bytes. This concerns temporary arithmetic input staging, separate from adopted borrowed pending/collision records and prior G2 index/tree gather trials.

Static inspection uses the actual standard SDK binaries, not isolated packing wrappers. All five fresh/accepted normalized stream instruction shapes agree. Candidate shapes differ, but arithmetic-site counts do not fall. The table shows two canonical monomorphs; counts include the whole function, not just a runtime loop.

|Canonical stream|Variant|Text bytes|Instructions|Stack-reference sites|Memory-operand sites|IFMA sites|
|---|---|---:|---:|---:|---:|---:|
|owned|accepted|26282|4423|591|1281|800|
|borrowed|accepted|26230|4428|589|1284|800|
|owned|candidate|26369|4421|563|1288|800|
|borrowed|candidate|26241|4429|592|1286|800|

Three normalizer tests retain stack offsets, registers, constants and local branch destinations while normalizing RIP displacements and external symbol addresses. Matching shapes do not prove identical global referents or dynamic behavior. Static stack-reference counts are not dynamic spill counts or stack/RSS savings. Replacing a source-level point array does not establish that actual copies were removed or identify a cause for the measured result.

## Verified complete SDK requests

Ryzen9950X, nine rotated/reversed fresh-process blocks; all nine shapes at32workers plus primary16 on CPUs0–15. Exact accepted824164, source-identical fresh control and one-file candidate use the same standard driver/support and native flags. Each32-worker process runs31fresh verified SDK requests after3primes; physical16 runs151after3. Witness solving, fresh blinding/proving, SDK verification and serialization are timed; loading/primes are excluded. All270latency processes finish before six separate warm profiles. No short screen, extension, successful repeat, exclusion or postcapture source/flag change occurs.

Times are medians of process medians. Ratios, savings, wins and intervals use within-block pairs; differences or ratios of displayed medians need not agree. RSS is the maximum lifetime process peak across blocks, including loading/primes.

|Case|Accepted ms|Fresh ms|Candidate ms|Paired /accepted|Individual median interval|Wins /accepted|Paired saving µs|Accepted / candidate RSS MiB|
|---|---:|---:|---:|---:|---|---:|---:|---:|
|1x2-w32|8.379486|8.340873|8.468905|1.007710|[0.994633, 1.026071]|3/9|-64.823|112.75 / 110.25|
|2x2-w32|11.514329|11.465648|11.492008|1.001281|[0.990447, 1.009509]|4/9|-14.618|271.50 / 265.25|
|4x2-w32|16.080934|15.965606|15.928726|0.991726|[0.972891, 1.013063]|5/9|132.059|312.75 / 311.25|
|8x2-w32|24.028846|24.179791|24.369048|1.018775|[0.991720, 1.026828]|2/9|-449.107|517.75 / 509.50|
|12x2-w32|34.108489|33.882784|34.020013|0.996469|[0.984244, 1.016105]|5/9|120.436|714.00 / 714.50|
|16x2-w32|40.041681|40.100563|40.102747|0.999819|[0.996013, 1.013612]|5/9|7.263|831.50 / 832.00|
|24x2-w32|67.244175|67.283719|67.245707|0.996920|[0.987942, 1.005067]|5/9|205.709|1285.39 / 1295.80|
|merge-24x1-w32|66.183164|65.766548|66.155943|0.999219|[0.995907, 1.010451]|6/9|51.576|1191.96 / 1135.93|
|merge-54x1-w32|164.598097|162.975977|164.192531|1.001591|[0.989781, 1.013550]|4/9|-259.670|1847.52 / 1846.78|
|1x2-w16|10.303266|10.122895|10.317271|0.984324|[0.921090, 1.028904]|5/9|161.083|90.25 / 90.00|

Primary32 candidate/accepted interval[0.994633,1.026071] and candidate/fresh[0.997064,1.017231] do not establish an improvement; their paired point estimates are0.77%/1.18%slower. Original-eight aggregate medians1.003491/1.005158 have intervals[0.994612,1.008147]/[0.995196,1.009576].

Positive paired estimates are retained:4x2 saves132.059µs versus accepted,12x2 saves120.436µs and24x2 saves205.709µs. Their intervals include parity, and the4x2paired ratio versus fresh is1.007400 despite its lower displayed candidate median. Merge24 saves51.576µs versus accepted but is slower against fresh, with individual interval[1.000248,1.009783] above parity. These are not called zero or discarded; they do not justify adopting this candidate globally.

Primary32 aggregate CPU time/request is192.026ms accepted,190.823fresh and191.187candidate. The lower CPU point estimate against accepted does not reproduce against fresh; it is not a demonstrated reduction in CPU work. RSS is also mixed: primary112.75/110.25MiB,24x21285.39/1295.80MiB and54 1847.52/1846.78MiB. No universal memory saving or minimumRAM is established.

## Frozen gate and profiles

The individual distribution-free median intervals have96.09375%nominal coverage for nine independent/exchangeable blocks (ranks2..8), not familywise coverage. Frozen requirements: all10fresh/accepted intervals wholly within factor1.03; candidate upper≤1.02 against both controls in all10; primary32 upper<1 against both; original-eight geometric aggregate upper≤1 against both. No gain floor or failed-bound waiver is applied. An unmet bound does not by itself prove a causal slowdown.

Calibration failures: `["1x2-w16 fresh/accepted interval [0.9152505693835518,1.0051651906780315]"]`.

Nonregression failures: `["1x2-w16 candidate/accepted upper=1.028904207026318","1x2-w16 candidate/fresh upper=1.042897389193432","1x2-w32 candidate/accepted upper=1.0260713988445695","8x2-w32 candidate/accepted upper=1.0268281319544914","8x2-w32 candidate/fresh upper=1.024185390684364"]`.

Primary gain failures: `["1x2-w32 candidate/accepted upper=1.0260713988445695","1x2-w32 candidate/fresh upper=1.017231075697211"]`.

Aggregate failures: `["candidate/accepted original8 upper=1.0081465209241964","candidate/fresh original8 upper=1.0095762596832165"]`.

Six warm199Hzcycles:u profiles run after all latency,251requests after3primes each:85,134samples, one sampled SDK PID each and zero lost records. Their per-request CPU and period-weighted categories are descriptive; different process runs and sampling/inlining prevent interpreting shares as exclusive wall cost, dynamic copy/spill counts or a cache cause.

|Case|Variant|Samples|CPU ms/request|G1 arithmetic %|Epoch %|
|---|---|---:|---:|---:|---:|
|1x2-w32|accepted|9126|191.315|35.57|8.62|
|1x2-w32|fresh|9355|191.984|36.77|8.74|
|1x2-w32|candidate|9411|192.445|37.43|10.10|
|4x2-w32|fresh|19283|428.937|34.77|10.29|
|4x2-w32|candidate|19239|425.976|33.39|10.97|
|4x2-w32|accepted|18720|416.042|34.16|10.19|

## Supplied M5 references

Historical supplied CPU/Metal figures are not fresh Apple measurements. Candidate numbers remain experimental; no54reference was supplied.

|Shape|Candidate ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required ms for CPU10×|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.468905|37|4.369×|24|2.834×|3.7|
|2x2|11.492008|59|5.134×|30|2.611×|5.9|
|4x2|15.928726|109|6.843×|48|3.013×|10.9|
|8x2|24.369048|189|7.756×|74|3.037×|18.9|
|12x2|34.020013|284|8.348×|101|2.969×|28.4|
|16x2|40.102747|335|8.354×|124|3.092×|33.5|
|24x2|67.245707|528|7.852×|176|2.617×|52.8|
|merge-24x1|66.155943|507|7.664×|175|2.645×|50.7|
|54→1|164.192531|—|—|—|—|—|

## Correctness, provenance and next step

Fresh control and candidate each pass all116unfiltered native core tests and raw2/cubic6; retained independent G1 signed-stream/tail/reuse/owned/borrowed/fused/ordinary generator oracles cover the edit. Candidate portable and noIFMA consumer checks pass. Field sources and field qualification remain unchanged; no new field-suite, portable-unit, pressure or huge-circuit qualification is claimed here.

The initial native workflow completed fresh116core/prover8/libraries, then stopped before invoking rustc because the standalone SDK driver/support are separate manifest fields and absent from the copied171library/source files. A new repaired workflow links the existing exact pinned driver/support and reuses those completed tests/libraries unchanged; it runs the remaining candidate tests/builds. Original plan/failure/source and repair receipts are retained. No arithmetic source, compiler flag, successful test or request capture was rerolled for the repair.

All nine four-worker candidate preflights pass36SDKcalls/nine independent exports. Latency adds12,420calls/270exports and profiles1,524calls/six exports. Total **13,980SDKcalls**, **285distinct independently Gnark-verified exports**, **570proof/public members**, ledger **9,154→9,439**. Every SDK call verifies internally; all final exports meet164-byte length, exact public binding and freshness against all prior/new identities. No helper/build/hash/archive/independent verifier or second owned CPU workload overlaps timing/profiles. Closing audits rehash all pins/proof members and all171production source bytes; exact824164acceptedSDK remains unchanged.

Do not adopt or retune reference-array staging, other pointer construction syntax or incidental code layout from these results. The source-array appearance did not establish the expected CPU reduction in the real caller. Pursue a distinct algorithmic hypothesis with a concrete CPU-work argument. Existing Montgomery-first grouping and prior failed bitmap/idle-yield/empty-victim policies are not new directions. The adopted3.13%combined gain remains in place. No new AWS/Aeglos/phone/ARM/TEE/large-circuit/pressure/minRAM/throughput result is claimed; previous C7a comparisons remain previous-source measurements.

[Design](artifacts/design.md), [SDK protocol](artifacts/sdk-design.md), [source audit](artifacts/source-audit.json), [code generation](artifacts/codegen.json), [analysis](artifacts/analysis.json), [native repair](artifacts/native-repair.json), [proof inventory](artifacts/proof-inventory.json), [raw local profile hashes](artifacts/raw-local-only-sha256.json), [artifact hashes](artifact-index.json).
