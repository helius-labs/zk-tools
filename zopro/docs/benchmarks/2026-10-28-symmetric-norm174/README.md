# Fused symmetric G2 denominator norms

**History correction:** the [2026-10-24 G2 norm trial](../2026-10-24-g2-norm-squares/README.md) already tested this arithmetic mechanism. The pre-capture audit missed it. This study supplies later measurements and a different integration context, not a new arithmetic idea. [Correction](artifacts/history-correction.md); timings, failed gate and non-adoption are unchanged.

The candidate is **not adopted**. Primary32 improves by a paired **0.533% / 43.803 µs against OFF**, winning all seven blocks. Its accepted comparison spans parity, and both merge54 nonregression upper bounds fail. Fresh OFF is byte-identical to accepted; the remaining process variation is not assigned to a compiler difference. See the [interpretation](artifacts/interpretation.md) for all positive estimates, failed checks and limitations.

The private candidate replaces general two-product evaluation of a²+b² with fused symmetric squares and a shared Montgomery reduction. The primary32 paired ON/OFF ratio is 0.994669471, with bounds [0.972928175, 0.999106396] and 7/7 wins. The prospective pilot gate is **false**; production remains unchanged.

The eight-lane Fq/R256 norm uses 60 product IFMA instructions instead of 100, with the common reduction retained. Product symmetry doubles only cross products. The six-coefficient diagonal schedule derives from the earlier integrated square; that G1 pilot preserved these fused Fq2 norms. The new method accepts normalized operands below2p and returns normalized values below2p after reduce_fused. The private cfg zopro_symmetric_norm selects it only at the native norm call. G1, general field multiplication, complex square/multiply, keys, solver, images, scalar conversion, allocator, pool and forecasts are unchanged. No table, heap scratch or retained plan is added.

Three original files have reversible module/wrapper/dispatch edits,171 originals remain exact, and one new child module gives175 candidate files. Reversing edits and removing the module recovers174 originals. Original OFF field methods and norm expression remain verbatim. Unsupported dispatch retains original arithmetic. Accepted, OFF and ON are separate executables with coherent frozen native libraries and proc macros; actual flags/features and closing pins are audited. OFF and ON use the same source and differ by the private compile cfg. The accepted SDK remains81033.

Both core suites pass122 tests and both prover suites pass8. Field suite counts are OFF33 and ON35; the two added ON tests compare full-width/lazy/edge values against Arkworks, check normalized output bounds, and exercise dependent chains. Some optional existing fixture tests return if their fixtures are absent; those scenarios are not claimed.

Seven fixed rotated/reversed blocks cover primary16/32,16x2/32 and merge54/32, with151 primary or31 other requests after3 primes. Loading is excluded; solving, fresh proving, internal verification and serialization are timed. No owned build/helper/hash/archive/verifier/profile or second benchmark overlaps capture. This is a shared host. Raw labels accept/offset/bypass mean accepted/OFF/ON; ON does not bypass grouping.

| Cell | Variant | Median ms | CPU ms/request | Median peak MiB | Max peak MiB |
|---|---|---:|---:|---:|---:|
| 16x2-w32 | accept | 40.267150 | 1130.558 | 801.00 | 817.50 |
| 16x2-w32 | bypass | 40.100693 | 1113.606 | 777.75 | 802.00 |
| 16x2-w32 | offset | 40.367325 | 1122.229 | 798.00 | 812.75 |
| 1x2-w16 | accept | 9.827200 | 122.097 | 88.00 | 89.00 |
| 1x2-w16 | bypass | 9.783643 | 122.662 | 86.50 | 89.50 |
| 1x2-w16 | offset | 10.795716 | 121.178 | 88.00 | 90.75 |
| 1x2-w32 | accept | 8.142533 | 187.642 | 131.00 | 133.75 |
| 1x2-w32 | bypass | 8.126523 | 186.562 | 130.75 | 133.00 |
| 1x2-w32 | offset | 8.217384 | 187.765 | 131.25 | 135.25 |
| merge-54x1-w32 | accept | 161.758486 | 4425.930 | 1754.68 | 1832.01 |
| merge-54x1-w32 | bypass | 161.927315 | 4432.526 | 1814.36 | 1905.09 |
| merge-54x1-w32 | offset | 162.412460 | 4455.276 | 1812.62 | 1892.81 |

RSS includes loading/priming and is not minimum required RAM.

| Pair | Median ratio | Lower | Upper | Wins | Paired saving ms | Required check |
|---|---:|---:|---:|---:|---:|---|
| 16x2-w32/bypass/accept | 0.997946151 | 0.954644152 | 1.013880097 | 4/7 | 0.082897 | pass |
| 16x2-w32/bypass/offset | 0.996811180 | 0.963282563 | 1.015393936 | 4/7 | 0.128853 | pass |
| 16x2-w32/offset/accept | 1.001138602 | 0.963888894 | 1.029697855 | 3/7 | -0.045956 | descriptive |
| 1x2-w16/bypass/accept | 0.998636845 | 0.961051722 | 1.109286174 | 5/7 | 0.013396 | descriptive |
| 1x2-w16/bypass/offset | 0.899353503 | 0.721255051 | 1.083933449 | 5/7 | 1.086551 | descriptive |
| 1x2-w16/offset/accept | 1.111401815 | 0.978069952 | 1.393956016 | 2/7 | -1.082113 | descriptive |
| 1x2-w32/bypass/accept | 1.000607796 | 0.985819409 | 1.005104052 | 3/7 | -0.004949 | fail |
| 1x2-w32/bypass/offset | 0.994669471 | 0.972928175 | 0.999106396 | 7/7 | 0.043803 | pass |
| 1x2-w32/offset/accept | 1.006903035 | 0.996475253 | 1.013249934 | 1/7 | -0.056336 | pass |
| merge-54x1-w32/bypass/accept | 1.003227876 | 0.984734958 | 1.034816547 | 1/7 | -0.516624 | fail |
| merge-54x1-w32/bypass/offset | 0.997012883 | 0.974186083 | 1.036043323 | 4/7 | 0.485145 | fail |
| merge-54x1-w32/offset/accept | 1.004849819 | 0.979842397 | 1.029811341 | 2/7 | -0.788045 | descriptive |

Failed checks: `["1x2-w32/bypass/accept","merge-54x1-w32/bypass/offset","merge-54x1-w32/bypass/accept"]`. All OFF/accepted calibration results remain separately recorded.

Bounds are extrema of7 block ratios, nominal98.4375% under independent exchangeable blocks, without familywise guarantees. Primary32 requires upper<1 and at least5/7 wins against both controls plus OFF/accepted bounds withinfactor1.03. Larger ON comparisons require upper<=1.03; physical16 is descriptive. No minimum gain floor, exclusions, extension, successful-capture repeat or changed threshold. A passing pilot selects further all-eight, clean, portable and resource qualification only.

After all latency and latency verification,6 FIFO-gated grouped counter captures and6 separate199Hz cycles profiles cover primary16/32 and all variants. Counter runs have31 requests; profiles251, after3 primes. All diagnostic captures precede their verification/decoding. Perf subprocesses run as root; latency processes run as the normal user. FIFO gates exclude loading/primes. Every counter runs at least99.9%; profiles have one sampled PID and zero lost events. Diagnostic times do not enter the latency decision. One counter/profile per cell has no seven-block confidence statement or isolated cache/port interpretation. Sampled CPU shares are not removable wall-time fractions.

| Counter process | Event | Count/request | Running % |
|---|---|---:|---:|
| arith-1x2-w16-accept | branch-misses:u | 1137560.839 | 100.00 |
| arith-1x2-w16-accept | cache-misses:u | 2191887.484 | 100.00 |
| arith-1x2-w16-accept | cycles:u | 596586785.677 | 100.00 |
| arith-1x2-w16-accept | instructions:u | 1413317173.032 | 100.00 |
| arith-1x2-w16-bypass | branch-misses:u | 1115242.161 | 100.00 |
| arith-1x2-w16-bypass | cache-misses:u | 2189382.645 | 100.00 |
| arith-1x2-w16-bypass | cycles:u | 598125321.613 | 100.00 |
| arith-1x2-w16-bypass | instructions:u | 1413558338.871 | 100.00 |
| arith-1x2-w16-offset | branch-misses:u | 1134903.387 | 100.00 |
| arith-1x2-w16-offset | cache-misses:u | 2209409.516 | 100.00 |
| arith-1x2-w16-offset | cycles:u | 599805426.710 | 100.00 |
| arith-1x2-w16-offset | instructions:u | 1413922054.065 | 100.00 |
| arith-1x2-w32-accept | branch-misses:u | 1248922.806 | 100.00 |
| arith-1x2-w32-accept | cache-misses:u | 3132433.516 | 100.00 |
| arith-1x2-w32-accept | cycles:u | 856087021.548 | 100.00 |
| arith-1x2-w32-accept | instructions:u | 1453699202.161 | 100.00 |
| arith-1x2-w32-bypass | branch-misses:u | 1215159.290 | 100.00 |
| arith-1x2-w32-bypass | cache-misses:u | 3132764.290 | 100.00 |
| arith-1x2-w32-bypass | cycles:u | 847862049.710 | 100.00 |
| arith-1x2-w32-bypass | instructions:u | 1453086739.903 | 100.00 |
| arith-1x2-w32-offset | branch-misses:u | 1239929.581 | 100.00 |
| arith-1x2-w32-offset | cache-misses:u | 3142716.581 | 100.00 |
| arith-1x2-w32-offset | cycles:u | 857045987.839 | 100.00 |
| arith-1x2-w32-offset | instructions:u | 1453581298.871 | 100.00 |

| Profile | Samples | Sampled TIDs | Lost events |
|---|---:|---:|---:|
| cycles-1x2-w16-accept | 5321 | 17 | 0 |
| cycles-1x2-w16-bypass | 5733 | 17 | 0 |
| cycles-1x2-w16-offset | 5583 | 17 | 0 |
| cycles-1x2-w32-accept | 9109 | 33 | 0 |
| cycles-1x2-w32-bypass | 9172 | 33 | 0 |
| cycles-1x2-w32-offset | 9166 | 33 | 0 |

All12 preflights independently verify before84 latency processes. Their84 final exports verify afterward, and12 diagnostic exports verify after all diagnostic captures. Total9672 internally verified SDK requests,108 fresh164-byte independent Gnark exports,216 proof/public members; ledger15325→15433. Raw perf data stays local with hashes; decoded data, complete source and provenance are archived.

The prior [whole-query scheduling negative](../2026-10-28-whole-query-waves174/README.md) remains closed. History also preserves failed G1 symmetric-square, radix52 bucket storage and smaller-record trials. This candidate does not reclassify their results. Primary-source [SRE](https://arxiv.org/html/2610.27452v1) was rechecked against prior local window-reuse studies; its speed claims are not predictions for this implementation. No paper speedup is transferred to this norm experiment.

M5 figures below are supplied historical references, not matched-machine measurements. New comparisons cover only1x2 and16x2.

| Cell | Variant | ms | M5 CPU speedup | M5 Metal speedup |
|---|---|---:|---:|---:|
| 16x2-w32 | accept | 40.267150 | 8.319× | 3.079× |
| 16x2-w32 | bypass | 40.100693 | 8.354× | 3.092× |
| 16x2-w32 | offset | 40.367325 | 8.299× | 3.072× |
| 1x2-w32 | accept | 8.142533 | 4.544× | 2.947× |
| 1x2-w32 | bypass | 8.126523 | 4.553× | 2.953× |
| 1x2-w32 | offset | 8.217384 | 4.503× | 2.921× |

The complete accepted-source all-eight table below is inherited. Accepted174/SDK81033 remains unchanged, with primary8.366931ms. The original10× CPU goal remains active/unmet,0/8. Historical adoption exceptions, physical16 uncertainty and the baseline-only forced-pressure timeout remain in the [adoption report](../2026-10-28-g2-borrowed-adoption/README.md). No54 M5 reference was supplied. No new AWS/Aeglos, phone/ARM/TEE, giant, swap, minimum-RAM, concurrent-proof or pressure qualification is established.

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
