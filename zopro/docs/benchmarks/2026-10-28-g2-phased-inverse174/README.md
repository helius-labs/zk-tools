# G2 norm inversion before point finishing

The candidate is **not adopted**. Primary32 has a paired ON/OFF point estimate 0.585% slower, with bounds spanning parity and 2/7 wins. Both primary gain checks fail; the other five required checks pass. Positive 16x2 and merge54/OFF estimates are retained with their uncertainty. See the [interpretation](artifacts/interpretation.md).

The private candidate uses four inversion-prefix chains and materializes inverse norms before point finishing. Primary32 ON/OFF paired ratio is 1.005845557, bounds [0.978894774, 1.023659325], with 2/7 wins. The prospective pilot gate is **false**. Production remains unchanged.

For borrowed G2 batches of at least64 points, the algorithm first gathers norms into the existing two-slot records, computes four interleaved prefix chains, combines and inverts their roots through the accepted scalar-tail/inversion code, then overwrites dead prefix slots with inverse norms. The final pass uses those inverses to complete each point. Immutable key/input points, distinct destination contracts, field formulas, signs and tails remain. Smaller batches retain the original path.

It adds nine vector products per selected batch, extra scratch passes and stack temporaries. This tests execution overlap and live values, not fewer mathematical products. There is no new heap buffer/header, table, point-key copy, plan, pool, scalar preparation, allocator or forecast change. Equal heap capacities do not prove equal stack, peak RSS or pressure behavior. Field and G1 sources remain exact. Two original files have reversible declaration/dispatch edits;172 originals remain exact, with one new module giving175 candidate files. OFF retains the original body and old line numbers. The private cfg selects ON only in native supported code.

Both modes pass124 unfiltered native core tests and8 prover tests. New tests check stored inverse norms, complete point results, both canonical/inverse modes, scalar tails and retained-record reuse. Existing complete-G2/MSM generator oracles remain. Field source and its qualification are inherited; no new field suite, portable or forced-pressure build is claimed. Optional absent external fixtures are not claimed. Coherent native rlibs/proc macros and actual flags/features are frozen and closing pins checked. OFF, ON and retained accepted are separate executable paths; identities.json records their exact hashes.

Seven fixed rotated/reversed blocks cover primary16/32,16x2/32 and merge54/32,151 primary or31 other requests after3 primes. Loading is excluded; solving, fresh proving, SDK verification and serialization are included. No owned build/helper/hash/archive/verifier/profile/second benchmark overlaps capture. This is a shared host. Raw labels accept/offset/bypass mean accepted/OFF/ON, without a grouping bypass.

| Cell | Variant | Median ms | CPU ms/request | Median peak MiB | Max peak MiB |
|---|---|---:|---:|---:|---:|
| 16x2-w32 | accept | 41.920122 | 1127.589 | 808.00 | 820.50 |
| 16x2-w32 | bypass | 41.828640 | 1122.943 | 789.75 | 818.52 |
| 16x2-w32 | offset | 41.718742 | 1122.658 | 784.00 | 830.25 |
| 1x2-w16 | accept | 10.572059 | 122.717 | 88.75 | 92.50 |
| 1x2-w16 | bypass | 10.592528 | 124.159 | 87.50 | 89.75 |
| 1x2-w16 | offset | 10.825898 | 123.357 | 87.25 | 89.75 |
| 1x2-w32 | accept | 8.456249 | 188.096 | 130.25 | 132.75 |
| 1x2-w32 | bypass | 8.566165 | 189.603 | 131.25 | 132.00 |
| 1x2-w32 | offset | 8.471277 | 188.298 | 129.75 | 131.25 |
| merge-54x1-w32 | accept | 166.921531 | 4382.497 | 1751.43 | 1783.26 |
| merge-54x1-w32 | bypass | 166.288072 | 4391.555 | 1782.90 | 1839.86 |
| merge-54x1-w32 | offset | 167.149065 | 4409.559 | 1808.00 | 1903.18 |

RSS includes loading/primes; it is not minimum required RAM.

| Pair | Median ratio | Lower | Upper | Wins | Paired saving ms | Required check |
|---|---:|---:|---:|---:|---:|---|
| 16x2-w32/bypass/accept | 0.994253355 | 0.968125402 | 1.009891668 | 5/7 | 0.239713 | pass |
| 16x2-w32/bypass/offset | 0.994182039 | 0.986070680 | 1.022874540 | 5/7 | 0.242718 | pass |
| 16x2-w32/offset/accept | 0.997780771 | 0.951915920 | 1.012426378 | 4/7 | 0.094348 | descriptive |
| 1x2-w16/bypass/accept | 0.989681764 | 0.960081038 | 1.040826319 | 4/7 | 0.109085 | descriptive |
| 1x2-w16/bypass/offset | 0.976845062 | 0.924842315 | 1.060875548 | 5/7 | 0.250673 | descriptive |
| 1x2-w16/offset/accept | 1.016906220 | 0.965054489 | 1.069696411 | 3/7 | -0.181402 | descriptive |
| 1x2-w32/bypass/accept | 1.012307345 | 0.975418804 | 1.024760416 | 3/7 | -0.104145 | fail |
| 1x2-w32/bypass/offset | 1.005845557 | 0.978894774 | 1.023659325 | 2/7 | -0.049783 | fail |
| 1x2-w32/offset/accept | 1.006424234 | 0.973314800 | 1.020901307 | 3/7 | -0.054362 | pass |
| merge-54x1-w32/bypass/accept | 1.003436390 | 0.981569693 | 1.007184689 | 3/7 | -0.574594 | pass |
| merge-54x1-w32/bypass/offset | 0.994848951 | 0.984363676 | 1.007323036 | 5/7 | 0.860993 | pass |
| merge-54x1-w32/offset/accept | 1.004103061 | 0.994242630 | 1.020322971 | 2/7 | -0.681567 | descriptive |

Failed required checks: `["1x2-w32/bypass/offset","1x2-w32/bypass/accept"]`. Other OFF/accepted calibration results are separately preserved in analysis.json.

Bounds are seven-ratio extrema, nominal98.4375% under independent exchangeable blocks, without familywise coverage. Primary32 requires upper<1 and at least5/7 wins against both controls, plus OFF/accepted bounds withinfactor1.03. Larger ON comparisons require upper<=1.03; physical16 is descriptive. No minimum gain floor, exclusion, extension, successful-capture repeat or changed threshold. Passing would select further all-eight/clean/portable/resource qualification only.

After all latency and its verification,6 FIFO-gated four-event PMU captures and6 separate199Hz cycles profiles cover primary16/32 and all variants. Counters use31 requests; profiles251, after3 primes. All diagnostic captures precede their verification and decoding. Perf subprocesses run as root; latency runs as the normal user. FIFO control excludes loading/primes. All event running times meet99.9%; every profile has one sampled PID and zero lost events. Diagnostic times do not enter latency decisions. Single counter/profile observations do not have the latency intervals or isolate ports/cache/causes. Sampled shares are not removable wall-time fractions.

| Counter process | Event | Count/request | Running % |
|---|---|---:|---:|
| arith-1x2-w16-accept | branch-misses:u | 1138091.129 | 100.00 |
| arith-1x2-w16-accept | cache-misses:u | 2133578.968 | 100.00 |
| arith-1x2-w16-accept | cycles:u | 587671867.839 | 100.00 |
| arith-1x2-w16-accept | instructions:u | 1411320307.065 | 100.00 |
| arith-1x2-w16-bypass | branch-misses:u | 1117266.097 | 100.00 |
| arith-1x2-w16-bypass | cache-misses:u | 2091952.742 | 100.00 |
| arith-1x2-w16-bypass | cycles:u | 598334997.742 | 100.00 |
| arith-1x2-w16-bypass | instructions:u | 1415721568.032 | 100.00 |
| arith-1x2-w16-offset | branch-misses:u | 1147769.710 | 100.00 |
| arith-1x2-w16-offset | cache-misses:u | 2067265.129 | 100.00 |
| arith-1x2-w16-offset | cycles:u | 592387932.710 | 100.00 |
| arith-1x2-w16-offset | instructions:u | 1411314292.645 | 100.00 |
| arith-1x2-w32-accept | branch-misses:u | 1251140.774 | 100.00 |
| arith-1x2-w32-accept | cache-misses:u | 3009135.935 | 100.00 |
| arith-1x2-w32-accept | cycles:u | 855221857.194 | 100.00 |
| arith-1x2-w32-accept | instructions:u | 1456055107.032 | 100.00 |
| arith-1x2-w32-bypass | branch-misses:u | 1249311.935 | 100.00 |
| arith-1x2-w32-bypass | cache-misses:u | 3219017.968 | 100.00 |
| arith-1x2-w32-bypass | cycles:u | 858599752.935 | 100.00 |
| arith-1x2-w32-bypass | instructions:u | 1462450705.871 | 100.00 |
| arith-1x2-w32-offset | branch-misses:u | 1271698.065 | 100.00 |
| arith-1x2-w32-offset | cache-misses:u | 3013290.032 | 100.00 |
| arith-1x2-w32-offset | cycles:u | 852084854.677 | 100.00 |
| arith-1x2-w32-offset | instructions:u | 1459464907.548 | 100.00 |

| Profile | Samples | Sampled TIDs | Lost |
|---|---:|---:|---:|
| cycles-1x2-w16-accept | 5366 | 17 | 0 |
| cycles-1x2-w16-bypass | 5498 | 17 | 0 |
| cycles-1x2-w16-offset | 5490 | 17 | 0 |
| cycles-1x2-w32-accept | 8792 | 33 | 0 |
| cycles-1x2-w32-bypass | 8801 | 33 | 0 |
| cycles-1x2-w32-offset | 8481 | 33 | 0 |

All12 preflight exports independently verify before84 latency processes; their84 final exports verify afterward. All12 diagnostic exports verify after diagnostic captures. Total9672 internally verified SDK requests,108 fresh164-byte independent Gnark exports,216 proof/public members; ledger15433→15541. Complete candidate source and decoded logs are archived; raw perf stays local with SHA256 receipts.

The inspected [pinned Aeglos source](https://github.com/helius-labs/aeglos/blob/0290613e8eeca17a4ce88f6f2d726e8e06a5385d/cpu/msm_impl.hpp) uses four independent inversion chains and materializes inverses before point finishing. That motivates this different memory/dataflow application; no Aeglos runtime or speed result is imported. Only source URLs/hashes are archived. Earlier [G1 chains](../2026-10-22-universal-g1-prefix-chains-sdk/README.md) and [owned G2 finishing](../2026-10-26-g2-owned-finish/README.md) remain rejected under their original gates. This candidate leaves incoming points immutable and separates norm inversion rather than output-Y finishing. The [norm history correction](../2026-10-28-symmetric-norm174/artifacts/history-correction.md) remains explicit; symmetric norm arithmetic is absent here.

New reference comparisons cover only1x2 and16x2. M5 values are supplied historical figures, not matched-machine measurements.

| Cell | Variant | ms | M5 CPU speedup | M5 Metal speedup |
|---|---|---:|---:|---:|
| 16x2-w32 | accept | 41.920122 | 7.991× | 2.958× |
| 16x2-w32 | bypass | 41.828640 | 8.009× | 2.964× |
| 16x2-w32 | offset | 41.718742 | 8.030× | 2.972× |
| 1x2-w32 | accept | 8.456249 | 4.375× | 2.838× |
| 1x2-w32 | bypass | 8.566165 | 4.319× | 2.802× |
| 1x2-w32 | offset | 8.471277 | 4.368× | 2.833× |

The complete accepted-source all-eight table below is inherited. Accepted174/SDK81033 remains unchanged, primary8.366931ms. The original10× CPU goal remains active/unmet,0/8. Inherited adoption exceptions, physical16 uncertainty and baseline-only forced-pressure timeout remain in the [adoption report](../2026-10-28-g2-borrowed-adoption/README.md). No54 M5 reference was supplied. There is no new AWS/Aeglos, mobile/ARM/TEE, giant, minimum-RAM, swap, concurrent-proof or pressure qualification.

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

[Protocol](artifacts/design.md), [all observations/profiles](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [native build](artifacts/native/result.json), [proof inventory](artifacts/proof-inventory.json), [closing pins](artifacts/pins.json).
