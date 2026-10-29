# Sequential in-place G2 decomposition

The full-SDK performance gate is **false**. Primary32 ON/OFF paired ratio is **0.996476742**, bounds **[0.984390159, 1.005386023]**, **6/7 wins**, and paired saving **28.374 µs**. ON/accepted is 1.001941697, bounds [0.996562788, 1.008352205]. Positive estimates are retained without a minimum gain floor. No production change is adopted.

## Change

Only the existing hb_raw scalar loop differs: a private selector chooses the original parallel iterator (OFF) or sequential iter_mut (ON), applying the same phi_split to the same input words. The existing exclusive eligibility range, grouping, query order, buffers, parts, widths, formulas, key/solver and resource forecasts remain unchanged. One original source differs, 174 remain byte-exact and reversing the edit recovers all 175. The changed iteration adds no buffer, table or plan; field arithmetic, unsafe code and ISA settings remain unchanged. The private std::env::var selector can transiently allocate in both OFF and ON. The design's no-allocation phrase refers to loop storage and does not make the diagnostic selector allocation-free; it would be removed for clean-source qualification. OFF/ON share one executable path/inode; accepted is the retained SDK20fde. The selector is experimental and is not a new public runtime option.

The preceding [preparation diagnostic](../2026-10-29-g2-preparation175/README.md) measured fold CPU separately from return delays. Those elapsed intervals were not assumed removable. This recipe removes the fork/join boundary, unlike the older [minimum-grain candidate](../2026-10-27-g2-scalar-grain/README.md), which retained it. Earlier grouping bypasses and whole-query scheduling negatives remain closed.

## Complete SDK comparison

Ryzen 9950X; seven fixed rotated/reversed fresh-process blocks, all nine shapes at 32 workers on CPUs 0–31 and primary at 16 workers on CPUs 0–15. Primary cases use 151 requests after three primes, other cases 31. All 210 latency processes finish before six separate CPU profiles. Fresh solving, randomized proving, SDK verification and serialization are timed. Loading is excluded from latency and included in whole-process RSS. No owned build, helper, hash, archive, independent verifier or second benchmark overlaps capture. No successful observation is excluded, extended or rerun.

Times are medians of process medians. Ratios, bounds, wins and savings use within-block pairs. CPU is summed user+system time per request across threads; RSS is the maximum lifetime peak, not minimum RAM.

|Case|Accepted ms|OFF ms|ON ms|ON/OFF bounds|Wins|CPU ms/request A/OFF/ON|Max RSS MiB A/OFF/ON|
|---|---:|---:|---:|---|---:|---:|---:|
|1x2-w32|8.039073|8.075161|8.027812|0.996476742 [0.984390159, 1.005386023]|6/7|181.812/182.325/181.853|132.25/132.00/134.00|
|2x2-w32|11.360593|11.344884|11.380913|1.004635488 [0.986587401, 1.022995522]|2/7|262.556/262.615/260.490|266.00/268.00/269.75|
|4x2-w32|15.990973|16.053881|15.993748|0.996093219 [0.967909753, 1.025704791]|4/7|418.074/420.720/419.993|309.00/313.00/308.50|
|8x2-w32|24.008837|24.602444|24.564574|0.992015560 [0.957401009, 1.015932743]|4/7|670.647/675.564/676.145|499.75/502.00/505.50|
|12x2-w32|33.922893|33.920439|33.727424|0.998209719 [0.972038993, 1.013494477]|6/7|942.380/942.886/936.592|712.50/708.75/715.00|
|16x2-w32|39.931070|40.149934|39.982636|1.000315961 [0.983844970, 1.019466044]|3/7|1120.638/1119.192/1122.872|847.00/820.50/833.50|
|24x2-w32|66.049598|66.097270|65.845880|0.996136613 [0.993280868, 1.010984236]|5/7|1891.563/1881.103/1877.746|1271.80/1315.18/1295.50|
|merge-24x1-w32|65.427115|65.693491|65.060987|0.995158104 [0.977843344, 1.006744580]|5/7|1808.687/1813.270/1803.837|1154.79/1171.85/1159.93|
|merge-54x1-w32|161.416106|161.459731|160.768399|0.999583825 [0.983545741, 1.013459396]|4/7|4435.767/4409.530/4451.516|1794.85/1885.55/1890.64|
|1x2-w16|9.852593|9.721515|9.768473|0.993903074 [0.870495299, 1.064972697]|4/7|119.085/119.901/119.720|89.00/88.50/89.25|

Bounds are extrema of seven block ratios, nominal 98.4375% individual coverage under independent/exchangeable block assumptions, without familywise coverage. Every 32-worker OFF/accepted interval must stay within factor 1.03; every 32-worker ON/control upper bound must be ≤1.03. Primary32 and the original-eight block-geometric aggregate require upper<1 and at least 5/7 wins against both controls. Physical16 is descriptive; earlier variability and accepted calibration exceptions remain. A passing gate still requires clean-source, portable, resource and concurrency qualification before adoption.

|Original-eight aggregate|Median|Bounds|Wins|Pass|
|---|---:|---|---:|---|
|original8/serial/accept|1.000606472|[0.999022225, 1.008819831]|3/7|false|
|original8/serial/offset|0.996838822|[0.989479803, 1.001947757]|5/7|false|

Failed checks: `["1x2-w32/serial/offset","1x2-w32/serial/accept","8x2-w32/offset/accept","8x2-w32/serial/accept","original8/serial/accept","original8/serial/offset"]`. Full block pairs and calibration are retained in analysis.json. No threshold is waived.

## Automatic CPU profiling and correctness

After all latency, six separate 199 Hz cycles:u profiles cover primary16/32 and all three variants, 251 requests after three primes. FIFO acknowledgments enable sampling after loading/primes and disable before export. All latency/profile captures finish before their independent export verification and decoding. Raw ownership is transferred only after capture for archival reading, without changing bytes. No counter panel is captured. Each profile has one sampled SDK PID and zero lost records. Sampled CPU weights and visible symbols have inlining/attribution limits; they are not removable wall-time fractions or proof of an exclusive scheduling/cache cause.

|Profile|Samples|Grouping %|Scheduler definitions %|Epoch definitions %|G1 feed %|G2 feed %|
|---|---:|---:|---:|---:|---:|---:|
|cycles-1x2-w16-accept|5370|0.635|0.990|2.763|24.008|20.207|
|cycles-1x2-w16-offset|5516|0.553|0.619|3.591|24.509|19.725|
|cycles-1x2-w16-serial|5501|0.509|0.857|3.793|25.336|19.480|
|cycles-1x2-w32-accept|9106|1.214|1.524|8.296|21.160|18.308|
|cycles-1x2-w32-offset|8973|0.621|1.089|8.542|19.416|18.754|
|cycles-1x2-w32-serial|9185|0.508|1.305|9.519|20.030|18.721|

Postcapture classification review corrects the initial raw-name matcher: perf supplied Rust v0 mangled symbols, so grouping was missed, and embedded rayon_core instantiation references caused epoch functions to be counted as scheduler. The revised table demangles names, classifies only the defining path before generic arguments, and leaves trait/closure contexts in other. Three new focused classifier tests pass. Original profile-categories.json and all raw symbol/period data remain; no profile is rerun.

The classifier helper's initial syntax error was repaired before its first test execution; its initial source is retained. Publication review preserves one trailing blank line in the raw test log, with no authored Rust whitespace findings.

The six profiles contain 43651 samples in total. Raw data hashes and original decoded stdout are retained. Visible phi_split symbols do not capture inlined decomposition and are not used as a complete stage estimate.

Both OFF and ON pass 124 unfiltered core and eight prover tests, including existing independent scalar decomposition and complete G2 generator/exceptional-input checks. No new arithmetic test is claimed for the unchanged scalar function. All 17 field files are exact and the 33-test native field qualification is inherited. Native flags/features and frozen transitive dependencies, including proc macros, are checked. Linked hb_raw disassembly and the selector string are retained; arithmetic binary identity is not asserted. The first inspection failed because objdump -C could not select the mangled symbol. The retained correction removes -C; the original instruction-line assertion then passes. No native build, test or successful SDK capture is repeated.

All capture and proof-verification steps complete. The original workflow completes 498 steps, then stops at its first offline decode because sudo perf rejects files already transferred to ubuntu. A separate owner-run decoder completes the remaining 18 steps; analysis changes only decoder result/stdout paths. Original failure, protocol and sources are retained, with no successful capture or verification repeated. A first archive copy also stops at a missing nested output directory; archive-v2 creates those parent directories and completes the same publication from retained data. Totals: **13,824 internally verified SDK requests, 246 distinct independently Gnark-verified 164-byte exports**, exact image/public bindings and freshness against all 17,830 prior proof identities. Ledger **18,076**, with 492 proof/public members. Every closing source/library/executable/input/helper pin passes. Compressed source, workflow receipt and proof/public archives round-trip byte-for-byte with per-member SHA checks; ELF/library and raw perf artifacts remain local with recorded identities.

This study establishes neither universal memory behavior nor minimum RAM. It creates no AWS resources and supplies no new Aeglos, phone/ARM/TEE, huge-circuit, pressure/swap or concurrent-throughput result.

## Supplied M5 CPU and Metal comparison

Historical supplied references, not new Apple measurements. All eight cases below are newly measured here; each cell shows milliseconds followed by CPU and Metal speedups. These experimental candidate results do not replace accepted timings.

|Shape|M5 CPU/Metal ms|Accepted ms; CPU/Metal ×|OFF ms; CPU/Metal ×|ON ms; CPU/Metal ×|CPU 10× target ms|
|---|---:|---|---|---|---:|
|12x2-w32|284/101|33.922893; 8.372/2.977×|33.920439; 8.373/2.978×|33.727424; 8.420/2.995×|28.4|
|16x2-w32|335/124|39.931070; 8.389/3.105×|40.149934; 8.344/3.088×|39.982636; 8.379/3.101×|33.5|
|1x2-w32|37/24|8.039073; 4.603/2.985×|8.075161; 4.582/2.972×|8.027812; 4.609/2.990×|3.7|
|24x2-w32|528/176|66.049598; 7.994/2.665×|66.097270; 7.988/2.663×|65.845880; 8.019/2.673×|52.8|
|2x2-w32|59/30|11.360593; 5.193/2.641×|11.344884; 5.201/2.644×|11.380913; 5.184/2.636×|5.9|
|4x2-w32|109/48|15.990973; 6.816/3.002×|16.053881; 6.790/2.990×|15.993748; 6.815/3.001×|10.9|
|8x2-w32|189/74|24.008837; 7.872/3.082×|24.602444; 7.682/3.008×|24.564574; 7.694/3.012×|18.9|
|merge-24x1-w32|507/175|65.427115; 7.749/2.675×|65.693491; 7.718/2.664×|65.060987; 7.793/2.690×|50.7|

The authoritative accepted-source comparison remains inherited below. Primary peak remains 134 MiB. The earlier full-performance-gate failure and physical16 calibration engineering exception remain.

|Shape|Accepted ms|M5 CPU speedup|M5 Metal speedup|
|---|---:|---:|---:|
|1x2|8.022165|4.612×|2.992×|
|2x2|11.335969|5.205×|2.646×|
|4x2|15.945544|6.836×|3.010×|
|8x2|24.090485|7.845×|3.072×|
|12x2|33.844855|8.391×|2.984×|
|16x2|39.931557|8.389×|3.105×|
|24x2|66.233309|7.972×|2.657×|
|merge-24x1|65.273224|7.767×|2.681×|

Accepted 54→1 remains 162.250464 ms without a supplied M5 reference. The original all-eight CPU 10× goal remains active and unmet (accepted 0/8). No source/default change is adopted by this experiment.

[Design](artifacts/design.md), [history audit](artifacts/history-audit.md), [all blocks and profiles](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [review](artifacts/review.json), [proof inventory](artifacts/proof-inventory.json).
