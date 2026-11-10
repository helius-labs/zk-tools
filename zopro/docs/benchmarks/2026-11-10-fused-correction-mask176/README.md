# Fused correction-mask query comparison

**Rejected; no SDK trial or production change.** The fixed correction-mask implementation is slower in complete G2 queries with all eight control-calibration checks passing. Primary32 ON/OFF is **1.012729 [1.009442, 1.014757]**, 0/31 wins. The equal-log two-G2-shape32 aggregate is **1.011905 [1.008992, 1.017152]**, 2/31. Nine of 54 checks fail: five nonregression and four required gains. All eight RSS, eight PMU, eight profile and two unchanged-A checks pass.

The candidate reuses the existing fused-result high-limb classification. The original needs==0 fast return remains exact. Inside correction, any equal top limb takes the original complete reduction. Otherwise the existing mask determines which lanes subtract 2p in a signed carry pass, returning normalized digits without the final five blends. Those blends are replaced by five masked constants plus equality/branch work; this is not five fewer total instructions. IFMA and field-product counts are unchanged. The earlier general high-borrow experiment added a guard to its common path; this narrower version reuses a guard already present. Neither result is reopened or claimed novel.

For normalized U in [0,4p), unequal top digits determine U>=2p exactly. Selected U−2p and unselected U are in [0,2p). Signed digit intermediates lie in [−2^52,2^52−1], with final carry zero. Equal-high fallback handles adversarial ties exactly. The four-lane runtime retains its original path under LANES==4. There is no changed representation, CIOS recurrence, point formula, input table, key copy, solver plan, heap scratch, pool or architectural RAM cap. Compiler layout and frames may change.

Qualification passes **36 native field tests** (35 existing and one new), **11 generic field tests** with SIMD excluded and **127 existing native core tests**. The new test checks **57,632 raw lane executions** against original exact digits and an independent four-word conditional subtraction: both primes and radix type parameters, all 256 top-tie masks in three directions, bounds and random inputs. Radix contexts repeat numerical tuples; these are not 57,632 unique cases. Generic tests are not a real ARM/device qualification. Existing core tests cover tails, signs, reuse and small-stack workers; no new core mathematical population is claimed.

All 176 production hashes remain exact. Three private parents are reversible (two field files and the unchanged benchmark-API suffix), with one test child. The existing generator's lane substitutions map the inserted four-lane fragment to the eight-lane fragment; the generator itself is not executed and no full regeneration check is claimed. One field-width typo is repaired before ON qualification, retaining the failed source/command and reusing the successful OFF build because the edit is cfg-excluded there. A reader count assumption125→127 and a repair-helper borrow typo are documented. Successful native builds/tests and successful capture are not replayed.

|Complete query|REF ms|OFF ms|ON ms|ON/OFF [bounds]; wins|ON/REF [bounds]; wins|
|---|---:|---:|---:|---|---|
|1x2-B-w1|36.697760|37.324926|37.931223|1.015890 [1.014506, 1.017246]; 0/31|1.033417 [1.031935, 1.035221]; 0/31|
|1x2-B-w16|3.656522|3.655701|3.730652|1.020431 [1.016755, 1.023802]; 0/31|1.020364 [1.015892, 1.024293]; 0/31|
|1x2-B-w32|3.229727|3.238834|3.277668|1.012729 [1.009442, 1.014757]; 0/31|1.013798 [1.011832, 1.017798]; 2/31|
|2x2-B-w1|39.312318|39.941018|40.320657|1.010058 [1.008503, 1.010811]; 0/31|1.024880 [1.023927, 1.025700]; 0/31|
|2x2-B-w16|4.210406|4.212198|4.268305|1.017387 [1.010783, 1.020085]; 2/31|1.015330 [1.012116, 1.018567]; 3/31|
|2x2-B-w32|3.787257|3.788922|3.834407|1.012736 [1.007000, 1.018840]; 5/31|1.013379 [1.008023, 1.016470]; 5/31|
|1x2-A-w32|1.182899|1.183340|1.178021|0.997868 [0.987779, 1.003550]; 18/31|1.000035 [0.985539, 1.002711]; 15/31|
|2x2-A-w32|1.507220|1.510457|1.503584|0.997088 [0.991759, 1.003959]; 18/31|0.996469 [0.987517, 0.999395]; 22/31|

Two-G2-shape32 ON/REF is **1.013112 [1.008754, 1.017590]; 2/31**. Solo 1×2 and 2×2 lose all 31 blocks against OFF. Positive estimates in the unchanged G1 controls remain in the table; they do not rescue failed G2 selection. All failures and observations are retained, with no new blocks, changed thresholds, curve/size selector or equality-policy rescue.

Ryzen 9950X, 16 physical/32 logical CPUs. One complete query uses a pinned internal Rayon pool with 512 KiB worker stacks: solo CPU2, other workers0..N−1. Timed work includes B-word reset, grouping/admission, windows, arithmetic, weighting/Horner, allocations/drop and affine normalization. Loading/pool, three primes, final postcheck and oracle comparison are excluded. Every timed result matches retained independent Arkworks full-MSM hashes, and point inputs remain unchanged. Original oracle producers and solver/FFT extraction are not repeated. These query latencies are not full-proof timings.

The preset is frozen before preflight: **31 matched rotated/reversed blocks × 51 queries/process**, per-process median, paired process ratios and zero-index sorted ranks9/15/21. Bounds require independent/exchangeable blocks and are not a simultaneous guarantee. The longer preset replaces the prior15×17 screen to improve resolution before collecting these data; it is not an optional extension based on results. Eight OFF/REF calibration intervals must fit factor1.03; ON/control nonregression upper≤1.02; primary-A32 unchanged bounds fit factor1.03; primary-B32 and two-B-shape32 aggregate gains require upper<1 and≥21/31wins against both controls. No minimum positive-gain floor is imposed.

Capture completes once with **824 commands / 784 caller processes / 47,504 checked queries including primes and postchecks**:24preflights,744latency processes,8PMU+8profile processes and40offline steps. No owned build, helper, hash, decoder, analysis or report overlaps preflight or recording. Exact handle76189returns terminal0. Across qualification/helper/capture/closure workflows there are845steps,843successes and two retained failures. Direct preparation failures and warnings are described separately.

The frozen analyzer missed two descriptive counter literals during preset adaptation: raw analysis says400callers/440steps. Its vectors and the separate reader correctly check784/824 and47,504queries. The raw analysis is preserved in the archive. The published corrected copy changes **only those two metadata fields**, bound to the raw SHA and actual row/plan lengths. No timing, ratio, decision, resource observation, source/binary or successful analysis is changed or replayed.

|Complete query|REF / OFF / ON lifetime max RSS MiB|
|---|---:|
|1x2-B-w1|15.50 / 15.50 / 15.50|
|1x2-B-w16|27.50 / 28.50 / 27.75|
|1x2-B-w32|40.50 / 36.50 / 41.00|
|2x2-B-w1|27.25 / 27.25 / 27.25|
|2x2-B-w16|44.00 / 51.00 / 43.75|
|2x2-B-w32|76.25 / 77.25 / 76.25|
|1x2-A-w32|30.00 / 28.25 / 27.50|
|2x2-A-w32|35.50 / 35.25 / 30.75|

RSS includes process loading/primes/timing and is not minimum RAM, an allocation bound or a pressure/device test. The per-context guard is ON≤maxcontrols×1.05+1MiB. All eight pass.

|Single PMU panel|Instructions ON/OFF|Cycles ON/OFF|Cache misses ON/OFF|
|---|---:|---:|---:|
|1x2-B-w16|1.003652|1.015828|0.937960|
|1x2-B-w32|1.003746|1.016534|1.007942|
|2x2-B-w32|1.002740|1.006397|0.905642|
|2x2-A-w32|0.998895|0.992997|0.979115|

All grouped counters report100%running. The eight fixed16Mcycle profiles qualify with **50,540 samples**, zero LOST, at least2,973samples/panel and at most1.144%unknown. PID/TID/CPU, period and executable MMAP checks pass. The separate reader checks all824command/stream/managed-environment receipts,594pins,all47,504query validations and recomputes all54decisions. The readers share sampling/PMU text parsers, and absolute sampled-period coverage is unvalidated. Single panels and flat samples do not isolate register pressure, branch frequency, cache/DRAM or a wall-critical cause. The correction branch's hit frequency was not instrumented. More G2 instructions coexist with slower queries; no unique cause is assigned.

Accepted local32-worker full-proof observations and supplied historical M5 references remain unchanged:

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.829286|37 / 4.726×|24 / 3.065×|3.7|
|2x2|10.979335|59 / 5.374×|30 / 2.732×|5.9|
|4x2|15.803969|109 / 6.897×|48 / 3.037×|10.9|
|8x2|23.944017|189 / 7.893×|74 / 3.091×|18.9|
|12x2|33.703579|284 / 8.426×|101 / 2.997×|28.4|
|16x2|39.858478|335 / 8.405×|124 / 3.111×|33.5|
|24x2|66.186709|528 / 7.977×|176 / 2.659×|52.8|
|merge-24x1|65.489902|507 / 7.742×|175 / 2.672×|50.7|

Primary remains **7.829286ms / 133.50MiB**; its earlier7.803404ms observation uses the same accepted executable. Primary16 remains9.192237ms; merge54 remains162.541995ms without an M5 reference. Canonical43,730verified proofs and accepted SDK/source176 are unchanged. CPUAVX10×ALL8 stays **active and unmet,0/8**. No new SDK request/proof/Gnark verification, AWS/Aeglos, phone/ARM, huge-circuit, swap/pressure, minimum-RAM or concurrent-throughput result is added.

[Corrected analysis](artifacts/analysis-corrected.json), [metadata correction](artifacts/metadata-correction.json), [independent result review](artifacts/review.json), [qualification review](artifacts/build-review.json), [prospective check](artifacts/prospective-review.json), [diagnostics](artifacts/diagnostic-summary.json), [decision](artifacts/decision.json), [protocol](artifacts/protocol.md), [design](artifacts/design.md), [field repair](artifacts/repair.md), [helper repairs](artifacts/helper-repair.md), [continuation/autoresearch loop](artifacts/next.md), [sources and exact receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Private root:/home/ubuntu/src/zopro-perf-work/fused-correction-mask176.
