# B2-only grouping bypass

The isolated bypass is **not adopted**. Primary32 is **1.285% slower than OFF**, with0/7wins and paired bounds[1.002332,1.025829]. It is1.934% slower than accepted. Primary OFF/accepted calibration passes, and both gain checks fail. The positive physical16 OFF comparison remains reported with wide uncertainty.

The private source changes only the grouping attempt inside g2_query_exclusive. GlobalDEDUP=1, workspace forecasts, G1/fusedKH, key/solver/field source and accepted native flags remain. This isolates the caller attempt from the [global grouping-switch negative](../2026-10-28-dedup-off174/README.md). No table, plan, unsafe code or production runtime option is added.173of174 original files remain exact; inverse patch recovers all174.

Both modes pass122unfiltered core and8prover tests. Coherent libraries are frozen with actual field/core flags/features and transitive proc macros. OFF/ON share one candidate executable path/inode; the accepted SDK is retained independently. The7fixed interleaved blocks cover primary16/32,16x2/32 and54/32,151primary/31other requests after3primes. Loading is excluded; solving, fresh proving, verification and serialization are included. No owned build/helper/hash/archive/verifier/profile/otherbench overlaps timing; no exclusive-host claim.

| Cell | Variant | Median ms | CPU ms/request | Median peak MiB | Max peak MiB |
|---|---|---:|---:|---:|---:|
| 16x2-w32 | accept | 40.284412 | 1122.765 | 793.00 | 830.00 |
| 16x2-w32 | bypass | 40.485223 | 1129.129 | 803.00 | 849.75 |
| 16x2-w32 | offset | 40.324146 | 1127.859 | 803.25 | 834.25 |
| 1x2-w16 | accept | 9.824441 | 121.766 | 87.00 | 89.25 |
| 1x2-w16 | bypass | 10.131861 | 122.940 | 64.25 | 67.25 |
| 1x2-w16 | offset | 10.487792 | 121.387 | 88.25 | 89.25 |
| 1x2-w32 | accept | 8.105792 | 186.982 | 132.25 | 136.25 |
| 1x2-w32 | bypass | 8.228011 | 192.407 | 85.25 | 88.25 |
| 1x2-w32 | offset | 8.123635 | 188.197 | 130.25 | 136.00 |
| merge-54x1-w32 | accept | 161.612585 | 4411.284 | 1765.97 | 1810.85 |
| merge-54x1-w32 | bypass | 164.634203 | 4489.666 | 1787.00 | 1852.27 |
| merge-54x1-w32 | offset | 162.279503 | 4455.030 | 1774.66 | 1791.64 |

Here accept is the retained SDK, offset is private OFF and bypass is private ON. RSS includes key load/primes and is not minimum RAM. Primary32 peak drops from136.00MiB OFF to88.25MiB ON, while CPU rises188.197→192.407ms/request. The lower peak does not override the latency regression.

| Pair | Median ratio | Lower | Upper | Wins | Paired saving ms | Required check |
|---|---:|---:|---:|---:|---:|---|
| 16x2-w32/bypass/accept | 1.009331917 | 0.984696919 | 1.028644479 | 1/7 | -0.373124 | pass |
| 16x2-w32/bypass/offset | 1.005643571 | 0.984668407 | 1.027630889 | 2/7 | -0.227199 | pass |
| 16x2-w32/offset/accept | 1.001347981 | 0.979170899 | 1.027289194 | 2/7 | -0.054473 | descriptive |
| 1x2-w16/bypass/accept | 1.019932718 | 0.991691448 | 1.234375351 | 2/7 | -0.195489 | descriptive |
| 1x2-w16/bypass/offset | 0.981179737 | 0.871928758 | 1.236685801 | 5/7 | 0.197383 | descriptive |
| 1x2-w16/offset/accept | 1.048408273 | 0.967051270 | 1.182770195 | 2/7 | -0.484254 | descriptive |
| 1x2-w32/bypass/accept | 1.019335438 | 1.009802219 | 1.022932276 | 0/7 | -0.156495 | fail |
| 1x2-w32/bypass/offset | 1.012848435 | 1.002331608 | 1.025829172 | 0/7 | -0.104376 | fail |
| 1x2-w32/offset/accept | 1.003475706 | 0.996229366 | 1.016833260 | 1/7 | -0.028244 | pass |
| merge-54x1-w32/bypass/accept | 1.017494851 | 0.980591661 | 1.029898311 | 1/7 | -2.801694 | pass |
| merge-54x1-w32/bypass/offset | 1.011427658 | 0.992028745 | 1.022651011 | 1/7 | -1.869318 | pass |
| merge-54x1-w32/offset/accept | 1.003111234 | 0.976656761 | 1.025670734 | 2/7 | -0.502046 | descriptive |

Failed checks: `["1x2-w32/bypass/offset","1x2-w32/bypass/accept"]`.

Bounds are the ordered extrema of7, nominal98.4375% under independent exchangeable blocks without familywise guarantees. The prospective primary condition requires upper<1 and at least5/7wins against both controls plus OFF/accepted calibration withinfactor1.03. Larger ON comparisons require upper<=1.03, with calibration separately reported. Physical16 is descriptive. No gain floor, excluded observation, extension, successful-capture repeat or changed threshold. Further all-eight/clean/portable/resource qualification is not performed after this failed gate.

The earlier traced1.4ms grouping interval was elapsed time, not a removable CPU or wall-time budget. This negative holds the original forecast fixed and therefore rejects a simple explanation based solely on the previous global-switch forecast difference. The result does not isolate a cache, allocator, scheduling or preemption cause. See [interpretation](artifacts/interpretation.md).

All12preflights independently verify before84timing processes;84final exports independently verify afterward. Totals:7944complete internally verified SDK requests,96fresh164-byte Gnark exports,192proof/public members, ledger15115→15211. Original source/library/executable/input/helper pins match after capture. Source174/SDK81033 and accepted8.366931ms remain unchanged; the private selector is not promoted.

New reference comparisons cover only1x2 and16x2. M5figures are supplied historical references, not matched-machine measurements.

| Cell | Variant | ms | M5 CPU speedup | M5 Metal speedup |
|---|---|---:|---:|---:|
| 16x2-w32 | accept | 40.284412 | 8.316× | 3.078× |
| 16x2-w32 | bypass | 40.485223 | 8.275× | 3.063× |
| 16x2-w32 | offset | 40.324146 | 8.308× | 3.075× |
| 1x2-w32 | accept | 8.105792 | 4.565× | 2.961× |
| 1x2-w32 | bypass | 8.228011 | 4.497× | 2.917× |
| 1x2-w32 | offset | 8.123635 | 4.555× | 2.954× |

The following complete accepted-source all-eight table is inherited. The original10×CPU goal remains active/unmet, accepted0/8. No54M5reference was supplied. No new AWS/Aeglos, mobile/ARM/TEE, giant, swap, minimum-RAM or pressure qualification.

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

[Protocol](artifacts/design.md), [all observations](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [native build](artifacts/native/result.json), [proof inventory](artifacts/proof-inventory.json), [closing source pins](artifacts/pins.json).
