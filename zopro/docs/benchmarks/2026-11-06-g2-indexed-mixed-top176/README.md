# G2 mixed spans with an indexed top

The candidate is slower in all six B-query contexts and is not adopted. At 32 workers, same-binary paired ON/OFF estimates are **10.54% slower for 1×2** and **13.30% slower for 2×2**, each with 0/15 winning blocks. The declared complete-query screen fails. Production and accepted proof times remain unchanged.

These are complete G2 query times, including grouping and normalization, not proof latency or concurrent-proof throughput. Ryzen 9950X, 16 physical/32 logical CPUs, shared host; worker masks are CPU2 for solo, CPUs0–15 for16 and0–31 for32.

|Context|FROZEN / OFF / ON ms|ON/OFF median [bounds]|Wins / 15|
|---|---:|---:|---:|
|1×2 B, 1 worker|37.318563 / 37.309105 / 46.400141|1.243311 [1.241798, 1.245390]|0|
|1×2 B, 16 workers|3.627017 / 3.434284 / 4.126718|1.201739 [1.190252, 1.205940]|0|
|1×2 B, 32 workers|3.248253 / 3.019171 / 3.340607|1.105370 [1.086524, 1.119733]|0|
|2×2 B, 1 worker|39.923363 / 39.649186 / 49.067719|1.237975 [1.236674, 1.239258]|0|
|2×2 B, 16 workers|4.227749 / 3.934888 / 4.682486|1.195898 [1.178353, 1.204111]|0|
|2×2 B, 32 workers|3.817586 / 3.471473 / 3.940678|1.133048 [1.114215, 1.135235]|0|
|1×2 A, 32 workers, unchanged control|1.177189 / 1.179854 / 1.183871|1.000009 [0.982043, 1.034709]|7|

The equal-log two-B-shape32 aggregate ON/OFF is **1.117307 [1.104199, 1.123692]**, 0/15 wins. Against FROZEN it is **1.031263 [1.013384, 1.032944]**, also 0/15. Several OFF/FROZEN calibrations fail; the cross-executable estimates retain that limitation. The same-binary B regressions and calibrated solo regressions remain negative evidence independently of those failures.

## Layout and resource scope

The original G2 Phi2 path has ten ordinary 12-bit windows and a signed indexed top with six magnitude bits at offset120. The candidate retains that topology but uses eight ordinary12-bit windows, two ordinary11-bit windows at96/107, and an eight-magnitude-bit indexed top at118. The top reader uses width12 with known-zero upper bits; its magnitude is at most256 including carry. Variable-width Horner combination reconstructs the original signed components.

This preserves the qualified scalar split, two point partitions, high-component→phi→low order, borrowed pending/collision/exception handling, prefix-record field kernels, original scalar grouping and final reductions. It adds no key table, retained duplicate plan, new pool, scalar/digit array or per-window scalar copy. All original functions remain byte-exact; six private adapted clones have exact inverse-edit recovery. The snapshot has178files,171unchanged originals and five append-only parents; all176accepted originals recover.

The proposed final bucket slots fall20,544→18,688 per partition,9.03%. That is a capacity reduction, not an executed-addition or latency bound. Two ordinary windows use the current152-entry batches rather than200. The top's signed routing bins grow128→512. Both changes can increase inversion/reduction work.

A checked typed forecast compares all candidate ordinary parents, the enlarged signed-top routing/partial arrays, three retained top output sets, scalar storage and maximum leaf scratch against the existing original G2 forecast. The query falls back before in-place decomposition if the geometry is unsupported, arithmetic overflows or the comparison fails. Native testing checks the complete held/leaf comparison for every eligible length1024..65535 under the recorded configuration. No extra per-call OS headroom query or arbitrary RAM cap is introduced. This forecast comparison is not a hard allocator/stack/RSS bound or pressure qualification.

The earlier [balanced Phi2 trial](../2026-10-29-balanced-phi175/README.md) and [later balanced query/SDK trial](../2026-10-30-g2-balanced-sdk175/README.md) replaced the indexed top with ordinary windows. Their positive CPU/query/primary observations and failed full gates remain recorded. This candidate preserves indexed reduction, rather than repeating those implementations. The rejected uniform13/indexed9 wide-top trial remains separate. No result from accepted balanced K/H is transferred.

## Correctness and fixed measurement

Four preceding exact scalar tests pass, covering109,200 magnitude/sign observations, independent bit-loop digits, weighted integer/Horner reconstruction, boundaries and top carry. They are inherited without rerunning. Native qualification passes128unfiltered core tests, including three new cases for complete-curve generator oracles, signs, zeros, identities, repeated/opposite points, tails, grouping/fallbacks, input preservation,1/4/16/32-worker512KiBstacks and the full eligible workspace range. Field sources remain exact and inherit prior qualification. No fresh portable/disabled-prefix, full-SDK, concurrent-proof or pressure qualification is claimed.

FROZEN is the exact optimized API-only executable SHA-256 `0ba4cd4ebd2747db943c484de7b4784ee32c53c946e0a45ec30ea2f94d15420b`; it is not the full SDK. Fresh OFF/ON share executable `75b8abfbe0bc042dd17c8272ddb3b3ad6d462042e02b90a2c6298e49214c4e2c`. Actual release/native/fat-LTO/one-codegen-unit consumer flags are checked before capture. The original driver differs only by the M-query selector and its reset-metadata predicate. Every ON B call reports actual candidate entry; the A control uses the original query.

Seven retained FROZEN preflight receipts are reused with identical actual argv, environment, binary and inputs. Fourteen new OFF/ON preflights pass before the fixed latency matrix. Each process executes three primes,17timedqueries and one postcheck. All prime/timed/final results match the existing independently qualified Arkworks complete-MSM hashes; key points remain unchanged. Fixtures and independent oracles are reused without new solves or oracle execution. The 2×2 query includes its original witness-dependent scalar and point grouping.

Fifteen fixed blocks rotate/reverse case and variant order. Times are medians of process medians; paired ratios need not equal ratios of displayed medians. Sorted ranks4–12/15 have nominal96.484375%marginal median coverage conditional on exchangeable blocks, without familywise coverage. All OFF/FROZEN intervals must fit factor1.03; all ON/control upper bounds must be≤1.03. Primary32 and the two-B-shape32 aggregate each require upper<1 and≥10/15wins against both controls. No minimum gain floor, exclusions, extensions, changed thresholds, subset promotion or unchanged retry is used.

There are21failed checks: four multithreaded B calibration checks, twelve B nonregression checks, the unchanged A control's ON/OFF nonregression upper bound, two primary gain checks and two aggregate gain checks. The A paired median is essentially parity; its upper bound is uncertainty, not a measured3.47%slowdown. The multithreaded OFF/FROZEN gap resembles earlier retained control-context observations and is not a new identified compiler/scheduler cause. The full check list and all blocks are preserved in `analysis.json`.

## CPU, profiles and memory

All315latency processes finish before six warm PMU runs and six separate16M-user-cycle profiles. Each PMU measures200queries; each profile measures1024. All hardware groups report100%running time. The profiles contain135,242samples, zeroLOST, with checked PID/TID/CPU membership, executable mappings, task events and fixed periods. Flat samples are not exclusive critical-path costs, complete absolute-cycle coverage or DRAM measurements.

|32-worker context|OFF / ON CPU ms per query|OFF / ON million instructions|OFF / ON million user cycles|
|---|---:|---:|---:|
|1×2 B|70.264 / 80.817|567.741 / 668.111|319.012 / 372.706|
|2×2 B|75.816 / 87.116|595.803 / 698.687|346.583 / 402.347|

For primary OFF/ON, sampled `reduce_layer` counts rise2,896→4,753 and scalar Fq inverse counts1,587→3,205, while the main borrowed SIMD apply counts remain8,185→8,236. These are descriptive fixed-workload samples, not exact operation counts or isolated phase timings. They support increased reduction/inversion work as a concern rather than identifying scalar-digit overhead as the cause. The indexed reducer creates independent point tiles within each signed bin; increasing bin count can make those trees thinner and create more short inversion batches. This source-level explanation is plausible, not a measured causal decomposition between the larger top and narrower ordinary windows.

Maximum latency-process RSS counters at primary32 are34.75/33.25/33.25MiB for FROZEN/OFF/ON; at2×2-32 they are54.0/60.0/53.5MiB. Long profile-process counters differ substantially with allocation history and are retained in the artifacts. These values include fixture loading/primes and are not precise lifetime peak RAM, minimum RAM, whole-prover savings, pressure, swap or device qualification. Lower capacity or a favorable RSS observation does not compensate for the measured regression.

The prototype is closed without full-SDK follow-up. No annotation, window-placement, width or partition sweep follows this failed fixed layout. A future reduction of indexed-tree inversion overhead would be a different mechanism and needs its own history, memory and complete-caller evidence.

## Records and accepted comparisons

The377successful workflow commands comprise two earlier scalar-algebra commands, three native build/test commands and372capture/check/decode commands. There are348qualified caller rows, including seven reused preflights:14,112checked query invocations in total,14,077new. These are not proofs. The canonical independent-proof ledger remains30,846; no SDK request or Gnark export is produced. No successful test/build/recording is repeated.

The first offline reviewer incorrectly compared the complete declared environment with the managed environment projection stored by xtask. Only RUSTC was omitted by the receipt filter. Its original source and failure note are preserved; the repaired checker applies that projection, retains every other environment/argv/hash/timing check, and passes. It independently reconstructs487pin occurrences, all377command streams, all caller outputs/entry metadata, reused argv/environments, source recovery, timing/CPU/RSS/gates, raw PMU counts and sample ownership/counts, and checks all six raw profiles for lost records. The analysis additionally checks binary mappings, forks and event attributes. Compiler flags remain checked through actual pre-capture rustc argv.

The following are inherited accepted local32-worker full-proof observations and supplied M5 references, not new proof measurements. The goal remains active and unmet,0/8; cross-machine ratios do not establish equal hardware efficiency.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37|24|3.7|
|2×2|11.151932|59|30|5.9|
|4×2|16.086386|109|48|10.9|
|8×2|24.179453|189|74|18.9|
|12×2|33.884455|284|101|28.4|
|16×2|40.077381|335|124|33.5|
|24×2|65.850777|528|176|52.8|
|Merge24|65.060937|507|175|50.7|
|Merge54|161.883899|—|—|—|

Accepted production176/SDK `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661` remain selected. No fresh AWS/Aeglos, phone/ARM/TEE,250/4999, concurrent-throughput, minimum-RAM or swap result is added. [Accepted manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json).
