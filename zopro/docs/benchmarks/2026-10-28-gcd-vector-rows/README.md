# Paired SIMD integer rows in complete GCD inversions

**The standalone prototype passes its complete-inversion screen.** At 32 logical CPUs, Fq selected/fresh paired median is 0.985412381, bounds [0.984074626,0.986847015], 31/31 wins; selected/frozen-library is 0.992434843, bounds [0.990014093,0.993630494], also 31/31. All six calibrations/nonregression contexts pass. This permits actual point-caller integration; it is not a prover speedup or adoption.

The candidate computes both full four-word, 31-bit GCD integer rows using four AVX-512 VPMULUDQ products. It shares conversion of A/B into eight radix-32 digits between rows. Multiplication by a coefficient's unsigned low32 bits and a masked subtraction of `digit << 32` for negative coefficients recovers the signed result modulo 2^64, including ±2^31. The existing row L1 bound keeps each signed column inside i64. Scalar radix-32 carries reconstruct both signed five-word values, followed by the original absolute-value/row-sign normalization and 31-bit shift.

With B=2^31, column bounds are [−(2^32−1)B,(2^32−1)B]. Adding an incoming carry in [−B,B−1] stays in i64, and the next radix-32 carry remains in that same interval. Only integer rows use this bound: modular coefficient updates can have 62-bit coefficients and remain unchanged. Matrices, balanced halving loop, Euclid finish, coefficient updates and final Montgomery correction retain accepted code. No active-word thresholds, lookup table, heap allocation, key preparation, assembly backend or public option is added. Rust intrinsics use bounded unaligned 32-byte loads and 64-byte stores in the standalone helper; no production unsafe code is enabled here.

Earlier packed30/31 and SSE2 trials changed the small matrix-coefficient loop. This changes the subsequent full-integer products and is separate from those trials and the rejected active-word update. The [Rust multiply intrinsic](https://doc.rust-lang.org/core/arch/x86_64/fn._mm512_mul_epu32.html) uses unsigned low32 inputs and 64-bit results; the [conversion intrinsic](https://doc.rust-lang.org/core/arch/x86_64/fn._mm512_cvtepu32_epi64.html) zero-extends the packed digits. These semantics are not performance predictions.

## Complete inverse measurements

One native executable contains a frozen accepted-library control, a copied ordinary routine, and the paired routine. The external prototype copies accepted inverse.rs/portable.rs exactly, with only the declared pair-update replacement and test-only reference wrappers. Production's 172 files and core are unchanged. All field multiplication and other operations link the exact immutable accepted SDK field library, SHA f1285944f3fc44982915821769ac1c646c3cb8c2f3c67003dc95afcda0ce8a04. This is library/source-level primitive calibration, not the accepted proof executable or an exact-SDK machine-code equivalence claim.

31 fixed rotated/reversed blocks cover Fq/Fr at solo [2], physical16 [0–15], logical32 [0–31] and three modes: 558 fresh processes. Each worker runs three checked one-pass primes over 4,096 deterministic nonzero field inputs, then 16 complete passes (65,536 inversions). Independent Arkworks references are prepared outside clocks. Worker output buffers are private; input/reference buffers are shared read-only. Start/done/release barriers prevent checking or allocation from overlapping any timed worker. Main ns/inverse-wave includes the slowest worker and barrier tail; per-worker clocks/CPU and aggregate throughput are retained. Input reads, inverse calls and output writes are timed. No eight-lane batch-inverse substitution is made.

| Field | Layout | Frozen ns/wave | Fresh ns/wave | Paired ns/wave | Paired/fresh median | Marginal bounds | Wins |
|---|---|---:|---:|---:|---:|---|---:|
| fq | solo | 526.229675 | 527.421967 | 523.880585 | 0.993322348 | [0.992070534, 0.995066371] | 30/31 |
| fq | physical16 | 572.801895 | 572.036911 | 569.256851 | 0.995701853 | [0.994371092, 0.997343778] | 30/31 |
| fq | logical32 | 816.879883 | 821.825729 | 809.700058 | 0.985412381 | [0.984074626, 0.986847015] | 31/31 |
| fr | solo | 525.290710 | 527.709976 | 520.667892 | 0.986555642 | [0.985760593, 0.988026595] | 31/31 |
| fr | physical16 | 572.003265 | 573.968628 | 567.841812 | 0.989260800 | [0.987493038, 0.990721889] | 30/31 |
| fr | logical32 | 823.158508 | 826.386658 | 809.122192 | 0.978746557 | [0.976717319, 0.981533385] | 31/31 |

| Field/layout | Fresh/frozen paired median | Marginal bounds | Selected/frozen paired median | Marginal bounds |
|---|---:|---|---:|---|
| fq/solo | 1.001478153 | [0.999910242, 1.004142722] | 0.995867515 | [0.992808890, 0.997350941] |
| fq/physical16 | 0.999658753 | [0.997233965, 1.002092111] | 0.996015315 | [0.993846587, 0.997210169] |
| fq/logical32 | 1.006606138 | [1.003224852, 1.008825267] | 0.992434843 | [0.990014093, 0.993630494] |
| fr/solo | 1.003873897 | [1.002082880, 1.006248567] | 0.990495840 | [0.988448082, 0.992227490] |
| fr/physical16 | 1.004426625 | [1.001243502, 1.005517860] | 0.992630642 | [0.991283097, 0.994402922] |
| fr/logical32 | 1.004100691 | [1.002047682, 1.006616972] | 0.983770223 | [0.981635976, 0.984759120] |

Bounds are ranks 10–22 of 31 paired process ratios, nominal 97.055063% marginal coverage under independent exchangeable blocks, without familywise guarantees. Fresh/frozen bounds must fit [1/1.03,1.03] in all contexts. Fq/logical32 must have selected upper bounds below parity and at least 21/31 wins against both controls; every selected upper bound must be at most 1.02. All checks pass. No gain floor, exclusions, schedule extensions or successful-capture repeats are used. Fresh/frozen differences remain visible, including a roughly 0.66% slower fresh Fq/logical32 control; selected beats both. The small positive solo/physical gains are retained without equating them to proof gains.

## Correctness and captured code

Checked-overflow/debug and optimized suites both pass 10 tests: three existing full-inverse/one-word tests for each copied algorithm, two new pair/carry tests and two inherited perf-helper tests. The new pair test compares 65,616 input pairs (131,232 rows) against the original four-word routine and an independent unsigned five-word magnitude oracle. It covers every bit width 1–256, arbitrary bounded rows with divisible inputs, zero, cancellation, full-word and pure ±2^31 cases. Zero is explicitly excluded from negative-sign normalization. Repeated builds are not additional distinct cases. Existing 10,000-random-input-per-field inversion tests retain their Fermat/product checks and raw-word/one-word boundaries.

Read-only disassembly confirms four VPMULUDQ sites in each selected Fq/Fr inverse and zero in the copied ordinary inverses; all six repeated-call functions are also recorded. Static counts do not establish performance causes. Native tests and every capture succeed on their first attempts. There is no Cargo/core/field-library rebuild, source/ISA/gate change or timing retry. All available rlibs/proc-macro objects in the two link dependency directories are hashed after the standalone build and before capture, including unused files; this supplements the direct build-input guards and does not claim every available library was used.

All **566** latency/stat/profile processes check complete outputs against Arkworks and `x * inverse = 1`, with matching input/output digests and zero global timed allocations. They perform **731,512,832 timed inverse calls**, not SDK/Gnark proofs. The helper's two eight-i64 digit buffers and two five-u64 reconstructed rows are fixed local storage; no measured stack footprint, proof RSS or minimum RAM follows from those source arrays.

## Counters and profiles

All 558 unprofiled captures finish before four separate FIFO-gated grouped perf-stat captures, then four separate 199 Hz cycles:u profiles. These use logical32 and 128 passes/worker (524,288 inversions), after three one-pass primes. All eight finish before decoding, hashing, compression or archive. No owned helper/build/hash/archive/other benchmark overlaps capture. Host/compiler/topology snapshots are retained; no exclusive-host claim is made.

| Field/mode | Cycles/inverse | Instructions/inverse | IPC | Branch misses/inverse | Generic cache misses/inverse |
|---|---:|---:|---:|---:|---:|
| stat-fq-m1 | 4038.815 | 8115.859 | 2.009466 | 13.234688 | 0.003390 |
| stat-fq-m2 | 3992.312 | 7914.014 | 1.982314 | 13.675960 | 0.002399 |
| stat-fr-m1 | 4070.814 | 8131.702 | 1.997562 | 13.718942 | 0.002568 |
| stat-fr-m2 | 3989.949 | 7930.006 | 1.987495 | 13.417675 | 0.001768 |

All four events run at 100% in every group. Counts sum logical-thread events and divide by all inverse calls; SMT cycles are not disjoint physical-core cycles. Single stat observations show about 2.49% fewer Fq instructions and 2.48% fewer Fr instructions, with cycles lower about 1.15%/1.99%. IPC is lower despite lower cycles, so IPC alone is not the outcome. These observations do not identify an exclusive instruction, execution-port, cache or DRAM cause.

| Profile | Samples | Sampled worker TIDs | PIDs | Lost |
|---|---:|---:|---:|---:|
| cycles-fq-m1 | 2530 | 32 | 1 | 0 |
| cycles-fq-m2 | 2470 | 32 | 1 | 0 |
| cycles-fr-m1 | 2524 | 32 | 1 | 0 |
| cycles-fr-m2 | 2480 | 32 | 1 | 0 |

Total samples: 10004. Each profile has one PID, all 32 worker TIDs sampled and zero lost records. Full symbols and decodes are archived; period shares are not removable proof wall time. Raw perf.data remains local with hashes, and loss dumps are archived losslessly with deterministic gzip.

## Inherited proof comparisons and next step

No proof measurement changes here. The all-eight 10× CPU objective remains active and unmet, 0/8; primary target 3.7 ms. These accepted 172-source observations and supplied M5 references are inherited. Accepted FFT engineering exception and false full-performance/qualified-scope gates remain unchanged.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU / accepted | Metal / accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 24 | 4.445× | 2.883× |
| 2x2 | 11.460675 | 59 | 30 | 5.148× | 2.618× |
| 4x2 | 15.854365 | 109 | 48 | 6.875× | 3.028× |
| 8x2 | 24.084626 | 189 | 74 | 7.847× | 3.072× |
| 12x2 | 33.693397 | 284 | 101 | 8.429× | 2.998× |
| 16x2 | 39.981636 | 335 | 124 | 8.379× | 3.101× |
| 24x2 | 66.010670 | 528 | 176 | 7.999× | 2.666× |
| merge-24x1 | 65.086624 | 507 | 175 | 7.790× | 2.689× |

Historical accepted primary peak is 134.50 MiB; merge54 is 161.341835 ms / 1910.1875 MiB with no supplied M5 reference. No SDK/Gnark proof is generated; ledger remains 13,082. No new actual point caller, all-eight/54, native prover, portable/mobile/TEE, pressure, giant-circuit or AWS/Aeglos result is claimed. No cloud resource is created.

Next: port the exact paired helper behind native guards into a private accepted-source snapshot, independently validate it, then compare complete concurrent point callers. SIMD root work can interact with the surrounding IFMA arithmetic differently from this inverse-only load. Only a passing caller study permits pure full-SDK/all-shape/resource qualification before adoption. [Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [code](artifacts/codegen.json), [field-library provenance](artifacts/field-library.json) and raw logs are included.
