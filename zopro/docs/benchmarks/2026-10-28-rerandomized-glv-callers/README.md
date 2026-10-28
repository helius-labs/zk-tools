# GLV in rerandomized proof assembly callers

The bounded GLV prototype passes the declared caller screen. The combined 32-worker assembly caller is **38.47% faster**, winning all 31 paired blocks. This is an assembly-only result, not a full-proof speedup; production and the accepted SDK remain unchanged.

The current assembly computes two scalar multiples of one G1 base and a two-term G2 sum. The candidate uses the accepted two-component integer decomposition and width-five signed digits. Both groups share the coordinate cube root, but their eigenvalues differ. G2 uses the existing components (a,b). G1 uses (a-b,-b), since lambda_G1=-1-lambda_G2. Its components have magnitude below2^127. This shortens the doubling chains while preserving the scalars. Small endomorphism images of the existing eight odd multiples are computed per call; no new persistent key table or execution plan is introduced.

## Complete caller comparison

Ryzen9950X, 31 fixed rotated/reversed process blocks, four cases, two variants. Each process primes three complete16-fixture passes, then times256 changing calls. All timed outputs are retained in a preallocated vector and verified afterward against independent Arkworks generator-scalar results. Odd-table creation, recoding, allocation/drop and endomorphism images are inside clocks; the existing fixed delta table is prepared beforehand. Both algorithms use the exact accepted field rlib. Controls are verbatim extracted original functions in the same executable, not a frozen native prover binary.

The combined caller preserves the windowed_outputs join and final C addition. Fixture scalars obey inverse=u^-1 and uv=u*v; fixtures include nonunit-Z projective coordinates. Pools and caller are restricted to CPU2 for serial cases or CPUs0..15/0..31 for16/32 workers. Actual worker masks are checked before and after timing. These are synthetic assembly inputs, not witness-derived proof requests.

| Caller | Original µs | GLV µs | Paired GLV/original | Marginal interval | Wins |
|---|---:|---:|---:|---|---:|
| g1 | 73.419 | 44.716 | 0.608995507 | [0.608394076,0.609893091] | 31/31 |
| g2 | 175.176 | 105.013 | 0.599642114 | [0.598865668,0.601370039] | 31/31 |
| w16 | 178.663 | 109.080 | 0.610079685 | [0.605489338,0.613733708] | 31/31 |
| w32 | 182.742 | 112.894 | 0.615310472 | [0.609945035,0.627869441] | 31/31 |

Ratios pair process medians by block; ratios of displayed medians may differ. Bounds use ordered ranks10..22 of31, nominal97.055% under independent exchangeable blocks, without a familywise guarantee. Both serial callers and combined32 require an upper bound below parity and at least21 wins; combined16 requires upper<=1.02. Every check passes. There is no gain floor, exclusion, extension or unchanged reroll. The serial G1 candidate's block4 outlier remains included.

## Validation, allocation and profiles

The checked-overflow/debug and optimized test executables each pass five cases: three new decomposition/digit, G1 pair and G2 joint oracle cases, plus two inherited FIFO helper tests. The decomposition case checks33 boundary scalars plus4096 full-width samples for both eigenvalues, including signed-digit integer reconstruction. Group tests use those33 boundary scalars plus32 full-width samples, identity/nonunit-Z bases, prepared and unprepared delta paths. They compare the original and candidate to independent Arkworks results. These are component tests; no fresh full field/core/SDK suite is claimed.

The initial checked build failed on an untyped closure index/zero. Its source, command and failure are preserved. Explicit usize/u64 types fix that compile error before tests/timing. After the tests, review corrected the benchmark-only scalar correlation to use inverse=u^-1 outside timing. Only the harness fixture and call arguments changed; original/candidate/test function bytes remained exact. A link-only step rebuilt the benchmark, retaining the successful test-source/executable identities. No successful tests or library builds were repeated. All corrected fixtures pass the normal three prime passes before any measured call.

The 248 latency processes and four profiles verify **129,024 timed assembly calls**, not SDK/Gnark proofs. All478 capture pins match after capture, including172 unchanged production source files and available linked/unused Rust dependencies. No owned build/helper/hash/archive or second benchmark overlaps capture. The process snapshots do not establish exclusive host use.

| Caller | Original / GLV allocation calls per round | Original / GLV requested bytes per round |
|---|---:|---:|
| g1 | 3.000 / 3.000 | 1536.000 / 1536.000 |
| g2 | 3.000 / 3.000 | 3072.000 / 3072.000 |
| w16 | 6.000 / 6.000 | 4608.000 / 4608.000 |
| w32 | 6.000 / 6.000 | 4608.000 / 4608.000 |

Requested bytes count allocation/reallocation traffic, not peak live memory or RSS. The fixed GLV-derived G1 image array is512bytes, and the three G2 image/odd-delta arrays total3072bytes. Four128-byte digit arrays replace two256-byte arrays. Compiler frames and other temporaries are additional; these payloads are not stack bounds or whole-proof memory claims. Existing odd-table allocations remain included. No mobile/TEE, concurrency, pressure or giant-circuit qualification follows.

All latency precedes four separate FIFO-gated199Hz cycles:u profiles of16384 calls each. Capture enables after priming and disables before output checks. All four profiles finish before decode and loss audits.

| Profile | Samples | Sampled TIDs | Lost |
|---|---:|---:|---:|
| cycles-g2-m0 | 569 | 2 | 0 |
| cycles-g2-m1 | 334 | 2 | 0 |
| cycles-w32-m0 | 1244 | 12 | 0 |
| cycles-w32-m1 | 852 | 7 | 0 |

Total2999samples; one sampled PID per profile, no lost records. Full symbol weights and raw hashes are retained. Sample shares are not removable proof time. Raw perf.data and executables stay local with hashes; four loss dumps are archived with verified deterministic-gzip round trips.

## Full-proof scope and next step

No SDK request, independent proof export or AWS/Aeglos timing occurs; the proof ledger remains13082. This passes only the standalone assembly gate. Next port the exact runtime mechanism into a private accepted-source snapshot, run native core/prover checks and a controlled accepted/fresh-OFF/ON SDK comparison before broader qualification or adoption. This assembly-only reduction cannot by itself close the2.25x primary gap.

The following accepted local timings and supplied M5 CPU/Metal references are inherited. The accepted FFT adoption retains its engineering exception and false full-performance/qualified-scope gates. The original all-eight10xCPU objective remains active and unmet0/8.

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

Historical primary peak134.50MiB; merge54 remains161.341835ms /1910.1875MiB without a supplied M5 reference. No new full-proof latency, RSS or minimum RAM is established.

References: [Arkworks G1 GLV constants](https://raw.githubusercontent.com/arkworks-rs/curves/master/bn254/src/curves/g1.rs), [G2 constants](https://raw.githubusercontent.com/arkworks-rs/curves/master/bn254/src/curves/g2.rs). The implementation reuses the accepted bounded integer decomposition rather than importing Arkworks BigInt code. This is a new assembly use of the known GLV method, not a novelty claim.

[Design](artifacts/design.md), [candidate](artifacts/candidate.rs), [extracted controls](artifacts/original.rs), [extraction audit](artifacts/extraction-audit.json), [results](artifacts/analysis.json), [all rows](artifacts/rows.json), [checked tests](artifacts/native-v2/test-checked/stdout.log), [optimized tests](artifacts/native-v2/test-optimized/stdout.log), [fixture correction](artifacts/fixture-repair.json).
