# G2 coordinate helper: full-proof comparison

The candidate is **not adopted**. Both primary 32-worker latency gain checks fail; the other 29 latency, CPU and RSS checks pass. Correctness passes in the tested scope. The helper adds instructions and cycles in the follow-up profiles. This experiment is closed; production is unchanged.

Accepted / fresh disabled control / candidate 1×2 latency is **7.821726 / 7.803131 / 7.879869 ms**, with maximum lifetime RSS **127.50 / 128.50 / 129.25 MiB**.

Paired ON/REF: **1.008949 [1.005684, 1.013273]**, a 0.895% slower paired median. Paired ON/OFF: **1.011576 [1.007683, 1.015167]**, a 1.158% slower paired median. Bounds are nominal 95.722605% marginal intervals, not joint confidence or a stationarity guarantee.

## Complete-proof measurements

Local Ryzen 9950X, fixed 32 workers on CPUs 0–31; the separate 16-worker row uses CPUs 0–15. Thirty paired blocks, three independently written byte-identical executable copies per arm, ten blocks per copy and context. All copies and outcomes are retained. Values are medians of process arithmetic means: 101 measured requests for primary cases, 31 for others, after three primes. Assignment, solving, fresh blinding, proving, SDK verification and serialization are included. Loading, primes and export are excluded from latency. Lifetime RSS includes loading and primes and is not minimum RAM. The host is shared and frequency is unlocked.

M5 CPU and Metal references are supplied historical timings, not fresh matched-hardware measurements. Speedup columns list accepted / candidate.

|Shape|Accepted ms|OFF ms|ON ms|Peak RSS MiB accepted / ON|M5 CPU / Metal ms|CPU speedup accepted / ON|Metal speedup accepted / ON|CPU 10× limit ms|
|:---|---:|---:|---:|:---|:---|:---|:---|---:|
|1x2|7.821726|7.803131|7.879869|127.50 / 129.25|37 / 24|4.730× / 4.696×|3.068× / 3.046×|3.7|
|2x2|10.887879|10.910199|11.025502|272.00 / 279.50|59 / 30|5.419× / 5.351×|2.755× / 2.721×|5.9|
|4x2|15.852187|15.813692|15.845777|306.75 / 317.00|109 / 48|6.876× / 6.879×|3.028× / 3.029×|10.9|
|8x2|24.217576|23.987843|24.052050|502.50 / 515.25|189 / 74|7.804× / 7.858×|3.056× / 3.077×|18.9|
|12x2|34.013966|33.834591|33.948502|719.00 / 723.50|284 / 101|8.350× / 8.366×|2.969× / 2.975×|28.4|
|16x2|40.059851|39.928129|39.975927|836.75 / 828.50|335 / 124|8.362× / 8.380×|3.095× / 3.102×|33.5|
|24x2|66.586815|66.284682|66.276813|1292.75 / 1296.67|528 / 176|7.929× / 7.967×|2.643× / 2.656×|52.8|
|merge-24x1|66.228695|65.893055|65.914470|1183.32 / 1176.37|507 / 175|7.655× / 7.692×|2.642× / 2.655×|50.7|

1x2-w16: accepted / OFF / ON **9.157423 / 9.141134 / 9.220336 ms**, peaks **87.75 / 86.00 / 88.00 MiB**.

merge-54x1-w32: accepted / OFF / ON **163.713679 / 163.521344 / 163.878275 ms**, peaks **1960.57 / 1937.50 / 1996.35 MiB**.

No M5 reference was supplied for merge 54×1. All eight CPU 10× targets remain unmet (0/8). This run adds no AWS or Aeglos measurement.

## Decision and backing copies

The prospective criteria require primary32 latency upper < 1 against accepted and OFF; other nine latency uppers ≤ 1.02 versus accepted; all ten CPU/request uppers ≤ 1.02; all ten maximum-RSS guards ON ≤ accepted × 1.05 + 1 MiB. No minimum gain floor applies. Other OFF contrasts are diagnostic. Thirty sorted paired ratios use ranks 9/20 as bounds and mean ranks 14/15 as median. Two integer coverage constructions and a separate selection-based reader agree.

Both predeclared primary backing-spread flags are false. This does not prove absence of placement effects. All copy strata are descriptive, with no individual confidence claim.

|Copy|ON/accepted paired median|ON/OFF paired median|
|---|---:|---:|
|0|1.014639|1.023194|
|1|1.008949|1.012068|
|2|1.004803|1.006490|

## Profile result

A separate frozen panel used all three retained OFF/ON pairs, each with plain execution, grouped user counters and FIFO-gated cycle sampling. Each process measured 251 requests after three primes. All four observer guards, six counter checks and six sampling-quality checks pass. Instrumented/plain median ratios range from 1.000585 to 1.005398. The six cycle profiles contain 76,235 samples, zero LOST records and at most 1.799% unknown samples. Actual executable/PID/TID/CPU attribution was checked; counter running coverage is at least 99.9%. These checks do not prove unbiased observation.

|Copy|Instructions/proof millions OFF / ON|Cycles/proof millions OFF / ON|OFF G2 caller samples|ON caller + helper samples|
|---|:---|:---|---:|:---|
|0|1370.389 / 1376.289|804.781 / 818.439|2044|1396 + 679 = 2075|
|1|1370.769 / 1376.205|807.049 / 814.118|2008|1349 + 728 = 2077|
|2|1370.713 / 1375.413|804.790 / 813.437|2017|1384 + 741 = 2125|

ON executes approximately 0.34–0.43% more instructions and 0.88–1.70% more cycles per proof. The caller alone has fewer samples because work moved into the helper; caller plus helper has more samples in every pair. This provides no evidence of a useful G2 reduction. Sample counts are not exclusive wall time, precise per-copy estimates or direct op-cache measurements. The earlier backing-associated IBS observations remain valid; this particular split did not translate that hypothesis into a gain. No new op-cache causal claim follows.

## Source, correctness and resources

The candidate outlines the unchanged coordinate X/Y formulas into one never-inline helper per full eight-point borrowed G2 reverse group, behind default-off `zopro_g2_coordinate_leaf` and native IFMA guards. The scalar tail and other arithmetic/scheduling paths retain accepted behavior. All 176 accepted originals recover, with 174 byte-identical originals; the other edit outside borrowed G2 is a test-only adapter declaration. Source, build and linked-code receipts are retained.

Linked caller size changes from 51,862 to 43,202 bytes plus a 9,466-byte helper: **806 more total symbol bytes**. IFMA sites remain 1,450. Explicit stack reservations in the selected caller/helper sum grow from 5,952 to 8,312 bytes; this excludes pushes, alignment and other callees and is not a measured stack peak. No new heap allocation, key table, expanded plan, production pool or architectural memory cap is added.

Preparation passed 130 native core tests, two prover unit tests, six cubic tests and 97 generic-core tests. The changing-caller adapter exercises signs, tails, repeated updates, retained capacities and all workers in 1/4/16/32 pools with 512 KiB requested stacks. This does not qualify ARM/phones, huge 250/4999 circuits, memory pressure, minimum RAM or concurrent throughput.

Preflight/main/profile contain **47,880 SDK calls, 45,045 measured samples and 945 distinct fresh Gnark-verified exports**. Canonical count advances 52,972 → 52,999 → 53,899 → **53,917**. Proof/public/image bindings, freshness, worker placement, pinned sources and command/stream receipts pass. The independent main reader reconstructs 900 process means and all 31 decisions with a separate rank implementation; this is a same-agent cross-check, not an external audit.

REF SDK SHA256: `c88a3528d5feb363732dd2557d1894c93e9161f4e3ec9e9d3c7e2564283bf2fe`.

OFF SDK SHA256: `7a345c0a49962c918b89fdd7af52dbb2348d1913c29bb0f122a4cf54167b9b6d`.

ON SDK SHA256: `d3a3f3b084a43de0cf856650b71834b845f64c7baf89a7910e4ce1d1bd653521`.

OFF/ON use coherent Rust 1.98.1 znver4 O3 fat-LTO builds with one codegen unit. The exact accepted SDK is retained. No source or runtime adjustment occurred during capture; no owned build/analysis overlapped SDK recording. No AWS resource was created and no push/publication is approved.

## Iteration cost and next work

This campaign used full qualification before establishing a useful mechanism. The next experiment should use the [updated playbook](../../cpu-performance-playbook.md): mechanism test, short verified full-proof screen, then full all-shape qualification only for a promising exact build. Keep this failed campaign closed. Further helper-boundary or annotation variants require new evidence. Prioritize a cost model and algorithmic changes that remove substantial operations or repeated work, with bounded memory and unchanged proof semantics; no such breakthrough is established here.

preflight-run recorded command time: 9.5 seconds SDK execution, 3.5 seconds other commands. These sums exclude orchestration, preparation and analysis time.

main-run recorded command time: 1594.6 seconds SDK execution, 107.2 seconds other commands. These sums exclude orchestration, preparation and analysis time.

run recorded command time: 43.2 seconds SDK execution, 0.8 seconds other commands. These sums exclude orchestration, preparation and analysis time.

[Decisions](artifacts/assessment.json), [all eight CPU/Metal comparisons](artifacts/independent-main-review.json), [screen protocol](artifacts/screen-design.md), [profile results](artifacts/profile-result.json), [profile protocol](artifacts/profile-protocol.md), [resource rows](artifacts/resource-rows.json), [build review](artifacts/build-review.json) and [archive index](artifacts/final-artifacts.json) preserve the evidence. Archives include raw driver/verifier receipts, public exports, samples and decoded profiles. Private key/image/witness payloads, libraries and SDK executables are excluded. Raw perf.data and exact binaries remain local with hashes. Reproduction requires the pinned inputs and dependencies.

Local roots: `/home/ubuntu/src/zopro-perf-work/g2-coordinate-leaf177/` and `/home/ubuntu/src/zopro-perf-work/g2-coordinate-leaf-profile177/`.
