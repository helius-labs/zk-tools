# G1 four-vector records: complete general-MSM results

**The fixed candidate is not adopted.** G1 group records shrink from 1,600 to 1,280 bytes, with identical retained record counts. Solo G1 improves about 1.1%, and the 16-worker comparison improves against fresh OFF, but no 32-worker gain is established. The complete screen passes **40/46 checks**. All memory and diagnostic-quality checks pass; one calibration, one nonregression bound and four gain checks fail. Accepted full-proof 1×2 latency remains **7.77 ms** at 32 workers.

The original G1 streaming group stores AX, AY, X-sum, denominator and scaled numerator. The candidate stores AX, AY, canonical BX and numerator. Forward denominator computation and prefix/numerator recurrence remain original. The reverse pass reconstructs the denominator from the same AX/BX operands and moves the original X-sum construction there. The net arithmetic cost is one extra vector subtraction per eight-point group; no multiplier, inverse, coordinate gather/repack or serial preprocessing is added. Scalar tails, signs, routing, queues, G2, generic trees and original field1200 stay unchanged.

|G1 record property|Original|Candidate|
|---|---:|---:|
|Field vectors per group|5|4|
|Record bytes|1600|1280|
|Alignment bytes|64|64|
|Retained record count for equal workloads|same|same|

Both reserved raw writes and push constructors use the new layout, initializing all four fields before publishing length. Unsupported native targets retain the original struct and body. Workspace forecasts already use the record type's size; no admission policy or artificial RAM cap changes. There is no new vector, key table/copy, solver plan, retained cache or production pool. This differs from closed G1 prefix-only streaming, G1 tree records, G2 denominator caches and affine-REDC-offset trials; those outcomes are unchanged.

The trial uses public `msm_g1` and `msm_g2` with immutable Fr inputs. It times internal canonical conversion, grouping, full MSM, allocations/drops and affine normalization. Fixture decoding into Fr occurs outside the timer. These are **complete general MSMs, not SDK proofs or the proof-specific exclusive Phi2 entry**. In particular, G2 control times are not interchangeable with the prior exclusive-Phi caller results. Every prime and timed output matches a retained independent Arkworks complete-query oracle; points and scalars remain unchanged.

|Context|Accepted REF ms|Fresh OFF ms|ON ms|ON/OFF [bounds], wins|ON/REF [bounds], wins|
|---|---:|---:|---:|---|---|
|1x2-A-w1|14.812169|14.795003|14.645102|0.989305 [0.987325,0.992653], 15/15|0.989529 [0.986009,0.993986], 15/15|
|1x2-A-w16|1.639942|1.644911|1.623806|0.990410 [0.984833,0.998637], 12/15|0.996775 [0.985028,1.003034], 10/15|
|1x2-A-w32|1.359398|1.370514|1.359605|0.995204 [0.966941,1.012530], 10/15|1.005298 [0.985839,1.017260], 6/15|
|2x2-A-w32|1.698082|1.693539|1.685208|1.001752 [0.988432,1.016261], 7/15|0.998048 [0.983411,1.032230], 8/15|
|1x2-B-w32|3.172914|3.177239|3.185761|1.004723 [0.994352,1.009525], 7/15|1.004854 [0.993192,1.012675], 4/15|
|2x2-B-w32|3.800355|3.785600|3.784236|0.996722 [0.992617,1.011988], 8/15|0.993693 [0.979144,1.017374], 9/15|

Primary32 ON/OFF is 0.995204 [0.966941, 1.012530], 10/15 wins; ON/REF is 1.005298 [0.985839, 1.017260], 6/15. Both gain bounds cross parity. Its OFF/REF calibration is 1.013095 [0.976336, 1.038562], outside the declared factor1.03 interval at the upper bound. This limits attribution of that context's differences solely to the record change. It does not establish a 3.86% control slowdown. Five other calibrations pass.

The equal-log two-A32 aggregate is ON/OFF 0.996239 [0.985898, 1.016504], 8/15, and ON/REF 1.005930 [0.981798, 1.017576], 6/15. Both gain checks fail. Secondary A ON/REF has a nonregression upper bound of1.032230 with lower0.983411; this fails the2% guard without proving a3.2% slowdown. All six ON/OFF nonregressions and five of six ON/REF nonregressions pass. G2 controls pass. Positive solo and 16-worker evidence is retained without favorable-worker adoption, extra blocks, altered records, threshold changes or an unchanged rerun. No full-SDK trial is selected for this fixed layout.

Ryzen 9950X, 16 physical/32 logical CPUs, shared host. REF reuses the accepted busy-reset176 core; OFF/ON reuse the exact newly qualified cores, with original field/dependencies. Three driver links share native/O3/fat-LTO/one-CGU flags, allocator and environment, including dense busy reset. Worker i pins to CPUi, solo CPU2; whole processes use taskset2/0–15/0–31. Query workers use default stacks, with no stack_size request or RUST_MIN_STACK. The requested512 KiB stacks belong to native qualification pools.

Eighteen preflights each run three checked primes and one query. Fifteen fixed rotated/reversed context/arm blocks retain all270 latency processes, with seventeen timed queries after three checked primes. The unit is arithmetic mean of integer nanoseconds, reported as the median of fifteen means. Paired ratios use sorted zero-based ranks3/7/11, nominal96.484375% marginal median coverage conditional on independent exchangeable blocks, without familywise or stationarity guarantees. The prospective46-check rule includes six calibrations, twelve nonregressions, four gains, twelve RSS comparisons and twelve diagnostic checks. There is no positive gain floor. Passing would permit separate full-SDK ALL8+54/primary1/16/32/fresh-control/Gnark qualification, never immediate adoption.

|Context|REF / OFF / ON peak process RSS MiB|OFF / ON CPU ms per query|
|---|---:|---:|
|1x2-A-w1|10.00 / 10.00 / 10.00|14.795 / 14.645|
|1x2-A-w16|19.50 / 19.25 / 19.25|18.380 / 18.189|
|1x2-A-w32|24.25 / 26.00 / 24.25|29.502 / 29.560|
|2x2-A-w32|32.75 / 33.00 / 33.00|31.259 / 31.065|
|1x2-B-w32|40.00 / 39.75 / 40.00|78.585 / 78.408|
|2x2-B-w32|59.50 / 58.75 / 59.00|85.849 / 86.073|

All twelve RSS guards pass: ONmax≤1.05×controlmax+1MiB. The20% saving concerns retained group-record payload, not whole-process RAM. RSS includes fixture/runtime setup and primes, and is not proof peak, minimum RAM, pressure/swap qualification or a universal memory reduction. Fault counts remain descriptive; they do not establish disk paging or a DRAM-capacity bottleneck.

|Diagnostic context|Instructions OFF / ON per query|User cycles OFF / ON per query|
|---|---:|---:|
|1x2-A-w16|210055401 / 211221948|92039969 / 91024078|
|1x2-A-w32|220768553 / 221371300|137107518 / 135091612|
|1x2-B-w32|551849733 / 551777730|353093825 / 351451563|

The primary-A32 panel has about0.27% more instructions and1.47% fewer cycles. G1-stream flat samples move2,469→2,414, confirming the affected consumer executes. These single panels and flat samples do not identify an exclusive critical path, executed spill count or causal cache/DRAM explanation. Record traffic is smaller, but latency remains uncertain at32workers.

After all latency, six FIFO-gated grouped PMU panels run101queries and six fixed16M cycles:u profiles run1,021queries. All recordings finish before ownership transfer, decode or analysis. All twelve quality checks pass:100% PMU scheduled coverage;73,640 profile samples; at least5,801samples per profile; maximum unknown below1.31%; sampled PID/TID/CPU/period and executable mapping checks; zeroLOST. Absolute period coverage is unvalidated. Raw symbols and resource counters are retained.

All130 unfiltered native core tests pass. New tests compare2,393,304 raw output/untouched slots with the exact old kernel and the same number with independent Arkworks over4,924cases. Full/tail boundaries, all256sign masks in fixed workloads, FAST/write/reserve modes, borrowed/unsigned forms and three reuse passes are covered. Another122 threaded/shared cases exercise all53 actual workers across1/4/16/32-worker pools and four OS callers sharing an eight-worker pool. Record-count capacities agree, and the retained byte relation is checked at every pass. Layout/checked-size cases pass. Original ordinary-pair preconditions remain; existing routing/exception tests stay enabled.

Native OFF/ON and the no_ifma-exclusion core build pass. The exclusion build uses native field1200 and is a compile guard, not full generic-field execution or physical ARM/mobile evidence. Existing field qualification is inherited without rebuild/replay. All176 production manifest hashes remain exact; the private snapshot has175 unchanged originals, one reversible msm.rs parent and one new test child.

All343 serialized xtask commands through the analyses succeed on first execution: five native, six caller/helper builds, eighteen preflights,282capture commands, thirty offline commands and two post-analysis commands. The300 caller processes produce12,240 checked complete MSMs including primes. No SDK request or new proof export occurs. Old fixture/oracle production, successful tests/builds, proof verification or captures are not replayed for bookkeeping. Direct preparers/compiler invocations and publication/checkpoint are separate. During preflight/recording only the exact confirmed live handle is polled; no owned helper/build/hash/analysis/report/push overlaps recording. Shared-host exclusivity is not claimed.

Producer and rank reader are separately compiled and frozen before preflight. They reconstruct all270 raw means and46 decisions, using u64sum/sort versus u128fold/rank selection. Receipt/profile/PMU parsers and source are shared: this is a same-agent cross-check, not an external independent audit. Both pass first post-capture execution.

The full-proof observations below are inherited from the accepted retained-tail SDK campaign. Supplied M5 CPU and Metal references are historical, on different hardware, and not remeasured here.

|Shape|Accepted local CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.772288|37 / 4.761×|24 / 3.088×|3.7|
|2x2|10.929386|59 / 5.398×|30 / 2.745×|5.9|
|4x2|15.731643|109 / 6.929×|48 / 3.051×|10.9|
|8x2|23.965774|189 / 7.886×|74 / 3.088×|18.9|
|12x2|33.742953|284 / 8.417×|101 / 2.993×|28.4|
|16x2|39.704242|335 / 8.437×|124 / 3.123×|33.5|
|24x2|66.034077|528 / 7.996×|176 / 2.665×|52.8|
|merge-24x1|65.387417|507 / 7.754×|175 / 2.676×|50.7|

Accepted primary remains7.772288ms at32workers,9.049269ms at16 and94.218289ms solo; the same accepted binary was already below8ms. Merge54 remains162.096312ms without a supplied M5 reference. Canonical45,335 verified proofs remain unchanged; every future export must seed retained-tail-sdk176/proof-sha256-after-capture.json. Full CPU AVX10× goal remains active and unmet,0/8. No new AWS/Aeglos, phone/ARM/TEE, huge250/4999, pressure/swap, minimum-RAM or concurrent-throughput result is added.

The fixed four-vector record recipe is closed. Next work must use the positive small-worker and uncertain32-worker evidence without treating record-byte savings as a proportional proof-speed improvement.

[Results and profiles](artifacts/caller-analysis.json), [rank reader](artifacts/caller-review.json), [decision](artifacts/decision.json), [design](artifacts/design.md), [protocol](artifacts/caller-protocol.md), [source recovery](artifacts/source-edits.json), [receipts](artifacts/receipts.json), [inherited CPU/Metal comparison](artifacts/inherited-comparison.json), [continuation](artifacts/next.md), [sources and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
