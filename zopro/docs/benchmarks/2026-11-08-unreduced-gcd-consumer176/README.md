# Unreduced GCD coefficients: complete-inverse comparison

**This implementation is slower in all six field/thread contexts and is closed without prover integration.** The mathematical coefficient reduction remains valid, but its roughly 53% saving in coefficient/finish word-product expressions does not produce a faster complete inverse. The equal-log six-context paired ratio is **1.055835 [1.054511, 1.058093]**, with **0/31 wins**. Production remains unchanged.

## Complete inverse results

One fat-LTO executable contains the accepted inverse (OFF) and the exact [qualified fixed-limb implementation](../2026-11-08-unreduced-gcd-native176/README.md) (ON), sharing one freshly built original field library. Mode selection is outside the loop. Times are medians of 31 process populations; ratios pair blocks and need not equal quotients of displayed medians. They are nanoseconds per inverse per worker in the fixed concurrent workload, **not proof times**.

|Field / workers|OFF ns|ON ns|Paired ON/OFF [bounds]|Wins /31|
|---|---:|---:|---|---:|
|Fq /1|529.883|568.576|1.069122 [1.067595,1.077520]|0|
|Fq /16|577.309|623.285|1.080554 [1.078641,1.081900]|0|
|Fq /32|787.946|812.670|1.031402 [1.027961,1.036799]|0|
|Fr /1|531.320|566.027|1.060499 [1.058458,1.068606]|0|
|Fr /16|580.000|620.235|1.067494 [1.065335,1.071757]|0|
|Fr /32|791.934|809.388|1.021772 [1.018589,1.026520]|0|

The prospective gate requires all six nonregression upper bounds≤1.02; Fq1, Fq32 and the six-context aggregate also require upper<1 and≥21/31 wins. All nine checks fail. There is no minimum gain floor, excluded block, sample extension, unchanged reroll or favorable subset. Ranks 10–22 of 31 give nominal 97.055% marginal median coverage under independent/exchangeable blocks, without familywise coverage. [Protocol](artifacts/protocol.json), [analysis](artifacts/analysis.json), [decision](artifacts/decision.json).

The Ryzen 9950X host has 16 physical and 32 logical CPUs. Worker affinity is CPU2 solo or CPUs0–workers−1; each diagnostic std thread has a 512 KiB stack. Each process checks every fixture result per worker, performs three checked 256-input primes, then times sixteen complete 4,096-input passes per worker. Ready/start/done/post barriers exclude startup, priming and postchecks; release/wait overhead is included. Timed outputs feed a checksum checked against the independent expectations; individual timed outputs are not retained. Every full pre/post output is checked. These temporary diagnostic threads are not a production-pool change.

## Counters and profiles

All latency finishes before diagnostic recordings. Valid counters/profiles use a **separate diagnostic-only executable** because the original acknowledgement reader failed, as described below. It shares the same field library and exact inverse/helper source. The actual optimized diagnostic artifact is inspected separately; matching arithmetic source and function sizes are not a proof of whole-executable equivalence. Its PMU evidence is not claimed to reproduce the exact latency executable's instruction count or exclusive slowdown cause.

|32-worker panel|Cycles /inverse|Instructions /inverse|Branch misses /inverse|
|---|---:|---:|---:|
|Fq OFF|3,866.323|8,178.541|12.949|
|Fq ON|3,870.800|8,095.484|14.366|
|Fr OFF|3,868.184|8,148.027|12.534|
|Fr ON|3,868.021|8,088.358|14.080|

All four grouped panels have 100% scheduled time. Instructions decrease about 1.02% for Fq and 0.73% for Fr, while cycles are approximately flat and branch misses increase. These are single diagnostic panels, not replicated uncertainty estimates or a causal decomposition. The source-level product count omits carries, signs, active-width control, copies, helper boundaries and the unchanged GCD work.

Four fixed-period cycles profiles qualify: 32,721 samples total, zero lost, correct PID/TID/CPU ownership and period, all samples in declared workers. Raw unknown-symbol percentages are 0.134%/4.770% for Fq OFF/ON and 0.024%/4.852% for Fr OFF/ON, below the predeclared 5% ceiling. ON attributes about 24.1%/25.5% of samples to `Wide::linear` and 4.9%/4.4% to late correction. Flat symbol samples include compiler decisions and are not exclusive wall-time shares.

A separate read of saved MMAP/sample outputs maps 384/387 Fq-ON and 394/395 Fr-ON unknown-symbol samples into libc. It does not identify exact IFUNC routines or a call stack. Static correction bodies contain memcpy/memmove calls, but this does not prove those calls account for every mapped sample or the measured slowdown. The original unknown-symbol metrics remain unchanged. [Saved-address attribution](artifacts/unknown-dso.json).

Static inspection finds an out-of-line candidate inverse, correction and shared wide-row helper. The original Fq/Fr entry bodies have 1,733/1,742 static instructions; candidate entries call 697-instruction inverse bodies plus 635/643-instruction correction bodies and a shared 426-instruction row helper. These separately emitted sizes cannot be added into a dynamic operation count. Bounds/panic sites and stack operands are also retained. [Latency-artifact codegen](artifacts/codegen.json), [diagnostic-artifact codegen](artifacts/profile-codegen.json).

## Qualification, harness failure and review

The field snapshot retains all 176 accepted source files byte-exact. The new inverse and its two original integer/matrix fragments remain byte-exact from the prior qualification. A std-only O3/native/fatLTO/CGU1 consumer resolves the previous support-library bitcode problem. Four new actual-consumer checks pass on first execution: 16,384 original/candidate output comparisons against 8,192 independently generated BigUint Fermat expectations, plus raw-zero checks. The original mathematical/non-LTO artifacts and old suites are inherited, not replayed. No field/core/SDK dispatch changes are made.

All 372 original latency processes finish successfully. The first counter process then rejects a NUL left by perf acknowledgement framing, panics before releasing its postcheck barrier and waits. Its child is terminated after inspection. Perf returns zero, but the missing final result and panic explicitly invalidate that diagnostic population. The workflow starts a second diagnostic before the workflow and that process are terminated; session13232 ends143 and a process check confirms no owned xtask/perf/consumer remains. Both attempts and all partial records are preserved; neither supplies reported counters.

A new diagnostic-only consumer ignores NUL framing bytes. That is its sole authored source change. It uses new empty FIFOs and output paths, passes 16,384 output comparisons in the new executable, and receives a separate nm/full-disassembly inspection. Eight fresh diagnostic populations and all offline decoding/loss/ownership checks complete successfully. **No latency process, successful old build, model, oracle population or decoder for an unchanged executable is rerun.** New-artifact qualification reuses the already generated expectations. [Recovery record](artifacts/diagnostic-recovery.md), [failure summary](artifacts/diagnostic-failure.json), [latency driver](artifacts/consumer.rs), [diagnostic driver](artifacts/profile.rs).

The first offline reader fails an exact aggregate floating-point equality by one binary64 rounding step. Its source and a failure summary are retained; the tool stderr was not separately archived. A separate corrected reader permits at most `32*EPSILON*max(1,|a|,|b|)` for aggregate reconstruction only. Raw observations, estimates and selection rules remain unchanged. It passes: six pin populations (366/374/379/383/387/393), 420 completed successful receipts across qualification/codegen/372 latency/repaired diagnostics, 380 complete process rows and all six paired estimates/aggregate/decision reconstructed. The first unqualified zero-exit diagnostic receipt is preserved separately and is not counted as qualified evidence. The reader does not replay arithmetic/recordings or independently reimplement every PMU parser. [Actual analyzer](artifacts/finish.rs), [actual reader](artifacts/review2.rs), [review](artifacts/review.json).

This exact widened-coefficient implementation is closed. The result supports returning to dominant MSM work removal. A materially different representation would need a new proof and complete native cost case; this experiment supplies no reason for an unchanged retry or an annotation/layout/width sweep.

## Accepted CPU and Metal comparison, inherited

No proof time is measured here. Current local 1×2 remains **roughly 7.9–8.0 ms** with the same accepted binary: older 7.915/7.929/7.996 ms, latest 8.003 ms. Variation remains unexplained, with no adopted regression established.

|Shape|Accepted local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.002770|37 / 4.623×|24 / 2.999×|3.7|
|2×2|11.241463|59 / 5.248×|30 / 2.669×|5.9|
|4×2|15.933816|109 / 6.841×|48 / 3.012×|10.9|
|8×2|24.234311|189 / 7.799×|74 / 3.054×|18.9|
|12×2|33.855565|284 / 8.389×|101 / 2.983×|28.4|
|16×2|40.071563|335 / 8.360×|124 / 3.094×|33.5|
|24×2|65.917462|528 / 8.010×|176 / 2.670×|52.8|
|merge-24×1|65.305023|507 / 7.764×|175 / 2.680×|50.7|

Source: [preceding accepted comparison](../2026-11-08-unreduced-gcd-native176/README.md). Seven nonprimary rows remain from the earlier full campaign; supplied M5 references use different hardware. Merge54 remains 162.240109 ms without a supplied M5 reference. The full all-eight verified CPU 10× goal remains active and unmet, **0/8**. Production176/SDK6d83 and the 35,181-proof inventory are unchanged. No fresh proof, SDK request, RSS/minimum RAM, AWS/Aeglos, huge-circuit, phone/ARM/TEE, RAM-pressure/swap or concurrent-proof throughput claim is added.

[Evidence archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Every UTF-8 member hash and gzip roundtrip is checked. The archive includes raw latency/diagnostic outputs, PMU CSVs, actual changed drivers, analyzer/reader variants, failure summaries, plans/pins and decodes. Executables/libraries/raw-perf/binary test fixtures/unchanged full source/private key-image-request-witness-proof-public arrays/complete proof inventories are excluded. Private root: `/home/ubuntu/src/zopro-perf-work/unreduced-gcd-consumer176`.
