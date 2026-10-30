# Doubled operands in symmetric IFMA norms

The standalone norm improves by 11.84% at one worker, 11.85% at 16 workers and 6.47% at 32 workers against fresh OFF. The combined 32-worker prefix expression improves by an estimated 0.27%, with bounds spanning parity and 18/31 wins. Its fresh-control gain check fails. Both 32-worker aggregates pass, all six calibrations pass and every nonregression check passes. These positive component results are retained; the declared necessary prefix check prevents this version advancing to a complete-query or SDK trial.

Component gate: **false**. Profile checks: **true**. Production is unchanged. These are synthetic arithmetic waves, not point additions, complete queries or proof timings. Failed checks: `["Fq-prefix-32/new/off/gain"]`.

The candidate forms upper-triangular square products row by row in six CIOS coefficients. Doubling a 52-bit input limb discards one bit in IFMA; a test mask restores that bit's exact contribution to the next coefficient. This avoids explicitly doubling low/high product temporaries. Two squares share the original cancellation, R256/R260 packing and fused correction. There is no new heap buffer, table, key copy, plan or field representation. The older symmetric-norm implementation already tested product symmetry and remains rejected; this experiment tests a different lowering. It makes no prediction of a twofold proof improvement.

Both field/radix variants pass two new tests with **303,200 exact lazy-lane comparisons**, covering modulus/2p corners, bit and carry boundaries, full-width random values and changing chains. All 33 existing field tests also pass in the same unfiltered 35-test native suite. New tests compare exact raw outputs against accepted fused arithmetic; scalar-field closed-form sums independently check the component harness. No new core/prover, portable, mobile or pressure qualification is claimed. Removing one include recovers all17 private original field sources; all175 production files are exact.

Ryzen9950X,16physical/32logical CPUs. Workers1/16/32 pin CPU2/0–15/0–31. Norm includes packing two inputs; prefix includes two canonical coordinate differences, norm and one following product. Inputs are synthetic canonical raw Montgomery fields. Each worker cycles32 changing rows, accumulates outputs, performs three checked1024-update primes and32768 measured updates. All workers finish before output checks resume. Every measured wave has zero allocation. A frozen executable precedes the field include; fresh OFF and ON share one executable with dispatch outside the loop.

|Pattern / workers|Frozen ns/wave|OFF ns/wave|ON ns/wave|Paired ON/OFF [bounds]|Wins/31|
|---|---:|---:|---:|---|---:|
|Fq-norm-1|30.306|30.283|26.644|0.881568025 [0.875915152, 0.884579131]|31|
|Fq-norm-16|33.529|33.562|29.604|0.881495347 [0.878480292, 0.885604798]|31|
|Fq-norm-32|49.461|49.309|46.291|0.935261980 [0.929174134, 0.962089986]|25|
|Fq-prefix-1|57.687|57.882|53.822|0.931557385 [0.925138800, 0.934513597]|31|
|Fq-prefix-16|63.576|63.792|59.693|0.935001313 [0.930995078, 0.938480213]|30|
|Fq-prefix-32|84.130|83.856|83.572|0.997348006 [0.988177350, 1.004141658]|18|

Thirty-one fixed rotated/reversed blocks; ranks10–22/31 give nominal97.055% individual coverage conditional on exchangeability, without a familywise guarantee. All six fresh/frozen intervals must fit factor1.03; every ON/control upper ratio must be<=1.03. Norm32, prefix32 and their equal-log aggregate each require upper<1 and>=21/31wins versus both controls to select complete-caller work. No gain floor, exclusion, sample extension, changed threshold, unchanged rerun or subset promotion. Independent waves are not dependent inversion-prefix chains.

32-worker aggregate versus off: **0.966665688 [0.960021568, 0.977138668], 25/31wins**.

32-worker aggregate versus frozen: **0.968737815 [0.958169611, 0.976102643], 26/31wins**.

Postcapture disassembly confirms the intended multiply removal. It also shows more surrounding instructions. Static counts include packing and the complete kernel body, not just the field expression; RSP-relative instruction counts are not dynamic spill counts or stack/RSS bounds. These observations do not isolate the cause of the threaded prefix result. [Full counts](artifacts/codegen.json), [reader](artifacts/codegen.rs), [disassembly](artifacts/codegen.txt.gz), [arithmetic bounds](artifacts/postcapture-bounds.md).

|Fq kernel|IFMA OFF / ON|Instructions OFF / ON|Bytes OFF / ON|RSP-relative instructions OFF / ON|
|---|---:|---:|---:|---:|
|Norm|150 / 110|303 / 318|1909 / 1998|12 / 12|
|Prefix expression|250 / 210|564 / 594|3543 / 3746|30 / 40|

Four fixed16M cycles:u profiles follow latency, norm/prefix32 OFF/ON with4194304updates per worker afterprimes. FIFO gates exclude setup/primes and output checks. All recordings precede decoding/hashing/archive work; no owned build/helper/analyzer or second workload overlaps capture. Profiles check fixed period, one PID, worker TID/CPU, all workers sampled and zero lost records. Unknown-period share must be<=5% with>=1000samples. Absolute period coverage, exclusive stalls and removable wall fractions are unvalidated. Process RSS remains a component high-water mark, not prover RSS/minimum RAM.

|Profile|Samples|Unknown period share|Qualified|
|---|---:|---:|---|
|profile-norm-new|1597|0.000626|true|
|profile-norm-off|1820|0.000549|true|
|profile-prefix-new|3136|0.000957|true|
|profile-prefix-off|3276|0.000305|true|

All600 workflow commands finish: four native build/test/freeze steps,18preflights,558latency processes,four profiles and16postcapture commands. All580 component processes match scalar sums and cross-variant digests, with864822464 updates including primes. A separate reader-format test passes before capture. The offline reader was revised before capture to accept perf's combined PID/TID column; exact native-time helper sources/binary remain local and bound to original native pins. No successful tests/builds/captures repeat. [Reader revision](artifacts/pre-capture-reader-note.md).

The precompiled publisher independently recomputes20paired distributions and24gate decisions from integer wave durations, closes all600 command receipts and all source/helper pins, and round-trips source/helpers/receipts/observations byte-exactly. Float serialization comparisons use relative1e-12; performance thresholds are exact and unchanged. Full kernels/protocols/command streams and decoded profiles are archived. Executables/libraries/rawperf remain local with hashes; no raw key/witness inputs are collected.

Research reviewed [the SQIsign AVX-512 paper](https://arxiv.org/html/2608.13948v1), whose sections3.2–3.3 use a special prime and describe multiple extension-lane layouts. Those prime-specific savings are not transferred to BN254. Existing radix51 pairwise, extension-layout and symmetric-square negatives informed this distinct operand-level experiment; no outside performance result is claimed.

## Accepted CPU and Metal comparisons

Inherited local full proofs versus supplied historical M5 references, unchanged. All eight CPU10x targets remain unmet0/8; primary peak RSS remains134MiB. Historical physical16 calibration exception and separate accepted-artifact qualification remain documented.

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10x target ms|
|---|---:|---:|---:|---:|
|1x2|8.022165|37 / 4.612×|24 / 2.992×|3.7|
|2x2|11.335969|59 / 5.205×|30 / 2.646×|5.9|
|4x2|15.945544|109 / 6.836×|48 / 3.010×|10.9|
|8x2|24.090485|189 / 7.845×|74 / 3.072×|18.9|
|12x2|33.844855|284 / 8.391×|101 / 2.984×|28.4|
|16x2|39.931557|335 / 8.389×|124 / 3.105×|33.5|
|24x2|66.233309|528 / 7.972×|176 / 2.657×|52.8|
|merge-24x1|65.273224|507 / 7.767×|175 / 2.681×|50.7|

Accepted54→1 remains162.250464ms without a supplied M5 reference. Latest C7a primary remains Zopro11.915591ms/32workers versus Aeglos7.959184ms/96workers, best tested configurations in that campaign. No new AWS, fullproof, forester250/4999, phone/ARM/TEE, pressure/swap/minimumRAM or concurrent-throughput measurement is established. [Nine exact local/AWS rows](accepted-rows.json). Ledger25231 unchanged.

[Protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [profiles](artifacts/profiles.json), [kernel](artifacts/kernel.rs), [source](artifacts/field-source.json.gz), [helpers](artifacts/helpers.json.gz), [receipts](artifacts/receipts.json.gz), [observations](artifacts/observations.json.gz), [review](artifacts/review.json), [checksums](SHA256SUMS).
