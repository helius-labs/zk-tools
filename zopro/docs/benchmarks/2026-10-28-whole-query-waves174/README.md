# Whole-query G1 then G2 phases

The candidate is **not adopted**. Primary32 is 6.078% slower than OFF and 6.999% slower than accepted, losing all seven paired blocks against both controls. Primary calibration passes. CPU time increases despite lower peak RSS; see the [interpretation](artifacts/interpretation.md). Physical16 and 16x2 OFF/accepted calibration bounds fail descriptively and remain in the report.

The private candidate completes A and fused K/H before B2, using the same full Rayon pool. The primary32 paired ON/OFF ratio is 1.060781652, with bounds [1.054621614, 1.083825746] and 0/7 wins. The prospective pilot gate is **false**; production is unchanged.

The accepted174 source is copied, with one changed branch in lib.rs and a private helper with ordering/panic tests (175 candidate files, 173 original files exact). Reversing the edit and removing the helper recovers all174 originals. OFF preserves the original three-query join expression. Arithmetic, field, grouping, key/solver, forecast and unsupported dispatch stay unchanged. There is no extra pool, lookup table, plan, delay, warmup allocation or new unsafe code.

Both OFF and ON pass124 unfiltered core and8 prover tests. Native dependencies, including proc macros, are frozen; actual core/field flags and features are audited. OFF/ON share the same candidate executable path and inode, private selector0/1; accept is the retained accepted SDK. Labels offset and bypass mean OFF and ON in the raw artifacts.

Seven fixed rotated/reversed blocks cover primary16/32,16x2/32 and merge54/32, with151 primary or31 other requests after3 primes. Key loading is excluded; solving, fresh proving, SDK verification and serialization are included. No owned compilation/helper/hash/archive/verifier/profile or second benchmark overlaps timing. This is a shared host.

| Cell | Variant | Median ms | CPU ms/request | Median peak MiB | Max peak MiB |
|---|---|---:|---:|---:|---:|
| 16x2-w32 | accept | 40.440256 | 1122.795 | 792.00 | 816.25 |
| 16x2-w32 | bypass | 40.271709 | 1122.356 | 794.50 | 829.25 |
| 16x2-w32 | offset | 40.144480 | 1120.735 | 808.75 | 824.50 |
| 1x2-w16 | accept | 9.943883 | 121.346 | 88.50 | 90.25 |
| 1x2-w16 | bypass | 10.282741 | 123.772 | 62.25 | 63.50 |
| 1x2-w16 | offset | 10.040426 | 121.468 | 87.00 | 88.75 |
| 1x2-w32 | accept | 8.138147 | 187.084 | 131.75 | 134.75 |
| 1x2-w32 | bypass | 8.714394 | 190.968 | 80.00 | 82.25 |
| 1x2-w32 | offset | 8.155049 | 187.818 | 130.50 | 133.25 |
| merge-54x1-w32 | accept | 162.140461 | 4423.654 | 1779.16 | 1847.13 |
| merge-54x1-w32 | bypass | 162.100030 | 4415.075 | 1777.69 | 1815.79 |
| merge-54x1-w32 | offset | 162.778499 | 4430.344 | 1787.93 | 1838.71 |

RSS covers the whole process including loading and priming; it is not minimum required RAM.

| Pair | Median ratio | Lower | Upper | Wins | Paired saving ms | Required check |
|---|---:|---:|---:|---:|---:|---|
| 16x2-w32/bypass/accept | 1.003301162 | 0.969537869 | 1.026193806 | 3/7 | -0.131710 | pass |
| 16x2-w32/bypass/offset | 1.003169278 | 0.993178346 | 1.011718441 | 1/7 | -0.127229 | pass |
| 16x2-w32/offset/accept | 0.997613400 | 0.966474842 | 1.022635625 | 4/7 | 0.095661 | descriptive |
| 1x2-w16/bypass/accept | 1.018755450 | 0.907982053 | 1.146240992 | 3/7 | -0.186502 | descriptive |
| 1x2-w16/bypass/offset | 0.999840345 | 0.889893020 | 1.085455509 | 4/7 | 0.001603 | descriptive |
| 1x2-w16/offset/accept | 1.014932546 | 0.882889548 | 1.162907959 | 3/7 | -0.145665 | descriptive |
| 1x2-w32/bypass/accept | 1.069986588 | 1.067483880 | 1.079545258 | 0/7 | -0.562170 | fail |
| 1x2-w32/bypass/offset | 1.060781652 | 1.054621614 | 1.083825746 | 0/7 | -0.498760 | fail |
| 1x2-w32/offset/accept | 1.007296339 | 0.994140894 | 1.022263357 | 3/7 | -0.059071 | pass |
| merge-54x1-w32/bypass/accept | 0.997945188 | 0.977947487 | 1.020369824 | 4/7 | 0.333771 | pass |
| merge-54x1-w32/bypass/offset | 1.000623309 | 0.967310162 | 1.004745005 | 3/7 | -0.100430 | pass |
| merge-54x1-w32/offset/accept | 1.004200648 | 0.993232295 | 1.018010907 | 3/7 | -0.681095 | descriptive |

Failed checks: `["1x2-w32/bypass/offset","1x2-w32/bypass/accept"]`. OFF/accepted calibration is separately recorded for all cells in analysis.json.

Bounds are the extrema of7 block ratios, nominal98.4375% under independent exchangeable blocks, without familywise guarantees. Primary32 requires upper<1 and at least5/7 wins against both controls plus OFF/accepted bounds withinfactor1.03. Larger ON comparisons require upper<=1.03; physical16 is descriptive. No gain floor, excluded observation, extension, successful-capture repeat or changed threshold. A passing pilot would select further all-eight, clean, portable and resource qualification.

After all latency and latency verification,12 FIFO-gated grouped counter captures and6 separate199Hz cycles profiles cover primary16/32 and all three variants. Each counter run has31 requests; each profile251, after3 primes. All diagnostic captures precede their verification/decoding. Diagnostic processes run as root through sudo for perf; latency processes run as the normal user. Priming/loading are excluded by the SDK FIFO control. All PMU events run at least99.9%; all profiles have one sampled PID and zero lost events. These diagnostic times do not enter the latency gate. A single counter/profile per cell does not establish a cache cause or confidence interval. Sampled CPU shares are not removable wall-time fractions.

| Counter process | Event | Count/request | Running % |
|---|---|---:|---:|
| fetch-1x2-w16-accept | L1-icache-load-misses | 585527.645 | 100.00 |
| fetch-1x2-w16-accept | cycles:u | 595488370.968 | 100.00 |
| fetch-1x2-w16-accept | iTLB-load-misses | 59.710 | 100.00 |
| fetch-1x2-w16-accept | instructions:u | 1413886301.935 | 100.00 |
| fetch-1x2-w16-bypass | L1-icache-load-misses | 554222.968 | 100.00 |
| fetch-1x2-w16-bypass | cycles:u | 602963799.839 | 100.00 |
| fetch-1x2-w16-bypass | iTLB-load-misses | 69.290 | 100.00 |
| fetch-1x2-w16-bypass | instructions:u | 1417645521.839 | 100.00 |
| fetch-1x2-w16-offset | L1-icache-load-misses | 542759.032 | 100.00 |
| fetch-1x2-w16-offset | cycles:u | 590851371.935 | 100.00 |
| fetch-1x2-w16-offset | iTLB-load-misses | 166.097 | 100.00 |
| fetch-1x2-w16-offset | instructions:u | 1412844452.387 | 100.00 |
| fetch-1x2-w32-accept | L1-icache-load-misses | 612614.097 | 100.00 |
| fetch-1x2-w32-accept | cycles:u | 857002915.581 | 100.00 |
| fetch-1x2-w32-accept | iTLB-load-misses | 1034.226 | 100.00 |
| fetch-1x2-w32-accept | instructions:u | 1454076876.903 | 100.00 |
| fetch-1x2-w32-bypass | L1-icache-load-misses | 613713.935 | 100.00 |
| fetch-1x2-w32-bypass | cycles:u | 868807348.839 | 100.00 |
| fetch-1x2-w32-bypass | iTLB-load-misses | 828.226 | 100.00 |
| fetch-1x2-w32-bypass | instructions:u | 1460757420.387 | 100.00 |
| fetch-1x2-w32-offset | L1-icache-load-misses | 627384.452 | 100.00 |
| fetch-1x2-w32-offset | cycles:u | 854972126.161 | 100.00 |
| fetch-1x2-w32-offset | iTLB-load-misses | 886.774 | 100.00 |
| fetch-1x2-w32-offset | instructions:u | 1453904413.452 | 100.00 |
| stall-1x2-w16-accept | L1-dcache-load-misses | 26590736.871 | 100.00 |
| stall-1x2-w16-accept | cycles:u | 590598286.290 | 100.00 |
| stall-1x2-w16-accept | instructions:u | 1412866817.419 | 100.00 |
| stall-1x2-w16-accept | stalled-cycles-frontend:u | 26585904.806 | 100.00 |
| stall-1x2-w16-bypass | L1-dcache-load-misses | 25977449.774 | 100.00 |
| stall-1x2-w16-bypass | cycles:u | 604912108.710 | 100.00 |
| stall-1x2-w16-bypass | instructions:u | 1418683559.323 | 100.00 |
| stall-1x2-w16-bypass | stalled-cycles-frontend:u | 27028031.710 | 100.00 |
| stall-1x2-w16-offset | L1-dcache-load-misses | 26442270.742 | 100.00 |
| stall-1x2-w16-offset | cycles:u | 592823151.258 | 100.00 |
| stall-1x2-w16-offset | instructions:u | 1412936690.903 | 100.00 |
| stall-1x2-w16-offset | stalled-cycles-frontend:u | 25766127.871 | 100.00 |
| stall-1x2-w32-accept | L1-dcache-load-misses | 34084847.290 | 100.00 |
| stall-1x2-w32-accept | cycles:u | 857187128.548 | 100.00 |
| stall-1x2-w32-accept | instructions:u | 1454719014.806 | 100.00 |
| stall-1x2-w32-accept | stalled-cycles-frontend:u | 59487366.645 | 100.00 |
| stall-1x2-w32-bypass | L1-dcache-load-misses | 34156682.065 | 100.00 |
| stall-1x2-w32-bypass | cycles:u | 872947810.226 | 100.00 |
| stall-1x2-w32-bypass | instructions:u | 1460818963.806 | 100.00 |
| stall-1x2-w32-bypass | stalled-cycles-frontend:u | 60219385.903 | 100.00 |
| stall-1x2-w32-offset | L1-dcache-load-misses | 33945335.419 | 100.00 |
| stall-1x2-w32-offset | cycles:u | 854274334.387 | 100.00 |
| stall-1x2-w32-offset | instructions:u | 1454594617.968 | 100.00 |
| stall-1x2-w32-offset | stalled-cycles-frontend:u | 59250413.419 | 100.00 |

| Profile | Samples | Sampled TIDs | Lost events |
|---|---:|---:|---:|
| cycles-1x2-w16-accept | 5385 | 17 | 0 |
| cycles-1x2-w16-bypass | 5546 | 17 | 0 |
| cycles-1x2-w16-offset | 5631 | 17 | 0 |
| cycles-1x2-w32-accept | 9160 | 33 | 0 |
| cycles-1x2-w32-bypass | 9132 | 33 | 0 |
| cycles-1x2-w32-offset | 9054 | 33 | 0 |

All12 preflight proofs independently verify before84 latency processes. Their84 exports verify afterward; the18 diagnostic exports verify after all diagnostic captures. Total9876 internally verified SDK requests,114 fresh164-byte Gnark exports,228 proof/public members, ledger15211→15325. Closing source, libraries, executables, inputs and helper pins match. Raw perf data stays local with SHA256 receipts; decoded data and complete candidate source are archived.

The prior [B2 grouping bypass](../2026-10-28-b2-group-only174/README.md) and [global grouping bypass](../2026-10-28-dedup-off174/README.md) remain closed negatives. This tests a whole-query completion barrier; it differs from earlier mixed joint passes and fixed worker partitions. Aeglos's inspected [pinned MSM source](https://github.com/helius-labs/aeglos/blob/0290613e8eeca17a4ce88f6f2d726e8e06a5385d/cpu/msm_impl.hpp#L388) shares passes across jobs; it does not use this G1-then-G2 ordering. There is no new Aeglos timing in this experiment.

New reference comparisons cover only1x2 and16x2. M5 figures are supplied historical references, not matched-machine measurements.

| Cell | Variant | ms | M5 CPU speedup | M5 Metal speedup |
|---|---|---:|---:|---:|
| 16x2-w32 | accept | 40.440256 | 8.284× | 3.066× |
| 16x2-w32 | bypass | 40.271709 | 8.318× | 3.079× |
| 16x2-w32 | offset | 40.144480 | 8.345× | 3.089× |
| 1x2-w32 | accept | 8.138147 | 4.546× | 2.949× |
| 1x2-w32 | bypass | 8.714394 | 4.246× | 2.754× |
| 1x2-w32 | offset | 8.155049 | 4.537× | 2.943× |

The complete accepted-source all-eight table below is inherited. Accepted174/SDK81033 remains unchanged; its primary result is8.366931ms. The original10× CPU goal remains active/unmet,0/8. The historical16x2 adoption bound failure, physical16 uncertainty and baseline-only forced-pressure timeout remain recorded in the [adoption report](../2026-10-28-g2-borrowed-adoption/README.md); this pilot does not overturn them. No54 M5 reference was supplied. There is no new AWS/Aeglos, mobile/ARM/TEE, giant, swap, minimum-RAM or pressure qualification.

| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| 12x2 | 34.858408 | 284.0 | 8.147× | 101.0 | 2.897× |
| 16x2 | 41.377043 | 335.0 | 8.096× | 124.0 | 2.997× |
| 1x2 | 8.366931 | 37.0 | 4.422× | 24.0 | 2.868× |
| 24x2 | 67.764001 | 528.0 | 7.792× | 176.0 | 2.597× |
| 2x2 | 11.539335 | 59.0 | 5.113× | 30.0 | 2.600× |
| 4x2 | 16.540233 | 109.0 | 6.590× | 48.0 | 2.902× |
| 8x2 | 25.136722 | 189.0 | 7.519× | 74.0 | 2.944× |
| merge-24x1 | 67.096516 | 507.0 | 7.556× | 175.0 | 2.608× |

[Protocol](artifacts/design.md), [all observations and profiles](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [native build](artifacts/native/result.json), [proof inventory](artifacts/proof-inventory.json), [closing pins](artifacts/pins.json).
