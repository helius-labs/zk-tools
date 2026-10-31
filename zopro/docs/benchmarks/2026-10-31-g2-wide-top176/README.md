# 13-bit G2 windows with a nine-bit indexed top

The candidate is rejected. Primary G2 query latency rises from **3.287470 to 4.492732 ms**, a paired **36.53% regression**, with zero wins in 15 blocks against either control. CPU service rises from 71.0715 to 109.4432 ms/query, and peak process RSS from 35.25 to 50.00 MiB. The primary 32-worker control calibration passes; its negative result does not depend on calibration failures elsewhere.

The complete-query gate is **false**. Production remains source 176 / SDK `6d83…`, with accepted primary proof latency **7.928660 ms**. This experiment measures complete MSM queries. It produces no new proof-latency result, adoption or full-SDK follow-up.

Primary B2/32 ON/OFF is **1.365277554 [1.356242657, 1.380966286], 0/15 wins**; ON/frozen is **1.362657056 [1.347701272, 1.383632946], 0/15 wins**. The 22 failed checks and every original observation remain in the [independent review](artifacts/review.json).

The current Phi2 decomposition gives two signed components with magnitudes below 2^126. The candidate replaces ten ordinary 12-bit windows and a six-bit indexed top with nine ordinary 13-bit windows and a nine-bit indexed top. It retains two point partitions and the existing width-13 batch size of 352. Ordinary bucket capacity doubles from 2,048 to 4,096; top magnitude bins grow from 64 to 512. Field arithmetic, borrowed queues, exception handling, indexed reduction, phi, merges, weighting and Horner recombination remain unchanged. The new child copies the original window/raw bodies, changing only its indexed-top guard and the relative module path needed by that child.

The rejected width-11/four-part, balanced 11/12-span, GLS4, independent-plane and G1-column experiments remain closed. This candidate adds no persistent table, transformed key, duplicate solver plan, worker pool, fixed RAM cap or field representation. A checked reservation covers the complete candidate layout using existing typed parent/top/leaf formulas before in-place decomposition. Denial preserves scalar words. Original forecasts remain charged too; this conservative admission is not a process-memory bound. Every measured ON B query records actual candidate entry. Zero overlay-payload metadata does not imply zero scratch.

Ryzen 9 9950X, 16 physical cores / 32 logical CPUs, shared host. The retained frozen original caller has the corrected optimized profile (`0ba4cd4e…`); fresh OFF and ON share one executable. Final driver arguments include `-O`, native CPU, fat LTO and one codegen unit. Exact driver/perf source and four independent Arkworks complete-MSM references are reused. Workers have 512 KiB stacks and individual affinity: CPU 2 solo, CPUs 0–15 at 16 workers, or 0–31 at 32. The coordinator is constrained to the same set. Timing includes grouping, B scalar reset, decomposition, all fills/reductions/merges, normalization, allocation/drop and dispatch/wait. Loading, pool construction and oracle checks are excluded. All timed outputs, three primes and one post-check match the independent oracle; input points remain unchanged.

Fifteen fixed rotated/reversed blocks measure 20 complete queries per process. Primary B32 and the primary equal-log A/B32 aggregate must improve against both controls: rank-4–12 upper ratio below one and at least 10/15 wins. Every context upper ratio must be at most 1.03; every OFF/frozen interval must fit factor 1.03. These rank bounds have nominal 96.484375% marginal coverage conditional on independent exchangeable blocks, without a familywise guarantee. There is no gain floor, exclusion, extension, favorable subset, threshold change or parameter sweep. A is an unchanged control. Displayed medians need not divide into paired estimates.

|Context|Frozen ms|OFF ms|ON ms|ON/OFF [bounds],wins|ON/frozen [bounds],wins|OFF/frozen [bounds],wins|
|---|---:|---:|---:|---|---|---|
|1x2-A-w1|14.588219|14.671075|14.669090|1.000695620 [0.999368238, 1.003212140], 7/15|1.006159926 [1.002380126, 1.008917745], 1/15|1.004773343 [1.002967140, 1.007530910], 0/15|
|1x2-A-w16|1.535964|1.532880|1.533763|0.997917679 [0.988562312, 1.005988917], 9/15|1.000365051 [0.985261745, 1.011543419], 7/15|1.000436694 [0.990856546, 1.011266039], 7/15|
|1x2-A-w32|1.211376|1.206471|1.211203|0.996734268 [0.984808730, 1.020973187], 9/15|0.993775026 [0.963762198, 1.030330235], 9/15|0.998060340 [0.982626983, 1.005747612], 9/15|
|1x2-B-w1|37.352115|38.971471|62.202521|1.594099694 [1.591285341, 1.596513537], 0/15|1.666568728 [1.660111543, 1.668842751], 0/15|1.045190554 [1.043805524, 1.046893538], 0/15|
|1x2-B-w16|3.667238|3.683652|5.361402|1.458193289 [1.440216098, 1.469712787], 0/15|1.451982012 [1.440540590, 1.486956691], 0/15|1.003538545 [0.988359887, 1.008138684], 6/15|
|1x2-B-w32|3.314487|3.287470|4.492732|1.365277554 [1.356242657, 1.380966286], 0/15|1.362657056 [1.347701272, 1.383632946], 0/15|0.991668116 [0.976467528, 1.021926485], 9/15|
|2x2-A-w1|14.590787|14.503265|14.494082|0.999809841 [0.997138534, 1.002913092], 8/15|0.993623062 [0.990623117, 0.998128356], 13/15|0.994646178 [0.991501372, 0.996474440], 14/15|
|2x2-A-w16|1.881469|1.877524|1.881402|1.002721709 [0.993886332, 1.028040928], 6/15|0.997577444 [0.944291384, 1.024752850], 9/15|0.999779664 [0.979533279, 1.017430181], 8/15|
|2x2-A-w32|1.545961|1.510587|1.535232|1.004107715 [0.971591907, 1.035545227], 6/15|0.991164957 [0.972075061, 1.014391974], 10/15|0.983885428 [0.961922785, 1.005497917], 10/15|
|2x2-B-w1|39.940540|41.905484|59.159698|1.412190596 [1.410672912, 1.414044599], 0/15|1.482026920 [1.481194241, 1.484420301], 0/15|1.050142161 [1.048018546, 1.051313254], 0/15|
|2x2-B-w16|4.250299|4.294850|5.607272|1.301047137 [1.297176931, 1.316052340], 0/15|1.324154101 [1.307017425, 1.335893951], 0/15|1.009896146 [0.997863762, 1.026703384], 6/15|
|2x2-B-w32|3.888798|3.939632|4.751345|1.216104237 [1.188917670, 1.240185100], 0/15|1.228337433 [1.217402760, 1.249677004], 0/15|1.008030063 [0.997034421, 1.037143995], 4/15|

Primary equal-log A/B32: ON/OFF **1.172693344 [1.151834515, 1.186843136], 0/15**; ON/frozen **1.166533718 [1.134136043, 1.195675744], 0/15**. All12 calibrations pass: **false**. Failures are primaryBsolo,2×2Asmt,2×2Bsolo and2×2Bsmt; all remain in the declared gate. All six B contexts lose every block against both controls. The few favorable unchanged-A estimates are control variation, not algorithm gains.


|Context|Frozen/OFF/ON CPU ms/query|Frozen/OFF/ON peak process RSS MiB|
|---|---:|---:|
|1x2-A-w1|14.588/14.671/14.669|9.50/9.50/9.50|
|1x2-A-w16|17.004/16.961/16.961|18.50/18.75/18.75|
|1x2-A-w32|24.800/25.085/24.996|24.00/22.75/24.50|
|1x2-B-w1|37.351/38.971/62.201|15.50/15.50/15.75|
|1x2-B-w16|44.839/46.605/73.202|27.00/27.75/38.75|
|1x2-B-w32|70.958/71.071/109.443|32.25/35.25/50.00|
|2x2-A-w1|14.591/14.503/14.494|12.75/13.00/13.00|
|2x2-A-w16|17.632/17.611/17.629|22.50/22.50/24.25|
|2x2-A-w32|26.384/26.319/26.188|28.50/33.25/28.50|
|2x2-B-w1|39.940/41.904/59.158|27.25/27.25/27.25|
|2x2-B-w16|48.191/50.362/70.999|43.25/36.00/50.00|
|2x2-B-w32|76.536/77.811/105.790|60.25/60.50/76.25|

CPU is summed process service. RSS is lifetime high-water, including loading, primes and fixture arrays; it is not prover peak, incremental workspace or minimum RAM. Raw faults and context switches remain in the receipts and do not isolate a latency cause.

All latency precedes 12 FIFO-gated grouped counter panels and 12 separate fixed-16M-cycle profiles, each with 200 queries after three excluded primes. A prospective diagnostic amendment adds unchanged-A and frozen panels to the initial B-only proposal; it changes no latency check. All 600 callers precede the 60 ownership/decode/event/loss steps. No owned build, helper, hash, archive, analyzer or second benchmark overlaps capture.

|Panel|Cycles M/query|Instructions M/query|Cache misses M/query|Branch misses M/query|Samples|Qualified counter/profile|
|---|---:|---:|---:|---:|---:|---|
|stat-1x2-A-w16-candidate|85.554|207.352|0.213710|0.136892|1066|true/true|
|stat-1x2-A-w16-control|85.587|207.283|0.217429|0.136196|1063|true/true|
|stat-1x2-A-w16-frozen|85.526|207.397|0.210457|0.137978|1063|true/true|
|stat-1x2-A-w32-candidate|113.356|212.754|0.287308|0.162598|1415|true/true|
|stat-1x2-A-w32-control|113.497|212.896|0.283692|0.161613|1412|true/true|
|stat-1x2-A-w32-frozen|114.630|213.359|0.282631|0.164436|1424|true/true|
|stat-1x2-B-w16-candidate|370.127|855.915|1.217808|0.556663|4619|true/true|
|stat-1x2-B-w16-control|232.546|550.175|1.002094|0.270759|2897|true/true|
|stat-1x2-B-w16-frozen|223.104|549.324|0.748307|0.264358|2787|true/true|
|stat-1x2-B-w32-candidate|512.437|880.051|1.681690|0.615941|6412|true/true|
|stat-1x2-B-w32-control|325.337|567.891|1.154481|0.304875|4057|true/true|
|stat-1x2-B-w32-frozen|321.718|568.030|1.043670|0.311752|4012|true/true|

All32227samples are independently checked against executablePID/MMAP/FORK, registered workerTIDs/pinnedCPU and16Mperiod, with original event/loss receipts retained. Single panels and symbol/IP attribution do not establish exclusive stalls, local CPI, dynamic spills, an isolated cache/RAM cause or removable wall-time fractions. Absolute period coverage remains unvalidated. Static linked functions/disassembly are retained separately.


Nominal magnitude-bucket output slots per point partition grow from 10×2,048+64 = 20,544 to 9×4,096+512 = 37,376, an 81.93% increase. This is source geometry, not executed additions or traffic. The 32-worker counter panel grows from 567.891M to 880.051M retired instructions/query. Borrowed-prefix sampled share falls from 38.21% to 18.92%, reduction grows from 17.03% to 23.03%, and scalar inversion from 7.49% to 13.08%. These scoped shares include inlined work and do not assign a causal fraction. Fewer input passes did not reduce the measured total work. The exact recipe is closed; no further width/partition search or unchanged retry is selected.

All 12 profiles satisfy the declared minimum of 1,000 samples and maximum 5% unknown share, alongside independently reviewed ownership, period and loss checks. The original analyzer records individual checks without an aggregate qualification field; the offline [qualification receipt](artifacts/diagnostic-qualification.json) supplies their conjunction. Original analysis, reviewer and pinned publisher stay byte-exact. No diagnostic is recaptured and no latency check changes.

Native qualification passes all 128 core tests and both raw-image prover tests. Three new core tests cover complete generator sums at 1/4 workers, 512 KiB stacks, 1,024/1,025/4,097/8,193 points, scalar boundaries and identity/equal/opposite points. Independent 13-bit reconstruction covers every component bit boundary, 8,192 deterministic full-width component values, both signs and both components. Ordinary windows 0/4 and indexed top 9 match independent generator bucket sums. Field source is byte-exact and inherits prior qualification; portable, cubic and full-SDK behavior is not newly qualified.

The first native compile failed before tests because the copied child needed one more `super` qualifier. Both 177-file snapshots and the failure remain. The corrected snapshot changes only that child path; 172 originals remain exact, and all four parent edits reverse to the 176 accepted files. A seal-helper compile also needed a borrowed `PathBuf`; its initial source is retained. Repairs precede successful qualification and capture. Display-only float comparisons use relative 1e-12 from before capture; integer durations and gates are unchanged. Successful tests, builds and captures run once.

Evidence retains 665 successful workflow commands and the initial failed native compile. The 600 callers check 18,036 complete query outputs against reused independent references. No solve, oracle execution, SDK proof or Gnark export is added; the verified ledger remains 27,353. Independent review recomputes command exits/streams, all paired wall/CPU/RSS statistics, the complete gate, counters and every sampled symbol bin/thread binding. Archive verification checks exact bytes without rerunning a query.

The following accepted full-proof results and supplied M5 CPU/Metal references are inherited, not remeasured here.

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge 24→1|65.060937|507 / 7.793×|175 / 2.690×|50.7|

The all-eight 10× CPU goal remains unmet: **0/8**. Primary remains 7.928660 ms / 132.75 MiB; its target is 3.7 ms. Merge54 remains 161.883899 ms / 1,876.9609375 MiB, without a supplied M5 reference. Historical source-175 c7a primary results are Zopro 11.915591 ms at 32 workers versus Aeglos 7.959184 ms at 96; these predate source 176 and are not refreshed here. The accepted physical-16 calibration exception remains documented. No new AWS resources or AWS/Aeglos, mobile/ARM/TEE, huge-circuit, pressure/swap, minimum-RAM or concurrent-throughput qualification follows.

[Design](artifacts/design.md), [capture protocol](artifacts/capture-design.md), [all observations](artifacts/analysis.json), [independent review](artifacts/review.json), [source audit](artifacts/source-audit-fixed.json), [identities](artifacts/identities.json), [symbols](artifacts/symbols.json), [linked code](artifacts/codegen.json). Archives: [both source snapshots](artifacts/source.tar.gz), [workflow/helper sources](artifacts/workflow.tar.gz), [all raw command receipts](artifacts/receipts.tar.gz), [linked disassembly](artifacts/codegen.tar.gz). [Checksums](SHA256SUMS). Raw fixture/scalar/unblinded-point inputs, executables/libraries and perf.data stay local with hashes. Local root:`/home/ubuntu/src/zopro-perf-work/g2-wide-top176`.

A [preparation supplement](artifacts/preparation-supplement.tar.gz) retains the successful capture-preparation log, an earlier rejected `capture setup` invocation before any plan/caller, the seal-helper borrow fix and post-capture editorial/qualification source. All 660 capture workflow steps and 600 callers complete once. No qualified executable or measurement changes after capture.
