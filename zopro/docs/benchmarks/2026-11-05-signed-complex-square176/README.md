# Signed square-difference qualification

An isolated Rust AVX512 implementation of `a² − b²` passes the signed integer model and checked/optimized native oracle tests. This qualifies a complete-caller experiment; **no performance result or adoption is claimed**. Production176 and the accepted SDK remain unchanged. The all-eight CPU10× goal is active and unmet, 0/8.

## Arithmetic change

The real component of an ordinary Fq2 square is `(a+b)(a−b)`. The candidate computes the symmetric signed raw product `a²−b²` and performs one signed Montgomery cancellation. The imaginary component `2ab` retains its existing arithmetic. This keeps the ordinary basis, field/point representation, inputs, keys and serialization.

For normalized numerical inputs `0 ≤ a,b ≤ 2p`, let `T=a²−b²` and `U=(T+kp)/R`, where cancellation chooses `0 ≤ k < R`. Since `4p < R` for both BN254 primes at R256/R260, `−p < U < 2p`. Adding p only to negative U yields a normalized output strictly below 2p. The raw product and cancellation coefficients can be negative; an unsigned-product bound is insufficient.

The implementation emits symmetric product diagonals into six coefficients, uses signed ceiling carries for radix52/48 cancellation, and uses arithmetic shifts in mixed-radix repacking. It adds p to negative final lanes. The exact row model and a separate conservative magnitude envelope keep coefficients below 2^61, within signed64 bounds. Intermediate intrinsic additions use the corresponding two's-complement representation. No pointer arithmetic or new memory access scheme is introduced.

The prospective real component uses 60 product plus 50 cancellation IFMAs, versus 50+50 for ordinary multiplication. It trades ten extra IFMAs and signed normalization against removing two general modular preparations. The complete caller must decide whether the trade helps. There is no heap buffer, key table/copy, retained plan, new pool or RAM cap, and no claim of a twofold gain.

## Qualification evidence

The exact integer model covers both primes and both radices. It checks **26,856 outputs** against a separate closed Montgomery REDC expression and **134,280 row identities**. Each field/radix includes 5,690 boundary/random pairs plus a 1,024-output changing chain. Signed negative pre-normalization results occur in all four contexts. Conservative coefficient bounds and observed maxima are retained separately; observed maxima are not universal proofs.

Two new native tests pass in each of checked-optimized and release-optimized builds. Each build performs **30,976 raw lane comparisons** and **65,536 changing complex-square component comparisons**, totaling **96,512**. Cases include modulus/2p endpoints, radix/power boundaries, deterministic lazy random operands, independent Arkworks field oracles, original arithmetic, normalized output bounds and all 256 negative-lane masks. Padding and repeated vectors are included in these output counts. Checked and release execute the same vectors; they do not double the distinct mathematical coverage.

All **35 inherited field tests are filtered**, not rerun. No core/prover/portable, valid-curve caller, full SDK, Gnark or memory-pressure qualification occurs here. The private snapshot has 177 files: 175 original files exact, one field-lib module suffix and one new field child; reversing the suffix/removing the child recovers all176 originals. Generated `ifma8.rs` and its generator remain exact. Cargo uses the existing release profile, native target flags and accepted cfgs; checked mode additionally enables overflow checks and debug assertions. Actual performance-driver argv still needs its own optimized-link audit before future timing.

The first model compilation stopped before execution on an ambiguous BigUint shift type. The preserved fixed source changes only integer/shift type annotations; the model then executes successfully once. Both native modes pass on their first executions. No successful model/test/capture is repeated.

The independent Rust reviewer imports neither model nor kernel code. It verifies **289 pins**, four successful workflow command receipts, the preserved compile failure, source recovery, exact annotation-only repair, native test/count/flag receipts, final test executable hashes, model totals, a separate coefficient-magnitude envelope, accepted SDK identity and the unchanged29,953-entry proof ledger. Its initial environment comparison assumed every declared variable appeared in `command.json`; xtask intentionally records managed variables only. The reviewer was corrected to account for the omitted `RUSTC`/`RUSTUP_TOOLCHAIN` receipt fields while retaining their protocol declarations. The execution runner passes the full environment. Both reviewer versions/logs are preserved; no qualification was repeated.

## Prior work and next experiment

The [ordinary symmetric square](../2026-10-24-integrated-square-proof/README.md) and [symmetric sum norms](../2026-10-28-symmetric-norm174/README.md) retain their failed performance gates. [Trace-one square](../2026-10-30-trace-one-square175/README.md) computes a different pair of unsigned expressions in a changed basis and remains rejected. This stage keeps the ordinary basis and uses signed square-difference cancellation. It does not transfer any prior timing gain.

The [canonical Xsum trial](../2026-10-28-g2-canonical-xsum/README.md) was also found during the history audit and remains closed, including its OFF-refactor/calibration limitation. No duplicate Xsum or ordinary square capture was launched. The preceding [native CRT operator](../2026-11-05-crt-euler-native176/README.md) remains closed negative.

Next: integrate this exact qualified real-component method into an isolated complete borrowed G2 addition caller, preserving the imaginary product, norm/inversion prefixes, point handling, tails, scratch and geometry. Preserve the original OFF expression byte-for-byte. Freeze a protocol with exact original/fresh controls, sizes200/352 at1/16/32workers, complete changing-output oracles, zero timed allocations, fixed interleaved blocks and post-latency profiles. Declare all gates before capture. Only a complete pass permits full-SDK/all-eight/54 correctness, latency and memory qualification. No timing gate exists yet and none is claimed passed.

## Unchanged full-proof comparison

Inherited accepted 32-worker local measurements and supplied M5 references, **not results from this candidate**. Different machines do not establish equal hardware efficiency. Source: [accepted manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json). Latest separate full-SDK evidence remains [finalization/drain composition](../2026-11-05-finalize-drain-bundle176/README.md), with its primary positive estimate and failed full gate preserved.

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

Merge54x1 remains161.883899ms without a supplied M5 reference. No new proof, AWS/Aeglos, giant-circuit, mobile, swap, minimum-RAM or concurrent-proof result is added. Fixed-size source temporaries do not establish actual stack usage or a lifetime RAM peak.

## Reproduction

See [design](artifacts/design.md), [native kernel/tests](artifacts/kernel.rs), [integer model](artifacts/model-fixed.rs), [model results](artifacts/model.json), [review](artifacts/review.json), [reproduction notes](artifacts/reproduce.md), [artifact index](artifact-index.json) and [checksums](SHA256SUMS).

Plans, source pins, exact source changes, raw command/exit/stdout/stderr receipts, both preserved failures and helper sources are included. The receipt gzip JSON archive has relative member paths, exact UTF-8 contents, lengths and SHA256s, with byte-exact extraction checked. Executables, libraries and private keys/images/witnesses are excluded. Source reconstruction uses the linked accepted manifest, the published module suffix and child, and the snapshot hash list. No private inputs are required for these synthetic algebra tests.
