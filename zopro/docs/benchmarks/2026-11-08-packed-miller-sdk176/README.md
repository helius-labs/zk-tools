# Packed Miller loops in the actual proving SDK

**This integration does not establish a full-proof speedup and is not adopted.** In the fixed local 1×2 trial, the candidate measures 8.009593 ms, accepted production 7.995682 ms and fresh OFF 8.019773 ms. Both candidate paired intervals cross 1.0 and both win-count checks fail. Control calibration, correctness, memory screening and automatic profile quality pass. Production remains unchanged.

This is the first actual prepared-key/verifier/SDK integration of the [earlier three-lane Miller prototype](../2026-11-07-three-lane-miller176/README.md). That study improved raw Miller loops, but its full-loop controls at 16 and 32 workers failed calibration; a [later synthetic caller](../2026-11-07-three-lane-final-exp176/README.md) found the prior packed strategy slightly slower than native at 32 workers. The exception allowing this SDK trial was declared before preparation, builds or clocks. Both old failed gates remain unchanged.

The candidate packs three independent Miller accumulators into existing vector field operations, borrows the actual G2Prepared lines without copying or translating the key, multiplies the results and keeps the original single scalar final exponentiation. The fourth complex lane is inactive. The packed state payload is 1,920 bytes; compiler frames and temporaries are additional. That number is not an RSS or minimum-RAM claim. There is no new per-key table, retained execution plan, buffer pool, allocator policy, C/ASM prover rewrite or combination with the rejected lane-wise final-exponentiation/runtime/arithmetic experiments.

The old arithmetic body is recovered exactly by reversing four import/type/line/schedule adaptations; the original function boundaries remain. Of 176 production files, 174 private copies are exact and two parent insertions reverse to the originals. New qualification passes 134 tests: one cached-oracle test covering 16 actual prepared-key triples on a 512 KiB stack, 125 core tests, two prover tests and six cubic integration tests. The original 35 field tests and independent arithmetic qualification are inherited by exact source identity. A generic noASM/noIFMA core/prover compile passes; ARM and phone execution remain untested. Disabled parallel verification and unsupported configurations retain the original policy.

Both coherent eight-library SDK links use O3, native CPU, fat LTO and one codegen unit. Actual SDK prove code calls packed routines only in the candidate. G1/G2 MSM consumers retain 800/1,450 static IFMA sites. Seven packed-symbol cycle samples confirm execution during profiling. Independent Gnark preflight verifies 22 fresh exports across all nine shapes at 32 workers, plus primary at one and 16 workers. Preflight establishes correctness; it provides no matched performance result.

Ryzen 9 9950X, 16 physical cores/32 logical CPUs, shared host. Thirty-one fixed rotated/reversed blocks compare accepted SDK `6d83`, fresh OFF `fda2` and candidate `2f85`, with 32 workers on CPUs 0–31. Each process primes three requests then measures 151 fresh requests. Timed work includes solving, fresh proof randomness, proving, internal verification and serialization; loading is excluded. All 93 latency processes finish before three PMU panels and three user-cycle profiles. All 99 Gnark verifiers and 15 ownership/decode/quality commands run after recordings. No owned build, helper, hash, analysis, report or push overlaps recording. Every observation is retained, with no extension or reroll.

|Comparison|Median paired ratio|Fixed rank interval|Wins / 31|
|---|---:|---:|---:|
|Candidate / fresh OFF|0.998302996|[0.994048603, 1.004102712]|17|
|Candidate / accepted|1.001523605|[0.997654964, 1.006299824]|13|
|Fresh OFF / accepted|1.001964165|[1.000358079, 1.005243413]|9|

The prospective rule uses sorted zero-based ranks 10/15/22 of 31 process-mean ratios. This is a fixed rank interval. Both candidate upper bounds must be below 1.0, with at least 21 wins against each control; there is no minimum-gain floor. Fresh OFF/accepted must remain within factor 1.03. The four gain and win-count checks fail. This closes the integration without selecting an all-eight performance campaign or adoption.

|Arm|Median process mean ms|Max process RSS MiB|Median user CPU ms/request|Median system CPU ms/request|Median minor faults/request|
|---|---:|---:|---:|---:|---:|
|accepted|7.995682|136.75|174.881|4.965|127.523|
|control|8.019773|134.75|173.922|4.914|127.007|
|candidate|8.009593|133.75|173.766|4.999|130.384|

RSS includes loading, priming and process high-water behavior. The predeclared screen allows candidate RSS up to max(accepted, OFF) × 1.05 + 1 MiB; it passes. These are not minimum-RAM, memory-pressure or swap measurements. Minor faults are page faults serviced without disk I/O; these observations alone do not establish a latency cause or a service-concurrency result.

|Arm|User cycles/request|Instructions/request|Cache misses/request|Branch misses/request|Profile samples|
|---|---:|---:|---:|---:|---:|
|accepted|811132044|1387319013|2959220|898575|12278|
|control|816461351|1388330024|2810649|895575|12285|
|candidate|814337294|1383329316|2939930|886555|12295|

The single PMU panel shows candidate/OFF ratios of 0.996398 for instructions and 0.997398 for cycles; it does not establish a repeatable latency gain. All counters run 100%. All three profiles pass the frozen quality checks: 36,858 samples, zero LOST records and at most 1.904% unknown symbols. Sampling uses `cycles:u`, period 16,000,000, 512 mmap pages and acknowledged FIFO enable/disable around measured requests. PID/TID/CPU/MMAP metadata are validated. G1/G2 consumers, reductions and inversions continue to dominate flat cycle samples. Flat cycle attribution does not establish exclusive critical-path time or causality.

The independent reader reconstructs 213 command receipts, 99 raw request outputs and Gnark bindings, all 31 paired blocks, PMU/profile values and all seven checks (850 pin/stream hash checks). Including preflight, this trial adds 15,274 SDK requests and 121 independently verified fresh exports; the canonical ledger grows 34,929 → 35,050. The source-audit helper was corrected to hash binary fixtures as bytes before any decoding or clock. The publisher was corrected to exclude binary SDK outputs from the text archive. Initial helper sources are preserved; neither repair repeats tests, builds, decoding or measurements.

The latest accepted 1×2 result is again slightly below 8 ms. The same unchanged SDK previously measured 7.915095/7.928660 ms and then 8.018344 ms locally; the cause of variation between runs remains unconfirmed. The remaining shapes below are **inherited** from the [previous full SDK campaign](../2026-11-07-fused-epoch-sdk176/README.md), with no new matched packed-candidate timing for them. Supplied M5 timings are historical references on different hardware.

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.995682|37|24|4.627×|3.002×|3.7|
|2x2|11.241463|59|30|5.248×|2.669×|5.9|
|4x2|15.933816|109|48|6.841×|3.012×|10.9|
|8x2|24.234311|189|74|7.799×|3.054×|18.9|
|12x2|33.855565|284|101|8.389×|2.983×|28.4|
|16x2|40.071563|335|124|8.360×|3.094×|33.5|
|24x2|65.917462|528|176|8.010×|2.670×|52.8|
|merge-24x1|65.305023|507|175|7.764×|2.680×|50.7|

All eight verified CPU 10× requirements remain active and unmet, **0/8**. Accepted merge54 remains 162.240109 ms from the previous campaign, without an M5 reference. There are no new AWS, Aeglos, 250/4999, phone, ARM, TEE, pressure, swap or minimum-RAM measurements.

The [text evidence archive](artifacts/evidence.json.gz) includes prospective plans/pins, helpers, changed sources, native qualification, optimized disassembly, all command receipts and outputs, per-request timings, PMU CSVs and complete decoded profiles. Raw perf data, binaries/libraries, keys/images, request/proof payloads, full private source trees and private proof ledgers are excluded; relevant identities remain pinned. Decompression yields a JSON map of relative paths to `{utf8, bytes, sha256}`. Every extracted payload and the compressed artifact is hash checked.
