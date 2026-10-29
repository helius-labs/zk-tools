# Pure Rust three-product Poseidon dot: primary SDK pilot

The declared pilot gate is **false**. Primary at 32 workers measures **8.043779 ms ON versus 8.007450 ms accepted and 8.016157 ms fresh OFF**. Paired ON/accepted is 1.004266627, bounds [1.001021559, 1.009463913], 3/15 wins; ON/OFF is 1.002180073, bounds [0.998212735, 1.005724910], 5/15 wins. Production remains source 175/SDK 20fde; no candidate is adopted.

Failed checks: `["1x2-w32/ON/OFF","1x2-w32/ON/accepted"]`. Paired change versus accepted is +0.427% and versus OFF +0.218%; positive values here mean slower. All observations remain. A passing pilot permits separate full-scope qualification; a failed pilot closes this exact mechanism without an unchanged rerun.

## Change and scope

Only width-three dense Poseidon rows select a new scalar Fr helper that computes three products with one Montgomery reduction. The accepted row used sum2 plus an ordinary product, with two reductions. Partial rounds, S-boxes, constants, recognition, all witness/constraint writes and scheduling are unchanged. Four CIOS rows use safe Rust u128 multiply-adds and five u64 accumulators, followed by the existing canonical correction.

The template has 48 full raw products, 16 modulus products and four low quotient products, versus 48/32/eight for the original sum2+mul expression. These source arithmetic counts are not emitted instructions or speed predictions. No assembly, unsafe block, heap allocation, table, copied key/plan or resource-policy change is added.

Three original parent files differ and one private field module is added: 176 total source files, 172 original files exact, and all 175 originals recovered by reversing the edits. IFMA kernels and representation stay unchanged. Native control passes all 33 field tests; candidate passes all 35, including two new independent tests. Both pass 124 core and eight prover tests. Small-radix coverage checks 28,561 exact integer cancellation cases. Actual-word tests compare both primes with Arkworks and accepted arithmetic, covering modulus/limb boundaries, 4,096 random six-tuples per prime and 16-step dependent chains, with canonical output and carry bounds checked. These are native checks, not a portable or giant-circuit qualification.

[Bounds](artifacts/bounds.md), [new field kernel/tests](artifacts/source/crates/zopro-bn254/src/sop3.rs), [history audit](artifacts/history-audit.md).

## Linked solver code

The actual SDK dense function changes and the original task caller still calls it. Source/cfg/fingerprint qualification and assembly are retained. Selected code sizes do not establish instruction-cache pressure or performance.

| Variant | Dense bytes | Task caller bytes | Dense call sites |
|---|---:|---:|---:|
| accepted | 4196 | 10010 | 6 |
| control | 4196 | 10010 | 6 |
| candidate | 2558 | 10010 | 6 |

## Complete verified requests

On this Ryzen 9950X, every latency process uses 32 workers on CPUs 0–31, measures 151 requests after 3 primes, and exports a fresh proof for independent verification. Fifteen fixed rotated/reversed blocks compare accepted, fresh OFF and ON. All 9 shapes × 3 variants pass preflight Gnark checks before latency; all 45 latency captures precede their 45 Gnark checks. No owned build, helper compilation, hashing, archiving or second workload overlaps captures. The host is shared; exclusive access is not claimed.

| Variant | Median latency ms | CPU ms/request | Median/max process RSS MiB |
|---|---:|---:|---:|
| accepted | 8.007450 | 181.202 | 130.25/133.75 |
| OFF | 8.016157 | 181.949 | 130.75/135.50 |
| ON | 8.043779 | 181.446 | 130.25/133.50 |

Process high-water RSS includes loading/primes; it is not minimum RAM. Displayed medians are not paired-ratio estimates.

| Comparison | Paired ratio [bounds] | Wins | Paired saving µs | Check |
|---|---|---:|---:|---|
| ON/accepted | 1.004266627 [1.001021559, 1.009463913] | 3 | -34.174 | FAIL |
| ON/OFF | 1.002180073 [0.998212735, 1.005724910] | 5 | -17.402 | FAIL |
| OFF/accepted | 1.002813854 [0.996081336, 1.007989010] | 6 | -22.493 | pass |

Accepted SHA256 `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f` and fresh OFF `0ae84275b85e51edfcc66ef6fda968d43bdc658a1b3301d630be3c6f00d1cc71` are distinct compiled executables. Pilot primary gains require upper paired ratios below 1 and at least 10/15 wins against both controls; OFF/accepted calibration bounds must lie within [1/1.03,1.03]. Sorted paired-ratio ranks 4–12 of 15 give nominal 96.484375% per-comparison coverage under independent exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusion, sample extension, changed threshold or fastest-layout selection is used.

## CPU profiles

After latency verification, three FIFO-gated four-event stat runs (31 requests) and three separate 199 Hz cycles:u profiles (251 requests), after 3 primes each, precede their six Gnark checks and decoding. FIFO gates exclude loading/primes. Diagnostic times do not enter the latency gate.

| Stat variant | Cycles/request, million | Instructions/request, million | Generic cache misses/request, million | Branch misses/request, million |
|---|---:|---:|---:|---:|
| arith-1x2-w32-OFF | 830.688 | 1420.683 | 2.763 | 1.187 |
| arith-1x2-w32-ON | 820.060 | 1421.434 | 2.770 | 1.199 |
| arith-1x2-w32-accepted | 834.644 | 1420.892 | 2.822 | 1.196 |

| Profile variant | Samples | Lost records |
|---|---:|---:|
| cycles-1x2-w32-OFF | 8999 | 0 |
| cycles-1x2-w32-ON | 9134 | 0 |
| cycles-1x2-w32-accepted | 8998 | 0 |

All stat events run at least 99.9%; each profile contains one SDK PID and zero lost records. Symbol aggregates include surrounding work and inlining; cycle-period shares are not removable wall time or an isolated cost. No exclusive memory/cache/scheduler cause is established.

The changed dense symbol has only three accepted, six OFF and three ON samples, accounting for 0.028%, 0.045% and 0.016% of their respective sampled cycle periods. These sparse observations cannot estimate an isolated kernel improvement or explain the measured proof-time difference. The single diagnostic counter runs also do not replace the paired latency result. The exact dense-only integration is rejected; no speedup is inferred from its lower source arithmetic count or smaller linked function.

## CPU and Metal scope

The fresh candidate primary comparison follows; M5 values are supplied historical references, not new M5 captures.

| Primary variant | Measured ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| accepted | 8.007450 | 37 | 4.621× | 24 | 2.997× |
| OFF | 8.016157 | 37 | 4.616× | 24 | 2.994× |
| ON | 8.043779 | 37 | 4.600× | 24 | 2.984× |

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

All 165 workflow steps complete. All **7,902 SDK requests** verify internally; all **78 fresh 164-byte exports** independently pass Gnark, exact image/public binding and freshness against 17,395 prior identities. The ledger is **17,473**, with 156 proof/public members representing 78 proofs. Native libraries, shared proc macros, helper/driver, source 176 and all image/cache pins close. Full source including non-Rust files, commands, observations, proofs and decoded profiles are archived. Three owned raw perf recordings and binary/rlib payloads stay local with hashes. No successful capture is repeated.

No new portable/ARM/mobile/TEE, physical 16 latency, giant 250/4999, concurrent-proof, swap, active-pressure, minimum-RAM or AWS/Aeglos result is claimed. [Accepted production](../2026-10-29-balanced-kh-adoption/README.md) remains unchanged. [Design](artifacts/design.md), [history audit](artifacts/history-audit.md), [analysis](artifacts/analysis.json), [build/tests](artifacts/native/result.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [capture completion](artifacts/runs/result.json).

Publication review retains thirteen whitespace findings in raw outputs and original source, preserving those bytes; authored report and helper files have none. The sparse-profile interpretation was added after report generation from the archived analysis, without changing captured data or pinned inputs. The final report and archive are covered by `SHA256SUMS`.
