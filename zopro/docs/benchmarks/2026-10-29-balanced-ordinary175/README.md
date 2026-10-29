# Balanced ordinary G1 windows: primary SDK pilot

The declared pilot gate is **false**. Primary at 32 workers measures **8.043919 ms ON versus 7.986570 ms accepted and 8.083824 ms fresh OFF**. Paired ON/accepted is 1.006651224, bounds [1.001143122, 1.010774580], 1/15 wins; ON/OFF is 0.996045329, bounds [0.991987029, 1.003537064], 11/15 wins. Production remains source 175/SDK 20fde; no candidate is adopted.

Failed checks: `["1x2-w32/ON/OFF","1x2-w32/ON/accepted"]`. Paired change versus accepted is +0.665% and versus OFF -0.395%; positive values here mean slower. All observations remain. A passing pilot permits separate full-scope qualification; a failed pilot closes this exact mechanism without an unchanged rerun.

## Change and scope

The real fused K/H caller hardwires width 13. Extending its unused widths would not affect these proofs, so that initially suggested scope was rejected before building. Instead, this candidate changes only eligible ordinary borrowed G1 queries at width 12. The accepted environment explicitly selects width 12/parts 1; supported runtime bounds remain 1024..65535 points, parts 1..2, native IFMA, packed/direct and deferred-sign guards. The accepted fused width 13 child, field/G2/solver/key/scheduler code and other widths remain byte-exact.

Twenty-two balanced signed spans cover 255 bits: nine of width 11 and thirteen of width 12, totaling 35840 bucket slots versus 21×2048 ordinary uniform buckets plus the indexed top. The signed reader uses the preceding bit as its carry, without a recoded digit array. Input points remain borrowed; pending/collision/exception kernels and equal-weight merging are unchanged. The last indexed digit becomes an ordinary balanced window. Existing batch sizes follow each span width.

Lower bucket capacity does not establish fewer total operations, faster proofs or lower process RSS: digit counts, collisions, batch inversions and top reduction differ. The existing conservative forecast charges 22 full 2048 parents plus top scratch; new spans fit those parent/batch bounds. Span metadata is 352 bytes on 64-bit, with compiler frames additional. No new input-sized buffer, retained key table, pool or plan is introduced.

The private 176-file snapshot copies 175 accepted originals: 174 remain byte-exact; reversing the same-line parent cfg hook/suffix and removing the child recovers 175. Both native configurations pass unfiltered core suites (124 OFF/126 ON) and 8 prover/raw-cubic tests each. The two new tests check exact 320-bit reconstruction over 4861 valid scalars/106942 digits, and independent generator/owned-control sums across workers 1/4, parts 1/2, lengths 0/1/17/1024/21006/35973/65535 and six scalar families, with identities, repeated/opposite points and limb/sign boundaries. The unchanged 17 field source/manifest files inherit 33 field tests; no new field suite is claimed.

## Complete verified requests

Ryzen 9950X, 32 logical workers on CPUs 0–31. Fifteen fixed rotated/reversed process blocks compare accepted, fresh OFF and ON; each measures 151 SDK requests after 3 primes. Witness solving, fresh randomized proving, SDK verification and serialization are timed; loading is excluded. All 9 shapes × 3 variants preflight and independently verify before latency. All 45 latency captures precede their 45 Gnark checks. No owned build/helper/hash/archive/second workload overlaps capture; no exclusive-host claim is made.

| Variant | Median block ms | Aggregate CPU ms/request | Process peak RSS median/max MiB |
|---|---:|---:|---:|
| accepted | 7.986570 | 181.619 | 130.00/133.00 |
| OFF | 8.083824 | 181.939 | 129.50/132.25 |
| ON | 8.043919 | 180.562 | 130.50/134.00 |

Process high-water RSS includes loading/primes; it is not minimum RAM. Displayed medians are not paired-ratio estimates.

| Comparison | Paired ratio [bounds] | Wins | Paired saving µs | Check |
|---|---|---:|---:|---|
| ON/accepted | 1.006651224 [1.001143122, 1.010774580] | 1 | -53.431 | FAIL |
| ON/OFF | 0.996045329 [0.991987029, 1.003537064] | 11 | 31.850 | FAIL |
| OFF/accepted | 1.005164428 [1.004117758, 1.014387143] | 0 | -41.519 | pass |

Accepted SHA256 `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f` and fresh OFF `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f` are byte-identical executables at different paths. Pilot primary gains require upper paired ratios below 1 and at least 10/15 wins against both controls; OFF/accepted calibration bounds must lie within [1/1.03,1.03]. Sorted paired-ratio ranks 4–12 of 15 give nominal 96.484375% per-comparison coverage under independent exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusion, sample extension, changed threshold or fastest-layout selection is used.

Aggregate median CPU/request falls from 181.619 to 180.562 ms (0.582%). The separate stat captures also show fewer retired instructions and cycles. These observations do not establish an accepted-proof latency gain. The positive paired saving against OFF is **31.850 µs**, with bounds crossing parity; it is retained rather than rounded to zero. The byte-identical OFF/accepted gap also remains, without a claimed exclusive cause.

## CPU profiles

After latency verification, three FIFO-gated four-event stat runs (31 requests) and three separate 199 Hz cycles:u profiles (251 requests), after 3 primes each, precede their six Gnark checks and decoding. FIFO gates exclude loading/primes. Diagnostic times do not enter the latency gate.

| Stat variant | Cycles/request, million | Instructions/request, million | Generic cache misses/request, million | Branch misses/request, million |
|---|---:|---:|---:|---:|
| arith-1x2-w32-OFF | 831.067 | 1419.094 | 2.765 | 1.196 |
| arith-1x2-w32-ON | 825.638 | 1411.657 | 2.704 | 1.176 |
| arith-1x2-w32-accepted | 837.543 | 1423.864 | 2.798 | 1.198 |

| Profile variant | Samples | Lost | Ordinary-balanced matched samples | Matched cycle-period share |
|---|---:|---:|---:|---:|
| cycles-1x2-w32-OFF | 8976 | 0 | 0 | 0.000% |
| cycles-1x2-w32-ON | 9077 | 0 | 74 | 1.411% |
| cycles-1x2-w32-accepted | 9081 | 0 | 0 | 0.000% |

All stat events run at least 99.9%; each profile contains one SDK PID and zero lost records. Name matches can miss inlined work; their period is not removable wall time or an isolated cost of the changed reader. These observations do not establish an exclusive cache/DRAM/scheduler cause.

## CPU and Metal scope

The fresh candidate primary comparison follows; M5 values are supplied historical references, not new M5 captures.

| Primary variant | Measured ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| accepted | 7.986570 | 37 | 4.633× | 24 | 3.005× |
| OFF | 8.083824 | 37 | 4.577× | 24 | 2.969× |
| ON | 8.043919 | 37 | 4.600× | 24 | 2.984× |

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

All 165 workflow steps complete. All **7,902 SDK requests** verify internally; all **78 fresh 164-byte exports** independently pass Gnark, exact image/public binding and freshness against 17,005 prior identities. The ledger is **17,083**, with 156 proof/public members representing 78 proofs. Native libraries, shared proc macros, helper/driver, source 176 and all image/cache pins close. Full source including non-Rust files, commands, observations, proofs and decoded profiles are archived. Three owned raw perf recordings and binary/rlib payloads stay local with hashes. No successful capture is repeated.

No new portable/ARM/mobile/TEE, physical 16 latency, giant 250/4999, concurrent-proof, swap, active-pressure, minimum-RAM or AWS/Aeglos result is claimed. [Accepted production](../2026-10-29-balanced-kh-adoption/README.md) remains unchanged. [Design](artifacts/design.md), [history audit](artifacts/history-audit.md), [analysis](artifacts/analysis.json), [build/tests](artifacts/native/result.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [capture completion](artifacts/runs/result.json).
