# Selective fused-Fq outlining: primary SDK pilot

The declared pilot gate is **false**. Primary at 32 workers measures **8.182866 ms ON versus 7.986616 ms accepted and 8.009178 ms fresh OFF**. Paired ON/accepted is 1.023425454, bounds [1.018043920, 1.026681337], 0/15 wins; ON/OFF is 1.019896291, bounds [1.018247558, 1.025291678], 0/15 wins. Production remains source 175/SDK 20fde; no candidate is adopted.

Failed checks: `["1x2-w32/ON/OFF","1x2-w32/ON/accepted"]`. Paired change versus accepted is +2.343% and versus OFF +1.990%; positive values here mean slower. All observations remain. A passing pilot permits separate full-scope qualification; a failed pilot closes this exact mechanism without an unchanged rerun.

## Change and scope

Only native eight-lane Fq R256 fused products use a new non-inlined helper. The helper calls the exact original CIOS arithmetic body; other field modes retain the original path. The source template and its officially generated eight-lane file are the only changes among 175 files. All 173 other files remain exact. Reversing the template edit and running the unchanged official generator recovers all 175 accepted files. Core, G2 consumers, solver, key, scheduler and memory forecasting are unchanged.

This tests code footprint and call boundaries, without adding heap allocations, lookup tables, point copies, plans, pools, unsafe code or handwritten assembly. Rust calls can add stack traffic, copies and overhead. Smaller linked functions alone establish neither faster proofs nor lower process memory.

Both configurations freshly pass all 33 existing field tests, 124 core tests and 8 prover tests. Field tests include independent Arkworks comparisons and fused reduction boundaries; no new arithmetic cases were added. The official generator check also passes. These results do not qualify portable or giant-circuit behavior.

## Linked code before timing

The linked audit verifies the candidate helper is the native eight-lane Fq/MIXED=true/FUSED=true specialization, and the G2 borrowed caller invokes it. The table describes selected linked functions, not whole-binary size or dynamic instruction counts.

| Variant | G2 caller bytes | Helper bytes | Caller outline sites |
|---|---:|---:|---:|
| accepted | 51950 | 0 | 0 |
| control | 51950 | 0 | 0 |
| candidate | 45042 | 1748 | 5 |

The candidate caller plus helper occupies 46,790 bytes versus 51,950 bytes for the original caller. This selected footprint difference does not establish instruction-cache pressure or explain a latency result.

## Complete verified requests

On this Ryzen 9950X, every latency process uses 32 workers on CPUs 0–31, measures 151 requests after 3 primes, and exports a fresh proof for independent verification. Fifteen fixed rotated/reversed blocks compare accepted, fresh OFF and ON. All 9 shapes × 3 variants pass preflight Gnark checks before latency; all 45 latency captures precede their 45 Gnark checks. No owned build, helper compilation, hashing, archiving or second workload overlaps captures. The host is shared; exclusive access is not claimed.

| Variant | Median latency ms | CPU ms/request | Median/max process RSS MiB |
|---|---:|---:|---:|
| accepted | 7.986616 | 181.207 | 129.75/133.50 |
| OFF | 8.009178 | 181.802 | 131.25/132.25 |
| ON | 8.182866 | 184.291 | 131.25/135.00 |

Process high-water RSS includes loading/primes; it is not minimum RAM. Displayed medians are not paired-ratio estimates.

| Comparison | Paired ratio [bounds] | Wins | Paired saving µs | Check |
|---|---|---:|---:|---|
| ON/accepted | 1.023425454 [1.018043920, 1.026681337] | 0 | -187.092 | FAIL |
| ON/OFF | 1.019896291 [1.018247558, 1.025291678] | 0 | -158.238 | FAIL |
| OFF/accepted | 1.003938928 [0.997388724, 1.008062368] | 5 | -31.540 | pass |

Accepted SHA256 `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f` and fresh OFF `e615d014d083492d201579fa9036f84e5825f6377c96d9d8d0384e4b3ed267bb` are distinct compiled executables. Pilot primary gains require upper paired ratios below 1 and at least 10/15 wins against both controls; OFF/accepted calibration bounds must lie within [1/1.03,1.03]. Sorted paired-ratio ranks 4–12 of 15 give nominal 96.484375% per-comparison coverage under independent exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusion, sample extension, changed threshold or fastest-layout selection is used.

## CPU profiles

After latency verification, three FIFO-gated four-event stat runs (31 requests) and three separate 199 Hz cycles:u profiles (251 requests), after 3 primes each, precede their six Gnark checks and decoding. FIFO gates exclude loading/primes. Diagnostic times do not enter the latency gate.

| Stat variant | Cycles/request, million | Instructions/request, million | Generic cache misses/request, million | Branch misses/request, million |
|---|---:|---:|---:|---:|
| arith-1x2-w32-OFF | 831.585 | 1421.454 | 3.017 | 1.208 |
| arith-1x2-w32-ON | 839.564 | 1437.815 | 2.835 | 1.187 |
| arith-1x2-w32-accepted | 835.201 | 1424.341 | 2.835 | 1.199 |

| Profile variant | Samples | Lost | Outlined helper matched samples | Matched cycle-period share |
|---|---:|---:|---:|---:|
| cycles-1x2-w32-OFF | 9081 | 0 | 0 | 0.000% |
| cycles-1x2-w32-ON | 9162 | 0 | 390 | 7.233% |
| cycles-1x2-w32-accepted | 9128 | 0 | 0 | 0.000% |

All stat events run at least 99.9%; each profile contains one SDK PID and zero lost records. Name matches can miss inlined work; their period is not removable wall time or an isolated cost of the changed call boundary. These observations do not establish an exclusive cache/DRAM/scheduler cause.

## CPU and Metal scope

The fresh candidate primary comparison follows; M5 values are supplied historical references, not new M5 captures.

| Primary variant | Measured ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| accepted | 7.986616 | 37 | 4.633× | 24 | 3.005× |
| OFF | 8.009178 | 37 | 4.620× | 24 | 2.997× |
| ON | 8.182866 | 37 | 4.522× | 24 | 2.933× |

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

All 165 workflow steps complete. All **7,902 SDK requests** verify internally; all **78 fresh 164-byte exports** independently pass Gnark, exact image/public binding and freshness against 17,083 prior identities. The ledger is **17,161**, with 156 proof/public members representing 78 proofs. Native libraries, shared proc macros, helper/driver, source 175 and all image/cache pins close. Full source including non-Rust files, commands, observations, proofs and decoded profiles are archived. Three owned raw perf recordings and binary/rlib payloads stay local with hashes. No successful capture is repeated.

No new portable/ARM/mobile/TEE, physical 16 latency, giant 250/4999, concurrent-proof, swap, active-pressure, minimum-RAM or AWS/Aeglos result is claimed. [Accepted production](../2026-10-29-balanced-kh-adoption/README.md) remains unchanged. [Design](artifacts/design.md), [history audit](artifacts/history-audit.md), [analysis](artifacts/analysis.json), [build/tests](artifacts/native/result.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [capture completion](artifacts/runs/result.json).
