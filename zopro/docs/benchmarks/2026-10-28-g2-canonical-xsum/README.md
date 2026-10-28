# Canonical X-coordinate sum in G2 prefix addition

The candidate **fails the declared native caller screen and is not adopted**. Primary G2/200 at 32 logical CPUs has ON/OFF ratio **0.978222567**, marginal interval [0.971139284, 0.985508446], 28/31 wins; ON/frozen is **1.008336717**, interval [0.987605403, 1.011346238], 14/31 wins. Production and accepted SDK remain unchanged.

## Change

The prefix reverse loop packs canonical Ax and Bx but uses generic Fq2::lanes_add for their sum. Under FAST=true, route these two base-field sums through existing canonical_add instead. Inputs below p yield a sum below 2p, removing the generic final 2p corrections. FAST=false uses the existing generic fallback. This is separate from the coordinate-scaled R260 model: no representation, wide field primitive, key/bucket layout, table, cache, heap allocation or public option changes.

One runtime expression in msm/g2_prefix.rs changes under private cfg zopro_g2_canonical_xsum; OFF retains its original expression. Original diagnostic adapters are appended in msm/lib. Reversing those edits recovers all 172 accepted sources; 169 originals remain byte-exact. Field, G1, FFT, solver and all other arithmetic are unchanged. Earlier rejected reduced-Gauss and other candidate code is absent.

## Complete caller results

31 fixed rotated/reversed blocks on Ryzen 9950X, independent accepted frozen/fresh OFF/ON executables. Solo uses CPU2, physical16 uses CPUs0–15, logical32 uses CPUs0–31. Each process runs three reset/checked 1024-call primes, then 8192 calls per worker. Ready/start/done/release barriers keep every worker's checking and allocations outside every worker's clock. The wave measurement includes slowest-worker and barrier tails. These synthetic ordinary points at real stream sizes are kernel timings, not full proofs or complete MSMs.

| Caller/pairs | Layout | Frozen µs/wave | OFF µs/wave | ON µs/wave | ON/OFF | Marginal interval | Wins |
|---|---|---:|---:|---:|---:|---|---:|
| g2-prefix/32 | solo | 2.143460 | 2.275623 | 2.269989 | 0.995956948 | [0.990343196, 0.998849087] | 22/31 |
| g2-prefix/32 | physical16 | 2.436216 | 2.441043 | 2.439909 | 0.996836026 | [0.990419504, 1.002611294] | 18/31 |
| g2-prefix/32 | logical32 | 4.397299 | 4.576323 | 4.499629 | 0.986964350 | [0.955316703, 1.010244045] | 20/31 |
| g2-prefix/200 | solo | 8.211749 | 9.390052 | 7.953882 | 0.986774141 | [0.975091864, 0.995073630] | 29/31 |
| g2-prefix/200 | physical16 | 10.317244 | 10.523335 | 10.352946 | 0.980017710 | [0.973018932, 0.992703400] | 29/31 |
| g2-prefix/200 | logical32 | 18.732141 | 19.186211 | 18.766299 | 0.978222567 | [0.971139284, 0.985508446] | 28/31 |
| g1-borrowed-signed/200 | solo | 4.441300 | 4.513090 | 4.807730 | 0.999758933 | [0.995006020, 1.006324906] | 16/31 |
| g1-borrowed-signed/200 | physical16 | 5.223108 | 5.121151 | 5.479966 | 1.058357355 | [1.055500281, 1.071919936] | 0/31 |
| g1-borrowed-signed/200 | logical32 | 9.477738 | 9.447133 | 9.629221 | 1.013914143 | [1.001061787, 1.039011237] | 9/31 |

| Caller/pairs/layout | OFF/frozen | Interval | ON/frozen | Interval |
|---|---:|---|---:|---|
| g2-prefix/32/solo | 1.017562302 | [1.003798317, 1.022445009] | 1.005782151 | [1.003680466, 1.016391177] |
| g2-prefix/32/physical16 | 1.011888544 | [0.999961174, 1.022028962] | 1.006187756 | [1.002046033, 1.012456297] |
| g2-prefix/32/logical32 | 1.038331771 | [1.012868411, 1.047982781] | 1.012348683 | [1.001155454, 1.042315086] |
| g2-prefix/200/solo | 1.010583208 | [1.001780143, 1.022880883] | 0.995346920 | [0.990655345, 1.002290224] |
| g2-prefix/200/physical16 | 1.024832347 | [1.008878574, 1.035407309] | 1.003604528 | [1.002487334, 1.008188357] |
| g2-prefix/200/logical32 | 1.021812585 | [1.016134544, 1.031461186] | 1.008336717 | [0.987605403, 1.011346238] |
| g1-borrowed-signed/200/solo | 0.999932278 | [0.997659872, 1.005208676] | 0.998654244 | [0.995037058, 1.010705056] |
| g1-borrowed-signed/200/physical16 | 0.991951316 | [0.979974480, 0.997839586] | 1.052318147 | [1.045004585, 1.057426085] |
| g1-borrowed-signed/200/logical32 | 0.999030287 | [0.986696868, 1.011708669] | 1.012709903 | [0.999388160, 1.032487272] |

Calibration **false**, nonregression **false**, primary gain **false**. Calibration failures: ["g2-prefix/200/logical32/1-over-0","g2-prefix/200/physical16/1-over-0","g2-prefix/32/logical32/1-over-0"]. Nonregression failures: ["g1-borrowed-signed/200/logical32/2-over-0","g1-borrowed-signed/200/logical32/2-over-1","g1-borrowed-signed/200/physical16/2-over-0","g1-borrowed-signed/200/physical16/2-over-1","g2-prefix/32/logical32/2-over-0"].

All nine OFF/frozen intervals must fit [1/1.03,1.03]; every ON/frozen and ON/OFF upper ratio must be <=1.02. The primary G2/200 logical32 gain requires upper<1 and at least21/31 wins against both controls. The legacy logical32_aggregate field contains only this primary caller. Ordered ranks10–22 of31 have nominal97.055% marginal coverage under independent exchangeable blocks, not familywise coverage. No minimum gain floor, excluded blocks, extension or unchanged reroll is used. All positive estimates remain reported within scope.

## Validation and profiling

Full OFF and ON native core suites each pass 122 tests: 119 accepted tests and three original point-adapter tests. Existing prefix tests cover FAST true/false, tails, scratch reuse and generator sums. Each frozen/OFF/ON harness passes three tests including complete point oracles and concurrency/barrier/affinity checks. Some optional external-fixture tests return when fixtures are absent; those scenarios are not claimed. No fresh field test is claimed; unchanged canonical_add retains its prior qualification. Every native dependency and proc-macro shared object is frozen separately; actual field/core flags and features are checked.

All 837 latency processes finish before four FIFO grouped perf-stat captures, then four separate FIFO199Hz cycles:u profiles, then 12 decode/owner/loss steps. PMU captures use logical32, G2/200 and G1/200 OFF/ON, 262144 calls per worker. No owned build/helper/hash/archive/other benchmark overlaps captures. Host snapshots do not prove exclusive-host use.

All 845 kernel captures verify generator outputs, capacity consistency and zero global timed allocations. There are 179101696 timed kernel calls, not proofs. Per-worker capacity and per-thread CPU/throughput data remain in analysis.json. No measured peak RSS, stack bound or minimum RAM is established.

| Counter case | Cycles/call | Instructions/call | IPC |
|---|---:|---:|---:|
| stat-g1-borrowed-signed-200-m1 | 29446.857988476753 | 52350.297936439514 | 1.777788922571142 |
| stat-g1-borrowed-signed-200-m2 | 29223.40643787384 | 52350.29786360264 | 1.7913824651104364 |
| stat-g2-prefix-200-m1 | 59158.65694499016 | 110878.77772414684 | 1.8742612400286511 |
| stat-g2-prefix-200-m2 | 57652.729570031166 | 109128.6415359974 | 1.8928616624029584 |

All counter events100% running: **true**. Counter observations do not isolate a memory level, execution port or removable proof latency. SMT event cycles are summed logical-thread cycles.

| Profile | Samples | Sampled worker TIDs | Lost |
|---|---:|---:|---:|
| cycles-g1-borrowed-signed-200-m1 | 11297 | 32 | 0 |
| cycles-g1-borrowed-signed-200-m2 | 11170 | 32 | 0 |
| cycles-g2-prefix-200-m1 | 23848 | 32 | 0 |
| cycles-g2-prefix-200-m2 | 23626 | 32 | 0 |

Total 69941 samples, one PID/profile and all32 worker TIDs with zero lost records. Static disassembly counts are archived, not executed work or a causal decomposition. Frozen/OFF executable bytes identical: **false**; all executable hashes are in interpretation.json. Process placement/address layout and host variation are not isolated. Raw profiles and executables stay local with hashes. Four loss dumps use deterministic gzip with verified byte-exact decompression. Captured logs, including trailing blank lines, retain original bytes; authored source/report whitespace is checked separately.

This exact candidate is closed without an SDK trial or production adoption. The broader coordinate-scaled R260 native prototype remains a separate next candidate; this result cannot establish its performance.

## Inherited CPU and Metal comparison

No new full-proof timing or peak RSS is established. Accepted FFT adoption retains its engineering exception and false full-performance/qualified-scope gates. The original all-eight CPU10x objective is active and unmet, 0/8. These accepted local timings and supplied M5 references are inherited.

| Shape | Accepted ms | M5 CPU ms | M5 Metal ms | CPU/accepted | Metal/accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37.0 | 24.0 | 4.445× | 2.883× |
| 2x2 | 11.460675 | 59.0 | 30.0 | 5.148× | 2.618× |
| 4x2 | 15.854365 | 109.0 | 48.0 | 6.875× | 3.028× |
| 8x2 | 24.084626 | 189.0 | 74.0 | 7.847× | 3.072× |
| 12x2 | 33.693397 | 284.0 | 101.0 | 8.429× | 2.998× |
| 16x2 | 39.981636 | 335.0 | 124.0 | 8.379× | 3.101× |
| 24x2 | 66.010670 | 528.0 | 176.0 | 7.999× | 2.666× |
| merge-24x1 | 65.086624 | 507.0 | 175.0 | 7.790× | 2.689× |

Historical primary peak134.50MiB; merge54 remains161.341835ms /1910.1875MiB with no supplied M5 reference. No new SDK/Gnark, portable/mobile/TEE, concurrent-proof, memory-pressure, giant-circuit or AWS/Aeglos measurement is claimed. Independent proof ledger remains13,181.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [native suites](artifacts/native/result.json), [harness tests](artifacts/linked/result.json), [all captures](artifacts/runs/result.json), [interpretation](artifacts/interpretation.json), [code generation](artifacts/codegen.json).
