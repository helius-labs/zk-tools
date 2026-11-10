# Rust 1.99.1: complete current-source CPU comparison

**The newer compiler build is rejected for adoption.** Local 32-worker 1×2 takes **7.955486 ms** with Rust 1.99.1, versus **7.850647 ms** for fresh Rust 1.98.1 and **7.863367 ms** for the accepted executable. ON/OFF paired ratio is **1.012421 [1.010377, 1.014264]**, with **2/31 wins**. The all-eight equal-log aggregate is **0.65% slower** than fresh OFF and **0.93% slower** than accepted REF. Production and the accepted toolchain remain unchanged.

All 11 control calibrations, 22 latency nonregressions, 22 RSS guards and 18 diagnostic-quality checks pass. The four required primary gain/win checks and two all-eight gain checks fail: **73/79 pass**. Independent raw-data review passes and reproduces every decision. This is a calibrated negative gain result; the exact recipe closes without sample extension, favorable-subset promotion, compiler-flag/layout sweep, unchanged rerun or AWS rescue.

This compares whole toolchain packages, not LLVM alone. REF is the accepted Zen5 Rust1.98.1 executable; OFF is a fresh 1.98.1 / LLVM22.1.8 build; ON is a fresh 1.99.1 / LLVM23.1.1 build. All 176 source files and Cargo.lock match accepted bytes in both arms, including original registry Rayon. Dependencies, accepted manual cfg flags including `zopro_dense_busy_reset`, native O3, fat LTO, one codegen unit, debug information, allocator, SDK driver, default stacks and fixed worker placement are preserved. Explicit compiler paths and matching eight-library sets are checked. No source algorithm, bitmap, model checker, key table, expanded plan or production pool is introduced. [Native/generic and preflight qualification](../2026-11-10-compiler199-qualification176/README.md) preceded the performance run.

Ryzen 9950X, 16 physical cores / 32 logical CPUs, shared host without an exclusivity claim. Solo uses CPU2, 16 workers use0–15 and32 use0–31. All processes are serialized.

|Condition|Accepted REF ms|Fresh 1.98 OFF ms|Fresh 1.99 ON ms|ON/OFF ratio [95% CI]; wins|ON/REF ratio [95% CI]; wins|
|---|---:|---:|---:|---|---|
|1x2-w32|7.863367|7.850647|7.955486|1.012421 [1.010377, 1.014264]; 2|1.010904 [1.009076, 1.012708]; 0|
|2x2-w32|11.051833|10.976950|11.135709|1.012818 [1.008169, 1.017783]; 1|1.008979 [1.002820, 1.014792]; 3|
|4x2-w32|15.850652|15.817109|15.859860|1.011166 [1.003651, 1.019574]; 5|1.009100 [1.002047, 1.016936]; 8|
|8x2-w32|24.050327|24.117065|24.154109|1.000762 [0.995736, 1.005327]; 8|1.005199 [0.995106, 1.013789]; 4|
|12x2-w32|33.877870|34.161258|34.201693|0.998820 [0.991988, 1.005283]; 7|1.008460 [1.003129, 1.013639]; 3|
|16x2-w32|39.777696|40.031213|40.301005|1.008107 [1.000745, 1.015675]; 3|1.015605 [1.010450, 1.021391]; 0|
|24x2-w32|66.208517|66.494671|66.600899|0.999543 [0.992573, 1.006455]; 9|1.005703 [1.000833, 1.010206]; 3|
|merge-24x1-w32|65.473144|65.857415|66.265366|1.008438 [1.003588, 1.013496]; 2|1.010562 [1.004365, 1.016531]; 4|
|merge-54x1-w32|162.652338|163.326040|165.176430|1.011670 [1.007831, 1.015278]; 1|1.015358 [1.008873, 1.021047]; 1|
|1x2-w1|95.058168|94.549586|95.634538|1.010153 [1.007869, 1.012259]; 0|1.003544 [1.001726, 1.005387]; 4|
|1x2-w16|9.257516|9.199577|9.345533|1.014018 [1.012333, 1.015675]; 0|1.011105 [1.009308, 1.012853]; 1|

Displayed latency is the median of process means. Ratios use matched process means and need not equal ratios of displayed medians. Primary16/32 each use31fixed blocks×151requests; other conditions use15×31, except solo with11requests. Case and arm order rotate/reverse prospectively. Three primes precede each process's measured requests. The SDK interval includes fresh solving, randomized proving, internal verification and serialization; load, pool creation, primes and export are excluded.

The inherited qualified statistic uses20,000paired-log bootstrap resamples, SplitMix64seed `0x551c36a48ee2d971`, and ranks499/19,499. All-eight equal-log aggregation resamples independently within each shape, with31primary and15other blocks; blocks across different circuits are not treated as simultaneous. ON/OFF is1.006494 [1.004403,1.008597]; ON/REF is1.009309 [1.007118,1.011464]. Nominal individual intervals assume exchangeable blocks and carry no familywise guarantee.

Primary16 ON/OFF is1.014018 [1.012333,1.015675],0/31wins; solo is1.010153 [1.007869,1.012259],0/15wins. Merge54 ON/OFF is1.011670 [1.007831,1.015278],1/15wins. Neither a smaller-worker nor a large-shape speedup is established.

|Condition|REF CPU ms / peak MiB|OFF CPU ms / peak MiB|ON CPU ms / peak MiB|
|---|---:|---:|---:|
|1x2-w32|178.602 / 135.000|178.083 / 135.500|178.968 / 133.250|
|2x2-w32|258.962 / 277.750|257.346 / 270.500|259.915 / 277.250|
|4x2-w32|421.525 / 315.000|422.565 / 313.000|421.127 / 311.750|
|8x2-w32|676.040 / 504.000|677.091 / 507.500|676.098 / 513.500|
|12x2-w32|950.657 / 720.000|955.782 / 710.000|955.475 / 721.250|
|16x2-w32|1134.245 / 835.000|1143.345 / 835.750|1141.772 / 813.750|
|24x2-w32|1923.949 / 1312.191|1942.558 / 1319.453|1938.895 / 1339.148|
|merge-24x1-w32|1851.437 / 1167.500|1859.264 / 1171.750|1872.196 / 1175.992|
|merge-54x1-w32|4521.741 / 1986.941|4544.354 / 1910.402|4588.183 / 1983.676|
|1x2-w1|95.057 / 34.250|94.546 / 34.250|95.632 / 34.750|
|1x2-w16|118.198 / 89.500|117.375 / 88.250|118.599 / 89.750|

CPU is the median summed user+system CPU time per request. RSS is the maximum lifetime process observation among latency processes, including load, primes and allocator history; it is not incremental scratch, minimum RAM or a pressure/concurrency capacity result. All22declared RSS guards pass, but exact equality is not claimed.

Primary32 OFF/ON CPU is178.083/178.968ms, peak RSS135.50/133.25MiB, user CPU173.496/174.389ms and system CPU4.583/4.578ms per request. Median voluntary switches are358.76/364.62, involuntary3.04/3.26, minor faults132.19/127.19 and major faults0/0. Separate medians need not add exactly. Switch counts are not syscall counts; minor faults are not disk paging and do not quantify servicing time. No large reduction in useful work is evident from these observations.

Nine FIFO-gated PMU panels each contain31requests; nine separate cycle profiles each contain251requests. They cover primary16/32 and2×2/32, all three arms. Every PMU event has100%running time, exceeding the frozen99.9%rule. All profiles qualify: **119,161 samples**, zero reported LOST,1.423–2.4033%unknown, fixed16,000,000`cycles:u` period and checked actual binary/PID/TID/CPU ownership.

Primary32 OFF/ON per-request cycles are804.24M/809.83M; instructions1,370.46M/1,375.68M; cache misses2.636M/2.693M; branch misses0.878M/0.876M. Flat epoch-symbol shares are9.99%/9.91% and runtime-symbol shares4.80%/4.67%. These single panels have no paired uncertainty estimates. Runtime functions can contain inlined useful payload, absolute sampled-period coverage remains unvalidated, and symbol shares are not idle time or removable wall time. The data does not isolate a compiler pass, instruction sequence, library change, cache effect or scheduling cause.

|Declared check group|Pass / total|
|---|---:|
|ALL8_gain|0 / 2|
|PMU_quality|9 / 9|
|RSS|22 / 22|
|calibration|11 / 11|
|nonregression|22 / 22|
|primary_gain_and_wins|0 / 4|
|profile_quality|9 / 9|

Calibration requires OFF/REF's entire paired interval inside[1/1.03,1.03]. Candidate/control latency upper bounds must be≤1.03; lifetime RSS maxima must be≤1.05×each control maximum. Primary32 needs upper ratio<1 and≥21/31wins against both controls. Both all-eight aggregate upper bounds must be<1. Profiles need≥1,000samples,≤5%unknown, zero LOST and checked period/ownership. All rules are frozen before recording, with no minimum positive gain floor. The six failures are:

```text
1x2-w32/ON/OFF/primary-gain
1x2-w32/ON/OFF/primary-wins
1x2-w32/ON/REF/primary-gain
1x2-w32/ON/REF/primary-wins
ALL8/ON/OFF/gain
ALL8/ON/REF/gain
```

Native1.99qualification passes35field+127core+2prover+6cubic tests; generic no-ASM/no-IFMA qualification passes11field+97core+2prover+6cubic. These are host tests, not physical ARM/mobile execution. Original1.98correctness is inherited after exact source/flag checks; its fresh build is the calibration control. Both compilers' libraries and ELF version markers are bound. Before SDK links, the pin extension includes all compiler root library files, including the actual non-`.so`-suffix LLVM target. No successful compilation/test is repeated for this extension. There are no failed runtime/helper workflow commands or performance-reader repairs in this stage.

All **1,331 serialized workflow commands pass**:8version/build/test,4explicit link/symbol,4helper builds,52functional preflight and1,263capture/offline/verifier commands. Preflight contributes104SDKcalls/26freshGnarkexports. Performance contributes591latency+9PMU+9cycle processes,44,106SDKcalls including primes, and609fresh distinct exports. All recordings finish before45offline commands and609independent Gnark checks. Total **44,210SDK requests and635new distinct, independently Gnark-verified exports**. The canonical ledger advances **46,802→46,828→47,437**. No request, successful native qualification, decoder or verifier is rerun for bookkeeping. Direct preparers/readers/publication compiles and the successful PMU capability check are outside the workflow-command count.

A separate reader in the same agent reconstructs all591raw process means,609proof bindings,79decisions and profile ownership; it shares qualified statistics/diagnostic parsers, so this is not an external audit or independent implementation of those components. Supplementary closeout checks every command/stream receipt and pin occurrence,198resource medians, profile category counts, exact176production/private source hashes and the unique47,437-entry ledger. Production remains unchanged.

The original verified CPU AVX single-proof ≥10× supplied M5 CPU objective remains **active and unmet,0/8**. Fresh accepted/OFF/ON values below are this campaign; supplied M5 CPU/Metal values are historical references on different hardware.

|Shape|Accepted REF ms|Fresh1.98 OFF ms|Rejected1.99 ON ms|M5 CPU ms|M5 Metal ms|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.863367|7.850647|7.955486|37|24.0|3.7|
|2x2|11.051833|10.976950|11.135709|59|30.0|5.9|
|4x2|15.850652|15.817109|15.859860|109|48.0|10.9|
|8x2|24.050327|24.117065|24.154109|189|74.0|18.9|
|12x2|33.877870|34.161258|34.201693|284|101.0|28.4|
|16x2|39.777696|40.031213|40.301005|335|124.0|33.5|
|24x2|66.208517|66.494671|66.600899|528|176.0|52.8|
|merge-24x1|65.473144|65.857415|66.265366|507|175.0|50.7|

54→1has no supplied M5 reference. No new AWS or Aeglos measurement occurs here. The [accepted c7a comparison](../2026-11-10-accepted-current-c7a176/README.md) remains separate: best tested1×2Zopro11.243892ms/32workers versus Aeglos7.902802ms/96workers; at32Aeglos11.870012ms. Best54→1Zopro120.757128ms versus Aeglos171.908504ms. These are accepted Rust1.98AWS observations, not measurements of this rejected1.99build. No owned cloud resource is added.

The next investigation returns to a distinct reduction in actual online G2/KH work or its required dependencies. This compiler package supplies no measured improvement. Existing bitmap and algebraic/table studies remain recorded separately; no unchanged rescue is selected. No new huge250/4999, phone/ARM/TEE, pressure/swap, minimum-RAM or concurrent-throughput claim follows.

Evidence: [analysis](artifacts/analysis.json), [raw-data review](artifacts/review-result.json), [closeout](artifacts/closeout.json), [resources/profiles](artifacts/report-summary.json), [identities](artifacts/identities.json), [source audit](artifacts/source-audit.json), [protocol](artifacts/protocol.md), [authored helpers, raw rows, plans and streams](artifacts/evidence.json.gz), [nine original perf captures](artifacts/perf-data.tar.gz), [publication checks](artifacts/quality.json), [checksums](SHA256SUMS). Archives are losslessly compressed and hash-bound. Keys, requests, solver/witness/proof payloads, full canonical ledgers, executable/library blobs, unchanged production source and complete third-party trees are excluded. Absolute local paths require adaptation.
