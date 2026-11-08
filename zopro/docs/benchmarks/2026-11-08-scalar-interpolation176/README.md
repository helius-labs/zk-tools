# Scalar and vector work in complete affine additions

**This fixed implementation is slower and is not adopted.** At 32 workers, the four-size G1 aggregate is 41.01% slower than fresh OFF and G2 is 49.62% slower. The all-24-context aggregate is 49.40% slower. All six aggregate comparisons have zero wins out of 15. Correctness, allocation, scratch bounds and profile quality pass. Production and accepted proof times are unchanged.

The hypothesis comes from [Ghanbari's 2025 Scalar Interpolation thesis](https://webdocs.cs.ualberta.ca/~amaral/thesis/RezaGhanbariMSc.pdf), chapters 3–4: whole scalar iterations may use execution resources left idle by vector work. Applying this idea to BN254 integer arithmetic is an engineering experiment; the thesis's other workloads do not predict prover performance. No compiler implementation is copied.

The candidate performs one original scalar-field addition alongside each eight-lane IFMA group, with a fixed nine-point stride. A final full eight-point remainder stays vectorized. Scalar and vector prefix chains share one existing eight-chain inverse by merging the scalar product into root zero, then recovering the separate inverses with three scalar products. Original field backends, point formulas, signs and exceptional-input contract remain. G1 reuses groups and tail records; G2 packs five scalar [prefix,norm] pairs into an existing tail record and reloads coordinates on reverse. There is no new field backend, assembly, scratch field, table, key copy, retained plan or production pool.

The initial G1 implementation uses push records rather than the accepted in-place record writes. Root joining, representation transfers and this record policy are part of the measured implementation. The result does not isolate a general scalar-interpolation effect or refute the paper. This exact recipe closes without an interpolation-factor, record-write, layout, attribute, compiler, worker, subset or host rescue.

|Aggregate|Candidate / fresh OFF, fixed rank interval|Wins / 15|Candidate / frozen, fixed rank interval|Wins / 15|
|---|---:|---:|---:|---:|
|G1|1.410100497 [1.365429228, 1.469313749]|0|1.400406409 [1.332001661, 1.446555843]|0|
|G2|1.496249107 [1.489666772, 1.499949805]|0|1.508470283 [1.502891706, 1.516152405]|0|
|ALL|1.493958009 [1.482770990, 1.501327990]|0|1.498808673 [1.486967306, 1.506210709]|0|

G1/G2 aggregate rows combine their four sizes at 32 workers; ALL combines all 24 contexts with equal log weights inside each block. Paired summaries need not equal ratios of unpaired medians. The fixed screen fails 60 of 78 timing checks: six fresh-control calibration checks, all 48 nonregression checks and all six aggregate gain checks. All 24 candidate/OFF median ratios exceed one. The six calibration failures remain visible and limit those control comparisons; they do not erase the separately calibrated solo regressions.

Ryzen 9 9950X, 16 physical cores/32 logical CPUs, shared host. Both curves, sizes 200/201/352/353, and 1/16/32 workers give 24 contexts. The frozen executable is the accepted-source diagnostic caller, not the production SDK. Fresh OFF and ON reuse the exact accepted field libraries with coherent optimized core/driver links. Drivers retain O3, native CPU, fat LTO, one codegen unit and the original debug setting.

Each process uses changing public-generator coordinates, independent generator-coefficient output checks and 512 KiB diagnostic worker stacks. Solo uses CPU 2; 16/32 workers use CPUs 0–15/0–31. Three checked 128-round primes and reset precede 8,192 timed rounds per worker. The clock includes complete additions, inversion, dispatch and borrowed-reference setup. Output checks follow the clock. These fixtures do not constitute actual SDK inputs or proof latency.

Seventy-two preflights and one checker precede 15 fixed rotated/reversed blocks across every context and all three arms. All 1,080 latency processes finish before 18 PMU panels and 18 cycle profiles; all ownership, decoding and quality commands follow recordings. There are 1,279 commands and 1,188 checked caller processes. No owned build, helper, hash, analysis, report or push overlaps any recording. No observations are excluded or repeated.

The prospective rule uses zero-based sorted ranks 3/7/11 of 15 paired ratios. Fresh controls must lie within factor 1.03, all candidate nonregression upper bounds must be ≤1.02, and each aggregate must have upper <1 and at least 10 wins against both controls. There is no minimum-gain floor. All raw ratios and failures remain in the [analysis](artifacts/analysis.json). These marginal rank intervals are not simultaneous family-wise guarantees.

|Context|OFF µs/wave|Candidate µs/wave|Candidate / OFF, fixed rank interval|Wins / 15|
|---|---:|---:|---:|---:|
|G1-200-w1|4.241|6.115|1.441775285 [1.440205798, 1.445724233]|0|
|G1-200-w16|4.818|6.979|1.447840318 [1.442182262, 1.453418240]|0|
|G1-200-w32|6.900|9.865|1.419761281 [1.402111630, 1.432085921]|0|
|G1-201-w1|4.357|6.220|1.428565648 [1.425159734, 1.430296791]|0|
|G1-201-w16|4.940|7.086|1.433520091 [1.423872626, 1.438384168]|0|
|G1-201-w32|7.078|10.001|1.408771962 [1.398017948, 1.436708240]|0|
|G1-352-w1|6.978|9.686|1.388512089 [1.387788548, 1.390247409]|0|
|G1-352-w16|7.970|11.137|1.396980887 [1.393841029, 1.400021422]|0|
|G1-352-w32|11.583|16.660|1.399229980 [1.319377033, 1.456008061]|0|
|G1-353-w1|7.085|9.855|1.390809046 [1.389796436, 1.395226651]|0|
|G1-353-w16|8.090|11.314|1.398014770 [1.394958435, 1.401743394]|0|
|G1-353-w32|11.733|17.005|1.446281320 [1.388826331, 1.663567787]|0|
|G2-200-w1|7.873|12.824|1.630240719 [1.628510838, 1.631830216]|0|
|G2-200-w16|9.115|14.981|1.645093814 [1.642044619, 1.647212286]|0|
|G2-200-w32|14.597|22.205|1.511918071 [1.500613423, 1.529995085]|0|
|G2-201-w1|8.299|13.044|1.570475350 [1.568166923, 1.572019350]|0|
|G2-201-w16|9.611|15.222|1.584201269 [1.579557416, 1.586653339]|0|
|G2-201-w32|15.141|22.491|1.481091857 [1.448752979, 1.510229430]|0|
|G2-352-w1|13.209|21.585|1.634333771 [1.633283979, 1.635335688]|0|
|G2-352-w16|15.379|25.299|1.646851836 [1.642819667, 1.647194458]|0|
|G2-352-w32|24.941|37.312|1.496034976 [1.494019716, 1.504246434]|0|
|G2-353-w1|13.606|21.782|1.601692763 [1.600530713, 1.603023734]|0|
|G2-353-w16|15.842|25.541|1.610635476 [1.607634652, 1.614514094]|0|
|G2-353-w32|25.361|37.648|1.485400716 [1.480695823, 1.489169351]|0|

All caller oracles and cross-arm output hashes match. Timed allocations are zero. Scratch is constant within each arm and worker population, and candidate capacity stays within the predeclared max(OFF,frozen)+1,024 bytes per worker. This is a comparison check, not an architecture memory cap. The source workspace planner conservatively charges the maximum of old/new buffer growth.

|Curve / points|OFF retained scratch bytes/worker|Candidate retained scratch bytes/worker|
|---|---:|---:|
|G1-200|40000|39040|
|G1-201|40640|39200|
|G1-352|70400|68800|
|G1-353|71040|68960|
|G2-200|16000|15680|
|G2-201|17280|15680|
|G2-352|28160|27520|
|G2-353|29440|27840|

Maximum observed candidate caller RSS is 19.00 MiB, including setup. This is not prover peak RSS, minimum RAM, pressure/swap behavior or concurrent-proof throughput. Zero warm-caller allocations do not imply zero allocations during fresh proof setup. Scratch decreases in this matrix, while latency increases.

|Diagnostic context|Instructions ON/OFF|Cycles ON/OFF|
|---|---:|---:|
|G1-200-w1|1.616514|1.453288|
|G1-200-w32|1.616400|1.451510|
|G1-352-w32|1.628728|1.398679|
|G2-200-w1|1.666332|1.652597|
|G2-200-w32|1.666245|1.501908|
|G2-352-w32|1.652435|1.501502|

Measured instructions rise about 61–63% for G1 and 65–67% for G2. Cycles also rise; in some panels instructions per cycle improve despite slower execution. Cache-miss changes differ by context, including decreases with slower G2 callers. These observations establish more executed work in this implementation; they do not identify an exclusive execution-port, register-pressure or DRAM-capacity cause.

All 18 profiles qualify: 140,997 samples, zero LOST, assigned workers checked, minimum 99.658% target samples and maximum 0.091% unknown. Grouped counters meet the 99.9% running criterion. Profiles use cycles:u, period 16,000,000, 512 mmap pages and acknowledged FIFO gating after primes. PID/TID/CPU/MMAP metadata and complete symbols are retained. Flat cycle attribution is not critical-path wall time. The [static-count clarification](artifacts/codegen-interpretation.md) distinguishes whole-function sites from executed branch work; no decoder or measurement is repeated for it.

Native ON passes 128 core tests: 125 existing and three new tests for partition/nine-root recovery, signed tails/reuse/capacity, and concurrent small stacks. OFF passes the two applicable new caller tests with existing tests filtered. Original field qualification is inherited by exact field source/library identity. Of 176 production files, 173 private copies are exact and three parents reverse to their originals.

The first generic compilation failed in the private native-only benchmark adapter. A separate portable view removes only its declaration and root reexports, retaining all production arithmetic/dispatch/resource edits. The generic/noASM/noIFMA check then passes. The eight successful native stages and original source/binaries are retained. This is a compile check without the native adapter, not ARM or phone execution. Both analyzers pass on first execution; the independent reader checks 629 pins, all commands/callers, 78 timing checks and all profiles without replaying successful work.

This caller trial creates zero SDK requests, exported proofs or cloud resources. Canonical proof inventory remains 35,050. The accepted local proof comparison below is **inherited**, including the latest 1×2 pilot and seven explicitly inherited full-SDK rows from the [preceding report](../2026-11-08-packed-miller-sdk176/README.md). Supplied M5 references use different hardware.

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.995682|37.0|24.0|4.627×|3.002×|3.7|
|2x2|11.241463|59.0|30.0|5.248×|2.669×|5.9|
|4x2|15.933816|109.0|48.0|6.841×|3.012×|10.9|
|8x2|24.234311|189.0|74.0|7.799×|3.054×|18.9|
|12x2|33.855565|284.0|101.0|8.389×|2.983×|28.4|
|16x2|40.071563|335.0|124.0|8.360×|3.094×|33.5|
|24x2|65.917462|528.0|176.0|8.010×|2.670×|52.8|
|merge-24x1|65.305023|507.0|175.0|7.764×|2.680×|50.7|

All eight verified CPU 10× requirements remain active and unmet, **0/8**. Merge54 remains 162.240109 ms without an M5 reference. There are no new AWS/Aeglos, 250/4999, mobile/ARM/TEE, pressure/swap or minimum-RAM measurements. This fixed interpolation recipe does not select an SDK trial or adoption.

The [evidence archive](artifacts/evidence.json.gz) retains exact prospective plans/pins, helpers, sources, native/portable receipts, optimized disassembly, raw latency outputs, full decoded profiles and PMU CSVs. The public analysis view omits only the repeated command/caller receipt arrays; the full independently reviewed `analysis.json` remains in the archive with its original hash. Decompression yields a JSON map of relative paths to {utf8,bytes,sha256}; every extracted payload is verified. Binaries/libraries, raw perf data, keys/images, witnesses/point payloads, private ledgers and full source copies are excluded, with relevant identities preserved. No report, benchmark or source is uploaded to an external analysis service.
