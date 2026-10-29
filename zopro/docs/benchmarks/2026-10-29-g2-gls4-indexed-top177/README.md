# Hybrid indexed-top GLS4: primary SDK pilot

The declared pilot gate is **false**. Primary at 32 workers measures **10.325850 ms ON versus 8.036072 ms accepted and 8.038908 ms fresh OFF**. Paired ON/accepted is 1.290868193, bounds [1.279597358, 1.293695592], 0/15 wins; ON/OFF is 1.283391602, bounds [1.279723389, 1.292575941], 0/15 wins. Production remains source 175/SDK 20fde; no candidate is adopted.

Failed checks: `["1x2-w32/ON/OFF","1x2-w32/ON/accepted"]`. Paired change versus accepted is +29.087% and versus OFF +28.339%; positive values here mean slower. All observations remain. A passing pilot permits separate full-scope qualification; a failed pilot closes this exact mechanism without an unchanged rerun.

## Change and scope

The candidate keeps the parent GLS4 lower five eleven-bit Horner windows. Its highest window instead reduces borrowed points using the existing tiled indexed affine tree, separately for positive/negative digit classes and sequentially for four components. Affine bucket outputs follow the same S3, phi, S2, phi²psi, S1, phi, S0 composition, now with three component merges in the top window. Four point partitions and 1,024 magnitude bins remain fixed. The full 65-bit magnitude plus Booth carry bound is used, rather than the observed 544 maximum. The old six ordinary-window candidate remains rejected.

The preceding [diagnostic](../2026-10-29-g2-gls4-census177/README.md) counted 38,842/45,725 (84.947%) existing projective feed updates in the highest window. This motivated the new reduction algorithm; it did not measure its speed or establish cycle cost. The top path removes that window's projective queue, but adds index construction, four tree passes and extra merges. Accepted phi2 already uses an indexed narrow top.

Six candidate GLS4 tests pass: the parent's decomposition/digit, map, full-MSM and admission tests rerun; two additional tests cover signed top digits including 65-bit extrema and generator sums, and full MSMs on custom Rayon pools with 1, 4, 16 and 32 threads. OFF/ON pass all 124/130 core tests and eight prover tests each. All 17 field sources are exact; 33 native field tests and 100,767 historical lattice integer cases are inherited. The official generator check passes.

The 40-byte-per-scalar temporary and entire candidate workspace are charged via dynamic Reservation::analysis in addition to existing forecasts. The conservative six-window output shape, four simultaneous indexed-top parent buffers and retained sign/component arrays, plus workers times the maximum applicable leaf scratch are included. Checked overflow/admission failure or scalar try_reserve_exact failure preserves original words and falls back to accepted phi2. No transformed key copy, point lookup table, retained setup, new runtime pool, field arithmetic or synthetic global cap. Temporary indices/scalars are additional memory; a lower RSS sample would not establish minimum RAM.

Source 177 retains 172 of 175 original files exact, three cfg-only parent edits and two new modules; inverse edits recover 175. Relative to the frozen GLS4 parent, only gls4.rs and its workspace estimate change. Original Window methods, lattice, field, solver, FFT and key sources remain exact. [Design](artifacts/design.md), [history](artifacts/history-audit.md), [implementation](artifacts/source/crates/zopro-core/src/msm_affine/g2_borrowed/gls4.rs), [cost](artifacts/source/crates/zopro-core/src/msm_affine/workspace.rs).

## Linked route and activation

New GLS4 symbols occur only in ON. The primary ON preflight must produce four hybrid-entry trace markers before the pinned Gnark verification and timing. These are the normal three primes and one preflight request. Timing and profiling have no trace. Selected disassemblies establish linked route structure, not speed or stack bounds.

| Variant | Selected GLS4 functions | Selected bytes |
|---|---:|---:|
| accepted | 0 | 0 |
| control | 0 | 0 |
| candidate | 24 | 40809 |

## Complete verified requests

On this Ryzen 9950X, every latency process uses 32 workers on CPUs 0–31, measures 151 requests after 3 primes, and exports a fresh proof for independent verification. Fifteen fixed rotated/reversed blocks compare accepted, fresh OFF and ON. All 9 shapes × 3 variants pass preflight Gnark checks before latency; all 45 latency captures precede their 45 Gnark checks. No owned build, helper compilation, hashing, archiving or second workload overlaps captures. The host is shared; exclusive access is not claimed.

| Variant | Median latency ms | CPU ms/request | Median/max process RSS MiB |
|---|---:|---:|---:|
| accepted | 8.036072 | 182.053 | 129.75/135.00 |
| OFF | 8.038908 | 182.635 | 131.00/135.00 |
| ON | 10.325850 | 257.335 | 129.25/133.50 |

Process high-water RSS includes loading/primes; it is not minimum RAM. Displayed medians are not paired-ratio estimates.

| Comparison | Paired ratio [bounds] | Wins | Paired saving µs | Check |
|---|---|---:|---:|---|
| ON/accepted | 1.290868193 [1.279597358, 1.293695592] | 0 | -2338.980 | FAIL |
| ON/OFF | 1.283391602 [1.279723389, 1.292575941] | 0 | -2276.862 | FAIL |
| OFF/accepted | 1.003384950 [0.995595132, 1.005825650] | 7 | -27.342 | pass |

Accepted SHA256 `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f` and fresh OFF `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f` are byte-identical executables at different paths. Pilot primary gains require upper paired ratios below 1 and at least 10/15 wins against both controls; OFF/accepted calibration bounds must lie within [1/1.03,1.03]. Sorted paired-ratio ranks 4–12 of 15 give nominal 96.484375% per-comparison coverage under independent exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusion, sample extension, changed threshold or fastest-layout selection is used.

## CPU profiles

After latency verification, three FIFO-gated four-event stat runs (31 requests) and three separate 199 Hz cycles:u profiles (251 requests), after 3 primes each, precede their six Gnark checks and decoding. FIFO gates exclude loading/primes. Diagnostic times do not enter the latency gate.

| Stat variant | Cycles/request, million | Instructions/request, million | Generic cache misses/request, million | Branch misses/request, million |
|---|---:|---:|---:|---:|
| arith-1x2-w32-OFF | 831.747 | 1419.254 | 3.007 | 1.194 |
| arith-1x2-w32-ON | 1179.064 | 2003.652 | 3.146 | 1.809 |
| arith-1x2-w32-accepted | 826.831 | 1420.251 | 2.807 | 1.196 |

| Profile variant | Samples | Lost records |
|---|---:|---:|
| cycles-1x2-w32-OFF | 9176 | 0 |
| cycles-1x2-w32-ON | 11933 | 0 |
| cycles-1x2-w32-accepted | 8981 | 0 |

All stat events run at least 99.9%; each profile contains one SDK PID and zero lost records. Symbol aggregates include surrounding work and inlining; cycle-period shares are not removable wall time or an isolated cost. No exclusive memory/cache/scheduler cause is established.

The change trades one hotspot for more affine-tree work. Fresh sampled cycle-period shares are:

| Variant | G2 projective add_affine | G2 reduce_layer | Fq inverse, matching monomorphizations |
|---|---:|---:|---:|
| accepted | 0.631% | 5.466% | 6.378% |
| OFF | 0.614% | 5.859% | 6.675% |
| ON | 2.744% | 13.814% | 13.809% |

The previous ordinary-top candidate's G2 projective add_affine share was 13.848% in its separate historical profile. That cross-run observation is context, not a paired speed estimate. Current ON uses 2,003.652 million instructions/request versus 1,419.254 million OFF. Its median per-block aggregate CPU-time/loop-wall ratio is 24.587 versus 22.428 OFF and 22.389 accepted; these are logical CPU-time ratios, not physical core counts. Higher parallel activity accompanies higher total work and worse latency.

The indexed implementation reduces each nonempty bucket tile independently before merging partials. With the observed top occupancy, many short reductions can pay inversion and scheduling costs separately. The source structure and profiles support investigating inversion amortization across buckets; invocation counts and exclusive top-window cycle attribution were not measured here. This exact hybrid is closed. A different reduction kernel would need a new design and correctness/performance gate.

## CPU and Metal scope

The fresh candidate primary comparison follows; M5 values are supplied historical references, not new M5 captures.

| Primary variant | Measured ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| accepted | 8.036072 | 37 | 4.604× | 24 | 2.987× |
| OFF | 8.038908 | 37 | 4.603× | 24 | 2.985× |
| ON | 10.325850 | 37 | 3.583× | 24 | 2.324× |

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

The all-eight CPU 10× goal remains **active and unmet**. Accepted 175 reaches 0/8; this candidate has only one fresh latency row and no all-eight qualification. Primary requires 3.7 ms. Accepted 54→1 remains 162.250464 ms without a supplied M5 reference. The accepted physical 16 baseline-calibration exception remains recorded; this pilot does not rerun or relabel it.

## Verification and provenance

All 165 workflow steps complete. All **7,902 SDK requests** verify internally; all **78 fresh 164-byte exports** independently pass Gnark, exact image/public binding and freshness against 17,552 prior identities. The ledger is **17,630**, with 156 proof/public members representing 78 proofs. Native libraries, shared proc macros, helper/driver, source 177 and all image/cache pins close. Full source including non-Rust files, commands, observations, proofs and decoded profiles are archived. Three owned raw perf recordings and binary/rlib payloads stay local with hashes. No successful capture is repeated.

No new portable/ARM/mobile/TEE, physical 16 latency, giant 250/4999, concurrent-proof, swap, active-pressure, minimum-RAM or AWS/Aeglos result is claimed. [Accepted production](../2026-10-29-balanced-kh-adoption/README.md) remains unchanged. [Design](artifacts/design.md), [history audit](artifacts/history-audit.md), [analysis](artifacts/analysis.json), [build/tests](artifacts/native/result.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [capture completion](artifacts/runs/result.json).

Publication review retains 12 whitespace findings: ten raw test/perf log lines, one inherited Cargo.toml blank line, and one authored operational handoff blank line. Captured and pinned implementation files remain byte-for-byte unchanged. The report interpretation and spacing were edited after generation, then hashes were regenerated and checked.
