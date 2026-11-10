# Parallel weighted roots: complete SDK results

**The candidate is slower and is not adopted.** Primary 1×2 at 32 workers is **9.356124 ms**, versus fresh OFF **7.835357 ms** and accepted REF **7.827060 ms**. Paired ON/OFF is **1.195278 [1.192883,1.197708]**, and ON/REF **1.196833 [1.194011,1.199478]**, with **0/31 wins** against either. Primary OFF/REF calibration passes; this primary regression is not explained away by an invalid primary control.

The declared gate passes **58/79 checks**. Ten of eleven calibrations, eight of twenty-two nonregression checks, all twenty-two RSS guards and all eighteen diagnostic-quality checks pass. All primary gain/win and both aggregate gain checks fail. The 8×2 control calibration fails, so the descriptively slower ALL8 aggregates are not a clean universal slowdown estimate: ON/OFF 1.097632 [1.090109,1.102959], ON/REF 1.101092 [1.098432,1.103692]. All blocks remain included.

The [qualified implementation](../2026-11-10-weighted-root-phases 176/README.md) builds one set of nine roots per 256-bucket group in parallel, then performs a second parallel pass folding eight groups with unchanged double-add arithmetic. Both weighted and unweighted group outputs and original offset corrections are preserved. It exposes more root work items than the preceding grouped-root recipe, but adds a separately admitted temporary root array, initialization/stores/loads, a barrier and drop. The existing policy checks current OS headroom before allocating the array; the full timer charges that cost. More work items do not guarantee more simultaneous useful work.

The root forecast is 576 bytes per group plus 64 bytes, with the previous 6176 logical bytes per leaf retained for fixed fold objects. Denial, overflow, disabled policy, short inputs and G2 preserve the original path. No retained key table, expanded plan, new pool or RAM cap is introduced. Native 131-test/8-build-link qualification is inherited without replay. This trial reuses the exact qualified REF/OFF/ON SDKs and original field 1200, with all accepted cfgs including dense busy reset.

Ryzen 9950X,16 physical/32 logical CPUs, shared host. ALL8 plus merge 54 run at 32 workers; primary also at 1/16. Worker i pins to CPUi, soloCPU2, with SDK default worker stacks. Timers include solve, randomized proving, internal verification and serialization; loading, pool startup, three primes and final proof export are excluded.

| Context | REF ms | OFF ms | ON ms | Paired ON/OFF [95% bounds], wins |
|---|---:|---:|---:|---|
|1x2-w32|7.827060|7.835357|9.356124|1.195278 [1.192883,1.197708], 0/31|
|2x2-w32|10.928922|10.933791|13.980825|1.277655 [1.270523,1.284173], 0/15|
|4x2-w32|15.793934|15.787542|18.110278|1.146940 [1.139929,1.153475], 0/15|
|8x2-w32|23.996602|24.026653|26.271916|1.069068 [1.016922,1.102026], 1/15|
|12x2-w32|34.000511|33.704621|35.223562|1.043003 [1.032108,1.052603], 1/15|
|16x2-w32|39.766683|39.747965|41.431370|1.034955 [1.023779,1.044353], 1/15|
|24x2-w32|66.285987|66.107021|67.652122|1.022000 [1.017645,1.026000], 0/15|
|merge-24x1-w32|65.442208|65.706528|66.934161|1.019925 [1.012408,1.027617], 0/15|
|merge-54x1-w32|163.340201|163.499673|164.279594|1.006856 [1.001435,1.012447], 4/15|
|1x2-w1|94.464617|94.182449|96.610283|1.022694 [1.019065,1.025737], 0/15|
|1x2-w16|9.170142|9.204555|9.733818|1.058272 [1.055601,1.060957], 0/31|

Times are medians of arithmetic process means from integer-rounded SDK nanoseconds; paired ratios need not equal quotients of displayed medians. The fixed recipe is closed without sample exclusion, extension, threshold relaxation, subset adoption or unchanged recapture. There is no minimum positive-gain floor. Failed checks:

- `12x2-w32/ON/OFF/nonregression`
- `12x2-w32/ON/REF/nonregression`
- `16x2-w32/ON/OFF/nonregression`
- `16x2-w32/ON/REF/nonregression`
- `1x2-w16/ON/OFF/nonregression`
- `1x2-w16/ON/REF/nonregression`
- `1x2-w32/ON/OFF/nonregression`
- `1x2-w32/ON/OFF/primary-gain`
- `1x2-w32/ON/OFF/primary-wins`
- `1x2-w32/ON/REF/nonregression`
- `1x2-w32/ON/REF/primary-gain`
- `1x2-w32/ON/REF/primary-wins`
- `2x2-w32/ON/OFF/nonregression`
- `2x2-w32/ON/REF/nonregression`
- `4x2-w32/ON/OFF/nonregression`
- `4x2-w32/ON/REF/nonregression`
- `8x2-w32/OFF/REF/calibration`
- `8x2-w32/ON/OFF/nonregression`
- `8x2-w32/ON/REF/nonregression`
- `ALL8/ON/OFF/gain`
- `ALL8/ON/REF/gain`

The frozen protocol uses 31 matched primary 16/32 blocks×151 requests,15 blocks otherwise×31 requests except solo 11, and three primes. Context/arm order rotates and reverses by block. There are 114 predetermined independently written complete-ELF backings, with equal-length names, modes, full hashes, sizes, distinct dev/inodes, single links and source immutability checked before/after. No backing is selected after timing. This does not establish physical-page coloring or remove all host drift. The inherited 20,000 fixed-seed paired-log bootstrap and stratified equal-shape aggregate have no familywise coverage guarantee; stationarity and block exchangeability are not guaranteed.

| Context | REF / OFF / ON peak RSS MiB | REF / OFF / ON CPU ms per proof | REF / OFF / ON minor faults per proof |
|---|---:|---:|---:|
|1x2-w32|133.250 / 135.000 / 132.000|178.550 / 178.422 / 179.342|130.272 / 128.536 / 126.563|
|2x2-w32|276.000 / 273.250 / 273.250|257.745 / 257.972 / 273.364|1301.419 / 1340.000 / 1296.032|
|4x2-w32|322.250 / 309.500 / 315.250|421.818 / 421.552 / 435.963|1150.968 / 1149.645 / 1137.484|
|8x2-w32|502.500 / 524.750 / 507.750|673.656 / 672.780 / 688.292|1678.194 / 1700.742 / 1731.323|
|12x2-w32|710.250 / 718.250 / 707.000|953.843 / 947.970 / 944.229|2305.968 / 2348.452 / 2327.032|
|16x2-w32|840.500 / 824.500 / 809.500|1132.410 / 1132.774 / 1136.329|2511.065 / 2463.065 / 2411.548|
|24x2-w32|1301.000 / 1283.367 / 1293.500|1929.441 / 1930.859 / 1934.582|6122.161 / 6211.968 / 5549.613|
|merge-24x1-w32|1168.500 / 1158.961 / 1152.500|1850.109 / 1858.013 / 1852.406|3414.710 / 3562.194 / 3834.355|
|merge-54x1-w32|1921.910 / 1948.734 / 1942.926|4547.202 / 4546.514 / 4551.025|21970.452 / 22166.968 / 21713.226|
|1x2-w1|34.250 / 34.250 / 34.250|94.462 / 94.180 / 96.601|0.000 / 0.000 / 0.000|
|1x2-w16|89.500 / 88.750 / 88.250|117.413 / 117.690 / 119.625|56.258 / 56.285 / 56.821|

All ON/control maximum-RSS guards pass at factor 1.05. RSS is process lifetime peak including loading, primes and retained buffers, not minimum RAM. CPU/fault counts cover the timed batch and are scaled per request. All batches have zero major faults. Minor faults do not measure their service time or establish pressure/swap behavior.

The existing batch rusage records expose a new useful distinction:

| Context / arm | Median user CPU ms/proof | Median system CPU ms/proof | Median voluntary switches/proof |
|---|---:|---:|---:|
|1x2-w1/REF|94.386273|0.182000|3.727273|
|1x2-w1/OFF|94.174364|0.091636|3.727273|
|1x2-w1/ON|94.519636|2.091182|3.636364|
|1x2-w16/REF|115.272364|2.023841|250.933775|
|1x2-w16/OFF|115.559675|1.978934|252.125828|
|1x2-w16/ON|115.527159|4.083066|287.920530|
|1x2-w32/REF|173.720603|4.699530|358.523179|
|1x2-w32/OFF|174.030596|4.627477|357.496689|
|1x2-w32/ON|170.920205|8.411172|532.238411|

These are marginal medians; their sums need not equal median total CPU. Primary 32 system CPU rises OFF 4.627477→ON 8.411172 ms/proof, while user CPU falls 174.030596→170.920205. Voluntary switches rise 357.50→532.24. They are not syscall counts, futex counts or per-function costs. Source inspection shows `Reservation::analysis` holds the admission mutex while querying current headroom/limits. That is a plausible serialized cost, but root allocation, the extra barrier and task distribution also change. These records do not isolate causality or justify removing memory safeguards.

| Single panel | ON/OFF instructions | ON/OFF cycles | ON/OFF branch misses | ON/OFF generic cache misses |
|---|---:|---:|---:|---:|
|1x2-w32|1.016650|1.015778|1.105238|1.058168|
|1x2-w16|1.001455|1.002636|1.062968|1.022689|
|2x2-w32|1.032886|1.040218|1.151822|1.034668|

These are descriptive single panels, not replicated attribution. All 9 PMU panels have at least 99.9% running coverage. Nine fixed 16M-user-cycle profiles contain **119925 samples**, maximum unknown **2.087%**, zero LOST and checked executable MMAP/PID/TID/CPU assignment. Weighted namespaces appear only in ON: 63 primary 16,63 primary 32 and 84 at 2×2-32, confirming execution. Flat symbols can contain inlined payload and do not measure exclusive critical-path or removable wall time. Absolute sampling-period coverage is unvalidated. All nine raw profiles and decodes are archived.

All **1329 serialized commands succeed once**: 33 functional SDK preflights plus 33 independent Gnark checks,591 latency processes,18 diagnostic processes,45 offline profile operations and 609 capture-proof verifications. There are **44,238 internally checked SDK requests including primes** and **642 fresh unique 164-byte Gnark-verified exports**, bound to images/public inputs. Checked bindings and freshness advance the canonical inventory **48,721→48,754→49,363**. No successful native suite, arithmetic model, recording or crypto check is repeated for bookkeeping.

Both actual readers, all 79 gates,398 pins, order, input and backing choices are frozen before any functional preflight. The separate same-agent reader reconstructs 591 raw means,79 decisions,609 capture exports plus preflight bindings,176-source recovery, backings and diagnostics. Statistical/diagnostic parsers are shared; this is not an external audit. No owned helper/build/hash/analysis/report overlaps preflight or recording. All recordings finish before offline work. New-stage preparation, capture, analysis and review pass first execution; earlier qualification tooling exceptions remain disclosed in its linked report.

Fresh accepted REF and unadopted ON compared with supplied historical M5 CPU/Metal follow. Different hardware is involved; M5 was not remeasured, and ratios do not establish equal efficiency.

| Shape | REF / ON ms | M5 CPU ms / REF speedup / ON speedup | M5 Metal ms / REF speedup / ON speedup | CPU10× limit ms |
|---|---:|---:|---:|---:|
|1x2|7.827060 / 9.356124|37 / 4.727× / 3.955×|24 / 3.066× / 2.565×|3.7|
|2x2|10.928922 / 13.980825|59 / 5.399× / 4.220×|30 / 2.745× / 2.146×|5.9|
|4x2|15.793934 / 18.110278|109 / 6.901× / 6.019×|48 / 3.039× / 2.650×|10.9|
|8x2|23.996602 / 26.271916|189 / 7.876× / 7.194×|74 / 3.084× / 2.817×|18.9|
|12x2|34.000511 / 35.223562|284 / 8.353× / 8.063×|101 / 2.971× / 2.867×|28.4|
|16x2|39.766683 / 41.431370|335 / 8.424× / 8.086×|124 / 3.118× / 2.993×|33.5|
|24x2|66.285987 / 67.652122|528 / 7.965× / 7.805×|176 / 2.655× / 2.602×|52.8|
|merge-24x1|65.442208 / 66.934161|507 / 7.747× / 7.575×|175 / 2.674× / 2.615×|50.7|

The accepted production source and SDK remain unchanged. The full CPU AVX10× ALL8 goal remains active and unmet, **0/8**. Merge 54 has no supplied M5 reference. No AWS/Aeglos, ARM/mobile/TEE,250/4999, minimum-RAM, pressure/swap or concurrent-proof-throughput claim follows; no cloud resource is created.

A distinct follow-up is to audit whether consumed temporary bucket vectors can hold root prefixes in place. That would change the ownership/storage contract and could remove the new root allocation and its per-entry admission while retaining memory safeguards. It is unimplemented and unqualified; it is not a rerun with checks disabled or a promised speedup. Complete source/capacity/fallback/native qualification and a new prospective full-proof comparison would be required.

[Analysis](artifacts/analysis.json), [raw-reader review](artifacts/review-result.json), [decision](artifacts/decision.json), [diagnostics](artifacts/diagnostic-summary.json), [SDK identities](artifacts/identities.json), [copy identities](artifacts/copies.json), [complete evidence](artifacts/evidence.tar.gz), [member hashes](artifacts/public-members.json), [checksums](SHA256SUMS). Evidence includes all native/link/capture receipts, raw profiles/decodes, plans/pins, authored source and the next-agent autoresearch prompt. Native executables/libraries, keys, requests, witnesses, proofs and full ledgers remain private and hash-bound.
