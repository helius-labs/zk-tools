# Fused numerator scaling: complete-caller result

The candidate is **not adopted**. G2 is 4.25–5.79% slower than fresh OFF in the calibrated solo and 16-worker cases, losing all 15 blocks in each. The four-size 32-worker G2 aggregate is 2.17% slower, and the all-24-context aggregate is 1.99% slower; both have zero winning blocks. Some 32-worker comparisons fail control calibration, so those aggregates alone do not establish a calibrated speed change. Production 176 and the accepted SDK remain unchanged.

The frozen gate fails **38 of 76 timing checks**: four calibrations, 30 nonregression checks and all four aggregate gain checks. All 1,188 caller processes pass correctness and resource checks. The independent results reader passes. No extra blocks, exclusions, thresholds, widths, compiler variants or favorable subsets are used to rescue this candidate.

## Change and qualification

The [qualified field helpers](../2026-11-08-fused-scale-closure176/README.md) omit the intermediate fused-product correction immediately before scaling. Their coupled range bound keeps the final result below 2p. This stage adds two native eight-lane `FqBatch` forwarders and routes the actual G2 complex scaled numerator through them. The two components compute `(a*b+c*d)*f` and `(a*b-c*d)*f`; the intermediate wide value stays private. General fused products, G1 arithmetic, scalar/four-lane/portable paths, norms, prefix recurrence, inverses, destinations and scheduling retain their original implementation.

Both new OFF and ON cores link the **same exact qualified field library**, SHA256 `6c217276ff294415c7f1783123bffc816902bf206ebe0063559710c7f34b5966`. The qualified native/generic field libraries, field oracles and field-wrapper decodes are reused without rebuilding or retesting. The 178-file private snapshot recovers all 176 accepted originals after removing the declared changes. Driver, G1/G2 fixture, performance handshake and original tests match the earlier original-source caller fixture byte for byte. The immutable frozen caller `f07229ce…` is a diagnostic reference, not the accepted proof SDK.

Both fresh cores pass 128 unfiltered tests. Per arm, qualification checks 39,960 sequential and 162,816 threaded generator point outputs, with 53 actual workers across 1/4/16/32-worker pools requesting 512 KiB stacks and nested joins. A new test checks 2,048 full-width, output-fed complex numerator results across all 256 sign masks, including a negative-zero lane. Across both arms this is 405,552 generator outputs, 4,096 complex results and 106 workers. A generic/no-IFMA/no-ASM core compile with the private selector passes. These checks do not establish ARM or mobile execution.

Two fresh symbol listings and eight actual linked function decodes verify both caller-to-production-kernel transfers. G1 run and kernel metrics match across arms. The G2 production function changes as follows:

|Static G2 metric|OFF|ON|
|---|---:|---:|
|Instruction sites|8,590|8,571|
|Function bytes|51,830|51,488|
|IFMA instruction sites|1,450|1,450|
|RSP-operand sites|1,469|1,433|
|`vpcmpnleuq` sites|5|3|
|`jne` sites|23|21|

These counts describe compiled sites, not executed instructions, dynamic spills, stack bounds or speed. The retained `G2-original` diagnostic label names the production entry; ON's inlined implementation contains the new route.

## Frozen complete-caller comparison

Ryzen 9 9950X, shared host: CPU 2 for solo and CPUs 0–15/0–31 for 16/32 workers. G1/G2 × 200/201/352/353 points × 1/16/32 workers gives 24 contexts. Each process runs three checked 128-batch primes, resets, then measures 8,192 changing batches per worker. The interval includes reference-array assembly, arithmetic, dispatch and waiting; setup, reset and generator verification are outside it. This is a complete point-update caller, not a whole MSM or proof.

The prospective plan and reader are frozen before preflight: 72 preflight processes and their checker, 15 rotated/reversed blocks containing 1,080 latency processes, then 18 grouped PMU panels and 18 profiles, followed by 90 offline ownership/decode/metadata/event/loss commands. All 1,279 capture commands succeed, covering 1,188 checked caller processes and 199,292,928 worker batches including primes. Those are **not proofs**. Only the confirmed live capture handle is polled during recording; no owned build, helper, hash, analysis, decode, verification, report, archive or push overlaps it.

The 76 checks require all 24 OFF/frozen paired intervals within reciprocal 1.03–1.03; all 48 ON/control nonregression interval upper bounds ≤1.02; and G2's four-size 32-worker and all-24 equal-log aggregate upper bounds <1 with at least 10/15 winning blocks against both controls. G1 is an unchanged negative control without its own gain condition. Paired rank intervals use zero-based ranks 3/7/11 of 15. They have no familywise guarantee. There is no minimum positive gain floor.

|Aggregate|ON/OFF ratio [rank interval]|Wins/15|ON/frozen ratio [rank interval]|Wins/15|
|---|---:|---:|---:|---:|
|G2|1.021693764 [1.013644180, 1.025925250]|0|1.026235766 [1.008646004, 1.029602820]|0|
|ALL|1.019909812 [1.015696115, 1.029712302]|0|1.022593626 [1.017331498, 1.031316036]|0|

Times below are medians of process means in microseconds. Paired ratios use matched blocks and need not equal ratios of displayed medians. All observations, including positive estimates for unchanged G1, remain in the data.

|Context|Frozen / OFF / ON µs|ON/OFF paired ratio [rank interval]|Wins/15|
|---|---:|---:|---:|
|G1-200-w1|4.233460 / 4.235462 / 4.232083|0.998613320 [0.997093991, 1.000954442]|11|
|G1-200-w16|4.794452 / 4.797765 / 4.833348|1.006170140 [1.000736167, 1.009183125]|2|
|G1-200-w32|6.894710 / 6.906069 / 6.916370|1.004699758 [0.845316413, 1.202887701]|5|
|G1-201-w1|4.351101 / 4.350395 / 4.347711|0.999377754 [0.996564746, 1.000778590]|9|
|G1-201-w16|4.968201 / 4.942642 / 4.978762|1.001775969 [0.992054242, 1.010075679]|7|
|G1-201-w32|7.041882 / 7.100433 / 7.071104|0.998214539 [0.876428882, 1.048913687]|8|
|G1-352-w1|6.970656 / 6.971001 / 6.969991|0.999259060 [0.998353746, 1.000788746]|8|
|G1-352-w16|7.959104 / 7.968731 / 7.996258|1.002196624 [1.001119572, 1.004395485]|2|
|G1-352-w32|11.783795 / 11.639366 / 11.609015|1.001437241 [0.985891386, 1.243312774]|7|
|G1-353-w1|7.078493 / 7.075608 / 7.077333|1.000174187 [0.998077070, 1.000439099]|7|
|G1-353-w16|8.096114 / 8.096837 / 8.105949|1.001125310 [0.997076812, 1.005134756]|5|
|G1-353-w32|11.722619 / 11.844356 / 11.881328|1.008774339 [0.995033715, 1.238304489]|5|
|G2-200-w1|7.694356 / 7.746630 / 8.122746|1.048475037 [1.047727753, 1.049449060]|0|
|G2-200-w16|8.939672 / 8.998493 / 9.375788|1.042503662 [1.038352498, 1.044428326]|0|
|G2-200-w32|14.477972 / 14.450050 / 14.889621|1.031159603 [0.983491597, 1.040708002]|5|
|G2-201-w1|8.170578 / 8.099304 / 8.564602|1.057921047 [1.056821439, 1.059657276]|0|
|G2-201-w16|9.487202 / 9.389156 / 9.872908|1.051565550 [1.050467509, 1.052362376]|0|
|G2-201-w32|14.858094 / 14.983079 / 15.447195|1.026808991 [1.020732306, 1.032833746]|3|
|G2-352-w1|13.029114 / 13.080537 / 13.630115|1.042516434 [1.040663176, 1.042861926]|0|
|G2-352-w16|15.220527 / 15.297150 / 15.981781|1.044437546 [1.040101023, 1.045561672]|0|
|G2-352-w32|25.049905 / 24.874432 / 25.285916|1.016542414 [1.012977401, 1.027695465]|0|
|G2-353-w1|13.452136 / 13.414793 / 14.028210|1.046027853 [1.044708583, 1.046379256]|0|
|G2-353-w16|15.703312 / 15.672798 / 16.383638|1.045115191 [1.042617741, 1.047555872]|0|
|G2-353-w32|25.572090 / 25.348044 / 25.863693|1.020357997 [1.016590205, 1.025115123]|0|

The failed calibrations are G1-201-w32, G1-352-w32, G2-200-w32 and G2-201-w32. Several other 32-worker ON/control intervals are wide. A failed nonregression check is not automatically a demonstrated regression. Separately, all eight calibrated solo/16-worker G2 comparisons have ON/OFF intervals entirely above 1.038 and zero wins; closing the candidate does not depend on the noisy contexts.

## CPU profiles and memory

All 18 profiles qualify: 120,577 samples in total, at least 3,018 per capture, zero reported LOST, 100% assigned worker samples, minimum target share 99.8714% and maximum unknown share 0.1286%. All four grouped user PMU events run at 100% coverage. Panels and profiles are collected separately after latency, using acknowledged FIFO gates after primes and before output checks. Each context/arm has one panel and profile, so these counters do not estimate counter uncertainty or isolate a causal stall source.

|Diagnostic context|Retired instructions ON/OFF|Cycles ON/OFF|
|---|---:|---:|
|G2-200-w1|1.012387|1.051122|
|G2-200-w32|1.012395|1.058397|
|G2-352-w32|1.012850|1.037335|

Although static sites decrease, these G2 panels retire 1.24–1.28% more instructions and take 3.73–5.84% more cycles. Static code size did not predict executed work. Cache-miss changes disagree across contexts and do not establish cache or DRAM as the cause. Removing a correction also changes the instruction schedule and register allocation; those are possible mechanisms, not demonstrated attribution.

All timed allocation counts are zero and primed scratch capacities match across arms. The change adds no production heap buffer, lookup table, key/point copy, retained solver plan or pool. G2 prefix payload is 16,000 bytes at 200 points and 28,160 bytes at 352; one-point tails retain 1,280 additional bytes. Maximum observed candidate diagnostic-process RSS is **18.75 MiB**, including setup and worker stacks but excluding proving keys. This is not whole-prover peak, minimum RAM or memory-pressure evidence.

## Receipt review and preparation repairs

Both frozen post-capture commands pass on their first execution. The independent results reader reconstructs all commands, outputs, ratios, 76 timing checks, profile ownership/CPU/period/loss and counter qualification. The publication helper separately checks frozen pins, command/log hashes, original-source recovery and archive roundtrip without rerunning scientific work. Shared statistical/parser assumptions and finite correctness populations remain limitations.

The first native workflow built OFF core and tests successfully, then rejected a test launch containing duplicate `--nocapture`, before executing tests. Only the duplicate argument was removed; both successful builds were reused and the remaining seven commands passed. The initial failed plan, pins, logs and the continuation mapping remain archived. `effective-build-result.json` is explicitly an aggregate of actual receipts, not a fabricated xtask run.

A preparation reader subsequently looked at that failed launch's empty stdout instead of the already-successful continuation test log. Only its log path and reuse of identical derived output were corrected. The failed executed reader source is retained as `prepare-inspection-before-testpath.rs`; an older preexecution version is also retained. A separate first helper-compiler invocation omitted `--extern rustc_demangle`, failed E0433 before linking, and then passed with that argument added to the unchanged source. The first publication reader attempted UTF8 decoding of unchanged binary source-manifest members and stopped before writing a report or archive. Its correction hashes unchanged members as bytes and parses only the declared changed Rust files. That initial reader is retained as `publish-initial.rs`. These helper failures are tool-transcript observations, not separate workflow receipts. No successful native test, oracle, decode or capture was replayed for the repairs.

## Unchanged full-proof CPU and Metal comparison

This experiment creates zero SDK requests or Gnark exports. The canonical inventory remains 37,023 proofs. Accepted full-proof results below come from the [latest standard-SDK campaign](../2026-11-08-tree-policy-sdk176/README.md); the supplied M5 CPU and Metal references were measured on different hardware. This caller experiment adds no AWS, Aeglos, phone, giant-circuit, pressure or concurrent-proof result.

|Shape|Accepted local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× limit ms|
|---|---:|---:|---:|---:|
|1x2|8.040649|37 / 4.602×|24 / 2.985×|3.7|
|2x2|11.149097|59 / 5.292×|30 / 2.691×|5.9|
|4x2|16.069415|109 / 6.783×|48 / 2.987×|10.9|
|8x2|24.131178|189 / 7.832×|74 / 3.067×|18.9|
|12x2|33.940628|284 / 8.368×|101 / 2.976×|28.4|
|16x2|40.164083|335 / 8.341×|124 / 3.087×|33.5|
|24x2|66.187704|528 / 7.977×|176 / 2.659×|52.8|
|merge-24x1|65.361571|507 / 7.757×|175 / 2.677×|50.7|

Merge 54 remains 160.890372 ms without a supplied M5 reference. The same accepted executable previously measured **7.915095 ms** for 1×2, and private, unadopted candidates measured 7.859756/7.869400 ms. Those sub-8 ms observations remain valid. The latest accepted 8.040649 ms does not establish an adopted regression or its cause. Describe accepted local 1×2 as roughly 8 ms while retaining the individual observations.

This fused-scale candidate is closed without adoption or a full-SDK timing follow-up. The next candidate needs a distinct reduction in executed work or dependency depth, checked against prior experiments. The verified single-proof CPU AVX ≥10× M5 CPU objective remains active and unmet, **0/8 shapes**.

[All context results](artifacts/analysis.json), [independent reader](artifacts/review.json), [native qualification](artifacts/qualification.json), [linked code](artifacts/codegen.json), [source recovery](artifacts/source-audit.json), [protocol](artifacts/protocol.md), [source/raw observations/receipts](artifacts/evidence.json.gz), [checksums](SHA256SUMS). Public analysis omits only duplicate full caller/command receipt maps; its exact original and all receipts remain in the hash-bound archive. Executables, libraries, raw perf recordings, keys, private arrays, witnesses, proofs and full unchanged production sources are excluded; original identities and raw-perf hashes remain recorded.
