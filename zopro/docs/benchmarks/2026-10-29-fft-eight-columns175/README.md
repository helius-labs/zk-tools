# Independent DFT8 columns in native DIF leaves

The declared pilot gate is **false**. Primary at 32 workers measures **8.102820 ms ON versus 7.970520 ms accepted and 8.018240 ms fresh OFF**. Paired ON/accepted is 1.013732696, bounds [1.009275922, 1.021377441], 1/15 wins; ON/OFF is 1.007484816, bounds [1.000221910, 1.013521412], 3/15 wins. Production remains source 175/SDK 20fde; no candidate is adopted.

Failed checks: `["1x2-w32/ON/OFF","1x2-w32/ON/accepted"]`. Paired change versus accepted is +1.373% and versus OFF +0.748%; positive values here mean slower. All observations remain. A passing pilot permits separate full-scope qualification; a failed pilot closes this exact mechanism without an unchanged rerun.

## Change and scope

The final three stages of eligible native untwisted DIF leaves now transpose four pairs of vectors into eight independent DFT8 sequences. The independently derived factored DFT8 runs across those sequences, then a second transpose restores the exact original bit-reversed order. Higher DIF stages retain their kernels; DIT/forward transforms, twisted roots, short leaves, FFT task geometry, field arithmetic, MSM, solver and key code remain unchanged. The private cfg selects the new path only for leaves of at least eight Pair vectors.

Over eight DFT8 sequences, the new source schedule uses four vector products, 26 vector additions/subtractions and 32 transpose shuffles; the original uses eight products, 24 additions/subtractions and 32 within-transform shuffles. These are static source-operation counts, not machine instructions or predicted latency. Root conversion, per-leaf constants, live vectors, stack and data movement are part of the actual SDK measurement. No input-sized buffer, table, key copy, unsafe code or assembly is added. Fixed stack temporaries and compiler frames are additional and not measured bounds.

The earlier packed DFT8 caller remains rejected. It used one product but 14 vector linear operations and 15 shuffles per two sequences, computing many discarded lanes. This changes the SIMD axis and real leaf integration, rather than rerunning its flags or annotations. Its mathematical identity is reused; no third-party GPL implementation is ported.

The private source has 176 files: 174 of 175 originals remain exact, one parent gains a cfg-only hook/module declaration, and one child is added. Reversing the parent edits recovers all 175 originals. All 17 field files remain exact, so field qualification is inherited (33 tests). Fresh OFF/ON pass 124/126 core tests and 8 prover tests each. New tests check tile basis/transpose/full-width cases at all four primitive eighth roots and whole selected DIF leaves against scalar direct transforms, including short fallbacks. Existing independent FFT/polynomial and 512 KiB worker-stack tests remain included. The unchanged official generator check passes.

A separate supplemental test executes the previous packed caller's specified inverse-roundtrip check: 512 round trips across both algorithms and four roots. Its three previously passed tests are filtered out and no performance capture is repeated. This supplements the earlier design's explicit check; it does not alter its negative measurements.

## Linked route before timing

The linked audit verifies the real candidate DIF leaf reaches the eight-column path and OFF lacks that child. Selected disassemblies and hashes are archived. Static sizes and profile symbol shares do not establish dynamic costs, minimum memory or an exclusive bottleneck.

## Complete verified requests

Ryzen 9950X, 32 workers on CPUs 0–31. Each latency process measures 151 complete SDK requests after three primes and exports a fresh proof. Fifteen fixed rotated/reversed blocks compare accepted, fresh OFF and ON. All nine shapes × three variants pass preflight Gnark checks before latency; all 45 latency captures precede their 45 Gnark checks. No owned build, helper compilation, hashing, archiving or second workload overlaps captures. The host is shared; exclusive access is not claimed.

| Variant | Median latency ms | CPU ms/request | Median/max process RSS MiB |
|---|---:|---:|---:|
| accepted | 7.970520 | 181.128 | 129.50/133.50 |
| OFF | 8.018240 | 182.261 | 130.25/134.50 |
| ON | 8.102820 | 181.513 | 130.25/133.50 |

Process high-water RSS includes loading/primes; it is not minimum RAM. Displayed medians are not paired-ratio estimates.

| Comparison | Paired ratio [bounds] | Wins | Paired saving µs | Check |
|---|---|---:|---:|---|
| ON/accepted | 1.013732696 [1.009275922, 1.021377441] | 1 | -108.745 | FAIL |
| ON/OFF | 1.007484816 [1.000221910, 1.013521412] | 3 | -59.653 | FAIL |
| OFF/accepted | 1.006049999 [1.001939671, 1.015028145] | 3 | -48.833 | pass |

Accepted SHA256 `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f` and fresh OFF `cfff712bb352aec3f48b9cf7f25a2a815edbaf1af3f35242bf0d4e18a55430e2` are distinct compiled executables. Pilot primary gains require upper paired ratios below 1 and at least 10/15 wins against both controls; OFF/accepted calibration bounds must lie within [1/1.03,1.03]. Sorted paired-ratio ranks 4–12 of 15 give nominal 96.484375% per-comparison coverage under independent exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusion, sample extension, changed threshold or fastest-layout selection is used.

## CPU profiles

After latency verification, three FIFO-gated four-event stat runs (31 requests) and three separate 199 Hz cycles:u profiles (251 requests), after 3 primes each, precede their six Gnark checks and decoding. FIFO gates exclude loading/primes. Diagnostic times do not enter the latency gate.

| Stat variant | Cycles/request, million | Instructions/request, million | Generic cache misses/request, million | Branch misses/request, million |
|---|---:|---:|---:|---:|
| arith-1x2-w32-OFF | 825.164 | 1421.927 | 2.915 | 1.192 |
| arith-1x2-w32-ON | 829.153 | 1418.894 | 2.998 | 1.202 |
| arith-1x2-w32-accepted | 816.036 | 1422.615 | 2.979 | 1.195 |

| Profile variant | Samples | Lost | Independent-column matched samples | Matched cycle-period share |
|---|---:|---:|---:|---:|
| cycles-1x2-w32-OFF | 9155 | 0 | 0 | 0.000% |
| cycles-1x2-w32-ON | 8986 | 0 | 36 | 0.998% |
| cycles-1x2-w32-accepted | 9292 | 0 | 0 | 0.000% |

All stat events run at least 99.9%; each profile contains one SDK PID and zero lost records. Name matches can miss inlined work; their period is not removable wall time or an isolated cost of the changed FFT stages. These observations do not establish an exclusive cache/DRAM/scheduler cause.

## CPU and Metal scope

The fresh candidate primary comparison follows; M5 values are supplied historical references, not new M5 captures.

| Primary variant | Measured ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| accepted | 7.970520 | 37 | 4.642× | 24 | 3.011× |
| OFF | 8.018240 | 37 | 4.614× | 24 | 2.993× |
| ON | 8.102820 | 37 | 4.566× | 24 | 2.962× |

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

All 165 workflow steps complete. All **7,902 SDK requests** verify internally; all **78 fresh 164-byte exports** independently pass Gnark, exact image/public binding and freshness against 17,239 prior identities. The ledger is **17,317**, with 156 proof/public members representing 78 proofs. Native libraries, shared proc macros, helper/driver, source176 and all image/cache pins close. Full source including non-Rust files, commands, observations, proofs and decoded profiles are archived. Three owned raw perf recordings and binary/rlib payloads stay local with hashes. No successful capture is repeated.

No new portable/ARM/mobile/TEE, physical 16 latency, giant 250/4999, concurrent-proof, swap, active-pressure, minimum-RAM or AWS/Aeglos result is claimed. [Accepted production](../2026-10-29-balanced-kh-adoption/README.md) remains unchanged. [Design](artifacts/design.md), [history audit](artifacts/history-audit.md), [analysis](artifacts/analysis.json), [build/tests](artifacts/native/result.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [capture completion](artifacts/runs/result.json).
