# Weighted-tree initialized prefixes under common fixed placement

**The candidate is not adopted.** With identical fixed worker placement, primary 1×2 is **0.43% slower than the accepted libraries** and **0.62% slower than fresh OFF** in the paired comparisons. Both primary gain checks fail, 8×2 fails non-regression, and neither all-eight aggregate shows a supported gain. All ten OFF/accepted calibration checks pass, including the previously unstable 16-worker case. All 594 exported proofs verify independently with Gnark. The all-eight CPU 10× objective remains active and unmet, 0/8.

The [earlier unbound experiment](../2026-11-09-weighted-live-prefix176/README.md) measured a small supported 32-worker gain but failed 16-worker qualification. The [neutral placement study](../2026-11-09-worker-placement-control176/README.md) then showed that individual worker placement sharply reduced 16-worker variation; its separate standard/unbound calibration failure remains retained. This new campaign compares all three library sets under the same fixed placement. It does not waive either earlier failure, replace their observations, or make pinning a universal default.

The private candidate clears and reserves its existing weighted tree, appends initialized leaves, then appends saved even points at each reduction level. This avoids initializing future internal slots to infinity. Point arithmetic, reduction order, inverse modes, representation and maximum reserved tree payload remain unchanged. Existing equal-size reuse already avoids some baseline initialization. The native math/generator/original-tree/worker/generic qualification from the earlier candidate is reused exactly. Only two new SDK drivers are linked, using coherent existing OFF/ON core, Prover and Zolana libraries; the accepted placement driver and all unchanged dependencies are reused. No prover library, field suite or successful old oracle/capture/decoder/verifier is rebuilt or replayed for bookkeeping.

All three fixed arms use byte-identical driver/support sources and the existing global Rayon pool, with worker i pinned to CPU i, default stacks and N+1 total threads. Main retains the broad process mask. Affinity/TID broadcasts are outside request timing. No extra pool, global OS-policy change, key/witness/point/scalar array copy, execution plan, lookup table, C/assembly implementation or artificial RAM cap is introduced. Production source remains unchanged.

## Complete-proof results

Ryzen 9 9950X, Rust 1.98.1, native O3, fat LTO and one codegen unit. Primary 32-worker comparison has 31 matched rotated/reversed blocks of 151 complete SDK requests. Each of the other nine contexts has 15 blocks of 51 requests, except primary 16 workers uses 151. Every process primes three times. Loading, setup and exports are excluded; solving, fresh randomized proving, internal verification and serialization are timed. All eight supplied shapes, merge 54→1 and primary 16 workers are measured. The exact standard SDK accompanies primary cases as a separate unbound reference, not an equivalent-placement control.

The table gives medians of process means. Paired geometric ratios use matched blocks and need not equal ratios of these medians. Ratio above one favors the denominator.

|Context|Accepted fixed ms|OFF fixed ms|ON fixed ms|Paired ON/accepted|Paired ON/OFF|
|---|---:|---:|---:|---:|---:|
|1x2-w32|7.869302|7.843081|7.888466|1.004318|1.006169|
|2x2-w32|10.984868|10.930308|10.934406|0.997014|1.001930|
|4x2-w32|15.734200|15.792521|15.695971|0.993220|0.990163|
|8x2-w32|23.929023|23.952335|24.276897|1.016043|1.020570|
|12x2-w32|33.722646|33.980946|33.629757|0.997517|0.989394|
|16x2-w32|39.776845|40.105564|39.805830|1.002360|0.995371|
|24x2-w32|66.183856|66.413129|66.120746|0.998372|0.993872|
|merge-24x1-w32|65.448733|65.372717|65.749592|1.000697|1.002584|
|merge-54x1-w32|162.489760|162.086034|162.743297|0.998558|1.003019|
|1x2-w16|9.197625|9.144196|9.213924|1.000591|1.005800|

Primary ON/accepted=1.004318, 95% paired bootstrap 1.001820–1.006333, 4/31 wins; ON/OFF=1.006169, interval 1.004540–1.007869, 2/31 wins. This is a measured slowdown in the common fixed condition, not merely an unresolved calibration check. Primary 16-worker OFF/accepted calibrates, all three fixed-arm process-mean CVs are 0.45–0.58%, and both ≤1.02 ON non-regression bounds pass there. Its separate standard reference still has 10.47% CV.

At 8×2, ON/accepted=1.016043 with upper bound 1.035879; ON/OFF=1.020570, interval 1.008283–1.034698. Both ≤1.02 non-regression checks fail. The equal-log all-eight ratio is 1.001172 versus accepted, stratified interval 0.997480–1.004740, and 0.999961 versus OFF, interval 0.997216–1.002845. Neither selects aggregate improvement. Some ON/OFF comparisons at 4×2, 12×2 and 24×2 are favorable; they remain visible and do not replace the full selection criteria. There is no minimum percentage-gain floor.

All ten calibration, all ten RSS and all profile-quality checks pass. Eight of 45 checks fail: four primary gain/win checks, two 8×2 non-regression checks and two aggregate-gain checks. No failed row, block or shape is excluded. The original unbound signal remains a result for its own condition; it does not establish a universally beneficial initialization change. No unbound-only dispatch, changed recipe or repeated campaign is selected to rescue this variant. Resource/admission/portable/concurrent adoption qualification is not continued after the performance gate fails.

Bootstrap uses 20,000 resamples, seed 0x551c36a48ee2d971 and ranks 499/19,499. Matched blocks are assumed exchangeable; all-eight resampling is independent within each shape. Cross-shape/shared-host correlation and familywise coverage are not established. The inherited statistics payload's `different_campaign_blocks_not_treated_as_synchronized` label describes its resampling rule; all current observations actually belong to this one campaign. It is not evidence of separate campaigns.

## Verification, placement and resources

All 1,233 recording/offline commands finish successfully: 594 recordings, 594 independent Gnark verifications and 45 ownership/decode/event/metadata/loss reads. The frozen analyzer and separately compiled same-agent reader also finish successfully. Prior setup adds two SDK-only links and one prospective review; no native library/test rebuild. Total: **51,222 complete SDK requests**, including three primes per process, and **594 fresh unique Gnark-verified final exports**. The canonical proof inventory advances 38,074→38,668; primes are not counted as exported identities. Image, public-input, VK and proof hashes agree for every export.

All recordings finish before independent verification and offline analysis. Only the exact confirmed-live session is polled during recording. No helper, build, hash, analyzer, verifier, report, archive or push overlaps it. Actual session 23150 terminates successfully; no observation timeout causes a restart. The result reader reconstructs raw means, resources, ratios, aggregate gates, all-eight reference comparisons, affinity cohorts and proof bindings. Bootstrap, binding and detailed profile parsers are shared; this is not an external audit. Floating comparisons use 1e−12 tolerance; integers/hashes/counts are exact.

Two manual pre-clock helper compilations initially fail: extracted stratified statistics need a BTreeMap import; a JSON array expression needs parentheses before `.map`. Narrow syntax/import fixes precede first execution. Original sources remain. Arithmetic, runtime paths, tests, cases, gates and measured observations do not change. Inherited unused-helper warnings remain without cleanup rebuilds.

All 546 fixed-driver processes retain worker identities and masks, unique indices and N+1 tasks. Every sampled worker TID in all nine profiles executes on its assigned CPU. Fixed placement improves the usefulness of the control, but the remaining variation in some larger contexts is retained.

|Context|Accepted / OFF / ON peak process MiB|Accepted / OFF / ON minor faults per request|
|---|---:|---:|
|1x2-w32|134.25 / 134.00 / 134.25|129.11 / 130.05 / 127.03|
|2x2-w32|297.25 / 293.75 / 299.50|883.29 / 879.67 / 890.27|
|4x2-w32|330.25 / 335.50 / 340.00|789.86 / 780.98 / 782.39|
|8x2-w32|548.00 / 535.50 / 536.25|1192.67 / 1165.33 / 1167.82|
|12x2-w32|767.75 / 757.25 / 751.50|1612.27 / 1623.57 / 1614.69|
|16x2-w32|846.00 / 866.75 / 855.00|1646.96 / 1671.12 / 1717.08|
|24x2-w32|1357.36 / 1376.89 / 1376.00|5070.29 / 4881.75 / 5949.96|
|merge-24x1-w32|1223.57 / 1252.50 / 1244.00|2587.57 / 3118.71 / 3174.14|
|merge-54x1-w32|2078.51 / 2057.85 / 2040.93|21303.14 / 21300.88 / 21117.65|
|1x2-w16|86.75 / 87.00 / 86.50|54.80 / 55.23 / 56.23|

Timed major faults are zero in all fixed-arm processes. Standard references do not expose that counter. RSS includes loading, allocator and worker state; it is not minimum RAM or a memory-pressure/phone/swap/TEE qualification. Fewer source zero writes do not imply lower live capacity, lower process RSS or better latency.

## Automatic CPU profiles

All nine profiles qualify: 197,703 samples, 100% assigned execution, unknown share 0.698–1.772%, zero LOST. All 36 grouped PMU event values run 100% of enabled time. Primary uses 501 profile requests and 31 PMU requests; merge 54 uses 17 and 7. Cycles:u period is 16,000,000 with 512 mmap pages; FIFO acknowledgements gate recording after priming.

|Context / arm|Instructions/proof, billions|Cycles/proof, millions|Cache misses/proof, millions|
|---|---:|---:|---:|
|1x2-w16-OFF|1.348286|565.496|1.908|
|1x2-w16-ON|1.348350|568.541|1.877|
|1x2-w16-accepted|1.348181|569.047|1.871|
|1x2-w32-OFF|1.387661|810.687|2.690|
|1x2-w32-ON|1.386070|815.901|2.655|
|1x2-w32-accepted|1.386848|815.977|2.679|
|merge-54x1-w32-OFF|18.012068|20786.956|77.427|
|merge-54x1-w32-ON|18.028763|21565.969|82.929|
|merge-54x1-w32-accepted|18.030552|20892.172|77.739|

The instruction counts show no material full-proof reduction. A single panel per arm does not isolate Vec bounds checks, code layout, stores, cache/DRAM traffic, faults or an exclusive critical-path fraction. The result closes this specific safe initialized-prefix variant; it does not rule out every initialization or representation optimization. The next distinct implementation should target actual G2/KH arithmetic or query work, informed by current profiles and the earlier [query-work sensitivity diagnostic](../2026-11-09-query-neutralization-sdk176/README.md). Cached same-key/witness query outputs remain diagnostic only.

## CPU and Metal references for every supplied shape

All fixed measurements below are fresh in this campaign. The ON candidate is unadopted; these are conditional fixed-placement comparisons on the Ryzen host against user-supplied M5 references on different hardware. They do not replace standard accepted SDK observations.

|Shape|Accepted fixed ms|ON fixed ms|M5 CPU ms / ON speedup|M5 Metal ms / ON speedup|CPU 10× limit ms|
|---|---:|---:|---:|---:|---:|
|1x2|7.869302|7.888466|37 / 4.690×|24 / 3.042×|3.7|
|2x2|10.984868|10.934406|59 / 5.396×|30 / 2.744×|5.9|
|4x2|15.734200|15.695971|109 / 6.944×|48 / 3.058×|10.9|
|8x2|23.929023|24.276897|189 / 7.785×|74 / 3.048×|18.9|
|12x2|33.722646|33.629757|284 / 8.445×|101 / 3.003×|28.4|
|16x2|39.776845|39.805830|335 / 8.416×|124 / 3.115×|33.5|
|24x2|66.183856|66.120746|528 / 7.985×|176 / 2.662×|52.8|
|merge-24x1|65.448733|65.749592|507 / 7.711×|175 / 2.662×|50.7|

Merge 54→1 accepted/OFF/ON medians are 162.489760/162.086034/162.743297 ms, without a supplied M5 reference. Fresh separate standard primary is 8.075143 ms at 32 workers and 10.605243 ms at 16. Historical same-standard-build 7.915095 ms remains valid; this campaign establishes no adopted regression. The accepted fixed primary 7.869302 ms agrees closely with the preceding neutral fixed observation 7.869279 ms, but neither is substituted for standard history. No new AWS/Aeglos/Gnark latency or concurrent-throughput comparison is made. No cloud resource is created. All eight CPU 10× targets remain unmet, 0/8.

[Analysis](artifacts/analysis.json), [profiles/counters](artifacts/diagnostic-analysis.json), [result review](artifacts/review.json), [prospective review](artifacts/plan-review.json), [protocol](artifacts/protocol.json), [design](artifacts/design.md), [continuation loop](artifacts/next.md), [source/logs/receipts](artifacts/evidence.json.gz), [checksums](SHA256SUMS). The archive retains individual UTF8-member SHA256 hashes and verified gzip roundtrip. Executables/libraries, perf.data, keys/witnesses/proofs and full private ledger payloads are excluded; proof hashes and verifier receipts remain. Reproduction needs pinned dependencies/inputs and adapted absolute paths.
