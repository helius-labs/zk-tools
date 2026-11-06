# Ryzen 9950X instruction calibration

The measured instruction costs differ materially from the inherited LLVM scheduling model. Independent register IFMA runs at about 0.50 cycles per instruction, while dependent vector add/subtract and carry shifts take about 2 cycles. **This is hardware calibration, not a new prover optimization or full-proof timing.**

All 240 native commands completed once: 15 blocks × 16 cases × 7 internal repetitions, using frozen LLVM-generated loops pinned to CPU 2. All 1,680 repetitions passed instruction-count and event-ID checks, and every counter group reports 100% running time. The original workflow ended at an offline floating-point equality failure after capture; the separately repaired analyzer and independent review pass without recapturing samples.

## Hardware measurements

Values include loop, register initialization and function-call user-space overhead divided by 1,048,576 target instructions. Each loop contains 256 target instructions repeated 4,096 times. Throughput cases use 16 independent outputs or accumulators. The host is not exclusive; CPU 18 is CPU 2’s SMT sibling. Bootstrap intervals describe block repeatability on this host.

| Case | Median cycles/target instruction | 95% block-bootstrap interval |
|---|---:|---:|
| ifma-lo-acc-lat | 4.001386 | 4.001280–4.001484 |
| ifma-lo-acc-source-lat | 4.001477 | 4.001245–4.001525 |
| ifma-lo-reg-tp | 0.500720 | 0.500497–0.500774 |
| ifma-lo-mem-tp | 0.594820 | 0.588703–0.597878 |
| ifma-hi-acc-lat | 4.001383 | 4.001281–4.001456 |
| ifma-hi-acc-source-lat | 4.001333 | 4.001254–4.001349 |
| ifma-hi-reg-tp | 0.500678 | 0.500467–0.500775 |
| ifma-hi-mem-tp | 0.587075 | 0.582661–0.595739 |
| add-lat | 2.000707 | 2.000653–2.000739 |
| add-tp | 0.250525 | 0.250522–0.250754 |
| sub-lat | 2.000666 | 2.000512–2.000765 |
| sub-tp | 0.250525 | 0.250521–0.250531 |
| shr52-lat | 2.000598 | 2.000561–2.000720 |
| shr52-tp | 0.500650 | 0.500473–0.500778 |
| sar52-lat | 2.000588 | 2.000484–2.000630 |
| sar52-tp | 0.500587 | 0.500458–0.500728 |

The accumulator/source-alias cases measure a combined recurrence, not an isolated multiply-input edge. Memory cases repeatedly read 1 KiB of prefaulted scratch filled with `0x5a`; they do not measure cold loads or DRAM. Both carry-shift immediates are 52. All raw repetitions, integer counters, commands, decodes, hashes and repaired-analysis provenance are archived below.

## Implication for the next candidate

The [earlier model audit](../2026-11-06-llvm-schedule-audit176/README.md) gave register IFMA reciprocal throughput 2 cycles and vector add/shift latency 1 cycle. Hardware here instead gives approximately 0.50 and 2 cycles respectively. IFMA accumulator latency remains approximately 4 cycles. The hot folded-memory loops cost 17–19% more than the register loops in paired blocks; this does not establish that eliminating folded operands improves a full kernel, where register pressure and spill traffic differ.

The working hypothesis is therefore to examine dependency chains and carry work in the actual accepted consumer before selecting another arithmetic change. A lower IFMA count alone is insufficient evidence of a faster field operation. The calibration does not prove the full kernel is dependency-bound, quantify removable proof time, validate a complete Zen 5 model, or transfer to AWS/Aeglos/M5. Existing rejected arithmetic/layout/width/scheduler candidates remain closed.

## Preparation and preserved failures

The automatic LLVM snippets were preparation only and were replaced before measurement by explicit fixed-register snippets. All 16 full objects were assembled and decoded once, then qualified without execution. The first LLVM measurement command rejected internal CPU pinning in in-process mode. The repaired command then failed because libpfm could not resolve `cycles_not_in_halt`. Both stopped before snippet execution.

A Rust diagnostic reused the existing relocation-free `.text` functions and read Linux hardware cycles/instructions through a pinned perf-event group. It validates executable section/function bounds, affinity, event IDs, raw retirement counts and enabled/running time. No C/assembly production rewrite, system package installation, production dependency, new production buffer, field candidate or proof is introduced. The diagnostic scratch mapping is 1 MiB; no prover RSS or minimum-RAM conclusion follows.

The final analyzer initially compared a reconstructed and JSON-parsed floating mean with exact equality (one-rounding-unit difference). The preserved offline repair changes only that comparison to a relative `1e-12` serialization tolerance. Hardware gates, samples and the original capture source/binary remain unchanged.

## Unchanged full-proof baseline and goal

These are inherited accepted-176 local 32-worker single-proof timings, **not measurements from this calibration**. The accepted qualification retains its previously documented physical-16 calibration engineering exception; this study does not reopen it. The original goal remains active and unmet on all eight shapes.

| Shape | Accepted CPU ms | M5 CPU ms | M5 Metal ms | CPU 10× target ms |
|---|---:|---:|---:|---:|
| 1x2 | 7.928660 | 37 | 24 | 3.7 |
| 2x2 | 11.151932 | 59 | 30 | 5.9 |
| 4x2 | 16.086386 | 109 | 48 | 10.9 |
| 8x2 | 24.179453 | 189 | 74 | 18.9 |
| 12x2 | 33.884455 | 284 | 101 | 28.4 |
| 16x2 | 40.077381 | 335 | 124 | 33.5 |
| 24x2 | 65.850777 | 528 | 176 | 52.8 |
| merge24 | 65.060937 | 507 | 175 | 50.7 |

Inherited merge54 time is 161.883899 ms, without a supplied M5 reference. The inherited 132.75 MiB RSS is not a newly measured peak or minimum RAM requirement. Proof ledger stays at 30,846. No new AWS, Aeglos, Gnark, portability or memory-pressure result is claimed.

## Artifacts

[Analysis and every block](artifacts/analysis.json), [independent review](artifacts/review.json), [protocol](artifacts/protocol.md), [capture source](artifacts/campaign.rs), [offline analyzer](artifacts/offline-analysis.rs), [review/publication source](artifacts/review-publish.rs), [repair notes](artifacts/offline-repair.md), [source/receipt/object archive](artifacts/source-and-receipts.json.gz), [archive member hashes](artifacts/archive-index.json) and [local input/receipt pins](artifacts/input-and-receipt-pins.json). The archive includes exact frozen ELF objects encoded as hex; third-party LLVM executables/libraries and source are not redistributed. Retain the original failed workflow evidence rather than resuming or repeating successful measurements.
