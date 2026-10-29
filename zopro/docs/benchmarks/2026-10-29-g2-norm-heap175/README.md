# G2 norm-product heap: primary SDK pilot

The declared pilot gate is **false**. Primary at 32 workers measures **8.036957 ms ON versus 7.993465 ms accepted and 8.059099 ms fresh OFF**. Paired ON/accepted is 1.004814498, bounds [1.001550858, 1.008691334], 3/15 wins; ON/OFF is 0.998097427, bounds [0.994602523, 1.002140674], 10/15 wins. Production remains source 175/SDK 20fde; no candidate is adopted.

Failed checks: `["1x2-w32/ON/OFF","1x2-w32/ON/accepted"]`. Paired change versus accepted is +0.481% and versus OFF -0.190%; positive values here mean slower. All observations remain. A passing pilot permits separate full-scope qualification; a failed pilot closes this exact mechanism without an unchanged rerun.

## Change and scope

Eligible full eight-lane borrowed G2 batches replace the sequential norm chain with a ragged binary product heap. For N lane groups, leaves occupy slots N through 2N-1 and internal nodes occupy 1 through N-1. The root uses the unchanged native eight-lane inverse; ascending inverse propagation then supplies one inverse norm per group. Existing point formulas consume those factors unchanged. This shortens only the norm dependency graph; it does not make MSM or proof latency logarithmic. Extra memory accesses and bounds checks may outweigh the changed dependencies.

The heap reuses exactly N existing two-vector prefix records, without power-of-two padding, extra key/point copies, lookup tables, new input-sized buffers, unsafe code, assembly, schedules or forecasts. Retained capacity is reused; growing records requires initialization. Compiler temporaries and total stack remain unmeasured. FAST=true, nonempty complete native eight-lane batches with vector inversion enabled select this path; tails and disabled paths retain the original body.

The private snapshot has 176 files: 174 of 175 originals remain exact, one parent gains a cfg-only hook/module declaration and one child is added. Reversing the parent edits recovers all 175 accepted files. All 17 field files remain exact, so the existing 33 field-test qualification is inherited. Fresh OFF/ON execute 124/126 core tests and 8 prover tests each. Two new tests compare every heap leaf against scalar inverses across arbitrary lengths and complete signed point batches, tails, disabled paths and scratch reuse against generator scalar sums. The unchanged official generator check passes.

## Linked route before timing

The linked audit verifies the candidate norm-heap path is reached from the actual G2 borrowed caller and is absent in OFF. Selected disassemblies and hashes are archived. Static sizes, sample shares and instruction sites are not dynamic costs, memory bounds or proof of a bottleneck.

## Complete verified requests

On this Ryzen 9950X, every latency process uses 32 workers on CPUs 0–31, measures 151 requests after 3 primes, and exports a fresh proof for independent verification. Fifteen fixed rotated/reversed blocks compare accepted, fresh OFF and ON. All 9 shapes × 3 variants pass preflight Gnark checks before latency; all 45 latency captures precede their 45 Gnark checks. No owned build, helper compilation, hashing, archiving or second workload overlaps captures. The host is shared; exclusive access is not claimed.

| Variant | Median latency ms | CPU ms/request | Median/max process RSS MiB |
|---|---:|---:|---:|
| accepted | 7.993465 | 181.486 | 130.50/133.00 |
| OFF | 8.059099 | 182.288 | 132.00/135.75 |
| ON | 8.036957 | 181.590 | 129.75/133.25 |

Process high-water RSS includes loading/primes; it is not minimum RAM. Displayed medians are not paired-ratio estimates.

| Comparison | Paired ratio [bounds] | Wins | Paired saving µs | Check |
|---|---|---:|---:|---|
| ON/accepted | 1.004814498 [1.001550858, 1.008691334] | 3 | -38.805 | FAIL |
| ON/OFF | 0.998097427 [0.994602523, 1.002140674] | 10 | 15.438 | FAIL |
| OFF/accepted | 1.006785677 [1.002203723, 1.014770184] | 3 | -54.243 | pass |

Accepted SHA256 `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f` and fresh OFF `c47feca7ff061784b011c88c137c6f2d2075ce6bffb0df071dee84ff9f40c558` are distinct compiled executables. Pilot primary gains require upper paired ratios below 1 and at least 10/15 wins against both controls; OFF/accepted calibration bounds must lie within [1/1.03,1.03]. Sorted paired-ratio ranks 4–12 of 15 give nominal 96.484375% per-comparison coverage under independent exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusion, sample extension, changed threshold or fastest-layout selection is used.

## CPU profiles

After latency verification, three FIFO-gated four-event stat runs (31 requests) and three separate 199 Hz cycles:u profiles (251 requests), after 3 primes each, precede their six Gnark checks and decoding. FIFO gates exclude loading/primes. Diagnostic times do not enter the latency gate.

| Stat variant | Cycles/request, million | Instructions/request, million | Generic cache misses/request, million | Branch misses/request, million |
|---|---:|---:|---:|---:|
| arith-1x2-w32-OFF | 824.227 | 1421.189 | 2.851 | 1.198 |
| arith-1x2-w32-ON | 824.812 | 1415.884 | 2.806 | 1.196 |
| arith-1x2-w32-accepted | 831.127 | 1420.052 | 3.048 | 1.192 |

| Profile variant | Samples | Lost | Norm-heap matched samples | Matched cycle-period share |
|---|---:|---:|---:|---:|
| cycles-1x2-w32-OFF | 9004 | 0 | 0 | 0.000% |
| cycles-1x2-w32-ON | 9253 | 0 | 1017 | 17.267% |
| cycles-1x2-w32-accepted | 9147 | 0 | 0 | 0.000% |

All stat events run at least 99.9%; each profile contains one SDK PID and zero lost records. Name matches can miss inlined work; their period is not removable wall time or an isolated cost of the norm inversion graph. These observations do not establish an exclusive cache/DRAM/scheduler cause.

## CPU and Metal scope

The fresh candidate primary comparison follows; M5 values are supplied historical references, not new M5 captures.

| Primary variant | Measured ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| accepted | 7.993465 | 37 | 4.629× | 24 | 3.002× |
| OFF | 8.059099 | 37 | 4.591× | 24 | 2.978× |
| ON | 8.036957 | 37 | 4.604× | 24 | 2.986× |

The complete all-eight accepted source 175 comparison is inherited below; it is not a candidate all-eight performance qualification or a new measurement of the other cases. All-nine preflights establish correctness, not their latency.

| Shape | Accepted ms | M5 CPU ms | CPU/accepted | M5 Metal ms | Metal/accepted | CPU 10× required ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.022165 | 37 | 4.612× | 24 | 2.992× | 3.7 |
| 2x2 | 11.335969 | 59 | 5.205× | 30 | 2.646× | 5.9 |
| 4x2 | 15.945544 | 109 | 6.836× | 48 | 3.010× | 10.9 |
| 8x2 | 24.090485 | 189 | 7.845× | 74 | 3.072× | 18.9 |
| 12x2 | 33.844855 | 284 | 8.391× | 101 | 2.984× | 28.4 |
| 16x2 | 39.931557 | 335 | 8.389× | 124 | 3.105× | 33.5 |
| 24x2 | 66.233309 | 528 | 7.972× | 176 | 2.657× | 52.8 |
| merge-24x1 | 65.273224 | 507 | 7.767× | 175 | 2.681× | 50.7 |

The all-eight CPU 10× goal remains **active and unmet**. Accepted175 reaches 0/8; this candidate has only one fresh latency row and no all-eight qualification. Primary requires 3.7 ms. Accepted 54→1 remains 162.250464 ms without a supplied M5 reference. The accepted physical 16 baseline-calibration exception remains recorded; this pilot does not rerun or relabel it.

## Verification and provenance

All 165 workflow steps complete. All **7,902 SDK requests** verify internally; all **78 fresh 164-byte exports** independently pass Gnark, exact image/public binding and freshness against 17,161 prior identities. The ledger is **17,239**, with 156 proof/public members representing 78 proofs. Native libraries, shared proc macros, helper/driver, source176 and all image/cache pins close. Full source including non-Rust files, commands, observations, proofs and decoded profiles are archived. Three owned raw perf recordings and binary/rlib payloads stay local with hashes. No successful capture is repeated.

No new portable/ARM/mobile/TEE, physical 16 latency, giant 250/4999, concurrent-proof, swap, active-pressure, minimum-RAM or AWS/Aeglos result is claimed. [Accepted production](../2026-10-29-balanced-kh-adoption/README.md) remains unchanged. [Design](artifacts/design.md), [history audit](artifacts/history-audit.md), [analysis](artifacts/analysis.json), [build/tests](artifacts/native/result.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [capture completion](artifacts/runs/result.json).
