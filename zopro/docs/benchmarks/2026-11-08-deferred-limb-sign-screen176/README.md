# Deferred limb-sign matched measurements

**The new dependency schedule improves the older pairwise recipe by 2.35% overall, but the full candidate remains 4.17% slower than the fresh original arithmetic control. It is not selected for point-caller or SDK integration.** The fixed qualification gate fails. All 3,136 caller output/allocation checks and all 32 profile-quality checks pass; production is unchanged.

This follows [exact algebra and native qualification](../2026-11-08-deferred-limb-sign176/README.md). The asymmetric identity sends signed limb differences directly to IFMA and compensates their signs afterward, preserving the exact low/high product words. The new comparison isolates that change from the pairwise convolution's existing product-count tradeoff. No speed result is inferred from the prior correctness/codegen stage.

## Matched campaign

Ryzen 9 9950X, 16 physical cores/32 logical CPUs, shared host. Four newly linked benchmark executables use the same driver: `accepted` links the frozen accepted-source **field library**; `OFF` uses fresh original arithmetic; `OLD` uses the exact previous sign-oriented pairwise lowering; `ON` uses the qualified deferred lowering. The last three share one coherent release field library. These are arithmetic diagnostics, not accepted SDK executables. Native -O, fat LTO, one codegen unit and common runtime dependencies are checked before clocks.

The [protocol](artifacts/protocol.md) covers 48 contexts: Fq/Fr × R256/R260 × ordinary one/four states, fused sum and fused difference × 1/16/32 Rayon workers. Each bank holds four distinct SHA-derived full-width inputs. Every round applies `[a,b,c,d] → [b,c,d,op(a,b,c,d)]`. Fused operands are not fixed constants. Four-state cases use a rolled slice loop; they are not four simultaneously register-resident streams. Array movement, loads/stores, the full arithmetic and dispatch/join are charged.

A separate Arkworks implementation executes that changing recurrence once per prime/worker, saves the declared round checkpoints and supplies 156 combined digests covering all banks, all four states, all eight lanes and logical worker order. R256/R260 constructors represent the same values, so those mathematical reference populations are shared. Each process checks three 128-round primes and resets before timing; final output, capacities, allocations and affinity are checked afterward.

The fixed run has 192 checked 64-round preflights and 15 rotated/reversed paired blocks of 262,144 rounds: 2,880 latency processes. Thirty-two separate grouped PMU panels and 32 cycle profiles follow. All 3,136 recordings finish before 160 ownership/decoder/metadata/event/loss commands. Total: 3,296 commands and 322 pins. All builds, independent gold, 48 actual kernel decodes, the plan, analyzer, independent reader and loss decoder are frozen before any preflight. No owned helper/build/hash/analysis/report/push overlaps recording. Nothing is rerun, extended or excluded.

Solo workers pin to CPU2; 16 and 32 workers pin individually to CPUs0–15 and0–31. Logical fixtures may execute on any allowed Rayon worker; digest order is logical. Worker stacks are requested at 512 KiB. The System counting allocator and process environment are identical among arms.

Ratios use 15 paired blocks, sorted zero-based ranks3/7/11 for lower/median/upper bounds. Nominal median coverage is96.484375% under exchangeable blocks, not familywise coverage. Every OFF/accepted interval must lie within factor1.03. Every ON/control upper bound must be≤1.02 against all three controls. The all48 and four separate field/radix32-worker aggregates must have upper<1 and at least10 winning blocks against each control. There is no positive gain floor or post-data subset rescue.

## All observations

Times are median **nanoseconds per joined batch wave**. One wave performs the stated one/four eight-lane operations on every logical worker. They are not proof latency, per-point latency or directly comparable to the supplied M5 proof references. Paired-ratio medians may differ from ratios of displayed marginal time medians. Bounds for every pair, all raw blocks and resource rows are in the artifacts.

|Context|Accepted ns|OFF ns|OLD ns|ON ns|ON/OFF paired median|ON/OLD paired median [lower, upper]|OLD wins /15|
|---|---:|---:|---:|---:|---:|---:|---:|
|fq256_mul1-w1|15.7320|15.4791|14.9214|14.9719|0.969487|0.997841 [0.995310, 1.012462]|9|
|fq256_mul1-w16|16.9283|17.0441|16.1828|16.2787|0.956692|1.009226 [1.001648, 1.039653]|3|
|fq256_mul1-w32|28.7597|28.5985|30.6880|29.6041|1.025848|0.946001 [0.878696, 0.975399]|14|
|fq256_mul4-w1|61.5625|61.7109|60.1250|60.1913|0.969108|0.998976 [0.990809, 1.017998]|8|
|fq256_mul4-w16|68.5733|68.5789|67.1943|67.1185|0.978265|0.998872 [0.997383, 1.002602]|10|
|fq256_mul4-w32|118.0596|117.2865|126.4720|124.1780|1.059509|0.982123 [0.973401, 0.990268]|15|
|fq256_sum1-w1|25.4327|25.3238|25.4725|24.9757|0.982777|0.986221 [0.976699, 0.988156]|15|
|fq256_sum1-w16|27.6237|27.7064|27.7812|27.4937|0.992995|0.990174 [0.988543, 0.997941]|12|
|fq256_sum1-w32|41.4662|41.5827|47.3814|46.2927|1.111003|0.975441 [0.965708, 0.987757]|14|
|fq256_diff1-w1|26.0329|26.0061|26.4927|25.6443|0.972107|0.962538 [0.958532, 0.969812]|15|
|fq256_diff1-w16|28.4271|28.3750|30.9989|28.0944|0.988953|0.908920 [0.876999, 0.921907]|15|
|fq256_diff1-w32|44.5257|44.5583|55.0109|50.9962|1.152053|0.927218 [0.914121, 0.932506]|15|
|fq260_mul1-w1|12.7527|12.7083|13.7078|13.2791|1.043930|0.971276 [0.960721, 0.978527]|15|
|fq260_mul1-w16|14.1784|14.2324|15.2934|14.9346|1.045690|0.977138 [0.962659, 0.981384]|14|
|fq260_mul1-w32|26.4670|25.8906|29.4363|28.2279|1.055257|0.930622 [0.872623, 0.962080]|13|
|fq260_mul4-w1|51.5170|51.7351|55.5876|55.5853|1.082035|0.990462 [0.979828, 1.026310]|10|
|fq260_mul4-w16|56.9204|57.3013|62.2792|61.7812|1.079197|0.995974 [0.987959, 1.001881]|11|
|fq260_mul4-w32|103.4143|104.1081|119.3550|118.2162|1.137384|0.987507 [0.977242, 0.999419]|12|
|fq260_sum1-w1|24.4144|24.1772|24.8070|24.2461|0.993619|0.970167 [0.966616, 0.977657]|15|
|fq260_sum1-w16|26.1037|26.3829|26.8008|26.8336|1.011294|0.989591 [0.984705, 1.018597]|10|
|fq260_sum1-w32|35.5183|35.6312|45.6252|42.8418|1.192850|0.937786 [0.925785, 0.969324]|15|
|fq260_diff1-w1|24.6715|24.9122|25.5095|25.3939|1.010786|0.994402 [0.983135, 1.014847]|9|
|fq260_diff1-w16|27.0832|26.8957|27.9522|27.5931|1.024821|0.985354 [0.976181, 0.995835]|13|
|fq260_diff1-w32|40.6429|40.5779|49.3025|48.0071|1.183416|0.972567 [0.931837, 1.008364]|11|
|fr256_mul1-w1|15.6929|15.6060|14.9450|14.9617|0.962741|0.999280 [0.992620, 1.004834]|10|
|fr256_mul1-w16|16.9026|17.0446|16.2180|16.2812|0.954677|1.002629 [1.001407, 1.006024]|3|
|fr256_mul1-w32|28.9112|28.8558|31.5506|30.7358|1.044632|0.969189 [0.953615, 0.973538]|14|
|fr256_mul4-w1|61.4051|61.7800|59.8913|60.0543|0.977133|1.006131 [0.996403, 1.021385]|5|
|fr256_mul4-w16|68.6190|68.4995|67.4038|66.9963|0.975597|0.995738 [0.991624, 1.002797]|11|
|fr256_mul4-w32|117.2396|116.3490|125.5999|122.9445|1.055181|0.975564 [0.973574, 0.980664]|14|
|fr256_sum1-w1|25.4035|25.3645|25.3475|24.9383|0.985435|0.984247 [0.976809, 0.989147]|15|
|fr256_sum1-w16|27.6636|27.8743|27.7108|27.4901|0.987869|0.990130 [0.973134, 0.993850]|14|
|fr256_sum1-w32|41.3253|41.3428|47.3304|45.8312|1.102637|0.968467 [0.960268, 0.983293]|15|
|fr256_diff1-w1|25.8977|25.9742|26.5892|25.6845|0.982643|0.965937 [0.958448, 0.972972]|15|
|fr256_diff1-w16|28.5292|28.5509|29.2491|28.1568|0.984306|0.962987 [0.959170, 0.970706]|15|
|fr256_diff1-w32|44.4283|44.5174|55.2361|51.2717|1.155285|0.925365 [0.915748, 0.934615]|15|
|fr260_mul1-w1|12.8346|12.6642|13.6528|13.3266|1.048346|0.976570 [0.973625, 0.979641]|15|
|fr260_mul1-w16|14.3261|14.3409|15.3998|15.1763|1.051994|0.977012 [0.963466, 0.980858]|15|
|fr260_mul1-w32|26.4263|26.0535|30.5979|27.9790|1.066029|0.950527 [0.906263, 1.010435]|11|
|fr260_mul4-w1|51.2409|51.3437|55.5038|55.2768|1.081702|0.997065 [0.988799, 1.019790]|8|
|fr260_mul4-w16|56.9766|57.0038|61.8525|61.7495|1.081399|0.997424 [0.988130, 1.001846]|11|
|fr260_mul4-w32|103.8793|104.5856|118.7952|118.1277|1.129212|0.995615 [0.988883, 1.006680]|9|
|fr260_sum1-w1|24.4254|24.1951|24.8098|24.1241|0.990668|0.969187 [0.963237, 0.975833]|15|
|fr260_sum1-w16|26.0233|26.3730|26.8949|26.5551|1.011843|0.986640 [0.982284, 1.031412]|10|
|fr260_sum1-w32|35.7397|35.6210|45.8251|42.7255|1.217580|0.935737 [0.928889, 0.953182]|15|
|fr260_diff1-w1|24.6841|24.8063|25.4555|25.3198|1.023298|0.997398 [0.981866, 1.009783]|9|
|fr260_diff1-w16|26.9661|26.7733|27.9041|27.4850|1.023497|0.989904 [0.978335, 0.997269]|13|
|fr260_diff1-w32|40.3799|40.3338|49.7020|47.0291|1.165996|0.948323 [0.927177, 0.986688]|12|

`OLD wins` means blocks where ON beats OLD. R256 medians at1/16 workers improve0.70–4.53% versus fresh original arithmetic; those positive observations remain. The four32-worker field/radix aggregates improve3.90–4.77% versus OLD, yet regress8.77–15.45% versus fresh OFF. A favorable worker, operation, field or radix subset is not selected.

|Declared aggregate / denominator|ON paired median [lower, upper]|Winning blocks /15|
|---|---:|---:|
|ALL/OFF|1.041687 [1.039191, 1.046085]|0|
|ALL/OLD|0.976455 [0.974522, 0.979963]|15|
|ALL/accepted|1.042245 [1.039174, 1.047962]|0|
|fq256/OFF|1.087729 [1.079781, 1.091196]|0|
|fq256/OLD|0.957154 [0.945249, 0.965286]|15|
|fq256/accepted|1.082987 [1.080600, 1.092663]|0|
|fq260/OFF|1.154483 [1.145291, 1.167386]|0|
|fq260/OLD|0.952329 [0.947843, 0.967731]|15|
|fq260/accepted|1.148289 [1.139689, 1.159140]|0|
|fr256/OFF|1.091063 [1.079609, 1.100397]|0|
|fr256/OLD|0.960095 [0.953126, 0.966065]|15|
|fr256/accepted|1.086336 [1.082470, 1.097156]|0|
|fr260/OFF|1.151655 [1.132306, 1.157417]|0|
|fr260/OLD|0.960997 [0.953244, 0.968047]|15|
|fr260/accepted|1.139253 [1.125513, 1.160128]|0|

All-context ON/OLD is0.976455 [0.974522,0.979963],15/15 wins: a retained positive result for the dependency change. ON/OFF is1.041687 [1.039191,1.046085],0/15; ON/accepted is1.042245 [1.039174,1.047962],0/15. The change helps an already slower overall recipe in this new changing-input harness. These observations do not replace or erase the older constant-input screen.

Of207 declared timing checks, **81 fail: seven calibration,64 nonregression and10 gain checks**. All five aggregate gain checks against OLD pass, while all ten against accepted/OFF fail. Calibration failures are Fq/R256 mul4/solo, Fq/R260 mul1/32,mul4/solo,difference/32, Fr/R256 mul4/solo and Fr/R260 mul1/32,mul4/solo. Exact causes of the control differences remain unresolved. Fresh ON/OFF and ON/OLD results remain visible, but do not repair calibration.

## Work, profiles and memory

All 48 actual release kernel functions are decoded once before recording. Original controls have100/150 static IFMAs for ordinary/fused arithmetic. OLD/ON both have80/110; OLD has10/20 vector absolute-value sites and ON has zero. No kernel contains a call instruction. These are whole-function static counts for the rolled changing-state kernels, not dynamic retirement or proof-time fractions.

Separate PMU observations for the eight Fq/R256 panels follow. Events are user cycles/instructions/cache misses/branch misses, grouped with100% running coverage in every panel and nonnegative counts. They are single diagnostic panels, not replicated causal estimates.

|Context|Instructions ON/OFF|Cycles ON/OFF|Instructions ON/OLD|Cycles ON/OLD|
|---|---:|---:|---:|---:|
|fq256_diff1-w1|1.390864|0.973921|0.979361|0.961852|
|fq256_diff1-w32|1.385203|1.109521|0.979443|0.897245|
|fq256_mul1-w1|1.237192|0.959182|1.000002|1.006947|
|fq256_mul1-w32|1.235063|1.081788|0.999752|0.942406|
|fq256_mul4-w1|1.241414|0.952586|1.000000|1.001644|
|fq256_mul4-w32|1.239779|0.915729|0.999867|0.929007|
|fq256_sum1-w1|1.385424|0.976692|1.002639|0.987476|
|fq256_sum1-w32|1.382417|1.050299|1.002991|0.964189|

ON retires about23.5–24.1% more instructions for ordinary products and38.2–39.1% more for fused operations than OFF despite fewer IFMAs. Relative to OLD, instruction counts are nearly unchanged for multiply/sum and about2.1% lower for difference. The dependency change therefore helps without removing most of the pairwise recipe's additional work. This is a limited inference from the observed kernels, not proof that RAM capacity, bandwidth, SMT, frontend contention or one particular instruction exclusively causes the regressions. Summed worker CPU cycles in a separate panel are not the paired joined-wall-time result; some panels improve while wall medians regress.

All32 profiles qualify:364,406 samples total, minimum1,186 per profile, at most0.06344% unknown, at least99.9197% assigned-worker samples and98.1487% in the exact predeclared case-kernel inventory, with zero LOST. Sample period is50,000cycles:u,512 mmap pages, warm FIFO acknowledgement. Actual binary MMAP/PID/TID/CPU, user-only inheritance and disable/enable/disable acknowledgements pass. Flat samples do not measure a causal critical path or dynamic spill costs.

All3,136 callers have zero timed allocation/reallocation requests and stable, exactly equal fixture capacities across all arms. Each logical worker's initial/current capacity pair is[1280,1280]bytes for one bank or[5120,5120] for four. These are benchmark fixtures, not retained proving-key structures. ON process high-water RSS across latency processes is2.50–3.25MiB. Stack reservation is512KiB/worker; actual stack peak, prover peak/minimumRAM, memory pressure/swap, mobile/ARM/TEE, huge circuits and concurrent-proof throughput are unmeasured. No production pool, key table/copy or retained solver plan is added.

## Evidence and decision

Seven release/gold steps finish once. Eighteen copied field originals are recovered exactly by removing one append-only parent suffix; the qualified new child is byte-identical. The old-control child preserves its original arithmetic body except configuration/name/wrappers. All176 production source identities remain unchanged. Actual binaries, libraries and exact target inventory are hash-bound.

The frozen analyzer and separately compiled reader each run once after all recording/offline commands. Review reconstructs3,296 command receipts/environments/streams,3,136 independent-gold outputs, all207 timing checks, capacities, affinity, PMU values and all32 profiles. Its pass confirms evidence consistency and the failed candidate gate. Inherited unused shared-helper warnings remain without cleanup rebuilds. No failed build, capture or decoder occurs in this stage.

This closes the fixed arithmetic candidate without a point-caller/full-SDK trial or adoption. A future distinct design needs to address the extra executed work and its behavior under parallel load, with actual operation/dependency evidence before implementation. An unchanged rerun, selected worker/field/radix promotion or compiler/layout/host rescue is not selected. The deferred-sign identity remains useful evidence, with its measured gains retained.

## Unchanged proof comparison

Every following proof time is inherited from the [native-stage report](../2026-11-08-deferred-limb-sign176/README.md): latest local primary and seven explicitly older full-campaign rows. Supplied M5 CPU/Metal references use different hardware. No arithmetic-wave time is substituted for proof latency.

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.002770|37.0|24.0|4.623×|2.999×|3.7|
|2x2|11.241463|59.0|30.0|5.248×|2.669×|5.9|
|4x2|15.933816|109.0|48.0|6.841×|3.012×|10.9|
|8x2|24.234311|189.0|74.0|7.799×|3.054×|18.9|
|12x2|33.855565|284.0|101.0|8.389×|2.983×|28.4|
|16x2|40.071563|335.0|124.0|8.360×|3.094×|33.5|
|24x2|65.917462|528.0|176.0|8.010×|2.670×|52.8|
|merge-24x1|65.305023|507.0|175.0|7.764×|2.680×|50.7|

Accepted local1×2 remains **about8ms**: the same binary previously measured7.915095/7.928660/7.995682ms; latest8.002770ms. Inter-run variation is unresolved; this experiment produces no adopted regression. Merge54 remains inherited162.240109ms without an M5 reference. Source176/SDK6d83/canonical35,171 stay unchanged. No new SDK request/proof, AWS/Aeglos or device/resource qualification is added. Verified all-eight CPU10× remains active and unmet, **0/8**.

[Complete timing/gate/resource data](artifacts/timings.json), [independent review](artifacts/review.json), [protocol](artifacts/protocol.md), [release qualification](artifacts/release-qualification.json), [binary identities](artifacts/identities.json), [independent gold](artifacts/independent-gold.json), [codegen](artifacts/codegen.json), [PMU diagnostics](artifacts/PMU-diagnostics.json), [source/evidence archive](artifacts/evidence.json.gz). Every archived UTF8 payload and gzip roundtrip is checked. Exact new helpers/driver/old-control child, plans/pins/receipts, codegen, raw observations and decoded profiles remain. Libraries/executables/raw perf data, unchanged full source copies, keys/images/witness/proofs/private ledgers and full external papers are excluded and relevant compiled artifacts hash-bound. Public protocol trims trailing blank lines only; frozen archive bytes remain exact.
