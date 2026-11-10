# SIMD Booth decoding: complete SDK results

**The candidate is slower and is not adopted.** Primary 1×2 at 32 workers measures **7.898484 ms**, versus **7.817748 ms fresh OFF** and **7.811658 ms accepted REF**. Paired ON/OFF is **1.014533**, 95% bootstrap interval **1.011876–1.017205**, with **1/31 wins**. ON/REF is **1.015100** [1.012449,1.017697], with **0/31 wins**. All declared control-calibration checks pass.

The [qualified native decoder](../2026-11-10-simd-booth176/README.md) reads eight independent Booth digits into a 16-byte stack array, then retains original scalar bucket dispatch and queue/batch boundaries. It adds no heap digit matrix, key table, retained plan, pool or resource-policy change. Field arithmetic, point arrivals and window geometry stay original. The native correctness suites are inherited without replay. This experiment charges the complete decoder, iterator, gathers, dispatch and tail through the actual SDK.

Ryzen 9950X, 16 physical/32 logical CPUs, shared host. All eight supplied shapes plus merge54 run at 32 workers; primary also runs at 1/16. Workers pin to CPUi, solo CPU2; SDK worker stacks retain defaults. Full solve, randomized proving, internal verification and serialization are timed. Loading, pool startup, three primes and final proof export are excluded.

| Context | REF ms | OFF ms | ON ms | Paired ON/OFF [95% bounds], wins |
|---|---:|---:|---:|---|
|1x2-w32|7.811658|7.817748|7.898484|1.014533 [1.011876,1.017205], 1/31|
|2x2-w32|10.967528|10.972307|11.085453|1.012921 [1.006666,1.019106], 2/15|
|4x2-w32|15.786866|15.739054|15.846228|1.003523 [0.995961,1.010118], 4/15|
|8x2-w32|23.926490|23.980937|24.048820|0.997948 [0.989011,1.007221], 8/15|
|12x2-w32|33.767079|33.861584|33.813532|1.001221 [0.993533,1.010563], 9/15|
|16x2-w32|39.718288|39.812343|39.831390|1.003968 [0.995834,1.012411], 6/15|
|24x2-w32|66.229409|66.080740|66.266344|1.001854 [0.997723,1.005879], 6/15|
|merge-24x1-w32|66.080238|65.862475|65.928465|1.003098 [0.998045,1.010013], 7/15|
|merge-54x1-w32|163.261357|163.726624|162.454621|0.994360 [0.989797,0.999746], 12/15|
|1x2-w1|94.298691|94.109117|95.761018|1.015373 [1.011646,1.019045], 0/15|
|1x2-w16|9.168599|9.179426|9.363156|1.017860 [1.016441,1.019332], 0/31|

Times are medians of arithmetic process means from integer-rounded SDK nanoseconds. Paired ratios need not equal quotients of displayed medians. ALL8 equal-log ON/OFF is **1.004869** [1.002473,1.007323]; ON/REF is **1.005784** [1.003601,1.007948]. Both aggregate intervals are above parity. Solo and 16-worker primary comparisons are also slower against OFF, with intervals above parity.

Merge54 retains a small positive ON/OFF estimate: 0.994360 [0.989797,0.999746], 12/15 wins. Against accepted REF it is 0.996112 [0.991798,1.000167], 9/15. That comparison does not confirm a gain against both controls, and the full recipe fails. There is no shape-subset adoption or unchanged rerun.

**73 of 79 declared checks pass.** All 11 OFF/REF calibration intervals fit factor 1.03; all 22 ON/control nonregression bounds, 22 RSS guards, nine PMU and nine profile-quality checks pass. The six failures are primary32 gain and winning-block requirements against both controls, plus both ALL8 gain checks. No minimum positive-gain floor is imposed.

The schedule has 31 matched blocks for primary16/32 and 15 for other contexts, with 151 requests per primary16/32 process, 11 solo and 31 otherwise. Context and arm order rotate/reverse by fixed block index. The inherited 20,000 fixed-seed paired-log bootstrap and stratified equal-shape aggregate are unchanged. No block is excluded, sample added, threshold relaxed or favorable backing selected. Independent/exchangeable-block and stationarity assumptions are not guaranteed; there is no familywise coverage guarantee.

To incorporate the prior [file-backing observation](../2026-11-10-backing-control176/README.md) prospectively, each arm uses a predetermined independently written complete-ELF file per latency block. Contexts within a block reuse that backing. Separate preflight and diagnostic backings make 114 copies, all retained. Equal-length arm names, identical modes, complete hashes, sizes, distinct dev/inodes and source immutability are checked before/after. These are disk benchmark fixtures, not prover lookup tables. This design does not identify physical-page coloring, guarantee removal of all drift or choose a faster copy for production.

Single diagnostic panels show a modest instruction reduction with higher cycles:

| Context | ON/OFF instructions | ON/OFF cycles | ON/OFF branch misses | ON/OFF generic cache misses |
|---|---:|---:|---:|---:|
|1x2-w32|0.993858|1.015341|1.104850|1.001758|
|1x2-w16|0.993445|1.026827|1.110918|1.003578|
|2x2-w32|0.996269|1.047188|1.095265|1.127277|

These are descriptive panels, not replicated attribution to a particular branch, gather, cache level or code-size change. Fewer retired instructions do not establish lower execution cost. All grouped events have at least 99.9% running coverage. Nine profiles with a fixed 16M user-cycle period contain **120,094 samples**, maximum unknown share **1.964%**, zero LOST and checked actual executable MMAP/PID/TID/CPU assignment. Absolute period coverage is unvalidated; flat samples can include inlined payload and are not exclusive critical-path or removable wall-time fractions. All nine raw profiles and decoded ownership/loss streams are included in the evidence archive.

| Context | REF / OFF / ON peak RSS MiB | REF / OFF / ON CPU ms per proof | REF / OFF / ON minor faults per proof |
|---|---:|---:|---:|
|1x2-w1|34.250 / 34.250 / 34.000|94.297 / 94.106 / 95.759|0.000 / 0.000 / 0.000|
|1x2-w16|89.000 / 88.500 / 87.500|117.249 / 117.224 / 119.786|56.444 / 56.675 / 56.874|
|1x2-w32|133.750 / 136.250 / 133.250|178.139 / 178.290 / 180.874|129.205 / 129.940 / 128.305|
|2x2-w32|267.500 / 272.500 / 278.000|257.764 / 257.461 / 262.070|1244.065 / 1295.548 / 1278.742|
|24x2-w32|1302.102 / 1271.000 / 1292.699|1928.634 / 1928.352 / 1928.407|6091.677 / 5632.581 / 6088.452|
|merge-54x1-w32|1965.828 / 1950.781 / 1937.766|4539.003 / 4551.518 / 4517.352|22318.000 / 22002.903 / 21838.419|

All comparative RSS guards pass: ON maximum≤1.05×control maximum. RSS is the process lifetime peak, including loading, primes and retained buffers; it is not minimum RAM or a stack bound. CPU/fault counters cover the timed batch interval and are scaled per request. Every timed/diagnostic batch reports zero major faults. Minor-fault counts do not measure service time; no pressure/swap or phone qualification follows.

All **1,329 serialized commands succeed once**: 33 functional SDK preflights and 33 independent Gnark checks, 591 latency processes, 18 diagnostic processes, 45 offline profile operations and 609 independent capture-proof verifications. There are **44,238 internally checked SDK requests** including primes and **642 fresh unique 164-byte Gnark-verified exports**, each bound to the image and public inputs. The canonical inventory advances **47,437→47,470→48,079** only after binding/freshness checks. No successful native suite, decoder, capture or cryptographic verification is repeated for bookkeeping.

The exact already-qualified OFF/ON SDKs and accepted REF are reused without rebuilding. Both raw readers, all gates/order/inputs and 402 pins are frozen before any SDK preflight. Primary analysis and the separate same-agent reader pass first execution; the latter reconstructs 591 raw means, all 79 decisions, all fresh export bindings, source recovery, backings and diagnostics. Shared qualified statistical/diagnostic parsers are disclosed; this is not an external audit. No owned build/helper/hash/analysis/report overlaps preflight or recording.

Preparation exceptions are retained. The first source check attempts UTF-8 decoding on a binary member of the accepted manifest after the copies are created but before any SDK request. It is corrected to hash unedited members as bytes and reverse only known text parents, preserving every existing copy. The first repair compile has an extra closing brace; the failed source/streams remain. Final helper builds succeed before freezing. Candidate binaries, protocol, gates and mathematical parsers remain unchanged.

Fresh accepted REF and unadopted ON timings compared with the supplied historical M5 CPU/Metal values follow. The M5 devices were not remeasured; cross-hardware ratios do not establish equal efficiency.

| Shape | REF / ON ms | M5 CPU ms / REF speedup / ON speedup | M5 Metal ms / REF speedup / ON speedup | CPU10× limit ms |
|---|---:|---:|---:|---:|
|1x2|7.811658 / 7.898484|37 / 4.737× / 4.684×|24 / 3.072× / 3.039×|3.7|
|2x2|10.967528 / 11.085453|59 / 5.380× / 5.322×|30 / 2.735× / 2.706×|5.9|
|4x2|15.786866 / 15.846228|109 / 6.904× / 6.879×|48 / 3.041× / 3.029×|10.9|
|8x2|23.926490 / 24.048820|189 / 7.899× / 7.859×|74 / 3.093× / 3.077×|18.9|
|12x2|33.767079 / 33.813532|284 / 8.411× / 8.399×|101 / 2.991× / 2.987×|28.4|
|16x2|39.718288 / 39.831390|335 / 8.434× / 8.410×|124 / 3.122× / 3.113×|33.5|
|24x2|66.229409 / 66.266344|528 / 7.972× / 7.968×|176 / 2.657× / 2.656×|52.8|
|merge-24x1|66.080238 / 65.928465|507 / 7.672× / 7.690×|175 / 2.648× / 2.654×|50.7|

The accepted production source/SDK remain unchanged. The full CPU10× goal remains active and unmet, **0/8**. Merge54 has no supplied M5 reference. No new AWS/Aeglos, ARM/mobile/TEE, huge 250/4999, minimum-RAM, pressure/swap or concurrent-proof-throughput claim follows; no cloud resource is created. This negative closes the exact SIMD decoder recipe. Further work needs a distinct material reduction in prover work, using existing profiles and prior experiments before new capture.

[Analysis](artifacts/analysis.json), [raw-reader review](artifacts/review-result.json), [decision](artifacts/decision.json), [diagnostic summary](artifacts/diagnostic-summary.json), [original SDK identities](artifacts/identities.json), [all copy identities](artifacts/copies.json), [complete evidence](artifacts/evidence.tar.gz), [archive member hashes](artifacts/public-members.json), [checksums](SHA256SUMS). The archive includes all raw SDK/verification receipts, all nine raw perf.data files, all decodes/mapping/loss streams, counters, plans/pins, authored readers, preparation failures and the next-agent autoresearch prompt. Private key/request/witness/proof payloads, full canonical ledger and native executables/libraries remain local and hash-bound.
