# Diagonal Fq Montgomery multiplication: primary SDK pilot

The declared pilot gate is **false**. Primary at 32 workers measures **9.040826 ms ON versus 8.021484 ms accepted and 8.140078 ms fresh OFF**. Paired ON/accepted is 1.118868387, bounds [1.112014559, 1.130207512], 0/15 wins; ON/OFF is 1.111189587, bounds [1.099318924, 1.117565285], 0/15 wins. Production remains source 175/SDK 20fde; no candidate is adopted.

Failed checks: `["1x2-w32/ON/OFF","1x2-w32/ON/accepted"]`. Paired change versus accepted is +11.887% and versus OFF +11.119%; positive values here mean slower. All observations remain. A passing pilot permits separate full-scope qualification; a failed pilot closes this exact mechanism without an unchanged rerun.

## Change and scope

Eight-lane Fq/R256 ordinary and fused products use anti-diagonal product scanning interleaved with Montgomery cancellation. Fr, R260, four-lane, corrections, field representation, core consumers and resource policies retain their accepted paths. Only the IFMA template and its officially generated eight-lane file differ among 175 originals; 173 are exact and the reverse check restores all 175. The original CIOS body is unchanged.

There are no new heap allocations, tables or copied keys/points. The kernel uses fixed vector locals, with the existing inline policy. Nominal IFMA counts remain 100 ordinary / 150 fused; new dependency order and register pressure can lose. No speed claim follows from operation counts.

An independently derived mixed-radix interpreter passes 38,176 exact BigUint comparisons and 32 cancellation identities, including both primes/radices as mathematical controls. Native dispatch remains only Fq/R256/eight lanes. Both native builds freshly pass 33 field, 124 core and 8 prover tests. The candidate adds Arkworks ordinary/fused comparisons and 16-product chains to the shared carry test; the generator check passes. This qualification is native only.

[ZKPU](https://arxiv.org/html/2610.27530v1), sections 2 and 3.4, suggested reconsidering FIPS dataflow. Its results describe hardware. The mixed-radix CPU implementation and tests here are independently derived; no implementation was copied and no hardware speedup is transferred to CPUs. See [algebra and bounds](artifacts/algebra.md) and [model receipt](artifacts/model.json).

## Linked code before timing

Source, cfg fingerprints, native test qualification and selected actual G1/G2 IFMA caller disassembly are recorded. Both caller implementations change in the linked SDK. This is a structural observation; no isolated arithmetic cost, dynamic instruction count or stack bound is inferred.

| Variant | Selected G1 bytes | Selected G2 bytes | Static IFMA sites |
|---|---:|---:|---:|
| accepted | 26230 | 51950 | 2250 |
| control | 26230 | 51950 | 2250 |
| candidate | 26096 | 51948 | 2250 |

## Complete verified requests

On this Ryzen 9950X, every latency process uses 32 workers on CPUs 0–31, measures 151 requests after 3 primes, and exports a fresh proof for independent verification. Fifteen fixed rotated/reversed blocks compare accepted, fresh OFF and ON. All 9 shapes × 3 variants pass preflight Gnark checks before latency; all 45 latency captures precede their 45 Gnark checks. No owned build, helper compilation, hashing, archiving or second workload overlaps captures. The host is shared; exclusive access is not claimed.

| Variant | Median latency ms | CPU ms/request | Median/max process RSS MiB |
|---|---:|---:|---:|
| accepted | 8.021484 | 182.081 | 131.50/134.00 |
| OFF | 8.140078 | 184.044 | 128.75/132.75 |
| ON | 9.040826 | 203.673 | 129.00/133.50 |

Process high-water RSS includes loading/primes; it is not minimum RAM. Displayed medians are not paired-ratio estimates.

| Comparison | Paired ratio [bounds] | Wins | Paired saving µs | Check |
|---|---|---:|---:|---|
| ON/accepted | 1.118868387 [1.112014559, 1.130207512] | 0 | -951.344 | FAIL |
| ON/OFF | 1.111189587 [1.099318924, 1.117565285] | 0 | -885.540 | FAIL |
| OFF/accepted | 1.011312294 [1.005839808, 1.017085244] | 2 | -90.490 | pass |

Accepted SHA256 `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f` and fresh OFF `56ac98f995e4d5bec3157c54bca6d6bc0ef6c29d9dce04813718ae9788a1c632` are distinct compiled executables. Pilot primary gains require upper paired ratios below 1 and at least 10/15 wins against both controls; OFF/accepted calibration bounds must lie within [1/1.03,1.03]. Sorted paired-ratio ranks 4–12 of 15 give nominal 96.484375% per-comparison coverage under independent exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusion, sample extension, changed threshold or fastest-layout selection is used.

## CPU profiles

After latency verification, three FIFO-gated four-event stat runs (31 requests) and three separate 199 Hz cycles:u profiles (251 requests), after 3 primes each, precede their six Gnark checks and decoding. FIFO gates exclude loading/primes. Diagnostic times do not enter the latency gate.

| Stat variant | Cycles/request, million | Instructions/request, million | Generic cache misses/request, million | Branch misses/request, million |
|---|---:|---:|---:|---:|
| arith-1x2-w32-OFF | 843.961 | 1419.777 | 2.738 | 1.190 |
| arith-1x2-w32-ON | 952.863 | 1416.930 | 3.019 | 1.196 |
| arith-1x2-w32-accepted | 826.753 | 1420.396 | 2.806 | 1.197 |

| Profile variant | Samples | Lost records |
|---|---:|---:|
| cycles-1x2-w32-OFF | 9077 | 0 |
| cycles-1x2-w32-ON | 9905 | 0 |
| cycles-1x2-w32-accepted | 9102 | 0 |

All stat events run at least 99.9%; each profile contains one SDK PID and zero lost records. Symbol aggregates include surrounding work and inlining; cycle-period shares are not removable wall time or an isolated cost. No exclusive memory/cache/scheduler cause is established.

The candidate uses about 12.9% more cycles than fresh OFF while retiring about 0.2% fewer instructions in the diagnostic runs. Its selected G1 caller is slightly smaller, and the static IFMA count is unchanged. Neither observation predicts the full-proof result. The G1 caller accounts for 23.86% of candidate sampled cycle periods versus 19.49% for OFF, including its surrounding work. These measurements support rejecting this implementation; they do not isolate dependency latency, spills, or a memory cause. The exact kernel is closed without an unchanged rerun.

## CPU and Metal scope

The fresh candidate primary comparison follows; M5 values are supplied historical references, not new M5 captures.

| Primary variant | Measured ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| accepted | 8.021484 | 37 | 4.613× | 24 | 2.992× |
| OFF | 8.140078 | 37 | 4.545× | 24 | 2.948× |
| ON | 9.040826 | 37 | 4.093× | 24 | 2.655× |

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

All 165 workflow steps complete. All **7,902 SDK requests** verify internally; all **78 fresh 164-byte exports** independently pass Gnark, exact image/public binding and freshness against 17,317 prior identities. The ledger is **17,395**, with 156 proof/public members representing 78 proofs. Native libraries, shared proc macros, helper/driver, source 175 and all image/cache pins close. Full source including non-Rust files, commands, observations, proofs and decoded profiles are archived. Three owned raw perf recordings and binary/rlib payloads stay local with hashes. No successful capture is repeated.

No new portable/ARM/mobile/TEE, physical 16 latency, giant 250/4999, concurrent-proof, swap, active-pressure, minimum-RAM or AWS/Aeglos result is claimed. [Accepted production](../2026-10-29-balanced-kh-adoption/README.md) remains unchanged. [Design](artifacts/design.md), [history audit](artifacts/history-audit.md), [analysis](artifacts/analysis.json), [build/tests](artifacts/native/result.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [capture completion](artifacts/runs/result.json).

Publication review preserves thirteen whitespace findings in raw test/perf outputs and original source, plus the intentional blank insertion separator at the end of the pinned kernel fragment. These inputs remain byte-exact. The CPU interpretation paragraph was added after report generation from the archived analysis; captured observations and pinned inputs were unchanged. The final report and all archived files are covered by `SHA256SUMS`.
