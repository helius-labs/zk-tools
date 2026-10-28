# G1 weighted Y anchor

**The complete point-addition caller improves about 1.24% in the four-context aggregate.** This passes the frozen component screen and permits a separate full-SDK trial. It is not a proof-latency result or production adoption.

Store `T = 2Ax + Bx` in the existing Ax record slot. After computing `U = lambda²`, compute `X = U − (Ax + Bx)` and `Y = lambda(T − U) − Ay`. Y no longer waits for the final X subtraction. This costs one additional forward vector addition per eight pairs. The existing general add restores the lazy bound below 2p; a canonical-only add would not cover the T input range. Field kernels, prefixes, inversions, signed inputs, scalar tails, point order and the five-vector record layout remain unchanged. No key table, extra heap buffer, assembly or new unsafe operation is added.

## Caller measurements

One executable contains both const-generic kernels; two labels select the ordinary kernel and the third selects weighted. A private scratch flag is set before priming. Production default selection is controlled by a private native compile cfg, with no new public runtime option. G2 uses the unchanged kernel in all three modes. This common-image experiment has no independent frozen-executable calibration and cannot establish proof speed.

31 rotated/reversed blocks cover six contexts and three modes, giving 558 fresh latency processes. CPU 2, one caller; three checked/reset 1,024-call primes precede 4,096 timed complete calls. Fixtures use independent generator-derived points and are synthetic ordinary batches, not witness/MSM/proof requests.

| Caller | Pairs | Ordinary 0 µs | Ordinary 1 µs | Weighted µs | Weighted/ordinary 0 paired median | Marginal bounds |
|---|---:|---:|---:|---:|---:|---|
| g1-owned | 200 | 4.032869 | 4.030514 | 4.022464 | 0.997604326 | [0.995693370, 0.999602392] |
| g1-owned | 352 | 6.608669 | 6.605269 | 6.582629 | 0.996056611 | [0.994619235, 0.996660807] |
| g1-borrowed-signed | 200 | 4.230740 | 4.233090 | 4.148649 | 0.980406425 | [0.979446648, 0.981360052] |
| g1-borrowed-signed | 352 | 6.963631 | 6.970086 | 6.805006 | 0.977739848 | [0.977201591, 0.978182537] |
| g2-prefix | 200 | 7.651441 | 7.647931 | 7.652161 | 0.999676651 | [0.998657428, 1.000574061] |
| g2-prefix | 352 | 12.921135 | 12.927363 | 12.926069 | 1.000368976 | [0.999743516, 1.000978560] |

Tables show medians of process means; paired ratios are computed within blocks. Rank 10–22 bounds on 31 ratios give nominal 97.055% marginal coverage under independent exchangeable blocks, without a familywise guarantee. No gain floor, exclusion, sample extension or successful-capture retry was used. All data, including an unusually slow ordinary control in block 5, remain included.

| Four-G1-context geometric aggregate | Paired median | Marginal bounds | Wins |
|---|---:|---|---:|
| weighted-over-ordinary0 | 0.987602733 | [0.987134894, 0.988071718] | 31/31 |
| weighted-over-ordinary1 | 0.987367665 | [0.986677287, 0.988236230] | 31/31 |

Both aggregate upper bounds are below parity and both comparisons win all 31 blocks. All six ordinary-control calibrations fit the frozen factor-1.03 bounds. Every weighted/ordinary upper bound is at most 1.02, including unchanged G2 controls; G2 is excluded from the gain aggregate. Borrowed signed G1 improves about 2.0–2.3%; owned unsigned G1 improves roughly 0.24–0.39%. Small owned gains are retained. Signedness and borrowing differ together in these fixtures, so the measurements do not isolate which makes the benefit larger.

## Correctness and storage

All 124 native core tests pass with weighted selected by default: 119 accepted tests, three existing caller-adapter checks, and two new tests. New coverage checks 1,024 eight-lane groups against independent scalar field formulas, including p−1 and lazy-bound cases, and alternates formulas on reused owned/borrowed scratch across scalar tails, boundaries and canonical/vector-inverse/write modes. Complete MSM/curve/exception coverage comes from the core suite. No field arithmetic source changed; its previous tests are inherited.

All 566 latency/stat/profile processes pass independent full-output generator checks, matching output digests per shape/round scope, and zero timed allocations. G1 scratch remains 40,000/70,400 bytes for 200/352 pairs; G2 remains 16,000/28,160 bytes. The existing borrowed-fixture reference vector is 3,200/5,632 bytes and is created outside clocks. These are scratch capacities, not proof peak RSS or a device RAM minimum.

The first artifact-freezer step failed after the successful tests and library build because it expected two fingerprints and found three: two field-library Cargo profiles plus one core library. The preserved versioned repair accepts and checks both field profiles' identical features/flags and distinct profile IDs. It freezes the exact bench build artifacts. No test, Cargo build or timing was repeated. All transitive rlibs/proc-macro objects were copied to immutable storage before linking. The source audit reverses the private changes to all 172 accepted originals; 170 remain exact, and only msm.rs plus diagnostic reexports differ.

## Counters and profiles

All 558 latency captures finish first, followed by four FIFO-gated grouped perf-stat processes and four separate 199 Hz cycles:u profiles, each 262,144 calls after three 1,024-call primes. Every capture finishes before decode/hash/archive work. All four counter groups report 100% running. No owned build/helper/hash/archive/other benchmark overlaps capture; unrelated ceremony host work is recorded and there is no exclusive-host claim.

| Stat context | Cycles/batch | Instructions/batch | IPC | Generic cache misses/batch |
|---|---:|---:|---:|---:|
| stat-g1-borrowed-signed-352-m0 | 37849.953 | 85033.982 | 2.246607 | 0.006695 |
| stat-g1-borrowed-signed-352-m2 | 36959.153 | 87367.991 | 2.363907 | 0.005539 |
| stat-g1-owned-200-m0 | 21964.521 | 51479.192 | 2.343743 | 0.001675 |
| stat-g1-owned-200-m2 | 21845.983 | 52483.177 | 2.402418 | 0.001324 |

The added arithmetic raises instruction counts while measured cycles fall and IPC rises, consistent with the intended shorter dependency chain. These single stat observations do not isolate an exclusive instruction cost. Four cycle captures total 1110 samples, one sampled PID each, zero lost records. Profiles identify the corresponding ordinary/weighted kernels and scalar inversion; sampled periods are not removable proof wall fractions. Generic cache misses do not identify a cache level or DRAM bottleneck. Raw perf.data remains local with recorded hashes; decodes and read-only disassemblies are archived.

## Accepted full-proof comparison (inherited)

No proof result changed in this caller study. The all-eight 10× CPU objective remains active and unmet, 0/8. Supplied M5 values are references.

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

Historical accepted 1x2 peak RSS is 134.50 MiB; merge54 is 161.341835 ms / 1910.1875 MiB. Primary CPU target is 3.7 ms. Accepted FFT engineering exception and false full-performance/qualified-scope gates remain unchanged.

No SDK/Gnark proof was generated; ledger remains 12,983. There is no new all-eight/54, pressure/concurrency, giant-circuit, portable/ARM/mobile/TEE or AWS/Aeglos result. No cloud resource was created.

Next: build a pure SDK copy without diagnostic adapters, compare accepted/fresh ordinary/weighted full-proof latency, then qualify all eight plus 54 and resource/portable behavior if that screen passes. Component success alone is insufficient for adoption. [Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [code generation](artifacts/codegen.json) and raw logs are included.
