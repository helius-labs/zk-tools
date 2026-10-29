# Packed indexed G2 leaves: primary SDK pilot

The declared pilot gate is **false**. Primary at 32 workers measures **8.060157 ms ON versus 8.000355 ms accepted and 8.019030 ms fresh OFF**. Paired ON/accepted is 1.008255470, bounds [1.001971382, 1.014923218], 1/15 wins; ON/OFF is 1.004955973, bounds [1.001335263, 1.011059294], 2/15 wins. Production remains source 175/SDK 20fde; no candidate is adopted.

Failed checks: `["1x2-w32/ON/OFF","1x2-w32/ON/accepted"]`. Paired change versus accepted is +0.826% and versus OFF +0.496%; positive values here mean slower. All observations remain. A passing pilot permits separate full-scope qualification; a failed pilot closes this exact mechanism without an unchanged rerun.

## Change and scope

The candidate preserves accepted phi2 and changes only the native G2 indexed reducer. The original reducer makes one task per nonempty bucket tile. The candidate packs the global ordered index stream into the same 1,024-point tiles, allowing one tile to span adjacent buckets. Each leaf runs the original complete reduce_layer over all of its segments together. Final segment sums stay ordered by original bucket and use original group_affine. This shares inversion work while keeping point-sized parallel tasks. Existing adaptive prefix selection can fire differently; topology and dispatch effects are coupled.

The rejected GLS4 decomposition/maps/windows are absent. Previous ordinary-window tree, SIMD-phi and transformed-X cache experiments remain closed. There is no field formula, scalar split, width/parts, key format, pool, point lookup table or retained setup change. Worker-local limb point copies remain temporary and bounded by the original tile size.

Three new tests check independent point/bucket ownership and tile boundaries; per-bucket generator sums under 1/4/32-thread pools with empty, sparse, skewed, equal/opposite/identity points; and checked cost/invalid geometry/admission denial preserving inputs. Fresh OFF/ON pass all 124/127 core tests and eight prover tests each. All 17 field files are exact, so 33 native field tests are inherited. The official generator check passes.

Dynamic Reservation::analysis charges the full candidate metadata and outputs, final collective grouping scratch and workers times bounded first-stage scratch/local metadata, on top of original forecasts. Segment count is at most ceil(N/tile)+bucketcount, with a tighter N bound. Metadata and partial outputs reserve fallibly before execution. Unsupported/empty/denied/overflow cases use the original reducer without changing inputs. Safe recursive Rayon joins split disjoint MaybeUninit output slices; length is published after all leaves finish. The forecast covers requested heap capacity, not minimum process RAM or worker stack bounds.

Source 176 retains 174 of 175 original files exact. One cfg-only hook and module declaration in msm.rs plus one new child recover the original 175 through inverse edits. Original indexed/layer/grouping bodies, field, Window, phi, solver, FFT and key sources remain exact. [Design](artifacts/design.md), [history](artifacts/history-audit.md), [implementation](artifacts/source/crates/zopro-core/src/msm/packed_index.rs), [source audit](artifacts/source-audit.json).

## Linked route and activation

New packed-index symbols occur only in ON. Before the primary preflight Gnark check and any timing, its wrapper requires 16 packed-entry markers with 128 buckets and 1,024-point tiles: four indexed-top calls per proof times four SDK requests. Timing/profiles have no trace. Selected disassemblies establish linked structure, not dynamic operation counts or speed.

| Variant | Selected packed-index functions | Selected bytes |
|---|---:|---:|
| accepted | 0 | 0 |
| control | 0 | 0 |
| candidate | 10 | 7417 |

## Complete verified requests

On this Ryzen 9950X, every latency process uses 32 workers on CPUs 0–31, measures 151 requests after 3 primes, and exports a fresh proof for independent verification. Fifteen fixed rotated/reversed blocks compare accepted, fresh OFF and ON. All 9 shapes × 3 variants pass preflight Gnark checks before latency; all 45 latency captures precede their 45 Gnark checks. No owned build, helper compilation, hashing, archiving or second workload overlaps captures. The host is shared; exclusive access is not claimed.

| Variant | Median latency ms | CPU ms/request | Median/max process RSS MiB |
|---|---:|---:|---:|
| accepted | 8.000355 | 181.376 | 131.50/135.00 |
| OFF | 8.019030 | 181.559 | 130.25/134.75 |
| ON | 8.060157 | 177.136 | 128.75/131.25 |

Process high-water RSS includes loading/primes; it is not minimum RAM. Displayed medians are not paired-ratio estimates.

| Comparison | Paired ratio [bounds] | Wins | Paired saving µs | Check |
|---|---|---:|---:|---|
| ON/accepted | 1.008255470 [1.001971382, 1.014923218] | 1 | -65.674 | FAIL |
| ON/OFF | 1.004955973 [1.001335263, 1.011059294] | 2 | -39.956 | FAIL |
| OFF/accepted | 1.004021830 [0.997030128, 1.008056239] | 6 | -32.271 | pass |

Accepted SHA256 `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f` and fresh OFF `9c506cea194667e224064c839eaad26878a8d34d1d59b48d9ab48262fe4a1e46` are distinct compiled executables. Pilot primary gains require upper paired ratios below 1 and at least 10/15 wins against both controls; OFF/accepted calibration bounds must lie within [1/1.03,1.03]. Sorted paired-ratio ranks 4–12 of 15 give nominal 96.484375% per-comparison coverage under independent exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusion, sample extension, changed threshold or fastest-layout selection is used.

## CPU profiles

After latency verification, three FIFO-gated four-event stat runs (31 requests) and three separate 199 Hz cycles:u profiles (251 requests), after 3 primes each, precede their six Gnark checks and decoding. FIFO gates exclude loading/primes. Diagnostic times do not enter the latency gate.

| Stat variant | Cycles/request, million | Instructions/request, million | Generic cache misses/request, million | Branch misses/request, million |
|---|---:|---:|---:|---:|
| arith-1x2-w32-OFF | 828.480 | 1418.472 | 2.922 | 1.191 |
| arith-1x2-w32-ON | 807.570 | 1373.138 | 3.122 | 1.164 |
| arith-1x2-w32-accepted | 834.976 | 1421.416 | 2.992 | 1.194 |

| Profile variant | Samples | Lost records |
|---|---:|---:|
| cycles-1x2-w32-OFF | 9204 | 0 |
| cycles-1x2-w32-ON | 8780 | 0 |
| cycles-1x2-w32-accepted | 9020 | 0 |

All stat events run at least 99.9%; each profile contains one SDK PID and zero lost records. Symbol aggregates include surrounding work and inlining; cycle-period shares are not removable wall time or an isolated cost. No exclusive memory/cache/scheduler cause is established.

The candidate reduces aggregate work without improving the requested latency metric. It uses 177.136 CPU ms/request versus 181.559 OFF, 1,373.138 million instructions versus 1,418.472 million, and 807.570 million cycles versus 828.480 million. Median per-block aggregate CPU-time/loop-wall ratios are 22.409 accepted, 22.417 OFF and 21.863 ON; these are logical CPU-time ratios, not physical core counts or a causal critical-path trace.

| Variant | G2 reduce_layer sampled cycle period | Fq inverse sampled cycle period |
|---|---:|---:|
| accepted | 6.124% | 7.340% |
| OFF | 5.816% | 6.586% |
| ON | 4.144% | 5.575% |

The packed fill helper has 26 samples / 0.566% of ON's sampled period. Kernel and inverse shares decrease, consistent with reduced arithmetic work, while the paired latency estimates remain slower against both controls. Aggregate counters and sampled shares do not identify which dependency or scheduling interval causes the wall-time regression. The lower observed maximum RSS also does not establish a minimum-RAM improvement or a throughput result. This exact packing implementation is rejected; no tile-size sweep, unchanged rerun or adoption follows from CPU savings alone.

## CPU and Metal scope

The fresh candidate primary comparison follows; M5 values are supplied historical references, not new M5 captures.

| Primary variant | Measured ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| accepted | 8.000355 | 37 | 4.625× | 24 | 3.000× |
| OFF | 8.019030 | 37 | 4.614× | 24 | 2.993× |
| ON | 8.060157 | 37 | 4.590× | 24 | 2.978× |

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

All 165 workflow steps complete. All **7,902 SDK requests** verify internally; all **78 fresh 164-byte exports** independently pass Gnark, exact image/public binding and freshness against 17,708 prior identities. The ledger is **17,786**, with 156 proof/public members representing 78 proofs. Native libraries, shared proc macros, helper/driver, source 176 and all image/cache pins close. Full source including non-Rust files, commands, observations, proofs and decoded profiles are archived. Three owned raw perf recordings and binary/rlib payloads stay local with hashes. No successful capture is repeated.

No new portable/ARM/mobile/TEE, physical 16 latency, giant 250/4999, concurrent-proof, swap, active-pressure, minimum-RAM or AWS/Aeglos result is claimed. [Accepted production](../2026-10-29-balanced-kh-adoption/README.md) remains unchanged. [Design](artifacts/design.md), [history audit](artifacts/history-audit.md), [analysis](artifacts/analysis.json), [build/tests](artifacts/native/result.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [capture completion](artifacts/runs/result.json).

Publication review retains 11 whitespace findings: ten raw test/perf log lines and one inherited Cargo.toml blank line. None is newly authored source. The report interpretation was added after generation; pinned implementation files and captured data remain exact, and archive hashes were regenerated and checked.
