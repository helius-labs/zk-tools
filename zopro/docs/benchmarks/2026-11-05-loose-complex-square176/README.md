# Deferred correction in an ordinary complex square

The carry-only operand preparation and final correction pass the integer model and checked/optimized native field tests. This qualifies a complete-caller experiment. **No caller timing, speedup or adoption is claimed.** Production176, SDK6d83, the29,953-proof ledger and all accepted CPU/Metal results remain unchanged. The all-eight CPU10× goal is active and unmet,0/8.

## Change and bound

For the real component of an ordinary Fq2 square, prepare numerical operands `S=a+b` and `D=2p+a−b` with carry-only passes. Multiply using the original unsigned ordinary CIOS body, then apply its existing final2p correction. The imaginary component `2ab` remains original. Neither private loose operand escapes the helper; both are normalized radix52 digits and fit256bits.

The combined bound is tighter than multiplying independent operand limits:

`0 ≤ (a+b)(2p+a−b) = (a+p)² − (b−p)² ≤ 9p²`, for `0≤a,b≤2p`.

Since `4p<R256`, the unsigned Montgomery result satisfies `U < p+9p²/R < 13p/4 < 4p`. One conditional subtraction of2p restores the established output bound below2p. For uniformR260, `R260>64p` gives `U<2p` without correction. The wrapper checks the modulus-size condition at compile time. It extends the original product's usual operand-range contract only within this helper and checks its per-digit coefficient safety separately.

The copied CIOS runtime body is exact apart from the declared helper signature/names, compile-timeFUSED=false and selection of the existing final correction for mixed mode. The original reduction/correction logic is copied unchanged apart from names. Original generated code and generator remain byte-exact. This retains the ordinary product count; unlike the preceding signed symmetric square, it adds no triangular products or signed raw cancellation. Actual linked code/performance still needs measurement.

There is no basis/representation change, new heap buffer, key table/copy, retained plan, pool, workspace policy or RAM cap. Fixed-size SIMD temporaries are not a measured stack or minimum-memory bound.

## Qualification

The exact integer model checks **26,856 outputs** and **134,280 row identities** across Fq/Fr andR256/R260. Each configuration has5,690boundary/random pairs plus1,024changing outputs. It checks prepared S/D values, the correlated9p²bound, exact unsigned CIOS state, closed Montgomery REDC, final correction, normalized bounds and field interpretation. Final corrections are exercised419times for Fq/R256 and431for Fr/R256; uniformR260 needs none. Conservative coefficient envelopes and observed maxima are distinct evidence.

Two new native tests pass in each checked-optimized and release-optimized build. Each build checks **30,976 raw lanes** and **65,536 changing complex-square component outputs**, totaling96,512, against independent Arkworks and original arithmetic. Cases include inclusive2p endpoints, modulus/radix/power boundaries, deterministic lazy random operands and all256negative-result masks. Output counts include padding/repeated vectors. Checked and release reuse the same geometry; they are not twice as many unique mathematical cases. The geometry is reused from the prior signed-square test harness for this different implementation.

The35inherited field tests are filtered, not rerun. No core/prover/portable, valid-curve caller, SDK, Gnark, pressure/mobile or AWS qualification occurs here. The source snapshot has177files:175originals exact, one module suffix and one new child; removing them recovers all176original bytes.

The initial direct model compilation failed before execution because an explanatory identity moved a BigUint later used by the REDC oracle. Only a borrow annotation was repaired. The original source/note and tool transcript retain that failure; its stderr was not separately saved locally. The model and both native modes then execute successfully once under their recorded workflows. No successful qualification is repeated.

The independent Rust reviewer imports no model/native code. It checks289pins and all four successful workflow commands, stream hashes, protocol defaults, the borrow-only repair, exact original CIOS/correction transformations, source recovery, native flags/counts/executable hashes, model totals, a separately derived coefficient envelope, accepted SDK and unchanged proof ledger. Review passes on its first execution. There is no timing gate to claim passed at this stage.

## History and next experiment

The [signed-square caller](../2026-11-05-signed-complex-caller176/README.md) remains closed: its0.646%all-six improvement did not qualify the required32-worker gain. This method replaces its signed symmetric110IFMAreal lowering with the ordinary unsigned multiplication and correlated loose preparation; it is not a layout/flag reroll of that implementation.

The earlier [loose-delta SDK](../2026-10-30-loose-delta-sdk175/README.md) changed the finalY product with a canonical anchor and remains negative. The generic [signed product difference](../2026-10-29-signed-difference175/README.md) and [caller](../2026-10-29-signed-g2-caller175/README.md), ordinary symmetric-square, sum-norm, trace-one, canonicalXsum and paired-cancellation negatives remain closed. A bounded report/history search found no exact ordinary correlatedS/Dsquare trial; no mathematical novelty claim is made.

Next: test this exact qualified wrapper inside complete signed borrowed-G2 addition, preserving the original OFF expression, imaginary component, norm/inversion, point handling, tails, capacities and geometry. The fixed proposal is200/352pairs ×1/16/32workers, exactFROZEN/freshOFF/ON, complete oracles/zero timed allocation,15rotated blocks, required all-six and32-worker aggregate gains against both controls, all calibration/nonregression checks and post-latency profiles. Freeze the full protocol/helpers before timing. Only a complete pass permits SDK/all-eight/54/portable/resource qualification. There is no prediction of a twofold proof gain.

## Unchanged accepted full-proof comparison

Inherited accepted32-worker local measurements and supplied M5 references, not measurements of this candidate. Source: [accepted manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json). Different machines do not establish equal hardware efficiency. Latest separate full-SDK evidence remains [finalization/drain composition](../2026-11-05-finalize-drain-bundle176/README.md), with its primary positive estimate and failed full gate retained.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU/local | Metal/local | CPU10× target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.928660 | 37 | 24 | 4.667× | 3.027× | 3.7 |
| 2x2 | 11.151932 | 59 | 30 | 5.291× | 2.690× | 5.9 |
| 4x2 | 16.086386 | 109 | 48 | 6.776× | 2.984× | 10.9 |
| 8x2 | 24.179453 | 189 | 74 | 7.817× | 3.060× | 18.9 |
| 12x2 | 33.884455 | 284 | 101 | 8.381× | 2.981× | 28.4 |
| 16x2 | 40.077381 | 335 | 124 | 8.359× | 3.094× | 33.5 |
| 24x2 | 65.850777 | 528 | 176 | 8.018× | 2.673× | 52.8 |
| merge24x1 | 65.060937 | 507 | 175 | 7.793× | 2.690× | 50.7 |

Merge54x1 remains161.883899ms without a supplied M5 reference. No new proof/export or AWS/Aeglos, giant-circuit, minimum-RAM, swap/pressure, mobile/TEE or concurrent-throughput result is added. Goal remains active and unmet,0/8.

## Reproduction

[Design](artifacts/design.md), [wrapper](artifacts/wrapper.rs.in), [generated private kernel/tests](artifacts/kernel.rs), [integer model](artifacts/model.rs), [model results](artifacts/model.json), [review](artifacts/review.json), [notes](artifacts/reproduce.md), [artifact index](artifact-index.json), [checksums](SHA256SUMS).

Plans, pins, exact source transformation/bootstrap, original model source/failure note and complete successful workflow streams are included. The gzip JSON receipt archive preserves exact UTF-8 contents, relative paths, byte lengths and SHA256s, with byte-exact extraction checked. Executables, libraries and private key/image/witness arrays are excluded. All vectors are generated in the published source.
