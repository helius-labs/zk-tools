# Signed differences in complete G2 batches

29 October 2026. The separately declared actual-caller gate **fails**. Production remains unchanged. This tests the exact signed CIOS helper from the [earlier arithmetic experiment](../2026-10-29-signed-difference175/README.md), whose generic component gate remains failed. Its 7.38% affine-expression gain is retained alongside the dependent-chain and scaled-product regressions.

## Implementation

Only two native IFMA8 G2 lane operations select the signed helper: the scaled numerator's imaginary difference and complex multiplication's real difference. The helper subtracts raw product halves inside signed Montgomery accumulation and adds p only when negative. The original global difference operation and portable/four-lane paths remain unchanged. No heap buffer, key table, point-array copy, persistent cache or worker pool is added.

The private snapshot has 178 files. Of the 175 originals, 171 are byte-exact; removing four reversible integration/diagnostic edits recovers all originals. Three children add the exact parent kernel, exact parent field tests and unchanged caller adapters. Production source hashes remain exact. The parent BigInt model is inherited; this build runs the full native field suite in its new integration context.

## Measurement method

Ryzen 9 9950X, 16 physical cores and 32 logical CPUs. Owned unsigned and borrowed signed G2 prefix batches use 32, 200 and 352 point pairs on CPU 2, physical CPUs 0–15 and logical CPUs 0–31. Fifteen fixed rotated/reversed blocks compare frozen original libraries, fresh OFF and candidate ON. Frozen libraries are accepted only after recovering their exact 175-source manifest, native flags and library hashes. All three use the same harness with fat LTO.

Each worker performs three checked/reset 1,024-call primes, then 8,192 timed complete batch calls. Ready/start/done/release barriers keep every worker's fixture allocation and generator oracle outside every worker's clock. The whole wave includes inversion, packing, coordinate arithmetic, output scatter and the barrier tail. Valid generator fixtures satisfy the production batch contract of disjoint nonexceptional pairs; the surrounding complete-MSM tests cover identities, repeated and opposite points. These synthetic fixtures do not represent proof witness distributions or concurrent proof throughput.

There are 90 preflights, including 199/351-pair scalar tails, then 810 latency processes. Tail preflights qualify correctness, not tail latency. Every process checks complete outputs against generator scalar sums, identical cross-variant digests/capacities, and zero timed global-allocator calls. Capacities and rusage before final oracles are retained in the receipts; they do not establish minimum prover RAM.

## Latency results

Units are microseconds per complete batch wave, median of 15 process means. Paired ratios need not equal ratios of displayed medians. A negative percentage means faster.

|Caller|Pairs|Workers|Frozen µs|OFF µs|ON µs|ON/OFF paired change|Ratio bounds|Wins / 15|
|---|---:|---:|---:|---:|---:|---:|---|---:|
|owned|32|1|1.886340|1.881515|1.887277|+0.484%|[0.997843281, 1.010973191]|5|
|owned|32|16|2.134350|2.121072|2.136400|+0.943%|[0.995234000, 1.016593823]|5|
|owned|32|32|3.176975|3.196256|3.228813|+0.942%|[1.003210043, 1.015827686]|2|
|owned|200|1|7.837655|7.837732|7.732178|-1.734%|[0.973695615, 0.999410105]|12|
|owned|200|16|9.102569|9.074446|8.945461|-1.113%|[0.985908057, 0.998607497]|14|
|owned|200|32|14.659061|14.724860|14.983833|+1.787%|[1.008209751, 1.020567238]|1|
|owned|352|1|13.212621|13.202565|13.017316|-1.542%|[0.978622691, 0.991688897]|13|
|owned|352|16|15.597203|15.506096|15.265200|-1.489%|[0.980434824, 0.989129570]|13|
|owned|352|32|25.282147|25.467406|25.695857|+1.042%|[1.005635665, 1.017212258]|2|
|borrowed signed|32|1|1.955745|1.964766|1.951107|-0.668%|[0.986591626, 1.000530592]|11|
|borrowed signed|32|16|2.182832|2.173797|2.182456|+0.045%|[0.997439657, 1.008216370]|7|
|borrowed signed|32|32|3.303258|3.308455|3.355014|+0.976%|[1.005301267, 1.021676765]|2|
|borrowed signed|200|1|8.486659|8.466290|8.273498|-2.417%|[0.965624296, 0.985798676]|15|
|borrowed signed|200|16|9.870972|9.826397|9.585111|-2.264%|[0.969917081, 0.984405957]|15|
|borrowed signed|200|32|15.917627|15.978127|16.255963|+1.875%|[1.011264751, 1.029165814]|1|
|borrowed signed|352|1|14.239289|14.249914|13.917212|-2.310%|[0.971156912, 0.992148086]|15|
|borrowed signed|352|16|16.769259|16.843977|16.555534|-1.613%|[0.962958889, 0.991756076]|15|
|borrowed signed|352|32|27.400424|27.414740|27.881203|+1.638%|[1.006472144, 1.022686372]|2|

|Aggregate|Ratio|Bounds|Wins / 15|
|---|---:|---|---:|
|all18/on/frozen|0.996913176|[0.996031448, 1.000000895]|11|
|all18/on/off|0.997421680|[0.995443811, 0.999687934]|13|
|threaded12/on/frozen|1.002934622|[1.000923399, 1.005559305]|2|
|threaded12/on/off|1.001960588|[0.999557705, 1.005031514]|5|

Borrowed 200-pair batches improve 2.42% solo and 2.26% on 16 physical cores versus fresh OFF, with 15/15 wins in both contexts. At 32 logical workers the same caller regresses 1.87%, with one win. Borrowed 352-pair batches show the same direction: improvements of 2.31% solo and 1.61% at 16 workers, versus a 1.64% regression at 32. These positive physical-core results remain valid within their measured contexts.

All calibration and nonregression checks pass. The prospective gate requires every OFF/frozen calibration interval inside [1/1.03, 1.03] and every candidate/control upper ratio ≤1.03. Both equal-log aggregates, all 18 contexts and independently the 12 threaded contexts, require upper ratio <1 and at least 10/15 wins against both controls. Paired ranks 4–12 give nominal 96.484375% individual median coverage under independent/exchangeable blocks, without a familywise guarantee. No gain floor, exclusions, sample extension, subset promotion or threshold change is applied.

Failed checks:

- `all18/on/frozen/gain`
- `threaded12/on/frozen/gain`
- `threaded12/on/off/gain`

## Correctness and automatic profiling

Unfiltered native suites pass 33/35 field, 128/128 core and eight/eight prover tests OFF/ON. The two added field tests reuse the exact parent's Arkworks changing-chain checks. Existing diagnostic tests cover owned/borrowed signs, tails, resets and scratch reuse; existing complete-MSM tests compare zero/one/minus-one scalars and identity/repeated/opposite points to scalar reference operations. The initial capture setup stopped before any timing because it expected eight prover tests in the library suite. That suite has two tests; six more are in the cubic integration target. The two missing integration suites then run separately, retaining all 13 successful original build steps without repetition. Initial controller/publisher sources and the repair are archived. No portable or full SDK qualification is inferred from these tests.

All latency captures precede six separate FIFO-gated 199 Hz cycles:u profiles, with explicit CPU IDs and timestamps: borrowed G2/200 at 16 and 32 workers, and owned G2/200 at 32 workers, OFF/ON. Each worker makes 262,144 profiled calls after primes/reset. All recordings finish before owner transfer, owner-user decoding and lost-record checks. No owned build/helper/hash/archive or second CPU workload overlaps capture. Selected linked G2 assembly is inspected before timing with read-only objdump.

|Profile|Samples|Lost records|
|---|---:|---:|
|profile-g2-borrowed-signed-200-logical32-off|26107|0|
|profile-g2-borrowed-signed-200-logical32-on|26553|0|
|profile-g2-borrowed-signed-200-physical16-off|8115|0|
|profile-g2-borrowed-signed-200-physical16-on|7999|0|
|profile-g2-prefix-200-logical32-off|24202|0|
|profile-g2-prefix-200-logical32-on|24486|0|

Each profile has one sampled process and zero lost records. Raw recordings and disassemblies remain local with hashes. Flat samples and static instruction sites do not isolate exclusive wall-time costs. All 930 workflow steps complete, totaling 196020160 batch calls including primes and profiles. These are kernel calls, not proof requests. Source and receipt archives round-trip byte-exactly.

## Proof performance and decision

This exact caller implementation is not promoted to the SDK. The failed checks and positive estimates remain in the record; no unchanged rerun or post-result subset is promoted as passing. The earlier arithmetic-expression gain remains valid within its original scope.

The direction changes when SMT siblings share each physical core. A plausible explanation is a tradeoff between shorter carry dependencies and the signed routine's additional vector subtractions; this capture does not isolate execution-port, front-end, clock or register-pressure costs. Selected borrowed-G2 code retains 1,450 static IFMA sites in every executable and grows from 51,942 to 53,351 bytes ON. These are static counts, not dynamic instruction totals. The result favors investigating arithmetic that reduces total work as well as shortening dependency chains. AWS c7a has no SMT, but its proof-level outcome cannot be inferred from this local caller test.

This stage adds zero SDK requests, proof exports or Gnark verifications. The distinct-proof ledger remains **21,259**. The all-eight verified CPU 10× goal remains active and unmet, 0/8. Accepted local timings and supplied historical M5 references follow; these are inherited measurements, not candidate proof results.

|Shape|Accepted local ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|

Accepted 1×2 peak RSS remains 134 MiB. Accepted 54→1 remains 162.250464 ms, without a supplied M5 reference. The accepted build's original performance-gate failure and physical-16 calibration engineering exception remain documented; its separate final-artifact gate passes. The latest AWS 1×2 remains Zopro 11.915591 ms at 32 workers versus Aeglos 7.959184 ms at 96. No new AWS, mobile/ARM/TEE, forester 250/4999, minimum-RAM, swap/pressure or concurrent-proof claim follows. [Current local/AWS comparison](../2026-10-29-current-cpu-summary/README.md), [benchmark snapshot](../2026-10-29-cpu-benchmark-report/README.md).

[Protocol](artifacts/design.md), [all observations and checks](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [kernel](artifacts/kernel.rs), [native source](artifacts/source.json.gz), [native and capture receipts](artifacts/receipts.json.gz), [publication review](artifacts/publication-review.json).
