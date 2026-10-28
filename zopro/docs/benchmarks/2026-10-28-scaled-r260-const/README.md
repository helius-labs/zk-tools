# Compile-time modulus constants in scaled G2 batches

The candidate **fails the declared complete native caller screen and is not adopted**. Primary G2/200 at32 logical CPUs has ON/OFF ratio **1.091312735**, marginal interval [1.070047888,1.121425064], 4/31 wins; ON/frozen **1.100214924**, interval [1.064785735,1.123739132], 3/31 wins. Production and the accepted SDK remain unchanged.

## Arithmetic and storage

Require explicit const evaluation at three fixed-modulus sites in the validated parent native implementation. The arithmetic remains actual AVX512 IFMA using (x,y) → (x/4,y/8), uniform Montgomery R=2^260=16H and host radix H=2^256. Canonical host raw coordinates import as4a/2a; outputs use modular quarter/half followed by canonicalization. Keep the original two-fused-product complex formulas. Wider temporaries survive several linear operations instead of repeated generic corrections: imported X4p/Y2p, dx8p/dy4p, norm3p, prefix/slope2p, square4p, Xout12p, finishing difference16p and Yout4p. Bounds are inclusive in the native types; strict product inequalities follow from64p<R.

A separate bounded child module calls the existing private uniform CIOS implementation for the proven wider domains. Public get/compact/difference2 contracts and original field runtime stay unchanged. Native codecs, five-product central inverse tree, host/root conversion, packing and scattering are inside the caller. No persistent recoded key, lookup table or second key plan is created. Identity/equal-x batches take a complete scalar fallback before vector mutation. Existing distinct queued-index contract remains; incomplete8lane groups use the original path.

The typed prefix Vec has640B per8point record, the same active record size as before. A24B optional Vec field is added to experimental scratch. Old and new prefix capacities can coexist after mixed complete/tail calls; this is a real memory cost. Ordinary caller capacity includes the new vector; no unchanged-peak-RSS/minimum-RAM or universal zero-overhead claim is made.

A private accepted172-source copy adds only cfg-only dispatch/scratch/module declarations and diagnostic capacity accounting.168 originals remain byte-exact; reversing edits recovers all172. The complete accepted OFF prefix body remains verbatim. Frozen and fresh OFF code and constants are checked byte-identical before timing; only build ID and20 verified panic source-line fields differ, addressing the prior canonical-Xsum control setup error. This is a distinct wider-arithmetic candidate; rejected Gauss/GCD/GLV/weighted/square/FFT candidate runtime is absent.

## Native correctness

The full field suite passes33 tests with debug and overflow checking. Inherited unchanged OFF qualification has122 core tests; the changed ON build passes124 core tests:119 accepted tests,3 original point-adapter tests, plus2 ON point tests. New native point coverage checks2560 full-width-generator outputs at sizes8/16/32/64/200 over8 changing rounds, scratch reuse, four whole-call exceptional batches and an empty case. Inherited tests cover original tails and FAST true/false. Optional external-fixture cases that return early are not claimed.

Three separate tests against the actual frozen ON field library compare42,885 wider-domain CIOS scalar tuples with independent BigUint full-radix REDC, check2076 canonical raw codec inputs for both X/Y, and16,462 wider export/alias cases including inclusive bounds. Padding into8lane groups repeats inputs and is not extra distinct coverage. This upgrades the scalar model to actual native instruction validation. Complete complex/linear chains are also exercised by the native point oracles.

Initial generation fails before setup/build because a replacement matches both link and capture step declarations. Anchoring to the link variant loop repairs only generation. The original generator/partial outputs are preserved. Parent-only setup/numeric/preflight failures are documented in its report; they are not new failures here. The narrow20source-line/build-ID metadata allowance is declared before this follow-up, without changing performance gates. Native arithmetic is unchanged apart from compile-time evaluation. No successful tests or captures are repeated for bookkeeping.

## Caller timing

31 fixed rotated/reversed blocks, G2prefix32/200 and unchangedG1borrow200, soloCPU2/physical16CPUs0–15/logical32CPUs0–31 on Ryzen9950X. Three reset checked1024call primes precede8192timedcalls/worker. Ready/start/done/release barriers put checking/allocation outside every worker's clock; wave time includes slowest worker and barrier tail. These synthetic ordinary points at real stream sizes are point-kernel timings, not full proofs or whole MSMs.

| Caller/pairs | Layout | Frozen µs/wave | OFF µs/wave | ON µs/wave | ON/OFF | Marginal interval | Wins |
|---|---|---:|---:|---:|---:|---|---:|
| g2-prefix/32 | solo | 1.878116 | 1.875784 | 2.022310 | 1.079770863 | [1.075625285,1.084880505] | 0/31 |
| g2-prefix/32 | physical16 | 2.382969 | 2.355165 | 2.547182 | 1.082421074 | [1.072606249,1.089184584] | 3/31 |
| g2-prefix/32 | logical32 | 4.118658 | 4.208232 | 4.473716 | 1.067688874 | [1.035829287,1.081695196] | 3/31 |
| g2-prefix/200 | solo | 7.852598 | 7.817288 | 8.498856 | 1.086945130 | [1.083789839,1.090710467] | 0/31 |
| g2-prefix/200 | physical16 | 9.735689 | 9.683095 | 10.835271 | 1.112081372 | [1.107880415,1.128289154] | 0/31 |
| g2-prefix/200 | logical32 | 17.807198 | 17.998577 | 19.410073 | 1.091312735 | [1.070047888,1.121425064] | 4/31 |
| g1-borrowed-signed/200 | solo | 4.361904 | 4.347878 | 4.324678 | 0.997058017 | [0.994871702,1.001022260] | 20/31 |
| g1-borrowed-signed/200 | physical16 | 5.097178 | 4.984147 | 5.109256 | 1.010958499 | [1.003182861,1.026808478] | 6/31 |
| g1-borrowed-signed/200 | logical32 | 9.117367 | 9.123132 | 8.976432 | 0.982831934 | [0.957200705,0.997357688] | 22/31 |

| Caller/pairs/layout | OFF/frozen | Interval | ON/frozen | Interval |
|---|---:|---|---:|---|
| g2-prefix/32/solo | 0.998988366 | [0.994255891,1.002865144] | 1.076687971 | [1.074891161,1.081231421] |
| g2-prefix/32/physical16 | 0.989202520 | [0.983894775,0.998585239] | 1.066171429 | [1.061226189,1.073386117] |
| g2-prefix/32/logical32 | 1.003647139 | [0.977845615,1.047667582] | 1.079657819 | [1.044389106,1.107769206] |
| g2-prefix/200/solo | 0.993888136 | [0.992625728,1.000559169] | 1.081137941 | [1.078773187,1.087147301] |
| g2-prefix/200/physical16 | 0.993141673 | [0.984188419,0.999023506] | 1.107824776 | [1.102885070,1.111840481] |
| g2-prefix/200/logical32 | 1.020312456 | [0.977433094,1.045149282] | 1.100214924 | [1.064785735,1.123739132] |
| g1-borrowed-signed/200/solo | 0.995730261 | [0.993437589,1.002398975] | 0.994760119 | [0.988663550,0.997607625] |
| g1-borrowed-signed/200/physical16 | 0.991444865 | [0.984160306,0.996435767] | 0.996455234 | [0.991230932,1.002870940] |
| g1-borrowed-signed/200/logical32 | 1.005481818 | [0.991489084,1.041944515] | 1.007638222 | [0.974467134,1.035305505] |

Calibration **false**; nonregression **false**; primary gain **false**. Calibration failures: ["g1-borrowed-signed/200/logical32/1-over-0","g2-prefix/200/logical32/1-over-0","g2-prefix/32/logical32/1-over-0"]. Nonregression failures: ["g1-borrowed-signed/200/logical32/2-over-0","g1-borrowed-signed/200/physical16/2-over-1","g2-prefix/200/logical32/2-over-0","g2-prefix/200/logical32/2-over-1","g2-prefix/200/physical16/2-over-0","g2-prefix/200/physical16/2-over-1","g2-prefix/200/solo/2-over-0","g2-prefix/200/solo/2-over-1","g2-prefix/32/logical32/2-over-0","g2-prefix/32/logical32/2-over-1","g2-prefix/32/physical16/2-over-0","g2-prefix/32/physical16/2-over-1","g2-prefix/32/solo/2-over-0","g2-prefix/32/solo/2-over-1"].

All9OFF/frozen intervals must fit[1/1.03,1.03], all18 ON/frozen and ON/OFF upper ratios<=1.02. The primary G2/200 logical32 gain requires upper<1 and>=21/31 wins against both controls; legacy logical32_aggregate contains only this primary caller. Ordered ranks10–22 of31 give nominal97.055% marginal coverage under independent exchangeable blocks, not familywise. No gain floor, exclusions, extension or unchanged reroll. All positive estimates and uncertainty remain within their measured scope. Identical control code and constants exclude different compiled arithmetic as a control gap; host variation, barrier tails and process placement/address layout remain unresolved.

## Counters, profiles and provenance

All837 latency processes complete before4FIFO grouped perf-stat, then4 separate FIFO199Hz cycles:u profiles(G2/200,G1/200 OFF/ON logical32,262144calls/worker), then12owner/decode/loss steps.845kernel captures/857workflow steps verify complete generator results, capacity consistency and zero global timed allocations.179,101,696timed kernel calls are not proofs. No owned helper/build/hash/archive/other benchmark overlaps captures; host snapshots do not prove exclusive use.

| Counter case | Cycles/call | Instructions/call | IPC |
|---|---:|---:|---:|
| stat-g1-borrowed-signed-200-m1 | 29526.869809150696 | 52350.29812467098 | 1.7729714820108382 |
| stat-g1-borrowed-signed-200-m2 | 29426.254184126858 | 52350.29819989205 | 1.77903371160747 |
| stat-g2-prefix-200-m1 | 58834.38155734539 | 109427.98592340946 | 1.85993262828387 |
| stat-g2-prefix-200-m2 | 65177.58117032051 | 112889.57029592992 | 1.7320306809319843 |

All events100% running: **true**. Counter observations do not identify a memory level, execution-port cause or removable proof latency. SMT cycles sum logical-thread events, not disjoint physical cores.

| Profile | Samples | Sampled worker TIDs | Lost |
|---|---:|---:|---:|
| cycles-g1-borrowed-signed-200-m1 | 11305 | 32 | 0 |
| cycles-g1-borrowed-signed-200-m2 | 11300 | 32 | 0 |
| cycles-g2-prefix-200-m1 | 23651 | 32 | 0 |
| cycles-g2-prefix-200-m2 | 25674 | 32 | 0 |

Total71930 samples, each profile one PID/all32worker TIDs/zero lost. Actual original and scaled apply disassembly is archived; static counts are not executed work, dynamic spills or stack bounds. Every transitive runtime library/proc macro is frozen and flags/features checked. Executables/raw profiles stay local with hashes;4loss dumps use deterministic gzip and verified byte-exact decompression. Captured logs and measured sources retain bytes, including trailing blank lines; authored report/helper whitespace is checked separately.

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

## Constant-only change and inherited control

The parent native implementation was30.4% slower than OFF on the primary caller. Its profile attributed8.596% of sampled period to the out-of-line modulus-constant builder. This follow-up requires const evaluation at exactly three sites: `modulus(B)`, `modulus(1)` and `modulus(P)`. Reversing those edits recovers the parent bounded module byte-for-byte; the point kernel and repaired numeric harness are also byte-identical to the parent. No formula, bound, representation, scratch, key or scheduling change accompanies this trial.

OFF denotes the unchanged parent frozen OFF libraries and their122-test core qualification, reused without recompilation or repeated native tests. Original accepted frozen libraries remain a separate control. New ON code runs the checkedfield33/core124/numeric3 suites; three harness tests pervariant validate the newly linked measurement executables. Reusing test inputs on changed code is validation, not extra distinct input coverage. Library/source/test receipts identify the inherited and new artifacts explicitly.

The20 panic-line/build-ID metadata allowance was declared before this follow-up; code, constants and all other runtime bytes must match between its frozen and OFF controls. Initial generator preparation failed because one replacement matched both link/capture step declarations; anchoring it to the link variant loop repairs generation before any native build. The initial generator/partial output and repair are preserved. No native arithmetic or successful tests were repeated for that preparation repair.

The next decision follows the complete caller gates and actual counters/profiles. Removing constant reconstruction or a symbol alone is not proof of a speedup. No full-proof claim is made here.

## Result after removing runtime constant construction

All six G2 ON/OFF median ratios are slower, by6.77%–11.21%, with every marginal interval above parity. Primary32worker G2/200 is9.13% slower than OFF (4/31wins) and10.02% slower than the original frozen control (3/31wins). Calibration, nonregression and primary-gain gates all fail. No full SDK trial or adoption follows.

The out-of-line modulus-builder symbol is absent from the linked candidate and its sampled profile weight is zero. The grouped G2 observation now records112,889.57instructions/call versus109,427.99OFF (+3.16%) and65,177.58cycles/call versus58,834.38OFF (+10.78%). The parent candidate recorded139,763.77instructions/call in a different study. That cross-study reduction is descriptive, not a paired latency estimate or a full-proof gain.

For this implementation, eliminating runtime constant reconstruction does not make the wider representation faster than the accepted mixed-radix path. Codec, guard, wider-linear and register/dependency costs remain included; the observations do not isolate each cost. This exact implementation is closed. The next selection should remove arithmetic or data movement at the complete prover level, rather than assume a smaller static kernel or fewer corrections guarantees speed.
