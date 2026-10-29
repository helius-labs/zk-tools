# Shared complex quotient preparation in the G2 caller

**Rejected at the actual-caller gate; no SDK trial or adoption.** The paired aggregate of the two G2 sizes at 32 workers is 2.23% slower than fresh OFF (ratio interval 1.019249–1.026561), with 0/31 wins. It is 2.68% slower than the byte-identical frozen baseline (1.023973–1.029234), with 0/31 wins. All control calibrations pass; gain and nonregression gates fail.

## Method

The new operator shares low-word inverse weights across both outputs of a normal or conjugate complex product. Three low-word products replace four per row when preparing Montgomery quotients. The original two fused CIOS product and reduction bodies, including exact lazy outputs and the `2p` negation correction, remain unchanged. It uses 315 IFMA sites per complex product versus 300 for the original and 322 for two independent row-quotient operations. This changes quotient preparation, not the number of full field products or reductions. Only the borrowed G2 slope numerator and final-y product use the operator. Norms, batch inversion, packing, tails, geometry and scratch remain original. No heap allocation, per-key table or copied point array is added. [Bounds](artifacts/bounds.md), [history audit](artifacts/history-audit.md).

## Fixed caller measurements

Ryzen 9950X, 16 physical / 32 logical CPUs; Rust 1.98.1 / LLVM 22.1.8. Solo pins CPU 2, physical16 pins CPUs 0–15, and logical32 pins 0–31. The fixed 31 rotated/reversed blocks cover nine contexts and three variants. Each worker performs 8,192 measured batches after three separately reset 1,024-batch primes. Operands change each batch. The primary statistic is common wall time per batch wave, including all workers' completion. Paired changes need not equal ratios of displayed medians.

|Caller|Size|Workers|Frozen µs|Fresh OFF µs|Candidate µs|Paired change/OFF|Wins/OFF|
|---|---:|---:|---:|---:|---:|---:|---:|
|G2 borrowed signed|200|1|8.470|8.479|8.412|-0.804%|26/31|
|G2 borrowed signed|200|16|9.782|9.687|9.745|+0.254%|8/31|
|G2 borrowed signed|200|32|15.827|15.848|16.213|+2.122%|1/31|
|G2 borrowed signed|352|1|14.272|14.242|14.227|-0.097%|18/31|
|G2 borrowed signed|352|16|16.746|16.418|16.764|+0.899%|3/31|
|G2 borrowed signed|352|32|27.175|27.269|27.949|+2.476%|0/31|
|G1 unchanged control|200|1|4.368|4.372|4.338|-0.787%|28/31|
|G1 unchanged control|200|16|4.886|4.879|4.860|-0.601%|25/31|
|G1 unchanged control|200|32|7.044|7.052|7.007|-0.472%|22/31|

G2 size 200 improves 0.804% at solo, but the unchanged G1 control also improves 0.787%. G2 size 352 solo has an uncertain 0.097% gain. Both G2 sizes regress with 16 and 32 workers. These observations do not establish a transferable solo arithmetic benefit.

The predeclared gate requires every fresh/frozen calibration interval within factor 1.03; every candidate nonregression upper ratio ≤1.03 against both controls; and both logical32 G2 sizes, individually and as their equal-log-weight aggregate, to have an upper ratio <1 and ≥21/31 wins against both controls. Rank endpoints 10–22 of 31 have nominal 97.055% individual coverage conditional on independent exchangeable blocks, without a familywise guarantee. No gain floor, exclusions, extensions, successful reruns, threshold/flag/layout sweep or subset promotion is used. The size-352 logical32 candidate/frozen nonregression upper ratio is 1.030763. [Design](artifacts/design.md), [analysis](artifacts/analysis.json).

## Correctness and memory

The new independent BigUint model passes 60,168 complex products, 601,680 quotient checks and 601,680 post-row integer checks across two primes, both Montgomery radices, normal/conjugate forms, lazy boundaries, random operands and changing chains. Two new native field tests pass 690,176 output-lane comparisons against both Arkworks and exact original lazy digits. The unchanged 33 field tests are inherited. Fresh OFF and ON each pass 128 unfiltered core tests, including independent borrowed G2 generator, tail and reuse checks. The frozen baseline binary, libraries, source and 128 tests are reused byte-for-byte from the prior caller experiment after hash, flag and reversible-source checks; those tests were not rerun.

All 868 capture processes pass independent output oracles; output digests and capacities match. Every timed interval allocates zero times. G2 scratch remains 16,000 bytes per worker at size 200 and 28,160 at 352; fixture-reference vectors remain 3,200/5,632 bytes. G1 control scratch remains 40,000 bytes with 3,200 reference bytes. These are caller capacities, not full-proof peak RAM or minimum RAM.

An initial controller assertion omitted the ignored/measured fields in Rust's test summary and failed before linking or measurement. Its corrected parser also checks all four native lane counters. Initial controller sources and the failure are retained; successful native tests were not repeated. No arithmetic change followed successful qualification. [Source audit](artifacts/source-audit.json), [qualification](artifacts/field-qualification.json).

## Generated code and profiles

|Build|Instruction sites|IFMA sites|RSP memory operand lines|Literal stack reservation bytes|
|---|---:|---:|---:|---:|
|frozen|8672|1450|1514|6464|
|off|8672|1450|1514|6464|
|on|8887|1480|1607|7488|

Counts use the actual linked borrowed G2 apply function and `objdump --no-show-raw-insn`, so byte-wrap lines are absent. They are static sites, not executed instruction counts, measured spill events or causal stall attribution. Additional quotient preparation coexists with larger code and stack use; these data do not isolate the regression's cause. [Code audit](artifacts/static-code.json).

Four FIFO-gated `cycles:u` profiles follow all latency measurements at 199 Hz, 262,144 batches per worker, G2 size 200, solo/logical32, OFF/ON. Enable follows priming/reset and disable precedes output oracles. All recordings precede ownership changes and offline decoding.

|Profile|Samples|Borrowed apply period|Lost|
|---|---:|---:|---:|
|profile-g2-200-solo-m1|442|91.72%|0|
|profile-g2-200-solo-m2|433|90.11%|0|
|profile-g2-200-logical32-m1|25866|92.27%|0|
|profile-g2-200-logical32-m2|26400|93.16%|0|

Each recording has one sampled process. Complete raw/demangled symbol weights and byte-exact decoded streams are archived compressed. Raw perf files and four expanded loss streams stay locally with hashes. Sampling confirms execution in the intended caller; it does not prove a specific latency cause. No owned build, helper, hash, archive or other benchmark overlaps capture.

All 880 capture-workflow steps, seven native steps and two link steps finish successfully. The archive retains 889 receipts and 3,552 original streams byte-for-byte, plus four local hash-only loss streams, 868 observations and all 177 private source files. Reversing private edits recovers all 175 production originals; 170 are already exact. Closing verification checks all 769 pins and the accepted SDK hash. [Closeout](artifacts/closeout.json).

## Inherited accepted CPU and Metal comparisons

Historical accepted local proof measurements versus supplied M5 references; **not remeasured in this caller experiment**. Accepted 1×2 peak RSS remains 134 MiB. The accepted full performance gate remains false, with its physical16 calibration engineering exception retained; its final artifact gate remains true.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|

Accepted 54→1 remains 162.250464 ms, without a supplied M5 reference. This experiment makes zero SDK proof requests or new Gnark verifications; the independent ledger remains 18,076. No new AWS, Aeglos, mobile, TEE, huge-circuit, pressure, minimum-RAM or concurrent-server-throughput conclusion follows. The all-eight CPU 10× goal remains active and unmet (0/8). This exact shared-quotient caller integration is closed.
