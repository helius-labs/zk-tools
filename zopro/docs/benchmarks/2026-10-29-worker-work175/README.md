# Retired work across worker counts

29 October 2026. This PMU diagnostic measures the exact accepted source-175 SDK at 1, 16 and 32 workers, without rebuilding or changing the prover. It compares retired instructions with cycles and CPU time; it does not qualify an optimization or replace accepted proof-latency results. Production remains unchanged.

At 32 workers, the prover retires **4.97% more instructions** than at one worker, while summed CPU time increases **89.89%** and cycles increase **59.63%**. IPC falls from **2.579 to 1.696**. The increase in CPU time therefore does not represent a comparable increase in retired instruction work. This does not rule out synchronization or memory stalls: a small instruction sequence can still consume substantial time.

## Capture conditions

Ryzen 9 9950X, 16 physical cores and 32 logical CPUs. One worker pins CPU 2, 16 workers pin physical CPUs 0–15, and 32 workers pin logical CPUs 0–31. The primary 1×2 image, request and accepted environment are fixed. Three fixed rotated/reversed blocks capture all three topologies. Each process performs three primes and 31 fresh randomized SDK requests, including solving, proving, SDK verification and serialization. FIFO control gates one group of cycles:u, instructions:u, cache-misses:u and branch-misses:u to the request loop. Loading, primes, output and independent verification are outside the counters. No uprobes are attached to the accepted executable.

The preceding stage diagnostic already independently verified the same three topology preflights. Their exact binary/image/request/environment/affinity and proof receipts are checked and reused. All nine new counter captures finish before ownership transfer, all nine remaining independent verifications and analysis. All helpers are compiled before capture; no owned compiler, helper/hash/archive/analysis or second CPU workload overlaps it. The host is shared; exclusive hardware access is not claimed.

## Counter qualification

A required event needs a finite nonnegative count, positive runtime and at least 99.9% running coverage in every process to qualify its panel. Unsupported or multiplexed data is retained as unqualified without resampling. There is no gain/adoption gate, sample extension, subset selection, timing qualification or unchanged rerun. Three observations are descriptive, without a confidence or familywise claim. All raw counts, coverage percentages and per-process values remain in analysis.json and the nine CSV files.

All nine panels qualified: **true**.

|Workers|Instructions / request|Cycles / request|IPC|CPU ms / request|Effective user-cycle GHz|Diagnostic request ms|
|---|---:|---:|---:|---:|---:|---:|
|1|1353869659|525000173|2.579|95.585|5.505|95.465122|
|16|1380964882|588026749|2.348|116.821|5.187|9.668104|
|32|1421211564|838068920|1.696|181.509|4.803|7.986322|

Values are medians of three per-process values after normalization by 31 requests. CPU time sums user and system time. User-only cycles divided by user CPU time give an approximate effective cycle rate, with clock/accounting limitations. Separately computed medians need not satisfy every displayed ratio exactly.

|Workers|Generic cache misses / request|Branch misses / request|Instruction count range / request|
|---|---:|---:|---|
|1|1380522|986339|[1353729818, 1353914829]|
|16|1948291|1081187|[1379364942, 1381358923]|
|32|2790233|1198451|[1420969261, 1421779239]|

## Interpretation and limits

|Comparison|Instructions|Cycles|Summed CPU time|Diagnostic request latency|
|---|---:|---:|---:|---:|
|32 versus 1 worker|+4.974%|+59.632%|+89.892%|—|
|32 versus 16 workers|+2.914%|+42.522%|+55.374%|−17.395%|

These are ratios of descriptive topology medians, not paired confidence estimates. All three count observations remain in the data. Locally, 32 workers share 16 physical cores through SMT; this experiment does not measure scaling from 16 to 32 physical cores. The AWS c7a comparison uses physical cores without SMT, so the local effect cannot be transferred directly to explain that gap. [Derived ratios](artifacts/interpretation.json).

Retired instructions count architectural instructions, not field operations, active SIMD lanes or micro-ops. Worker-dependent partition/window decisions are part of the measured behavior, along with runtime joins, stealing and idle work. An increase is not automatically scheduler overhead; changing worker count can change the actual arithmetic and memory work. IPC, CPU clocks, SMT sharing, OS placement and cache behavior can also change. Generic cache misses are not a byte-bandwidth measurement. These counters do not isolate one hardware cause or supply a removable wall-time fraction.

The effective user-cycle rate is calculated from user cycles and user CPU time; it is not an independent frequency measurement. The small instruction increase gives no evidence for wholesale duplication of the serial instruction stream. It also provides no upper bound on the latency benefit of reducing waits or contention. The next analysis should inspect instruction and cycle attribution or address a concrete reduction in arithmetic/data work, while keeping these distinctions intact.

The [preceding entry/return diagnostic](../2026-10-29-stage-uprobes175/README.md) keeps its failed parallel probe-overhead checks. This PMU-only run does not rescue them. It answers a different question using the same frozen binary, with no entry/return probes. No new SDK implementation follows automatically from these observations.

## Verification and accepted comparisons

All 27 capture/offline steps complete. The trial makes 306 internally verified SDK requests and exports nine fresh 164-byte proofs. Every export independently verifies with Gnark, matches the image/public-input hashes and is distinct from 22,189 prior identities; the new ledger is **22,198**. Three existing independently verified preflights are reused, not counted as new proofs. Five parser tests cover native CSV format, unsupported events, malformed rows, coverage thresholds and multiplexing rejection. Accepted field/core/prover qualification is inherited because source and executable are unchanged. Source, receipt and proof archives round-trip byte-exactly.

Accepted local 1×2 remains **8.022165 ms / 134 MiB observed peak RSS**; these counter-process latencies do not replace it. Latest AWS c7a.24xlarge remains Zopro 11.915591 ms at 32 workers versus Aeglos Sorted 7.959184 ms at 96. Supplied historical M5 CPU/Metal comparisons are unchanged:

|Shape|Accepted local ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|

Accepted 54→1 is 162.250464 ms without a supplied M5 reference. The original all-eight CPU 10× goal remains active and unmet, 0/8. The accepted physical-16 calibration engineering exception and separate final-artifact qualification remain documented. No new AWS, mobile/ARM/TEE, memory-pressure/swap, minimum-RAM, giant-circuit or concurrent-throughput result is claimed. [All nine local/AWS rows](../2026-10-29-current-cpu-summary/README.md).

[Protocol](artifacts/design.md), [all counts and observations](artifacts/analysis.json), [inherited preflights](artifacts/inherited-preflights.json), [verification inventory](artifacts/proof-inventory.json), [source](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [proofs](artifacts/proofs.json.gz), [archive review](artifacts/publication-review.json).
