# Early canonical output X in MSM addition

The fixed actual-caller screen **passes**. The all-context aggregate improves **0.320% versus fresh OFF** and **0.269% versus frozen**; the six logical-32 contexts improve **0.597% and 0.443%**, respectively. Each aggregate wins 14/15 blocks. All control-calibration and candidate nonregression checks pass. This selects a separate standard-SDK proof trial; production is unchanged and no proving-time improvement or adoption is established.

## Change and arithmetic bounds

The original reverse addition computes lazy output X, uses generic subtraction for Ax−X in Y, and canonicalizes X during host export. The private candidate performs that same final p reduction before Y, retains canonical X in a typed native-vector wrapper, and reuses it in the canonical-input subtraction. With Ax,X<p, p+Ax−X is nonnegative and below 2p, so the generic final 2p correction is unnecessary. Host export then uses the exact original transpose/store body without repeating the p reduction. Export remains after Y; the source does not deliberately retain an extra host-coordinate array across Y arithmetic.

Only FAST=true native eight-lane G1 streaming and signed borrowed G2 prefix reverse loops select this route. Existing products, scalar tails, signs, complete-curve handling, bucket geometry, queue order, field domains, record capacities and query scheduling remain unchanged. There is no key table, heap buffer, point copy, pool, new assembly or public production option. The wrapper constructor is private; callers cannot construct a purported canonical vector from unchecked limbs. Existing SIMD export instructions and their unsafe block are copied verbatim, with the same layout and target-feature assumptions. Stack/register traffic can still change and is not a RAM minimum.

This differs from the rejected canonical-input X-sum experiment, which changed only Ax+Bx, and from final-Y subtraction/export experiments. Their outcomes remain unchanged. This prototype appends a private include to the generated eight-lane field file; removing it recovers the exact original. Template/generator integration and portable/resource qualification remain future work if full-proof evidence supports adoption.

## Actual caller measurements

Ryzen 9950X, 16 physical/32 logical CPUs. Six caller geometries—owned unsigned G1, borrowed signed G1 and borrowed signed G2 at 200/352 pairs—run on solo CPU 2 and on CPUs 0–31. Fifteen fixed blocks rotate/reverse contexts and three executable orders. Frozen accepted-source libraries, freshly built OFF and ON use the same benchmark source, native compiler target and fat LTO. Frozen source is independently recovered to the same 175-file production manifest after removing diagnostic adapters; all frozen library hashes/flags are checked. These are actual production stream kernels on synthetic valid generator points, not a captured proof witness distribution.

Each worker performs three checked/reset 1,024-call primes, then 8,192 timed calls after reset. Ready/start/done/release barriers keep every worker's allocation and final checking outside every worker's timing. The process wave includes the slowest worker and barrier tail; individual worker timing/CPU, aggregate CPU, throughput and capacities are retained. Every result checks full point outputs against generator scalar sums and checks identical digests/capacities between variants. All 580 processes pass and report zero timed global-allocator calls.

Times below are microseconds per complete batch wave, median of 15 process means. Ratios are paired within blocks and need not equal ratios of the displayed medians. A negative change means faster.

|Caller and pairs|CPUs|Frozen µs|OFF µs|ON µs|ON/OFF paired change|Bounds|Wins/15|
|---|---|---:|---:|---:|---:|---|---:|
|g1-owned / 200|solo|4.107339|4.114661|4.120957|+0.225%|[0.996854925, 1.010196247]|6|
|g1-owned / 200|logical32|6.829691|6.858997|6.801406|-0.591%|[0.986790222, 1.000644498]|11|
|g1-owned / 352|solo|6.703702|6.697533|6.729729|+0.882%|[0.996326939, 1.013027157]|6|
|g1-owned / 352|logical32|11.407939|11.410130|11.357379|-0.349%|[0.987629531, 1.002145027]|11|
|g1-borrowed-signed / 200|solo|4.317139|4.317162|4.304247|-0.818%|[0.988429924, 1.003219629]|10|
|g1-borrowed-signed / 200|logical32|7.042884|7.054757|7.064633|+0.185%|[0.994694335, 1.003487146]|6|
|g1-borrowed-signed / 352|solo|7.067231|7.065438|7.031591|-0.091%|[0.987056117, 1.002843546]|9|
|g1-borrowed-signed / 352|logical32|11.785478|11.844054|11.636214|-0.922%|[0.983407241, 0.997825301]|13|
|g2-borrowed-signed / 200|solo|8.425530|8.434745|8.388145|-0.097%|[0.986138514, 1.004874819]|9|
|g2-borrowed-signed / 200|logical32|15.798444|15.811689|15.692030|-0.603%|[0.986355182, 0.997831788]|12|
|g2-borrowed-signed / 352|solo|14.184941|14.202413|14.215172|+0.032%|[0.995885517, 1.007570404]|7|
|g2-borrowed-signed / 352|logical32|27.189458|27.164840|26.927045|-1.209%|[0.986931258, 0.992377771]|15|

Some individual estimates are slower or uncertain; all remain in the table. No single-caller strict-gain requirement was declared. The gate requires all 12 OFF/frozen calibration intervals inside [1/1.03,1.03] and every ON/control upper nonregression ratio≤1.03. Both equal-log aggregates—the full 12 contexts and the six logical-32 contexts—must have upper ratio<1 and at least 10/15 wins against both controls. Every condition passes.

Bounds use paired order ranks 4–12 of 15, nominal 96.484375% individual median coverage under independent/exchangeable blocks, not familywise guarantees. There is no gain floor, exclusion, sample extension, subset promotion or post-capture threshold adjustment.

|Aggregate comparison|Ratio|Bounds|Wins/15|
|---|---:|---|---:|
|all12/on/frozen|0.997310801|[0.995713526, 0.998013740]|14|
|all12/on/off|0.996799375|[0.994779618, 0.998272535]|14|
|logical32_6/on/frozen|0.995574824|[0.992059013, 0.997866354]|14|
|logical32_6/on/off|0.994034828|[0.990557283, 0.997351683]|14|

## Correctness and profiling

Fresh OFF passes 33 field and 128 core tests; ON passes 34 field and 129 core tests, all unfiltered. One new field test checks 65,536 canonical-export and canonical-difference lanes against independent Arkworks values, including lazy 0…2p inputs and zero/one/minus-one corners. A new core test checks 1,024 groups of eight G1/G2 coordinate expressions against scalar fields. The four existing diagnostic generator tests cover real stream entry points, signs, tails, resets and scratch reuse; their cases are not newly invented coverage. No prover/Gnark or portable qualification is inferred.

The snapshot has 178 files: 170 of 175 originals are byte-exact; five originals have reversible prototype/adapter edits and three children are new. Every production source hash remains unchanged. The initial setup stopped before creating a build plan when it decoded a binary fixture as UTF-8; byte comparison repaired only that audit. OFF's five build/test/link steps then succeeded. ON's first compile caught an inner doc comment in an include and a missing Arkworks trait import; no ON test had executed. The comment/import were repaired, and the new core finishers were marked inline before their first build to retain the original caller boundary. OFF successes are reused, followed by six successful ON/frozen steps. Initial sources, failure records and repairs are archived; the original setup-controller executable remains local with its recorded SHA256. No successful test, build, link or capture is repeated.

All 540 latency processes finish before four separate FIFO-gated 199 Hz cycles:u recordings: logical-32 borrowed G1/200 and G2/200, OFF/ON, 262,144 calls per worker after primes/reset. All four finish before ownership transfer and offline owner-user decoding/raw-loss checks. Each recording contains one benchmark PID and zero lost records. No owned helper/build/hash/archive or second CPU workload overlaps capture. Selected actual linked G1/G2 disassemblies are checked before timing. Static instruction sites, sampled residence and function sizes are not dynamic work or exclusive wall-time costs.

|Profile|Samples|
|---|---:|
|profile-g1-borrowed-signed-200-logical32-off|11496|
|profile-g1-borrowed-signed-200-logical32-on|11484|
|profile-g2-borrowed-signed-200-logical32-off|26016|
|profile-g2-borrowed-signed-200-logical32-on|25842|

All 596 capture/offline steps complete. Total **136153664 batch calls**, including primes, preflights and profiles; these are not proof requests. Every closing pin and compressed source/receipt round trip passes. Lifetime process RSS and capacity observations are retained in raw results; they do not establish minimum RAM or memory-pressure behavior.

## Full proof status and M5 references

This component creates no SDK requests, proof exports or independent Gnark verifications. Ledger remains **20,755**. It passes only the prospectively declared caller screen. A separate trace-free, adapter-free standard-SDK build, fresh controls, full-proof gain checks and compatibility/resource qualification are required before changing production. No new AWS, Aeglos, phone/ARM/TEE, large-circuit, concurrent-throughput or minimum-RAM claim follows.

The following table is inherited accepted production performance, with supplied historical M5 references. These are not new timings from the component. The accepted build's earlier full-performance-gate failure and physical16 engineering exception remain retained. Primary accepted peak remains 134 MiB.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|

Accepted 54→1 remains 162.250464 ms without a supplied M5 reference. The original all-eight CPU 10× goal remains active and unmet, 0/8. [Current local/AWS summary](../2026-10-29-current-cpu-summary/README.md).

[Design](artifacts/design.md), [history](artifacts/history-audit.md), [all observations and profiles](artifacts/analysis.json), [field prototype](artifacts/field.rs), [caller finishers](artifacts/finish.rs), [source audit](artifacts/source-audit.json), [review](artifacts/review.json).
