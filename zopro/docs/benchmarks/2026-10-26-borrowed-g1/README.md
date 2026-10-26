# Borrowed pending operands for ordinary G1

The ordinary-G1 extension improves the 32-worker primary screen against accepted, owned and K/H-only controls in all three blocks. The screen does not advance: the physical-16 nonregression checks fail. Both borrowed candidates are retained; production is unchanged. No minimum gain floor is imposed.

The private Rust module now supports `CurveConfig<Base=Fq>`. A default-none `BatchInverse` hook permits native Fq to use borrowed pending records in ordinary window MSMs after grouping. Admission requires 1,024–65,535 points, one/two partitions, widths 10–14 and the existing IFMA/deferred-sign/packed/direct guards. G2 and unsupported cases retain owned code. Grouped point vectors outlive the synchronous parallel collect. The original window geometry, digit extraction, short-top algorithm, partition merge order, weighted reduction, grouping and conservative forecast remain. The inherited specialized window also omits unused scalar-inversion vectors while retaining used SIMD scratch. No unsafe lifetime extension, point table, scalar matrix, new pool, solver plan, allocator or image format.

Compared with accepted, three originals and one new module change; 167 originals remain identical. The incremental change from the [K/H-only candidate](../2026-10-26-borrowed-pending/README.md) affects two originals and that private module. Closing audits check the retained owned K/H and ordinary MSM bodies exactly, the borrowed K/H body after type substitution, and the G1 arithmetic body after operand-access substitution.

Host: Ryzen 9950X. Four variants compare the exact accepted executable with three modes of one newly linked native/fat-LTO executable: owned, K/H-only and all-G1. Three rotated/reversed blocks use fresh processes with 17 timed SDK requests and three primes each, at 32 workers on CPUs 0–31 and 16 physical workers on CPUs 0–15. Timers include witness construction, fresh blinded proving, verification and serialization; loading is excluded. No owned helper, build, hash or archive overlaps SDK captures.

|Workers|Accepted ms|Owned ms|K/H-only ms|All-G1 ms|All/accepted paired|All/owned paired|All/KH paired|All CPU ms/request|All peak RSS MiB|
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
|32|8.500571|8.617030|8.364043|8.191548|0.965767922|0.954445771|0.982159972|189.833|95.25|
|16|10.589420|10.498078|10.046065|9.994728|1.062563184|1.067630020|1.107636758|122.370|74.50|

Displayed medians are independent. Ratios pair process medians within blocks and take a geometric mean. At 32 workers, all-G1 is 3.423% faster than accepted, 4.555% faster than owned and 1.784% faster than K/H-only, with 3/3 wins against each. Paired median savings are 291.791, 384.275 and 172.495 µs respectively. This supports an incremental ordinary-G1 benefit in this screen. All owned/accepted calibration checks pass at both worker settings.

At 16 workers, all-G1 process medians are 9.994728, 9.845567 and 15.565774 ms. The final process fails nonregression against accepted/owned/KH; paired geometric ratios are 1.062563, 1.067630 and 1.107637. The displayed median across the three process medians is lower than the controls, but that does not erase the failed rule. The slow process has 17 request times from 13.578426 to 16.405377 ms, not one isolated slow request. It uses 111.626 summed CPU ms/request versus 122.370 and 123.204 in the first two all-G1 processes; involuntary switches/request are 3,867.882 versus 471.588 and 81.353. Control processes also slow in that block.

Those counters associate the slow process with reduced CPU occupancy and increased switching. They do not identify forced external preemption, caller yielding, a cache cause or a removable wall-time fraction. Earlier scheduler captures cannot establish this process's cause. All observations remain included; no successful capture is repeated or excluded. The full matrix, new cycle profiles and pressure qualification were conditional on advancement and were not run.

Nominal ordinary-window capacity falls by 22,400 bytes for pending records plus 12,800 bytes for unused scalar scratch at width 12/batch size 200. This is a layout calculation, not measured peak/minimum RAM. Finite whole-process RSS includes loading and primes and does not qualify a memory saving. The K/H nominal calculation remains in the parent report.

Validation: 33 native field tests, 114 core tests, two raw tests, six cubic tests, generator check, portable/no-IFMA consumers and 11/14 field tests pass. Added ordinary-query tests compare 96 borrowed and 96 owned results with generator oracles across 1,024/21,006 terms, one/four workers, one/two partitions, widths 12/13 and six scalar families. Dispatch and small/G2/geometry fallbacks are checked. All parent borrowed-kernel, tail, collision, reuse and K/H tests remain enabled.

The initial core compile rejected `sum::<Fr>()` in a test oracle; a fold repairs it before SDK capture. Previously passed compiler/generator/33-field evidence is reused with 26 unchanged field/workspace inputs checked. The corrected workflow's link step initially referenced the old artifact-log directory; a recorded directory alias resolves it and the identical protocol resumes without repeating passed steps. Original failed source, plans and receipts are retained. These repairs do not change prover arithmetic.

All nine owned/all preflights produce 72 internally verified SDK requests and 18 distinct Gnark exports. The short screen adds 480 requests and 24 exports. Total: 552 requests, 42 distinct independently verified 164-byte Gnark exports, 84 proof/public members, pinned public binding and freshness against 6,886 prior identities; next ledger 6,928. Closing source/library/binary/input/tool pins and all 170 production hashes are checked. SDK: `/tmp/zopro-borrowed-g1/native-repair/on-sdk`, SHA256 `f63c6426ab449eaa7c4d44635f7a59dfdf2a5ae93c793cb196e03d0460e3868b`.

Experimental primary32 at 8.191548 ms is 4.517× the supplied 37 ms M5 CPU reference and 2.930× the 24 ms Metal reference. No other shape has a new latency measurement here. Below are the accepted controls from the [latest full matrix](../2026-10-26-pairing-x-chain-sdk/README.md), compared with historical supplied Apple references.

|Shape|Previous full accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|
|---|---:|---:|---:|---:|---:|
|1x2|8.642453|37|4.281×|24|2.777×|
|2x2|11.756497|59|5.019×|30|2.552×|
|4x2|16.230727|109|6.716×|48|2.957×|
|8x2|24.740679|189|7.639×|74|2.991×|
|12x2|34.551946|284|8.220×|101|2.923×|
|16x2|40.814660|335|8.208×|124|3.038×|
|24x2|68.811062|528|7.673×|176|2.558×|
|merge-24x1|67.064555|507|7.560×|175|2.609×|
|merge-54x1|177.082271|—|—|—|—|

Merge 54 has no supplied reference. The all-eight CPU 10× goal remains active and unmet. No new AWS/Aeglos, phone, TEE, giant-circuit, swap, minimum-RAM or throughput result is claimed.

[Design](artifacts/design.md), [analysis](artifacts/screen-analysis.json), [physical-16 observations](artifacts/physical16-observations.json), [source audit](artifacts/closing-source-audit.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).
