# Native coordinate-scaled R260 G2 batches

The candidate **fails the declared complete native caller screen and is not adopted**. Primary G2/200 at32 logical CPUs has ON/OFF ratio **1.303592414**, marginal interval [1.272994855,1.346972922], 0/31 wins; ON/frozen **1.203985993**, interval [1.183757226,1.263044717], 0/31 wins. Production and the accepted SDK remain unchanged.

## Arithmetic and storage

Port the passed exact screen to actual AVX512 IFMA using (x,y) → (x/4,y/8), uniform Montgomery R=2^260=16H and host radix H=2^256. Canonical host raw coordinates import as4a/2a; outputs use modular quarter/half followed by canonicalization. Keep the original two-fused-product complex formulas. Wider temporaries survive several linear operations instead of repeated generic corrections: imported X4p/Y2p, dx8p/dy4p, norm3p, prefix/slope2p, square4p, Xout12p, finishing difference16p and Yout4p. Bounds are inclusive in the native types; strict product inequalities follow from64p<R.

A separate bounded child module calls the existing private uniform CIOS implementation for the proven wider domains. Public get/compact/difference2 contracts and original field runtime stay unchanged. Native codecs, five-product central inverse tree, host/root conversion, packing and scattering are inside the caller. No persistent recoded key, lookup table or second key plan is created. Identity/equal-x batches take a complete scalar fallback before vector mutation. Existing distinct queued-index contract remains; incomplete8lane groups use the original path.

The typed prefix Vec has640B per8point record, the same active record size as before. A24B optional Vec field is added to experimental scratch. Old and new prefix capacities can coexist after mixed complete/tail calls; this is a real memory cost. Ordinary caller capacity includes the new vector; no unchanged-peak-RSS/minimum-RAM or universal zero-overhead claim is made.

A private accepted172-source copy adds only cfg-only dispatch/scratch/module declarations and diagnostic capacity accounting.168 originals remain byte-exact; reversing edits recovers all172. The complete accepted OFF prefix body remains verbatim. Frozen and fresh OFF code and constants are checked byte-identical before timing; only build ID and20 verified panic source-line fields differ, addressing the prior canonical-Xsum control setup error. This is a distinct wider-arithmetic candidate; rejected Gauss/GCD/GLV/weighted/square/FFT candidate runtime is absent.

## Native correctness

The full field suite passes33 tests with debug and overflow checking. Full core suites pass122 OFF and124 ON:119 accepted tests,3 original point-adapter tests, plus2 ON point tests. New native point coverage checks2560 full-width-generator outputs at sizes8/16/32/64/200 over8 changing rounds, scratch reuse, four whole-call exceptional batches and an empty case. Inherited tests cover original tails and FAST true/false. Optional external-fixture cases that return early are not claimed.

Three separate tests against the actual frozen ON field library compare42,885 wider-domain CIOS scalar tuples with independent BigUint full-radix REDC, check2076 canonical raw codec inputs for both X/Y, and16,462 wider export/alias cases including inclusive bounds. Padding into8lane groups repeats inputs and is not extra distinct coverage. This upgrades the scalar model to actual native instruction validation. Complete complex/linear chains are also exercised by the native point oracles.

Initial setup failed before builds because the reverse-source audit tried to decode unchanged binary fixtures as UTF-8. The repair skips textual decoding for already SHA-exact files; arithmetic is unchanged. Original helper and partial copy are preserved. The driver disassembly audit is extended before linking to include new scaled apply symbols. The independent numeric harness first fails compilation at one borrowed BigUint fixture entry; p.clone() repairs only that test value. Nine completed links/harness checks and all native suites are preserved; only the unfinished numeric build/test runs under revised pins. The strict ELF preflight then stops before capture. A section/relocation audit proves all runtime bytes identical except20 Rust panic source-line fields (each+2) and build ID. Before any timing, the preflight is refined to accept only this verified metadata difference; original preflight/failures are preserved and every performance gate remains unchanged. See numeric-repair.json, control-locations.json and preflight-revision.json. Successful tests and captures are not repeated for bookkeeping.

## Caller timing

31 fixed rotated/reversed blocks, G2prefix32/200 and unchangedG1borrow200, soloCPU2/physical16CPUs0–15/logical32CPUs0–31 on Ryzen9950X. Three reset checked1024call primes precede8192timedcalls/worker. Ready/start/done/release barriers put checking/allocation outside every worker's clock; wave time includes slowest worker and barrier tail. These synthetic ordinary points at real stream sizes are point-kernel timings, not full proofs or whole MSMs.

| Caller/pairs | Layout | Frozen µs/wave | OFF µs/wave | ON µs/wave | ON/OFF | Marginal interval | Wins |
|---|---|---:|---:|---:|---:|---|---:|
| g2-prefix/32 | solo | 1.981922 | 1.869762 | 2.085421 | 1.117551822 | [1.111333770,1.119878296] | 1/31 |
| g2-prefix/32 | physical16 | 2.623275 | 2.342196 | 2.916028 | 1.246003989 | [1.236577539,1.250398663] | 0/31 |
| g2-prefix/32 | logical32 | 4.434796 | 4.140190 | 4.972371 | 1.199080004 | [1.167584238,1.211037312] | 0/31 |
| g2-prefix/200 | solo | 7.951619 | 7.804505 | 8.972401 | 1.150390802 | [1.148207139,1.154709026] | 1/31 |
| g2-prefix/200 | physical16 | 11.295828 | 9.689745 | 13.395702 | 1.383291702 | [1.376311438,1.388788776] | 0/31 |
| g2-prefix/200 | logical32 | 18.570241 | 17.587873 | 22.558492 | 1.303592414 | [1.272994855,1.346972922] | 0/31 |
| g1-borrowed-signed/200 | solo | 4.357551 | 4.335971 | 4.323691 | 0.996639176 | [0.993033866,1.000013854] | 21/31 |
| g1-borrowed-signed/200 | physical16 | 5.092186 | 4.986335 | 5.014799 | 1.004332637 | [1.000084531,1.010293377] | 9/31 |
| g1-borrowed-signed/200 | logical32 | 9.075653 | 9.022705 | 9.038924 | 1.010126771 | [0.980411751,1.033865898] | 13/31 |

| Caller/pairs/layout | OFF/frozen | Interval | ON/frozen | Interval |
|---|---:|---|---:|---|
| g2-prefix/32/solo | 0.941632300 | [0.939820660,0.946402964] | 1.053093674 | [1.047108639,1.057669884] |
| g2-prefix/32/physical16 | 0.893430120 | [0.886920429,0.898310061] | 1.112282225 | [1.100925320,1.116730442] |
| g2-prefix/32/logical32 | 0.946149328 | [0.928676366,0.967206415] | 1.117643727 | [1.064701847,1.141902318] |
| g2-prefix/200/solo | 0.980779856 | [0.978760283,0.986678892] | 1.129023604 | [1.125411367,1.132960231] |
| g2-prefix/200/physical16 | 0.868620013 | [0.858234947,0.874338681] | 1.192889910 | [1.181414798,1.209465443] |
| g2-prefix/200/logical32 | 0.934888447 | [0.922981278,0.979671523] | 1.203985993 | [1.183757226,1.263044717] |
| g1-borrowed-signed/200/solo | 0.995797822 | [0.993360601,1.004535132] | 0.992507981 | [0.991411166,0.997789765] |
| g1-borrowed-signed/200/physical16 | 0.993968319 | [0.986131322,0.998285017] | 0.996910401 | [0.992114552,0.998810420] |
| g1-borrowed-signed/200/logical32 | 0.989892884 | [0.959614065,1.010627940] | 0.994229528 | [0.978594393,1.021069139] |

Calibration **false**; nonregression **false**; primary gain **false**. Calibration failures: ["g1-borrowed-signed/200/logical32/1-over-0","g2-prefix/200/logical32/1-over-0","g2-prefix/200/physical16/1-over-0","g2-prefix/32/logical32/1-over-0","g2-prefix/32/physical16/1-over-0","g2-prefix/32/solo/1-over-0"]. Nonregression failures: ["g1-borrowed-signed/200/logical32/2-over-0","g1-borrowed-signed/200/logical32/2-over-1","g2-prefix/200/logical32/2-over-0","g2-prefix/200/logical32/2-over-1","g2-prefix/200/physical16/2-over-0","g2-prefix/200/physical16/2-over-1","g2-prefix/200/solo/2-over-0","g2-prefix/200/solo/2-over-1","g2-prefix/32/logical32/2-over-0","g2-prefix/32/logical32/2-over-1","g2-prefix/32/physical16/2-over-0","g2-prefix/32/physical16/2-over-1","g2-prefix/32/solo/2-over-0","g2-prefix/32/solo/2-over-1"].

All9OFF/frozen intervals must fit[1/1.03,1.03], all18 ON/frozen and ON/OFF upper ratios<=1.02. The primary G2/200 logical32 gain requires upper<1 and>=21/31 wins against both controls; legacy logical32_aggregate contains only this primary caller. Ordered ranks10–22 of31 give nominal97.055% marginal coverage under independent exchangeable blocks, not familywise. No gain floor, exclusions, extension or unchanged reroll. All positive estimates and uncertainty remain within their measured scope. Identical control code and constants exclude different compiled arithmetic as a control gap; host variation, barrier tails and process placement/address layout remain unresolved.

## Counters, profiles and provenance

All837 latency processes complete before4FIFO grouped perf-stat, then4 separate FIFO199Hz cycles:u profiles(G2/200,G1/200 OFF/ON logical32,262144calls/worker), then12owner/decode/loss steps.845kernel captures/857workflow steps verify complete generator results, capacity consistency and zero global timed allocations.179,101,696timed kernel calls are not proofs. No owned helper/build/hash/archive/other benchmark overlaps captures; host snapshots do not prove exclusive use.

| Counter case | Cycles/call | Instructions/call | IPC |
|---|---:|---:|---:|
| stat-g1-borrowed-signed-200-m1 | 29672.792424440384 | 52350.29860532284 | 1.7642525130935716 |
| stat-g1-borrowed-signed-200-m2 | 29574.804127573967 | 52350.297850608826 | 1.7700978719855731 |
| stat-g2-prefix-200-m1 | 59377.5508518219 | 109428.07943332195 | 1.8429200575551248 |
| stat-g2-prefix-200-m2 | 72383.77288258076 | 139763.77188682556 | 1.930871607280641 |

All events100% running: **true**. Counter observations do not identify a memory level, execution-port cause or removable proof latency. SMT cycles sum logical-thread events, not disjoint physical cores.

| Profile | Samples | Sampled worker TIDs | Lost |
|---|---:|---:|---:|
| cycles-g1-borrowed-signed-200-m1 | 11504 | 32 | 0 |
| cycles-g1-borrowed-signed-200-m2 | 11469 | 32 | 0 |
| cycles-g2-prefix-200-m1 | 24009 | 32 | 0 |
| cycles-g2-prefix-200-m2 | 30072 | 32 | 0 |

Total77054 samples, each profile one PID/all32worker TIDs/zero lost. Actual original and scaled apply disassembly is archived; static counts are not executed work, dynamic spills or stack bounds. Every transitive runtime library/proc macro is frozen and flags/features checked. Executables/raw profiles stay local with hashes;4loss dumps use deterministic gzip and verified byte-exact decompression. Captured logs and measured sources retain bytes, including trailing blank lines; authored report/helper whitespace is checked separately.

This exact native candidate is closed without a full SDK trial or adoption. No unchanged arithmetic/layout reroll is selected.

## Inherited CPU and Metal comparison

No new full-proof latency or peak RSS is established. Accepted FFT adoption retains its engineering exception and false full-performance/qualified-scope gates. The original all-eight CPU10x objective remains active and unmet,0/8. These accepted local timings and supplied M5 references are inherited.

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

Historical primary peak134.50MiB; merge54 remains161.341835ms /1910.1875MiB, with no supplied M5 reference. No new SDK/Gnark proof, portable/mobile/TEE, concurrent-proof, pressure/giant-circuit or AWS/Aeglos measurement is claimed. Independent proofledger remains13,181.

[Design](artifacts/design.md), [bounded field code](artifacts/scaled.rs), [point kernel](artifacts/g2_scaled.rs), [numeric oracle](artifacts/wide-check.rs), [analysis](artifacts/analysis.json), [native suites](artifacts/native/result.json), [raw native checks](artifacts/numeric-linked/wide-tests/stdout.log), [all captures](artifacts/runs/result.json), [source audit](artifacts/source-audit.json), [control preflight](artifacts/control-preflight.json).

## Profile-directed follow-up

The selected G2 profile attributes **8.596% of sampled period** to the out-of-line `zopro_bn254::ifma8::scaled::modulus` function. The source calls `modulus(1)` and `modulus(P)` from inlined export/linear operations without an explicit const context. Although its arguments are fixed, this implementation reconstructs five-limb constants at runtime. The checked bound helper also calls `modulus(B)`. This work is unnecessary; it is separate from the intended arithmetic trade.

The grouped G2 observation records about27.7% more instructions and21.9% more cycles per call. The constant builder's sampled weight does not quantify all consequences of the calls, conversion costs or spills. No exclusive cause or recoverable full-proof latency is inferred.

The next distinct implementation change is to require const evaluation at those three source sites. The five-limb constants are fixed by the field modulus and small bounds, without a per-key cache or point table. Keep this measured negative and all failed gates. Revalidate the changed native code and compare complete concurrent callers; neither const syntax nor disappearance of the symbol establishes speed. See [numeric insight](artifacts/constant-insight.json).
