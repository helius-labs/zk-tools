# Lane-wise final exponentiation after packed Miller loops

**The lane-wise finish is slower than the prior packed-Miller/scalar-finish strategy at every worker count. At 32 workers its paired regression is 12.41%. All control calibrations pass; 13 of 18 checks fail.** Production remains unchanged. This prototype keeps three independent pairing factors packed through final exponentiation, then multiplies the three target-group values. It is a distinct computation from the earlier packed-Miller/scalar-finish strategy. Its historical gate remains false. These are synthetic complete pairing-arithmetic consumers, not SDK proof or complete verifier timings.

## Computation and correctness

For nonzero factors, final exponentiation is multiplicative: FE(a*b*c)=FE(a)*FE(b)*FE(c). Any zero factor preserves the original inverse failure. Three exponentiations replace one, using six active base-field lanes and persistent packed Fq12 state. The original fixed-X binary schedule, tower formulas, scalar field1200 and prepared lines remain unchanged. A scalar bridge inverts four base-field norms; its cost is timed. No proving-key table, input-sized side buffer or new production pool is introduced.

The new isolated source recovers the old packed Miller source exactly after reversing two replacements and a suffix. Frobenius constants recover the original Fq6/Fq12 source text after reversing visibility. Old sources/binaries/cache/results are untouched. Native qualification passes80 complete final outputs(16cached,64newraw),80 inverse identities,80 conjugates,960 Frobenius outputs,160 easy-subgroup identities and80 cyclotomic/general-square comparisons. All 15 zero-lane masks reject through inverse/easy/final entrypoints. Multiplicativity checks191new combined arguments and reuses3known arguments/individual final oracles. Sixteen cached end-to-end Miller finishes and49actual workers across1/16/32pools pass on512 KiBstacks. No field/core suite, old capture, solver/key/input or successful individual oracle was rerun. Warnings for unused diagnostic methods remain without cleanup rebuilds.

## Fixed complete-consumer comparison

Ryzen 9 9950X, native optimized/fatLTO/one-codegen-unit executable and coherent frozen dependencies. Four arms: retained scalar policy(native), exact-copied scalar policy(fresh), previous packed Miller with scalar final exponentiation(packed), and the new lane-wise finish(fully). Native/fresh use the current three-way Rayon Miller policy at16/32workers and shared recurrence solo. All use the same eight cached nondegenerate point tuples and full-field oracles.

Thirty-one fixed rotated/reversed process blocks,101 timed calls after 7 checked primes, actual CPU/TID pins and512 KiBworker stacks. Each process contributes median 101 samples. Clocks include pool.install dispatch/join, all arithmetic, allocations, extraction and final product; loading/static lines, dynamic proof-B preparation and public-input MSM are excluded. Every complete output is checked outside clocks. Ratios pair blocks and need not equal ratios of displayed medians.

| Workers | Native µs | Fresh µs | Prior packed µs | Fully packed µs | Fully/native [bounds],wins | Fully/fresh [bounds],wins | Fully/prior [bounds],wins |
|---|---:|---:|---:|---:|---|---|---|
| 1 | 383.542 | 383.472 | 290.026 | 326.194 | 0.850286[0.849671,0.851772],31/31 | 0.850565[0.849773,0.851751],31/31 | 1.124779[1.123364,1.125492],0/31 |
| 16 | 275.498 | 276.000 | 287.822 | 324.401 | 1.183966[1.174709,1.187860],0/31 | 1.179348[1.165604,1.189315],0/31 | 1.129788[1.123065,1.134227],0/31 |
| 32 | 281.560 | 284.976 | 289.014 | 325.562 | 1.168962[1.135681,1.191051],2/31 | 1.163744[1.137994,1.178178],0/31 | 1.124066[1.116790,1.132486],0/31 |

All-three equal-log aggregates: native `{"high":1.0669481480193552,"low":1.0441684411681544,"median":1.0560997260008145,"wins":2}`, fresh `{"high":1.0609727064665042,"low":1.039671999505413,"median":1.0458583983620136,"wins":0}`, prior packed `{"high":1.1331029833657795,"low":1.1071018396312415,"median":1.1229560143701962,"wins":0}`. Failed checks: `["nonregression w1/packed","nonregression w16/native","nonregression w16/fresh","nonregression w16/packed","nonregression w32/native","gain w32/native","nonregression w32/fresh","gain w32/fresh","nonregression w32/packed","gain w32/packed","gain all3/native","gain all3/fresh","gain all3/packed"]`. Calibration, individual results and all raw process records remain available. Eighteen prospective checks use ranks10/16/22 of31, no familywise claim:3factor1.03calibrations,9nonregressions(upper<=1.03), and32worker/all3gain against each control(upper<1,>=21wins). No gain floor, exclusions, extension, favorable subset, lane/layout/compiler/attribute rescue or reroll. The tested lane-wise finish is closed without worker/lane/layout/attribute/compiler rescue. This does not reject all packed extension strategies or discard the earlier calibrated Miller gains.

## Profiles and memory

PMU and cycles panels follow all latency: native/fully solo, all4arms at32workers,8192calls each after7primes. FIFO controls exclude setup/primes/oracle checks. Profiles use fixed4M cycles:u period,512pages,sampleCPU/timestamp/period. Stream sample/LOST/unknown counts have a single implementation; exact ELF/IP attribution and exclusive/removable wall-time or causal cache/spill diagnoses are not claimed. Raw data/decode streams are locally hash-indexed; summary reports are archived.

| Profile | Samples | Unknown | LOST | ≥1000samples |
|---|---:|---:|---:|---|
| profile-b0-full-w1-native | 4313 | 2 | 0 | true |
| profile-b0-full-w1-fully | 3650 | 81 | 0 | true |
| profile-b0-full-w32-native | 6424 | 36 | 0 | true |
| profile-b0-full-w32-fresh | 6418 | 37 | 0 | true |
| profile-b0-full-w32-packed | 3477 | 24 | 0 | true |
| profile-b0-full-w32-fully | 3858 | 75 | 0 | true |

Fq12 packed payload is1,920 bytes; compiler frames/temporaries, prepared inputs, scalar bridge and worker stacks are additional. Diagnostic output buffers and fixture preparation affect process RSS/faults outside the kernel. No minimum RAM, memory-pressure/mobile/portable/concurrent-proof/huge-circuit result is inferred.

## Full-proof baseline, unchanged

No new SDK request, Gnark proof, AWS/Aeglos/M5 benchmark or production adoption is added. Accepted production176/SDK6d83 remains unchanged, with latest matched local32-worker1×2=7.915095ms and merge54×1=161.718956ms. [All eight supplied M5 CPU and Metal comparisons](../2026-11-07-nonbinary-radix176/README.md#accepted-proof-baseline-unchanged) remain current. CPU10× requires1×2<=3.7ms; all eight requirements remain active and unmet0/8. The canonical proof ledger remains31,387.

[Design](artifacts/design.md), [timing protocol](artifacts/timing-design.md), [qualified finish](artifacts/packed/finish.rs), [qualification](artifacts/qualification.json), [analysis](artifacts/analysis.json), [observations, gzip JSON](artifacts/observations.json.gz), [receipts, gzip JSON](artifacts/receipts.json.gz), [bulk hashes](artifacts/bulk-index.json) and [checksums](SHA256SUMS). Synthetic point/result payloads, binaries/libraries, proving keys/witnesses and raw proof payloads are not published. Raw perf reports retain original whitespace.

## Independent review and interpretation

An independent reader reconstructs all 18 checks from raw process medians, all 396 complete-output digests from the reused canonical cache, 427 command receipts/stream hashes, and every worker CPU/TID set. All six PMU groups report 100% running. Header checks bind each profile's PID, 4M period and sampled TID/CPU to its observed workers or coordinator; this is not an independent ELF/IP or LOST-record parser. [Review](artifacts/independent-review.json), [reader](artifacts/review.rs).

The solo result retains a positive comparison with native policy: ratio 0.850286 [0.849671, 0.851772], 31/31 wins. It still loses all 31 comparisons with prior packed solo, as do the 16/32-worker cases. At 32 workers, fully/native is 1.168962 [1.135681, 1.191051], 2/31 wins; fully/fresh is 1.163744 [1.137994, 1.178178], 0/31 wins; fully/prior packed is 1.124066 [1.116790, 1.132486], 0/31 wins. These calibrated regressions independently close this implementation.

Absolute native/fresh/prior timings differ from the earlier study. Do not infer a regression or improvement of unchanged production from cross-study numbers. Earlier calibrated raw-Miller gains remain evidence in their original consumer; they did not establish full-proof gains. LLVM may coalesce identical old/new packed square or nonresidue helpers, so an old-module symbol in the new profile is not evidence of a fallback. The actual fully arm runs the new final-exponentiation body. No cause of the timing differences is isolated by this study.
