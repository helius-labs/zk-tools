# Weighted double-add: complete SDK results

**No confirmed full-proof gain; the candidate is not adopted.** Primary 1×2 at 32 workers is 7.824952 ms, versus fresh OFF 7.848311 ms and accepted REF 7.843523 ms. Paired ON/OFF is 1.000061 [0.993927,1.008862], 18/31 wins; ON/REF 1.002458 [0.996015,1.011527], 20/31 wins. Neither interval establishes improvement. The full declared gate passes 67/79 checks.

ALL8 equal-log ON/OFF is 1.006867 [1.003161,1.010860]; ON/REF 1.007210 [1.002843,1.011660]. These aggregates are descriptively slower, but the 4×2 OFF/REF calibration upper bound 1.032599 exceeds the declared 1.03 limit. Controls therefore are not fully qualified, and these data do not establish a clean universal slowdown estimate. All observations remain included.

The candidate uses G1-only weighted reduction for resource-admitted inputs with at least 2048 buckets. It reuses one 511-slot tree and the existing scratch per outer job, processes up to eight 256-bucket groups sequentially, collects nine affine correction roots per group and performs eight batched double-add steps. Both weighted and unweighted partials and original outer offset corrections are preserved. Identity/equal-X/zero-denominator cases retain complete projective fallback. G2, short inputs and disabled optional paths retain the original reduction.

The existing workspace forecast charges 6176 additional logical bytes per G1 leaf. This is not an emitted stack or RSS bound. There is no retained key table, execution plan, digit matrix, new pool or RAM cap. An ordinary eight-lane double-add step takes 15 vector point-products, two scalar root-products and two scalar inversions; a point-product reduction alone does not predict proof latency or parallel task supply.

This trial ports the previously qualified but unmeasured implementation to the current 176-source accepted build, preserving dense_busy_reset and all other accepted cfgs. The three arithmetic/kernel/test children match the earlier qualified source byte for byte. Current integration passes 130 unfiltered native core tests, including 175 inherited weighted/sum cases checked against independent Arkworks oracles, exceptional/sparse/tail cases,53 actual workers across 1/4/16/32 pools with requested 512 KiB stacks, and 12 concurrent calls sharing an 8-worker pool. Portable-disabled compilation passes; it is not ARM execution. All 13 native/build/link commands pass first execution. These correctness checks are completed once before timing.

Ryzen 9950X,16 physical/32 logical CPUs, shared host. ALL8 plus merge 54 run at 32 workers; primary also runs at 1/16. Workers pin to CPUi, soloCPU2. SDK worker stacks retain defaults. Full solve, randomized proving, internal verification and serialization are timed; loading, pool startup, three primes and final export are excluded.

| Context | REF ms | OFF ms | ON ms | Paired ON/OFF [95% bounds], wins |
|---|---:|---:|---:|---|
|1x2-w32|7.843523|7.848311|7.824952|1.000061 [0.993927,1.008862], 18/31|
|2x2-w32|10.999178|10.983944|11.078033|1.009379 [1.000097,1.021337], 7/15|
|4x2-w32|15.904657|15.851197|15.999054|1.004609 [0.993664,1.015349], 5/15|
|8x2-w32|24.339150|24.197788|24.733548|1.023799 [1.008801,1.042604], 4/15|
|12x2-w32|34.042084|34.013417|33.864397|1.001691 [0.990421,1.016587], 10/15|
|16x2-w32|40.074491|40.094550|40.249401|1.007863 [0.996873,1.019141], 6/15|
|24x2-w32|66.240170|66.540223|66.745529|1.004576 [0.998969,1.010778], 5/15|
|merge-24x1-w32|65.627663|65.627346|65.656835|1.003148 [0.995992,1.010953], 6/15|
|merge-54x1-w32|163.098343|162.882616|163.150466|1.000195 [0.995929,1.004244], 7/15|
|1x2-w1|94.628742|94.639782|94.149876|1.004364 [0.988195,1.032038], 13/15|
|1x2-w16|9.200161|9.195758|9.161138|1.001297 [0.993041,1.015100], 27/31|

Times are medians of arithmetic process means from integer-rounded SDK nanoseconds. Ratios need not equal quotients of the displayed medians. Positive medians are retained; none rescues the failed complete recipe. Merge 54 is essentially parity against both controls, with intervals crossing 1.

The failed checks are listed in full:

- `1x2-w1/ON/OFF/nonregression`
- `1x2-w1/ON/REF/nonregression`
- `1x2-w32/ON/OFF/primary-gain`
- `1x2-w32/ON/OFF/primary-wins`
- `1x2-w32/ON/REF/primary-gain`
- `1x2-w32/ON/REF/primary-wins`
- `4x2-w32/OFF/REF/calibration`
- `4x2-w32/ON/REF/nonregression`
- `8x2-w32/ON/OFF/nonregression`
- `8x2-w32/ON/REF/nonregression`
- `ALL8/ON/OFF/gain`
- `ALL8/ON/REF/gain`

Solo ON/control nonregression upper bounds are approximately 1.032; 8×2 ON/OFF is 1.023799 [1.008801,1.042604], and ON/REF 1.018677 [1.004884,1.033153]. All 22 RSS guards and all 18 diagnostic-quality checks pass. No threshold is relaxed, block excluded, sample added, favorable subset selected or unchanged capture repeated. There is no minimum positive-gain floor.

The prospective protocol uses 31 matched blocks for primary 16/32 and 15 otherwise,151 requests per primary 16/32 process,11 solo and 31 otherwise, with three primes. Context and arm order rotate/reverse by fixed block. The 20,000 fixed-seed paired-log bootstrap and stratified equal-shape aggregate are inherited unchanged. Block exchangeability and stationarity are not guaranteed; intervals have no familywise coverage guarantee.

Each arm has a predetermined independently written complete-ELF backing per latency block, with separate preflight and diagnostic files: 114 copies total. Equal-length names, modes, full hashes, sizes, distinct dev/inodes, single links and source immutability are checked before/after. Copies are benchmark disk fixtures. This does not establish physical-page coloring, remove all host drift or select a preferred production backing.

| Context | REF / OFF / ON peak RSS MiB | REF / OFF / ON CPU ms per proof | REF / OFF / ON minor faults per proof |
|---|---:|---:|---:|
|1x2-w32|132.500 / 134.500 / 134.000|178.022 / 178.469 / 177.221|127.424 / 127.808 / 128.702|
|2x2-w32|274.500 / 271.000 / 276.500|259.804 / 259.100 / 253.695|1295.806 / 1286.323 / 1281.097|
|4x2-w32|309.000 / 315.500 / 309.250|425.725 / 422.951 / 416.021|1144.935 / 1125.484 / 1124.645|
|8x2-w32|498.750 / 511.000 / 500.000|682.186 / 679.196 / 685.665|1654.677 / 1676.290 / 1655.645|
|12x2-w32|710.250 / 716.750 / 700.500|961.884 / 956.187 / 947.838|2213.677 / 2319.677 / 2229.032|
|16x2-w32|818.250 / 819.000 / 813.750|1140.757 / 1140.670 / 1139.883|2441.710 / 2518.613 / 2432.387|
|24x2-w32|1295.605 / 1321.250 / 1289.527|1932.131 / 1938.221 / 1931.499|5172.677 / 5446.935 / 5634.097|
|merge-24x1-w32|1158.570 / 1179.258 / 1161.523|1855.451 / 1855.705 / 1845.105|3357.839 / 3690.290 / 3418.194|
|merge-54x1-w32|1905.277 / 1928.949 / 1951.352|4543.693 / 4531.243 / 4524.132|22097.613 / 22090.742 / 22315.903|
|1x2-w1|34.500 / 34.500 / 34.250|94.626 / 94.628 / 94.148|1.909 / 1.909 / 0.000|
|1x2-w16|89.250 / 89.750 / 87.250|117.319 / 117.564 / 116.612|56.185 / 55.815 / 57.695|

RSS is the process lifetime peak including loading, primes and retained buffers; it is not minimum RAM. CPU/fault counts cover the timed batch and are scaled per request. Every timed/diagnostic batch has zero major faults. Minor faults do not measure their service time. No pressure, swap, phone or stack bound follows.

Single diagnostic panels show a small instruction/cycle reduction against OFF:

| Context | ON/OFF instructions | ON/OFF cycles | ON/OFF branch misses | ON/OFF generic cache misses |
|---|---:|---:|---:|---:|
|1x2-w32|0.994734|0.988446|0.994380|0.974013|
|1x2-w16|0.993891|0.984973|1.006869|0.945175|
|2x2-w32|0.994063|0.976966|1.003186|1.008801|

These panels are descriptive, not replicated causal attribution or a confirmed proof-latency gain. All PMU groups have at least 99.9% running coverage. Nine fixed 16M-user-cycle profiles contain 119,150 samples, at most 2.251% unknown, zero LOST and checked executable MMAP/PID/TID/CPU assignment. Weighted-double namespaces appear only in ON: 61 samples at primary 16,67 at primary 32 and 73 at 2×2-32, confirming execution. Namespace/flat samples can include inlined payload; they are not exclusive critical-path or removable wall-time fractions. Absolute sampling-period coverage is unvalidated. All nine raw profiles are archived.

All 1329 serialized capture-workflow commands succeed once: 33 functional SDK preflights and 33 independent Gnark checks,591 latency processes,18 diagnostic processes,45 offline operations and 609 capture-proof verifications. There are 44,238 internally checked SDK requests including primes and 642 fresh unique 164-byte Gnark-verified exports, bound to images and public inputs. Checked freshness advances the canonical inventory 48,079→48,112→48,721. The full ledger remains private and hash-bound.

Both actual readers,396 capture pins, all gates, inputs, copies and command order are frozen before the first SDK preflight. The separate same-agent reader reconstructs 591 raw means,79 decisions,609 capture bindings, preflight bindings,176-source recovery, backings and diagnostics. Shared statistical/diagnostic parsers are disclosed; this is not an external audit. No owned helper/build/hash/analysis/report overlaps preflight or recording. All recordings finish before offline operations. No successful native suite or cryptographic check is replayed for bookkeeping.

Preparation exception: the first adaptation-helper compile fails with Rust E0369 from adding a String to a string reference. The original source is retained, the expression is repaired before any helper execution, and subsequent build/prepare succeeds. Candidate mathematics, SDKs, gates and data are unchanged. No successful benchmark or verification is repeated.

Fresh accepted REF and unadopted ON compared with the supplied historical M5 references follow. M5 CPU/Metal were not remeasured; cross-hardware ratios do not establish equal efficiency.

| Shape | REF / ON ms | M5 CPU ms / REF speedup / ON speedup | M5 Metal ms / REF speedup / ON speedup | CPU10× limit ms |
|---|---:|---:|---:|---:|
|1x2|7.843523 / 7.824952|37 / 4.717× / 4.728×|24 / 3.060× / 3.067×|3.7|
|2x2|10.999178 / 11.078033|59 / 5.364× / 5.326×|30 / 2.727× / 2.708×|5.9|
|4x2|15.904657 / 15.999054|109 / 6.853× / 6.813×|48 / 3.018× / 3.000×|10.9|
|8x2|24.339150 / 24.733548|189 / 7.765× / 7.641×|74 / 3.040× / 2.992×|18.9|
|12x2|34.042084 / 33.864397|284 / 8.343× / 8.386×|101 / 2.967× / 2.982×|28.4|
|16x2|40.074491 / 40.249401|335 / 8.359× / 8.323×|124 / 3.094× / 3.081×|33.5|
|24x2|66.240170 / 66.745529|528 / 7.971× / 7.911×|176 / 2.657× / 2.637×|52.8|
|merge-24x1|65.627663 / 65.656835|507 / 7.725× / 7.722×|175 / 2.667× / 2.665×|50.7|

The accepted production source and SDK remain unchanged. The full CPU10× goal is active and unmet,0/8. Merge 54 has no supplied M5 reference. No new AWS/Aeglos, ARM/mobile/TEE,250/4999, minimum-RAM, pressure/swap or concurrent-proof-throughput claim follows. No cloud resource is created. Further work needs a distinct material reduction in complete prover work, accounting for roots/inversions, batching and task supply; the exact measured recipe is closed.

[Analysis](artifacts/analysis.json), [raw-reader review](artifacts/review-result.json), [decision](artifacts/decision.json), [diagnostic summary](artifacts/diagnostic-summary.json), [SDK identities](artifacts/identities.json), [copy identities](artifacts/copies.json), [complete evidence](artifacts/evidence.tar.gz), [member hashes](artifacts/public-members.json), [checksums](SHA256SUMS). Evidence includes all capture/native/link receipts, raw profiles/decodes, plans, pins, authored source and next-agent autoresearch prompt. Native executables/libraries, keys, requests, witnesses, proofs and the full ledger stay local and hash-bound.
