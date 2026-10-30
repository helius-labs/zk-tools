# Direct symmetric square in the trace-one Fq2 basis

30 October 2026. Arithmetic screen, with conversion outside timing. Production is unchanged. This does not measure a valid-curve caller or proof latency.

Declared composite gate **false**; failures `["norm-affine/SQUARE/OFF/gain","norm-affine/SQUARE/FROZEN/gain"]`. Norm-affine SQUARE/OFF **1.045108395 [1.043746316, 1.046602792], 0/31 wins**; SQUARE/FROZEN **1.044269787 [1.042524206, 1.045525417], 0/31 wins**. Lower is faster.

The candidate is rejected before valid-curve or SDK integration. The complete norm-affine expression is **4.511% slower versus OFF and 4.427% slower versus FROZEN**, losing all 31 blocks against both. All six ordinary-control calibration intervals pass, so no calibration failure limits this negative decision. The new formula improves the old basis expression by **15.691%**, with 31/31 wins; that useful improvement remains insufficient to beat ordinary arithmetic.

The new square is **16.294% slower than ordinary square**, versus the old basis square's roughly twofold cost. Multiplication-only cells retain the old basis's gains, and the unchanged norm retains its additional cost. Every context and all four variants remain in the data; no favorable subset is selected.

## Change and correctness

For j²=j+1, (a+bj)²=(a²+b²)+(2ab+b²)j. The new helper forms both raw products together, sharing b² and symmetric cross-limb terms, then performs two Montgomery reductions. This removes the previous division-by-three square; the original [trace-one basis screen](../2026-10-29-golden-basis175/README.md) remains rejected. Its product, norm, conjugation, encoding and decoding source is reused exactly.

The standalone new square has 210 IFMA sites versus ordinary square's 200; it is not a claimed IFMA reduction against ordinary square. It removes division and summed-operand preparation from the previous basis square. X≤8p² and Y≤12p² for inclusive inputs up to2p; one unchanged 2p correction restores outputs strictly below2p. The exact integer model checks16,528 REDC outputs and82,640 row identities across R256/R260, with the maximum observed coefficient96,929,567,098,157,907 below2^58. Observed maxima complement the conservative bound; they do not prove a universal bound by themselves.

Two new native tests pass132,096 Arkworks field-output comparisons, including all36 input-edge pairs, full-width lazy values, both radices, normalized outputs and eight-step dependent chains. The33 unchanged tests are filtered and their qualification inherited. The initial model build caught an ambiguous BigUint shift type before execution; an explicit type repairs it. Initial source/stderr are retained. The model and native tests pass on their first executions; no successful run repeats. A later probe-only rebuild adds static wrappers and diagnostic repetition without changing field arithmetic.

All17 production field files recover exactly by removing one include suffix; inherited and new private helpers are separate. There is no key table, heap buffer, point array, plan, pool or persistent cache. SIMD temporaries can affect register/stack use; no whole-process RSS or minimum-memory claim is made. No core, SDK, portable or new proof qualification follows from these field tests.

The R260 branch needs no final subtraction because R260>64p gives REDC(Y)<1.1875p; the R256 branch uses the existing correction. The pinned original bounds note states the shared below4p bound without spelling out this stronger R260 condition. [Bounds clarification and offline-analysis repair](artifacts/offline-review.md), [original bounds](artifacts/bounds.md).

## Fixed arithmetic comparison

Ryzen9 9950X, CPU2, shared host. FROZEN is the exact original ordinary executable; OFF is fresh ordinary arithmetic; OLD_BASIS is the exact previously rejected basis executable; SQUARE is the new direct square with the unchanged basis operations. OLD_BASIS is diagnostic, not a replacement for either ordinary control. Every mode starts from identical scalar Fq2 inputs and checks32 decoded outputs independently outside timing. Conversion and scalar oracles are excluded. The norm-affine recurrence is a synthetic expression with a supplied factor and does not stay on the curve or include actual inversion, buckets or proof scheduling.

Thirty-one fixed blocks rotate/reverse cases and modes. Each process performs three2,000-iteration primes and100,000 measured iterations. Four streams divide elapsed time by four; other rows count one stream. Ratios pair blocks and need not equal ratios of displayed medians. All six OFF/FROZEN calibration intervals must fit factor1.03. Only complete norm-affine gain versus both ordinary controls selects further caller work: upper ratio<1 and≥21/31 wins. Isolated rows, including the known norm cost, are diagnostic and cannot independently select adoption. This prospectively declared composite criterion differs from, and does not rewrite, the earlier all-operation gate. Ranks10–22 give97.055% nominal individual median coverage under independent/exchangeable blocks, without a familywise guarantee. No gain floor, exclusion, extension, alternative square after timing or unchanged reroll.

|Context|FROZEN ns|OFF ns|OLD_BASIS ns|SQUARE ns|SQUARE/OFF|Bounds|Wins /31|
|---|---:|---:|---:|---:|---:|---|---:|
|chain|6.434884|6.433255|6.033126|6.039601|0.937269018|[0.935461196, 0.939669295]|31|
|streams4|6.292058|6.294447|5.688226|5.695818|0.905362950|[0.903130192, 0.906732704]|29|
|scaled|9.940920|9.988209|9.563133|9.582294|0.959358331|[0.957967251, 0.961033637]|31|
|square|5.010411|5.008508|10.189089|5.816480|1.162943396|[1.159637521, 1.165145450]|0|
|norm|6.755351|6.744555|7.652543|7.648210|1.134296697|[1.132960677, 1.135161334]|0|
|norm-affine|28.908875|28.876889|35.808371|30.203704|1.045108395|[1.043746316, 1.046602792]|0|

All control comparisons and calibration bounds remain in the full analysis.

## Automatic counters and profiles

All latency precedes four FIFO-gated grouped counter panels and four fixed16-million user-cycle profiles for square/norm-affine, OFF/SQUARE. Profile mode repeats the exact100,000-iteration arithmetic2048times, resetting inputs each time, consuming every output through an optimizer barrier and checking the identical final output once after disabling counters. Setup, primes, conversion and the scalar oracle are outside capture. This is repeated-expression profiling, not a long dependent chain, and its times do not enter the latency gate. All helpers are compiled and pinned before captures; no owned compiler/helper/hash/analyzer/archive or independent workload overlaps any capture.

|Counter panel|Instructions/Fq2|Cycles/Fq2|Qualified|
|---|---:|---:|---|
|stat-0-norm-affine-OFF|360.426|158.437|true|
|stat-0-norm-affine-SQUARE|448.397|164.660|true|
|stat-0-square-OFF|70.127|27.642|true|
|stat-0-square-SQUARE|99.590|32.214|true|

|Profile|Samples|Unknown period share|Attribution qualified|Lost|
|---|---:|---:|---|---:|
|cycles-0-norm-affine-OFF|16110|0.000062|true|0|
|cycles-0-norm-affine-SQUARE|16862|0.000652|true|0|
|cycles-0-square-OFF|2835|0.000705|true|0|
|cycles-0-square-SQUARE|3242|0.000000|true|0|

Counter qualification requires≥99.9% running coverage. Profiles require one PID/TID on CPU2, fixed period, executable/task mappings, zero lost records,≥1,000 samples and≤5% unknown period. The known absolute-period coverage deficit remains unresolved. Shares are qualitative, not exclusive costs, local CPI or removable proof wall time; single counter panels do not establish a causal mechanism.

The original offline analyzer stopped at an inherited FORK-record assertion. This single-threaded arithmetic process has an executable MMAP and matching COMM exec record, with no in-capture child fork. A separate analyzer requires that COMM exec PID to match the sole sampled PID/TID; all CPU2, mapping, fixed-period, loss, sample, hash, output and timing checks remain. The original pinned analyzer is unchanged and its failure is retained. No capture, native test or integer model is repeated. All four counter and four profile-attribution panels pass.

## Scope and accepted comparisons

All796 steps complete:24 preflights,744 latency processes,8 diagnostic processes and20 postcapture ownership/audit steps. Total898,656,408 vector-loop iterations include primes and profile repeats; four-stream contexts perform four states per iteration. Zero SDK requests or new Gnark exports; ledger23,971. A passing screen still requires actual valid-curve callers charging representation/conversion and memory costs, followed by full accepted/OFF/ON proof/Gnark/portable/resource/final-artifact qualification. No basis serialization, key transformation, curve integration or adoption is implemented. No new AWS/Aeglos/mobile/TEE/large-key/minRAM/pressure/concurrency claim.

The following accepted local proof results and supplied historical M5 references are inherited. All eight CPU10x targets remain unmet.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10x target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37.0|4.612×|24.0|2.992×|3.7|
|2x2|11.335969|59.0|5.205×|30.0|2.646×|5.9|
|4x2|15.945544|109.0|6.836×|48.0|3.010×|10.9|
|8x2|24.090485|189.0|7.845×|74.0|3.072×|18.9|
|12x2|33.844855|284.0|8.391×|101.0|2.984×|28.4|
|16x2|39.931557|335.0|8.389×|124.0|3.105×|33.5|
|24x2|66.233309|528.0|7.972×|176.0|2.657×|52.8|
|merge-24x1|65.273224|507.0|7.767×|175.0|2.681×|50.7|

Accepted primary is8.022165ms/134MiB;54→1 is162.250464ms/1831.87MiB without a supplied M5 reference. Latest AWS primary remains11.915591ms at32 workers versus pinned Aeglos Sorted7.959184ms at96; no new AWS measurement. The accepted physical16 calibration exception remains documented and does not transfer to this experiment. [Current summary](../2026-10-29-current-cpu-summary/README.md).

[Full analysis](artifacts/analysis.json), [design](artifacts/design.md), [new square](artifacts/square.rs), [native tests](artifacts/test-square.rs), [source recovery](artifacts/source-audit.json), [source and build records](artifacts/source.json.gz), [capture receipts](artifacts/receipts.json.gz), [observations and plan](artifacts/observations-and-plan.json.gz), [archive checks](artifacts/archive-review.json), [local raw hashes](artifacts/raw-local-pins.json), [publication review](artifacts/publication-review.json), [inherited summary bytes](artifacts/inherited-summary.json.gz).
