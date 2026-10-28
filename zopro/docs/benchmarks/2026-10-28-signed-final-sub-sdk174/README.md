# Signed final-carry subtraction SDK pilot

The candidate is **not adopted**. Primary 1×2 at 32 workers is **1.290% slower than fresh OFF** and **1.164% slower than accepted**, with 0/15 winning blocks against either. Both primary gain checks fail; the other five required checks pass. The component's 1.39–2.00% caller gains do not translate into lower proof latency. [Decision](artifacts/interpretation.md).

The candidate folds subtraction into the final signed carry after unchanged CIOS reduction/radix packing, using the existing negative repair. Only native8 G1 streaming X/Y finishes change. It uses the exact frozen libraries from the [qualified caller experiment](../2026-10-28-signed-final-sub174/README.md), linked to the ordinary cpu-sdk driver. No source or arithmetic changes follow that caller gate. There is no new heap buffer, lookup table, prepared-key structure, solver, weighted reduction, G2 or resource-policy change.

## Whole-proof results

Ryzen 9950X, shared host, 15 fixed rotated/reversed blocks across four cells and three variants. Every process loads outside timing, performs three primes, then 151 measured primary proofs or 31 larger proofs. Timed work includes solving, fresh randomized proving, SDK verification and serialization. Twelve preflight exports independently verify before 180 latency processes; all 180 final latency exports verify afterward. No owned build/helper/hash/archive/verifier/profile/second benchmark overlaps capture.

The table shows medians across process medians; RSS includes loading and primes and is not minimum RAM.

| Cell | Variant | Median ms | CPU ms/request | Median peak MiB | Max peak MiB |
|---|---|---:|---:|---:|---:|
| 16x2-w32 | OFF | 40.297738 | 1121.196 | 789.00 | 826.75 |
| 16x2-w32 | ON | 40.220612 | 1115.256 | 801.50 | 831.25 |
| 16x2-w32 | accepted | 40.269175 | 1124.118 | 804.00 | 832.00 |
| 1x2-w16 | OFF | 9.973316 | 122.780 | 87.00 | 89.25 |
| 1x2-w16 | ON | 10.192532 | 120.950 | 86.75 | 89.50 |
| 1x2-w16 | accepted | 10.040697 | 121.448 | 86.25 | 89.75 |
| 1x2-w32 | OFF | 8.146555 | 187.553 | 130.00 | 133.00 |
| 1x2-w32 | ON | 8.248926 | 186.694 | 130.50 | 134.75 |
| 1x2-w32 | accepted | 8.161132 | 187.451 | 131.25 | 135.50 |
| merge-54x1-w32 | OFF | 162.013861 | 4422.991 | 1784.43 | 1843.36 |
| merge-54x1-w32 | ON | 162.468361 | 4419.918 | 1778.94 | 1860.96 |
| merge-54x1-w32 | accepted | 162.128469 | 4431.147 | 1796.00 | 1818.01 |

Ratios below use paired within-block medians, not the ratio of the displayed medians. Intervals use ranks 4..12 of 15, nominal 96.484375% under independent/exchangeable blocks, without familywise coverage.

| Pair | Median ratio | Lower | Upper | Wins | Paired saving ms | Required check |
|---|---:|---:|---:|---:|---:|---|
| 16x2-w32/OFF/accepted | 1.000043788 | 0.994358750 | 1.003819001 | 7/15 | -0.001774 | descriptive |
| 16x2-w32/ON/OFF | 1.000824183 | 0.994256991 | 1.005994337 | 7/15 | -0.033392 | pass |
| 16x2-w32/ON/accepted | 1.000868007 | 0.992994108 | 1.007145603 | 6/15 | -0.035166 | pass |
| 1x2-w16/OFF/accepted | 0.989566245 | 0.966877368 | 1.015750552 | 9/15 | 0.103675 | descriptive |
| 1x2-w16/ON/OFF | 1.011348948 | 0.990303653 | 1.064028253 | 5/15 | -0.110588 | descriptive |
| 1x2-w16/ON/accepted | 0.997666784 | 0.961741446 | 1.056244345 | 8/15 | 0.023184 | descriptive |
| 1x2-w32/OFF/accepted | 0.998843830 | 0.993775089 | 1.002495379 | 9/15 | 0.009458 | pass |
| 1x2-w32/ON/OFF | 1.012900708 | 1.009507413 | 1.015769551 | 0/15 | -0.105378 | fail |
| 1x2-w32/ON/accepted | 1.011640277 | 1.007473848 | 1.014595149 | 0/15 | -0.094559 | fail |
| merge-54x1-w32/OFF/accepted | 0.998611950 | 0.989278229 | 1.008486551 | 9/15 | 0.224871 | descriptive |
| merge-54x1-w32/ON/OFF | 1.003598093 | 0.998053806 | 1.006714201 | 4/15 | -0.578800 | pass |
| merge-54x1-w32/ON/accepted | 0.999767703 | 0.990029486 | 1.006919594 | 8/15 | 0.037662 | pass |

Primary32 requires upper<1 and at least 10/15 wins against both controls; OFF/accepted calibration bounds must remain within [1/1.03,1.03]. Larger ON upper bounds must be<=1.03; physical16 is descriptive. There is no minimum gain floor, exclusion, extension or post-result threshold change. Primary calibration passes. Physical16 is uncertain, and larger paired estimates are mixed despite passing their nonregression checks. No unchanged rerun or adoption follows.

## CPU work, profiles and memory

Primary32 CPU time falls 187.553→186.694ms/request OFF→ON while latency rises. Separate single-run PMU counts fall 854.519→852.346million cycles and 1453.407→1449.709million instructions per request. That supports a distinction between aggregate work and parallel proof latency; it does not identify scheduling, critical-path, code-layout, cache or memory-capacity causes. The one-thread caller result remains valid within its scope.

After latency and verification, six FIFO-gated four-event PMU runs and six separate 199 Hz cycles profiles cover both primary worker counts and all variants. Counter runs use 31 proofs and profiles 251, after three primes. All diagnostic captures precede their independent verification and decode. Loading/primes are outside the FIFO gate. All events run 100%, every profile has one PID and zero lost records; total 43,434 samples. Diagnostic timing does not enter latency decisions, and sample shares are not removable wall-time fractions.

| Counter run | Event | Count/request | Running % |
|---|---|---:|---:|
| arith-1x2-w16-OFF | branch-misses:u | 1116753.871 | 100.00 |
| arith-1x2-w16-OFF | cache-misses:u | 2151503.742 | 100.00 |
| arith-1x2-w16-OFF | cycles:u | 599369339.129 | 100.00 |
| arith-1x2-w16-OFF | instructions:u | 1413973299.774 | 100.00 |
| arith-1x2-w16-ON | branch-misses:u | 1101520.806 | 100.00 |
| arith-1x2-w16-ON | cache-misses:u | 2087637.355 | 100.00 |
| arith-1x2-w16-ON | cycles:u | 595848134.581 | 100.00 |
| arith-1x2-w16-ON | instructions:u | 1408602526.516 | 100.00 |
| arith-1x2-w16-accepted | branch-misses:u | 1134965.065 | 100.00 |
| arith-1x2-w16-accepted | cache-misses:u | 2228713.968 | 100.00 |
| arith-1x2-w16-accepted | cycles:u | 596417920.161 | 100.00 |
| arith-1x2-w16-accepted | instructions:u | 1414084034.710 | 100.00 |
| arith-1x2-w32-OFF | branch-misses:u | 1239075.548 | 100.00 |
| arith-1x2-w32-OFF | cache-misses:u | 3128530.645 | 100.00 |
| arith-1x2-w32-OFF | cycles:u | 854518860.806 | 100.00 |
| arith-1x2-w32-OFF | instructions:u | 1453407255.484 | 100.00 |
| arith-1x2-w32-ON | branch-misses:u | 1224859.000 | 100.00 |
| arith-1x2-w32-ON | cache-misses:u | 3011202.806 | 100.00 |
| arith-1x2-w32-ON | cycles:u | 852345531.161 | 100.00 |
| arith-1x2-w32-ON | instructions:u | 1449708705.516 | 100.00 |
| arith-1x2-w32-accepted | branch-misses:u | 1249603.968 | 100.00 |
| arith-1x2-w32-accepted | cache-misses:u | 3150285.452 | 100.00 |
| arith-1x2-w32-accepted | cycles:u | 860493093.355 | 100.00 |
| arith-1x2-w32-accepted | instructions:u | 1453958710.839 | 100.00 |

| Profile | Samples | Sampled threads | Lost |
|---|---:|---:|---:|
| cycles-1x2-w16-OFF | 5418 | 17 | 0 |
| cycles-1x2-w16-ON | 5628 | 17 | 0 |
| cycles-1x2-w16-accepted | 5388 | 17 | 0 |
| cycles-1x2-w32-OFF | 9118 | 33 | 0 |
| cycles-1x2-w32-ON | 8831 | 33 | 0 |
| cycles-1x2-w32-accepted | 9051 | 33 | 0 |

Memory peaks are mixed. Primary ON maximum 134.75 MiB exceeds OFF 133.00, while accepted reaches 135.50. Merge54 ON maximum 1860.96 MiB exceeds both controls, despite a lower median. Unchanged heap structure and caller capacities do not guarantee lower process RSS. No peak-memory reduction, minimum-RAM or pressure qualification is claimed.

## Verification and provenance

Both SDK builds pass 8 prover/raw-cubic tests. Same-source component evidence is reused: 33/35 field tests and 125 core tests per build, including the exact-digit and independent Arkworks arithmetic checks. Those suites are not rerun or counted as new distinct inputs here. Frozen coherent standard8-library sets and procedural macro copies are retained with actual native flags/features. OFF is a separately rebuilt control; its hash differs from accepted, and no accepted-byte-identity claim is made.

All 18,696 SDK requests verify internally. All 204 fresh 164-byte exports independently verify with Gnark, have correct public-input bindings and distinct hashes from the complete prior set. There are 408 proof/public members; ledger 15,553→15,757. The private 176-file source snapshot exactly matches the compiled component bytes; 170 of 174 originals are unchanged, and reversible parent edits recover the others. All source/library/helper/input closing pins pass. Production is unchanged.

A preparation-only assertion initially found 198 diagnostic jobs instead of 204 because the freshness predecessor's census case list contained 2x2 rather than physical16 primary. The declared four performance cases were then read independently from the earlier SDK pilot. Only owned empty FIFOs were recreated. Source, executables and tests did not change; no capture had started and none was repeated. [Preparation record](artifacts/prepare-failure.json). The component-stage missing-sha2-link repair remains documented in its separate report.

Complete source, proof exports, commands, raw observations and decoded diagnostics are archived. Six raw perf recordings remain local with SHA256 receipts; only their ownership is changed after all captures so they can be read for archival hashing. No new AWS/Aeglos, phone/ARM/TEE, giant-circuit, portable, concurrent-proof, swap or forced-pressure qualification is claimed.

## CPU and Metal references

Fresh comparisons cover 1x2 and 16x2. M5 figures are supplied historical references. The fresh accepted control is distinct from a new all-eight production qualification.

| Cell | Variant | ms | M5 CPU speedup | M5 Metal speedup |
|---|---|---:|---:|---:|
| 16x2-w32 | OFF | 40.297738 | 8.313× | 3.077× |
| 16x2-w32 | ON | 40.220612 | 8.329× | 3.083× |
| 16x2-w32 | accepted | 40.269175 | 8.319× | 3.079× |
| 1x2-w32 | OFF | 8.146555 | 4.542× | 2.946× |
| 1x2-w32 | ON | 8.248926 | 4.485× | 2.909× |
| 1x2-w32 | accepted | 8.161132 | 4.534× | 2.941× |

The complete accepted table below is inherited from the [source174 adoption](../2026-10-28-g2-borrowed-adoption/README.md), including its failed performance gate and explicit engineering exception. Qualified primary remains 8.366931 ms; source174/SDK81033 is unchanged. No M5 reference was supplied for54.

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

The original all-eight CPU 10× goal remains **active and unmet, 0/8**. This rejected streaming integration establishes no new accepted proving-time result.

[Protocol](artifacts/design.md), [all observations and profiles](artifacts/analysis.json), [identities](artifacts/identities.json), [native SDK build](artifacts/native/result.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [closing pins](artifacts/pins.json), [capture completion](artifacts/runs/result.json).
