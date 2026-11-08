# Late-centered complete caller results

**The fixed late-centered recipe fails the declared performance gates and is not selected for SDK integration.** All 1,188 caller processes pass changing-state output, allocation and capacity checks; independent review confirms the results. The all-context paired aggregate is 3.22% slower than the fresh control and 4.25% slower than the frozen original. These are component results on the local Ryzen 9 9950X, not new proof timings. Production remains unchanged.

The experiment follows [native arithmetic qualification](../2026-11-08-late-centered-native176/README.md). It keeps signed intermediates through prefix/reverse operations, performs unsigned CIOS with late sign compensation, and centers the final quotient. The ordinary full-group point-product recipe remains 600 G1 / 1,450 G2 IFMA sites; wide export adds two/four. This tests correction removal, without removing the underlying multiplication work.

## Declared comparison

The [protocol](artifacts/protocol.md) is frozen before any caller or preflight execution. G1/G2 at 200/201/352/353 points run on 1/16/32 private Rayon workers: 24 contexts, including full groups and tails. Arms are the exact frozen accepted-source diagnostic executable, a fresh OFF control and fixed candidate ON. None is an accepted SDK executable. Fresh arms share the same private field/core libraries and driver, differing only by compile-time candidate selection.

Seventy-two 64-round preflights precede 15 rotated/reversed paired blocks, with 8,192 rounds per worker: 1,080 latency processes. Eighteen separate PMU processes and 18 cycle profiles follow. All 1,188 recordings finish before the 90 ownership/decoder/metadata/event/loss commands. Total: 1,278 commands, 447 pins. No preflight-check helper, build, hash, analysis, report or push overlaps recording. The fixed run finishes once; no observation is excluded, extended or repeated.

Each process performs three checked 128-round primes, resets, snapshots capacities, and times the complete borrowed-reference assembly, prefix, inverse, reverse, sign handling, normalization, export, scalar tails, scatter and dispatch/join. Every final bucket coordinate is checked against the original coefficient oracle and an independent Arkworks combined digest. The independent reference covers all 168 context/round combinations and hashes 688,128 complete group outputs. Native exception coverage is inherited; ordinary diagnostic callers retain finite/distinct-destination/nonzero-denominator preconditions. The diagnostic O(n²) fallback guard is not in either timed path. This does not qualify production Window integration.

Latency ratios use the 15 paired block ratios, with sorted zero-based ranks 3/7/11 as lower/median/upper bounds. Their nominal 96.484375% marginal coverage assumes exchangeable blocks; they are not familywise bounds. A ratio below one favors ON. OFF/frozen calibration must stay within factor 1.03 in every context; ON must have upper bounds at most 1.02 against both controls. Each curve's four-size 32-worker aggregate and the all-context equal-log aggregate must have upper bounds below one and at least 10 winning blocks against both controls.

## Complete timings

Times below are median microseconds per joined batch wave, not proof latency or per-point cost. Ratio medians are computed from paired block ratios, so they can differ from ratios of the displayed marginal time medians.

|Context|Frozen µs|OFF µs|ON µs|ON/OFF paired median [lower, upper]|Winning blocks / 15|
|---|---:|---:|---:|---:|---:|
|G1-200-w1|4.2302|4.2313|4.1649|0.984933 [0.982764, 0.986039]|15|
|G1-200-w16|4.8057|4.7932|4.7821|0.998142 [0.996436, 1.005010]|11|
|G1-200-w32|6.9257|6.9003|6.9984|1.012679 [1.004715, 1.017301]|2|
|G1-201-w1|4.3500|4.3397|4.2776|0.985763 [0.984413, 0.987845]|15|
|G1-201-w16|4.9383|4.9237|4.9020|0.995551 [0.993375, 1.000335]|11|
|G1-201-w32|7.0745|7.1323|8.5126|1.051239 [1.013801, 1.213985]|2|
|G1-352-w1|6.9710|6.9617|6.8511|0.983054 [0.982335, 0.984297]|15|
|G1-352-w16|7.9588|7.9496|7.9067|0.994203 [0.993280, 0.996381]|14|
|G1-352-w32|11.7834|11.6258|12.1952|1.039005 [0.958889, 1.063950]|4|
|G1-353-w1|7.0757|7.0605|6.9493|0.983661 [0.981902, 0.984969]|15|
|G1-353-w16|8.0961|8.0837|8.0321|0.996042 [0.991325, 1.007998]|8|
|G1-353-w32|11.7889|11.7424|15.0040|1.049260 [1.034375, 1.287968]|1|
|G2-200-w1|7.7062|7.8366|8.1539|1.040849 [1.039885, 1.042101]|0|
|G2-200-w16|8.9450|9.0905|9.6706|1.064265 [1.057145, 1.067420]|0|
|G2-200-w32|14.5569|15.0396|15.5340|1.038569 [1.022507, 1.110771]|0|
|G2-201-w1|8.1360|8.3956|8.5921|1.022156 [1.020663, 1.023015]|0|
|G2-201-w16|9.4324|9.7020|10.1418|1.043780 [1.038880, 1.047829]|0|
|G2-201-w32|14.8924|15.4498|16.1885|1.057871 [1.032084, 1.100960]|0|
|G2-352-w1|13.0393|13.1802|13.8189|1.048368 [1.047535, 1.048990]|0|
|G2-352-w16|15.2109|15.3361|16.5067|1.076438 [1.073985, 1.076987]|0|
|G2-352-w32|24.9100|24.9737|26.9563|1.078528 [1.067782, 1.083824]|0|
|G2-353-w1|13.4316|13.6969|14.2114|1.037271 [1.036017, 1.038019]|0|
|G2-353-w16|15.6569|15.9368|16.9699|1.064662 [1.062559, 1.082903]|0|
|G2-353-w32|25.3604|25.5705|27.3870|1.071333 [1.060165, 1.074736]|0|

Single-worker G1 is 1.42–1.69% faster against fresh OFF across the four sizes, with 15/15 wins in each. The G1 352-point/16-worker case also has an upper bound below one. These positive observations are retained; they do not establish a full-recipe or proof gain. G2 ON/OFF medians regress in all 12 contexts, by 2.22–7.85%. No favorable worker/curve subset is selected after measurement.

|Declared aggregate|Paired median [lower, upper]|Winning blocks / 15|
|---|---:|---:|
|ALL/OFF|1.032184 [1.027573, 1.044294]|0|
|ALL/frozen|1.042522 [1.033400, 1.051238]|0|
|G1/OFF|1.042641 [1.014507, 1.112198]|3|
|G1/frozen|1.058109 [0.999418, 1.125457]|4|
|G2/OFF|1.064737 [1.056006, 1.074463]|0|
|G2/frozen|1.081915 [1.074261, 1.086257]|0|

There are **43 failed timing checks: six calibration, 31 nonregression and all six aggregate gain checks**. Calibration failures are G1 200/201 at 32 workers and G2 200/32, 201/1, 201/16 and 201/32. They remain failures; source identity alone does not guarantee equivalent execution time. Fresh ON/OFF paired results remain visible, but do not repair calibration failures. Exact causes of the control differences and 32-worker variability are unresolved.

## Profiles, work and memory

All 18 cycle profiles qualify: 121,484 total samples, at least 2,958 per profile, zero lost records, 100% assigned-worker samples, at least 99.7681% within the exact predeclared caller/kernel/inverse symbol inventory, and at most 0.2062% unknown samples. Binary MMAP, PID/TID/CPU ownership, 16-million-cycle user-only period, inheritance and FIFO disable/enable/disable acknowledgements are checked. These are warm flat cycle profiles, not call-stack or causal critical-path measurements. All four grouped user PMU events have nonnegative counts and at least 99.9% running coverage.

The separate PMU observations show more retired instructions despite unchanged selected scratch sizes. These six counter captures per arm are diagnostic, not repeated statistical estimates. Cycles count summed worker CPU work; they are not joined wall latency.

|Context|ON/OFF retired instructions|ON/OFF user cycles|
|---|---:|---:|
|G1-200-w1|1.080586|0.979050|
|G1-200-w32|1.080528|0.985297|
|G1-352-w32|1.087270|1.009795|
|G2-200-w1|1.064403|1.037731|
|G2-200-w32|1.064393|1.026923|
|G2-352-w32|1.066647|1.044780|

Instruction growth is about 8.1–8.7% for these G1 cases and 6.4–6.7% for G2. This supports the limited inference that compensation/normalization/export and their compiled execution are material costs; it does not attribute every regression to one instruction or to RAM bandwidth. The profiles are concentrated in the actual point kernels and inverse routines. Cache-miss observations vary strongly by case and are not a replicated causal memory diagnosis.

All 1,188 callers request zero timed allocations/reallocations and retain capacities across the ROI. Selected scratch capacities are exactly equal among all three arms for each context and stable across blocks. G1 scratch bytes per worker are [40000,0]/[40000,640]/[70400,0]/[70400,640] for 200/201/352/353; G2 uses [16000,0]/[16000,1280]/[28160,0]/[28160,1280]. Worker stacks are requested at 512 KiB; actual stack usage is unmeasured. No table, retained key/solver copy or production pool is added. Unselected empty Vec headers belong to this diagnostic adapter.

Across latency processes, ON process high-water RSS ranges from 2.75 to 18.50 MiB. This covers fixture/allocator/library/thread process state, not a prover memory peak, peak live bytes or a minimum-RAM/mobile bound. No synthetic memory cap, pressure/swap, huge circuit, concurrent-proof throughput, ARM/TEE or AWS/Aeglos result is added.

Four new release-consumer functions are decoded once: OFF G1/G2 are 26,198/51,830 bytes; ON G1/G2 are 36,814/67,441 bytes. Exact disassemblies and all overlapping backward spans are archived. Whole-function static IFMA/stack rows include other branches and are not disjoint dynamic work, spill counts or speed estimates.

## Build and evidence scope

All six release/gold build steps finish once. Libraries use embedded bitcode; final consumers use Rust 1.98.1, -O, native CPU, fat LTO and one codegen unit, with accepted configuration flags. Three original parents have append-only child/export declarations; stripping them recovers all 176 original inputs (173 copies are already exact). Qualified native kernel/caller bytes remain fixed. An initial offline qualifier incorrectly required fat LTO on rlib compilation; its source/error are retained, and only that checker assertion is corrected to require embedded bitcode for libraries. No compiler flag/source change or successful build/test rerun follows. Before clocks, final symbol inventory also includes the actual demangled `<Caller>::run` names; the earlier inventory and codegen remain preserved.

The independently compiled reader verifies all command order/arguments/environments and stream receipts, all pins, 1,188 independent-gold outputs, capacities, affinity, 78 timing checks, PMU values and all profiles. Its review passes while confirming the candidate's declared gate failure. The archive preserves exact text inputs/results/receipts, failed qualifier source/error, plans/pins, adapters, codegen and decoded profiles. Libraries/executables/raw perf data are bound by hashes and excluded, along with unchanged full source copies, keys/witnesses/proofs/private ledgers and external full papers. Every archived UTF8 payload and gzip roundtrip is checked.

This closes the fixed recipe without adoption or an unchanged rerun. The next research step is to use the retained instruction/codegen/profile evidence and prior experiment history to identify a distinct reduction in multiplication work or dependency depth. Merely reducing modular corrections while increasing total execution work has not delivered the needed jump. A future candidate still needs complete-caller qualification, actual Window/core integration and verified SDK/all-shape/resource checks.

## Unchanged proof comparison

All proof results below are inherited from the [native-stage report](../2026-11-08-late-centered-native176/README.md): latest local primary plus seven explicitly older full-campaign rows. Supplied M5 CPU/Metal references use different hardware. No component time above is substituted for a proof time.

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.002770|37.0|24.0|4.623×|2.999×|3.7|
|2x2|11.241463|59.0|30.0|5.248×|2.669×|5.9|
|4x2|15.933816|109.0|48.0|6.841×|3.012×|10.9|
|8x2|24.234311|189.0|74.0|7.799×|3.054×|18.9|
|12x2|33.855565|284.0|101.0|8.389×|2.983×|28.4|
|16x2|40.071563|335.0|124.0|8.360×|3.094×|33.5|
|24x2|65.917462|528.0|176.0|8.010×|2.670×|52.8|
|merge-24x1|65.305023|507.0|175.0|7.764×|2.680×|50.7|

Accepted local 1×2 remains **about 8 ms**: the same binary previously measured 7.915095/7.928660/7.995682 ms; latest 8.002770 ms. Inter-run variation is unresolved; there is no adopted regression from this trial. Merge54 remains inherited 162.240109 ms without a supplied M5 reference. Production176, SDK6d83 and canonical35,171 remain unchanged. The verified all-eight CPU10× goal remains active and unmet, **0/8**.

[Full timing/gate/resource data](artifacts/timings.json), [independent review](artifacts/review.json), [release qualification](artifacts/release-qualification.json), [independent gold](artifacts/independent-gold.json), [codegen](artifacts/codegen.json), [binary identities](artifacts/identities.json), [exact profile inventory](artifacts/target-symbols.json), [protocol](artifacts/protocol.md), [evidence archive](artifacts/evidence.json.gz).
